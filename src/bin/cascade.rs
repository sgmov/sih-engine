//! 引擎侧级联（Cascade）命令行面 —— lease-mergeleg23-parallel 簇E 移植件。
// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//!
//! 行为面移植自围堰 sih-tools/cascade 0.4.0（src/cascade/：cli.py、core.py），
//! 只读对表移植，围堰源码零改动。路径即 id 不改名即删除新建，引用即边，
//! 上游洁净不变式即当前内容哈希等于链上最近认证哈希，只读不写判在别处。
//! 建册核心五件已提取库侧 sih_engine::cascade_registry（pk-055
//! cascadeclose-solo 批，lease close 前投影重建复用），本 bin 行为面零改动。
//!
//! CLI 三子命令，退出码三值：
//!   build   --root <语料根> [--out <册文件>]   0 建册毕 / 2 工具异常
//!   check   --registry <册> --root <语料根> --trail <ndjson>（可重复）--reports-root <报告根>
//!                                          0 上游洁净 / 1 有拒写 blocked / 2 异常
//!   orphans --registry <册> --root <语料根>    0 零孤儿 / 1 有孤儿 / 2 异常
//! 输出 json 报告 sort_keys + indent 2，错误即 stdout 单行 {"error": ...} 退出码二。
//!
//! 落差申报（相对围堰）：
//! 1. 代码载体扫描与 use 边（scan_code/build_use_edges/_rs_entries，围堰经
//!    sih-tools/parser 句读包纯函数链承载）未移植——build 产册 code 节恒空对象、
//!    use_* 注记恒空、代码到文档边与代码到代码边不建；md 语料文档引用边
//!    （scan_corpus/map_tokens/build_edges）全行为移植。含 code 节的围堰册仍可
//!    load/check/orphans（check_targets 与 find_orphans 只读 edges 表）。build
//!    不再耦合句读环境（围堰缺句读即 CascadeError 退出码二，本侧无此耦合）。
//! 2. 注释侧引用词不进映射宇宙（依附落差一，extra_texts 恒空）。
//! 3. runtime_ledger 四计数器（围堰 cli.py 导入未调用的死代码）未移植。
//! 4. trail 事件缺 doc_id 键按跳过处理（围堰 KeyError 未捕获即整工具崩溃形，
//!    本侧如实记行跳过）；content_hashes 键缺省按空表、非对象表亦按空表
//!    （围堰 AttributeError 崩溃形不对齐）；report 值非字符串按缺席跳过。
//! 5. md 语料非 UTF-8 解码失败按 CascadeError 退出码二（围堰 UnicodeDecodeError
//!    未捕获即 traceback 退出码一）；目录名命中 *.md 的病态形按跳过处理。
//! 6. 用法错（缺子命令、缺必填参、未知旗标）为自足解析 stderr 回报，退出码 2
//!    与围堰 argparse 对齐，报文形不对齐。

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::exit;

// 建册核心库件承接（pk-055 cascadeclose-solo 批）：本 bin 建册路径五件
// （版本锚、路径解析、键序归一、建册、正典序列化）原位提取至库侧
// sih_engine::cascade_registry，lease close 前投影重建复用同件，零 shell out；
// 本侧行为面零改动（函数体逐字节承提取前原文）。
use sih_engine::cascade_registry::{build_registry, dump_canonical, resolve_abs, sort_value, ENGINE_VERSION};

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn file_hash(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("file hash failed: {}: {e}", path.display()))?;
    Ok(sha256_hex(&bytes))
}

fn emit_error(msg: &str) -> ! {
    println!("{}", json!({"error": msg}));
    exit(2);
}

fn dump_report(payload: &Value) {
    println!("{}", serde_json::to_string_pretty(&sort_value(payload.clone())).unwrap());
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!(
        "用法: cascade build --root <语料根> [--out <册文件>]\n\
         \x20     cascade check --registry <册> --root <语料根> --trail <ndjson>... --reports-root <报告根>\n\
         \x20     cascade orphans --registry <册> --root <语料根>"
    );
    exit(2);
}

// ---------- 读册（core.py load_registry 对表） ----------

fn load_registry(path: &str) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|_| format!("registry unreadable: {path}"))?;
    let data: Value =
        serde_json::from_str(&text).map_err(|e| format!("registry json invalid: {e}"))?;
    if !data.get("edges").map(|e| e.is_object()).unwrap_or(false) {
        return Err("registry missing edges table".to_string());
    }
    Ok(data)
}

// ---------- 孤儿边侦查（core.py find_orphans 对表，不改名规则的机械兜底） ----------

