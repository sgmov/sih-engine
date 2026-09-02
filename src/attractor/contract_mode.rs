//! contract_mode——采样腿契约的机械内核（融回自围堰 facet/contract_mode.py，
//! 调用面两半切分承 CONTRACT-MODE-SPEC v1）。
//!
//! facet 侧确定性内核四件即出题、计分、闸门、验证，零 LLM 零网络零 key；
//! 框架侧一动作即执行采样合同（把每条提示词原样发给自己的模型，模型原文
//! 逐发写回，不改写不摘要不挑选）留围堰。提示词构造与围堰 runner parallel
//! 路径逐字节同源：同一 atom 模板加同一替换序列。
//!
//! 哈希绑定链（工程基线四）：合同哈希绑定响应原文哈希绑定裁决材料，verify
//! 全量重算。解析失败的响应不静默丢：落 no_answer + boundary_flag=true 空转
//! 标记（复现即作废语义），逐发计入裁决材料。

use std::path::Path;

use serde_json::{json, Value};

use super::jsonc::{canonical_json, io_to_py, parse_json, py_list_repr, verr, PyError};

pub const CONTRACT_KIND: &str = "facet-sampling-contract";
pub const CONTRACT_VERSION: i64 = 1;
pub const SCORE_KIND: &str = "facet-contract-score";
pub const SCORE_VERSION: i64 = 2;
pub const RESPONSE_DISCIPLINE: &str = "逐发模型完整原文回填，不改写、不摘要、不挑选";
pub const CONTRACT_SOURCE_TAG: &str = "contract-mode";

/// 围堰 runner parallel 路径的空发散发谱标记（compiler.format_divergence_shape
/// 空形快照），合同提示词防漂移锚。
pub const EMPTY_SHAPE_MARKER: &str = "（无前置微分数据，发散发谱为空）";

/// 围堰 time.strftime("%Y-%m-%dT%H:%M:%SZ", gmtime()) 对齐形。
fn now_stamp() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn sha256_bytes(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

pub fn sha256_file(path: &Path) -> Result<String, PyError> {
    let bytes = std::fs::read(path).map_err(|e| io_to_py(&e))?;
    Ok(sha256_bytes(&bytes))
}

/// write_scheme 块标量经 yaml 解析后的 ng 文本（clip 语义）：非空文本恰多
/// 一个尾换行。
pub fn scheme_clipped_ng(ng_text: &str) -> String {
    if ng_text.trim().is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for (i, line) in ng_text.trim().split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(line);
    }
    out.push('\n');
    out
}

/// normative_convergence 单段 integrator 提示词构造，与围堰 runner parallel
/// 路径同源：atom.yaml integrator 第一方向，user 先 {topic_content} 后
/// {proposition}，system 按围堰序替换七标记。atoms 由 paradigm_loader 装载。
pub fn build_integrator_prompts(atoms: &Value, topic_md_text: &str, ng_text: &str) -> (String, String) {
    let integrator = &atoms["integrator"];
    let direction = &integrator["directions"][0];
    let user_tmpl = direction
        .get("user_prompt_template")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let user = user_tmpl
        .replace("{topic_content}", topic_md_text)
        .replace("{proposition}", topic_md_text);
    let mut system = direction
        .get("system_prompt_template")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let subs: [(&str, &str); 7] = [
        ("{round}", "1"),
        ("{prev_round}", "0"),
        ("{prior_facets}", ""),
        ("{normative_guidance}", ng_text),
        ("{divergence_shape}", EMPTY_SHAPE_MARKER),
        ("{role}", ""),
        ("{role_prompt}", ""),
    ];
    for (old, new) in subs {
        system = system.replace(old, new);
    }
    (system, user)
}

/// 同一提示词 n 发的 shots 列表，key 形如 `<gid>#r<k>`。
pub fn make_shots(system_prompt: &str, user_prompt: &str, gid: &str, n: i64, start: i64) -> Value {
    let shots: Vec<Value> = (start..start + n)
        .map(|k| {
            json!({
                "key": format!("{}#r{}", gid, k),
                "shot": k,
                "system_prompt": system_prompt,
                "user_prompt": user_prompt,
            })
        })
        .collect();
    Value::Array(shots)
}

