//! compiler——facet 确定性聚合器（融回自围堰 facet/src/compiler.py 能力层）。
//!
//! 不调 LLM：算异质性指标 + 锚点级分类 + 决策收敛装配 + 发散发谱投影。
//! severity 三档 critical / major / minor（info 经 OQ-17 定档砍除）；run
//! jsonl 输出 facet 用 `facets` key。import 面仅 anchors 与 model_utils
//! （融回闭包内互依），零 LLM 零网络。
//!
//! 注：围堰 cross_round 分类漂移的锚点列表遍历集合（字符串哈希序，跨进程
//! 不定）；本移植以字典序遍历承载确定性，属确定性加固非语义漂移。

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;

use serde_json::{json, Map, Value};

use super::anchors::{load_whitelist, parse_range, validate};
use super::jsonc::{io_to_py, py_round};
use super::model_utils::{extract_family, extract_framework, is_non_default_framework};

/// severity 合法字面（三档）。
pub const KNOWN_SEVERITIES: [&str; 3] = ["critical", "major", "minor"];

/// severity 权重（severity 加权 Jaccard 用）。
pub fn severity_weight(sev: &str) -> f64 {
    match sev {
        "critical" => 3.0,
        "major" => 2.0,
        "minor" => 1.0,
        _ => 1.0,
    }
}

fn fence_re() -> &'static regex::Regex {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"(?s)```(?:[a-zA-Z]*)\s*(.*?)\s*```").unwrap())
}

/// 剥掉 markdown 围栏，保留围栏内文本。
pub fn strip_code_fence(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    match fence_re().captures(text) {
        Some(c) => c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
        None => text.to_string(),
    }
}

/// 从 LLM output 解析 facet JSON：剥围栏后找首个花括号平衡区间 parse。
pub fn parse_payload(output: &str) -> Option<Value> {
    let stripped = strip_code_fence(output);
    if stripped.is_empty() {
        return None;
    }
    let start = stripped.find('{')?;
    let chars: Vec<char> = stripped.chars().collect();
    let mut depth = 0i32;
    let mut byte_pos = 0usize;
    let mut start_byte = 0usize;
    for (i, ch) in chars.iter().enumerate() {
        let len = ch.len_utf8();
        if i == start {
            start_byte = byte_pos;
        }
        byte_pos += len;
        if *ch == '{' {
            depth += 1;
        } else if *ch == '}' {
            depth -= 1;
            if depth == 0 {
                let end_byte = byte_pos;
                let slice = &stripped[start_byte..end_byte];
                let obj: Value = serde_json::from_str(slice).ok()?;
                return if obj.is_object() { Some(obj) } else { None };
            }
        }
    }
    None
}

fn facets_of(parsed: &Value) -> Option<&Vec<Value>> {
    let f = parsed.get("facets").or_else(|| parsed.get("attacks"));
    f.and_then(|v| v.as_array())
}

/// 从 facet output 解析 anchor_ref 字段列表（支持逗号分隔多锚点）。
pub fn extract_facet_refs(output: &str) -> Vec<String> {
    let parsed = match parse_payload(output) {
        Some(p) => p,
        None => return Vec::new(),
    };
    let facets = match facets_of(&parsed) {
        Some(f) => f,
        None => return Vec::new(),
    };
    let mut refs = Vec::new();
    for f in facets {
        if let Some(r) = f.get("anchor_ref").and_then(|v| v.as_str()) {
            if !r.is_empty() {
                for part in r.split(',') {
                    let p = part.trim();
                    if !p.is_empty() {
                        refs.push(p.to_string());
                    }
                }
            }
        }
    }
    refs
}

/// 从 facet output 解析 severity 字段列表。
pub fn extract_severities(output: &str) -> Vec<String> {
    let parsed = match parse_payload(output) {
        Some(p) => p,
        None => return Vec::new(),
    };
    let facets = match facets_of(&parsed) {
        Some(f) => f,
        None => return Vec::new(),
    };
    let mut sevs = Vec::new();
    for f in facets {
        if let Some(s) = f.get("severity").and_then(|v| v.as_str()) {
            if !s.is_empty() {
                sevs.push(s.to_string());
            }
        }
    }
    sevs
}

/// 从 facet output 解析 (anchor_ref, severity) 配对列表（severity 缺省 minor）。
pub fn extract_ref_severity_pairs(output: &str) -> Vec<(String, String)> {
    let parsed = match parse_payload(output) {
        Some(p) => p,
        None => return Vec::new(),
    };
    let facets = match facets_of(&parsed) {
        Some(f) => f,
        None => return Vec::new(),
    };
    let mut pairs = Vec::new();
    for f in facets {
        let Some(obj) = f.as_object() else { continue };
        let Some(r) = obj.get("anchor_ref").and_then(|v| v.as_str()) else { continue };
        if r.is_empty() {
            continue;
        }
        let sev = obj.get("severity").and_then(|v| v.as_str()).unwrap_or("minor");
        for part in r.split(',') {
            let p = part.trim();
            if !p.is_empty() {
                pairs.push((p.to_string(), sev.to_string()));
            }
        }
    }
    pairs
}

