//! wikirecall 三通道确定性召回引擎 bin —— lease-mergeleg6-parallel 簇H 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/wikirecall（recall.py 召回主链与 semantic.py 语义层），
//! 只读对表移植，围堰源码零改动。零模型零网络：词面打触发集、骨架全读清单、
//! 关系图一跳；缺省判定路径种子由语义通道承担（语料内 TF-IDF 统计加权与余弦
//! 相似度全量暴力排序前 K=3 正分命中），并集出应读书单，同输入逐字节同输出。
//!
//! CLI：`wikirecall --repo <数学仓路径> --query <词> [--query ...]
//!       [--aliases <别名件.json>] [--semantic K] [--word] [--out <out.json>]`
//! 缺省裁决（调用位对表 recall.resolve_semantic_k）：--word 显式回退即词面种子；
//! --semantic 显式给参即其值（0 仍即关）；无旗标缺省即语义层 K=3。
//! 退出码三值：0 = 召回成功（含零命中与空仓）；1 = 本面未用（围堰 recall 恒成功，
//! 退出码一属未移植的 checkcite 消费面）；2 = 用法错或工具自身异常。
//!
//! 零网络红线核对（SPEC-025 A9）：围堰全源零外部网络调用，纯本地语料检索，
//! 无在线调用面需降级申报。未移植面（边缘伴生，非召回主链）：checkcite.py
//! 消费闭环审计、metrics.py 召回度量、ab_run.py A/B 与两件 selftest。
//!
//! 落差申报（相对围堰，零粉饰）：
//! 1. semantic_scores 浮点按 Python repr 形（最短表示、指数两位下限、科学计数
//!    阈值 exp<-4 或 >=16）序列化；范数累加对表围堰 Python 3.12+ builtins.sum
//!    的 Neumaier 补偿求和形（实测与围堰 3.14 位级一致），Python <=3.11 朴素
//!    求和基线不在对表范围；超越函数平台库 1ulp 边缘理论存在、实测未见。
//! 2. 行界按 \n 与行尾 \r 处理，Python splitlines/re.M 的 Unicode 行界字
//!    （\u2028 等）不切行。
//! 3. argparse 缩写旗标（--rep 代 --repo）不对齐，旗标须全拼。
//! 4. 别名值非字符串与条目读取 IO 异常：围堰即 Python 崩溃（traceback 退出码一），
//!    本件归退出码二硬失败不静默，报文不对齐。
//! 5. 条目 id 重号后文件覆盖先文件与围堰 dict 语义一致；目录与输出遍历恒排序。

use regex::Regex;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::OnceLock;

/// 缺省语义通道前 K 正分命中（recall.DEFAULT_SEMANTIC_K 对表）。
const DEFAULT_SEMANTIC_K: i64 = 3;

/// 条目扫描分支（recall.BRANCHES 对表）。
const BRANCHES: [&str; 5] = [
    "probability/entries",
    "order/entries",
    "topology/entries",
    "algebra/entries",
    "calculus/llm-friendly-build/entries",
];

/// 骨架全读清单（recall.SKELETON 对表，在册即入书单）。
const SKELETON: [&str; 6] = [
    "llm-friendly-build/mapping.md",
    "calculus/llm-friendly-build/mapping.md",
    "probability/INDEX.md",
    "order/INDEX.md",
    "topology/INDEX.md",
    "algebra/INDEX.md",
];

fn id_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"\b(?:PROB|ORD|TOP|ALG|LIM|DIFF|INT|SER|MUL|SPEC|HIS|NS|APP)-\d+\b").unwrap()
    })
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!(
        "用法: wikirecall --repo <数学仓路径> --query <词> [--query ...] \
         [--aliases <别名件.json>] [--semantic K] [--word] [--out <out.json>]"
    );
    exit(2);
}

// ---------- 条目解析与索引（recall.parse_entry / build_index 对表） ----------

struct Entry {
    id: String,
    path: String,
    #[allow(dead_code)]
    title: String,
    rel: Vec<String>,
    /// 通道对价文本面：title + 换行 + 触发问句逐行连接（semantic._hay_of 同源）。
    hay: String,
}