/// 合同落盘（确定性：sort_keys 无时间戳，同输入逐字节一致）。
pub fn emit_contract(
    path: &Path,
    seat: &Value,
    pack: &Value,
    proposition: &Value,
    shots: &Value,
    meta: &Value,
) -> Result<(), PyError> {
    let contract = json!({
        "kind": CONTRACT_KIND,
        "contract_version": CONTRACT_VERSION,
        "seat": seat,
        "pack": pack,
        "proposition": proposition,
        "responses_format": {
            "file": "jsonl",
            "fields": ["key", "shot", "raw"],
            "discipline": RESPONSE_DISCIPLINE,
        },
        "shots": shots,
        "meta": meta,
    });
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| io_to_py(&e))?;
    }
    std::fs::write(path, format!("{}\n", canonical_json(&contract))).map_err(|e| io_to_py(&e))?;
    Ok(())
}

/// 合同装载：kind 与版本严格校验。
pub fn load_contract(path: &Path) -> Result<Value, PyError> {
    let text = std::fs::read_to_string(path).map_err(|e| io_to_py(&e))?;
    let contract = parse_json(&text)?;
    let kind = contract.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    if kind != CONTRACT_KIND {
        let shown = contract.get("kind").map(value_display).unwrap_or_else(|| "None".into());
        return Err(verr(format!("非 facet 采样合同：kind={}", shown)));
    }
    let ver = contract.get("contract_version").and_then(|v| v.as_i64());
    if ver != Some(CONTRACT_VERSION) {
        let shown = contract.get("contract_version").map(value_display).unwrap_or_else(|| "None".into());
        return Err(verr(format!("合同版本不符：{}", shown)));
    }
    Ok(contract)
}

/// Value 的人读形（围堰 f-string 对齐）：字符串原样、null 即 None。
pub fn value_display(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "None".into(),
        other => other.to_string(),
    }
}

/// 响应严格对表：每行恰三键、key 全在合同内、无重复、shot 对齐、零缺失。
/// 任何违例拒收整批不静默（围堰消息原文逐字对齐）。返回按合同 shots 顺序
/// 排列的响应列表。
pub fn load_responses(path: &Path, contract: &Value) -> Result<Vec<Value>, PyError> {
    let text = std::fs::read_to_string(path).map_err(|e| io_to_py(&e))?;
    let expected_src = contract["shots"]
        .as_array()
        .ok_or_else(|| verr("合同 shots 缺席".to_string()))?;
    // key -> shot 与 shots 顺序
    let mut expected: Vec<(String, Value)> = Vec::new();
    for s in expected_src {
        expected.push((
            s["key"].as_str().unwrap_or_default().to_string(),
            s["shot"].clone(),
        ));
    }
    let mut seen: Vec<(String, Value)> = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let lineno = idx + 1;
        if line.trim().is_empty() {
            continue;
        }
        let r = parse_json(line).map_err(|e| {
            verr(format!("响应第 {} 行非 json：{}", lineno, e.msg))
        })?;
        let obj = match r.as_object() {
            Some(o) => o,
            None => return Err(verr(format!("响应第 {} 行键不符：{}", lineno, py_list_repr(&[])))),
        };
        let keys: Vec<String> = obj.keys().cloned().collect();
        let mut sorted_keys = keys.clone();
        sorted_keys.sort();
        if sorted_keys != vec!["key".to_string(), "raw".to_string(), "shot".to_string()] {
            return Err(verr(format!(
                "响应第 {} 行键不符：{}",
                lineno,
                py_list_repr(&sorted_keys)
            )));
        }
        let key = obj["key"].as_str().unwrap_or_default().to_string();
        if !expected.iter().any(|(k, _)| *k == key) {
            return Err(verr(format!("响应第 {} 行 key 不在合同内：{}", lineno, key)));
        }
        if seen.iter().any(|(k, _)| *k == key) {
            return Err(verr(format!("响应第 {} 行 key 重复：{}", lineno, key)));
        }
        let exp_shot = &expected.iter().find(|(k, _)| *k == key).unwrap().1;
        if &obj["shot"] != exp_shot {
            return Err(verr(format!("响应第 {} 行 shot 与合同不符：{}", lineno, key)));
        }
        let raw_ok = obj["raw"].is_string() && !obj["raw"].as_str().unwrap_or("").trim().is_empty();
        if !raw_ok {
            return Err(verr(format!("响应第 {} 行 raw 为空或非字符串：{}", lineno, key)));
        }
        seen.push((key, r.clone()));
    }
    let missing: Vec<String> = expected
        .iter()
        .filter(|(k, _)| !seen.iter().any(|(sk, _)| sk == k))
        .map(|(k, _)| k.clone())
        .collect();
    if !missing.is_empty() {
        let mut sorted_missing = missing.clone();
        sorted_missing.sort();
        let shown: Vec<String> = sorted_missing.iter().take(5).cloned().collect();
        let ellipsis = if sorted_missing.len() > 5 { "..." } else { "" };
        return Err(verr(format!(
            "缺 {} 发响应：{}{}",
            sorted_missing.len(),
            py_list_repr(&shown),
            ellipsis
        )));
    }
    // 按合同 shots 顺序重排
    let mut out = Vec::new();
    for (k, _) in &expected {
        if let Some((_, r)) = seen.iter().find(|(sk, _)| sk == k) {
            out.push(r.clone());
        }
    }
    Ok(out)
}

