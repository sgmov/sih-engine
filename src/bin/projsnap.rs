//! 引擎侧派生快照投影器（projsnap）命令行面 —— lease-mergeleg6-parallel 簇J 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/projsnap 0.1.0（src/projsnap/cli.py，承 DES-018），
//! 只读对表移植，围堰源码零改动。纪律三件构成性：零 LLM；同参双跑逐字节一致；
//! 投影与源不符时以源为准。投影写面仅限登记派生节标记对之间，手写节零触碰。
//!
//! CLI：`projsnap <build|check|diff> --root <工作区根> [--target <文件>] [--json]`
//! 退出码三值：0 = build 完成 / check 无 stale / diff 无差；1 = check 有 stale /
//! diff 有差；2 = 用法错或读入异常。--target 缺省 <root>/AGENTS.md；未知旗标静默
//! 跳过与围堰一致；build 恒退出码 0 与围堰一致。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. 文本读入按 Python read_text 通用换行形预归一（\r\n 与 \r 归 \n）后处理；
//!    splitlines 的异形分隔符（\v、\f、U+2028 等）不对齐，按 \n 单一分隔处理。
//! 2. 非 UTF-8 目标或读盘失败按退出码 2 报（围堰为未捕 traceback 退出码 1）。
//! 3. 泊界计数面 JSON 顶层非对象（如数组）按跳过处理（围堰 AttributeError
//!    traceback 退出码 1）；pyproject 非法 TOML 跳过与围堰一致。
//! 4. diff 统一差 hunk 形按 CPython difflib SequenceMatcher 算法逐字对表移植
//!    （含 autojunk 门 n>=200），退出码判据（有差/无差）与围堰严格一致。
//! 5. 目标为目录等读失败形按退出码 2 报（围堰 traceback 退出码 1）。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

/// 围堰版本锚：sih-tools/projsnap/src/projsnap/cli.py VERSION。
const VERSION: &str = "0.1.0";
/// 派生节全集与顺序（SECTIONS 插入序，sources_hash 拼接序）。
const SECTIONS: [&str; 3] = ["file-index", "tool-versions", "parked-count"];

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn begin_mark(name: &str) -> String {
    format!("<!-- projsnap:begin {name} -->")
}

fn end_mark(name: &str) -> String {
    format!("<!-- projsnap:end {name} -->")
}

/// 块形（_block 对表）：begin 行 + 换行 + 节体 + end 行 + 换行。
fn section_block(name: &str, body: &str) -> String {
    format!("{}\n{}{}\n", begin_mark(name), body, end_mark(name))
}

// ---- Python 形 JSON 序列化（json.dumps 对表：sort_keys / separators / indent） ----

fn py_dumps(v: &Value, sort_keys: bool, tight: bool, indent: Option<usize>) -> String {
    let mut out = String::new();
    write_py(v, sort_keys, tight, indent, 0, &mut out);
    out
}