fn parse_entry(path: &Path) -> Result<Entry, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("条目不可读: {}: {}", path.display(), e))?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let own = id_re().find(&name).map(|m| m.as_str().to_string());
    // 标题：首个 "# " 前缀行（re ^# (.+)$ 逐行语义，组内至少一字）。
    let mut title = String::new();
    for line in text.split('\n') {
        if let Some(rest) = line.strip_prefix("# ") {
            if !rest.is_empty() {
                title = rest.to_string();
                break;
            }
        }
    }
    // 触发问句：## 触发问题 {#triggers} 节内 "- " 行（TRIG_RE 对表）。
    let lines: Vec<&str> = text.split('\n').collect();
    let mut triggers: Vec<String> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if line.trim_end() == "## 触发问题 {#triggers}" {
            for l in lines.iter().skip(i + 1) {
                if l.starts_with("## ") {
                    break;
                }
                if let Some(rest) = l.strip_prefix("- ") {
                    triggers.push(rest.trim().to_string());
                }
            }
            break;
        }
    }
    // 关系面：全文 ID 集（去自身）排序。
    let mut ids: BTreeSet<String> =
        id_re().find_iter(&text).map(|m| m.as_str().to_string()).collect();
    if let Some(o) = &own {
        ids.remove(o);
    }
    let rel: Vec<String> = ids.into_iter().collect();
    let hay = format!("{}\n{}", title, triggers.join("\n"));
    Ok(Entry {
        id: own.unwrap_or_else(|| {
            path.file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        }),
        path: path.to_string_lossy().into_owned(),
        title,
        rel,
        hay,
    })
}

fn build_index(repo: &Path) -> Result<BTreeMap<String, Entry>, String> {
    let mut entries = BTreeMap::new();
    for b in BRANCHES {
        let d = repo.join(b);
        if !d.is_dir() {
            continue;
        }
        let rd =
            fs::read_dir(&d).map_err(|e| format!("目录不可读: {}: {}", d.display(), e))?;
        let mut files: Vec<PathBuf> = vec![];
        for e in rd.flatten() {
            let p = e.path();
            let is_md = p
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| x == "md")
                .unwrap_or(false);
            if p.is_file() && is_md {
                files.push(p);
            }
        }
        files.sort_by(|a, b| {
            a.file_name()
                .unwrap_or_default()
                .cmp(&b.file_name().unwrap_or_default())
        });
        for f in files {
            let e = parse_entry(&f)?;
            entries.insert(e.id.clone(), e);
        }
    }
    Ok(entries)
}

// ---------- 词面与图通道（recall.word_channel / graph_channel 对表） ----------

fn word_channel(index: &BTreeMap<String, Entry>, terms: &[String]) -> BTreeSet<String> {
    let mut hits = BTreeSet::new();
    for (eid, e) in index {
        let low = e.hay.to_lowercase();
        for q in terms {
            if !q.is_empty() && low.contains(&q.to_lowercase()) {
                hits.insert(eid.clone());
                break;
            }
        }
    }
    hits
}

fn graph_channel(index: &BTreeMap<String, Entry>, seeds: &BTreeSet<String>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for eid in seeds {
        if let Some(e) = index.get(eid) {
            for r in &e.rel {
                if index.contains_key(r) && r != eid {
                    out.insert(r.clone());
                }
            }
        }
    }
    for (eid, e) in index {
        if seeds.contains(eid) {
            continue;
        }
        if e.rel.iter().any(|r| seeds.contains(r)) {
            out.insert(eid.clone());
        }
    }
    let seeds = seeds.clone();
    out.retain(|x| !seeds.contains(x));
    out
}

// ---------- 别名与词项展开（recall.load_aliases / expand_terms 对表） ----------

fn load_aliases(path: Option<&str>) -> Result<BTreeMap<String, String>, String> {
    let Some(p) = path else {
        return Ok(BTreeMap::new());
    };
    let text =
        fs::read_to_string(p).map_err(|e| format!("别名件不可读: {p}: {e}"))?;
    let data: Value =
        serde_json::from_str(&text).map_err(|e| format!("别名件不可解析: {p}: {e}"))?;
    let obj = data
        .as_object()
        .ok_or_else(|| format!("别名件顶层须为对象: {p}"))?;
    let mut out = BTreeMap::new();
    for (k, v) in obj {
        let Some(s) = v.as_str() else {
            return Err(format!("别名件含非字符串值: {p}: {k}"));
        };
        out.insert(k.clone(), s.to_string());
    }
    Ok(out)
}

fn expand_terms(
    terms: &[String],
    aliases: &BTreeMap<String, String>,
) -> Result<Vec<String>, String> {
    let mut out: BTreeSet<String> = BTreeSet::new();
    for t in terms {
        out.insert(t.clone());
        if let Some(a) = aliases.get(t) {
            out.insert(a.clone());
        }
    }
    Ok(out.into_iter().collect())
}