/// 模型原文 → 判定四键（围堰双解析同语义合成：先围栏剥离再花括号切片）。
/// 解析失败不静默：no_answer + boundary_flag=true + void 标记。
pub fn parse_raw_answer(raw: &str) -> Value {
    let mut text = raw.trim().to_string();
    if text.starts_with("```") {
        let body = match text.find('\n') {
            Some(pos) => text[pos + 1..].to_string(),
            None => String::new(),
        };
        text = if body.ends_with("```") { body[..body.len() - 3].to_string() } else { body };
        text = text.trim().to_string();
    }
    let start = text.find('{');
    let end = text.rfind('}');
    if let (Some(s), Some(e)) = (start, end) {
        if e >= s {
            if let Ok(o) = serde_json::from_str::<Value>(&text[s..=e]) {
                let decision = o.get("decision").cloned().unwrap_or(Value::String("no_answer".into()));
                let basis = o.get("basis_regulation").cloned().unwrap_or(Value::String(String::new()));
                let boundary = o.get("boundary_flag").cloned().unwrap_or(Value::Bool(false));
                let reason = o.get("reason").cloned().unwrap_or(Value::String(String::new()));
                return json!({
                    "decision": decision,
                    "basis_regulation": basis,
                    "boundary_flag": boundary.as_bool().unwrap_or(false),
                    "reason": reason,
                    "void": false,
                });
            }
        }
    }
    let head: String = text.chars().take(80).collect();
    json!({
        "decision": "no_answer",
        "basis_regulation": "",
        "boundary_flag": true,
        "reason": format!("non-JSON output: {}", head),
        "void": true,
    })
}

/// 响应列表 → per_actor 列表与空转发明细。actor_id 形如 `<framework>+<model>`。
pub fn dc_from_responses(responses: &[Value], seat: &Value) -> Result<(Vec<Value>, Vec<Value>), PyError> {
    let framework = seat["framework"].as_str().unwrap_or_default();
    let model_id = seat["model_id"].as_str().unwrap_or_default();
    let actor_id = format!("{}+{}", framework, model_id);
    let mut per_actor = Vec::new();
    let mut voids = Vec::new();
    for r in responses {
        let d = parse_raw_answer(r["raw"].as_str().unwrap_or_default());
        per_actor.push(json!({
            "actor_id": actor_id,
            "model_id": model_id,
            "family": "Seat",
            "decision": d["decision"],
            "basis_regulation": d["basis_regulation"],
            "boundary_flag": d["boundary_flag"],
            "reason": d["reason"],
        }));
        if d["void"].as_bool().unwrap_or(false) {
            voids.push(json!({
                "key": r["key"],
                "reason": d["reason"],
            }));
        }
    }
    Ok((per_actor, voids))
}

/// 逐发 flywheel_run 记录追加（与围堰判定流同构），存量感知：run_id 已在
/// trail 即跳过该发（幂等续跑，R4 历史不可篡改）。返回实写的 run_id 列表。
pub fn append_contract_runs(
    trail_path: &Path,
    gid: &str,
    responses: &[Value],
    per_actor: &[Value],
    seat: &Value,
    ng_label: &str,
) -> Result<Vec<String>, PyError> {
    let mut existing: Vec<String> = Vec::new();
    if trail_path.exists() {
        let text = std::fs::read_to_string(trail_path).map_err(|e| io_to_py(&e))?;
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(o) = serde_json::from_str::<Value>(line) {
                existing.push(o.get("run_id").and_then(|v| v.as_str()).unwrap_or("").to_string());
            }
        }
    }
    let mut written = Vec::new();
    if let Some(parent) = trail_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| io_to_py(&e))?;
    }
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(trail_path)
        .map_err(|e| io_to_py(&e))?;
    let model_id = seat["model_id"].as_str().unwrap_or_default();
    for (r, pa) in responses.iter().zip(per_actor.iter()) {
        let key = r["key"].as_str().unwrap_or_default();
        let run_id = format!("contract-{}", key.replace('#', "-"));
        if existing.contains(&run_id) {
            continue;
        }
        let record = json!({
            "trail_type": "flywheel_run",
            "trail_version": 1,
            "guidance_id": gid,
            "timestamp": now_stamp(),
            "run_id": run_id,
            "source_run_dir": CONTRACT_SOURCE_TAG,
            "actors": [{"model_id": model_id, "family": "Seat"}],
            "normative_guidance": ng_label,
            "shape": Value::Null,
            "decision_convergence": {"per_actor": [pa]},
        });
        writeln!(f, "{}", serde_json::to_string(&record).unwrap_or_default()).map_err(|e| io_to_py(&e))?;
        written.push(run_id);
    }
    Ok(written)
}

