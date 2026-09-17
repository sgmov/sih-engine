//! 引擎侧句读（parser）命令行面 —— lease-mergeleg23-parallel 簇D 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/parser 0.1.0（src/parser/：lexer.py、peg.py、tree.py、
//! engine.py、entries.py、langpack.py、lint.py、vectors.py、cli*.py），只读对表移植，
//! 围堰源码零改动。
//!
//! CLI：`parser parse --pack <语言包目录> --in <输入>`、
//! `parser entries --pack <语言包目录> --in <输入> --out <输出>`、
//! `parser lint --pack <语言包目录>`、`parser vectors --pack <语言包目录> [--freeze]`。
//! 退出码三值：0 = 产树或抽取或校验过、1 = 包校验违例或输入不可读、2 = 工具异常。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 词法模式引擎为自足回溯 VM（承围堰零第三方解析依赖纪律，std 自足），支持围堰
//!    三包（json/markdown/rust）实际用到的子集：字符类、前瞻、定宽后顾、回引、
//!    非贪婪量词、(?s:) 域内点通配、{m,n} 计数；语义近似处——\d 取 Unicode 数字符
//!    （宽于 Python 的 Nd）、\s 取 White_Space 性质（窄于 Python \s 的 \x1c-\x1f）、
//!    环视内捕获组捕获不外持久。
//! 2. 模式不合本引擎子集时报 "{where} pattern 不合 re 语法: {err}"，err 内文为本
//!    引擎报文非 Python re.error 报文；包 JSON 解析错、OSError 报文内文同为引擎
//!    报文（结构逐字对表，退出码对齐）。
//! 3. 规则匹配深度超闸（十万）按工具异常退出码二，不仿 Python 抬升递归限后
//!    RecursionError 形。
//! 4. _compose_path 同名兄弟序号按结构等值首个定位（与 Python list.index 对齐），
//!    与围堰逐字节等值。
//! 5. 词法 text u4 的孤立代理对转义出 U+FFFD（Python 孤代理可入串但 UTF-8 输出
//!    即 UnicodeEncodeError，围堰实际不可出）。
//! 6. argparse 用法报文与 --help 文本不复刻，退出码 2/0 对齐；argparse 旗标缩写
//!    （--pa 代 --pack）不受理，须全名。
//! 7. entries compact 文本形遇 >64 位整数与 NaN/Infinity 走兜底原样出（Python 可
//!    规范重排）。
//! 8. 输入 decode replace 的替换序列粒度随 Rust from_utf8_lossy（WHATWG 规则），
//!    与 Python 逐错替换在畸形序列处 U+FFFD 个数可差。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;

/// 围堰版本锚：sih-tools/parser/src/parser/__init__.py __version__。
const VERSION: &str = "0.1.0";

// ================================ 错误与杂项 ================================

/// 错误两态：Pack 即退出码一（包校验违例或输入不可读），Tool 即退出码二（工具异常）。
enum ToolErr {
    Pack(String),
    Tool(String),
}

type R<T> = Result<T, ToolErr>;

fn pack_err<T>(msg: impl Into<String>) -> R<T> {
    Err(ToolErr::Pack(msg.into()))
}

fn tool_err<T>(msg: impl Into<String>) -> R<T> {
    Err(ToolErr::Tool(msg.into()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// Python repr(str) 近似：默认单引号，含单引号且无双引号时用双引号。
fn py_repr(s: &str) -> String {
    if s.contains('\'') && !s.contains('"') {
        format!("\"{s}\"")
    } else {
        format!("'{s}'")
    }
}

/// Python list 字面量形态（sorted(set(...)) 报文用）。
fn py_list(items: &[String]) -> String {
    let parts: Vec<String> = items.iter().map(|s| py_repr(s)).collect();
    format!("[{}]", parts.join(", "))
}

/// Python str.splitlines(keepends) 等价：全边界集切分。
fn py_splitlines(s: &str, keepends: bool) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let mut cur = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        let boundary_len = if c == '\r' {
            if i + 1 < chars.len() && chars[i + 1] == '\n' {
                2
            } else {
                1
            }
        } else if matches!(
            c,
            '\n' | '\u{b}' | '\u{c}' | '\u{1c}' | '\u{1d}' | '\u{1e}' | '\u{85}' | '\u{2028}'
                | '\u{2029}'
        ) {
            1
        } else {
            0
        };
        if boundary_len > 0 {
            if keepends {
                cur.push(c);
                if boundary_len == 2 {
                    cur.push('\n');
                }
            }
            out.push(std::mem::take(&mut cur));
            i += boundary_len;
        } else {
            cur.push(c);
            i += 1;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

// ================================ 词法模式引擎 ================================
// 自足回溯 VM：支持围堰三包实际用到的 Python re 子集（见头注落差一）。

#[derive(Clone, Debug)]
enum CItem {
    Ch(char),
    Rg(char, char),
    W,
    NW,
    D,
    ND,
    S,
    NS,
}

#[derive(Clone, Debug)]
struct CClass {
    neg: bool,
    items: Vec<CItem>,
}

fn is_w(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_d(c: char) -> bool {
    c.is_ascii_digit() || c.is_numeric()
}

fn is_s(c: char) -> bool {
    c.is_whitespace()
}

impl CClass {
    fn m(&self, c: char) -> bool {
        let mut hit = false;
        for it in &self.items {
            let ok = match it {
                CItem::Ch(x) => *x == c,
                CItem::Rg(a, b) => *a <= c && c <= *b,
                CItem::W => is_w(c),
                CItem::NW => !is_w(c),
                CItem::D => is_d(c),
                CItem::ND => !is_d(c),
                CItem::S => is_s(c),
                CItem::NS => !is_s(c),
            };
            if ok {
                hit = true;
                break;
            }
        }
        hit != self.neg
    }
}

#[derive(Clone, Debug)]
enum Ast {
    Cl(CClass),
    Dot(bool),
    Grp(Box<Ast>),
    Cap(usize, Box<Ast>),
    Seq(Vec<Ast>),
    Alt(Vec<Ast>),
    Rpt { sub: Box<Ast>, min: usize, max: Option<usize>, lazy: bool },
    Look { sub: Box<Ast>, neg: bool },
    Behind { sub: Box<Ast>, neg: bool, width: usize },
    Ref(usize),
    Start,
    End,
    Dollar,
    WB(bool),
}

fn esc_char(e: char) -> Option<char> {
    match e {
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        'f' => Some('\u{c}'),
        'v' => Some('\u{b}'),
        'a' => Some('\u{7}'),
        '0' => Some('\0'),
        other => {
            if !other.is_ascii_alphanumeric() {
                Some(other)
            } else {
                None
            }
        }
    }
}

struct PatParser {
    chars: Vec<char>,
    pos: usize,
    ngroups: usize,
}

impl PatParser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    fn peek2(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }
    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }
    fn expect(&mut self, c: char) -> Result<(), String> {
        if self.next() == Some(c) {
            Ok(())
        } else {
            Err(format!("期望 '{c}'"))
        }
    }

    fn parse_alt(&mut self, dotall: bool) -> Result<Ast, String> {
        let mut alts = vec![self.parse_seq(dotall)?];
        while self.peek() == Some('|') {
            self.pos += 1;
            alts.push(self.parse_seq(dotall)?);
        }
        if alts.len() == 1 {
            Ok(alts.pop().unwrap())
        } else {
            Ok(Ast::Alt(alts))
        }
    }

    fn parse_seq(&mut self, dotall: bool) -> Result<Ast, String> {
        let mut items: Vec<Ast> = vec![];
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            items.push(self.parse_repeat(dotall)?);
        }
        Ok(match items.len() {
            0 => Ast::Seq(vec![]),
            1 => items.pop().unwrap(),
            _ => Ast::Seq(items),
        })
    }

    fn parse_repeat(&mut self, dotall: bool) -> Result<Ast, String> {
        let atom = self.parse_atom(dotall)?;
        let (min, max) = match self.peek() {
            Some('*') => {
                self.pos += 1;
                (0, None)
            }
            Some('+') => {
                self.pos += 1;
                (1, None)
            }
            Some('?') => {
                self.pos += 1;
                (0, Some(1))
            }
            Some('{') => {
                let save = self.pos;
                match self.parse_counted() {
                    Some(mm) => mm,
                    None => {
                        self.pos = save;
                        return Ok(atom);
                    }
                }
            }
            _ => return Ok(atom),
        };
        let lazy = if self.peek() == Some('?') {
            self.pos += 1;
            true
        } else {
            false
        };
        if let Some(mx) = max {
            if mx < min {
                return Err("量词 min 不得大于 max".to_string());
            }
        }
        Ok(Ast::Rpt { sub: Box::new(atom), min, max, lazy })
    }

    /// {m} {m,} {m,n}；不合数即 None（'{' 作字面量）。
    fn parse_counted(&mut self) -> Option<(usize, Option<usize>)> {
        let start = self.pos;
        self.pos += 1; // '{'
        let mut lo = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                lo.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        if lo.is_empty() {
            self.pos = start;
            return None;
        }
        let min: usize = lo.parse().ok()?;
        match self.peek() {
            Some('}') => {
                self.pos += 1;
                Some((min, Some(min)))
            }
            Some(',') => {
                self.pos += 1;
                let mut hi = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        hi.push(c);
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                if self.peek() != Some('}') {
                    self.pos = start;
                    return None;
                }
                self.pos += 1;
                if hi.is_empty() {
                    Some((min, None))
                } else {
                    let hiv: usize = hi.parse().ok()?;
                    Some((min, Some(hiv)))
                }
            }
            _ => {
                self.pos = start;
                None
            }
        }
    }

    fn parse_atom(&mut self, dotall: bool) -> Result<Ast, String> {
        let c = self.next().ok_or("模式提前结束")?;
        match c {
            '(' => {
                if self.peek() == Some('?') {
                    self.pos += 1;
                    match self.next() {
                        Some(':') => {
                            let body = self.parse_alt(dotall)?;
                            self.expect(')')?;
                            Ok(Ast::Grp(Box::new(body)))
                        }
                        Some('s') => {
                            self.expect(':')?;
                            let body = self.parse_alt(true)?;
                            self.expect(')')?;
                            Ok(Ast::Grp(Box::new(body)))
                        }
                        Some('=') => {
                            let body = self.parse_alt(dotall)?;
                            self.expect(')')?;
                            Ok(Ast::Look { sub: Box::new(body), neg: false })
                        }
                        Some('!') => {
                            let body = self.parse_alt(dotall)?;
                            self.expect(')')?;
                            Ok(Ast::Look { sub: Box::new(body), neg: true })
                        }
                        Some('<') => match self.next() {
                            Some('=') => {
                                let body = self.parse_alt(dotall)?;
                                self.expect(')')?;
                                let width = ast_width(&body).ok_or("后顾须定宽")?;
                                Ok(Ast::Behind { sub: Box::new(body), neg: false, width })
                            }
                            Some('!') => {
                                let body = self.parse_alt(dotall)?;
                                self.expect(')')?;
                                let width = ast_width(&body).ok_or("后顾须定宽")?;
                                Ok(Ast::Behind { sub: Box::new(body), neg: true, width })
                            }
                            _ => Err("未知组形 (?<...")?,
                        },
                        _ => Err("不支持该 (?...) 组形".to_string()),
                    }
                } else {
                    self.ngroups += 1;
                    let n = self.ngroups;
                    let body = self.parse_alt(dotall)?;
                    self.expect(')')?;
                    Ok(Ast::Cap(n, Box::new(body)))
                }
            }
            '[' => self.parse_class(),
            '.' => Ok(Ast::Dot(dotall)),
            '^' => Ok(Ast::Start),
            '$' => Ok(Ast::Dollar),
            '\\' => self.parse_escape(dotall),
            ')' | '*' | '+' | '?' => Err("悬空量词或括号".to_string()),
            other => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::Ch(other)] })),
        }
    }

    fn parse_escape(&mut self, _dotall: bool) -> Result<Ast, String> {
        let c = self.next().ok_or("模式提前结束")?;
        match c {
            'd' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::D] })),
            'D' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::ND] })),
            'w' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::W] })),
            'W' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::NW] })),
            's' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::S] })),
            'S' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::NS] })),
            'b' => Ok(Ast::WB(true)),
            'B' => Ok(Ast::WB(false)),
            'A' => Ok(Ast::Start),
            'Z' => Ok(Ast::End),
            'x' => {
                let mut h = String::new();
                for _ in 0..2 {
                    if let Some(hc) = self.peek() {
                        if hc.is_ascii_hexdigit() {
                            h.push(hc);
                            self.pos += 1;
                        }
                    }
                }
                if h.len() != 2 {
                    return Err("\\x 须两位十六进制".to_string());
                }
                let code = u32::from_str_radix(&h, 16).unwrap();
                let ch = char::from_u32(code).ok_or("非法 \\x 码点")?;
                Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::Ch(ch)] }))
            }
            '1'..='9' => {
                let mut num = c.to_digit(10).unwrap() as usize;
                if let Some(c2) = self.peek() {
                    if let Some(d2) = c2.to_digit(10) {
                        let two = num * 10 + d2 as usize;
                        if two <= self.ngroups.max(9) {
                            num = two;
                            self.pos += 1;
                        }
                    }
                }
                Ok(Ast::Ref(num))
            }
            other => match esc_char(other) {
                Some(ch) => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::Ch(ch)] })),
                None => Err(format!("不支持转义 \\{other}")),
            },
        }
    }

    fn parse_class(&mut self) -> Result<Ast, String> {
        // 已消费 '['
        let neg = if self.peek() == Some('^') {
            self.pos += 1;
            true
        } else {
            false
        };
        let mut items: Vec<CItem> = vec![];
        let mut first = true;
        loop {
            let c = self.next().ok_or("字符集未闭合")?;
            if c == ']' && !first {
                break;
            }
            first = false;
            let lo = self.class_item_head(c)?;
            match lo {
                ClassHead::Sh(it) => {
                    items.push(it);
                    continue;
                }
                ClassHead::Ch(lo_c) => {
                    if self.peek() == Some('-') && self.peek2().is_some() && self.peek2() != Some(']')
                    {
                        self.pos += 1; // '-'
                        let hc = self.next().unwrap();
                        match self.class_item_head(hc)? {
                            ClassHead::Ch(hi_c) => {
                                if lo_c > hi_c {
                                    return Err("字符区间逆序".to_string());
                                }
                                items.push(CItem::Rg(lo_c, hi_c));
                            }
                            ClassHead::Sh(_) => return Err("区间端不得为类简写".to_string()),
                        }
                    } else {
                        items.push(CItem::Ch(lo_c));
                    }
                }
            }
        }
        Ok(Ast::Cl(CClass { neg, items }))
    }

    fn class_item_head(&mut self, c: char) -> Result<ClassHead, String> {
        if c != '\\' {
            return Ok(ClassHead::Ch(c));
        }
        let e = self.next().ok_or("模式提前结束")?;
        match e {
            'd' => Ok(ClassHead::Sh(CItem::D)),
            'D' => Ok(ClassHead::Sh(CItem::ND)),
            'w' => Ok(ClassHead::Sh(CItem::W)),
            'W' => Ok(ClassHead::Sh(CItem::NW)),
            's' => Ok(ClassHead::Sh(CItem::S)),
            'S' => Ok(ClassHead::Sh(CItem::NS)),
            'x' => {
                let mut h = String::new();
                for _ in 0..2 {
                    if let Some(hc) = self.peek() {
                        if hc.is_ascii_hexdigit() {
                            h.push(hc);
                            self.pos += 1;
                        }
                    }
                }
                if h.len() != 2 {
                    return Err("\\x 须两位十六进制".to_string());
                }
                let code = u32::from_str_radix(&h, 16).unwrap();
                Ok(ClassHead::Ch(char::from_u32(code).ok_or("非法 \\x 码点")?))
            }
            other => match esc_char(other) {
                Some(ch) => Ok(ClassHead::Ch(ch)),
                None => Err(format!("类内不支持转义 \\{other}")),
            },
        }
    }
}

