//! 引擎侧寻址（locator）命令行面 —— lease-mergeleg23-parallel 簇D 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/locator 0.1.0（src/locator/：pack.py、builder.py、
//! entry.py、identity.py、storage.py、query.py、stale.py、carriers/、cli.py），只读
//! 对表移植，围堰源码零改动。代码载体（.rs）内嵌句读核心紧凑副本（词法回溯 VM 加
//! PEG 加条目投影，与本批 src/bin/parser.rs 同源同算法）。
//!
//! CLI：`locator build --pack <域包> --root <语料根> --out <索引件>`、
//! `locator query --index <索引件> (--id|--term|--prefix|--ref|--domain) <值> [--pack] [--root]`、
//! `locator stale --pack <域包> --root <语料根> --index <索引件>`。
//! 退出码三值：0 = 建毕或查得或未陈旧、1 = 查无所指或陈旧超阈、2 = 工具异常。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. vectors 子命令未移植（金向量冻结回归属装饰面，本批由引擎集成测试承载）；
//!    调入即用法错退出码二。
//! 2. markdown 载体为行级 lite 解析（ATX 标题、围栏、列表项、段落、引用剥标、
//!    水平线与缩进码块与 HTML 块跳过），非 markdown-it-py 全量 CommonMark：
//!    setext 标题并入段落、行内标记不剥离（markdown-it inline content 同为原文，
//!    常形一致）、嵌套列表缩进语义为近似、列表项含尾随空行的跨度边界为近似。
//! 3. yaml 载体经 serde_yaml 结构遍历，line_start 与 line_end 恒 null（PyYAML
//!    compose 的 start_mark 与 end_mark 行位未复刻）；锚别名、多文档、自定义标签
//!    语义按 serde_yaml 行为。
//! 4. toml 载体键序按 toml crate 的 Map 字典序，非 tomllib 文档序（crate 无
//!    preserve_order 特性，依赖面零新增约束）；同本引擎双跑仍逐字节确定。datetime
//!    标量按 Display 字符串出（tomllib 为 datetime 对象、Python json.dumps 即抛、
//!    围堰该文件记入 parse_errors）。
//! 5. 代码载体语言包定位改环境变量 PARSER_PACKS_ROOT（指语言包父目录）加相对
//!    候选探测（首位 exe 派生引擎仓根 packs/parser/rust 与 cwd 相对引擎位，
//!    gap-packs-assets；围堰位 packs/rust、sih-tools/parser/packs/rust 等降为
//!    后续兼容候选），非 Python 包元数据定位；未探得即该文件记入 parse_errors
//!    不致命。语言包装载跳过 schema 全查（parser bin 为全查位），缺件或 JSON
//!    不合即按文件记 parse_errors。
//! 6. carriers 元数据串为本引擎自述（md-lite、serde_json、serde_yaml、toml、
//!    judou 紧凑副本），非围堰的 markdown-it-py 与 PyYAML 版本串；围堰 Python 版
//!    所建索引与本引擎互查 stale 即报 grammar 全量重建，同引擎自洽双跑一致。
//! 7. 词法模式引擎为自足回溯 VM（同 parser bin 落差一）：\d 与 \s 语义近似、
//!    环视内捕获组不持久；模式不合子集即该代码文件记 parse_errors。
//! 8. 浮点标量文本化随各引擎 Display（指数形 1e20 与 1e+20 类差异）；文件读取
//!    decode replace 粒度随 from_utf8_lossy。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// 围堰版本锚：sih-tools/locator/src/locator/__init__.py __version__。
const VERSION: &str = "0.1.0";
const TOOL: &str = "locator";

// ================================ 错误与杂项 ================================

enum ToolErr {
    Tool(String),
}

type R<T> = Result<T, ToolErr>;

