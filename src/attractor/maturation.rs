//! maturation——判据 v4 三态计算（m-gatesplit 批，闸自围堰融回引擎）。
//!
//! 语义移植自围堰 assess_maturation 基座（sih-tools/facet/probes/
//! maturation_gate.py，冻结只读只作定位引用）：三子判据合取裁决三态
//! stable_clear / near_threshold / boundary，任一明确挂 → boundary；全过
//! 但有子判据贴近阈值 → near_threshold；全过无贴近 → stable_clear。
//!
//! v4 修订（判据修订批 m-gatesplit，gid m-gatesplit-1）：旗语分立——
//! boundary_flag 重定义为裁判信心信号（拿不准举旗），coverage_flag 新增为
//! 题材超纲描述信号。boundary_rate 只统计 boundary_flag；coverage 计数
//! 单列为报告项（规则缺口台账的机械承载），不参与三态判定。
//!
//! 移植范畴声明：围堰 v3 闸的面板专属层（家族体温 V3-T、unanimous 快速
//! 通道 V3-U、动态基线 flip 率）为多席位多家族面板机械，合同模式采样腿
//! 为单席 Seat 家族九发，不属本腿语境，不在 v4 移植面；单席语境核心的
//! B1（basis 退化维）与 B2（单席决策噪声）降级规则如实移植（红线承 v2：
//! 任何降级不得越过 boundary_low）。阈值沿用基座三元组 0.2 / 0.34 / 0.10
//! 形。foregrounding 子判据在合同模式无输入面，不适用不入册（围堰语义
//! 缺席即恒过）。历史 trail 不重算不改写，v4 向前生效。
//!
//! 机械腿红线：零网络、零 LLM、零 key 读取、零目标仓写入。

use std::collections::HashMap;

use serde_json::{json, Map, Value};

use super::jsonc::py_round;

/// 判据代际署名（tally 校验面认 v3 历史件与 v4 新件）。
pub const CRITERIA_VERSION: &str = "v4";

/// 决策稳定性阈值：模型 flip 率低于此判稳（围堰基座 0.2 形）。
pub const DECISION_FLIP_THRESHOLD: f64 = 0.2;
/// 信心旗率阈值：boundary_rate 达此即挂（围堰基座 0.34 形）。
pub const BOUNDARY_RATE_THRESHOLD: f64 = 0.34;
/// 贴边裕度：子判据值距阈值在此内判 near（围堰基座 0.10 形）。
pub const NEAR_MARGIN: f64 = 0.10;
/// B2 降级容差：decision_stable 唯一硬挂且整体非众数比例不超此值时降 near
/// （承 v2 降级规则族 `_V2_FLIP_TOLERANCE`，v3_base 原值 0.2）。
pub const B2_FLIP_TOLERANCE: f64 = 0.2;

/// actor_id（形如 `<framework>+<model>`）取模型名：末段。
fn model_name(actor_id: &str) -> String {
    if actor_id.is_empty() {
        return String::new();
    }
    actor_id.split('+').next_back().unwrap_or("").to_string()
}

/// flip_rate = 非众数值占比（围堰 repeat_metrics.flip_rate 同语义，
/// 众数平局取先遇）。返回 (flip_rate, modal_value)。
fn flip_rate(values: &[String]) -> (f64, String) {
    if values.is_empty() {
        return (0.0, String::new());
    }
    let mut order: Vec<String> = Vec::new();
    let mut counts: HashMap<String, i64> = HashMap::new();
    for v in values {
        if !order.iter().any(|x| x == v) {
            order.push(v.clone());
        }
        *counts.entry(v.clone()).or_insert(0) += 1;
    }
    let mut modal = order[0].clone();
    let mut modal_count = counts[&modal];
    for d in order.iter().skip(1) {
        if counts[d] > modal_count {
            modal = d.clone();
            modal_count = counts[d];
        }
    }
    (py_round(1.0 - modal_count as f64 / values.len() as f64, 4), modal)
}