fn find_orphans(registry: &Value, root: &Path) -> Vec<Value> {
    let mut orphans = Vec::new();
    if let Some(edges) = registry["edges"].as_object() {
        for (rel, upstreams) in edges {
            let Some(ups) = upstreams.as_array() else {
                continue;
            };
            for up in ups {
                let Some(up_str) = up.as_str() else { continue };
                if !root.join(up_str).is_file() {
                    orphans.push(json!({
                        "edge": format!("{rel} -> {up_str}"),
                        "missing": up_str,
                    }));
                }
            }
        }
    }
    orphans.sort_by(|a, b| {
        a["edge"]
            .as_str()
            .unwrap_or_default()
            .cmp(b["edge"].as_str().unwrap_or_default())
    });
    orphans
}

// ---------- 链上最近认证哈希（core.py latest_cert_hashes 对表，后链覆盖前链） ----------

fn latest_cert_hashes(
    trail_paths: &[String],
    reports_root: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut last_event: Map<String, Value> = Map::new();
    for trail_path in trail_paths {
        let text =
            fs::read_to_string(trail_path).map_err(|_| format!("trail unreadable: {trail_path}"))?;
        for line in text.lines() {
            let Ok(event) = serde_json::from_str::<Value>(line) else {
                continue; // 非法行跳过，围堰同语义
            };
            if event.get("event_type").and_then(|v| v.as_str())
                != Some("certification_completed")
            {
                continue;
            }
            if event.get("report_type").and_then(|v| v.as_str()) != Some("scrutinator") {
                continue;
            }
            let Some(doc_id) = event.get("doc_id") else {
                continue; // 围堰 KeyError 崩溃形不对齐（落差四）
            };
            last_event.insert(
                doc_id.to_string(),
                event.get("report").cloned().unwrap_or(Value::Null),
            );
        }
    }
    let mut hashes = BTreeMap::new();
    for (_doc_id, report) in &last_event {
        let Some(report_rel) = report.as_str().filter(|s| !s.is_empty()) else {
            continue;
        };
        let report_path = Path::new(reports_root).join(report_rel);
        if !report_path.is_file() {
            continue;
        }
        let Ok(payload_text) = fs::read_to_string(&report_path) else {
            continue;
        };
        let Ok(payload) = serde_json::from_str::<Value>(&payload_text) else {
            continue;
        };
        let Some(content_hashes) = payload.get("content_hashes").and_then(|v| v.as_object())
        else {
            continue; // 围堰 AttributeError 崩溃形不对齐（落差四）
        };
        for (abs_path, digest) in content_hashes {
            let key = resolve_abs(Path::new(abs_path));
            let digest_text = digest.as_str().unwrap_or_default().to_string();
            hashes.insert(key, digest_text);
        }
    }
    Ok(hashes)
}

// ---------- 上游洁净判定（core.py check_targets 对表） ----------

