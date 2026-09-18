//! dispatch 面（pl-02）：档位配置与采样合同生成与回填插拔接口。
//!
//! 承 DES-017 编排三段之二「分派」：按档位（CLI --gears > 配置件 --config
//! > 缺省 3，值域 3/6/9/off）为每候选项产一份测量合同 JSON，形对齐
//! attractor/facet 合同模式（kind=facet-sampling-contract，字段含命题文本、
//! 采样发数 shots=gear 值与席位框架），经引擎 attractor 合同内核落盘——
//! 未来活模型回填腿可直接把本件合同喂给既有采样与计分链。
//!
//! 回填腿插拔形：v1 不调任何模型不回填，合同落盘后由 `pendline backfill`
//! 接受外部回填件（--responses jsonl，facet 纪律严格对表合同 shots）进
//! 评分路由；判词（verdict）系上游直读（--verdict 或 --verdict-material
//! 的 verdict/gate_verdict 字段），编排层零新判定语义。
//! off 档即零采样：不落合同，候裁单直呈人节点全人工（manifest mode=manual）。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};

use super::collect::CANDIDATES_KIND;
use super::config;
use super::{read_json, sanitize, write_json, Args, Gear, PendError};

pub const MANIFEST_KIND: &str = "pendline-manifest";
pub const MANIFEST_VERSION: i64 = 1;
pub const BACKFILL_KIND: &str = "pendline-backfill";
pub const BACKFILL_VERSION: i64 = 1;

/// v1 合同的系统席位框架串（零模型采样，仅占位合同数据；回填腿插上时
/// 由调用方以 --seat 覆写为真实框架:模型）。
pub const DEFAULT_SEAT: &str = "pendline:unbound";

/// v1 合同固定系统框架文本（合同数据非提示词活动，零模型调用）。
/// 输出格式行须字面携带 basis_regulation 枚举声明（值段以「 或 」分隔），
/// 评分器 declared_basis_enum 按此提取合法枚举集零回落（pendfix 批）。
const SYSTEM_PROMPT: &str = "pendline 候裁处置测量合同：对下列候裁项依既立命题独立判定，输出 JSON 判定四键，不要输出 JSON 以外的文本。输出格式：{\"decision\": \"comply 或 violate\", \"basis_regulation\": \"baseline_1 或 baseline_4 或 baseline_5\", \"reason\": \"一句话理由\", \"boundary_flag\": true 或 false}。字段说明：decision 即命题符合规约与否；basis_regulation 即判定依据哪条基线即 baseline_1 确定性执行者与 baseline_4 可验证性与 baseline_5 减少 LLM 参与；reason 即一句话理由指明依据哪条规约的哪条要求；boundary_flag 即命题是否落在规范边界须推理适用。";