enum ClassHead {
    Ch(char),
    Sh(CItem),
}

/// 形可零宽匹配判定（零宽无限重复编译期拒）。
fn ast_nullable(a: &Ast) -> bool {
    match a {
        Ast::Cl(_) | Ast::Dot(_) | Ast::Ref(_) => false,
        Ast::Grp(s) | Ast::Cap(_, s) => ast_nullable(s),
        Ast::Seq(items) => items.iter().all(ast_nullable),
        Ast::Alt(alts) => alts.iter().any(ast_nullable),
        Ast::Rpt { min, .. } => *min == 0,
        Ast::Look { .. } | Ast::Behind { .. } | Ast::Start | Ast::End | Ast::Dollar | Ast::WB(_) => {
            true
        }
    }
}

/// 定宽后顾的固定宽度。
fn ast_width(a: &Ast) -> Option<usize> {
    match a {
        Ast::Cl(_) | Ast::Dot(_) => Some(1),
        Ast::Grp(s) | Ast::Cap(_, s) => ast_width(s),
        Ast::Seq(items) => items.iter().try_fold(0usize, |acc, x| Some(acc + ast_width(x)?)),
        Ast::Alt(alts) => {
            let w = ast_width(alts.first()?)?;
            alts.iter().all(|x| ast_width(x) == Some(w)).then_some(w)
        }
        Ast::Rpt { sub, min, max, .. } => match max {
            Some(mx) if mx == min => Some(min * ast_width(sub)?),
            _ => None,
        },
        Ast::Look { .. } | Ast::Behind { .. } | Ast::Start | Ast::End | Ast::Dollar | Ast::WB(_) => {
            Some(0)
        }
        Ast::Ref(_) => None,
    }
}

#[derive(Clone, Debug)]
enum Inst {
    Cl(CClass),
    Dot(bool),
    Split(usize, usize),
    Jmp(usize),
    Save(usize),
    Match,
    Ref(usize),
    Look { pi: usize, neg: bool },
    Behind { pi: usize, neg: bool, width: usize },
    Start,
    End,
    Dollar,
    WB(bool),
}

#[derive(Debug)]
struct Prog {
    insts: Vec<Inst>,
    subs: Vec<Prog>,
    ngroups: usize,
}

fn compile_ast(a: &Ast, prog: &mut Vec<Inst>, subs: &mut Vec<Prog>) -> Result<(), String> {
    match a {
        Ast::Cl(cc) => prog.push(Inst::Cl(cc.clone())),
        Ast::Dot(all) => prog.push(Inst::Dot(*all)),
        Ast::Start => prog.push(Inst::Start),
        Ast::End => prog.push(Inst::End),
        Ast::Dollar => prog.push(Inst::Dollar),
        Ast::WB(w) => prog.push(Inst::WB(*w)),
        Ast::Ref(n) => prog.push(Inst::Ref(*n)),
        Ast::Grp(s) => compile_ast(s, prog, subs)?,
        Ast::Cap(n, s) => {
            prog.push(Inst::Save(2 * n));
            compile_ast(s, prog, subs)?;
            prog.push(Inst::Save(2 * n + 1));
        }
        Ast::Seq(items) => {
            for it in items {
                compile_ast(it, prog, subs)?;
            }
        }
        Ast::Alt(alts) => {
            let mut jmps = vec![];
            for (i, alt) in alts.iter().enumerate() {
                if i + 1 < alts.len() {
                    let s = prog.len();
                    prog.push(Inst::Split(0, 0));
                    compile_ast(alt, prog, subs)?;
                    jmps.push(prog.len());
                    prog.push(Inst::Jmp(0));
                    let after = prog.len();
                    prog[s] = Inst::Split(s + 1, after);
                } else {
                    compile_ast(alt, prog, subs)?;
                }
            }
            let end = prog.len();
            for j in jmps {
                prog[j] = Inst::Jmp(end);
            }
        }
        Ast::Rpt { sub, min, max, lazy } => {
            if (*max != Some(*min)) && ast_nullable(sub) {
                return Err("零宽体不可无界重复".to_string());
            }
            for _ in 0..*min {
                compile_ast(sub, prog, subs)?;
            }
            match max {
                None => {
                    let l1 = prog.len();
                    prog.push(Inst::Split(0, 0));
                    compile_ast(sub, prog, subs)?;
                    prog.push(Inst::Jmp(l1));
                    let l2 = prog.len();
                    prog[l1] = if *lazy { Inst::Split(l2, l1 + 1) } else { Inst::Split(l1 + 1, l2) };
                }
                Some(mx) => {
                    let extra = mx - min;
                    let mut splits = vec![];
                    for _ in 0..extra {
                        let s = prog.len();
                        prog.push(Inst::Split(0, 0));
                        splits.push(s);
                        compile_ast(sub, prog, subs)?;
                    }
                    let end = prog.len();
                    for s in splits {
                        prog[s] =
                            if *lazy { Inst::Split(end, s + 1) } else { Inst::Split(s + 1, end) };
                    }
                }
            }
        }
        Ast::Look { sub, neg } => {
            let mut sp = vec![];
            let mut ss = vec![];
            compile_ast(sub, &mut sp, &mut ss)?;
            sp.push(Inst::Match);
            subs.push(Prog { insts: sp, subs: ss, ngroups: 0 });
            prog.push(Inst::Look { pi: subs.len() - 1, neg: *neg });
        }
        Ast::Behind { sub, neg, width } => {
            let mut sp = vec![];
            let mut ss = vec![];
            compile_ast(sub, &mut sp, &mut ss)?;
            sp.push(Inst::Match);
            subs.push(Prog { insts: sp, subs: ss, ngroups: 0 });
            prog.push(Inst::Behind { pi: subs.len() - 1, neg: *neg, width: *width });
        }
    }
    Ok(())
}

fn compile_pattern(pat: &str) -> Result<Prog, String> {
    let mut p = PatParser { chars: pat.chars().collect(), pos: 0, ngroups: 0 };
    let ast = p.parse_alt(false)?;
    if p.pos != p.chars.len() {
        return Err("括号不配平".to_string());
    }
    check_refs(&ast, p.ngroups)?;
    let mut prog = vec![];
    let mut subs = vec![];
    compile_ast(&ast, &mut prog, &mut subs)?;
    prog.push(Inst::Match);
    Ok(Prog { insts: prog, subs, ngroups: p.ngroups })
}

fn check_refs(a: &Ast, ngroups: usize) -> Result<(), String> {
    match a {
        Ast::Ref(n) => {
            if *n == 0 || *n > ngroups {
                Err(format!("回引组号越界 \\{n}"))
            } else {
                Ok(())
            }
        }
        Ast::Grp(s) | Ast::Cap(_, s) => check_refs(s, ngroups),
        Ast::Seq(items) => items.iter().try_for_each(|x| check_refs(x, ngroups)),
        Ast::Alt(alts) => alts.iter().try_for_each(|x| check_refs(x, ngroups)),
        Ast::Rpt { sub, .. } => check_refs(sub, ngroups),
        Ast::Look { sub, .. } | Ast::Behind { sub, .. } => check_refs(sub, ngroups),
        _ => Ok(()),
    }
}

