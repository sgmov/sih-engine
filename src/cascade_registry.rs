//! 级联建册核心库件 —— pk-055 cascadeclose-solo 批自 src/bin/cascade.rs 原位提取。
//! 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面逐字节承 src/bin/cascade.rs 建册路径（围堰 sih-tools/cascade 0.4.0 core.py
//! 对表移植件）：语料扫描、引用词映射、引用即边、建册五件与正典序列化。提取动机：
//! lease close 前投影重建（src/bin/lease/closegate.rs）复用建册核心，零 shell out。
//! bin 行为面零改动：cascade build/check/orphans 三子命令经库件出同字节产物。
//!
//! 落差申报承 bin 侧头注（一至六）不变：code 节恒空对象、注释侧引用词不进映射
//! 宇宙、runtime_ledger 未移植、trail 事件缺键按跳过、语料解码失败 panic 形、
//! 用法错 stderr 退出码二。

use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// 围堰版本锚：sih-tools/cascade/src/cascade/__init__.py __version__。
pub const ENGINE_VERSION: &str = "0.4.0";

/// 引用词形：DEC/GOV/PRO/DES/SPEC 前缀族加三位数字（core.py TOKEN_RE 逐字对表）。
const TOKEN_RE: &str = r"\b(?:DEC|GOV|PRO|DES|SPEC)-\d{3}\b";

/// Python path.resolve() 非严格形：canonicalize 优先，失败退绝对化拼接。
pub fn resolve_abs(p: &Path) -> String {
    if let Ok(c) = std::fs::canonicalize(p) {
        return c.to_string_lossy().into_owned();
    }
    if p.is_absolute() {
        p.to_string_lossy().into_owned()
    } else {
        std::env::current_dir()
            .unwrap_or_default()
            .join(p)
            .to_string_lossy()
            .into_owned()
    }
}

/// 对象键递归排序（json.dumps sort_keys=True 语义）。
pub fn sort_value(v: Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut kvs: Vec<(String, Value)> = m.into_iter().collect();
            kvs.sort_by(|a, b| a.0.cmp(&b.0));
            let mut out = Map::new();
            for (k, val) in kvs {
                out.insert(k, sort_value(val));
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.into_iter().map(sort_value).collect()),
        other => other,
    }
}

// ---------- 语料扫描（core.py scan_corpus 对表） ----------

fn scan_corpus(root: &Path) -> Result<BTreeMap<String, String>, String> {
    if !root.is_dir() {
        return Err(format!("corpus root missing: {}", root.display()));
    }
    let mut docs = BTreeMap::new();
    fn walk(dir: &Path, root: &Path, docs: &mut BTreeMap<String, String>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, root, docs);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                let Ok(rel) = path.strip_prefix(root) else {
                    continue;
                };
                let rel_posix = rel
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/");
                match std::fs::read(&path).map_err(|e| e.to_string()).and_then(|b| {
                    String::from_utf8(b).map_err(|e| e.to_string())
                }) {
                    Ok(text) => {
                        docs.insert(rel_posix, text);
                    }
                    Err(e) => {
                        // 围堰 UnicodeDecodeError 崩溃形不对齐（落差五）。
                        panic!("corpus file unreadable: {}: {e}", path.display())
                    }
                }
            }
        }
    }
    walk(root, root, &mut docs);
    Ok(docs)
}

// ---------- 引用词映射（core.py map_tokens 对表） ----------