/// dispatch 子命令入口。
pub fn run(args: &Args) -> Result<i32, PendError> {
    let candidates_path = args.get("candidates")?;
    let out_dir = args.get("out")?;
    let gear = config::resolve(args.opt("gears"), args.opt("config"))?;
    let seat_raw = args.opt("seat").unwrap_or_else(|| DEFAULT_SEAT.to_string());
    let seat_parts: Vec<&str> = seat_raw.split(':').collect();
    if seat_parts.len() != 2 || seat_parts[0].is_empty() || seat_parts[1].is_empty() {
        return Err(PendError::usage(format!("--seat 须为 framework:model 形：{}", seat_raw)));
    }

    let cv = read_json(Path::new(&candidates_path))?;
    if let Some(k) = cv.get("kind").and_then(|v| v.as_str()) {
        if k != CANDIDATES_KIND {
            return Err(PendError::usage(format!(
                "候选清单 kind 不符：{}（应为 {}）",
                k, CANDIDATES_KIND
            )));
        }
    }
    let candidates = cv
        .get("candidates")
        .and_then(|v| v.as_array())
        .ok_or_else(|| PendError::usage("候选清单缺 candidates 数组"))?;

    std::fs::create_dir_all(&out_dir)
        .map_err(|e| PendError::usage(format!("输出目录创建失败 {}：{}", out_dir, e)))?;

    let mut contracts: Vec<Value> = Vec::new();
    if let Some(n) = gear.shots() {
        for cand in candidates {
            let id = cand.get("id").map(str_field).unwrap_or_default();
            let title = cand
                .get("title")
                .map(str_field)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| id.clone());
            let summary = cand
                .get("summary")
                .map(str_field)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| title.clone());
            let source = cand.get("source").map(str_field).unwrap_or_default();
            let path = emit_measurement_contract(
                &PathBuf::from(&out_dir),
                &id,
                &title,
                &summary,
                &source,
                n,
                &seat_parts,
            )?;
            contracts.push(json!({"id": id, "path": path.display().to_string(), "shots": n}));
        }
    }

    let (mode, note) = match gear {
        Gear::Off => (
            "manual",
            "关档零采样：候裁单直呈人节点全人工处置，零采样合同落盘（DES-017 档位配置面）",
        ),
        _ => ("sampling", "按档位出采样合同，回填腿插拔：backfill 子命令接外部回填件"),
    };
    let manifest = json!({
        "kind": MANIFEST_KIND,
        "version": MANIFEST_VERSION,
        "gears": gear.to_json(),
        "mode": mode,
        "note": note,
        "seat": format!("{}:{}", seat_parts[0], seat_parts[1]),
        "candidates": candidates,
        "contracts": contracts,
        "out": out_dir,
    });
    let manifest_path = Path::new(&out_dir).join("manifest.json");
    write_json(&manifest_path, &manifest)?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "dispatched": contracts.len(),
            "gears": gear.to_json(),
            "mode": mode,
            "manifest": manifest_path.display().to_string(),
        }))
        .unwrap_or_default()
    );
    Ok(0)
}

/// 单份测量合同落盘（attractor contract_mode 内核，kind=facet-sampling-contract）。
fn emit_measurement_contract(
    out_dir: &Path,
    gid: &str,
    title: &str,
    summary: &str,
    source: &str,
    shots_n: i64,
    seat_parts: &[&str],
) -> Result<PathBuf, PendError> {
    use sih_engine::attractor::contract_mode;
    let seat = json!({
        "framework": seat_parts[0],
        "model_id": seat_parts[1],
        "version": "self-reported",
    });
    let ng_label = format!("pendline-gear-{}", shots_n);
    let pack = json!({
        "paradigm_id": "pendline_adjudication",
        "atom": "integrator",
        "direction": "judge",
        "ng_label": ng_label,
        "ng_text_sha256": contract_mode::sha256_bytes(ng_label.as_bytes()),
    });
    let proposition = json!({
        "gid": gid,
        "title": title,
        "text": summary,
        "topic_sha256": contract_mode::sha256_bytes(summary.as_bytes()),
    });
    let shots = contract_mode::make_shots(SYSTEM_PROMPT, summary, gid, shots_n, 1);
    let meta = json!({
        "measurement_entry": format!("pendline://{}", source),
        "n_declared": shots_n,
        "gear": shots_n,
        "candidate_source": source,
    });
    let path = out_dir.join(format!("{}.contract.json", sanitize(gid)));
    contract_mode::emit_contract(&path, &seat, &pack, &proposition, &shots, &meta)
        .map_err(py_err)?;
    Ok(path)
}