/// 判据 v4：从飞轮 dc 列表（各含 per_actor）算三态裁决与 coverage 单列记账。
///
/// 输入只读；coverage_flag 键缺席按 false 计（旧四键响应向后兼容）。
pub fn assess_maturation_v4(dc_list: &[Value]) -> Value {
    // 模型名 → 跨 run decision 序列（围堰 repeat_metrics 分组同构）
    let mut per_model_decisions: HashMap<String, Vec<String>> = HashMap::new();
    let mut bflags: Vec<bool> = Vec::new();
    let mut cflags: Vec<bool> = Vec::new();
    let mut basses: Vec<String> = Vec::new();
    let mut n_shots = 0usize;
    for dc in dc_list {
        let Some(pas) = dc.get("per_actor").and_then(|v| v.as_array()) else {
            continue;
        };
        for pa in pas {
            let name = model_name(pa.get("actor_id").and_then(|v| v.as_str()).unwrap_or(""));
            let decision = pa.get("decision").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if !name.is_empty() {
                per_model_decisions.entry(name).or_default().push(decision);
            }
            bflags.push(pa.get("boundary_flag").and_then(|v| v.as_bool()).unwrap_or(false));
            cflags.push(pa.get("coverage_flag").and_then(|v| v.as_bool()).unwrap_or(false));
            if let Some(b) = pa.get("basis_regulation").and_then(|v| v.as_str()) {
                if !b.is_empty() {
                    basses.push(b.to_string());
                }
            }
            n_shots += 1;
        }
    }

    // 子判据一：决策稳定性——稳定模型同 modal 且无噪声模型
    let mut per_model = Map::new();
    let mut stable_modals: Vec<(String, String)> = Vec::new();
    let mut noise_models: Vec<String> = Vec::new();
    let mut names: Vec<String> = per_model_decisions.keys().cloned().collect();
    names.sort();
    for name in &names {
        let decs = &per_model_decisions[name];
        let (d_flip, d_modal) = flip_rate(decs);
        let stable = d_flip < DECISION_FLIP_THRESHOLD;
        per_model.insert(
            name.clone(),
            json!({
                "n_shots": decs.len(),
                "modal_decision": d_modal,
                "decision_flip_rate": d_flip,
                "stable": stable,
            }),
        );
        if stable {
            stable_modals.push((name.clone(), d_modal));
        } else {
            noise_models.push(name.clone());
        }
    }
    let stable_modal_decisions: Vec<String> = {
        let mut v: Vec<String> = stable_modals.iter().map(|(_, m)| m.clone()).collect();
        v.sort();
        v.dedup();
        v
    };
    let n_hetero_modals = stable_modal_decisions.len();
    let n_noise = noise_models.len();
    let decision_stable = n_hetero_modals <= 1 && n_noise == 0;
    let decision_near = n_hetero_modals <= 1 && n_noise > 0;

    // 子判据二：清晰性——信心旗率（v4：boundary_flag 只承载裁判信心信号）
    let boundary_rate = if bflags.is_empty() {
        0.0
    } else {
        py_round(bflags.iter().filter(|b| **b).count() as f64 / bflags.len() as f64, 4)
    };
    let boundary_low = boundary_rate < BOUNDARY_RATE_THRESHOLD;
    let boundary_near =
        (BOUNDARY_RATE_THRESHOLD - NEAR_MARGIN) <= boundary_rate && boundary_rate < BOUNDARY_RATE_THRESHOLD;

    // 子判据三：basis 共识——跨模型依据一致
    let mut distinct_basis: Vec<String> = basses.clone();
    distinct_basis.sort();
    distinct_basis.dedup();
    let basis_consensus = distinct_basis.len() <= 1;
    let basis_near = distinct_basis.len() == 2;

    // v4 单列报告项：超纲旗计数与率（描述信号，不入三态）
    let coverage_flags = cflags.iter().filter(|c| **c).count();
    let coverage_rate = if cflags.is_empty() {
        0.0
    } else {
        py_round(coverage_flags as f64 / cflags.len() as f64, 4)
    };

    let criteria = json!({
        "decision_stable": decision_stable,
        "boundary_low": boundary_low,
        "basis_consensus": basis_consensus,
    });
    let nears = json!({
        "decision_near": decision_near,
        "boundary_near": boundary_near,
        "basis_near": basis_near,
    });
    let any_fail = criteria.as_object().map(|m| m.values().any(|v| v.as_bool() == Some(false))).unwrap_or(true);
    let any_near = nears.as_object().map(|m| m.values().any(|v| v.as_bool() == Some(true))).unwrap_or(false);

    let (verdict, note) = if any_fail {
        let failed: Vec<&str> = criteria
            .as_object()
            .map(|m| m.iter().filter(|(_, v)| v.as_bool() == Some(false)).map(|(k, _)| k.as_str()).collect())
            .unwrap_or_default();
        let mut verdict = "boundary";
        let mut note = format!("v4 子判据挂：{:?} → 路由法层 refine（分歧诊断→补规范→重入飞轮）", failed);
        // B1/B2 降级重放（承 v2/v3 降级规则族，单席语境核心面）：
        // B1（basis 退化维）：basis_consensus 是唯一硬挂 + 恰两类依据 → near
        //   （单席化后 basis 维判别力退化为引用习惯噪声）；
        // B2（单席决策噪声）：decision_stable 是唯一硬挂 + 整体非众数比例
        //   ≤ 0.2 + 信心旗未挂 → near。
        // 红线承 v2：任何降级不得越过 boundary_low（信心旗是最可靠谨慎信号）。
        let mut all_decisions: Vec<String> = Vec::new();
        for decs in per_model_decisions.values() {
            all_decisions.extend(decs.iter().cloned());
        }
        let (_, global_modal) = flip_rate(&all_decisions);
        let global_flip = if all_decisions.is_empty() {
            0.0
        } else {
            let modal_count = all_decisions.iter().filter(|d| **d == global_modal).count();
            py_round(1.0 - modal_count as f64 / all_decisions.len() as f64, 4)
        };
        if failed.len() == 1 && failed[0] == "basis_consensus" && distinct_basis.len() == 2 {
            verdict = "near_threshold";
            note = "v4: B1_basis_degenerate 降级（两类依据贴边，单席引用习惯噪声）".to_string();
        } else if failed.len() == 1 && failed[0] == "decision_stable"
            && global_flip <= B2_FLIP_TOLERANCE && boundary_low
        {
            verdict = "near_threshold";
            note = "v4: B2_single_seat_noise 降级（非众数比例不超容差且信心旗未挂）".to_string();
        }
        (verdict, note)
    } else if any_near {
        let near_crit: Vec<&str> = nears
            .as_object()
            .map(|m| m.iter().filter(|(_, v)| v.as_bool() == Some(true)).map(|(k, _)| k.as_str()).collect())
            .unwrap_or_default();
        (
            "near_threshold",
            format!("v4 全过但贴近阈值：{:?} → 人复核（分离不确定）", near_crit),
        )
    } else {
        ("stable_clear", "v4 全过且无贴近 → 送终签核对腿".to_string())
    };

    json!({
        "verdict": verdict,
        "note": note,
        "criteria_version": CRITERIA_VERSION,
        "criteria": criteria,
        "near_flags": nears,
        "metrics": {
            "n_runs": dc_list.len(),
            "n_shots": n_shots,
            "per_model": Value::Object(per_model),
            "stable_modal_decisions": stable_modal_decisions,
            "noise_models": noise_models,
            "boundary_rate": boundary_rate,
            "distinct_basis": distinct_basis,
            "coverage_flags": coverage_flags,
            "coverage_rate": coverage_rate,
        },
        "thresholds": {
            "decision_flip": DECISION_FLIP_THRESHOLD,
            "boundary_rate": BOUNDARY_RATE_THRESHOLD,
            "near_margin": NEAR_MARGIN,
        },
        "coverage_note": "coverage_flag 为题材超纲描述信号：单列记账不入三态（规则缺口台账机械承载）",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pa(boundary: bool, coverage: bool, decision: &str, basis: &str) -> Value {
        json!({
            "actor_id": "ZCode+GLM-Test",
            "model_id": "GLM-Test",
            "family": "Seat",
            "decision": decision,
            "basis_regulation": basis,
            "boundary_flag": boundary,
            "coverage_flag": coverage,
            "reason": "r",
        })
    }

    fn dc(entries: Vec<Value>) -> Value {
        json!({ "per_actor": entries })
    }

    #[test]
    fn v4_all_clear_stable() {
        let entries: Vec<Value> = (0..9).map(|_| pa(false, false, "comply", "baseline_4")).collect();
        let out = assess_maturation_v4(&[dc(entries)]);
        assert_eq!(out["verdict"], "stable_clear");
        assert_eq!(out["metrics"]["coverage_flags"], 0);
        assert_eq!(out["metrics"]["boundary_rate"], 0.0);
    }

    #[test]
    fn v4_confidence_flag_trips_boundary() {
        // 信心旗 4/9 ≈ 0.4444 ≥ 0.34 → boundary（旧单旗语义下超纲也走这里，v4 分立后只有信心旗走）
        let mut entries: Vec<Value> = (0..5).map(|_| pa(false, true, "comply", "baseline_4")).collect();
        entries.extend((0..4).map(|_| pa(true, false, "comply", "baseline_4")));
        let out = assess_maturation_v4(&[dc(entries)]);
        assert_eq!(out["verdict"], "boundary");
        assert_eq!(out["criteria"]["boundary_low"], false);
        assert_eq!(out["metrics"]["boundary_rate"], 0.4444);
        assert_eq!(out["metrics"]["coverage_flags"], 5);
    }

    #[test]
    fn v4_coverage_flag_never_trips_only_books() {
        // 超纲旗 9/9 全举：三态仍 stable_clear，coverage 单列记账
        let entries: Vec<Value> = (0..9).map(|_| pa(false, true, "comply", "baseline_4")).collect();
        let out = assess_maturation_v4(&[dc(entries)]);
        assert_eq!(out["verdict"], "stable_clear");
        assert_eq!(out["criteria"]["boundary_low"], true);
        assert_eq!(out["metrics"]["coverage_flags"], 9);
        assert_eq!(out["metrics"]["coverage_rate"], 1.0);
    }

    #[test]
    fn v4_boundary_near_band() {
        // 信心旗 3/9 ≈ 0.3333 落 [0.24, 0.34) → near_threshold
        let mut entries: Vec<Value> = (0..6).map(|_| pa(false, false, "comply", "baseline_4")).collect();
        entries.extend((0..3).map(|_| pa(true, false, "comply", "baseline_4")));
        let out = assess_maturation_v4(&[dc(entries)]);
        assert_eq!(out["verdict"], "near_threshold");
        assert_eq!(out["near_flags"]["boundary_near"], true);
    }

    #[test]
    fn v4_decision_flip_trips() {
        // 4/9 非众数 → flip 0.4444 ≥ 0.2 → 噪声模型 → decision_stable 挂；
        // B2 降级要求 flip ≤ 0.2 不满足 → boundary（不降级）
        let mut entries: Vec<Value> = (0..5).map(|_| pa(false, false, "comply", "baseline_4")).collect();
        entries.extend((0..4).map(|_| pa(false, false, "violate", "baseline_4")));
        let out = assess_maturation_v4(&[dc(entries)]);
        assert_eq!(out["verdict"], "boundary");
        assert_eq!(out["criteria"]["decision_stable"], false);
        assert_eq!(out["near_flags"]["decision_near"], true);
    }

    #[test]
    fn v4_b2_single_seat_noise_downgrade() {
        // 10 发 2 发非众数 → flip 恰 0.2：decision_stable 挂（0.2 不小于 0.2），
        // B2 降级（flip ≤ 0.2 且信心旗未挂）→ near_threshold
        let mut entries: Vec<Value> = (0..8).map(|_| pa(false, false, "comply", "baseline_4")).collect();
        entries.extend((0..2).map(|_| pa(false, false, "violate", "baseline_4")));
        let out = assess_maturation_v4(&[dc(entries)]);
        assert_eq!(out["verdict"], "near_threshold");
        assert!(out["note"].as_str().unwrap().contains("B2_single_seat_noise"));
    }

    #[test]
    fn v4_basis_split_trips() {
        // basis 两类 → B1 退化降级（单席引用习惯噪声）→ near_threshold
        let two: Vec<Value> = (0..9)
            .map(|i| pa(false, false, "comply", if i < 5 { "baseline_1" } else { "baseline_4" }))
            .collect();
        let out = assess_maturation_v4(&[dc(two)]);
        assert_eq!(out["verdict"], "near_threshold");
        assert!(out["note"].as_str().unwrap().contains("B1_basis_degenerate"));
        assert_eq!(out["near_flags"]["basis_near"], true);

        // basis 三类 → 超 B1 降级面 → boundary
        let mut mixed: Vec<Value> = Vec::new();
        for b in ["baseline_1", "baseline_4", "baseline_5"] {
            for _ in 0..3 {
                mixed.push(pa(false, false, "comply", b));
            }
        }
        let out2 = assess_maturation_v4(&[dc(mixed)]);
        assert_eq!(out2["verdict"], "boundary");
        assert_eq!(out2["criteria"]["basis_consensus"], false);
    }

    #[test]
    fn v4_missing_coverage_key_counts_false() {
        // 旧四键 per_actor（无 coverage_flag 键）：缺席即 false，向后兼容
        let entries: Vec<Value> = (0..9)
            .map(|_| {
                json!({
                    "actor_id": "ZCode+GLM-Test",
                    "model_id": "GLM-Test",
                    "family": "Seat",
                    "decision": "comply",
                    "basis_regulation": "baseline_4",
                    "boundary_flag": false,
                    "reason": "r",
                })
            })
            .collect();
        let out = assess_maturation_v4(&[dc(entries)]);
        assert_eq!(out["verdict"], "stable_clear");
        assert_eq!(out["metrics"]["coverage_flags"], 0);
        assert_eq!(out["metrics"]["coverage_rate"], 0.0);
    }

    #[test]
    fn v4_multi_run_flattens() {
        // 多 run 摊平：跨 run 信心旗累计计数
        let r1 = dc(vec![pa(true, false, "comply", "baseline_4"); 3]);
        let r2 = dc(vec![pa(false, true, "comply", "baseline_4"); 6]);
        let out = assess_maturation_v4(&[r1, r2]);
        assert_eq!(out["metrics"]["n_runs"], 2);
        assert_eq!(out["metrics"]["n_shots"], 9);
        assert_eq!(out["metrics"]["boundary_rate"], 0.3333);
        assert_eq!(out["metrics"]["coverage_flags"], 6);
        assert_eq!(out["verdict"], "near_threshold");
    }
}