/// 锚定回溯执行：从 start 起整匹配则返回终位。
fn exec(prog: &Prog, input: &[char], start: usize) -> Option<usize> {
    let mut saves: Vec<Option<usize>> = vec![None; 2 * (prog.ngroups + 1)];
    let mut log: Vec<(usize, Option<usize>)> = Vec::new();
    let mut bt: Vec<(usize, usize, usize)> = Vec::new();
    let mut pc = 0usize;
    let mut pos = start;
    loop {
        let inst = prog.insts[pc].clone();
        let mut matched = true;
        match inst {
            Inst::Match => return Some(pos),
            Inst::Cl(cc) => {
                if pos < input.len() && cc.m(input[pos]) {
                    pos += 1;
                    pc += 1;
                } else {
                    matched = false;
                }
            }
            Inst::Dot(all) => {
                if pos < input.len() && (all || input[pos] != '\n') {
                    pos += 1;
                    pc += 1;
                } else {
                    matched = false;
                }
            }
            Inst::Split(a, b) => {
                bt.push((b, pos, log.len()));
                pc = a;
            }
            Inst::Jmp(a) => pc = a,
            Inst::Save(n) => {
                log.push((n, saves[n]));
                saves[n] = Some(pos);
                pc += 1;
            }
            Inst::Ref(n) => match (saves[2 * n], saves[2 * n + 1]) {
                (Some(s), Some(e)) if s <= e && e <= input.len() => {
                    let seg = &input[s..e];
                    if pos + seg.len() <= input.len() && input[pos..pos + seg.len()] == *seg {
                        pos += seg.len();
                        pc += 1;
                    } else {
                        matched = false;
                    }
                }
                _ => matched = false,
            },
            Inst::Look { pi, neg } => {
                let ok = exec(&prog.subs[pi], input, pos).is_some();
                if ok != neg {
                    pc += 1;
                } else {
                    matched = false;
                }
            }
            Inst::Behind { pi, neg, width } => {
                let ok = pos >= width
                    && exec(&prog.subs[pi], input, pos - width).map(|e| e == pos).unwrap_or(false);
                if ok != neg {
                    pc += 1;
                } else {
                    matched = false;
                }
            }
            Inst::Start => {
                if pos == 0 {
                    pc += 1;
                } else {
                    matched = false;
                }
            }
            Inst::End => {
                if pos == input.len() {
                    pc += 1;
                } else {
                    matched = false;
                }
            }
            Inst::Dollar => {
                if pos == input.len() || (pos + 1 == input.len() && input[pos] == '\n') {
                    pc += 1;
                } else {
                    matched = false;
                }
            }
            Inst::WB(want) => {
                let before = pos > 0 && is_w(input[pos - 1]);
                let after = pos < input.len() && is_w(input[pos]);
                if (before != after) == want {
                    pc += 1;
                } else {
                    matched = false;
                }
            }
        }
        if !matched {
            match bt.pop() {
                Some((pc2, pos2, ll)) => {
                    while log.len() > ll {
                        let (n, old) = log.pop().unwrap();
                        saves[n] = old;
                    }
                    pc = pc2;
                    pos = pos2;
                }
                None => return None,
            }
        }
    }
}

// ================================ 词法 ================================

#[derive(Clone, Debug)]
struct Token {
    name: String,
    text: String,
    line_start: usize,
    line_end: usize,
    start: usize,
    end: usize,
}

fn hex4(slice: &[char]) -> Option<u32> {
    if slice.len() != 4 || !slice.iter().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let s: String = slice.iter().collect();
    u32::from_str_radix(&s, 16).ok()
}

/// text 操作即 dequote 剥首尾各一字符后按表解转义（lexer._apply_text 对表）。
fn apply_text(raw: &[char], spec: Option<&Value>) -> String {
    let spec = match spec {
        Some(s) if s.is_object() => s,
        _ => return raw.iter().collect(),
    };
    let mut value: Vec<char> = raw.to_vec();
    if spec.get("dequote").and_then(|v| v.as_bool()).unwrap_or(false) {
        if value.len() >= 2 {
            value = value[1..value.len() - 1].to_vec();
        } else {
            value = vec![];
        }
    }
    let mut u4 = spec.get("u4").and_then(|v| v.as_bool()).unwrap_or(false);
    let unescape = match spec.get("unescape") {
        Some(u) if u.is_object() => {
            if u.get("u4").and_then(|v| v.as_bool()) == Some(true) {
                u4 = true;
            }
            Some(u)
        }
        _ => {
            if !u4 {
                return value.iter().collect();
            }
            None
        }
    };
    let mut out = String::new();
    let mut i = 0usize;
    let n = value.len();
    while i < n {
        let ch = value[i];
        if ch != '\\' || i + 1 >= n {
            out.push(ch);
            i += 1;
            continue;
        }
        let nxt = value[i + 1];
        if u4 && nxt == 'u' && i + 6 <= n {
            if let Some(mut code) = hex4(&value[i + 2..i + 6]) {
                let mut j = i + 6;
                if (0xD800..=0xDBFF).contains(&code)
                    && j + 6 <= n
                    && value[j] == '\\'
                    && value[j + 1] == 'u'
                {
                    if let Some(low) = hex4(&value[j + 2..j + 6]) {
                        if (0xDC00..=0xDFFF).contains(&low) {
                            code = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                            j += 6;
                        }
                    }
                }
                // 孤立代理对出 U+FFFD（落差五）。
                out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                i = j;
                continue;
            }
        }
        let target = unescape.and_then(|u| u.get(nxt.to_string())).and_then(|v| v.as_str());
        if let Some(t) = target {
            out.push_str(t);
            i += 2;
            continue;
        }
        out.push(ch);
        i += 1;
    }
    out
}

/// 按有序词法表扫描产记号流，全表不中产 error 记号吞一字符继续（lexer.lex 对表）。
fn lex(text: &[char], tokens_spec: &Value) -> R<Vec<Token>> {
    let entries = tokens_spec.as_array().ok_or_else(|| {
        ToolErr::Tool("词法表须列表".to_string())
    })?;
    let mut compiled: Vec<(String, Prog, bool, Option<Value>)> = vec![];
    for entry in entries {
        let obj = match entry.as_object() {
            Some(o) => o,
            None => return tool_err(format!("词法表元非对象 {entry}")),
        };
        let name = match obj.get("name").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return tool_err(format!("词法名须非空字符串")),
        };
        let pattern = match obj.get("pattern").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => return tool_err(format!("词法模式须字符串")),
        };
        let prog = compile_pattern(&pattern)
            .map_err(|e| ToolErr::Tool(format!("词法模式编译失败 {name}: {e}")))?;
        compiled.push((
            name,
            prog,
            obj.get("skip").and_then(|v| v.as_bool()).unwrap_or(false),
            obj.get("text").cloned(),
        ));
    }
    let mut out: Vec<Token> = vec![];
    let mut pos = 0usize;
    let mut line = 1usize;
    let n = text.len();
    while pos < n {
        let mut hit: Option<(usize, usize, bool, Option<Value>)> = None; // (end, idx, skip, tspec)
        for (idx, (name, prog, skip, tspec)) in compiled.iter().enumerate() {
            if let Some(end) = exec(prog, text, pos) {
                if end > pos {
                    hit = Some((end, idx, *skip, tspec.clone()));
                    let _ = name;
                    break;
                }
            }
        }
        let (end, _idx, skip, tspec) = match hit {
            Some(h) => h,
            None => {
                let ch = text[pos];
                out.push(Token {
                    name: "error".to_string(),
                    text: ch.to_string(),
                    line_start: line,
                    line_end: line,
                    start: pos,
                    end: pos + 1,
                });
                if ch == '\n' {
                    line += 1;
                }
                pos += 1;
                continue;
            }
        };
        let raw: Vec<char> = text[pos..end].to_vec();
        if !skip {
            let line_end = line + raw[..raw.len() - 1].iter().filter(|c| **c == '\n').count();
            out.push(Token {
                name: compiled[_idx].0.clone(),
                text: apply_text(&raw, tspec.as_ref()),
                line_start: line,
                line_end,
                start: pos,
                end,
            });
        }
        line += raw.iter().filter(|c| **c == '\n').count();
        pos = end;
    }
    Ok(out)
}

// ================================ 树与 PEG ================================

#[derive(Clone, Copy, Debug, PartialEq)]
enum NType {
    Rule,
    Token,
    Error,
}

#[derive(Clone, Debug)]
struct Node {
    t: NType,
    name: String,
    text: String,
    children: Vec<Node>,
    line_start: Option<usize>,
    line_end: Option<usize>,
    start_char: Option<usize>,
    end_char: Option<usize>,
    expected: String,
    got: Option<String>,
    message: String,
    start_tok: Option<usize>,
    end_tok: Option<usize>,
}

impl Node {
    fn new(t: NType) -> Self {
        Node {
            t,
            name: String::new(),
            text: String::new(),
            children: vec![],
            line_start: None,
            line_end: None,
            start_char: None,
            end_char: None,
            expected: String::new(),
            got: None,
            message: String::new(),
            start_tok: None,
            end_tok: None,
        }
    }

    /// 冻结序列化形态（tree.to_dict 对表，字段序固定）。
    fn to_value(&self) -> Value {
        let mut m = Map::new();
        match self.t {
            NType::Token => {
                m.insert("type".into(), json!("token"));
                m.insert("name".into(), json!(self.name));
                m.insert("text".into(), json!(self.text));
                m.insert("line_start".into(), json!(self.line_start));
                m.insert("line_end".into(), json!(self.line_end));
            }
            NType::Error => {
                m.insert("type".into(), json!("error"));
                m.insert("expected".into(), json!(self.expected));
                m.insert("got".into(), json!(self.got));
                m.insert("message".into(), json!(self.message));
                m.insert("line_start".into(), json!(self.line_start));
                m.insert("line_end".into(), json!(self.line_end));
            }
            NType::Rule => {
                m.insert("type".into(), json!("rule"));
                m.insert("name".into(), json!(self.name));
                m.insert(
                    "children".into(),
                    Value::Array(self.children.iter().map(|c| c.to_value()).collect()),
                );
                m.insert("line_start".into(), json!(self.line_start));
                m.insert("line_end".into(), json!(self.line_end));
            }
        }
        Value::Object(m)
    }
}

/// 节点结构等值（Python dataclass __eq__ 对表）。
fn node_eq(a: &Node, b: &Node) -> bool {
    a.t == b.t
        && a.name == b.name
        && a.text == b.text
        && a.line_start == b.line_start
        && a.line_end == b.line_end
        && a.start_char == b.start_char
        && a.end_char == b.end_char
        && a.expected == b.expected
        && a.got == b.got
        && a.message == b.message
        && a.start_tok == b.start_tok
        && a.end_tok == b.end_tok
        && a.children.len() == b.children.len()
        && a.children.iter().zip(b.children.iter()).all(|(x, y)| node_eq(x, y))
}

const DEPTH_LIMIT: usize = 100_000;

struct MR {
    nodes: Vec<Node>,
    pos: usize,
}

/// 组合子解释器，零语言知识，行为全由数据树支配（peg._Interp 对表）。
struct Interp<'a> {
    tokens: &'a [Token],
    rules: &'a Map<String, Value>,
    far_pos: Option<usize>,
    far_exp: Vec<String>,
    maxpos: usize,
    depth: usize,
    blew: bool,
}

impl<'a> Interp<'a> {
    fn note_fail(&mut self, pos: usize, expect: &str) {
        match self.far_pos {
            Some(fp) if pos > fp => {
                self.far_pos = Some(pos);
                self.far_exp = vec![expect.to_string()];
            }
            Some(fp) if pos == fp => {
                if !self.far_exp.iter().any(|e| e == expect) {
                    self.far_exp.push(expect.to_string());
                }
            }
            None => {
                self.far_pos = Some(pos);
                self.far_exp = vec![expect.to_string()];
            }
            _ => {}
        }
    }

    fn enter(&mut self) -> bool {
        self.depth += 1;
        if self.depth > DEPTH_LIMIT {
            self.blew = true;
            self.depth -= 1;
            false
        } else {
            true
        }
    }
    fn leave(&mut self) {
        self.depth -= 1;
    }

    fn rule_form(&self, name: &str) -> (Option<&'a Value>, Option<Vec<String>>) {
        let spec = match self.rules.get(name) {
            Some(s) if s.is_object() => s,
            _ => return (None, None),
        };
        if spec.get("node").is_some() || spec.get("sync").is_some() {
            let form = spec.get("node");
            let sync = match spec.get("sync") {
                Some(Value::Array(a)) if !a.is_empty() => Some(
                    a.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .collect::<Vec<String>>(),
                ),
                _ => None,
            };
            (form, sync)
        } else {
            (Some(spec), None)
        }
    }

    /// 规则匹配，sync 规则失败且已消费即跳读恢复（match_rule 对表）。
    fn match_rule(&mut self, name: &str, pos: usize) -> Option<MR> {
        if !self.enter() {
            return None;
        }
        let r = self.match_rule_inner(name, pos);
        self.leave();
        r
    }

    fn match_rule_inner(&mut self, name: &str, pos: usize) -> Option<MR> {
        let (form, sync) = self.rule_form(name);
        let form = form?;
        let prev_max = self.maxpos;
        match self.match_form(form, pos) {
            Some(mut res) => {
                let node = self.rule_node(name, std::mem::take(&mut res.nodes), pos, res.pos);
                Some(MR { nodes: vec![node], pos: res.pos })
            }
            None => {
                if let Some(sync) = sync {
                    if self.maxpos > prev_max {
                        let total = self.tokens.len();
                        let mut target = pos + 1;
                        while target < total && !sync.contains(&self.tokens[target].name) {
                            target += 1;
                        }
                        let err = self.make_error(
                            format!("规则 {name} 失败跳读至同步点恢复"),
                            pos,
                            target,
                            name,
                        );
                        let node = self.rule_node(name, vec![err], pos, target);
                        return Some(MR { nodes: vec![node], pos: target });
                    }
                }
                None
            }
        }
    }