/// backfill 子命令入口：外部回填件严格对表合同 + 判词上游直读 → 回填评分
/// 材料落盘（route 的输入面）。
pub fn run_backfill(args: &Args) -> Result<i32, PendError> {
    use sih_engine::attractor::contract_mode;
    let contract_path = args.get("contract")?;
    let responses_path = args.get("responses")?;
    let out_path = args.get("out")?;

    // 判词上游直读：--verdict 串形或 --verdict-material 的 verdict/
    // gate_verdict 字段形（attractor score 材料形兼容），零计算零判定。
    let verdict = match (args.opt("verdict"), args.opt("verdict-material")) {
        (Some(v), _) => v,
        (None, Some(m)) => {
            let mv = read_json(Path::new(&m))?;
            let field = mv
                .get("gate_verdict")
                .or_else(|| mv.get("verdict"))
                .ok_or_else(|| {
                    PendError::usage(format!("判词材料缺 verdict/gate_verdict 字段：{}", m))
                })?;
            field
                .as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| PendError::usage(format!("判词材料 verdict 非字符串：{}", m)))?
        }
        (None, None) => {
            return Err(PendError::usage("backfill 须 --verdict <判词> 或 --verdict-material <判词材料.json> 二择一"))
        }
    };

    let contract = contract_mode::load_contract(Path::new(&contract_path)).map_err(py_err)?;
    let responses = contract_mode::load_responses(Path::new(&responses_path), &contract)
        .map_err(py_err)?;
    let (per_actor, voids) =
        contract_mode::dc_from_responses(&responses, &contract["seat"]).map_err(py_err)?;

    let gid = contract["proposition"]["gid"].as_str().unwrap_or_default().to_string();
    let title = contract["proposition"]["title"].as_str().unwrap_or_default().to_string();
    let summary = contract["proposition"]["text"].as_str().unwrap_or_default().to_string();
    let gear = contract["meta"]["gear"].clone();
    let material = json!({
        "kind": BACKFILL_KIND,
        "backfill_version": BACKFILL_VERSION,
        "gid": gid,
        "title": title,
        "summary": summary,
        "gear": gear,
        "n_shots": responses.len(),
        "contract_path": contract_path,
        "contract_sha256": contract_mode::sha256_file(Path::new(&contract_path)).map_err(py_err)?,
        "responses_path": responses_path,
        "responses_sha256": contract_mode::sha256_file(Path::new(&responses_path)).map_err(py_err)?,
        "voids": voids,
        "n_voids": voids.len(),
        "decisions": per_actor.iter().map(|p| p["decision"].clone()).collect::<Vec<Value>>(),
        "verdict": verdict,
        "gate_verdict": verdict,
        "runs_written": 0,
        "backfilled_at": super::now_iso(),
        "note": "v1 夹具回填：零模型零飞轮写，verdict 系上游直读零判定（DES-017 回填腿插拔）",
    });
    write_json(Path::new(&out_path), &material)?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "backfill": out_path,
            "gid": material["gid"],
            "n_shots": material["n_shots"],
            "verdict": material["verdict"],
        }))
        .unwrap_or_default()
    );
    Ok(0)
}

fn str_field(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// attractor PyError → pendline 错误信封：OSError 即环境错误 2，其余
/// （ValueError 拒收形）即违规 1（对表 attractor bin score 域退出码分域）。
fn py_err(e: sih_engine::attractor::jsonc::PyError) -> PendError {
    if e.kind == "OSError" {
        PendError::usage(e.msg)
    } else {
        PendError::violate(e.msg)
    }
}

#[cfg(test)]
mod pendfix_tests {
    use super::SYSTEM_PROMPT;

    /// 评分器 declared_basis_enum 同语义提取：值段按「 或 」分割。
    /// 病灶即旧提示词无此声明行，评分器 ValueError shot r1（pendlive 批活体
    /// 红证与残次合同在 pendlive-materials 在档），本钉防复发。
    #[test]
    fn system_prompt_declares_basis_enum_scorer_form() {
        let re = regex::Regex::new(r#""basis_regulation"\s*:\s*"([^"]*)""#).unwrap();
        let m = re
            .captures(SYSTEM_PROMPT)
            .expect("system_prompt 须含字面 basis_regulation 声明行");
        let tokens: Vec<&str> = m.get(1).unwrap().as_str().split(" 或 ").collect();
        let mut set: Vec<&str> = tokens.into_iter().map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
        set.sort_unstable();
        assert_eq!(
            set,
            vec!["baseline_1", "baseline_4", "baseline_5"],
            "枚举集须恰三值即评分器合法域"
        );
    }

    /// 四键 schema 全声明钉：decision 与 basis_regulation 与 reason 与 boundary_flag。
    #[test]
    fn system_prompt_declares_four_judgment_keys() {
        for key in ["\"decision\"", "\"basis_regulation\"", "\"reason\"", "\"boundary_flag\""] {
            assert!(
                SYSTEM_PROMPT.contains(key),
                "system_prompt 缺四键 schema 声明：{key}"
            );
        }
    }
}
