//! tally——执契的确定性基线核对与终签编排（融回自围堰
//! sih-tools/tally/src/tally/cli.py 全件，契约源 DES-011）。
//!
//! check 对一份裁决材料（kind tally-check-input）执行 R2 至 R7 规则核对与
//! 三态映射落四值处置；R1 由实现本身承载（版本署名、材料只读）。verify 对
//! 同输入重跑并逐字节比对报告（可重放即同判）。sign 仅在处置为裁决通过时
//! 把核对报告经引擎 scribe crosscheck 子命令落据（机器终签，写权走书简管
//! 线，listzero-solo 五修后现文）。assemble 确定性装配 tally-check-input。
//! watch 边界变更监视批量重放浮异常视图。
//!
//! 判定规约形态（des-011 随迁）：七规则与三态映射为代码承载核对逻辑，
//! rules_version 随材料声明代际（缺省 des-011-r1）；闸三态映射四值处置按
//! DES-011 优先级：席位异常挂起先于闸三态；boundary 打回重作；
//! near_threshold 挂起；stable_clear 且核对全过则裁决通过、方向沿用席位
//! 众数；任一规则失败则材料退回。本腿不重新计算语义方向。
//!
//! 退出码对齐围堰：check/verify 0 全过或一致、1 失败或不一致、2 用法或
//! 环境错误；sign refused 1、scribe 失败透传其退出码、signed 0；watch
//! 无异常 0 有异常 1。

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Map, Value};

use super::jsonc::{
    canonical_json, io_to_py, jsonerr, keyerr, parse_json, py_compact_sorted, py_int_list_repr,
    verr, PyError,
};

pub const VERSION: &str = "1.0.0";
pub const CRITERIA_VERSION: &str = "v3";
// [constclear2c] 登记行 f1 态工程实践三件套｜账面 sih-math/docs/constclear2-routing-2026-09-08.md
pub const BUDGET_PER_GID: i64 = 9;
pub const RULES_VERSION: &str = "des-011-r1";

const PER_ACTOR_FIELDS: [&str; 7] = [
    "actor_id",
    "model_id",
    "family",
    "decision",
    "basis_regulation",
    "boundary_flag",
    "reason",
];

fn r5_state_of(raw: &str) -> &'static str {
    match raw {
        "可用" => "ok",
        "漂移告警" => "suspend",
        "基线异常" => "abnormal",
        _ => "",
    }
}

/// rules_version 形校验（与引擎既有 crosscheck 守卫同形：des 三位号 r 数字形）。
pub fn is_rules_version(s: &str) -> bool {
    let rest = match s.strip_prefix("des-") {
        Some(r) => r,
        None => return false,
    };
    let (num, rev) = match rest.split_once("-r") {
        Some((n, r)) => (n, r),
        None => return false,
    };
    num.len() == 3 && num.chars().all(|c| c.is_ascii_digit()) && !rev.is_empty() && rev.chars().all(|c| c.is_ascii_digit())
}

fn sha256_file(path: &Path) -> Result<String, PyError> {
    let bytes = std::fs::read(path).map_err(|e| io_to_py(&e))?;
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(&bytes);
    Ok(hex::encode(h.finalize()))
}

/// trail → dc_list 与 flywheel_run 原始行。格式错行跳过。
fn load_dc_list(trail_path: &Path) -> Result<(Vec<Value>, Vec<Value>), PyError> {
    let text = std::fs::read_to_string(trail_path).map_err(|e| io_to_py(&e))?;
    let mut dc_list: Vec<Value> = Vec::new();
    let mut runs: Vec<Value> = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let r: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if r.get("trail_type").and_then(|v| v.as_str()) == Some("flywheel_run") {
            runs.push(r.clone());
            if let Some(dc) = r.get("decision_convergence") {
                if dc.is_object() && dc.get("per_actor").is_some() {
                    dc_list.push(dc.clone());
                }
            }
        }
    }
    Ok((dc_list, runs))
}

/// dc 指纹：紧凑排序形 sha256 前 16 位（v3 公式）。
fn fingerprint(dc_list: &[Value]) -> String {
    let payload = py_compact_sorted(&Value::Array(dc_list.to_vec()));
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(payload.as_bytes());
    hex::encode(h.finalize())[..16].to_string()
}