/// 规范席位串：框架:模型:版本 三段式，大小写敏感禁自由变形，缺版本补
/// self-reported。
pub fn canonical_seat(seat: &Value) -> Result<String, PyError> {
    let fw = str_field(seat, "framework");
    let mid = str_field(seat, "model_id");
    let ver_raw = seat.get("version").filter(|v| !v.is_null());
    let ver = match ver_raw {
        Some(v) => value_display(v),
        None => String::new(),
    };
    let ver = if ver.trim().is_empty() { "self-reported".to_string() } else { ver.trim().to_string() };
    let fw = fw.trim().to_string();
    let mid = mid.trim().to_string();
    for seg in [&fw, &mid, &ver] {
        if seg.is_empty() || seg.contains(':') || seg.chars().any(|c| c.is_whitespace()) {
            return Err(verr(format!("席位串段违例：'{}'", seg)));
        }
    }
    Ok(format!("{}:{}:{}", fw, mid, ver))
}

fn str_field(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

/// identity_hash 强制携带（pk-035）：六十四位十六进制。
fn require_identity_hash(identity_hash: &str) -> Result<String, PyError> {
    let h = identity_hash.trim();
    if h.len() != 64 || !h.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()) {
        return Err(verr("identity_hash 缺席或非六十四位十六进制，强制携带承 pk-035"));
    }
    Ok(h.to_string())
}

/// 计分材料落盘：合同哈希绑定响应哈希（工程基线四，verify 据此重算）。
pub fn write_score_material(
    path: &Path,
    contract_path: &Path,
    responses_path: &Path,
    gid: &str,
    n_shots: i64,
    voids: &[Value],
    identity_hash: &str,
    extra: &Value,
) -> Result<Value, PyError> {
    let ih = require_identity_hash(identity_hash)?;
    let contract = load_contract(contract_path)?;
    let seat_string = canonical_seat(&contract["seat"])?;
    let mut material = json!({
        "kind": SCORE_KIND,
        "score_version": SCORE_VERSION,
        "gid": gid,
        "identity_hash": ih,
        "seat_string": seat_string,
        "contract_sha256": sha256_file(contract_path)?,
        "responses_sha256": sha256_file(responses_path)?,
        "n_shots": n_shots,
        "voids": voids,
        "contract_path": contract_path.to_string_lossy(),
        "responses_path": responses_path.to_string_lossy(),
    });
    if let (Some(m), Some(e)) = (material.as_object_mut(), extra.as_object()) {
        for (k, v) in e {
            m.insert(k.clone(), v.clone());
        }
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| io_to_py(&e))?;
    }
    std::fs::write(path, format!("{}\n", canonical_json(&material))).map_err(|e| io_to_py(&e))?;
    Ok(material)
}

/// 计分管线：合同装载 → 响应严格对表 → dc 装配 → 飞轮追加（幂等）→
/// 计分材料落盘。gate_verdict 为闸上游产出（判据 v3 闸留围堰，机械腿只装配
/// 不判闸），identity_hash 强制携带承 pk-035。
pub fn score_pipeline(
    contract_path: &Path,
    responses_path: &Path,
    trail_path: &Path,
    out_path: &Path,
    identity_hash: &str,
    gate_verdict: &str,
) -> Result<Value, PyError> {
    let contract = load_contract(contract_path)?;
    let gid = contract["proposition"]["gid"].as_str().unwrap_or_default().to_string();
    let responses = load_responses(responses_path, &contract)?;
    let (per_actor, voids) = dc_from_responses(&responses, &contract["seat"])?;
    let ng_label = contract["pack"]["ng_label"].as_str().unwrap_or_default();
    let written = append_contract_runs(trail_path, &gid, &responses, &per_actor, &contract["seat"], ng_label)?;
    write_score_material(
        out_path,
        contract_path,
        responses_path,
        &gid,
        responses.len() as i64,
        &voids,
        identity_hash,
        &json!({
            "trail_path": trail_path.to_string_lossy(),
            "runs_written": written.len() as i64,
            "gate_verdict": gate_verdict,
        }),
    )
}