fn jaccard(a: &BTreeSet<(String, i64)>, b: &BTreeSet<(String, i64)>) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let inter = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 {
        0.0
    } else {
        inter as f64 / union as f64
    }
}

/// 读 jsonl 全文，跳过空行与坏行。
pub fn load_jsonl(jsonl_path: &Path) -> Vec<Value> {
    if !jsonl_path.exists() {
        return Vec::new();
    }
    let mut lines = Vec::new();
    let text = match std::fs::read_to_string(jsonl_path) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            lines.push(v);
        }
    }
    lines
}

/// 锚点覆盖率：锚点并集 / 白名单总数。
pub fn compute_anchor_coverage(
    facetor_anchor_sets: &HashMap<String, BTreeSet<(String, i64)>>,
    whitelist_size: usize,
) -> Value {
    let mut union_refs: BTreeSet<(String, i64)> = BTreeSet::new();
    for refs in facetor_anchor_sets.values() {
        union_refs.extend(refs.iter().cloned());
    }
    let mut per_actor = Map::new();
    let mut actors: Vec<&String> = facetor_anchor_sets.keys().collect();
    actors.sort();
    for actor in actors {
        per_actor.insert(
            actor.clone(),
            json!(facetor_anchor_sets[actor].len() as i64),
        );
    }
    let covered = union_refs.len();
    if whitelist_size == 0 {
        return json!({
            "covered_lines": 0,
            "total_lines": 0,
            "coverage_rate": 0.0,
            "per_actor": per_actor,
            "note": "whitelist_size 为 0,coverage_rate 不可计算",
        });
    }
    json!({
        "covered_lines": covered as i64,
        "total_lines": whitelist_size as i64,
        "coverage_rate": py_round(covered as f64 / whitelist_size as f64, 4),
        "per_actor": per_actor,
    })
}

/// facetor 两两 Jaccard 相似度取平均；facetor 数不足二时报不可算。
pub fn compute_jaccard_similarity(
    facetor_anchor_sets: &HashMap<String, BTreeSet<(String, i64)>>,
) -> Value {
    let mut actors: Vec<String> = facetor_anchor_sets.keys().cloned().collect();
    actors.sort();
    if actors.len() < 2 {
        return json!({
            "mean_jaccard": 0.0,
            "pairwise": [],
            "actors": actors,
            "note": "facetor 数 < 2,无法算两两 Jaccard",
        });
    }
    let mut pairwise = Vec::new();
    let mut total = 0.0f64;
    for i in 0..actors.len() {
        for b in actors.iter().skip(i + 1) {
            let score = jaccard(&facetor_anchor_sets[&actors[i]], &facetor_anchor_sets[b]);
            total += score;
            pairwise.push(json!({"a": actors[i], "b": b, "score": py_round(score, 4)}));
        }
    }
    let mean = if pairwise.is_empty() { 0.0 } else { total / pairwise.len() as f64 };
    json!({
        "mean_jaccard": py_round(mean, 4),
        "pairwise": pairwise,
        "actors": actors,
    })
}

/// severity 加权 Jaccard：权重 = 双侧 max，高 severity 锚点重叠更有意义。
fn severity_weighted_jaccard(
    set_a: &BTreeSet<(String, i64)>,
    set_b: &BTreeSet<(String, i64)>,
    sev_a: &HashMap<(String, i64), f64>,
    sev_b: &HashMap<(String, i64), f64>,
) -> f64 {
    let union: BTreeSet<(String, i64)> = set_a.union(set_b).cloned().collect();
    if union.is_empty() {
        return 0.0;
    }
    let w = |x: &(String, i64)| sev_a.get(x).copied().unwrap_or(0.0).max(sev_b.get(x).copied().unwrap_or(0.0));
    let w_inter: f64 = set_a.intersection(set_b).map(&w).sum();
    let w_union: f64 = union.iter().map(&w).sum();
    if w_union > 0.0 {
        w_inter / w_union
    } else {
        0.0
    }
}

/// severity 加权 Jaccard 相似度均值。
pub fn compute_sw_jaccard_similarity(
    facetor_anchor_sets: &HashMap<String, BTreeSet<(String, i64)>>,
    facetor_anchor_severity: &HashMap<String, HashMap<(String, i64), f64>>,
) -> Value {
    let mut actors: Vec<String> = facetor_anchor_sets.keys().cloned().collect();
    actors.sort();
    if actors.len() < 2 {
        return json!({"mean_sw_jaccard": 0.0, "note": "facetor 数 < 2"});
    }
    let mut scores = Vec::new();
    for i in 0..actors.len() {
        for b in actors.iter().skip(i + 1) {
            let score = severity_weighted_jaccard(
                &facetor_anchor_sets[&actors[i]],
                &facetor_anchor_sets[b],
                facetor_anchor_severity.get(&actors[i]).unwrap_or(&HashMap::new()),
                facetor_anchor_severity.get(b).unwrap_or(&HashMap::new()),
            );
            scores.push(score);
        }
    }
    let mean = if scores.is_empty() { 0.0 } else { scores.iter().sum::<f64>() / scores.len() as f64 };
    json!({"mean_sw_jaccard": py_round(mean, 4)})
}