/// 方向沿用席位众数（平局取先遇，围堰 Counter.most_common 稳定语义）。
fn modal_direction(dc_list: &[Value]) -> Option<String> {
    let mut order: Vec<String> = Vec::new();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for dc in dc_list {
        if let Some(pas) = dc.get("per_actor").and_then(|v| v.as_array()) {
            for pa in pas {
                if let Some(d) = pa.get("decision").and_then(|v| v.as_str()) {
                    if !d.is_empty() {
                        if !order.iter().any(|x| x == d) {
                            order.push(d.to_string());
                        }
                        *counts.entry(d.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    if order.is_empty() {
        return None;
    }
    let mut best = order[0].clone();
    for d in order.iter().skip(1) {
        if counts[d] > counts[&best] {
            best = d.clone();
        }
    }
    Some(best)
}

fn get_str<'a>(m: &'a Value, k: &str) -> Option<&'a str> {
    m.get(k).and_then(|v| v.as_str()).filter(|s| !s.is_empty())
}

fn fail(failed: &mut Vec<Value>, rule: &str, where_: impl Into<String>) {
    failed.push(json!({"rule": rule, "where": where_.into()}));
}

/// 全量核对：R2 至 R7 逐条 + 三态映射落处置。只读，零网络零 LLM。
pub fn check_material(material_path: &Path) -> Result<Value, PyError> {
    let text = std::fs::read_to_string(material_path).map_err(|e| io_to_py(&e))?;
    let material = parse_json(&text)?;
    let kind = material.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    if kind != "tally-check-input" {
        return Err(verr(format!("材料 kind 不符：{}", kind)));
    }

    let gid = material.get("gid").cloned().unwrap_or(Value::Null);
    let verdict = material.get("gate_verdict").cloned().unwrap_or(Value::Null);
    let verdict_str = match &verdict {
        Value::String(s) => s.clone(),
        Value::Null => "None".to_string(),
        other => other.to_string(),
    };

    let mut passed: Vec<Value> = Vec::new();
    let mut failed: Vec<Value> = Vec::new();
    let mut alarms: Vec<Value> = Vec::new();

    // R1 执行者归属：版本署名于此；材料全程只读（本函数无任何写操作）
    passed.push(json!(format!("R1: 核对程序版本署名 tally/{}，材料只读消费", VERSION)));

    // R2 来源可追溯：三关键字段、所指文件存在、哈希复算一致
    for field in ["gid", "topic_sha256", "dc_fingerprint"] {
        if material.get(field).is_none() {
            fail(&mut failed, "R2", format!("材料缺字段 {}", field));
        }
    }
    let topic: Option<PathBuf> = get_str(&material, "topic_path").map(PathBuf::from);
    match &topic {
        None => fail(&mut failed, "R2", "topic.md 不存在"),
        Some(t) if !t.is_file() => fail(&mut failed, "R2", "topic.md 不存在"),
        Some(t) => {
            let want = material.get("topic_sha256").and_then(|v| v.as_str()).unwrap_or("");
            if want != sha256_file(t)? {
                fail(&mut failed, "R2", "topic_sha256 与 topic.md 复算不符");
            } else {
                passed.push(json!("R2: topic_sha256 复算一致"));
            }
        }
    }
    let trail: Option<PathBuf> = get_str(&material, "trail_path").map(PathBuf::from);
    match &trail {
        None => fail(&mut failed, "R2", "trail 文件不存在"),
        Some(t) if !t.is_file() => fail(&mut failed, "R2", "trail 文件不存在"),
        Some(_) => passed.push(json!("R2: trail 文件在场")),
    }
    for (key, path_key) in [("contract_sha256", "contract_path"), ("responses_sha256", "responses_path")] {
        if let Some(pf_str) = get_str(&material, path_key) {
            let pf = PathBuf::from(pf_str);
            if !pf.is_file() {
                fail(&mut failed, "R2", format!("{} 不存在", path_key));
            } else {
                let want = material.get(key).and_then(|v| v.as_str()).unwrap_or("");
                if want != sha256_file(&pf)? {
                    fail(&mut failed, "R2", format!("{} 复算不符", key));
                } else {
                    passed.push(json!(format!("R2: {} 复算一致", key)));
                }
            }
        }
    }

    // R3 结果可机械校验：三值、版本、指纹重算、per_actor 七字段与方向枚举
    if !["stable_clear", "near_threshold", "boundary"].contains(&verdict_str.as_str()) {
        fail(&mut failed, "R3", format!("闸裁决不在三值集合：{}", verdict_str));
    } else {
        passed.push(json!(format!("R3: 闸裁决 {} 在三值集合", verdict_str)));
    }
    let criteria = material
        .get("criteria_version")
        .cloned()
        .unwrap_or(Value::String(CRITERIA_VERSION.into()));
    if criteria != Value::String(CRITERIA_VERSION.into()) {
        fail(&mut failed, "R3", format!("criteria_version 非 {}", CRITERIA_VERSION));
    } else {
        passed.push(json!(format!("R3: criteria_version {}", CRITERIA_VERSION)));
    }
    let mut dc_list: Vec<Value> = Vec::new();
    let mut runs: Vec<Value> = Vec::new();
    if let Some(t) = &trail {
        if t.is_file() {
            let (d, r) = load_dc_list(t)?;
            dc_list = d;
            runs = r;
            if material.get("dc_fingerprint").and_then(|v| v.as_str()).unwrap_or("") != fingerprint(&dc_list) {
                fail(&mut failed, "R3", "dc_fingerprint 复算不符");
            } else {
                passed.push(json!("R3: dc_fingerprint 复算一致"));
            }
            let mut bad_actor: BTreeSet<usize> = BTreeSet::new();
            for (i, dc) in dc_list.iter().enumerate() {
                if let Some(pas) = dc.get("per_actor").and_then(|v| v.as_array()) {
                    for pa in pas {
                        let fields_ok = pa
                            .as_object()
                            .map(|o| o.keys().cloned().collect::<BTreeSet<_>>()
                                == PER_ACTOR_FIELDS.iter().map(|s| s.to_string()).collect())
                            .unwrap_or(false);
                        let dec_ok = pa.get("decision").and_then(|v| v.as_str())
                            .map(|d| d == "comply" || d == "violate")
                            .unwrap_or(false);
                        if !fields_ok || !dec_ok {
                            bad_actor.insert(i);
                        }
                    }
                }
            }
            if !bad_actor.is_empty() {
                let v: Vec<usize> = bad_actor.iter().copied().collect();
                let shown: Vec<usize> = v.iter().take(3).copied().collect();
                fail(&mut failed, "R3", format!("per_actor 字段或 decision 枚举不符：行 {}", py_int_list_repr(&shown)));
            } else {
                passed.push(json!("R3: per_actor 七字段与方向枚举全符"));
            }
        }
    }

    // R5 席位有效性：当日基线三态联动
    let mut r5_state = "missing";
    let bp = material.get("seat_baseline_path").filter(|v| !v.is_null());
    if bp.map(|v| v.as_str().unwrap_or("").is_empty()).unwrap_or(true) {
        fail(&mut failed, "R5", "席位当日基线缺席");
    } else {
        let bf = PathBuf::from(bp.unwrap().as_str().unwrap_or(""));
        if !bf.is_file() {
            fail(&mut failed, "R5", "席位当日基线文件不存在");
        } else {
            let base_text = std::fs::read_to_string(&bf).map_err(|e| io_to_py(&e))?;
            let base = parse_json(&base_text)?;
            let raw_val = base
                .get("verdict")
                .filter(|v| !v.is_null() && v.as_str().map(|s| !s.is_empty()).unwrap_or(true))
                .cloned()
                .or_else(|| base.get("判定").cloned().filter(|v| !v.is_null()));
            let raw = match &raw_val {
                Some(Value::String(s)) => s.clone(),
                Some(Value::Null) | None => String::new(),
                Some(other) => other.to_string(),
            };
            let m_hash = material.get("identity_hash").and_then(|v| v.as_str());
            let b_hash = base.get("identity_hash").and_then(|v| v.as_str());
            match (m_hash.filter(|s| !s.is_empty()), b_hash.filter(|s| !s.is_empty())) {
                (Some(mh), Some(bh)) => {
                    if mh == bh {
                        // 哈希优先配对承 pk-035：双带即比对，异即不同环境按漂移挂起
                        r5_state = r5_state_of(&raw);
                        match r5_state {
                            "ok" => passed.push(json!("R5: 席位当日基线判定可用且身份哈希一致")),
                            "suspend" | "abnormal" => {
                                passed.push(json!(format!("R5: 身份哈希一致、基线判定 {}（处置走优先级映射）", raw)))
                            }
                            _ => fail(&mut failed, "R5", format!("基线判定值不可读：{}", raw)),
                        }
                    } else {
                        r5_state = "suspend";
                        passed.push(json!("R5: 席位身份哈希与基线不一致（处置走优先级映射）"));
                    }
                }
                _ => {
                    // 任一缺席回退现行为：旧材料旧基线重放逐字节同判
                    r5_state = r5_state_of(&raw);
                    match r5_state {
                        "ok" => passed.push(json!("R5: 席位当日基线判定可用")),
                        "suspend" | "abnormal" => {
                            passed.push(json!(format!("R5: 席位当日基线判定 {}（处置走优先级映射）", raw)))
                        }
                        _ => fail(&mut failed, "R5", format!("基线判定值不可读：{}", raw)),
                    }
                }
            }
        }
    }

    // R6 预算上限：同 gid 累计判定流不超过九
    let gid_str = gid.as_str().unwrap_or_default();
    let gid_runs: Vec<&Value> = runs
        .iter()
        .filter(|r| r.get("guidance_id").and_then(|v| v.as_str()) == Some(gid_str))
        .collect();
    if gid_runs.len() as i64 > BUDGET_PER_GID {
        fail(&mut failed, "R6", format!("同 gid 累计 {} 发超预算 {}", gid_runs.len(), BUDGET_PER_GID));
    } else {
        passed.push(json!(format!("R6: 同 gid 累计 {} 发未超预算", gid_runs.len())));
    }

    // R7 谱系完整：同 gid 一致、改写披露、超限告警
    if runs.iter().any(|r| r.get("guidance_id").and_then(|v| v.as_str()) != Some(gid_str)) {
        fail(&mut failed, "R7", "trail 内存在异 gid 判定行");
    } else {
        passed.push(json!("R7: trail 判定流同 gid 一致"));
    }
    let rewrite_of = material
        .get("rewrite_of")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty());
    if let Some(ro) = rewrite_of {
        let mut authored = String::new();
        if let Some(t) = &topic {
            if t.is_file() {
                if let Ok(text) = std::fs::read_to_string(t) {
                    let parts: Vec<&str> = text.splitn(3, "---").collect();
                    if parts.len() >= 2 {
                        authored = parts[1].to_string();
                    }
                }
            }
        }
        if authored.contains(ro) {
            passed.push(json!("R7: 改写谱系披露在场"));
        } else {
            fail(&mut failed, "R7", "改写重测缺前置 gid 谱系披露");
        }
        let chain_len = material
            .get("rewrite_chain")
            .and_then(|v| v.as_array())
            .map(|a| a.len())
            .unwrap_or(1);
        if chain_len > 3 {
            alarms.push(json!("R7: 同命题改写超三次，钓邻域风险告警"));
        }
    }

    // 三态映射落处置（DES-011 优先级）
    let direction = modal_direction(&dc_list);
    let disposition = if r5_state == "suspend" || r5_state == "abnormal" {
        "挂起"
    } else if !failed.is_empty() {
        "材料退回"
    } else if verdict_str == "boundary" {
        "打回重作"
    } else if verdict_str == "near_threshold" {
        "挂起"
    } else {
        "裁决通过"
    };

    Ok(json!({
        "tool": "tally",
        "version": VERSION,
        "material": material_path.to_string_lossy(),
        "gid": gid,
        "gate_verdict": verdict,
        "direction": direction,
        "disposition": disposition,
        "rules_version": material.get("rules_version").cloned().unwrap_or(Value::String(RULES_VERSION.into())),
        "passed": passed,
        "failed": failed,
        "alarms": alarms,
        "verdict": if failed.is_empty() { "pass" } else { "fail" },
    }))
}

/// 确定性装配：从命题区发现材料件，拼装 tally-check-input。零判断。
///
/// 择件规则：计分材料取 responses_sha256 与当前响应文件哈希一致者取 n_shots
/// 最大，无一致者报错不猜；材料内相对路径按仓库根（des_root 上两级）解；
/// topic 按合同内 topic_sha256 哈希匹配发现，无匹配即报错。
pub fn assemble_material(
    gid: &str,
    des_root: &Path,
    topics_dirs: &[PathBuf],
    baseline_path: Option<&Path>,
    date: Option<&str>,
    out_path: &Path,
) -> Result<Value, PyError> {
    let repo_root = des_root
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let resolve = |v: Option<&str>| -> Option<String> {
        v.filter(|s| !s.is_empty()).map(|s| {
            let pf = Path::new(s);
            if pf.is_absolute() {
                s.to_string()
            } else {
                repo_root.join(s).to_string_lossy().into_owned()
            }
        })
    };

    let cell = des_root.join(gid);
    if !cell.is_dir() {
        return Err(verr(format!("命题区目录不存在：{}", cell.display())));
    }
    let mut scores: Vec<PathBuf> = std::fs::read_dir(&cell)
        .map_err(|e| io_to_py(&e))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().ends_with("-score-material.json"))
                .unwrap_or(false)
        })
        .collect();
    scores.sort();
    if scores.is_empty() {
        return Err(verr(format!("无计分材料：{}/*-score-material.json", cell.display())));
    }
    let mut matched: Vec<Value> = Vec::new();
    for pf in &scores {
        let text = std::fs::read_to_string(pf).map_err(|e| io_to_py(&e))?;
        let mut sc = parse_json(&text)?;
        let rp = resolve(sc.get("responses_path").and_then(|v| v.as_str()));
        let want = sc.get("responses_sha256").and_then(|v| v.as_str()).unwrap_or("");
        let m = match &rp {
            Some(r) => {
                let p = PathBuf::from(r);
                p.is_file() && want == sha256_file(&p)?
            }
            None => false,
        };
        if let Some(o) = sc.as_object_mut() {
            o.insert("_m".into(), Value::Bool(m));
        }
        matched.push(sc);
    }
    let matched: Vec<Value> = matched
        .into_iter()
        .filter(|sc| sc.get("_m").and_then(|v| v.as_bool()).unwrap_or(false))
        .collect();
    if matched.is_empty() {
        return Err(verr("无计分材料与当前响应文件哈希一致（响应已变而未重计分？）"));
    }
    // max by (n_shots, responses_sha256)，取首个最大（Python max 稳定语义）
    let mut score = matched[0].clone();
    for sc in matched.iter().skip(1) {
        let a = (score.get("n_shots").and_then(|v| v.as_i64()).unwrap_or(0),
                 score.get("responses_sha256").and_then(|v| v.as_str()).unwrap_or(""));
        let b = (sc.get("n_shots").and_then(|v| v.as_i64()).unwrap_or(0),
                 sc.get("responses_sha256").and_then(|v| v.as_str()).unwrap_or(""));
        if b > a {
            score = sc.clone();
        }
    }
    if let Some(o) = score.as_object_mut() {
        o.remove("_m");
        for key in ["responses_path", "trail_path", "contract_path"] {
            if let Some(v) = o.get(key).and_then(|x| x.as_str()).map(|s| s.to_string()) {
                o.insert(key.into(), json!(resolve(Some(&v)).unwrap_or(v)));
            }
        }
    }
    let contract_path = PathBuf::from(score["contract_path"].as_str().unwrap_or_default());
    let contract_text = std::fs::read_to_string(&contract_path).map_err(|e| io_to_py(&e))?;
    let contract = parse_json(&contract_text)?;
    let topic_sha = contract
        .get("proposition")
        .and_then(|p| p.get("topic_sha256"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let trail_path_str = match score.get("trail_path").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        Some(t) => t.to_string(),
        None => cell.join("flywheel-trail.jsonl").to_string_lossy().into_owned(),
    };
    let trail_path = PathBuf::from(&trail_path_str);
    let mut cands: Vec<PathBuf> = vec![cell.join("topic.md")];
    for td in topics_dirs {
        if let Ok(rd) = std::fs::read_dir(td) {
            let mut mds: Vec<PathBuf> = rd
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().map(|x| x == "md").unwrap_or(false))
                .collect();
            mds.sort();
            cands.extend(mds);
        }
    }
    let mut topic_path: Option<PathBuf> = None;
    for cand in &cands {
        if cand.is_file() && sha256_file(cand)? == topic_sha {
            topic_path = Some(cand.clone());
            break;
        }
    }
    let topic_path = match topic_path {
        Some(t) => t,
        None => {
            return Err(verr(format!(
                "topic 按哈希无匹配：{}（topics_dirs 覆盖不足）",
                &topic_sha[..topic_sha.len().min(12)]
            )))
        }
    };
    let (dc_list, _runs) = load_dc_list(&trail_path)?;
    let mut material = json!({
        "kind": "tally-check-input",
        "gid": gid,
        "topic_path": topic_path.to_string_lossy(),
        "topic_sha256": topic_sha,
        "trail_path": trail_path_str,
        "dc_fingerprint": fingerprint(&dc_list),
        "gate_verdict": score.get("gate_verdict").cloned().unwrap_or(Value::Null),
        "criteria_version": CRITERIA_VERSION,
        "contract_path": score.get("contract_path").cloned().unwrap_or(Value::Null),
        "contract_sha256": score.get("contract_sha256").cloned().unwrap_or(Value::Null),
        "responses_path": score.get("responses_path").cloned().unwrap_or(Value::Null),
        "responses_sha256": score.get("responses_sha256").cloned().unwrap_or(Value::Null),
        "n_shots": score.get("n_shots").cloned().unwrap_or(Value::Null),
        "voids": score.get("voids").cloned().unwrap_or(Value::Array(vec![])),
        "rules_version": RULES_VERSION,
    });
    if let (Some(o), Some(ih)) = (material.as_object_mut(), score.get("identity_hash").filter(|v| !v.is_null())) {
        o.insert("identity_hash".into(), ih.clone());
    }
    if let Some(b) = baseline_path {
        if let Some(o) = material.as_object_mut() {
            o.insert("seat_baseline_path".into(), json!(b.to_string_lossy()));
        }
    }
    if let Some(d) = date {
        if let Some(o) = material.as_object_mut() {
            o.insert("date".into(), json!(d));
        }
    }
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| io_to_py(&e))?;
    }
    std::fs::write(out_path, format!("{}\n", canonical_json(&material))).map_err(|e| io_to_py(&e))?;
    Ok(material)
}