fn tool_err<T>(msg: impl Into<String>) -> R<T> {
    Err(ToolErr::Tool(msg.into()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn content_hash(text: Option<&str>) -> String {
    sha256_hex(text.unwrap_or("").as_bytes())
}

/// 稳定标识派生承 DES-009（identity.stable_id 对表）。
fn stable_id(path: &str, carrier: &str, kind: &str, seq: usize, chash: &str) -> String {
    let material = format!("{path}\u{0}{carrier}:{kind}\u{0}{seq}\u{0}{chash}");
    sha256_hex(material.as_bytes())
}

fn py_splitlines(s: &str) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let mut cur = String::new();
    for c in s.chars() {
        if c == '\n' || c == '\r' || c == '\u{b}' || c == '\u{c}' || c == '\u{1c}' || c == '\u{1d}'
            || c == '\u{1e}' || c == '\u{85}' || c == '\u{2028}' || c == '\u{2029}'
        {
            out.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn slice_chars(chars: &[char], a: usize, b: usize) -> String {
    let a = a.min(chars.len());
    let b = b.min(chars.len()).max(a);
    chars[a..b].iter().collect()
}

// ================================ 词法模式引擎（句读核心副本） ================================

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

impl CClass {
    fn m(&self, c: char) -> bool {
        let mut hit = false;
        for it in &self.items {
            let ok = match it {
                CItem::Ch(x) => *x == c,
                CItem::Rg(a, b) => *a <= c && c <= *b,
                CItem::W => is_w(c),
                CItem::NW => !is_w(c),
                CItem::D => c.is_ascii_digit() || c.is_numeric(),
                CItem::ND => !(c.is_ascii_digit() || c.is_numeric()),
                CItem::S => c.is_whitespace(),
                CItem::NS => !c.is_whitespace(),
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
        other if !other.is_ascii_alphanumeric() => Some(other),
        _ => None,
    }
}

struct PatParser {
    chars: Vec<char>,
    pos: usize,
    ngroups: usize,
}

enum ClassHead {
    Ch(char),
    Sh(CItem),
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

    fn parse_counted(&mut self) -> Option<(usize, Option<usize>)> {
        let start = self.pos;
        self.pos += 1;
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
                            _ => Err("未知组形 (?<...".to_string()),
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
            '$' => Ok(Ast::End),
            '\\' => self.parse_escape(),
            ')' | '*' | '+' | '?' => Err("悬空量词或括号".to_string()),
            other => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::Ch(other)] })),
        }
    }

    fn parse_escape(&mut self) -> Result<Ast, String> {
        let c = self.next().ok_or("模式提前结束")?;
        match c {
            'd' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::D] })),
            'D' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::ND] })),
            'w' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::W] })),
            'W' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::NW] })),
            's' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::S] })),
            'S' => Ok(Ast::Cl(CClass { neg: false, items: vec![CItem::NS] })),
            'b' => Ok(Ast::End),
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
            match self.class_item_head(c)? {
                ClassHead::Sh(it) => items.push(it),
                ClassHead::Ch(lo_c) => {
                    if self.peek() == Some('-') && self.peek2().is_some() && self.peek2() != Some(']')
                    {
                        self.pos += 1;
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

fn ast_nullable(a: &Ast) -> bool {
    match a {
        Ast::Cl(_) | Ast::Dot(_) | Ast::Ref(_) => false,
        Ast::Grp(s) | Ast::Cap(_, s) => ast_nullable(s),
        Ast::Seq(items) => items.iter().all(ast_nullable),
        Ast::Alt(alts) => alts.iter().any(ast_nullable),
        Ast::Rpt { min, .. } => *min == 0,
        Ast::Look { .. } | Ast::Behind { .. } | Ast::Start | Ast::End => true,
    }
}

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
        Ast::Look { .. } | Ast::Behind { .. } | Ast::Start | Ast::End => Some(0),
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

fn check_refs(a: &Ast, ngroups: usize) -> Result<(), String> {
    match a {
        Ast::Ref(n) if *n == 0 || *n > ngroups => Err(format!("回引组号越界 \\{n}")),
        Ast::Ref(_) => Ok(()),
        Ast::Grp(s) | Ast::Cap(_, s) => check_refs(s, ngroups),
        Ast::Seq(items) => items.iter().try_for_each(|x| check_refs(x, ngroups)),
        Ast::Alt(alts) => alts.iter().try_for_each(|x| check_refs(x, ngroups)),
        Ast::Rpt { sub, .. } => check_refs(sub, ngroups),
        Ast::Look { sub, .. } | Ast::Behind { sub, .. } => check_refs(sub, ngroups),
        _ => Ok(()),
    }
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

// ================================ 句读词法与 PEG（紧凑副本） ================================

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

fn lex(text: &[char], tokens_spec: &Value) -> Result<Vec<Token>, String> {
    let entries = tokens_spec.as_array().ok_or("词法表须列表")?;
    let mut compiled: Vec<(String, Prog, bool, Option<Value>)> = vec![];
    for entry in entries {
        let obj = entry.as_object().ok_or("词法表元非对象")?;
        let name = obj
            .get("name")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or("词法名须非空字符串")?
            .to_string();
        let pattern = obj.get("pattern").and_then(|v| v.as_str()).ok_or("词法模式须字符串")?;
        let prog = compile_pattern(pattern).map_err(|e| format!("词法模式编译失败 {name}: {e}"))?;
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
        let mut hit: Option<(usize, usize, bool, Option<Value>)> = None;
        for (idx, (_name, prog, skip, tspec)) in compiled.iter().enumerate() {
            if let Some(end) = exec(prog, text, pos) {
                if end > pos {
                    hit = Some((end, idx, *skip, tspec.clone()));
                    break;
                }
            }
        }
        let (end, idx, skip, tspec) = match hit {
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
                name: compiled[idx].0.clone(),
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
}

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
const EMPTY_INPUT_MESSAGE: &str = "输入为空记号流为零";

struct MR {
    nodes: Vec<Node>,
    pos: usize,
}

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
            // 有序选择：按声明序取首个成功者，全败不回溯（ALG-012）。
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

fn parse_tokens(tokens: &[Token], grammar: &Value) -> Result<Node, String> {
    let empty = Map::new();
    let rules: &Map<String, Value> =
        grammar.get("rules").and_then(|v| v.as_object()).unwrap_or(&empty);
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
        return Err(format!("规则匹配深度超闸 {DEPTH_LIMIT}"));
    }
    root.line_start = Some(tokens[0].line_start);
    root.line_end = Some(tokens[total - 1].line_end);
    root.start_char = Some(tokens[0].start);
    root.end_char = Some(tokens[total - 1].end);
    root.start_tok = Some(0);
    root.end_tok = Some(total);
    Ok(root)
}

// ================================ 句读条目投影（紧凑副本） ================================

fn walk_rules<'a>(
    node: &'a Node,
    chain: &mut Vec<&'a Node>,
    out: &mut Vec<(&'a Node, Vec<&'a Node>)>,
) {
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

fn scalar_text(node: &Node) -> Option<String> {
    let last = node.children.last()?;
    if last.t == NType::Token {
        Some(last.text.clone())
    } else {
        None
    }
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

fn py_splitlines_keepends(s: &str) -> Vec<String> {
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
            cur.push(c);
            if boundary_len == 2 {
                cur.push('\n');
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

fn join_rules(src: &[char], node: &Node, spec: &Value) -> Result<String, String> {
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
    let line_strip_max = spec.get("line_strip_max").and_then(|v| v.as_i64()).unwrap_or(0).max(0) as usize;

    let span = |n: &Node| -> String {
        let a = n.start_char.unwrap_or(0);
        let b = n.end_char.unwrap_or(0);
        let mut s = slice_chars(src, a, b);
        if !strip_prefix.is_empty() || line_strip_max > 0 {
            let lines = py_splitlines_keepends(&s);
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

fn raw_lines(src: &[char], node: &Node, head: usize, tail: usize, strip_prefix: &[String]) -> String {
    let span = slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0));
    let mut lines = py_splitlines_keepends(&span);
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

fn entry_text(src: &[char], node: &Node, text_form: Option<&Value>) -> Result<Option<String>, String> {
    match text_form {
        None => Ok(Some(slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0)))),
        Some(Value::String(s)) if s == "raw" => {
            Ok(Some(slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0))))
        }
        Some(Value::String(s)) if s == "compact" => {
            let span = slice_chars(src, node.start_char.unwrap_or(0), node.end_char.unwrap_or(0));
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
        _ => Err(format!("text_form 未知形: {text_form:?}")),
    }
}

/// 树到条目流（entries.project_entries 对表）。
fn project_entries(
    src: &[char],
    tree: &Node,
    mapping_entries: &[Value],
    path: &str,
    carrier: &str,
) -> Result<Vec<Value>, String> {
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
                Some(other) => return Err(format!("name_from 未知形: {other:?}")),
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

// ================================ 句读语言包（定位与简载） ================================

fn grammar_version(pack_dir: &Path) -> String {
    let revision = fs::read_to_string(pack_dir.join("mapping.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|m| m.get("revision").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    format!("judou-rust-rev{revision}+parser-{VERSION}")
}

static RUST_PACK: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();

fn resolve_rust_pack() -> Option<PathBuf> {
    RUST_PACK
        .get_or_init(|| {
            if let Ok(root) = std::env::var("PARSER_PACKS_ROOT") {
                let p = PathBuf::from(root).join("rust");
                if p.is_dir() {
                    return Some(p);
                }
            }
            // 引擎位默认候选首位（gap-packs-assets）：exe 派生引擎仓根与 cwd
            // 相对引擎位在前；围堰位降为后续兼容候选，行为对表不受影响。
            let mut cands: Vec<PathBuf> = Vec::new();
            if let Some(engine_root) = std::env::current_exe().ok().and_then(|e| {
                e.parent()
                    .and_then(|p| p.parent())
                    .and_then(|p| p.parent())
                    .map(|p| p.to_path_buf())
            }) {
                cands.push(engine_root.join("packs/parser/rust"));
            }
            cands.push(PathBuf::from("packs/parser/rust"));
            cands.push(PathBuf::from("sih-engine/packs/parser/rust"));
            cands.push(PathBuf::from("packs/rust"));
            cands.push(PathBuf::from("sih-tools/parser/packs/rust"));
            cands.push(PathBuf::from("../sih-tools/parser/packs/rust"));
            cands.push(PathBuf::from("../../sih-tools/parser/packs/rust"));
            for cand in &cands {
                if cand.join("mapping.json").is_file() && cand.join("tokens.json").is_file() {
                    return Some(cand.clone());
                }
            }
            None
        })
        .clone()
}

struct CodePack {
    tokens: Value,
    grammar: Value,
    mapping: Vec<Value>,
}

static CODE_PACK: std::sync::OnceLock<Result<CodePack, String>> = std::sync::OnceLock::new();

fn get_code_pack() -> Result<&'static CodePack, String> {
    // 简载：三件齐备与 JSON 可解即过（schema 全查归 parser bin，落差五）。
    match CODE_PACK.get_or_init(|| {
        let dir = resolve_rust_pack().ok_or_else(|| "句读语言包未探得".to_string())?;
        let read = |name: &str| -> Result<Value, String> {
            let raw = fs::read(dir.join(name)).map_err(|e| format!("{name} 不可读: {e}"))?;
            serde_json::from_slice(&raw).map_err(|e| format!("{name} JSON 不合: {e}"))
        };
        let tokens = read("tokens.json")?;
        let grammar = read("grammar.json")?;
        let mapping = read("mapping.json")?;
        let entries = mapping
            .get("entries")
            .and_then(|v| v.as_array())
            .cloned()
            .ok_or_else(|| "mapping.json entries 须列表".to_string())?;
        Ok(CodePack { tokens: tokens.get("tokens").cloned().unwrap_or(Value::Null), grammar, mapping: entries })
    }) {
        Ok(p) => Ok(p),
        Err(e) => Err(e.clone()),
    }
}

/// 代码载体即句读解析条目投影（carriers/code.py parse_code 对表）。
fn parse_code(rel: &str, src: &[char]) -> Result<Vec<Unit>, String> {
    let pack = get_code_pack()?;
    let tokens = lex(src, &pack.tokens).map_err(|e| e.to_string())?;
    let tree = parse_tokens(&tokens, &pack.grammar)?;
    let entries = project_entries(src, &tree, &pack.mapping, rel, "rust")?;
    Ok(entries
        .iter()
        .map(|e| Unit {
            kind: e["kind"].as_str().unwrap_or("").to_string(),
            name: e["name"].as_str().map(|s| s.to_string()),
            line_start: e["line_start"].as_u64().map(|v| v as usize),
            line_end: e["line_end"].as_u64().map(|v| v as usize),
            text: e["text"].as_str().map(|s| s.to_string()),
        })
        .collect())
}

// ================================ 域包与语料遍历 ================================

struct LocPack {
    name: String,
    version: Value,
    include: Vec<String>,
    exclude: Vec<String>,
    stale_threshold: i64,
    max_file_bytes: u64,
    raw_hash: String,
}

fn load_pack(pack_path: &str) -> R<LocPack> {
    let p = Path::new(pack_path);
    let f = if p.is_dir() { p.join("pack.json") } else { p.to_path_buf() };
    if !f.is_file() {
        return tool_err(format!("{}", pack_path));
    }
    let raw = fs::read(&f).map_err(|e| ToolErr::Tool(format!("{e}")))?;
    let obj: Value = serde_json::from_slice(&raw)
        .map_err(|e| ToolErr::Tool(format!("{pack_path}: {e}")))?;
    let name = match obj.get("name").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return tool_err(format!("'name'")),
    };
    let version = obj.get("version").cloned().unwrap_or(Value::Null);
    Ok(LocPack {
        name,
        version,
        include: obj
            .get("include")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
        exclude: obj
            .get("exclude")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
        stale_threshold: obj.get("stale_threshold").and_then(|v| v.as_i64()).unwrap_or(0),
        max_file_bytes: obj.get("max_file_bytes").and_then(|v| v.as_u64()).unwrap_or(2_000_000),
        raw_hash: sha256_hex(&raw),
    })
}

/// fnmatch 单段匹配（fnmatch.fnmatchcase 语义：星问号加方括号类）。
fn fnmatch_seg(pat: &[char], s: &[char]) -> bool {
    match pat.first() {
        None => s.is_empty(),
        Some('*') => {
            for k in 0..=s.len() {
                if fnmatch_seg(&pat[1..], &s[k..]) {
                    return true;
                }
            }
            false
        }
        Some('?') => !s.is_empty() && fnmatch_seg(&pat[1..], &s[1..]),
        Some('[') => {
            // 解析 [..] 或 [!..]；未闭合按字面 '['
            let mut i = 1usize;
            let mut neg = false;
            if pat.get(i) == Some(&'!') {
                neg = true;
                i += 1;
            }
            let mut items: Vec<(char, char)> = vec![];
            let mut first = true;
            let mut closed = false;
            while i < pat.len() {
                let c = pat[i];
                if c == ']' && !first {
                    closed = true;
                    i += 1;
                    break;
                }
                first = false;
                if i + 2 < pat.len() && pat[i + 1] == '-' && pat[i + 2] != ']' {
                    items.push((c, pat[i + 2]));
                    i += 3;
                } else {
                    items.push((c, c));
                    i += 1;
                }
            }
            if !closed || s.is_empty() {
                // 未闭合 '[' 按字面量
                return s[0] == '[' && fnmatch_seg(&pat[1..], &s[1..]);
            }
            let c = s[0];
            let hit = items.iter().any(|(a, b)| *a <= c && c <= *b);
            hit != neg && fnmatch_seg(&pat[i..], &s[1..])
        }
        Some(c) => !s.is_empty() && *c == s[0] && fnmatch_seg(&pat[1..], &s[1..]),
    }
}

/// 分段模式匹配，双星跨任意段（pack._match_pattern 对表）。
fn match_pattern(pattern: &str, segs: &[&str]) -> bool {
    let psegs: Vec<&str> = pattern.split('/').collect();
    fn rec(psegs: &[&str], segs: &[&str]) -> bool {
        if psegs.is_empty() {
            return segs.is_empty();
        }
        let seg = psegs[0];
        if seg == "**" {
            return (0..=segs.len()).any(|k| rec(&psegs[1..], &segs[k..]));
        }
        if segs.is_empty() {
            return false;
        }
        let pc: Vec<char> = seg.chars().collect();
        let sc: Vec<char> = segs[0].chars().collect();
        fnmatch_seg(&pc, &sc) && rec(&psegs[1..], &segs[1..])
    }
    rec(&psegs, segs)
}

fn match_path(pack: &LocPack, relpath: &str) -> bool {
    let segs: Vec<&str> = relpath.split('/').collect();
    if !pack.include.iter().any(|pat| match_pattern(pat, &segs)) {
        return false;
    }
    if pack.exclude.iter().any(|pat| match_pattern(pat, &segs)) {
        return false;
    }
    true
}

fn walk_dir(pack: &LocPack, root: &Path, dir: &Path, out: &mut Vec<String>, oversize: &mut Vec<String>) -> R<()> {
    let entries = fs::read_dir(dir).map_err(|e| ToolErr::Tool(format!("{e}")))?;
    let mut names: Vec<(String, PathBuf, bool)> = vec![];
    for e in entries {
        let e = e.map_err(|e| ToolErr::Tool(format!("{e}")))?;
        let path = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        let is_dir = path.is_dir();
        names.push((name, path, is_dir));
    }
    names.sort_by(|a, b| a.0.cmp(&b.0));
    for (_name, path, is_dir) in names {
        if is_dir {
            // os.walk followlinks=False：符号链接目录不深入。
            let symlink = fs::symlink_metadata(&path).map(|m| m.file_type().is_symlink()).unwrap_or(false);
            if !symlink {
                walk_dir(pack, root, &path, out, oversize)?;
            }
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        if !match_path(pack, &rel) {
            continue;
        }
        let size = fs::metadata(&path).map_err(|e| ToolErr::Tool(format!("{e}")))?.len();
        if size > pack.max_file_bytes {
            oversize.push(rel);
            continue;
        }
        out.push(rel);
    }
    Ok(())
}

fn walk_corpus(pack: &LocPack, root: &Path) -> R<(Vec<String>, Vec<String>)> {
    let mut included = vec![];
    let mut oversize = vec![];
    walk_dir(pack, root, root, &mut included, &mut oversize)?;
    included.sort();
    oversize.sort();
    Ok((included, oversize))
}

// ================================ 载体 ================================

#[derive(Clone, Debug)]
struct Unit {
    kind: String,
    name: Option<String>,
    line_start: Option<usize>,
    line_end: Option<usize>,
    text: Option<String>,
}

const EXT_MAP: [(&str, &str); 7] =
    [(".md", "markdown"), (".markdown", "markdown"), (".json", "json"), (".yaml", "yaml"), (".yml", "yaml"), (".toml", "toml"), (".rs", "code")];

fn json_scalar_text(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::Null => Some("null".to_string()),
        _ => None,
    }
}

fn walk_json(v: &Value, prefix: &str, units: &mut Vec<Unit>) {
    match v {
        Value::Object(m) => {
            for (k, val) in m {
                let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                units.push(Unit {
                    kind: "key".to_string(),
                    name: Some(path.clone()),
                    line_start: None,
                    line_end: None,
                    text: json_scalar_text(val),
                });
                if !json_scalar_text_simple(val) {
                    walk_json(val, &path, units);
                }
            }
        }
        Value::Array(a) => {
            for (idx, val) in a.iter().enumerate() {
                let path = format!("{prefix}[{idx}]");
                units.push(Unit {
                    kind: "idx".to_string(),
                    name: Some(path.clone()),
                    line_start: None,
                    line_end: None,
                    text: json_scalar_text(val),
                });
                if !json_scalar_text_simple(val) {
                    walk_json(val, &path, units);
                }
            }
        }
        _ => {}
    }
}

fn json_scalar_text_simple(v: &Value) -> bool {
    !matches!(v, Value::Object(_) | Value::Array(_))
}

#[allow(non_snake_case)]
fn parse_jsonCarrier(_rel: &str, src: &str) -> Result<Vec<Unit>, String> {
    let v: Value = serde_json::from_str(src).map_err(|e| e.to_string())?;
    let mut units = vec![];
    walk_json(&v, "", &mut units);
    Ok(units)
}

fn yaml_scalar_text(v: &serde_yaml::Value) -> Option<String> {
    match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Bool(b) => Some(b.to_string()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        serde_yaml::Value::Null => Some("null".to_string()),
        _ => None,
    }
}

fn yaml_scalar(v: &serde_yaml::Value) -> bool {
    !matches!(
        v,
        serde_yaml::Value::Mapping(_) | serde_yaml::Value::Sequence(_) | serde_yaml::Value::Tagged(_)
    )
}

fn walk_yaml(v: &serde_yaml::Value, prefix: &str, units: &mut Vec<Unit>) {
    let v: &serde_yaml::Value = match v {
        serde_yaml::Value::Tagged(t) => &t.value,
        other => other,
    };
    match v {
        serde_yaml::Value::Mapping(m) => {
            for (k, val) in m.iter() {
                let key_str = yaml_scalar_text(k).unwrap_or_default();
                let path = if prefix.is_empty() { key_str } else { format!("{prefix}.{key_str}") };
                units.push(Unit {
                    kind: "key".to_string(),
                    name: Some(path.clone()),
                    line_start: None,
                    line_end: None,
                    text: yaml_scalar_text(val).filter(|_| yaml_scalar(val)),
                });
                if !yaml_scalar(val) {
                    walk_yaml(val, &path, units);
                }
            }
        }
        serde_yaml::Value::Sequence(s) => {
            for (idx, val) in s.iter().enumerate() {
                let path = format!("{prefix}[{idx}]");
                units.push(Unit {
                    kind: "idx".to_string(),
                    name: Some(path.clone()),
                    line_start: None,
                    line_end: None,
                    text: yaml_scalar_text(val).filter(|_| yaml_scalar(val)),
                });
                if !yaml_scalar(val) {
                    walk_yaml(val, &path, units);
                }
            }
        }
        _ => {}
    }
}

#[allow(non_snake_case)]
fn parse_yamlCarrier(_rel: &str, src: &str) -> Result<Vec<Unit>, String> {
    let v: serde_yaml::Value = serde_yaml::from_str(src).map_err(|e| e.to_string())?;
    let mut units = vec![];
    walk_yaml(&v, "", &mut units);
    Ok(units)
}

fn toml_scalar_text(v: &toml::Value) -> Option<String> {
    match v {
        toml::Value::String(s) => Some(s.clone()),
        toml::Value::Integer(i) => Some(i.to_string()),
        toml::Value::Float(f) => Some(f.to_string()),
        toml::Value::Boolean(b) => Some(b.to_string()),
        toml::Value::Datetime(d) => Some(d.to_string()),
        _ => None,
    }
}

fn walk_toml(v: &toml::Value, prefix: &str, units: &mut Vec<Unit>) {
    match v {
        toml::Value::Table(t) => {
            for (k, val) in t {
                let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                units.push(Unit {
                    kind: "key".to_string(),
                    name: Some(path.clone()),
                    line_start: None,
                    line_end: None,
                    text: toml_scalar_text(val),
                });
                if !matches!(val, toml::Value::Table(_) | toml::Value::Array(_)) {
                    continue;
                }
                walk_toml(val, &path, units);
            }
        }
        toml::Value::Array(a) => {
            for (idx, val) in a.iter().enumerate() {
                let path = format!("{prefix}[{idx}]");
                units.push(Unit {
                    kind: "idx".to_string(),
                    name: Some(path.clone()),
                    line_start: None,
                    line_end: None,
                    text: toml_scalar_text(val),
                });
                if matches!(val, toml::Value::Table(_) | toml::Value::Array(_)) {
                    walk_toml(val, &path, units);
                }
            }
        }
        _ => {}
    }
}

#[allow(non_snake_case)]
fn parse_tomlCarrier(_rel: &str, src: &str) -> Result<Vec<Unit>, String> {
    let v: toml::Value = src.parse().map_err(|e| format!("{e}"))?;
    let mut units = vec![];
    walk_toml(&v, "", &mut units);
    Ok(units)
}

/// markdown 载体行级 lite 解析（落差二）。
fn parse_markdown(_rel: &str, src: &str) -> Result<Vec<Unit>, String> {
    // 引用剥标预 pass：行首（至多三空格后）'>' 加一可选空白剥除，行号不变。
    let raw_lines: Vec<String> = src.split('\n').map(|l| l.to_string()).collect();
    let mut lines: Vec<(usize, String)> = vec![];
    for (i, l) in raw_lines.iter().enumerate() {
        let t = l.trim_start_matches(' ');
        let stripped = if t.starts_with('>') {
            let rest = &t[1..];
            let rest = rest.strip_prefix(' ').unwrap_or(rest);
            rest.to_string()
        } else {
            l.clone()
        };
        lines.push((i, stripped));
    }
    let n = lines.len();
    let mut units: Vec<Unit> = vec![];
    let mut para: Vec<(usize, String)> = vec![];

    fn flush_para(para: &mut Vec<(usize, String)>, units: &mut Vec<Unit>) {
        if para.is_empty() {
            return;
        }
        let first = para[0].0;
        let last = para[para.len() - 1].0;
        let text: Vec<&str> = para.iter().map(|(_, l)| l.as_str()).collect();
        units.push(Unit {
            kind: "para".to_string(),
            name: None,
            line_start: Some(first + 1),
            line_end: Some(last + 1),
            text: Some(text.join("\n")),
        });
        para.clear();
    }

    fn is_indent_code(l: &str) -> bool {
        let stripped = l.trim_start_matches(' ');
        l.len() - stripped.len() >= 4 && !stripped.is_empty()
    }

    let mut i = 0usize;
    while i < n {
        let (idx, line) = (lines[i].0, lines[i].1.clone());
        let trimmed = line.trim();
        // 围栏
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            flush_para(&mut para, &mut units);
            let fence_ch = trimmed.chars().next().unwrap();
            let fence_len = trimmed.chars().take_while(|c| *c == fence_ch).count();
            let mut j = i + 1;
            let mut content: Vec<String> = vec![];
            let mut end_idx = n - 1;
            while j < n {
                let lj = &lines[j].1;
                let lt = lj.trim();
                if lt.chars().next() == Some(fence_ch)
                    && lt.chars().take_while(|c| *c == fence_ch).count() >= fence_len
                    && lt.chars().skip_while(|c| *c == fence_ch).all(|c| c.is_whitespace())
                {
                    end_idx = j;
                    break;
                }
                content.push(lj.clone());
                j += 1;
            }
            if j >= n {
                end_idx = n - 1;
            }
            let mut text = content.join("\n");
            if !content.is_empty() {
                text.push('\n');
            }
            units.push(Unit {
                kind: "fence".to_string(),
                name: None,
                line_start: Some(idx + 1),
                line_end: Some((end_idx + 1).max(idx + 1)),
                text: Some(text),
            });
            i = if j < n { j + 1 } else { n };
            continue;
        }
        // 空行
        if trimmed.is_empty() {
            flush_para(&mut para, &mut units);
            i += 1;
            continue;
        }
        // ATX 标题
        let hashes = line.chars().take_while(|c| *c == '#').count();
        if hashes >= 1 && hashes <= 6 {
            let rest = &line[hashes..];
            if rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t') {
                flush_para(&mut para, &mut units);
                let mut content = rest.trim().to_string();
                // 闭串剥除：尾随 # 串且其前为空白或全为 #
                let trimmed_end = content.trim_end();
                if trimmed_end.ends_with('#') {
                    let core = trimmed_end.trim_end_matches('#');
                    if core.is_empty() || core.ends_with(' ') || core.ends_with('\t') {
                        content = core.trim_end().to_string();
                    }
                }
                units.push(Unit {
                    kind: "heading".to_string(),
                    name: Some(content.clone()),
                    line_start: Some(idx + 1),
                    line_end: Some(idx + 1),
                    text: Some(content),
                });
                i += 1;
                continue;
            }
        }
        // 水平线
        {
            let t = trimmed;
            for ch in ['-', '*', '_'] {
                if !t.is_empty() && t.chars().all(|c| c == ch || c.is_whitespace()) && t.matches(ch).count() >= 3 {
                    flush_para(&mut para, &mut units);
                    i += 1;
                    continue;
                }
            }
        }
        // 缩进码块
        if is_indent_code(&line) {
            flush_para(&mut para, &mut units);
            i += 1;
            continue;
        }
        // HTML 块行（近似：行首 < 标记）
        if trimmed.starts_with('<')
            && trimmed
                .chars()
                .nth(1)
                .map(|c| c.is_ascii_alphabetic() || c == '!' || c == '/' || c == '?')
                .unwrap_or(false)
        {
            flush_para(&mut para, &mut units);
            i += 1;
            continue;
        }
        // 列表项
        let indent = line.len() - line.trim_start_matches(' ').len();
        let rest = &line[indent..];
        let marker_len = rest
            .find(|c: char| c == ' ' || c == '\t')
            .unwrap_or(rest.len());
        let marker = &rest[..marker_len];
        let is_marker = (marker.len() == 1 && matches!(marker, "-" | "*" | "+"))
            || (marker.len() >= 2
                && marker.ends_with(|c| c == '.' || c == ')')
                && marker[..marker.len() - 1].chars().all(|c| c.is_ascii_digit())
                && !marker[..marker.len() - 1].is_empty());
        // "**加粗**" 类不误判：* 后须空白或行尾
        let marker_ok = is_marker
            && (marker.len() == 1
                || marker.starts_with(|c: char| c.is_ascii_digit())
                || !marker.starts_with('*'));
        if marker_ok && (rest.len() == marker_len || rest[marker_len..].starts_with(' ') || rest[marker_len..].starts_with('\t')) {
            flush_para(&mut para, &mut units);
            let first_content = if rest.len() > marker_len { rest[marker_len..].trim().to_string() } else { String::new() };
            let mut span_lines: Vec<String> = vec![];
            if !first_content.is_empty() {
                span_lines.push(first_content);
            }
            let mut j = i + 1;
            let mut end = i;
            let mut terminated = false;
            while j < n {
                let lj = &lines[j].1;
                let lt = lj.trim();
                if lt.is_empty() {
                    j += 1;
                    continue;
                }
                let ind_j = lj.len() - lj.trim_start_matches(' ').len();
                let rest_j = &lj[ind_j..];
                let ml_j = rest_j.find(|c: char| c == ' ' || c == '\t').unwrap_or(rest_j.len());
                let mk_j = &rest_j[..ml_j];
                let is_item_j = (mk_j.len() == 1 && matches!(mk_j, "-" | "*" | "+"))
                    || (mk_j.len() >= 2
                        && mk_j.ends_with(|c| c == '.' || c == ')')
                        && mk_j[..mk_j.len() - 1].chars().all(|c| c.is_ascii_digit())
                        && !mk_j[..mk_j.len() - 1].is_empty());
                if ind_j <= indent && (is_item_j || !is_indent_code(lj)) {
                    // 终止行：把此前连续空行计入跨度
                    end = j - 1;
                    terminated = true;
                    break;
                }
                // 深层续行或嵌套项：剥标记取内容
                let content_j = if is_item_j && rest_j.len() > ml_j {
                    rest_j[ml_j..].trim().to_string()
                } else {
                    lt.to_string()
                };
                span_lines.push(content_j);
                end = j;
                j += 1;
            }
            if !terminated {
                // EOF 截止：去掉尾随空行
                end = if span_lines.is_empty() { i } else { j - 1 };
            }
            let text = span_lines.join(" ");
            units.push(Unit {
                kind: "list_item".to_string(),
                name: None,
                line_start: Some(idx + 1),
                line_end: Some(end + 1),
                text: Some(text),
            });
            i = if terminated { j } else { n };
            continue;
        }
        // 段落行
        para.push((idx, line.clone()));
        i += 1;
    }
    flush_para(&mut para, &mut units);
    Ok(units)
}

/// Python Path.suffix 语义：末段首个后零位圆点起算；隐藏文件无后缀。
fn suffix_of(rel: &str) -> String {
    let name = rel.rsplit('/').next().unwrap_or("");
    match name.rfind('.') {
        Some(i) if i > 0 => name[i..].to_string(),
        _ => String::new(),
    }
}

fn carrier_of(rel: &str) -> Option<&'static str> {
    let s = suffix_of(rel);
    EXT_MAP.iter().find(|(ext, _)| *ext == s).map(|(_, c)| *c)
}

fn parse_by_carrier(rel: &str, src: &[char]) -> Result<(&'static str, Vec<Unit>), String> {
    let carrier = carrier_of(rel).ok_or_else(|| format!("载体未知: {rel}"))?;
    let s = src.iter().collect::<String>();
    let units = match carrier {
        "markdown" => parse_markdown(rel, &s)?,
        "json" => parse_jsonCarrier(rel, &s)?,
        "yaml" => parse_yamlCarrier(rel, &s)?,
        "toml" => parse_tomlCarrier(rel, &s)?,
        "code" => parse_code(rel, src)?,
        _ => return Err(format!("载体未知: {rel}")),
    };
    Ok((carrier, units))
}

// ================================ 条目组装与存储 ================================

fn make_entry(path: &str, carrier: &str, kind: &str, name: Option<String>, seq: usize, ls: Option<usize>, le: Option<usize>, text: Option<String>) -> Value {
    let chash = content_hash(text.as_deref());
    let mut m = Map::new();
    m.insert("type".into(), json!("entry"));
    m.insert("id".into(), json!(stable_id(path, carrier, kind, seq, &chash)));
    m.insert("path".into(), json!(path));
    m.insert("carrier".into(), json!(carrier));
    m.insert("kind".into(), json!(kind));
    m.insert("name".into(), json!(name));
    m.insert("seq".into(), json!(seq));
    m.insert("line_start".into(), json!(ls));
    m.insert("line_end".into(), json!(le));
    m.insert("content_hash".into(), json!(chash));
    m.insert("text".into(), json!(text));
    Value::Object(m)
}

fn assemble(path: &str, carrier: &str, units: &[Unit]) -> Vec<Value> {
    let mut counters: HashMap<String, usize> = HashMap::new();
    let mut entries = vec![];
    for u in units {
        let c = counters.entry(u.kind.clone()).or_insert(0);
        *c += 1;
        entries.push(make_entry(
            path,
            carrier,
            &u.kind,
            u.name.clone(),
            *c,
            u.line_start,
            u.line_end,
            u.text.clone(),
        ));
    }
    entries
}

/// carriers 元数据（落差六：本引擎自述串）。
fn carriers_meta() -> Value {
    let mut code = Map::new();
    code.insert("parser".into(), json!("judou"));
    code.insert("version".into(), json!(VERSION));
    let gv = resolve_rust_pack().map(|p| grammar_version(&p)).unwrap_or_else(|| format!("judou-rust-revunknown+parser-{VERSION}"));
    code.insert("grammars".into(), json!({"rust": gv}));
    json!({
        "markdown": {"parser": "md-lite", "version": "0.1.0"},
        "json": {"parser": "serde_json", "version": "1"},
        "yaml": {"parser": "serde_yaml", "version": "0.9"},
        "toml": {"parser": "toml", "version": "0.8"},
        "code": Value::Object(code),
    })
}

struct Index {
    header: Value,
    file_records: Vec<Value>,
    entries: Vec<Value>,
}

fn dump_record(rec: &Value) -> String {
    rec.to_string()
}

fn load_index(index_path: &str) -> R<Index> {
    let p = Path::new(index_path);
    if !p.is_file() {
        return tool_err(format!("{index_path}"));
    }
    let text = fs::read_to_string(p).map_err(|e| ToolErr::Tool(format!("{e}")))?;
    let mut header: Option<Value> = None;
    let mut files = vec![];
    let mut entries = vec![];
    for ln in text.lines() {
        if ln.trim().is_empty() {
            continue;
        }
        let rec: Value = serde_json::from_str(ln)
            .map_err(|e| ToolErr::Tool(format!("{e}")))?;
        match rec.get("type").and_then(|v| v.as_str()) {
            Some("header") => header = Some(rec),
            Some("file") => files.push(rec),
            Some("entry") => entries.push(rec),
            _ => {}
        }
    }
    match header {
        Some(h) => Ok(Index { header: h, file_records: files, entries }),
        None => tool_err("索引件缺头部"),
    }
}

fn batch_write(index_path: &str, header: &Value, file_records: &[Value], entries: &[Value]) -> R<()> {
    let mut by_path: HashMap<&str, Vec<&Value>> = HashMap::new();
    for e in entries {
        let p = e["path"].as_str().unwrap_or("");
        by_path.entry(p).or_default().push(e);
    }
    let mut lines = vec![dump_record(header)];
    for r in file_records {
        lines.push(dump_record(r));
        let p = r["path"].as_str().unwrap_or("");
        if let Some(es) = by_path.get(p) {
            for e in es {
                lines.push(dump_record(e));
            }
        }
    }
    let out = Path::new(index_path);
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| ToolErr::Tool(format!("{e}")))?;
        }
    }
    fs::write(out, format!("{}\n", lines.join("\n"))).map_err(|e| ToolErr::Tool(format!("{e}")))
}

// ================================ build / query / stale ================================

fn build(pack_path: &str, root: &str, out_path: &str) -> R<Value> {
    let pack = load_pack(pack_path)?;
    let root_path = Path::new(root);
    if !root_path.is_dir() {
        return tool_err(format!("{root}"));
    }
    let (included, oversize) = walk_corpus(&pack, root_path)?;
    let mut file_records: Vec<Value> = vec![];
    let mut entries: Vec<Value> = vec![];
    let mut parse_errors: Vec<String> = vec![];
    for rel in &included {
        let full = root_path.join(rel);
        let src_bytes = fs::read(&full).map_err(|e| ToolErr::Tool(format!("{e}")))?;
        let chash = sha256_hex(&src_bytes);
        let src_chars: Vec<char> = String::from_utf8_lossy(&src_bytes).chars().collect();
        let parsed = parse_by_carrier(rel, &src_chars);
        let (carrier, units) = match parsed {
            Ok(x) => x,
            Err(_) => {
                parse_errors.push(rel.clone());
                continue;
            }
        };
        let es = assemble(rel, carrier, &units);
        let mut fr = Map::new();
        fr.insert("type".into(), json!("file"));
        fr.insert("path".into(), json!(rel));
        fr.insert("content_hash".into(), json!(chash));
        fr.insert("entries".into(), json!(es.len()));
        file_records.push(Value::Object(fr));
        entries.extend(es);
    }
    parse_errors.extend(oversize);
    parse_errors.sort();
    let mut pack_meta = Map::new();
    pack_meta.insert("name".into(), json!(pack.name));
    pack_meta.insert("version".into(), json!(pack.version));
    pack_meta.insert("hash".into(), json!(pack.raw_hash));
    let header = json!({
        "type": "header",
        "tool": TOOL,
        "tool_version": VERSION,
        "pack": Value::Object(pack_meta),
        "carriers": carriers_meta(),
        "file_count": file_records.len(),
        "entry_count": entries.len(),
        "parse_errors": parse_errors,
    });
    batch_write(out_path, &header, &file_records, &entries)?;
    Ok(json!({"files": file_records.len(), "entries": entries.len(), "parse_errors": parse_errors}))
}

fn occurrence_scan(pack: &LocPack, root: &Path, word: &str) -> R<Vec<Value>> {
    let rx = regex::Regex::new(&format!(r"\b{}\b", regex::escape(word)))
        .map_err(|e| ToolErr::Tool(format!("{e}")))?;
    let (included, _) = walk_corpus(pack, root)?;
    let mut occurrences = vec![];
    for rel in included {
        let text = String::from_utf8_lossy(&fs::read(root.join(&rel)).map_err(|e| ToolErr::Tool(format!("{e}")))?)
            .into_owned();
        for (ln, line) in py_splitlines(&text).into_iter().enumerate() {
            for m in rx.find_iter(&line) {
                let col = line[..m.start()].chars().count() + 1;
                occurrences.push(json!({
                    "path": rel,
                    "line": ln + 1,
                    "col": col,
                    "line_text": line.trim(),
                }));
            }
        }
    }
    Ok(occurrences)
}

fn run_query(index_path: &str, mode: &str, value: &str, pack_path: Option<&str>, root: Option<&str>) -> R<Value> {
    let index = load_index(index_path)?;
    let mut entries_out: Vec<Value> = vec![];
    let mut occurrences: Vec<Value> = vec![];
    match mode {
        "id" => {
            if let Some(hit) = index.entries.iter().find(|e| e["id"].as_str() == Some(value)) {
                entries_out.push(hit.clone());
            }
        }
        "term" => {
            for e in &index.entries {
                if e["name"].as_str() == Some(value) {
                    entries_out.push(e.clone());
                }
            }
        }
        "prefix" => {
            for e in &index.entries {
                if e["path"].as_str().map(|p| p.starts_with(value)).unwrap_or(false) {
                    entries_out.push(e.clone());
                }
            }
        }
        "domain" => {
            for e in &index.entries {
                if e["carrier"].as_str() == Some(value) {
                    entries_out.push(e.clone());
                }
            }
        }
        "ref" => {
            let (Some(pp), Some(rt)) = (pack_path, root) else {
                return tool_err("ref 查询需 --pack 与 --root");
            };
            let rx = regex::Regex::new(&format!(r"\b{}\b", regex::escape(value)))
                .map_err(|e| ToolErr::Tool(format!("{e}")))?;
            for e in &index.entries {
                let hay_name = e["name"].as_str().unwrap_or("");
                let hay_text = e["text"].as_str().unwrap_or("");
                if rx.is_match(hay_name) || rx.is_match(hay_text) {
                    entries_out.push(e.clone());
                }
            }
            let pack = load_pack(pp)?;
            let root_path = Path::new(rt);
            occurrences = occurrence_scan(&pack, root_path, value)?;
        }
        other => return tool_err(format!("查询模式未知: {other}")),
    }
    Ok(json!({"mode": mode, "value": value, "entries": entries_out, "occurrences": occurrences}))
}

fn run_stale(pack_path: &str, root: &str, index_path: &str) -> R<Value> {
    let pack = load_pack(pack_path)?;
    let root_path = Path::new(root);
    if !root_path.is_dir() {
        return tool_err(format!("{root}"));
    }
    let index = load_index(index_path)?;
    let mut current: HashMap<String, String> = HashMap::new();
    let (included, _) = walk_corpus(&pack, root_path)?;
    for rel in included {
        let h = sha256_hex(&fs::read(root_path.join(&rel)).map_err(|e| ToolErr::Tool(format!("{e}")))?);
        current.insert(rel, h);
    }
    let mut idx_paths: HashMap<String, String> = HashMap::new();
    for r in &index.file_records {
        if let (Some(p), Some(h)) = (r["path"].as_str(), r["content_hash"].as_str()) {
            idx_paths.insert(p.to_string(), h.to_string());
        }
    }
    let mut fresh = vec![];
    let mut stale = vec![];
    let mut missing = vec![];
    for (p, h) in &idx_paths {
        match current.get(p) {
            None => missing.push(p.clone()),
            Some(ch) if ch == h => fresh.push(p.clone()),
            Some(_) => stale.push(p.clone()),
        }
    }
    for p in current.keys() {
        if !idx_paths.contains_key(p) {
            missing.push(p.clone());
        }
    }
    fresh.sort();
    stale.sort();
    missing.sort();
    let reason = if index.header["pack"]["hash"] != json!(pack.raw_hash) {
        Some(json!("pack"))
    } else if index.header["carriers"] != carriers_meta() {
        Some(json!("grammar"))
    } else {
        None
    };
    let stale_count = stale.len() + missing.len();
    Ok(json!({
        "files": {"fresh": fresh, "stale": stale, "missing": missing},
        "rebuild_required": reason.is_some() || stale_count > 0,
        "reason": reason,
        "over_threshold": stale_count as i64 > pack.stale_threshold,
        "stale_threshold": pack.stale_threshold,
    }))
}

// ================================ CLI ================================

fn emit(obj: &Value) {
    println!("{}", obj.to_string());
}

fn usage_fail(msg: &str) -> ! {
    eprintln!(
        "{}",
        json!({"error": format!("用法错: {msg} (locator build --pack --root --out | query --index --id/--term/--prefix/--ref/--domain | stale --pack --root --index)")})
    );
    std::process::exit(2);
}

struct QFlags {
    pack: Option<String>,
    root: Option<String>,
    out: Option<String>,
    index: Option<String>,
    qid: Option<String>,
    term: Option<String>,
    prefix: Option<String>,
    r#ref: Option<String>,
    domain: Option<String>,
}

fn parse_flags(args: &[String], allowed: &[&str], cmd: &str) -> QFlags {
    let mut f = QFlags {
        pack: None,
        root: None,
        out: None,
        index: None,
        qid: None,
        term: None,
        prefix: None,
        r#ref: None,
        domain: None,
    };
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        let (key, inline) = match a.split_once('=') {
            Some((k, v)) => (k.to_string(), Some(v.to_string())),
            None => (a.clone(), None),
        };
        if !allowed.contains(&key.as_str()) {
            usage_fail(&format!("{cmd}: 未知旗标或参数: {a}"));
        }
        let val = match inline {
            Some(v) => {
                i += 1;
                v
            }
            None => {
                i += 1;
                if i >= args.len() {
                    usage_fail(&format!("{cmd}: {key} 缺值"));
                }
                let v = args[i].clone();
                i += 1;
                v
            }
        };
        match key.as_str() {
            "--pack" => f.pack = Some(val),
            "--root" => f.root = Some(val),
            "--out" => f.out = Some(val),
            "--index" => f.index = Some(val),
            "--id" => f.qid = Some(val),
            "--term" => f.term = Some(val),
            "--prefix" => f.prefix = Some(val),
            "--ref" => f.r#ref = Some(val),
            "--domain" => f.domain = Some(val),
            _ => unreachable!(),
        }
    }
    f
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令");
    }
    let cmd = args[0].as_str();
    let rest = &args[1..];
    let work: R<Value> = match cmd {
        "build" => {
            let f = parse_flags(rest, &["--pack", "--root", "--out"], "build");
            let (Some(pack), Some(root), Some(out)) = (f.pack, f.root, f.out) else {
                usage_fail("build 缺 --pack --root --out（均必填）");
            };
            build(&pack, &root, &out)
        }
        "query" => {
            let f = parse_flags(rest, &["--index", "--id", "--term", "--prefix", "--ref", "--domain", "--pack", "--root"], "query");
            let Some(index) = f.index else { usage_fail("query 缺 --index（必填）") };
            let modes: [(&str, &Option<String>); 5] =
                [("id", &f.qid), ("term", &f.term), ("prefix", &f.prefix), ("ref", &f.r#ref), ("domain", &f.domain)];
            let picked: Vec<(&str, &String)> =
                modes.iter().filter(|(_, v)| v.is_some()).map(|(m, v)| (*m, v.as_ref().unwrap())).collect();
            if picked.len() != 1 {
                usage_fail("query 须恰一个 --id/--term/--prefix/--ref/--domain");
            }
            let (mode, value) = picked[0];
            run_query(&index, mode, value, f.pack.as_deref(), f.root.as_deref())
        }
        "stale" => {
            let f = parse_flags(rest, &["--pack", "--root", "--index"], "stale");
            let (Some(pack), Some(root), Some(index)) = (f.pack, f.root, f.index) else {
                usage_fail("stale 缺 --pack --root --index（均必填）");
            };
            run_stale(&pack, &root, &index)
        }
        "vectors" => usage_fail("vectors 子命令未移植（落差一）"),
        _ => usage_fail(&format!("未知子命令: {cmd}")),
    };
    match work {
        Ok(report) => {
            emit(&report);
            match cmd {
                "query" => {
                    let has = report["entries"].as_array().map(|a| !a.is_empty()).unwrap_or(false)
                        || report["occurrences"].as_array().map(|a| !a.is_empty()).unwrap_or(false);
                    if has {
                        0
                    } else {
                        1
                    }
                }
                "stale" => {
                    let over = report["over_threshold"].as_bool().unwrap_or(false)
                        || report["rebuild_required"].as_bool().unwrap_or(false);
                    if over {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        }
        Err(ToolErr::Tool(m)) => {
            eprintln!("{}", json!({"error": m}));
            2
        }
    }
}

fn main() {
    // 代码载体深文法递归需要大栈；panic 归工具异常退出码二。
    let code = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)
        .expect("线程启动失败")
        .join()
        .unwrap_or(2);
    std::process::exit(code);
}