/// 引用词到路径的映射：前缀族优先精确匹配，多义入注记，无主入注记。
fn map_tokens(
    docs: &BTreeMap<String, String>,
) -> (BTreeMap<String, String>, Vec<String>, Vec<String>) {
    let re = regex::Regex::new(TOKEN_RE).unwrap();
    let mut by_name: BTreeMap<String, String> = BTreeMap::new();
    for rel in docs.keys() {
        let name = match rel.rsplit('/').next() {
            Some(n) => n.to_string(),
            None => continue,
        };
        by_name.entry(name).or_insert_with(|| rel.clone());
    }
    let mut tokens: BTreeSet<String> = BTreeSet::new();
    for text in docs.values() {
        for m in re.find_iter(text) {
            tokens.insert(m.as_str().to_string());
        }
    }
    let mut mapping = BTreeMap::new();
    let mut ambiguous = Vec::new();
    let mut unmapped = Vec::new();
    for token in &tokens {
        let Some((family, number)) = token.split_once('-') else {
            continue;
        };
        let prefix_dash = format!("{family}-{number}-");
        let prefix_dot = format!("{family}-{number}.");
        let mut candidates: Vec<&String> = by_name
            .iter()
            .filter(|(name, _)| name.starts_with(&prefix_dash) || name.starts_with(&prefix_dot))
            .map(|(_, rel)| rel)
            .collect();
        candidates.sort();
        let bare_prefix = format!("{number}-");
        let mut bare: Vec<&String> = if family == "DEC" {
            by_name
                .iter()
                .filter(|(name, _)| name.starts_with(&bare_prefix))
                .map(|(_, rel)| rel)
                .collect()
        } else {
            vec![]
        };
        bare.sort();
        if candidates.len() == 1 {
            mapping.insert(token.clone(), candidates[0].clone());
        } else if !candidates.is_empty() {
            ambiguous.push(token.clone());
        } else if !bare.is_empty() && bare.len() == 1 && family == "DEC" {
            mapping.insert(token.clone(), bare[0].clone());
        } else {
            unmapped.push(token.clone());
        }
    }
    (mapping, ambiguous, unmapped)
}

// ---------- 引用即边（core.py build_edges 对表，d ≻ u 上游集） ----------

fn build_edges(docs: &BTreeMap<String, String>, mapping: &BTreeMap<String, String>) -> BTreeMap<String, Vec<String>> {
    let re = regex::Regex::new(TOKEN_RE).unwrap();
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (rel, text) in docs {
        let mut upstreams: BTreeSet<String> = BTreeSet::new();
        for m in re.find_iter(text) {
            if let Some(target) = mapping.get(m.as_str()) {
                if target != rel {
                    upstreams.insert(target.clone());
                }
            }
        }
        if !upstreams.is_empty() {
            edges.insert(rel.clone(), upstreams);
        }
    }
    edges
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect()
}

// ---------- 建册（core.py build_registry 对表，落差一二：code 节恒空） ----------

/// 语料根与册记 root 同位形（bin cascade build 承接位）。
pub fn build_registry(root: &Path) -> Result<Value, String> {
    build_registry_recorded(root, root)
}

/// 建册核心（pk-055 投影重建承接位）：语料根与册记 root 可分离——close 前重建
/// 自工地语料建册（归并后态近似）而册记 root 锚主树语料位，防工地路径进册
/// header 致每次收约伪 diff。册形五节（edges/header/ledger/code/notes）零改。
pub fn build_registry_recorded(root: &Path, recorded_root: &Path) -> Result<Value, String> {
    let docs = scan_corpus(root)?;
    let (mapping, ambiguous, unmapped) = map_tokens(&docs);
    let edges = build_edges(&docs, &mapping);
    let mut ledger = Map::new();
    for (rel, ups) in &edges {
        let mut entry = Map::new();
        for up in ups {
            entry.insert(
                up.clone(),
                json!({"upstream_changes": 0, "consulted": 0, "blocked": 0, "hits": 0}),
            );
        }
        ledger.insert(rel.clone(), Value::Object(entry));
    }
    Ok(json!({
        "edges": edges,
        "header": {
            "root": resolve_abs(recorded_root),
            "tool": {"name": "cascade", "version": ENGINE_VERSION},
        },
        "ledger": Value::Object(ledger),
        "code": {},
        "notes": {
            "ambiguous": ambiguous,
            "unmapped": unmapped,
            "use_ambiguous": [],
            "use_unmapped": [],
            "use_wildcard": [],
        },
    }))
}

/// 册文件正典序列化形：sort_keys + indent 2 + 尾换行（bin cascade build --out
/// 落盘形逐字节对表），close 前重建落盘与在盘比对同用此形。
pub fn dump_canonical(v: &Value) -> String {
    let mut text = serde_json::to_string_pretty(&sort_value(v.clone())).unwrap();
    text.push('\n');
    text
}