// ---------- 语义层（semantic.py 逐面对表） ----------

/// 词元化：小写化后拉丁连续 [a-z0-9]+ 串与汉字重叠二字组（奇数长段尾落单字）。
fn tokenize(text: &str) -> Vec<String> {
    let lowered = text.to_lowercase();
    let mut tokens: Vec<String> = vec![];
    let mut cur = String::new();
    for c in lowered.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            cur.push(c);
        } else if !cur.is_empty() {
            tokens.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }
    let mut run: Vec<char> = vec![];
    for c in lowered.chars() {
        if ('\u{4e00}'..='\u{9fff}').contains(&c) {
            run.push(c);
        } else if !run.is_empty() {
            push_cjk_run(&mut tokens, &run);
            run.clear();
        }
    }
    if !run.is_empty() {
        push_cjk_run(&mut tokens, &run);
    }
    tokens
}

fn push_cjk_run(tokens: &mut Vec<String>, run: &[char]) {
    for i in 0..run.len().saturating_sub(1) {
        let mut bigram = String::new();
        bigram.push(run[i]);
        bigram.push(run[i + 1]);
        tokens.push(bigram);
    }
    if run.len() % 2 == 1 {
        tokens.push(run[run.len() - 1].to_string());
    }
}

/// 词元计数（BTreeMap 迭代恒按词元排序，累加序确定）。
fn term_counts(text: &str) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for tok in tokenize(text) {
        *counts.entry(tok).or_insert(0) += 1;
    }
    counts
}

struct Stats {
    n_docs: usize,
    docs: BTreeMap<String, BTreeMap<String, usize>>,
    df: BTreeMap<String, usize>,
}

fn build_stats(index: &BTreeMap<String, Entry>) -> Stats {
    let mut docs = BTreeMap::new();
    let mut df: BTreeMap<String, usize> = BTreeMap::new();
    for (eid, e) in index {
        let tc = term_counts(&e.hay);
        for tok in tc.keys() {
            *df.entry(tok.clone()).or_insert(0) += 1;
        }
        docs.insert(eid.clone(), tc);
    }
    Stats { n_docs: index.len(), docs, df }
}

/// 逆文档频率 log2(N/df)，df=0 或 df=N 即 0.0（零鉴别力退化）。
fn idf(stats: &Stats, tok: &str) -> f64 {
    let n = stats.n_docs;
    let d = *stats.df.get(tok).unwrap_or(&0);
    if d == 0 || d >= n {
        return 0.0;
    }
    (n as f64 / d as f64).log2()
}

fn query_vector(stats: &Stats, text: &str) -> BTreeMap<String, f64> {
    let mut vec = BTreeMap::new();
    for (tok, cnt) in term_counts(text) {
        let w = idf(stats, &tok);
        if w > 0.0 {
            vec.insert(tok, cnt as f64 * w);
        }
    }
    vec
}

fn doc_vector(stats: &Stats, eid: &str) -> BTreeMap<String, f64> {
    let mut vec = BTreeMap::new();
    if let Some(doc) = stats.docs.get(eid) {
        for (tok, cnt) in doc {
            let w = idf(stats, tok);
            if w > 0.0 {
                vec.insert(tok.clone(), *cnt as f64 * w);
            }
        }
    }
    vec
}

/// Python 3.12+ builtins.sum 浮点路径的 Neumaier 补偿求和对表
/// （围堰语义层范数累加位即此形；朴素顺序累加在长和上差 1-2 ulp）。
fn py_sum_f64(values: impl Iterator<Item = f64>) -> f64 {
    let mut total = 0.0f64;
    let mut c = 0.0f64;
    for x in values {
        let t = total + x;
        if total.abs() >= x.abs() {
            c += (total - t) + x;
        } else {
            c += (x - t) + total;
        }
        total = t;
    }
    total + c
}

/// 余弦相似度：累加按排序词元序固定；零向量即 0.0。
fn cosine(q: &BTreeMap<String, f64>, d: &BTreeMap<String, f64>) -> f64 {
    let mut dot = 0.0;
    for (tok, qv) in q {
        if let Some(dv) = d.get(tok) {
            dot += qv * dv;
        }
    }
    let nq: f64 = py_sum_f64(q.values().map(|v| v * v)).sqrt();
    let nd: f64 = py_sum_f64(d.values().map(|v| v * v)).sqrt();
    if nq == 0.0 || nd == 0.0 {
        return 0.0;
    }
    dot / (nq * nd)
}