/// 重放：同输入重跑核对，与既报报告逐字节比对。
pub fn verify_material(material_path: &Path, report_path: &Path) -> Result<i32, PyError> {
    let fresh = check_material(material_path)?;
    let text = std::fs::read_to_string(report_path).map_err(|e| io_to_py(&e))?;
    let recorded = parse_json(&text)?;
    if canonical_json(&fresh) == canonical_json(&recorded) {
        println!(
            "{}",
            py_compact_sorted(&json!({
                "verify": "identical", "gid": fresh["gid"], "disposition": fresh["disposition"],
            }))
        );
        Ok(0)
    } else {
        eprintln!(
            "{}",
            serde_json::to_string(&json!({
                "verify": "divergent", "gid": recorded.get("gid").cloned().unwrap_or(Value::Null),
                "note": "复算与既报不符，整包作废",
            }))
            .unwrap_or_default()
        );
        Ok(1)
    }
}

/// 机器终签：仅裁决通过时把核对报告经专属 crosscheck 子命令落链。
pub fn sign_material(
    material_path: &Path,
    out_dir: &Path,
    trail: &str,
    scribe_binary: &str,
    session: &str,
    locks: &str,
) -> Result<i32, PyError> {
    let report = check_material(material_path)?;
    if report["disposition"].as_str() != Some("裁决通过") {
        println!(
            "{}",
            py_compact_sorted(&json!({
                "sign": "refused",
                "reason": format!("处置为 {}，仅裁决通过可落据", report["disposition"].as_str().unwrap_or_default()),
                "gid": report["gid"],
            }))
        );
        return Ok(1);
    }
    std::fs::create_dir_all(out_dir).map_err(|e| io_to_py(&e))?;
    let report_path = out_dir.join(format!("{}-signcheck.json", report["gid"].as_str().unwrap_or_default()));
    std::fs::write(&report_path, format!("{}\n", canonical_json(&report))).map_err(|e| io_to_py(&e))?;
    let done = Command::new(scribe_binary)
        .args([
            "crosscheck",
            "--report",
            &report_path.to_string_lossy(),
            "--material",
            &material_path.to_string_lossy(),
            "--trail",
            trail,
            "--locks",
            locks,
            "--session",
            session,
        ])
        .status()
        .map_err(|e| io_to_py(&e))?;
    let rc = done.code().unwrap_or(-1);
    if rc != 0 {
        println!(
            "{}",
            py_compact_sorted(&json!({
                "sign": "failed",
                "reason": "scribe crosscheck 非零退出，落据未发生链上无事件，唯退出码与此输出为准",
                "gid": report["gid"],
                "scribe_exit": rc,
            }))
        );
        return Ok(rc);
    }
    // 签署后明确输出（人类知晓视图，零动作要求；反对权与异常视图见 CONTRACT）
    println!(
        "{}",
        py_compact_sorted(&json!({
            "signed": report["gid"],
            "disposition": report["disposition"],
            "direction": report["direction"],
            "replay_anchor": report_path.to_string_lossy(),
            "note": "人类知晓无需复签；一票绝对反对经 append inconsistency 翻案；边界变更经 watch 浮出",
        }))
    );
    Ok(0)
}