/// severity 分布：unknown 归 (unknown) 桶。
pub fn compute_severity_distribution(all_severities: &[String]) -> Value {
    let mut counter: Vec<(String, i64)> = Vec::new();
    let mut bump = |key: &str| {
        if let Some(e) = counter.iter_mut().find(|(k, _)| k == key) {
            e.1 += 1;
        } else {
            counter.push((key.to_string(), 1));
        }
    };
    for s in all_severities {
        if KNOWN_SEVERITIES.contains(&s.as_str()) {
            bump(s);
        } else {
            bump("(unknown)");
        }
    }
    let mut m = Map::new();
    for (k, v) in counter {
        m.insert(k, json!(v));
    }
    Value::Object(m)
}

/// token 消耗汇总 + latency 汇总。
pub fn compute_token_usage(records: &[Value]) -> Value {
    let mut total_in = 0i64;
    let mut total_out = 0i64;
    let mut total_lat = 0i64;
    for r in records {
        total_in += r.get("input_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        total_out += r.get("output_tokens").and_then(|v| v.as_i64()).unwrap_or(0);
        total_lat += r.get("latency_ms").and_then(|v| v.as_i64()).unwrap_or(0);
    }
    json!({
        "input_tokens": total_in,
        "output_tokens": total_out,
        "total_tokens": total_in + total_out,
        "latency_ms_total": total_lat,
        "calls": records.len() as i64,
    })
}

/// 锚点级分类：consensus / majority / minority / singleton。
pub fn classify_facets(facetor_anchors: &HashMap<String, BTreeSet<(String, i64)>>) -> Value {
    if facetor_anchors.is_empty() {
        return json!({"consensus": [], "majority": [], "minority": [], "singleton": []});
    }
    let n = facetor_anchors.len();
    let mut all_anchors: BTreeSet<(String, i64)> = BTreeSet::new();
    for s in facetor_anchors.values() {
        all_anchors.extend(s.iter().cloned());
    }
    let mut consensus = Vec::new();
    let mut majority = Vec::new();
    let mut minority = Vec::new();
    let mut singleton = Vec::new();
    for a in &all_anchors {
        let count = facetor_anchors.values().filter(|s| s.contains(a)).count();
        let entry = json!([a.0, a.1]);
        if count == n {
            consensus.push(entry);
        } else if count == 1 {
            singleton.push(entry);
        } else if (count as f64) > n as f64 / 2.0 {
            majority.push(entry);
        } else {
            minority.push(entry);
        }
    }
    json!({
        "consensus": consensus,
        "majority": majority,
        "minority": minority,
        "singleton": singleton,
    })
}

/// severity weight 反转回字面：>=3 critical，>=2 major，其余 minor。
pub fn weight_to_severity_str(w: f64) -> &'static str {
    if w >= 3.0 {
        "critical"
    } else if w >= 2.0 {
        "major"
    } else {
        "minor"
    }
}