    /// 组合子五类与叶两类的匹配，失败即 None 不抛异常（match 对表）。
    fn match_form(&mut self, form: &Value, pos: usize) -> Option<MR> {
        if !self.enter() {
            return None;
        }
        let r = self.match_form_inner(form, pos);
        self.leave();
        r
    }

    fn match_form_inner(&mut self, form: &Value, pos: usize) -> Option<MR> {
        let obj = form.as_object()?;
        if let Some(subs) = obj.get("seq") {
            let subs = subs.as_array()?;
            let mut nodes: Vec<Node> = vec![];
            let mut p = pos;
            for sub in subs {
                let r = self.match_form(sub, p)?;
                nodes.extend(r.nodes);
                p = r.pos;
            }
            return Some(MR { nodes, pos: p });
        }
        if let Some(alts) = obj.get("choice") {
            // 有序选择：按声明序逐个尝试取首个成功者，全败不回溯（ALG-012）。
            let alts = alts.as_array()?;
            for alt in alts {
                if let Some(r) = self.match_form(alt, pos) {
                    return Some(r);
                }
            }
            return None;
        }
        if let Some(rep) = obj.get("repeat") {
            let rep = rep.as_object()?;
            let sub = rep.get("sub")?;
            let minimum = rep.get("min").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            let maximum = rep.get("max").and_then(|v| v.as_u64()).map(|v| v as usize);
            let mut nodes: Vec<Node> = vec![];
            let mut p = pos;
            let mut count = 0usize;
            while maximum.map(|m| count < m).unwrap_or(true) {
                let r = match self.match_form(sub, p) {
                    Some(r) => r,
                    None => break,
                };
                nodes.extend(r.nodes);
                if r.pos == p {
                    break;
                }
                p = r.pos;
                count += 1;
            }
            if count < minimum {
                return None;
            }
            return Some(MR { nodes, pos: p });
        }
        if let Some(sub) = obj.get("opt") {
            if let Some(r) = self.match_form(sub, pos) {
                return Some(r);
            }
            return Some(MR { nodes: vec![], pos });
        }
        if let Some(ah) = obj.get("ahead") {
            let ah = ah.as_object()?;
            let sub = ah.get("sub")?;
            let ok = self.match_form(sub, pos).is_some();
            let ok = if ah.get("neg").and_then(|v| v.as_bool()).unwrap_or(false) {
                !ok
            } else {
                ok
            };
            if ok {
                return Some(MR { nodes: vec![], pos });
            }
            return None;
        }
        if let Some(tname) = obj.get("token").and_then(|v| v.as_str()) {
            if pos < self.tokens.len() && self.tokens[pos].name == tname {
                if pos + 1 > self.maxpos {
                    self.maxpos = pos + 1;
                }
                let tk = &self.tokens[pos];
                let mut leaf = Node::new(NType::Token);
                leaf.name = tname.to_string();
                leaf.text = tk.text.clone();
                leaf.line_start = Some(tk.line_start);
                leaf.line_end = Some(tk.line_end);
                leaf.start_char = Some(tk.start);
                leaf.end_char = Some(tk.end);
                return Some(MR { nodes: vec![leaf], pos: pos + 1 });
            }
            self.note_fail(pos, tname);
            return None;
        }
        if let Some(rname) = obj.get("rule").and_then(|v| v.as_str()) {
            return self.match_rule(rname, pos);
        }
        None
    }

    /// 产错误节点，期望取最远失败位记录，行位与字符区间取跨度（make_error 对表）。
    fn make_error(
        &self,
        message: String,
        span_lo: usize,
        span_hi: usize,
        fallback_expected: &str,
    ) -> Node {
        let expected = if self.far_exp.is_empty() {
            fallback_expected.to_string()
        } else {
            self.far_exp.join(" | ")
        };
        let far = self.far_pos.unwrap_or(span_lo);
        let got = self.tokens.get(far).map(|t| t.name.clone());
        let (ls, le, sc, ec) = if span_hi > span_lo {
            let first = &self.tokens[span_lo];
            let last = &self.tokens[span_hi - 1];
            (Some(first.line_start), Some(last.line_end), Some(first.start), Some(last.end))
        } else if let Some(tk) = self.tokens.get(far) {
            (Some(tk.line_start), Some(tk.line_end), Some(tk.start), Some(tk.end))
        } else {
            (None, None, None, None)
        };
        let mut node = Node::new(NType::Error);
        node.expected = expected;
        node.got = got;
        node.message = message;
        node.line_start = ls;
        node.line_end = le;
        node.start_char = sc;
        node.end_char = ec;
        node
    }

    /// 规则节点记区间即记号半开区间与字符区间与首末行（_rule_node 对表）。
    fn rule_node(&self, name: &str, children: Vec<Node>, pos: usize, end: usize) -> Node {
        let mut node = Node::new(NType::Rule);
        node.name = name.to_string();
        node.children = children;
        if end > pos {
            let first = &self.tokens[pos];
            let last = &self.tokens[end - 1];
            node.line_start = Some(first.line_start);
            node.line_end = Some(last.line_end);
            node.start_char = Some(first.start);
            node.end_char = Some(last.end);
            node.start_tok = Some(pos);
            node.end_tok = Some(end);
        }
        node
    }
}

const EMPTY_INPUT_MESSAGE: &str = "输入为空记号流为零";

/// 记号流产生式解析产根规则节点，残缺截断空输入皆产树零异常（peg._parse 对表）。
fn parse_tokens(tokens: &[Token], grammar: &Value) -> R<Node> {
    let empty = Map::new();
    let rules: &Map<String, Value> = grammar
        .get("rules")
        .and_then(|v| v.as_object())
        .unwrap_or(&empty);
    let start = grammar
        .get("start")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("");
    let mut interp = Interp {
        tokens,
        rules,
        far_pos: None,
        far_exp: vec![],
        maxpos: 0,
        depth: 0,
        blew: false,
    };
    let mut root = Node::new(NType::Rule);
    root.name = start.to_string();
    let total = tokens.len();
    if total == 0 {
        let mut err = Node::new(NType::Error);
        err.expected = start.to_string();
        err.got = None;
        err.message = EMPTY_INPUT_MESSAGE.to_string();
        root.children.push(err);
        return Ok(root);
    }
    let mut pos = 0usize;
    while pos < total {
        match interp.match_rule(start, pos) {
            Some(res) if res.pos > pos => {
                for wrapper in res.nodes {
                    root.children.extend(wrapper.children);
                }
                pos = res.pos;
            }
            Some(_) => {
                let err = interp.make_error("顶层空匹配跳一记号重启".to_string(), pos, pos + 1, start);
                root.children.push(err);
                pos += 1;
            }
            None => {
                let err = interp.make_error("顶层失败跳一记号重启".to_string(), pos, pos + 1, start);
                root.children.push(err);
                pos += 1;
            }
        }
    }
    if interp.blew {
        return tool_err(format!("规则匹配深度超闸 {DEPTH_LIMIT}"));
    }
    root.line_start = Some(tokens[0].line_start);
    root.line_end = Some(tokens[total - 1].line_end);
    root.start_char = Some(tokens[0].start);
    root.end_char = Some(tokens[total - 1].end);
    root.start_tok = Some(0);
    root.end_tok = Some(total);
    Ok(root)
}

fn parse_text(text: &[char], tokens_spec: &Value, grammar: &Value) -> R<(Vec<Token>, Node)> {
    let tokens = lex(text, tokens_spec)?;
    let tree = parse_tokens(&tokens, grammar)?;
    Ok((tokens, tree))
}

// ================================ 条目投影 ================================

fn content_hash(text: Option<&str>) -> String {
    let data = text.unwrap_or("").as_bytes();
    sha256_hex(data)
}

fn stable_id(path: &str, carrier: &str, kind: &str, seq: usize, chash: &str) -> String {
    let material = format!("{path}\u{0}{carrier}:{kind}\u{0}{seq}\u{0}{chash}");
    sha256_hex(material.as_bytes())
}

/// 先序遍历规则节点即文档序，不降入错误节点（entries._walk_rules 对表）。
fn walk_rules<'a>(node: &'a Node, chain: &mut Vec<&'a Node>, out: &mut Vec<(&'a Node, Vec<&'a Node>)>) {
    if node.t == NType::Rule {
        out.push((node, chain.clone()));
        for child in &node.children {
            chain.push(node);
            walk_rules(child, chain, out);
            chain.pop();
        }
    }
}

fn key_token_text<'a>(node: &'a Node, key_token: &str) -> Option<&'a str> {
    node.children.iter().find_map(|c| {
        if c.t == NType::Token && c.name == key_token {
            Some(c.text.as_str())
        } else {
            None
        }
    })
}

/// 路径合成即键段点连加元素规则序号缀（entries._compose_path 对表）。
fn compose_path(
    node: &Node,
    chain: &[&Node],
    key_token: &str,
    element_rules: &HashSet<String>,
) -> Option<String> {
    let mut nodes: Vec<&Node> = chain.to_vec();
    nodes.push(node);
    let mut parts: Vec<String> = vec![];
    for (i, n) in nodes.iter().enumerate() {
        if let Some(key) = key_token_text(n, key_token) {
            parts.push(key.to_string());
            continue;
        }
        if !element_rules.contains(&n.name) || i == 0 {
            continue;
        }
        let parent = nodes[i - 1];
        let sibs: Vec<&Node> =
            parent.children.iter().filter(|c| c.t == NType::Rule && c.name == n.name).collect();
        let idx = sibs.iter().position(|s| node_eq(s, n)).unwrap_or(0);
        if !parts.is_empty() {
            let last = parts.last_mut().unwrap();
            *last = format!("{last}[{idx}]");
        } else {
            parts.push(format!("[{idx}]"));
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("."))
    }
}

/// 末个直接子为词法叶取其文本，规则节点或无子即 null（entries._scalar_text 对表）。
fn scalar_text(node: &Node) -> Option<String> {
    let last = node.children.last()?;
    if last.t == NType::Token {
        Some(last.text.clone())
    } else {
        None
    }
}

fn slice_chars(chars: &[char], a: usize, b: usize) -> String {
    let a = a.min(chars.len());
    let b = b.min(chars.len()).max(a);
    chars[a..b].iter().collect()
}

fn strip_line(line: &str, strip_prefix: &[String], line_strip_max: usize) -> String {
    let (mut body, eol) = match line.strip_suffix('\n') {
        Some(b) => (b, "\n"),
        None => (line, ""),
    };
    let mut hit = false;
    for pref in strip_prefix {
        if let Some(rest) = body.strip_prefix(pref.as_str()) {
            body = rest;
            hit = true;
            break;
        }
    }
    if !hit && line_strip_max > 0 {
        let stripped = body.trim_start_matches([' ', '\t']);
        if body.len() - stripped.len() <= line_strip_max {
            body = stripped;
        }
    }
    format!("{body}{eol}")
}

fn join_rules(src: &[char], node: &Node, spec: &Value) -> R<String> {
    let names: HashSet<String> = spec
        .get("rules")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let sep = spec.get("sep").and_then(|v| v.as_str()).unwrap_or(" ");
    let strip_prefix: Vec<String> = spec
        .get("strip_prefix")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let line_strip_max = spec
        .get("line_strip_max")
        .and_then(|v| v.as_i64())
        .unwrap_or(0)
        .max(0) as usize;

    let span = |n: &Node| -> String {
        let a = n.start_char.unwrap_or(0);
        let b = n.end_char.unwrap_or(0);
        let mut s = slice_chars(src, a, b);
        if !strip_prefix.is_empty() || line_strip_max > 0 {
            let lines = py_splitlines(&s, true);
            s = lines.iter().map(|l| strip_line(l, &strip_prefix, line_strip_max)).collect();
        }
        s
    };

    if spec.get("direct").and_then(|v| v.as_bool()).unwrap_or(false) {
        let parts: Vec<String> = node
            .children
            .iter()
            .filter(|c| c.t == NType::Rule && names.contains(&c.name))
            .map(span)
            .collect();
        return Ok(parts.join(sep));
    }
    let mut out: Vec<String> = vec![];
    fn rec<'a>(
        n: &'a Node,
        names: &HashSet<String>,
        out: &mut Vec<String>,
        span: &dyn Fn(&Node) -> String,
    ) {
        if n.t != NType::Rule {
            return;
        }
        if names.contains(&n.name) {
            out.push(span(n));
            return;
        }
        for c in &n.children {
            rec(c, names, out, span);
        }
    }
    for c in &node.children {
        rec(c, &names, &mut out, &span);
    }
    Ok(out.join(sep))
}