/// 边界变更监视：对目录内全部签署核对报告重放复算，浮出异常视图。
pub fn watch_reports(reports_dir: &Path) -> Result<i32, PyError> {
    let mut reports: Vec<PathBuf> = std::fs::read_dir(reports_dir)
        .map_err(|e| io_to_py(&e))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().ends_with("-signcheck.json"))
                .unwrap_or(false)
        })
        .collect();
    reports.sort();
    if reports.is_empty() {
        println!(
            "{}",
            py_compact_sorted(&json!({"watch": "empty", "dir": reports_dir.to_string_lossy()}))
        );
        return Ok(0);
    }
    let mut anomalies: Vec<Value> = Vec::new();
    let mut ok = 0usize;
    for rp in &reports {
        // 由报告反查材料路径重算
        let outcome = (|| -> Result<bool, PyError> {
            let text = std::fs::read_to_string(rp).map_err(|e| io_to_py(&e))?;
            let recorded = parse_json(&text)?;
            let material_path = recorded
                .get("material")
                .and_then(|v| v.as_str())
                .ok_or_else(|| keyerr("'material'"))?;
            let fresh = check_material(Path::new(material_path))?;
            Ok(canonical_json(&fresh) == canonical_json(&recorded))
        })();
        match outcome {
            Ok(true) => ok += 1,
            Ok(false) => anomalies.push(json!({
                "report": rp.to_string_lossy(),
                "gid": parse_json(&std::fs::read_to_string(rp).unwrap_or_default())
                    .ok()
                    .and_then(|v| v.get("gid").cloned())
                    .unwrap_or(Value::Null),
                "note": "边界条件变更：复算与既报不符（异常视图，人重命题或令 agent 重跑）",
            })),
            Err(e) => anomalies.push(json!({
                "report": rp.to_string_lossy(),
                "error": format!("{}", e),
            })),
        }
    }
    println!(
        "{}",
        canonical_json(&json!({"watch": "done", "ok": ok, "anomalies": anomalies}))
    );
    Ok(if anomalies.is_empty() { 0 } else { 1 })
}

/// 围堰 main 的错误信封形（域错误统一此形出 stderr）。
pub fn error_envelope(e: &PyError) -> String {
    json!({"error": format!("{}", e)}).to_string()
}

/// JSON 解析坏行的信封构造（供 CLI 层归类环境错误）。
pub fn json_error_envelope(msg: &str) -> String {
    jsonerr(msg.to_string()).to_string()
}

#[allow(dead_code)]
fn map_ref(_m: &Map<String, Value>) {}