/// 语义通道：全量暴力余弦排序取前 K 正分命中，序决胜为得分降序加条目 id 升序。
fn semantic_channel(
    index: &BTreeMap<String, Entry>,
    terms: &[String],
    k: i64,
) -> (Vec<String>, BTreeMap<String, f64>) {
    let stats = build_stats(index);
    let qtext = terms.join("\n");
    let qvec = query_vector(&stats, &qtext);
    let mut scored: Vec<(String, f64)> = vec![];
    for eid in index.keys() {
        let dv = doc_vector(&stats, eid);
        let score = cosine(&qvec, &dv);
        if score > 0.0 {
            scored.push((eid.clone(), score));
        }
    }
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });
    let take = k.max(0) as usize;
    let hits: Vec<String> = scored.iter().take(take).map(|(e, _)| e.clone()).collect();
    let scores: BTreeMap<String, f64> = scored.into_iter().collect();
    (hits, scores)
}

// ---------- 召回主链（recall.recall 对表） ----------

fn recall(
    repo: &str,
    terms: &[String],
    aliases_path: Option<&str>,
    semantic_k: Option<i64>,
) -> Result<Value, String> {
    let repo_path = Path::new(repo);
    let index = build_index(repo_path)?;
    let aliases = load_aliases(aliases_path)?;
    let expanded = expand_terms(terms, &aliases)?;
    let word = word_channel(&index, &expanded);
    let sem = semantic_k.map(|k| semantic_channel(&index, &expanded, k));
    let seeds: BTreeSet<String> = match &sem {
        Some((hits, _)) => hits.iter().cloned().collect(),
        None => word.clone(),
    };
    let graph = graph_channel(&index, &seeds);
    let skeleton: Vec<String> = SKELETON
        .iter()
        .filter(|s| repo_path.join(s).exists())
        .map(|s| s.to_string())
        .collect();
    let ids: Vec<String> = seeds.union(&graph).cloned().collect();
    let entry_paths: Vec<String> = ids.iter().map(|i| index[i].path.clone()).collect();

    let mut prov = Map::new();
    for id in &ids {
        let mut src = vec![];
        if word.contains(id) {
            src.push(json!("word"));
        }
        if let Some((hits, _)) = &sem {
            if hits.contains(id) {
                src.push(json!("semantic"));
            }
        }
        if graph.contains(id) {
            src.push(json!("graph"));
        }
        prov.insert(id.clone(), Value::Array(src));
    }

    let mut channels = Map::new();
    channels.insert("word".into(), json!(word));
    channels.insert("graph".into(), json!(graph));
    channels.insert("skeleton".into(), json!(skeleton));
    if let Some((hits, scores)) = &sem {
        channels.insert("semantic".into(), json!(hits));
        let mut sm = Map::new();
        for h in hits {
            sm.insert(h.clone(), json!(scores[h]));
        }
        channels.insert("semantic_scores".into(), Value::Object(sm));
    }

    let mut sk_sorted = skeleton.clone();
    sk_sorted.sort();
    let mut reading = Map::new();
    reading.insert("skeleton_files".into(), json!(sk_sorted));
    reading.insert("entry_ids".into(), json!(ids));
    reading.insert("entry_paths".into(), json!(entry_paths));

    let mut plan = Map::new();
    plan.insert("repo".into(), json!(repo));
    plan.insert("query".into(), json!(terms));
    plan.insert("expanded".into(), json!(expanded));
    plan.insert("channels".into(), Value::Object(channels));
    plan.insert("reading_list".into(), Value::Object(reading));
    plan.insert("provenance".into(), Value::Object(prov));
    Ok(Value::Object(plan))
}

// ---------- 出参序列化：json.dumps(sort_keys=True, indent=1, ensure_ascii=False) 对表 ----------