fn join_tokens(node: &Node, name: &str, sep: &str) -> String {
    let mut out: Vec<String> = vec![];
    fn rec(n: &Node, name: &str, out: &mut Vec<String>) {
        if n.t == NType::Token {
            if n.name == name {
                out.push(n.text.clone());
            }
            return;
        }
        for c in &n.children {
            rec(c, name, out);
        }
    }
    rec(node, name, &mut out);
    out.join(sep)
}

fn raw_lines(
    src: &[char],
    node: &Node,
    head: usize,
    tail: usize,
    strip_prefix: &[String],
) -> String {
    let span = slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0));
    let mut lines = py_splitlines(&span, true);
    if head > 0 {
        lines = if head >= lines.len() { vec![] } else { lines[head..].to_vec() };
    }
    if tail > 0 {
        lines = if lines.len() > tail { lines[..lines.len() - tail].to_vec() } else { vec![] };
    }
    if !strip_prefix.is_empty() {
        let mut out: Vec<String> = vec![];
        for line in lines {
            let (mut body, eol) = match line.strip_suffix('\n') {
                Some(b) => (b.to_string(), "\n"),
                None => (line.clone(), ""),
            };
            for pref in strip_prefix {
                if let Some(rest) = body.strip_prefix(pref.as_str()) {
                    body = rest.to_string();
                    break;
                }
            }
            out.push(format!("{body}{eol}"));
        }
        lines = out;
    }
    lines.concat()
}

/// 文本派生机（entries._entry_text 对表）。
fn entry_text(src: &[char], node: &Node, text_form: Option<&Value>) -> R<Option<String>> {
    match text_form {
        None => Ok(Some(slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0)))),
        Some(Value::String(s)) if s == "raw" => {
            Ok(Some(slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0))))
        }
        Some(Value::String(s)) if s == "compact" => {
            let span = slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0));
            // 落差七：大整数与 NaN/Infinity 解析败即兜底原样出。
            match serde_json::from_str::<Value>(&span) {
                Ok(v) => Ok(Some(v.to_string())),
                Err(_) => Ok(Some(span)),
            }
        }
        Some(Value::Object(o)) if o.contains_key("scalar_text") => Ok(scalar_text(node)),
        Some(Value::Object(o)) if o.contains_key("raw_lines") => {
            let spec = &o["raw_lines"];
            let head = spec.get("head").and_then(|v| v.as_i64()).unwrap_or(0).max(0) as usize;
            let tail = spec.get("tail").and_then(|v| v.as_i64()).unwrap_or(0).max(0) as usize;
            let prefixes: Vec<String> = spec
                .get("strip_prefix")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            Ok(Some(raw_lines(src, node, head, tail, &prefixes)))
        }
        Some(Value::Object(o)) if o.contains_key("join_tokens") => {
            let spec = &o["join_tokens"];
            let name = spec.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let sep = spec.get("sep").and_then(|v| v.as_str()).unwrap_or(" ");
            Ok(Some(join_tokens(node, name, sep)))
        }
        Some(Value::Object(o)) if o.contains_key("join_rules") => {
            Ok(Some(join_rules(src, node, &o["join_rules"])?))
        }
        _ => tool_err(format!("text_form 未知形: {text_form:?}")),
    }
}

/// 树到条目流即映射命中先序产出，seq 按文件按 kind 文档序起一（project_entries 对表）。
fn project_entries(
    src: &[char],
    tree: &Node,
    mapping_entries: &[Value],
    path: &str,
    carrier: &str,
) -> R<Vec<Value>> {
    let mut mapping: HashMap<&str, Vec<&Value>> = HashMap::new();
    for ent in mapping_entries {
        let rule = ent.get("rule").and_then(|v| v.as_str()).unwrap_or("");
        mapping.entry(rule).or_default().push(ent);
    }
    let mut counters: HashMap<String, usize> = HashMap::new();
    let mut walked: Vec<(&Node, Vec<&Node>)> = vec![];
    walk_rules(tree, &mut vec![], &mut walked);
    let mut entries: Vec<Value> = vec![];
    for (node, chain) in walked {
        let Some(ents) = mapping.get(node.name.as_str()) else { continue };
        for ent in ents {
            let kind = ent.get("kind").and_then(|v| v.as_str()).unwrap_or("");
            let counter = counters.entry(kind.to_string()).or_insert(0);
            *counter += 1;
            let seq = *counter;
            let name: Option<String> = match ent.get("name_from") {
                None | Some(Value::Null) => None,
                Some(Value::String(nf)) => key_token_text(node, nf).map(|s| s.to_string()),
                Some(Value::Object(o)) if o.contains_key("path") => {
                    let spec = &o["path"];
                    let key_token = spec.get("key_token").and_then(|v| v.as_str()).unwrap_or("");
                    let element_rules: HashSet<String> = spec
                        .get("element_rules")
                        .and_then(|v| v.as_array())
                        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    compose_path(node, &chain, key_token, &element_rules)
                }
                Some(Value::Object(o)) if o.contains_key("join_tokens") => {
                    let spec = &o["join_tokens"];
                    let name = spec.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let sep = spec.get("sep").and_then(|v| v.as_str()).unwrap_or(" ");
                    Some(join_tokens(node, name, sep))
                }
                Some(Value::Object(o)) if o.contains_key("text") => {
                    entry_text(src, node, Some(&o["text"]))?.filter(|s| !s.is_empty())
                }
                Some(other) => return tool_err(format!("name_from 未知形: {other:?}")),
            };
            let text = entry_text(src, node, ent.get("text_form"))?;
            let chash = content_hash(text.as_deref());
            let lines_null = ent.get("lines").and_then(|v| v.as_str()) == Some("null");
            let mut m = Map::new();
            m.insert("id".into(), json!(stable_id(path, carrier, kind, seq, &chash)));
            m.insert("path".into(), json!(path));
            m.insert("carrier".into(), json!(carrier));
            m.insert("kind".into(), json!(kind));
            m.insert("name".into(), json!(name));
            m.insert("seq".into(), json!(seq));
            m.insert(
                "line_start".into(),
                json!(if lines_null { None } else { node.line_start }),
            );
            m.insert(
                "line_end".into(),
                json!(if lines_null { None } else { node.line_end }),
            );
            m.insert("content_hash".into(), json!(chash));
            m.insert("text".into(), json!(text));
            entries.push(Value::Object(m));
        }
    }
    Ok(entries)
}

fn render_ndjson(entries: &[Value]) -> String {
    let mut out = String::new();
    for e in entries {
        out.push_str(&e.to_string());
        out.push('\n');
    }
    out
}

// ================================ 语言包 ================================

const COMBINATOR_KEYS: [&str; 7] = ["seq", "choice", "repeat", "opt", "ahead", "token", "rule"];

fn check_keys(obj: &Map<String, Value>, allowed: &[&str], wh: &str, problems: &mut Vec<String>) {
    let extra: BTreeSet<&str> =
        obj.keys().map(|s| s.as_str()).collect::<BTreeSet<&str>>().difference(
            &allowed.iter().copied().collect::<BTreeSet<&str>>(),
        )
    .copied()
    .collect();
    if !extra.is_empty() {
        let items: Vec<String> = extra.iter().map(|s| s.to_string()).collect();
        problems.push(format!("{wh} 含未知字段 {}", py_list(&items)));
    }
}

fn is_int(v: Option<&Value>) -> bool {
    matches!(v, Some(Value::Number(n)) if n.is_i64())
}

fn check_token_entry(tok: &Value, idx: usize, problems: &mut Vec<String>) {
    let wh = format!("tokens[{idx}]");
    let Some(obj) = tok.as_object() else {
        problems.push(format!("{wh} 非对象"));
        return;
    };
    check_keys(obj, &["name", "pattern", "skip", "text"], &wh, problems);
    let name_ok = obj.get("name").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
    if !name_ok {
        problems.push(format!("{wh} name 须非空字符串"));
    }
    let Some(pattern) = obj.get("pattern").and_then(|v| v.as_str()) else {
        problems.push(format!("{wh} pattern 须字符串"));
        return;
    };
    if let Err(e) = compile_pattern(pattern) {
        problems.push(format!("{wh} pattern 不合 re 语法: {e}"));
    }
    if let Some(skip) = obj.get("skip") {
        if !skip.is_boolean() {
            problems.push(format!("{wh} skip 须布尔"));
        }
    }
    if let Some(text) = obj.get("text") {
        let Some(t) = text.as_object() else {
            problems.push(format!("{wh} text 须对象"));
            return;
        };
        check_keys(t, &["dequote", "unescape", "u4"], &format!("{wh}.text"), problems);
        if let Some(dq) = t.get("dequote") {
            if !dq.is_boolean() {
                problems.push(format!("{wh}.text.dequote 须布尔"));
            }
        }
        if let Some(un) = t.get("unescape") {
            let Some(u) = un.as_object() else {
                problems.push(format!("{wh}.text.unescape 须对象"));
                return;
            };
            for (key, target) in u {
                if key.chars().count() != 1 {
                    problems.push(format!("{wh}.text.unescape 键 {} 须单字符", py_repr(key)));
                }
                if !target.is_string() {
                    problems.push(format!("{wh}.text.unescape 值 {} 须字符串", py_repr(key)));
                }
            }
        }
        if let Some(u4) = t.get("u4") {
            if !u4.is_boolean() {
                problems.push(format!("{wh}.text.u4 须布尔"));
            }
        }
    }
}

fn check_tokens(doc: &Value, problems: &mut Vec<String>) {
    let Some(obj) = doc.as_object() else {
        problems.push("tokens.json 顶层须对象".to_string());
        return;
    };
    check_keys(obj, &["version", "revision", "tokens"], "tokens.json", problems);
    let version = obj.get("version").and_then(|v| v.as_f64());
    if !matches!(version, Some(1.0) | Some(2.0)) {
        problems.push("tokens.json version 须为 1 或 2".to_string());
    }
    let Some(toks) = obj.get("tokens").and_then(|v| v.as_array()) else {
        problems.push("tokens.json tokens 须列表".to_string());
        return;
    };
    for (idx, tok) in toks.iter().enumerate() {
        check_token_entry(tok, idx, problems);
    }
}