fn write_py(v: &Value, sort_keys: bool, tight: bool, indent: Option<usize>, level: usize, out: &mut String) {
    match v {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            out.push_str(&v.to_string())
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            match indent {
                Some(w) => {
                    let inner = " ".repeat(w * (level + 1));
                    let close = " ".repeat(w * level);
                    out.push_str("[\n");
                    for (i, item) in a.iter().enumerate() {
                        if i > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&inner);
                        write_py(item, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('\n');
                    out.push_str(&close);
                    out.push(']');
                }
                None => {
                    let sep = if tight { "," } else { ", " };
                    out.push('[');
                    for (i, item) in a.iter().enumerate() {
                        if i > 0 {
                            out.push_str(sep);
                        }
                        write_py(item, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push(']');
                }
            }
        }
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            if sort_keys {
                kvs.sort_by(|a, b| a.0.cmp(b.0));
            }
            match indent {
                Some(w) => {
                    let inner = " ".repeat(w * (level + 1));
                    let close = " ".repeat(w * level);
                    out.push_str("{\n");
                    for (i, (k, val)) in kvs.iter().enumerate() {
                        if i > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&inner);
                        out.push_str(&Value::String((*k).clone()).to_string());
                        out.push_str(": ");
                        write_py(val, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('\n');
                    out.push_str(&close);
                    out.push('}');
                }
                None => {
                    let (isep, ksep) = if tight { (",", ":") } else { (", ", ": ") };
                    out.push('{');
                    for (i, (k, val)) in kvs.iter().enumerate() {
                        if i > 0 {
                            out.push_str(isep);
                        }
                        out.push_str(&Value::String((*k).clone()).to_string());
                        out.push_str(ksep);
                        write_py(val, sort_keys, tight, indent, level + 1, out);
                    }
                    out.push('}');
                }
            }
        }
    }
}

// ---- 三派生节渲染器（cli.py _section_* 逐字对表） ----

/// 后缀过滤排序文件名（_sorted_files 对表：仅直下文件，名字典序）。
fn sorted_files(base: &Path, suffix: &str) -> Vec<String> {
    let mut names: Vec<String> = vec![];
    if let Ok(rd) = fs::read_dir(base) {
        for e in rd.flatten() {
            let p = e.path();
            if !p.is_file() {
                continue;
            }
            let name = e.file_name().to_string_lossy().into_owned();
            if name.len() > suffix.len() && name.ends_with(suffix) {
                names.push(name);
            }
        }
    }
    names.sort();
    names
}

fn section_file_index(root: &Path) -> String {
    let mut lines = vec!["<!-- 派生节：由 projsnap build 再生，手改无效 -->".to_string()];
    for (label, face) in [
        ("decision", "sih-engine/doc/decision"),
        ("design", "sih-engine/doc/design"),
        ("spec", "sih-engine/doc/spec"),
    ] {
        for name in sorted_files(&root.join(face), ".md") {
            lines.push(format!("- {label}: {name}"));
        }
    }
    format!("{}\n", lines.join("\n"))
}

fn section_tool_versions(root: &Path) -> String {
    let tools = root.join("sih-tools");
    let mut rows: Vec<(String, String)> = vec![];
    if tools.is_dir() {
        // glob "*/pyproject.toml" 对表：直下子项的 pyproject.toml，全路径字典序。
        let mut candidates: Vec<PathBuf> = vec![];
        if let Ok(rd) = fs::read_dir(&tools) {
            for e in rd.flatten() {
                let pp = e.path().join("pyproject.toml");
                if pp.is_file() {
                    candidates.push(pp);
                }
            }
        }
        candidates.sort_by_key(|p| p.to_string_lossy().into_owned());
        for pp in candidates {
            let text = match fs::read_to_string(&pp) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let data: toml::Value = match toml::from_str(&text) {
                Ok(d) => d,
                Err(_) => continue,
            };
            let project = data.get("project");
            let dirname = pp
                .parent()
                .map(|p| {
                    p.file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default()
                })
                .unwrap_or_default();
            let name = project
                .and_then(|p| p.get("name"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or(dirname);
            let ver = project
                .and_then(|p| p.get("version"))
                .and_then(|v| v.as_str())
                .unwrap_or("?")
                .to_string();
            rows.push((name, ver));
        }
    }
    rows.sort();
    let mut lines = vec!["<!-- 派生节：由 projsnap build 再生，手改无效 -->".to_string()];
    lines.extend(rows.iter().map(|(n, v)| format!("- {n} {v}")));
    format!("{}\n", lines.join("\n"))
}

fn section_parked_count(root: &Path) -> String {
    let mat = root.join("sih-engine/sih/state/parking/materials");
    let mut parked = 0u64;
    if mat.is_dir() {
        let mut names: Vec<PathBuf> = vec![];
        if let Ok(rd) = fs::read_dir(&mat) {
            for e in rd.flatten() {
                let p = e.path();
                let name = e.file_name().to_string_lossy().into_owned();
                if p.is_file() && name.starts_with("pk-") && name.ends_with(".json") {
                    names.push(p);
                }
            }
        }
        names.sort_by_key(|p| p.to_string_lossy().into_owned());
        for f in names {
            let text = match fs::read_to_string(&f) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let data: Value = match serde_json::from_str(&text) {
                Ok(d) => d,
                Err(_) => continue,
            };
            if data.get("state").and_then(|v| v.as_str()) == Some("parked") {
                parked += 1;
            }
        }
    }
    let lines = vec![
        "<!-- 派生节：由 projsnap build 再生，手改无效 -->".to_string(),
        format!("- 泊界计数面: 在泊 {parked} 项（投影，真相在链）"),
    ];
    format!("{}\n", lines.join("\n"))
}

fn render(root: &Path) -> Vec<(&'static str, String)> {
    vec![
        ("file-index", section_file_index(root)),
        ("tool-versions", section_tool_versions(root)),
        ("parked-count", section_parked_count(root)),
    ]
}

fn sources_hash(fresh: &[(&'static str, String)]) -> String {
    let mut h = Sha256::new();
    for (name, body) in fresh {
        h.update(name.as_bytes());
        h.update(body.as_bytes());
    }
    hex::encode(h.finalize())
}

// ---- 目标文本拆装（split_target 对表） ----

/// 通用换行形读入（read_text 对表：\r\n 与 \r 归 \n）。
fn read_or_empty(target: &Path) -> Result<String, String> {
    if !target.exists() {
        return Ok(String::new());
    }
    let bytes = fs::read(target).map_err(|e| format!("读取失败: {e}"))?;
    let s = String::from_utf8(bytes).map_err(|e| format!("非 UTF-8: {e}"))?;
    Ok(s.replace("\r\n", "\n").replace('\r', "\n"))
}

fn line_content(line: &str) -> &str {
    line.strip_suffix('\n').unwrap_or(line)
}

/// 文本拆为（前缀, 节名到节块有序对, 尾段）；同节重现按原位覆盖值（dict 语义）。
fn split_target(text: &str) -> (String, Vec<(String, String)>, String) {
    let mut head = String::new();
    let mut sections: Vec<(String, String)> = vec![];
    let mut tail = String::new();
    let mut current: Option<String> = None;
    let mut buf = String::new();
    for line in text.split_inclusive('\n') {
        match &current {
            None => {
                let content = line_content(line);
                let opened = SECTIONS.iter().find(|n| content == &begin_mark(n));
                if let Some(n) = opened {
                    current = Some(n.to_string());
                    buf = line.to_string();
                    continue;
                }
                if sections.is_empty() {
                    head.push_str(line);
                } else {
                    tail.push_str(line);
                }
            }
            Some(cur) => {
                buf.push_str(line);
                if line_content(line) == end_mark(cur) {
                    if let Some(slot) = sections.iter_mut().find(|(n, _)| n == cur) {
                        slot.1 = std::mem::take(&mut buf);
                    } else {
                        sections.push((cur.clone(), std::mem::take(&mut buf)));
                    }
                    current = None;
                }
            }
        }
    }
    if current.is_some() {
        tail.push_str(&buf);
    }
    (head, sections, tail)
}

// ---- build ----

fn cmd_build(root: &Path, target: &Path, as_json: bool) -> i32 {
    let text = match read_or_empty(target) {
        Ok(t) => t,
        Err(e) => return read_fail(&e),
    };
    let fresh = render(root);
    let mut written: Vec<Value> = vec![];
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let mut out = String::new();
    let mut emitted: Vec<&str> = vec![];
    let begin_prefix = "<!-- projsnap:begin ";
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        let content = line_content(line);
        let mut matched: Option<&'static str> = None;
        if content.starts_with(begin_prefix) {
            let rest = &content[begin_prefix.len()..];
            let name = rest.rsplit_once(" -->").map(|(a, _)| a).unwrap_or(rest).trim();
            if let Some(m) = SECTIONS.iter().find(|n| **n == name) {
                matched = Some(m);
            }
        }
        if let Some(m) = matched {
            let mut j = i;
            let endm = end_mark(m);
            while j < lines.len() && line_content(lines[j]) != endm {
                j += 1;
            }
            j += 1;
            let upto = j.min(lines.len());
            let body = &fresh.iter().find(|x| x.0 == m).unwrap().1;
            let new_block = section_block(m, body);
            let old_block: String = lines[i..upto].concat();
            if old_block != new_block {
                written.push(json!({"action": "updated", "section": m}));
            }
            out.push_str(&new_block);
            emitted.push(m);
            i = upto.max(i + 1);
        } else {
            out.push_str(line);
            i += 1;
        }
    }
    for (name, body) in &fresh {
        if !emitted.contains(name) {
            out.push_str(&section_block(name, body));
            written.push(json!({"action": "created", "section": name}));
        }
    }
    let changed = out != text;
    if changed {
        if let Err(e) = fs::write(target, out.as_bytes()) {
            eprintln!("{}", py_dumps(&json!({"error": format!("写入失败: {e}")}), false, false, None));
            return 2;
        }
    }
    if as_json {
        let mut sections = Map::new();
        for (n, b) in &fresh {
            sections.insert(n.to_string(), Value::String(sha256_hex(b.as_bytes())));
        }
        let result = json!({
            "changed": changed,
            "sections": Value::Object(sections),
            "sources_hash": sources_hash(&fresh),
            "status": "ok",
            "tool": "projsnap",
            "version": VERSION,
            "written": written,
        });
        println!("{}", py_dumps(&result, true, false, None));
    }
    0
}

// ---- check ----

fn cmd_check(root: &Path, target: &Path, as_json: bool) -> i32 {
    let text = match read_or_empty(target) {
        Ok(t) => t,
        Err(e) => return read_fail(&e),
    };
    let (_, current, _) = split_target(&text);
    let fresh = render(root);
    let mut stale: Vec<Value> = vec![];
    let mut fresh_ok: Vec<Value> = vec![];
    for (name, body) in &fresh {
        match current.iter().find(|(n, _)| n == name) {
            None => stale.push(json!({"reason": "absent", "section": name})),
            Some((_, block)) if block != &section_block(name, body) => {
                stale.push(json!({"reason": "source_newer", "section": name}))
            }
            Some(_) => fresh_ok.push(json!(name)),
        }
    }
    if as_json {
        let result = json!({
            "fresh": fresh_ok,
            "sources_hash": sources_hash(&fresh),
            "stale": stale,
            "tool": "projsnap",
            "version": VERSION,
        });
        println!("{}", py_dumps(&result, true, false, None));
    }
    if stale.is_empty() {
        0
    } else {
        1
    }
}

// ---- diff（difflib.SequenceMatcher + unified_diff 逐字对表移植） ----

struct SeqMatch {
    a: usize,
    b: usize,
    size: usize,
}

fn find_longest_match(
    a: &[&str],
    b: &[&str],
    b2j: &HashMap<&str, Vec<usize>>,
    bjunk: &HashSet<&str>,
    alo: usize,
    ahi: usize,
    blo: usize,
    bhi: usize,
) -> SeqMatch {
    let (mut besti, mut bestj, mut bestsize) = (alo, blo, 0usize);
    let mut j2len: HashMap<usize, usize> = HashMap::new();
    for i in alo..ahi {
        let mut newj2len: HashMap<usize, usize> = HashMap::new();
        if let Some(js) = b2j.get(a[i]) {
            for &j in js {
                if j < blo {
                    continue;
                }
                if j >= bhi {
                    break;
                }
                // Python j2lenget(j-1, 0)：j==0 时 j-1=-1 恒未命中即 0+1。
                let k = if j == 0 {
                    1
                } else {
                    j2len.get(&(j - 1)).copied().unwrap_or(0) + 1
                };
                newj2len.insert(j, k);
                if k > bestsize {
                    besti = i + 1 - k;
                    bestj = j + 1 - k;
                    bestsize = k;
                }
            }
        }
        j2len = newj2len;
    }
    let isbj = |j: usize| bjunk.contains(b[j]);
    // 非垃圾元边界延伸与垃圾元边界延伸（CPython 双段扩展对表）。
    while besti > alo && bestj > blo && !isbj(bestj - 1) && a[besti - 1] == b[bestj - 1] {
        besti -= 1;
        bestj -= 1;
        bestsize += 1;
    }
    while besti + bestsize < ahi
        && bestj + bestsize < bhi
        && !isbj(bestj + bestsize)
        && a[besti + bestsize] == b[bestj + bestsize]
    {
        bestsize += 1;
    }
    while besti > alo && bestj > blo && isbj(bestj - 1) && a[besti - 1] == b[bestj - 1] {
        besti -= 1;
        bestj -= 1;
        bestsize += 1;
    }
    while besti + bestsize < ahi
        && bestj + bestsize < bhi
        && isbj(bestj + bestsize)
        && a[besti + bestsize] == b[bestj + bestsize]
    {
        bestsize += 1;
    }
    SeqMatch { a: besti, b: bestj, size: bestsize }
}

fn get_matching_blocks(
    a: &[&str],
    b: &[&str],
    b2j: &HashMap<&str, Vec<usize>>,
    bjunk: &HashSet<&str>,
) -> Vec<SeqMatch> {
    let (la, lb) = (a.len(), b.len());
    let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, la, 0, lb)];
    let mut blocks: Vec<SeqMatch> = vec![];
    while let Some((alo, ahi, blo, bhi)) = queue.pop() {
        let m = find_longest_match(a, b, b2j, bjunk, alo, ahi, blo, bhi);
        if m.size > 0 {
            let (ma, mb, msize) = (m.a, m.b, m.size);
            blocks.push(m);
            if alo < ma && blo < mb {
                queue.push((alo, ma, blo, mb));
            }
            if ma + msize < ahi && mb + msize < bhi {
                queue.push((ma + msize, ahi, mb + msize, bhi));
            }
        }
    }
    blocks.sort_by_key(|m| (m.a, m.b, m.size));
    let mut non_adjacent: Vec<SeqMatch> = vec![];
    let (mut i1, mut j1, mut k1) = (0usize, 0usize, 0usize);
    for m in blocks {
        if i1 + k1 == m.a && j1 + k1 == m.b {
            k1 += m.size;
        } else {
            if k1 > 0 {
                non_adjacent.push(SeqMatch { a: i1, b: j1, size: k1 });
            }
            i1 = m.a;
            j1 = m.b;
            k1 = m.size;
        }
    }
    if k1 > 0 {
        non_adjacent.push(SeqMatch { a: i1, b: j1, size: k1 });
    }
    non_adjacent.push(SeqMatch { a: la, b: lb, size: 0 });
    non_adjacent
}

type Opcode = (&'static str, usize, usize, usize, usize);

fn get_opcodes(a: &[&str], b: &[&str], b2j: &HashMap<&str, Vec<usize>>, bjunk: &HashSet<&str>) -> Vec<Opcode> {
    let mut answer: Vec<Opcode> = vec![];
    let (mut i, mut j) = (0usize, 0usize);
    for m in get_matching_blocks(a, b, b2j, bjunk) {
        let (ai, bj, size) = (m.a, m.b, m.size);
        let tag = if i < ai && j < bj {
            "replace"
        } else if i < ai {
            "delete"
        } else if j < bj {
            "insert"
        } else {
            ""
        };
        if !tag.is_empty() {
            answer.push((tag, i, ai, j, bj));
        }
        i = ai + size;
        j = bj + size;
        if size > 0 {
            answer.push(("equal", ai, i, bj, j));
        }
    }
    answer
}

fn get_grouped_opcodes(codes: &[Opcode], n: usize) -> Vec<Vec<Opcode>> {
    let mut codes = codes.to_vec();
    if codes.is_empty() {
        codes = vec![("equal", 0, 1, 0, 1)];
    }
    if codes[0].0 == "equal" {
        let (t, i1, i2, j1, j2) = codes[0];
        codes[0] = (t, i1.max(i2.saturating_sub(n)), i2, j1.max(j2.saturating_sub(n)), j2);
    }
    let last = codes.len() - 1;
    if codes[last].0 == "equal" {
        let (t, i1, i2, j1, j2) = codes[last];
        codes[last] = (t, i1, i2.min(i1 + n), j1, j2.min(j1 + n));
    }
    let nn = n + n;
    let mut groups: Vec<Vec<Opcode>> = vec![];
    let mut group: Vec<Opcode> = vec![];
    for (tag, mut i1, i2, mut j1, j2) in codes {
        if tag == "equal" && i2 - i1 > nn {
            group.push((tag, i1, i2.min(i1 + n), j1, j2.min(j1 + n)));
            groups.push(std::mem::take(&mut group));
            i1 = i1.max(i2.saturating_sub(n));
            j1 = j1.max(j2.saturating_sub(n));
        }
        group.push((tag, i1, i2, j1, j2));
    }
    if !group.is_empty() && !(group.len() == 1 && group[0].0 == "equal") {
        groups.push(group);
    }
    groups
}

fn format_range_unified(start: usize, stop: usize) -> String {
    let beginning = start + 1;
    let length = stop - start;
    if length == 1 {
        return format!("{beginning}");
    }
    if length == 0 {
        return format!("{},{length}", beginning - 1);
    }
    format!("{beginning},{length}")
}

fn unified_diff(a: &[&str], b: &[&str], fromfile: &str, tofile: &str, out: &mut String) {
    let mut b2j: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, line) in b.iter().enumerate() {
        b2j.entry(*line).or_default().push(i);
    }
    let mut bjunk: HashSet<&str> = HashSet::new();
    let n = b.len();
    if n >= 200 {
        let ntest = n / 100 + 1;
        let popular: Vec<&str> = b2j
            .iter()
            .filter(|(_, idxs)| idxs.len() > ntest)
            .map(|(k, _)| *k)
            .collect();
        for p in popular {
            b2j.remove(&p);
            bjunk.insert(p);
        }
    }
    let codes = get_opcodes(a, b, &b2j, &bjunk);
    let groups = get_grouped_opcodes(&codes, 3);
    if groups.is_empty() {
        return;
    }
    out.push_str(&format!("--- {fromfile}\n"));
    out.push_str(&format!("+++ {tofile}\n"));
    for group in &groups {
        let first = group[0];
        let last = group[group.len() - 1];
        out.push_str(&format!(
            "@@ -{} +{} @@\n",
            format_range_unified(first.1, last.2),
            format_range_unified(first.3, last.4)
        ));
        for &(tag, i1, i2, j1, j2) in group {
            if tag == "equal" {
                for line in &a[i1..i2] {
                    out.push(' ');
                    out.push_str(line);
                }
                continue;
            }
            if tag != "insert" {
                for line in &a[i1..i2] {
                    out.push('-');
                    out.push_str(line);
                }
            }
            if tag != "delete" {
                for line in &b[j1..j2] {
                    out.push('+');
                    out.push_str(line);
                }
            }
        }
    }
}

fn cmd_diff(root: &Path, target: &Path) -> i32 {
    let text = match read_or_empty(target) {
        Ok(t) => t,
        Err(e) => return read_fail(&e),
    };
    let (head, current, tail) = split_target(&text);
    let fresh = render(root);
    let old_text = format!(
        "{}{}{}",
        head,
        current.iter().map(|(_, b)| b.as_str()).collect::<String>(),
        tail
    );
    let mut new_mid = String::new();
    for (name, body) in &fresh {
        new_mid.push_str(&section_block(name, body));
    }
    let new_text = format!("{head}{new_mid}{tail}");
    if old_text == new_text {
        return 0;
    }
    let a: Vec<&str> = old_text.split_inclusive('\n').collect();
    let b: Vec<&str> = new_text.split_inclusive('\n').collect();
    let mut stdout = String::new();
    unified_diff(&a, &b, "current", "regenerated", &mut stdout);
    print!("{stdout}");
    1
}

// ---- 入口 ----

fn read_fail(msg: &str) -> i32 {
    eprintln!("{}", py_dumps(&json!({"error": msg}), false, false, None));
    2
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || !matches!(args[0].as_str(), "build" | "check" | "diff") {
        eprintln!(
            "{}",
            py_dumps(
                &json!({"error": "用法 projsnap <build|check|diff> --root <工作区根> [--target <文件>]"}),
                false,
                false,
                None
            )
        );
        exit(2);
    }
    let sub = args[0].clone();
    let mut root = String::from(".");
    let mut target: Option<String> = None;
    let mut as_json = false;
    let mut i = 1usize;
    while i < args.len() {
        if args[i] == "--root" && i + 1 < args.len() {
            root = args[i + 1].clone();
            i += 2;
        } else if args[i] == "--target" && i + 1 < args.len() {
            target = Some(args[i + 1].clone());
            i += 2;
        } else if args[i] == "--json" {
            as_json = true;
            i += 1;
        } else {
            i += 1;
        }
    }
    let root_path = PathBuf::from(root);
    let target_path = target.map(PathBuf::from).unwrap_or_else(|| root_path.join("AGENTS.md"));
    let code = match sub.as_str() {
        "build" => cmd_build(&root_path, &target_path, as_json),
        "check" => cmd_check(&root_path, &target_path, as_json),
        _ => cmd_diff(&root_path, &target_path),
    };
    exit(code);
}