fn dump_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Python float repr 形：最短表示；exp<-4 或 >=16 走科学计数，指数至少两位。
fn py_float_repr(f: f64) -> String {
    if f.is_nan() {
        return "NaN".into();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-Infinity".into() } else { "Infinity".into() };
    }
    if f == 0.0 {
        return if f.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    let s = format!("{:e}", f);
    let (mant, exp_s) = s.split_once('e').unwrap_or((s.as_str(), "0"));
    let exp: i32 = exp_s.parse().unwrap_or(0);
    let neg = mant.starts_with('-');
    let digits: String = mant.trim_start_matches('-').chars().filter(|c| *c != '.').collect();
    let sign = if neg { "-" } else { "" };
    if (-4..16).contains(&exp) {
        let mut out = String::from(sign);
        let e = exp;
        if e < 0 {
            out.push_str("0.");
            for _ in 0..(-e - 1) {
                out.push('0');
            }
            out.push_str(&digits);
        } else if (e as usize) + 1 >= digits.len() {
            out.push_str(&digits);
            for _ in 0..((e as usize + 1) - digits.len()) {
                out.push('0');
            }
            out.push_str(".0");
        } else {
            out.push_str(&digits[..(e + 1) as usize]);
            out.push('.');
            out.push_str(&digits[(e + 1) as usize..]);
        }
        out
    } else {
        let m = if digits.len() > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            digits.clone()
        };
        let ae = exp.abs();
        format!("{sign}{m}e{}{:02}", if exp < 0 { '-' } else { '+' }, ae)
    }
}

fn dump_val(v: &Value, depth: usize, out: &mut String) {
    match v {
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
            kvs.sort_by(|a, b| a.0.cmp(b.0));
            out.push_str("{\n");
            for (i, (k, val)) in kvs.iter().enumerate() {
                out.push_str(&" ".repeat(depth + 1));
                dump_string(k, out);
                out.push_str(": ");
                dump_val(val, depth + 1, out);
                if i + 1 < kvs.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&" ".repeat(depth));
            out.push('}');
        }
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (i, val) in a.iter().enumerate() {
                out.push_str(&" ".repeat(depth + 1));
                dump_val(val, depth + 1, out);
                if i + 1 < a.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&" ".repeat(depth));
            out.push(']');
        }
        Value::String(s) => dump_string(s, out),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                out.push_str(&i.to_string());
            } else if let Some(u) = n.as_u64() {
                out.push_str(&u.to_string());
            } else if let Some(f) = n.as_f64() {
                out.push_str(&py_float_repr(f));
            }
        }
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Null => out.push_str("null"),
    }
}

fn py_dump_sorted(v: &Value) -> String {
    let mut out = String::new();
    dump_val(v, 0, &mut out);
    out
}

// ---------- CLI ----------

fn take_value(args: &[String], i: &mut usize, inline: Option<String>, flag: &str) -> String {
    if let Some(v) = inline {
        return v;
    }
    let n = *i + 1;
    if n >= args.len() {
        usage_fail(&format!("{flag} 缺值"));
    }
    *i = n;
    args[n].clone()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut repo: Option<String> = None;
    let mut queries: Vec<String> = vec![];
    let mut aliases: Option<String> = None;
    let mut semantic_arg: Option<i64> = None;
    let mut word_flag = false;
    let mut out: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].clone();
        let (flag, inline) = match a.split_once('=') {
            Some((f, v)) if f.starts_with("--") => (f.to_string(), Some(v.to_string())),
            _ => (a.clone(), None),
        };
        match flag.as_str() {
            "--repo" => repo = Some(take_value(&args, &mut i, inline, "--repo")),
            "--query" => queries.push(take_value(&args, &mut i, inline, "--query")),
            "--aliases" => aliases = Some(take_value(&args, &mut i, inline, "--aliases")),
            "--semantic" => {
                let v = take_value(&args, &mut i, inline, "--semantic");
                semantic_arg = Some(v.parse::<i64>().unwrap_or_else(|_| usage_fail("--semantic 须为整数")));
            }
            "--word" => {
                if inline.is_some() {
                    usage_fail("--word 不取值");
                }
                word_flag = true;
            }
            "--out" => out = Some(take_value(&args, &mut i, inline, "--out")),
            _ => usage_fail(&format!("未知旗标: {a}")),
        }
        i += 1;
    }
    let Some(repo) = repo else { usage_fail("缺 --repo（必填）") };
    // 调用位缺省裁决：--word 即词面；--semantic 显式 0 即关；无旗标缺省 K=3。
    let semantic_k: Option<i64> = if word_flag {
        None
    } else {
        match semantic_arg {
            None => Some(DEFAULT_SEMANTIC_K),
            Some(0) => None,
            Some(k) => Some(k),
        }
    };
    let plan = match recall(&repo, &queries, aliases.as_deref(), semantic_k) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("工具异常: {e}");
            exit(2);
        }
    };
    let text = py_dump_sorted(&plan);
    if let Some(o) = &out {
        if let Err(e) = fs::write(o, format!("{text}\n")) {
            eprintln!("工具异常: 落档失败: {o}: {e}");
            exit(2);
        }
    }
    println!("{text}");
}