fn check_form(form: &Value, wh: &str, problems: &mut Vec<String>) {
    let Some(obj) = form.as_object() else {
        problems.push(format!("{wh} 须对象"));
        return;
    };
    let hit: Vec<&str> = COMBINATOR_KEYS.iter().filter(|k| obj.contains_key(**k)).map(|k| *k).collect();
    if hit.len() != 1 {
        let keys: Vec<String> = obj.keys().cloned().collect();
        let mut sorted_keys = keys;
        sorted_keys.sort();
        problems.push(format!("{wh} 须恰一个组合子键，得 {}", py_list(&sorted_keys)));
        return;
    }
    let key = hit[0];
    match key {
        "seq" | "choice" => {
            let Some(items) = obj.get(key).and_then(|v| v.as_array()) else {
                problems.push(format!("{wh}.{key} 须列表"));
                return;
            };
            for (idx, sub) in items.iter().enumerate() {
                check_form(sub, &format!("{wh}.{key}[{idx}]"), problems);
            }
        }
        "repeat" => {
            let Some(rep) = obj.get(key).as_ref().and_then(|v| v.as_object()) else {
                problems.push(format!("{wh}.repeat 须对象"));
                return;
            };
            check_keys(rep, &["min", "max", "sub"], &format!("{wh}.repeat"), problems);
            let min_ok = is_int(rep.get("min"))
                && rep.get("min").and_then(|v| v.as_i64()).unwrap_or(0) >= 0;
            if !min_ok {
                problems.push(format!("{wh}.repeat.min 须非负整数"));
            }
            let max_v = rep.get("max");
            if !max_v.map(|v| v.is_null()).unwrap_or(true)
                && !is_int(max_v.filter(|v| !v.is_null()))
            {
                problems.push(format!("{wh}.repeat.max 须整数或 null"));
            }
            if let (Some(mn), Some(mx)) = (rep.get("min").and_then(|v| v.as_i64()), max_v.and_then(|v| v.as_i64())) {
                if mx < mn {
                    problems.push(format!("{wh}.repeat.max 不得小于 min"));
                }
            }
            match rep.get("sub") {
                None => problems.push(format!("{wh}.repeat 缺 sub")),
                Some(sub) => check_form(sub, &format!("{wh}.repeat.sub"), problems),
            }
        }
        "opt" => {
            check_form(obj.get(key).unwrap(), &format!("{wh}.opt"), problems);
        }
        "ahead" => {
            let Some(ah) = obj.get(key).as_ref().and_then(|v| v.as_object()) else {
                problems.push(format!("{wh}.ahead 须对象"));
                return;
            };
            check_keys(ah, &["neg", "sub"], &format!("{wh}.ahead"), problems);
            if !ah.get("neg").map(|v| v.is_boolean()).unwrap_or(false) {
                problems.push(format!("{wh}.ahead.neg 须布尔"));
            }
            match ah.get("sub") {
                None => problems.push(format!("{wh}.ahead 缺 sub")),
                Some(sub) => check_form(sub, &format!("{wh}.ahead.sub"), problems),
            }
        }
        "token" => {
            let ok = obj.get(key).and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
            if !ok {
                problems.push(format!("{wh}.token 须非空字符串"));
            }
        }
        _ => {
            let ok = obj.get(key).and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
            if !ok {
                problems.push(format!("{wh}.rule 须非空字符串"));
            }
        }
    }
}

fn check_grammar(doc: &Value, problems: &mut Vec<String>) {
    let Some(obj) = doc.as_object() else {
        problems.push("grammar.json 顶层须对象".to_string());
        return;
    };
    check_keys(obj, &["version", "revision", "start", "rules"], "grammar.json", problems);
    let version = obj.get("version").and_then(|v| v.as_f64());
    if !matches!(version, Some(1.0) | Some(2.0)) {
        problems.push("grammar.json version 须为 1 或 2".to_string());
    }
    let start_ok = obj.get("start").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
    if !start_ok {
        problems.push("grammar.json start 须非空字符串".to_string());
    }
    let Some(rules) = obj.get("rules").and_then(|v| v.as_object()) else {
        problems.push("grammar.json rules 须映射".to_string());
        return;
    };
    for (name, value) in rules {
        if name.is_empty() {
            problems.push("grammar.json 规则名须非空".to_string());
            continue;
        }
        let wh = format!("rules.{name}");
        let Some(v) = value.as_object() else {
            problems.push(format!("{wh} 须对象"));
            continue;
        };
        if v.contains_key("node") || v.contains_key("sync") {
            check_keys(v, &["node", "sync"], &wh, problems);
            if !v.contains_key("node") {
                problems.push(format!("{wh} 缺 node"));
            } else {
                check_form(&v["node"], &format!("{wh}.node"), problems);
            }
            if let Some(sync) = v.get("sync") {
                let ok = sync
                    .as_array()
                    .map(|a| !a.is_empty() && a.iter().all(|x| x.as_str().map(|s| !s.is_empty()).unwrap_or(false)))
                    .unwrap_or(false);
                let ok_empty_ok = sync.as_array().map(|a| a.iter().all(|x| x.as_str().map(|s| !s.is_empty()).unwrap_or(false))).unwrap_or(false);
                let _ = ok;
                if !sync.is_array() || !ok_empty_ok {
                    problems.push(format!("{wh}.sync 须词法名列表"));
                }
            }
        } else {
            check_form(value, &wh, problems);
        }
    }
}

fn check_text_form(text_form: &Value, wh: &str, problems: &mut Vec<String>) {
    if let Some(s) = text_form.as_str() {
        if s != "raw" && s != "compact" {
            problems.push(format!("{wh}.text_form 枚举不合 {}", py_repr(s)));
        }
        return;
    }
    if let Some(o) = text_form.as_object() {
        if o.len() == 1 && o.contains_key("scalar_text") && o["scalar_text"].is_object() {
            return;
        }
        if o.len() == 1 && o.contains_key("raw_lines") && o["raw_lines"].is_object() {
            let rl = o["raw_lines"].as_object().unwrap();
            check_keys(rl, &["head", "tail", "strip_prefix"], &format!("{wh}.text_form.raw_lines"), problems);
            for side in ["head", "tail"] {
                let ok = rl.get(side).and_then(|v| v.as_u64()).is_some();
                if !ok {
                    problems.push(format!("{wh}.text_form.raw_lines.{side} 须非负整数"));
                }
            }
            let sp_ok = rl
                .get("strip_prefix")
                .map(|v| {
                    v.as_array()
                        .map(|a| a.iter().all(|x| x.as_str().map(|s| !s.is_empty()).unwrap_or(false)))
                        .unwrap_or(false)
                })
                .unwrap_or(true);
            if !sp_ok {
                problems.push(format!(
                    "{wh}.text_form.raw_lines.strip_prefix 须非空字符串列表"
                ));
            }
            return;
        }
        if o.len() == 1 && o.contains_key("join_tokens") && o["join_tokens"].is_object() {
            let jt = o["join_tokens"].as_object().unwrap();
            check_keys(jt, &["name", "sep"], &format!("{wh}.text_form.join_tokens"), problems);
            let name_ok = jt.get("name").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
            if !name_ok {
                problems.push(format!("{wh}.text_form.join_tokens.name 须非空字符串"));
            }
            if let Some(sep) = jt.get("sep") {
                if !sep.is_string() {
                    problems.push(format!("{wh}.text_form.join_tokens.sep 须字符串"));
                }
            }
            return;
        }
        if o.len() == 1 && o.contains_key("join_rules") && o["join_rules"].is_object() {
            let jr = o["join_rules"].as_object().unwrap();
            check_keys(
                jr,
                &["rules", "sep", "strip_prefix", "line_strip_max", "direct"],
                &format!("{wh}.text_form.join_rules"),
                problems,
            );
            let rules_ok = jr
                .get("rules")
                .map(|v| {
                    v.as_array()
                        .map(|a| !a.is_empty() && a.iter().all(|x| x.as_str().map(|s| !s.is_empty()).unwrap_or(false)))
                        .unwrap_or(false)
                })
                .unwrap_or(false);
            if !rules_ok {
                problems.push(format!("{wh}.text_form.join_rules.rules 须非空规则名列表"));
            }
            if let Some(sep) = jr.get("sep") {
                if !sep.is_string() {
                    problems.push(format!("{wh}.text_form.join_rules.sep 须字符串"));
                }
            }
            let sp_ok = jr
                .get("strip_prefix")
                .map(|v| {
                    v.as_array()
                        .map(|a| a.iter().all(|x| x.as_str().map(|s| !s.is_empty()).unwrap_or(false)))
                        .unwrap_or(false)
                })
                .unwrap_or(true);
            if !sp_ok {
                problems.push(format!("{wh}.text_form.join_rules.strip_prefix 须非空字符串列表"));
            }
            let lsm_ok = jr
                .get("line_strip_max")
                .map(|v| v.as_u64().is_some())
                .unwrap_or(true);
            if !lsm_ok {
                problems.push(format!("{wh}.text_form.join_rules.line_strip_max 须非负整数"));
            }
            if let Some(d) = jr.get("direct") {
                if !d.is_boolean() {
                    problems.push(format!("{wh}.text_form.join_rules.direct 须布尔"));
                }
            }
            return;
        }
    }
    problems.push(format!("{wh}.text_form 形不合 {text_form}"));
}

fn check_name_from(name_from: &Value, wh: &str, problems: &mut Vec<String>) {
    if let Some(s) = name_from.as_str() {
        if s.is_empty() {
            problems.push(format!("{wh}.name_from 字符串须非空"));
        }
        return;
    }
    if let Some(o) = name_from.as_object() {
        if o.len() == 1 && o.contains_key("path") && o["path"].is_object() {
            let path = o["path"].as_object().unwrap();
            check_keys(path, &["key_token", "element_rules"], &format!("{wh}.name_from.path"), problems);
            let kt_ok = path
                .get("key_token")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false);
            if !kt_ok {
                problems.push(format!("{wh}.name_from.path.key_token 须非空字符串"));
            }
            let er_ok = path
                .get("element_rules")
                .map(|v| {
                    v.as_array()
                        .map(|a| a.iter().all(|x| x.is_string()))
                        .unwrap_or(false)
                })
                .unwrap_or(true);
            if !er_ok {
                problems.push(format!("{wh}.name_from.path.element_rules 须规则名列表"));
            }
            return;
        }
        if o.len() == 1 && o.contains_key("join_tokens") && o["join_tokens"].is_object() {
            let jt = o["join_tokens"].as_object().unwrap();
            check_keys(jt, &["name", "sep"], &format!("{wh}.name_from.join_tokens"), problems);
            let name_ok = jt.get("name").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
            if !name_ok {
                problems.push(format!("{wh}.name_from.join_tokens.name 须非空字符串"));
            }
            if let Some(sep) = jt.get("sep") {
                if !sep.is_string() {
                    problems.push(format!("{wh}.name_from.join_tokens.sep 须字符串"));
                }
            }
            return;
        }
        if o.len() == 1 && o.contains_key("text") {
            check_text_form(&o["text"], &format!("{wh}.name_from.text"), problems);
            return;
        }
    }
    problems.push(format!("{wh}.name_from 形不合 {name_from}"));
}

fn check_mapping(doc: &Value, problems: &mut Vec<String>) {
    let Some(obj) = doc.as_object() else {
        problems.push("mapping.json 顶层须对象".to_string());
        return;
    };
    check_keys(obj, &["version", "revision", "entries"], "mapping.json", problems);
    let version = obj.get("version").and_then(|v| v.as_f64());
    if !matches!(version, Some(1.0) | Some(2.0)) {
        problems.push("mapping.json version 须为 1 或 2".to_string());
    }
    let Some(entries) = obj.get("entries").and_then(|v| v.as_array()) else {
        problems.push("mapping.json entries 须列表".to_string());
        return;
    };
    for (idx, entry) in entries.iter().enumerate() {
        let wh = format!("entries[{idx}]");
        let Some(e) = entry.as_object() else {
            problems.push(format!("{wh} 非对象"));
            continue;
        };
        check_keys(e, &["rule", "kind", "name_from", "text_form", "lines"], &wh, problems);
        let rule_ok = e.get("rule").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
        if !rule_ok {
            problems.push(format!("{wh}.rule 须非空字符串"));
        }
        let kind_ok = e.get("kind").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
        if !kind_ok {
            problems.push(format!("{wh}.kind 须非空字符串"));
        }
        if let Some(nf) = e.get("name_from") {
            check_name_from(nf, &wh, problems);
        }
        if let Some(tf) = e.get("text_form") {
            check_text_form(tf, &wh, problems);
        }
        if let Some(lines) = e.get("lines") {
            let ok = lines.as_str().map(|s| s == "node" || s == "null").unwrap_or(false);
            if !ok {
                problems.push(format!("{wh}.lines 枚举不合 {lines}"));
            }
        }
    }
}

fn read_pack_file(pack_dir: &Path, name: &str) -> R<Value> {
    let path = pack_dir.join(name);
    if !path.exists() {
        return pack_err(format!("{name} 缺失"));
    }
    let raw = match fs::read(&path) {
        Ok(r) => r,
        Err(e) => return pack_err(format!("{name} 不可读: {e}")),
    };
    let text = match String::from_utf8(raw) {
        Ok(t) => t,
        Err(e) => return pack_err(format!("{name} JSON 不合: {e}")),
    };
    serde_json::from_str(&text).map_err(|e| ToolErr::Pack(format!("{name} JSON 不合: {e}")))
}