fn check_targets(registry: &Value, root: &Path, cert_hashes: &BTreeMap<String, String>) -> Value {
    let edges = registry["edges"].as_object().expect("edges validated");
    let mut rels: Vec<&String> = edges.keys().collect();
    rels.sort();
    let resolved_root = resolve_abs(root);
    let mut targets = Map::new();
    for rel in rels {
        let ups: Vec<&str> = edges[rel]
            .as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let mut upstream_states = Map::new();
        for up in &ups {
            let disk = Path::new(&resolved_root).join(up);
            if !disk.is_file() {
                upstream_states.insert(up.to_string(), json!("orphan"));
                continue;
            }
            let current = file_hash(&disk).unwrap_or_default();
            let recorded = cert_hashes.get(&resolve_abs(&disk));
            let state = match recorded {
                None => "unverified",
                Some(r) if *r == current => "clean",
                _ => "dirty",
            };
            upstream_states.insert(up.to_string(), json!(state));
        }
        let dirty: Vec<&String> = upstream_states
            .iter()
            .filter(|(_, v)| v.as_str() == Some("dirty"))
            .map(|(k, _)| k)
            .collect();
        let unverified: Vec<&String> = upstream_states
            .iter()
            .filter(|(_, v)| v.as_str() == Some("unverified"))
            .map(|(k, _)| k)
            .collect();
        let mut dirty_sorted = dirty.clone();
        dirty_sorted.sort();
        let mut unverified_sorted = unverified.clone();
        unverified_sorted.sort();
        let verdict = if dirty_sorted.is_empty() {
            "writable"
        } else {
            "blocked"
        };
        targets.insert(
            rel.clone(),
            json!({
                "dirty": dirty_sorted,
                "unverified": unverified_sorted,
                "upstreams": Value::Object(upstream_states),
                "verdict": verdict,
            }),
        );
    }
    Value::Object(targets)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage_fail("缺子命令（build|check|orphans）");
    }
    let command = args[0].clone();
    let mut opt_root: Option<String> = None;
    let mut opt_out: Option<String> = None;
    let mut opt_registry: Option<String> = None;
    let mut opt_trails: Vec<String> = Vec::new();
    let mut opt_reports_root: Option<String> = None;
    let mut i = 1usize;
    while i < args.len() {
        let a = args[i].clone();
        let take_value = |slot: &mut Option<String>, name: &str, i: &mut usize| {
            *i += 1;
            if *i >= args.len() {
                usage_fail(&format!("{name} 缺值"));
            }
            *slot = Some(args[*i].clone());
        };
        match a.as_str() {
            "--root" => take_value(&mut opt_root, "--root", &mut i),
            "--out" => take_value(&mut opt_out, "--out", &mut i),
            "--registry" => take_value(&mut opt_registry, "--registry", &mut i),
            "--trail" => {
                i += 1;
                if i >= args.len() {
                    usage_fail("--trail 缺值");
                }
                opt_trails.push(args[i].clone());
            }
            "--reports-root" => take_value(&mut opt_reports_root, "--reports-root", &mut i),
            _ if a.starts_with("--") => usage_fail(&format!("未知旗标: {a}")),
            _ => usage_fail(&format!("多余位置参数: {a}")),
        }
        i += 1;
    }

    match command.as_str() {
        "build" => {
            let Some(root) = opt_root else {
                usage_fail("缺 --root（必填）");
            };
            let registry = match build_registry(Path::new(&root)) {
                Ok(r) => r,
                Err(e) => emit_error(&e),
            };
            if let Some(out) = &opt_out {
                let text = dump_canonical(&registry);
                if let Err(e) = fs::write(out, text) {
                    emit_error(&format!("registry write failed: {out}: {e}"));
                }
                dump_report(&json!({"header": registry["header"].clone(), "written": out}));
                exit(0);
            }
            dump_report(&registry);
            exit(0);
        }
        "orphans" => {
            let Some(registry_path) = opt_registry else {
                usage_fail("缺 --registry（必填）");
            };
            let Some(root) = opt_root else {
                usage_fail("缺 --root（必填）");
            };
            let registry = match load_registry(&registry_path) {
                Ok(r) => r,
                Err(e) => emit_error(&e),
            };
            let orphans = find_orphans(&registry, Path::new(&root));
            let report = json!({
                "header": {"tool": {"name": "cascade", "version": ENGINE_VERSION}},
                "orphans": orphans,
                "summary": {"orphans": orphans.len()},
            });
            dump_report(&report);
            exit(if orphans.is_empty() { 0 } else { 1 });
        }
        "check" => {
            let Some(registry_path) = opt_registry else {
                usage_fail("缺 --registry（必填）");
            };
            let Some(root) = opt_root else {
                usage_fail("缺 --root（必填）");
            };
            let Some(reports_root) = opt_reports_root else {
                usage_fail("缺 --reports-root（必填）");
            };
            if opt_trails.is_empty() {
                usage_fail("缺 --trail（必填，可重复）");
            }
            let registry = match load_registry(&registry_path) {
                Ok(r) => r,
                Err(e) => emit_error(&e),
            };
            let cert_hashes = match latest_cert_hashes(&opt_trails, &reports_root) {
                Ok(h) => h,
                Err(e) => emit_error(&e),
            };
            let targets = check_targets(&registry, Path::new(&root), &cert_hashes);
            let blocked = targets
                .as_object()
                .unwrap()
                .values()
                .filter(|t| t["verdict"] == "blocked")
                .count();
            let dirty_edges: usize = targets
                .as_object()
                .unwrap()
                .values()
                .map(|t| t["dirty"].as_array().map(|a| a.len()).unwrap_or(0))
                .sum();
            let unverified_edges: usize = targets
                .as_object()
                .unwrap()
                .values()
                .map(|t| t["unverified"].as_array().map(|a| a.len()).unwrap_or(0))
                .sum();
            let total = targets.as_object().unwrap().len();
            let report = json!({
                "header": {
                    "tool": {"name": "cascade", "version": ENGINE_VERSION},
                    "trails": opt_trails,
                },
                "summary": {
                    "blocked": blocked,
                    "dirty_edges": dirty_edges,
                    "targets": total,
                    "unverified_edges": unverified_edges,
                    "writable": total - blocked,
                },
                "targets": targets,
            });
            dump_report(&report);
            exit(if blocked > 0 { 1 } else { 0 });
        }
        _ => usage_fail(&format!("未知子命令: {command}")),
    }
}