/// 对一组 records 算四类指标（不读写文件，只处理内存 records）。
pub fn compute_metrics(records: &[Value], topic_path: &Path, project_root: &Path) -> Value {
    // 白名单（只加载一次，失败按零容）
    let mut whitelist_size = 0usize;
    let mut whitelist: HashSet<(String, i64)> = HashSet::new();
    if let Ok(w) = load_whitelist(topic_path, project_root) {
        whitelist_size = w.len();
        whitelist = w;
    }

    // 按 actor_id 分组（保持首遇序）
    let mut actor_order: Vec<String> = Vec::new();
    let mut by_actor: HashMap<String, Vec<&Value>> = HashMap::new();
    for r in records {
        let actor = r.get("actor_id").and_then(|v| v.as_str()).unwrap_or("");
        if !actor.is_empty() {
            if !by_actor.contains_key(actor) {
                actor_order.push(actor.to_string());
            }
            by_actor.entry(actor.to_string()).or_default().push(r);
        }
    }

    let mut facetor_anchor_sets: HashMap<String, BTreeSet<(String, i64)>> = HashMap::new();
    let mut facetor_anchor_severity: HashMap<String, HashMap<(String, i64), f64>> = HashMap::new();
    let mut facetor_anchor_latest: HashMap<String, HashMap<(String, i64), f64>> = HashMap::new();
    let mut facetor_anchor_trajectory: HashMap<String, HashMap<(String, i64), HashMap<i64, f64>>> = HashMap::new();
    let mut all_severities: Vec<String> = Vec::new();
    let mut anchor_ref_counts: HashMap<String, i64> = HashMap::new();

    for actor in &actor_order {
        let rs = &by_actor[actor];
        let mut refs: BTreeSet<(String, i64)> = BTreeSet::new();
        let mut sev_map: HashMap<(String, i64), f64> = HashMap::new();
        let mut latest_map: HashMap<(String, i64), f64> = HashMap::new();
        let mut trajectory_map: HashMap<(String, i64), HashMap<i64, f64>> = HashMap::new();
        let mut latest_round: HashMap<(String, i64), i64> = HashMap::new();
        for r in rs {
            let round_num = r.get("round").and_then(|v| v.as_i64()).unwrap_or(1);
            let output = r.get("output").and_then(|v| v.as_str()).unwrap_or("");
            for (raw_ref, sev_str) in extract_ref_severity_pairs(output) {
                let validation = validate(&raw_ref, &whitelist, project_root);
                if validation.result == "Keep" {
                    if let Some((start, _end)) = parse_range(&validation.range) {
                        let anchor_key = (validation.path.clone(), start);
                        refs.insert(anchor_key.clone());
                        let weight = severity_weight(&sev_str);
                        // max（跨轮最高，向后兼容）
                        let cur = sev_map.get(&anchor_key).copied().unwrap_or(0.0);
                        sev_map.insert(anchor_key.clone(), cur.max(weight));
                        // latest（最高轮 severity；同轮多次取 max）
                        let prev_lr = latest_round.get(&anchor_key).copied().unwrap_or(0);
                        if round_num > prev_lr {
                            latest_map.insert(anchor_key.clone(), weight);
                            latest_round.insert(anchor_key.clone(), round_num);
                        } else if round_num == prev_lr {
                            let cur = latest_map.get(&anchor_key).copied().unwrap_or(0.0);
                            latest_map.insert(anchor_key.clone(), cur.max(weight));
                        }
                        // trajectory（{round: 该轮 max weight}）
                        let slot = trajectory_map.entry(anchor_key.clone()).or_default();
                        let cur = slot.get(&round_num).copied().unwrap_or(0.0);
                        slot.insert(round_num, cur.max(weight));
                    }
                    let range = validation.range.clone();
                    *anchor_ref_counts.entry(range).or_insert(0) += 1;
                }
            }
            let output2 = r.get("output").and_then(|v| v.as_str()).unwrap_or("");
            all_severities.extend(extract_severities(output2));
        }
        facetor_anchor_sets.insert(actor.clone(), refs);
        facetor_anchor_severity.insert(actor.clone(), sev_map);
        facetor_anchor_latest.insert(actor.clone(), latest_map);
        facetor_anchor_trajectory.insert(actor.clone(), trajectory_map);
    }

    // csnx 积分：对差异信号加权积分，产出收敛方向建议
    let mut all_anchors_csnx: BTreeSet<(String, i64)> = BTreeSet::new();
    for s in facetor_anchor_sets.values() {
        all_anchors_csnx.extend(s.iter().cloned());
    }

    let mut ranking: Vec<Value> = Vec::new();
    for anchor in &all_anchors_csnx {
        let finders: Vec<&String> = actor_order
            .iter()
            .filter(|a| facetor_anchor_sets[*a].contains(anchor))
            .collect();

        let mut max_sev_weight = 0.0f64;
        for actor in &finders {
            let w = facetor_anchor_severity[*actor].get(anchor).copied().unwrap_or(0.0);
            if w > max_sev_weight {
                max_sev_weight = w;
            }
        }
        if max_sev_weight == 0.0 {
            max_sev_weight = 1.0;
        }

        let mut latest_sev_weight = 0.0f64;
        for actor in &finders {
            let w = facetor_anchor_latest[*actor].get(anchor).copied().unwrap_or(0.0);
            if w > latest_sev_weight {
                latest_sev_weight = w;
            }
        }
        if latest_sev_weight == 0.0 {
            latest_sev_weight = max_sev_weight; // 回退（旧数据无 round 字段）
        }

        let mut traj: HashMap<i64, f64> = HashMap::new();
        for actor in &finders {
            if let Some(tm) = facetor_anchor_trajectory[*actor].get(anchor) {
                for (rn, w) in tm {
                    let cur = traj.get(rn).copied().unwrap_or(0.0);
                    traj.insert(*rn, cur.max(*w));
                }
            }
        }
        let mut rns: Vec<i64> = traj.keys().copied().collect();
        rns.sort();
        let traj_str: Map<_, _> = rns
            .iter()
            .map(|rn| (rn.to_string(), json!(weight_to_severity_str(traj[rn]))))
            .collect();

        let mut families: Vec<String> = finders.iter().map(|a| extract_family(a)).collect();
        families.sort();
        families.dedup();
        let source_diversity_score = families.len();

        let mut frameworks: Vec<String> = finders.iter().map(|a| extract_framework(a)).collect();
        frameworks.sort();
        frameworks.dedup();
        let attention_bonus = if finders.iter().any(|a| is_non_default_framework(a)) { 1.5 } else { 1.0 };

        let composite = latest_sev_weight * (1.0 + source_diversity_score as f64) * attention_bonus;

        ranking.push(json!({
            "anchor": anchor.0,
            "line": anchor.1,
            "max_severity": weight_to_severity_str(max_sev_weight),
            "latest_severity": weight_to_severity_str(latest_sev_weight),
            "severity_trajectory": Value::Object(traj_str),
            "severity_weight": latest_sev_weight,
            "source_families": families,
            "source_diversity_score": source_diversity_score as i64,
            "attention_frameworks": frameworks,
            "attention_diversity_bonus": attention_bonus,
            "composite_score": py_round(composite, 2),
            "n_finders": finders.len() as i64,
        }));
    }
    // 稳定降序排（围堰 sort key=-composite，稳定序）
    ranking.sort_by(|a, b| {
        let ca = a["composite_score"].as_f64().unwrap_or(0.0);
        let cb = b["composite_score"].as_f64().unwrap_or(0.0);
        cb.partial_cmp(&ca).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut anchor_density = Map::new();
    let mut pairs: Vec<(&String, &i64)> = anchor_ref_counts.iter().map(|(k, v)| (k, v)).collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    for (k, v) in pairs {
        anchor_density.insert(k.clone(), json!(v));
    }

    json!({
        "actors": actor_order.iter().map(|s| json!(s)).collect::<Vec<_>>().into_iter().collect::<Vec<_>>(),
        "records_analyzed": records.len() as i64,
        "anchor_coverage": compute_anchor_coverage(&facetor_anchor_sets, whitelist_size),
        "jaccard_similarity": compute_jaccard_similarity(&facetor_anchor_sets),
        "sw_jaccard_similarity": compute_sw_jaccard_similarity(&facetor_anchor_sets, &facetor_anchor_severity),
        "severity_distribution": compute_severity_distribution(&all_severities),
        "token_usage": compute_token_usage(records),
        "facet_classification": classify_facets(&facetor_anchor_sets),
        "csnx_integral": {
            "total_input_facets": records.len() as i64,
            "total_unique_anchors": all_anchors_csnx.len() as i64,
            "ranking": ranking,
        },
        "severity_yield": {
            "total_facets": records.len() as i64,
            "per_actor": if by_actor.is_empty() { json!(0) } else { json!(py_round(records.len() as f64 / by_actor.len() as f64, 2)) },
            "max_anchor_density": anchor_ref_counts.values().copied().max().unwrap_or(0),
            "anchor_density": Value::Object(anchor_density),
        },
    })
}

/// 从 integrator output 解析 decision JSON 字段。
pub fn extract_decision_fields(output: &str) -> Option<Value> {
    let parsed = parse_payload(output)?;
    if parsed.get("decision").is_none() {
        return None;
    }
    let required = ["decision", "basis_regulation", "reason", "boundary_flag"];
    if required.iter().any(|k| parsed.get(k).is_none()) {
        return None;
    }
    Some(json!({
        "decision": parsed["decision"],
        "basis_regulation": parsed["basis_regulation"],
        "reason": parsed.get("reason").cloned().unwrap_or(Value::String(String::new())),
        "boundary_flag": parsed.get("boundary_flag").and_then(|v| v.as_bool()).unwrap_or(false),
    }))
}

/// 对 integrator records 算决策收敛指标（单 run 内聚合）。
pub fn compute_decision_convergence(records: &[Value]) -> Value {
    if records.is_empty() {
        return json!({
            "n_integrators": 0, "agreement_rate": 0.0, "declared_boundary_rate": 0.0,
            "cell": "no_data", "modal_decision": null, "per_actor": [], "dissent": [],
        });
    }
    let mut decisions: Vec<Value> = Vec::new();
    for r in records {
        let output = r.get("output").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(mut d) = extract_decision_fields(output) {
            let obj = d.as_object_mut().unwrap();
            obj.insert(
                "actor_id".into(),
                r.get("actor_id").cloned().unwrap_or(Value::String(String::new())),
            );
            obj.insert(
                "model_id".into(),
                r.get("model_id").cloned().unwrap_or(Value::String(String::new())),
            );
            let actor_id = r.get("actor_id").and_then(|v| v.as_str()).unwrap_or("");
            let model_id = r.get("model_id").and_then(|v| v.as_str()).unwrap_or("");
            let family_src = if actor_id.is_empty() { model_id } else { actor_id };
            obj.insert("family".into(), json!(extract_family(family_src)));
            decisions.push(d);
        }
    }
    if decisions.is_empty() {
        return json!({
            "n_integrators": records.len() as i64, "agreement_rate": 0.0,
            "declared_boundary_rate": 0.0, "cell": "no_data", "modal_decision": null,
            "per_actor": [], "dissent": [],
        });
    }
    let n = decisions.len();
    // 众数：first-inserted max on tie（围堰 Counter.most_common 稳定语义）
    let mut order: Vec<String> = Vec::new();
    let mut counts: HashMap<String, i64> = HashMap::new();
    let mut boundary_count = 0i64;
    for d in &decisions {
        let dec = d["decision"].as_str().unwrap_or_default().to_string();
        if !order.contains(&dec) {
            order.push(dec.clone());
        }
        *counts.entry(dec).or_insert(0) += 1;
        if d["boundary_flag"].as_bool().unwrap_or(false) {
            boundary_count += 1;
        }
    }
    let mut modal_decision = order[0].clone();
    let mut modal_count = counts[&modal_decision];
    for dec in order.iter().skip(1) {
        let c = counts[dec];
        if c > modal_count {
            modal_decision = dec.clone();
            modal_count = c;
        }
    }
    let agreement_rate = modal_count as f64 / n as f64;
    let declared_boundary_rate = boundary_count as f64 / n as f64;
    let is_boundary = declared_boundary_rate >= 0.5;
    let is_converged = agreement_rate >= 0.5;
    let cell = if is_boundary {
        if is_converged { "boundary_converged" } else { "boundary_diverged" }
    } else if is_converged {
        "clear_converged"
    } else {
        "clear_diverged"
    };

    let per_actor: Vec<Value> = decisions
        .iter()
        .map(|d| {
            json!({
                "actor_id": d["actor_id"],
                "model_id": d["model_id"],
                "family": d["family"],
                "decision": d["decision"],
                "basis_regulation": d["basis_regulation"],
                "reason": d["reason"],
                "boundary_flag": d["boundary_flag"],
            })
        })
        .collect();

    let mut dissent = Vec::new();
    if agreement_rate < 1.0 {
        for d in &decisions {
            if d["decision"].as_str() != Some(modal_decision.as_str()) {
                dissent.push(json!({
                    "actor_id": d["actor_id"],
                    "model_id": d["model_id"],
                    "family": d["family"],
                    "decision": d["decision"],
                    "reason": d["reason"],
                }));
            }
        }
    }

    json!({
        "n_integrators": n as i64,
        "agreement_rate": py_round(agreement_rate, 4),
        "declared_boundary_rate": py_round(declared_boundary_rate, 4),
        "cell": cell,
        "modal_decision": modal_decision,
        "per_actor": per_actor,
        "dissent": dissent,
    })
}

/// 从 facetor records 投影文本自由的发散发谱（A2，积分可见）。
pub fn compute_divergence_shape(records: &[Value], topic_path: &Path, project_root: &Path) -> Value {
    let empty_classification = json!({"consensus": 0, "majority": 0, "minority": 0, "singleton": 0});
    if records.is_empty() {
        return json!({
            "n_facetors": 0, "total_facets": 0, "severity_distribution": {},
            "jaccard_mean": 0.0, "sw_jaccard_mean": 0.0, "anchor_coverage_rate": 0.0,
            "classification_counts": empty_classification, "top_anchors": [],
        });
    }
    let metrics = compute_metrics(records, topic_path, project_root);
    let severity_distribution = metrics["severity_distribution"].clone();
    let total_facets: i64 = severity_distribution
        .as_object()
        .map(|m| m.values().filter_map(|v| v.as_i64()).sum())
        .unwrap_or(0);
    let facet_class = &metrics["facet_classification"];
    let count = |k: &str| facet_class[k].as_array().map(|a| a.len() as i64).unwrap_or(0);
    let classification_counts = json!({
        "consensus": count("consensus"),
        "majority": count("majority"),
        "minority": count("minority"),
        "singleton": count("singleton"),
    });
    let ranking = &metrics["csnx_integral"]["ranking"];
    let top_anchors: Vec<Value> = ranking
        .as_array()
        .map(|a| a.iter().take(5).map(|r| json!({
            "anchor": r["anchor"],
            "line": r["line"],
            "composite_score": r["composite_score"],
            "max_severity": r["max_severity"],
            "n_finders": r["n_finders"],
            "source_families": r["source_families"],
        })).collect())
        .unwrap_or_default();
    json!({
        "n_facetors": metrics["actors"].as_array().map(|a| a.len() as i64).unwrap_or(0),
        "total_facets": total_facets,
        "severity_distribution": severity_distribution,
        "jaccard_mean": metrics["jaccard_similarity"]["mean_jaccard"],
        "sw_jaccard_mean": metrics["sw_jaccard_similarity"]["mean_sw_jaccard"],
        "anchor_coverage_rate": metrics["anchor_coverage"]["coverage_rate"],
        "classification_counts": classification_counts,
        "top_anchors": top_anchors,
    })
}

/// 渲染 shape dict 为 integrator prompt 可读的紧凑文本。空发散发谱显式
/// 标注，不静默留空。
pub fn format_divergence_shape(shape: &Value) -> String {
    let n_facetors = shape.get("n_facetors").and_then(|v| v.as_i64()).unwrap_or(0);
    if shape.as_object().map(|m| m.is_empty()).unwrap_or(true) || n_facetors == 0 {
        return "（无前置微分数据，发散发谱为空）".to_string();
    }
    let sd = &shape["severity_distribution"];
    let cc = &shape["classification_counts"];
    let sd_get = |k: &str| sd.get(k).and_then(|v| v.as_i64()).unwrap_or(0);
    let cc_get = |k: &str| cc.get(k).and_then(|v| v.as_i64()).unwrap_or(0);
    let mut lines = vec![
        format!("审阅者数：{}", n_facetors),
        format!("总发现数：{}", shape["total_facets"].as_i64().unwrap_or(0)),
        format!(
            "严重度分布：critical={} major={} minor={}",
            sd_get("critical"),
            sd_get("major"),
            sd_get("minor")
        ),
        format!("锚点重叠（Jaccard 均值）：{}", fmt_num(shape["jaccard_mean"].as_f64().unwrap_or(0.0))),
        format!("严重度加权 Jaccard：{}", fmt_num(shape["sw_jaccard_mean"].as_f64().unwrap_or(0.0))),
        format!("锚点覆盖率：{}", fmt_num(shape["anchor_coverage_rate"].as_f64().unwrap_or(0.0))),
        format!(
            "发现分类计数：一致={} 多数={} 少数={} 独有={}",
            cc_get("consensus"),
            cc_get("majority"),
            cc_get("minority"),
            cc_get("singleton")
        ),
    ];
    if let Some(top) = shape.get("top_anchors").and_then(|v| v.as_array()) {
        if !top.is_empty() {
            lines.push("高发散锚点（composite 排序 top）:".to_string());
            for a in top {
                lines.push(format!(
                    "  - {}:L{} severity={} 发现者={} 家族={} composite={}",
                    a["anchor"].as_str().unwrap_or_default(),
                    a["line"].as_i64().unwrap_or(0),
                    a["max_severity"].as_str().unwrap_or_default(),
                    a["n_finders"].as_i64().unwrap_or(0),
                    a["source_families"],
                    fmt_num(a["composite_score"].as_f64().unwrap_or(0.0)),
                ));
            }
        }
    }
    lines.join("\n")
}

/// 浮点文本形对齐 Python str(f)（repr 最短往返）。
fn fmt_num(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e16 {
        format!("{:.1}", x)
    } else {
        format!("{}", x)
    }
}

/// 读 run.jsonl（单文件）或 run_r*.jsonl（目录），返回分轮聚合 metrics。
pub fn aggregate(run_path: &Path, topic_path: &Path, project_root: &Path) -> Result<Value, super::jsonc::PyError> {
    let _ = io_to_py; // 保留信封引用（后续 IO 错误路径）
    let is_file_mode = run_path.is_file() || !run_path.exists();

    let mut records: Vec<Value> = Vec::new();
    let mut round_files: Vec<String> = Vec::new();
    let mut by_round: std::collections::BTreeMap<i64, Vec<Value>> = std::collections::BTreeMap::new();

    if is_file_mode {
        records = load_jsonl(run_path);
    } else {
        let re = regex::Regex::new(r"run_r([0-9]+)").unwrap();
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(run_path)
            .map_err(|e| super::jsonc::io_to_py(&e))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().starts_with("run_r") && n.to_string_lossy().ends_with(".jsonl"))
                    .unwrap_or(false)
            })
            .collect();
        files.sort_by_key(|p| {
            let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            re.captures(&name)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse::<i64>().ok())
                .unwrap_or(i64::MAX)
        });
        if files.is_empty() {
            let fallback = run_path.join("run.jsonl");
            if fallback.exists() {
                records = load_jsonl(&fallback);
            }
        } else {
            round_files = files
                .iter()
                .map(|f| f.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default())
                .collect();
            for f in &files {
                let name = f.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                let round_num = re
                    .captures(&name)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().parse::<i64>().ok())
                    .unwrap_or(1);
                by_round.insert(round_num, load_jsonl(f));
            }
            records = by_round.values().flatten().cloned().collect();
        }
    }

    let mut merged = compute_metrics(&records, topic_path, project_root);
    let decision_convergence = compute_decision_convergence(&records);
    if let Some(m) = merged.as_object_mut() {
        m.insert("decision_convergence".into(), decision_convergence);
    }

    let mut per_round: Map<String, Value> = Map::new();
    let mut cross_round: Value = json!({});

    if !is_file_mode && !by_round.is_empty() {
        for (round_num, rnd_records) in &by_round {
            per_round.insert(round_num.to_string(), compute_metrics(rnd_records, topic_path, project_root));
        }
        if by_round.len() >= 2 {
            let keys: Vec<i64> = by_round.keys().copied().collect();
            let r1_key = keys[0].to_string();
            let r2_key = keys[1].to_string();
            let r1_data = &per_round[&r1_key];
            let r2_data = &per_round[&r2_key];
            let j1 = r1_data["jaccard_similarity"]["mean_jaccard"].as_f64().unwrap_or(0.0);
            let j2 = r2_data["jaccard_similarity"]["mean_jaccard"].as_f64().unwrap_or(0.0);

            let extract_anchor_set = |records: &[Value]| -> BTreeSet<(String, i64)> {
                let mut s = BTreeSet::new();
                let whitelist = load_whitelist(topic_path, project_root).unwrap_or_default();
                for r in records {
                    let output = r.get("output").and_then(|v| v.as_str()).unwrap_or("");
                    for raw_ref in extract_facet_refs(output) {
                        let val = validate(&raw_ref, &whitelist, project_root);
                        if val.result == "Keep" {
                            if let Some((start, _)) = parse_range(&val.range) {
                                s.insert((val.path.clone(), start));
                            }
                        }
                    }
                }
                s
            };
            let anchors1 = extract_anchor_set(&by_round[&keys[0]]);
            let anchors2 = extract_anchor_set(&by_round[&keys[1]]);

            let cls_level = |name: &str| match name {
                "singleton" => 1i64,
                "minority" => 2,
                "majority" => 3,
                "consensus" => 4,
                _ => 0,
            };
            let cls_to_anchor_levels = |cls: &Value| -> BTreeSet<(String, i64, i64)> {
                let mut result = BTreeSet::new();
                if let Some(m) = cls.as_object() {
                    for (level_name, anchors_list) in m {
                        if let Some(arr) = anchors_list.as_array() {
                            for anchor in arr {
                                if let Some(pair) = anchor.as_array() {
                                    if pair.len() >= 2 {
                                        result.insert((
                                            pair[0].as_str().unwrap_or_default().to_string(),
                                            pair[1].as_i64().unwrap_or(0),
                                            cls_level(level_name),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                result
            };
            let r1_levels = cls_to_anchor_levels(&r1_data["facet_classification"]);
            let r2_levels = cls_to_anchor_levels(&r2_data["facet_classification"]);
            let mut promotion = 0i64;
            let mut demotion = 0i64;
            let mut stable = 0i64;
            let mut promoted_anchors: Vec<Value> = Vec::new();
            let mut demoted_anchors: Vec<Value> = Vec::new();
            for anchor in r1_levels.iter().chain(r2_levels.iter()).map(|(p, l, _)| (p.clone(), *l)) {
                let l1 = r1_levels
                    .iter()
                    .find(|(p, l, _)| *p == anchor.0 && *l == anchor.1)
                    .map(|(_, _, lv)| *lv)
                    .unwrap_or(0);
                let l2 = r2_levels
                    .iter()
                    .find(|(p, l, _)| *p == anchor.0 && *l == anchor.1)
                    .map(|(_, _, lv)| *lv)
                    .unwrap_or(0);
                let delta = l2 - l1;
                if delta > 0 {
                    promotion += 1;
                    promoted_anchors.push(json!([anchor.0, anchor.1]));
                } else if delta < 0 {
                    demotion += 1;
                    demoted_anchors.push(json!([anchor.0, anchor.1]));
                } else {
                    stable += 1;
                }
            }

            cross_round = json!({
                "jaccard_drift": py_round(j1 - j2, 4),
                "new_anchors": anchors2.difference(&anchors1).count() as i64,
                "repeated_anchors": anchors2.intersection(&anchors1).count() as i64,
                "classification_drift": {
                    "promotion": promotion,
                    "demotion": demotion,
                    "stable": stable,
                    "promoted_anchors": promoted_anchors,
                    "demoted_anchors": demoted_anchors,
                },
            });
        }
    }

    if is_file_mode {
        // 单文件模式：旧字段在顶层（向后兼容），merged 仅作承载后弹出
        let mut result = match merged {
            Value::Object(m) => m,
            _ => Map::new(),
        };
        if let Some(per) = result.get("decision_convergence") {
            // 保留在顶层（围堰 merged 内含 dc 字段，顶层即同字段）
            let _ = per;
        }
        result.insert(
            "per_round".into(),
            Value::Object(per_round),
        );
        result.insert("cross_round".into(), cross_round);
        result.insert("_round_files".into(), json!(round_files));
        result.insert("jsonl_path".into(), json!(run_path.to_string_lossy()));
        Ok(Value::Object(result))
    } else {
        Ok(json!({
            "per_round": Value::Object(per_round),
            "cross_round": cross_round,
            "merged": merged,
            "_round_files": round_files,
        }))
    }
}