/// 三件齐备与 schema 校验，逐条收集不首断（collect_schema_problems 对表）。
fn collect_schema_problems(pack_dir: &Path) -> (HashMap<String, Value>, Vec<String>) {
    let mut problems: Vec<String> = vec![];
    let mut docs: HashMap<String, Value> = HashMap::new();
    for (name, checker) in [
        ("tokens.json", 0),
        ("grammar.json", 1),
        ("mapping.json", 2),
    ] {
        match read_pack_file(pack_dir, name) {
            Ok(doc) => {
                let mut p = vec![];
                match checker {
                    0 => check_tokens(&doc, &mut p),
                    1 => check_grammar(&doc, &mut p),
                    _ => check_mapping(&doc, &mut p),
                }
                problems.extend(p);
                docs.insert(name.to_string(), doc);
            }
            Err(ToolErr::Pack(m)) => problems.push(m),
            Err(ToolErr::Tool(m)) => problems.push(m),
        }
    }
    (docs, problems)
}

struct LangPack {
    tokens: Value,
    grammar: Value,
    mapping: Value,
}

/// --pack 相对形包名补解析（SPEC-025 融回缺口 gap-packs-assets，DES-019：
/// 引擎位候选序消对围堰的运行时依赖）。候选序落地方案：现存路径形
/// （绝对形与现存相对形）原样直通居首——temp 自建包显式路径既有链同形
/// 零扰动，较 DES-019「exe 派生首位」字面取保守序，差异在此申报；裸包名
/// 沿 exe 祖先链逐级找 <root>/packs/parser/<pack>（target/debug 与
/// target/debug/deps 两种 exe 落位同覆），cwd 无关；全不中原样返回，交
/// load_pack 既有「语言包目录不存在或不可读」报错路径（退出码一），
/// 不静默不降级。
fn resolve_pack_arg(pack: &str) -> String {
    let direct = Path::new(pack);
    if direct.is_absolute() || direct.exists() {
        return pack.to_string();
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(bin_dir) = exe.parent() {
            for anc in bin_dir.ancestors() {
                let cand = anc.join("packs").join("parser").join(pack);
                if cand.is_dir() {
                    return cand.display().to_string();
                }
            }
        }
    }
    pack.to_string()
}

fn load_pack(pack_dir: &str) -> R<LangPack> {
    let pack_path = Path::new(pack_dir);
    if !pack_path.is_dir() {
        return pack_err(format!("语言包目录不存在或不可读 {pack_dir}"));
    }
    let (docs, problems) = collect_schema_problems(pack_path);
    if !problems.is_empty() {
        return pack_err(problems.join("; "));
    }
    Ok(LangPack {
        tokens: docs["tokens.json"].get("tokens").cloned().unwrap_or(Value::Null),
        grammar: docs["grammar.json"].clone(),
        mapping: docs["mapping.json"].get("entries").cloned().unwrap_or(Value::Null),
    })
}

// ================================ lint ================================

/// 枚举形内引用（lint._form_refs 对表）。
fn form_refs(form: &Value) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut tokens: BTreeSet<String> = BTreeSet::new();
    let mut rules: BTreeSet<String> = BTreeSet::new();
    let Some(obj) = form.as_object() else {
        return (tokens, rules);
    };
    if let Some(t) = obj.get("token").and_then(|v| v.as_str()) {
        tokens.insert(t.to_string());
    }
    if let Some(r) = obj.get("rule").and_then(|v| v.as_str()) {
        rules.insert(r.to_string());
    }
    let items = obj.get("seq").or_else(|| obj.get("choice")).and_then(|v| v.as_array());
    if let Some(items) = items {
        for sub in items {
            let (t, r) = form_refs(sub);
            tokens.extend(t);
            rules.extend(r);
        }
    }
    if let Some(rep) = obj.get("repeat").and_then(|v| v.as_object()) {
        let (t, r) = form_refs(rep.get("sub").unwrap_or(&Value::Null));
        tokens.extend(t);
        rules.extend(r);
    }
    if let Some(sub) = obj.get("opt") {
        let (t, r) = form_refs(sub);
        tokens.extend(t);
        rules.extend(r);
    }
    if let Some(ah) = obj.get("ahead").and_then(|v| v.as_object()) {
        let (t, r) = form_refs(ah.get("sub").unwrap_or(&Value::Null));
        tokens.extend(t);
        rules.extend(r);
    }
    (tokens, rules)
}

fn rule_form<'a>(rules: &'a Map<String, Value>, name: &str) -> &'a Value {
    match rules.get(name) {
        Some(v) if v.is_object() && v.get("node").map(|n| n.is_object()).unwrap_or(false) => {
            &v["node"]
        }
        Some(v) => v,
        None => &Value::Null,
    }
}

/// 形可空判定，规则引用递归带环即视为可空防死循环（lint._nullable 对表）。
fn nullable(
    form: &Value,
    rules: &Map<String, Value>,
    cache: &mut HashMap<String, bool>,
    visiting: &mut HashSet<String>,
) -> bool {
    let Some(obj) = form.as_object() else {
        return true;
    };
    if obj.contains_key("token") {
        return false;
    }
    if let Some(name) = obj.get("rule").and_then(|v| v.as_str()) {
        if let Some(c) = cache.get(name) {
            return *c;
        }
        if visiting.contains(name) {
            return true;
        }
        visiting.insert(name.to_string());
        let v = nullable(rule_form(rules, name), rules, cache, visiting);
        visiting.remove(name);
        cache.insert(name.to_string(), v);
        return v;
    }
    if let Some(items) = obj.get("seq").and_then(|v| v.as_array()) {
        return items.iter().filter(|x| x.is_object()).all(|x| nullable(x, rules, cache, visiting));
    }
    if let Some(items) = obj.get("choice").and_then(|v| v.as_array()) {
        return items.iter().filter(|x| x.is_object()).any(|x| nullable(x, rules, cache, visiting));
    }
    if let Some(rep) = obj.get("repeat").and_then(|v| v.as_object()) {
        return rep.get("min").and_then(|v| v.as_u64()).unwrap_or(0) == 0;
    }
    true
}

/// 形在零消费前位可触发的规则引用集合（lint._first_rule_refs 对表）。
fn first_rule_refs(
    form: &Value,
    rules: &Map<String, Value>,
    cache: &mut HashMap<String, bool>,
    visiting: &mut HashSet<String>,
) -> BTreeSet<String> {
    let mut refs: BTreeSet<String> = BTreeSet::new();
    let Some(obj) = form.as_object() else {
        return refs;
    };
    if obj.contains_key("token") {
        return refs;
    }
    if let Some(name) = obj.get("rule").and_then(|v| v.as_str()) {
        refs.insert(name.to_string());
        let sub = rule_form(rules, name);
        if !visiting.contains(name) && sub.is_object() {
            visiting.insert(name.to_string());
            refs.extend(first_rule_refs(sub, rules, cache, visiting));
            visiting.remove(name);
        }
        return refs;
    }
    if let Some(items) = obj.get("seq").and_then(|v| v.as_array()) {
        for sub in items {
            if !sub.is_object() {
                continue;
            }
            refs.extend(first_rule_refs(sub, rules, cache, visiting));
            if !nullable(sub, rules, cache, &mut visiting.clone()) {
                break;
            }
        }
        return refs;
    }
    if let Some(items) = obj.get("choice").and_then(|v| v.as_array()) {
        for sub in items {
            if sub.is_object() {
                refs.extend(first_rule_refs(sub, rules, cache, visiting));
            }
        }
        return refs;
    }
    if let Some(rep) = obj.get("repeat").and_then(|v| v.as_object()) {
        refs.extend(first_rule_refs(rep.get("sub").unwrap_or(&Value::Null), rules, cache, visiting));
        return refs;
    }
    if let Some(sub) = obj.get("opt") {
        return first_rule_refs(sub, rules, cache, visiting);
    }
    if let Some(ah) = obj.get("ahead").and_then(|v| v.as_object()) {
        return first_rule_refs(ah.get("sub").unwrap_or(&Value::Null), rules, cache, visiting);
    }
    refs
}

/// 有向图找环（lint._find_cycles 对表）。
fn find_cycles(graph: &HashMap<String, BTreeSet<String>>) -> (BTreeSet<String>, Vec<String>) {
    let mut in_cycle: BTreeSet<String> = BTreeSet::new();
    let mut examples: Vec<String> = vec![];
    let mut color: HashMap<String, u8> = HashMap::new();

    fn dfs(
        node: &str,
        stack: &mut Vec<String>,
        graph: &HashMap<String, BTreeSet<String>>,
        color: &mut HashMap<String, u8>,
        in_cycle: &mut BTreeSet<String>,
        examples: &mut Vec<String>,
    ) {
        color.insert(node.to_string(), 1);
        if let Some(nexts) = graph.get(node) {
            for nxt in nexts {
                match color.get(nxt).copied().unwrap_or(0) {
                    1 => {
                        let idx = stack.iter().position(|s| s == nxt).unwrap();
                        let mut cycle: Vec<String> = stack[idx..].to_vec();
                        cycle.push(nxt.clone());
                        for c in &cycle[..cycle.len() - 1] {
                            in_cycle.insert(c.clone());
                        }
                        if examples.len() < 8 {
                            examples.push(cycle.join(" -> "));
                        }
                    }
                    0 => {
                        stack.push(nxt.clone());
                        dfs(nxt, stack, graph, color, in_cycle, examples);
                        stack.pop();
                    }
                    _ => {}
                }
            }
        }
        color.insert(node.to_string(), 2);
    }

    let mut nodes: Vec<&String> = graph.keys().collect();
    nodes.sort();
    for node in nodes {
        if color.get(node.as_str()).copied().unwrap_or(0) == 0 {
            let mut stack = vec![node.clone()];
            dfs(node, &mut stack, graph, &mut color, &mut in_cycle, &mut examples);
        }
    }
    (in_cycle, examples)
}

/// lint 三查即 schema 合与引用闭合与条目映射可达（lint.lint_pack 对表）。
fn lint_pack(pack_dir: &str) -> R<Vec<String>> {
    let (docs, problems) = collect_schema_problems(Path::new(pack_dir));
    let mut findings = problems;
    let (Some(grammar), Some(mapping)) = (docs.get("grammar.json"), docs.get("mapping.json"))
    else {
        return Ok(findings);
    };
    let Some(rules) = grammar.get("rules").and_then(|v| v.as_object()) else {
        return Ok(findings);
    };
    let mut token_names: BTreeSet<String> = BTreeSet::new();
    if let Some(toks) = docs.get("tokens.json").and_then(|d| d.get("tokens")).and_then(|v| v.as_array())
    {
        for t in toks {
            if let Some(n) = t.get("name").and_then(|v| v.as_str()) {
                token_names.insert(n.to_string());
            }
        }
    }
    let start = grammar.get("start").and_then(|v| v.as_str()).unwrap_or("");

    let mut token_refs: BTreeSet<String> = BTreeSet::new();
    let mut rule_refs: BTreeSet<String> = BTreeSet::new();
    for (name, value) in rules {
        let form = rule_form(rules, name);
        let (t, r) = form_refs(form);
        token_refs.extend(t);
        rule_refs.extend(r);
        if let Some(sync) = value.get("sync").and_then(|v| v.as_array()) {
            for x in sync {
                if let Some(s) = x.as_str() {
                    token_refs.insert(s.to_string());
                }
            }
        }
    }
    for missing in &token_refs {
        if !token_names.contains(missing) {
            findings.push(format!("词法引用未定义 {missing}"));
        }
    }
    for missing in &rule_refs {
        if !rules.contains_key(missing.as_str()) {
            findings.push(format!("规则引用未定义 {missing}"));
        }
    }

    let mut reachable: BTreeSet<String> = BTreeSet::new();
    if rules.contains_key(start) {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        seen.insert(start.to_string());
        let mut frontier = vec![start.to_string()];
        while let Some(name) = frontier.pop() {
            let (_, refs) = form_refs(rule_form(rules, &name));
            for r in refs {
                if rules.contains_key(r.as_str()) && !seen.contains(&r) {
                    seen.insert(r.clone());
                    frontier.push(r);
                }
            }
        }
        reachable = seen.clone();
        for orphan in rules.keys() {
            if !seen.contains(orphan) {
                findings.push(format!("规则自 start 不可达 {orphan}"));
            }
        }
    } else if !start.is_empty() {
        findings.push(format!("start 规则未定义 {start}"));
    }

    let mut graph: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut cache: HashMap<String, bool> = HashMap::new();
    for name in rules.keys() {
        let refs = first_rule_refs(rule_form(rules, name), rules, &mut cache, &mut HashSet::new());
        graph.insert(name.clone(), refs);
    }
    let (in_cycle, examples) = find_cycles(&graph);
    for name in &in_cycle {
        findings.push(format!("规则左递归 {name}"));
    }
    for ex in examples {
        findings.push(format!("左递归环 {ex}"));
    }

    if let Some(entries) = mapping.get("entries").and_then(|v| v.as_array()) {
        for (idx, entry) in entries.iter().enumerate() {
            let Some(e) = entry.as_object() else { continue };
            let Some(rule) = e.get("rule").and_then(|v| v.as_str()) else { continue };
            if !rules.contains_key(rule) {
                findings.push(format!("条目 {idx} 规则未定义 {rule}"));
            } else if !reachable.contains(rule) {
                findings.push(format!("条目 {idx} 规则自 start 不可导 {rule}"));
            }
            if let Some(er) = e
                .get("name_from")
                .and_then(|v| v.get("path"))
                .and_then(|p| p.get("element_rules"))
                .and_then(|v| v.as_array())
            {
                for element in er {
                    if let Some(el) = element.as_str() {
                        if !rules.contains_key(el) {
                            findings.push(format!("条目 {idx} element_rules 规则未定义 {el}"));
                        }
                    }
                }
            }
        }
    }
    Ok(findings)
}

