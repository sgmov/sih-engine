//! paradigm_loader——PRO-001 范式装载与编排规则检查（融回自围堰
//! facet/src/paradigm_loader.py，contract_mode 出题依赖件）。
//!
//! yaml 装载以 serde_yaml 承载（引擎侧按需引依赖，Cargo.toml 随批声明）；
//! atom 与 chain yaml 路径由调用方显式给位（围堰缺省路径语义不迁，双模
//! 并存期围堰原位有效）。

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;

use super::jsonc::{io_to_py, PyError};

fn load_yaml(path: &Path) -> Result<Value, PyError> {
    let text = std::fs::read_to_string(path).map_err(|e| io_to_py(&e))?;
    let v: Value = serde_yaml::from_str(&text)
        .map_err(|e| super::jsonc::jsonerr(format!("{}", e)))?;
    Ok(v)
}

/// 加载 atom.yaml，返回 atom_name -> atom 配置映射。
pub fn load_atoms(yaml_path: &Path) -> Result<Value, PyError> {
    let raw = load_yaml(yaml_path)?;
    Ok(raw.get("atoms").cloned().unwrap_or(Value::Null))
}

/// 加载 atom-chain.yaml，返回 paradigm_id -> 范式配置映射。
pub fn load_paradigms(yaml_path: &Path) -> Result<BTreeMap<String, Value>, PyError> {
    let raw = load_yaml(yaml_path)?;
    let mut out = BTreeMap::new();
    if let Some(list) = raw.get("paradigms").and_then(|v| v.as_array()) {
        for p in list {
            if let Some(id) = p.get("id").and_then(|v| v.as_str()) {
                out.insert(id.to_string(), p.clone());
            }
        }
    }
    Ok(out)
}

/// 校验范式编排合法性，返回错误列表（空列表即合法）。
pub fn validate_paradigm(
    paradigm: &Value,
    atoms: &Value,
    available_validators: &[String],
) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    let pid = paradigm.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    // 1. 引用完整性：所有 atoms 必须在 atom.yaml 里存在
    if let Some(list) = paradigm.get("atoms").and_then(|v| v.as_array()) {
        for atom_name in list {
            let name = atom_name.as_str().unwrap_or_default();
            if atoms.get(name).is_none() {
                errors.push(format!(
                    "paradigm '{}': atom '{}' 不在 atom.yaml 中",
                    pid, name
                ));
            }
        }
        // 2. validators.func 必须在 VALIDATORS 里（只检查 facetor_independent 的 directions）
        for atom_name in list {
            let name = atom_name.as_str().unwrap_or_default();
            if let Some(atom_cfg) = atoms.get(name) {
                if let Some(directions) = atom_cfg.get("directions").and_then(|v| v.as_array()) {
                    for direction in directions {
                        if let Some(func_name) = direction.get("validator").and_then(|v| v.as_str()) {
                            if !available_validators.contains(&func_name.to_string()) {
                                errors.push(format!(
                                    "paradigm '{}': validator '{}' 不在 VALIDATORS 中",
                                    pid, func_name
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. single_round_explore 特殊检查：composition 必须是 parallel
    if pid == "single_round_explore" {
        let composition = paradigm.get("composition").and_then(|v| v.as_array()).cloned();
        match composition {
            None => errors.push(format!("paradigm '{}': composition 为空", pid)),
            Some(c) => {
                if c.is_empty() {
                    errors.push(format!("paradigm '{}': composition 为空", pid));
                } else {
                    let step = &c[0];
                    let has_parallel = step.get("parallel").is_some();
                    if !has_parallel {
                        errors.push(format!(
                            "paradigm '{}': composition 第一步必须是 parallel",
                            pid
                        ));
                    }
                }
            }
        }
    }
    errors
}

/// 加载所有范式，逐个校验，返回 {paradigm_id: [errors]}。
pub fn validate_all(
    atom_yaml: &Path,
    chain_yaml: &Path,
    available_validators: &[String],
) -> Result<BTreeMap<String, Vec<String>>, PyError> {
    let atoms = load_atoms(atom_yaml)?;
    let paradigms = load_paradigms(chain_yaml)?;
    let mut result = BTreeMap::new();
    for (pid, paradigm) in &paradigms {
        result.insert(pid.clone(), validate_paradigm(paradigm, &atoms, available_validators));
    }
    Ok(result)
}