// ================================ vectors ================================

/// 金向量冻结与回归即逐字节复算（vectors.run_vectors 对表）。
fn run_vectors(pack_dir: &str, freeze: bool) -> R<Value> {
    let mode = if freeze { "冻结" } else { "回归" };
    let pack_path = Path::new(pack_dir);
    let in_dir = pack_path.join("vectors").join("in");
    let exp_dir = pack_path.join("vectors").join("expected");
    if !in_dir.is_dir() {
        return Ok(json!({
            "mode": mode,
            "pass": 0,
            "fail": [],
            "detail": "无 vectors 目录即零向量过",
        }));
    }
    let pack = load_pack(pack_dir)?;
    let carrier = fs::canonicalize(pack_path)
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_default();
    let mut case_names: Vec<String> = vec![];
    for p in fs::read_dir(&in_dir).map_err(|e| ToolErr::Tool(format!("vectors 目录不可读: {e}")))? {
        let p = p.map_err(|e| ToolErr::Tool(format!("vectors 目录不可读: {e}")))?;
        if p.path().is_file() {
            case_names.push(p.file_name().to_string_lossy().into_owned());
        }
    }
    case_names.sort();
    if freeze {
        fs::create_dir_all(&exp_dir).map_err(|e| ToolErr::Tool(format!("expected 不可建: {e}")))?;
    }
    let mut failed: Vec<Value> = vec![];
    let mut results_count = 0usize;
    for case in &case_names {
        results_count += 1;
        let raw = fs::read(in_dir.join(case))
            .map_err(|e| ToolErr::Tool(format!("向量不可读 {case}: {e}")))?;
        let src: Vec<char> = String::from_utf8_lossy(&raw).chars().collect();
        let (_, tree1) = parse_text(&src, &pack.tokens, &pack.grammar)?;
        let (_, tree2) = parse_text(&src, &pack.tokens, &pack.grammar)?;
        let parse_out = format!("{}\n", serde_json::to_string_pretty(&tree1.to_value()).unwrap());
        let double_ok = parse_out == format!("{}\n", serde_json::to_string_pretty(&tree2.to_value()).unwrap());
        let rel = format!("vectors/in/{case}");
        let entries = project_entries(&src, &tree1, pack.mapping.as_array().unwrap_or(&vec![]), &rel, &carrier)?;
        let entries_out = render_ndjson(&entries);
        let exp_parse = exp_dir.join(format!("{case}.parse.json"));
        let exp_entries = exp_dir.join(format!("{case}.entries.ndjson"));
        if freeze {
            fs::write(&exp_parse, parse_out.as_bytes())
                .map_err(|e| ToolErr::Tool(format!("期望件不可写: {e}")))?;
            fs::write(&exp_entries, entries_out.as_bytes())
                .map_err(|e| ToolErr::Tool(format!("期望件不可写: {e}")))?;
            continue;
        }
        let mut ok = double_ok;
        let mut detail: Vec<&str> = vec![];
        if !double_ok {
            detail.push("双跑不一致");
        }
        let parse_ok = fs::read(&exp_parse).map(|b| b == parse_out.as_bytes()).unwrap_or(false);
        if !parse_ok {
            ok = false;
            detail.push("parse 树逐字节不一致");
        }
        let entries_ok =
            fs::read(&exp_entries).map(|b| b == entries_out.as_bytes()).unwrap_or(false);
        if !entries_ok {
            ok = false;
            detail.push("entries 逐字节不一致");
        }
        if !ok {
            failed.push(json!({"case": case, "detail": detail.join("; ")}));
        }
    }
    Ok(json!({
        "mode": mode,
        "pass": results_count - failed.len(),
        "fail": failed,
        "detail": format!("cases={results_count}"),
    }))
}

// ================================ CLI ================================

/// Python json.dumps 默认分隔符（', '/': '）单行序列化（vectors stdout 对表）。
fn py_compact(v: &Value) -> String {
    match v {
        Value::Object(m) => {
            let parts: Vec<String> =
                m.iter().map(|(k, val)| format!("{}: {}", py_compact_str(k), py_compact(val))).collect();
            format!("{{{}}}", parts.join(", "))
        }
        Value::Array(a) => {
            let parts: Vec<String> = a.iter().map(py_compact).collect();
            format!("[{}]", parts.join(", "))
        }
        Value::String(s) => py_compact_str(s),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}

fn py_compact_str(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("parser: {msg}");
    eprintln!("用法: parser parse --pack <语言包目录> --in <输入>");
    eprintln!("      parser entries --pack <语言包目录> --in <输入> --out <输出>");
    eprintln!("      parser lint --pack <语言包目录>");
    eprintln!("      parser vectors --pack <语言包目录> [--freeze]");
    std::process::exit(2);
}

fn print_usage() -> ! {
    println!("parser {VERSION} —— 句读即空腹确定性解析工具");
    eprintln!("用法: parser <parse|entries|lint|vectors> [旗标]");
    std::process::exit(0);
}

struct Flags {
    pack: Option<String>,
    infile: Option<String>,
    outfile: Option<String>,
    freeze: bool,
}

fn take_flags(args: &[String], allow: &[&str]) -> Flags {
    let mut f = Flags { pack: None, infile: None, outfile: None, freeze: false };
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        let (key, inline_val) = match a.split_once('=') {
            Some((k, v)) => (k.to_string(), Some(v.to_string())),
            None => (a.clone(), None),
        };
        let known = matches!(key.as_str(), "--pack" | "--in" | "--out" | "--freeze" | "--help" | "-h")
            && allow.contains(&key.as_str());
        if key == "--help" || key == "-h" {
            print_usage();
        }
        if !known {
            usage_fail(&format!("未知旗标或参数: {a}"));
        }
        if key == "--freeze" {
            f.freeze = true;
            i += 1;
            continue;
        }
        let val = match inline_val {
            Some(v) => {
                i += 1;
                v
            }
            None => {
                i += 1;
                if i >= args.len() {
                    usage_fail(&format!("{key} 缺值"));
                }
                let v = args[i].clone();
                i += 1;
                v
            }
        };
        match key.as_str() {
            "--pack" => f.pack = Some(val),
            "--in" => f.infile = Some(val),
            _ => f.outfile = Some(val),
        }
    }
    f
}

fn read_input_chars(infile: &str) -> R<Vec<char>> {
    let raw = fs::read(infile).map_err(|e| ToolErr::Pack(format!("输入不可读 {infile}: {e}")))?;
    Ok(String::from_utf8_lossy(&raw).chars().collect())
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令");
    }
    let cmd = args[0].as_str();
    let rest = &args[1..];
    let work: R<()> = match cmd {
        "parse" => cmd_parse(rest),
        "entries" => cmd_entries(rest),
        "lint" => cmd_lint(rest),
        "vectors" => cmd_vectors(rest),
        _ => usage_fail(&format!("未知子命令: {cmd}")),
    };
    match work {
        Ok(()) => 0,
        Err(ToolErr::Pack(m)) => {
            eprintln!("parser: {m}");
            1
        }
        Err(ToolErr::Tool(m)) => {
            eprintln!("parser: 工具异常: {m}");
            2
        }
    }
}

fn cmd_parse(args: &[String]) -> R<()> {
    let f = take_flags(args, &["--pack", "--in", "--help", "-h"]);
    let pack_dir = match &f.pack {
        Some(p) => p,
        None => usage_fail("parse 缺 --pack（必填）"),
    };
    let infile = match &f.infile {
        Some(v) => v,
        None => usage_fail("parse 缺 --in（必填）"),
    };
    let pack = load_pack(&resolve_pack_arg(pack_dir))?;
    let src = read_input_chars(infile)?;
    let (_, tree) = parse_text(&src, &pack.tokens, &pack.grammar)?;
    println!("{}", serde_json::to_string_pretty(&tree.to_value()).unwrap());
    Ok(())
}

fn cmd_entries(args: &[String]) -> R<()> {
    let f = take_flags(args, &["--pack", "--in", "--out", "--help", "-h"]);
    let pack_dir = match &f.pack {
        Some(p) => p,
        None => usage_fail("entries 缺 --pack（必填）"),
    };
    let infile = match &f.infile {
        Some(v) => v,
        None => usage_fail("entries 缺 --in（必填）"),
    };
    let outfile = match &f.outfile {
        Some(v) => v,
        None => usage_fail("entries 缺 --out（必填）"),
    };
    let pack_dir = &resolve_pack_arg(pack_dir);
    let pack = load_pack(pack_dir)?;
    let src = read_input_chars(infile)?;
    let (_, tree) = parse_text(&src, &pack.tokens, &pack.grammar)?;
    let carrier = fs::canonicalize(Path::new(pack_dir))
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_default();
    let entries = project_entries(
        &src,
        &tree,
        pack.mapping.as_array().unwrap_or(&vec![]),
        infile,
        &carrier,
    )?;
    let out = Path::new(outfile);
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| ToolErr::Tool(format!("输出目录不可建: {e}")))?;
        }
    }
    fs::write(out, render_ndjson(&entries).as_bytes())
        .map_err(|e| ToolErr::Tool(format!("输出不可写 {outfile}: {e}")))?;
    eprintln!("entries: {} 条 -> {outfile}", entries.len());
    Ok(())
}

fn cmd_lint(args: &[String]) -> R<()> {
    let f = take_flags(args, &["--pack", "--help", "-h"]);
    let pack_dir = match &f.pack {
        Some(p) => p,
        None => usage_fail("lint 缺 --pack（必填）"),
    };
    match lint_pack(&resolve_pack_arg(pack_dir)) {
        Ok(findings) => {
            for finding in &findings {
                eprintln!("lint: {finding}");
            }
            if findings.is_empty() {
                Ok(())
            } else {
                // lint 的包不可载与有 finding 同归退出码一（cli_lint 对表）。
                std::process::exit(1);
            }
        }
        Err(ToolErr::Pack(m)) => {
            eprintln!("lint: 包不可载 {m}");
            std::process::exit(1);
        }
        Err(ToolErr::Tool(m)) => Err(ToolErr::Tool(m)),
    }
}

fn cmd_vectors(args: &[String]) -> R<()> {
    let f = take_flags(args, &["--pack", "--freeze", "--help", "-h"]);
    let pack_dir = match &f.pack {
        Some(p) => p,
        None => usage_fail("vectors 缺 --pack（必填）"),
    };
    let report = run_vectors(&resolve_pack_arg(pack_dir), f.freeze)?;
    if let Some(fails) = report.get("fail").and_then(|v| v.as_array()) {
        for fail in fails {
            eprintln!(
                "vectors {} 败: {} {}",
                report["mode"].as_str().unwrap_or(""),
                fail["case"].as_str().unwrap_or(""),
                fail["detail"].as_str().unwrap_or("")
            );
        }
    }
    println!("{}", py_compact(&report));
    if report.get("fail").and_then(|v| v.as_array()).map(|a| !a.is_empty()).unwrap_or(false) {
        std::process::exit(1);
    }
    Ok(())
}

fn main() {
    // 深文法递归需要大栈；panic 归工具异常退出码二。
    let code = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)
        .expect("线程启动失败")
        .join()
        .unwrap_or(2);
    std::process::exit(code);
}
