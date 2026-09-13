//! 附件腿：域上下文推导加 stem 查册闸全查加甲表核验加守卫安装拆卸。
//!
//! 对表面：围堰 core.detect_domain_context 与 core.check_stem_gate 与
//! core._verify_new_stem_claim（甲表机械核对 pk-090 件五）与
//! commitcore.install_hooks/uninstall_hooks（DEC-011 修订四接锁收口位）。
//! pk-103 收口：root 缺省自 cwd 上溯（canonical 标记先检域界即停，再检
//! first_domain 双仓标记），词典包路径随 root 锚定与 cwd 无关。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

use crate::commitlaw::py_resolve;
use crate::{die, emit, one};

pub(crate) const STEM_GATE_SEG_DROP: &[&str] = &["solo"];
pub(crate) const CLAIM_CODE_NONE: &str = "无承";
pub(crate) const CLAIM_TEACHING: &str = "甲表三件：概念锚 zh（--claim-zh）与既裁 code 形或显式申报无承（--claim-code，字面 无承 即显式申报无承）与语素派生（--claim-derivation，逗号分隔 段:形，形域 established 与 new）；序承先乙后名再甲：取名前先拉 nomenclator map --concept 语义映射报告入上下文（只报不判），看真材料取名后填甲表，闸做机械兜底（DEC-017 修订六）";

fn is_canonical_domain(path: &Path) -> bool {
    path.join("sih").join("ledger").is_dir()
}

fn is_first_domain(path: &Path) -> bool {
    path.join("sih-engine").join("Cargo.toml").is_file()
        && path.join("sih-tools").join("pyproject.toml").is_file()
}

/// 域上下文判定（core.detect_domain_context 对表）：显式 root 优先，缺省自
/// cwd 上溯；每候选先检 canonical 标记（域界即停不越界），再检 first_domain
/// 双仓标记；零命中回落 cwd。
pub(crate) fn detect_domain_context(explicit_root: Option<&str>) -> PathBuf {
    if let Some(r) = explicit_root {
        if !r.is_empty() {
            return py_resolve(&PathBuf::from(r));
        }
    }
    let cur = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut cur = py_resolve(&cur);
    loop {
        if is_canonical_domain(&cur) || is_first_domain(&cur) {
            return cur;
        }
        match cur.parent() {
            Some(p) if p != cur => cur = p.to_path_buf(),
            _ => break,
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn nomenclator_project_root() -> PathBuf {
    // nomenclator uv 项目位：lease 仓的兄弟目录（代码根解析，core.
    // _nomenclator_project_root 对表——本二进制代码根即工作区根）。
    detect_domain_context(None).join("sih-tools").join("nomenclator")
}

fn nomenclator_query(word: &str, root: &Path) -> Result<Value, String> {
    // 只读子进程（闸是消费方，工具本体零改动）；工具缺席或异常即 fail-closed。
    let proj = nomenclator_project_root();
    if !proj.is_dir() {
        return Err(format!("stem 查册闸 fail-closed：nomenclator 项目位缺席：{}", proj.display()));
    }
    let nom_pack = root.join("sih-tools").join("nomenclator").join("packs").join("core");
    let out = Command::new("uv")
        .args([
            "run",
            "--project",
            proj.to_str().unwrap_or("."),
            "nomenclator",
            "query",
            "--pack",
            nom_pack.to_str().unwrap_or("."),
            "--word",
            word,
        ])
        .current_dir(&proj)
        .output()
        .map_err(|e| format!("stem 查册闸 fail-closed：nomenclator query 异常：{e}"))?;
    if !out.status.success() {
        let detail = String::from_utf8_lossy(&out.stderr);
        let detail = detail.trim();
        let detail = if detail.is_empty() {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        } else {
            detail.to_string()
        };
        return Err(format!(
            "stem 查册闸 fail-closed：nomenclator query 退出码 {}：{}",
            out.status.code().unwrap_or(-1),
            &detail.chars().take(200).collect::<String>()
        ));
    }
    serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("stem 查册闸 fail-closed：nomenclator query 出参不可解析：{e}"))
}

pub(crate) fn parse_claim_derivation(raw: &str) -> Result<Vec<(String, String)>, String> {
    let mut entries = Vec::new();
    for part in raw.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (seg, kind) = part
            .split_once(':')
            .ok_or_else(|| "派生对表不圆：段:形 缺冒号".to_string())?;
        let seg = seg.trim();
        let kind = kind.trim();
        if seg.is_empty() || kind.is_empty() {
            return Err("派生对表不圆：段:形 有空段".to_string());
        }
        if kind != "established" && kind != "new" {
            return Err(format!(
                "派生对表不圆：表外段 {seg}；形域 established 与 new"
            ));
        }
        entries.push((seg.to_string(), kind.to_string()));
    }
    if entries.is_empty() {
        return Err("派生对表不圆：派生表空".to_string());
    }
    Ok(entries)
}

/// 甲表机械核对（pk-090 件五对表）：指称完整与派生对表，零 LLM 判词位。
fn verify_new_stem_claim(
    claim_zh: &str,
    claim_code: &str,
    claim_der: &str,
    verdicts: &[Value],
    root: &Path,
) -> Result<Value, String> {
    if claim_zh.trim().is_empty() && claim_code.trim().is_empty() && claim_der.trim().is_empty() {
        return Err(format!("stem 查册闸拒：--new-stem 裸认领：新词认领须带甲表；{CLAIM_TEACHING}"));
    }
    let mut missing = Vec::new();
    if claim_zh.trim().is_empty() {
        missing.push("--claim-zh");
    }
    if claim_code.trim().is_empty() {
        missing.push("--claim-code");
    }
    if claim_der.trim().is_empty() {
        missing.push("--claim-derivation");
    }
    if !missing.is_empty() {
        return Err(format!(
            "stem 查册闸拒：甲表填不圆：缺 {}；{CLAIM_TEACHING}",
            missing.join(", ")
        ));
    }
    let code = claim_code.trim();
    let mut code_verdict = "declared_none";
    if code != CLAIM_CODE_NONE {
        let state = nomenclator_query(code, root)?
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if state != "established" {
            return Err(format!(
                "stem 查册闸拒：甲表填不圆：既裁 code 形 {code:?} 非在册 established（词典 state={state}）：既裁形须在册，或显式申报 无承；{CLAIM_TEACHING}"
            ));
        }
        code_verdict = "established_ref";
    }
    let entries = parse_claim_derivation(claim_der)
        .map_err(|e| format!("stem 查册闸拒：甲表填不圆：{e}；{CLAIM_TEACHING}"))?;
    let seg_verdict: Vec<&Value> = verdicts.iter().collect();
    let mut missing_segs: Vec<String> = Vec::new();
    let mut extra_segs: Vec<String> = Vec::new();
    for v in &seg_verdict {
        let seg = v["segment"].as_str().unwrap_or("");
        if !entries.iter().any(|(s, _)| s == seg) {
            missing_segs.push(seg.to_string());
        }
    }
    for (s, _) in &entries {
        if !seg_verdict.iter().any(|v| v["segment"].as_str() == Some(s.as_str())) {
            extra_segs.push(s.clone());
        }
    }
    missing_segs.sort();
    extra_segs.sort();
    if !missing_segs.is_empty() || !extra_segs.is_empty() {
        let mut detail = Vec::new();
        if !missing_segs.is_empty() {
            detail.push(format!("缺段 {}", missing_segs.join(", ")));
        }
        if !extra_segs.is_empty() {
            detail.push(format!("表外段 {}", extra_segs.join(", ")));
        }
        return Err(format!(
            "stem 查册闸拒：甲表填不圆：派生对表不圆：{}；{CLAIM_TEACHING}",
            detail.join("；")
        ));
    }
    let mut derivation = Vec::new();
    for v in &seg_verdict {
        let seg = v["segment"].as_str().unwrap_or("");
        let kind = entries
            .iter()
            .find(|(s, _)| s == seg)
            .map(|(_, k)| k.as_str())
            .unwrap_or("");
        let actual = if v["verdict"] == "pass" { "established" } else { "unknown" };
        if kind != actual {
            return Err(format!(
                "stem 查册闸拒：甲表填不圆：派生对表不圆：段 {seg} 申报 {kind} 而查册实态 {actual}；{CLAIM_TEACHING}"
            ));
        }
        derivation.push(json!({"segment": seg, "kind": kind, "verified_state": actual}));
    }
    Ok(json!({
        "zh": claim_zh.trim(),
        "code": code,
        "code_verdict": code_verdict,
        "derivation": derivation,
    }))
}

/// stem 查册闸（core.check_stem_gate 对表，pk-103 全查路收口）。
pub(crate) fn stem_check_full(
    root: &Path,
    stem: &str,
    new_stem: bool,
    claim_zh: &str,
    claim_code: &str,
    claim_der: &str,
) -> Value {
    let first = root.join("sih-engine").is_dir() && root.join("sih-tools").is_dir();
    if !first {
        return json!({
            "checked": true, "domain_scope": "canonical", "segments": [],
            "disposition": "external_teaching",
            "teaching": "新城正典域：词典在中央（司衡工作区 sih-tools/nomenclator），查册自愿走 nomenclator query，新词立名走司衡立名程序；本域零代强制（DES-016）"
        });
    }
    let pack = root.join("sih-tools/nomenclator/packs/core");
    if !pack.join("envelope.json").is_file() {
        return json!({
            "checked": false, "domain_scope": "first", "segments": [],
            "disposition": "pack_absent_skip",
            "teaching": format!("词典包缺席（{}）：本域命名查册未执行，回执显形不遮掩", pack.display())
        });
    }
    let segs: Vec<&str> = stem
        .split(|c| c == '-' || c == '_' || c == '.')
        .filter(|s| !s.is_empty())
        .filter(|s| !STEM_GATE_SEG_DROP.contains(&s.to_lowercase().as_str()))
        .filter(|s| s.chars().any(|c| !c.is_ascii_digit()))
        .collect();
    let mut verdicts: Vec<Value> = Vec::new();
    let mut collisions: Vec<String> = Vec::new();
    let mut unknowns: Vec<String> = Vec::new();
    for seg in &segs {
        let state = match nomenclator_query(seg, root) {
            Ok(v) => v.get("state").and_then(|s| s.as_str()).unwrap_or("unknown").to_string(),
            Err(e) => die(2, &e, json!({"word": seg})),
        };
        if state == "established" {
            verdicts.push(json!({"segment": seg, "state": state, "verdict": "pass"}));
        } else if state.starts_with("dead") {
            verdicts.push(json!({"segment": seg, "state": state, "verdict": "reject_dead"}));
            collisions.push(format!("{seg}={state}"));
        } else if state == "lazy" || state == "candidate" {
            verdicts.push(json!({"segment": seg, "state": state, "verdict": format!("reject_{state}")}));
            collisions.push(format!("{seg}={state}"));
        } else {
            verdicts.push(json!({"segment": seg, "state": state, "verdict": "new_coinage"}));
            unknowns.push(seg.to_string());
        }
    }
    if !collisions.is_empty() {
        die(
            2,
            &format!(
                "stem 查册闸拒：任务包名与词典既有条目相撞：{}；处置：另择名（dead 翻案须显式推翻原死因；lazy 升格走 nomenclator register；candidate 呈报既有条目阻断重推）",
                collisions.join("; ")
            ),
            json!({"segments": segs}),
        );
    }
    if !unknowns.is_empty() && !new_stem {
        die(
            2,
            &format!(
                "stem 查册闸拒：任务包名含未立名新词：{}；处置：走司衡立名程序后 nomenclator register 在册，或 --new-stem 显式认领新词（认领即声明，入回执可追）",
                unknowns.join(", ")
            ),
            json!({"segments": segs}),
        );
    }
    let disposition = if segs.is_empty() {
        "no_segments"
    } else if !unknowns.is_empty() {
        "new_coinage_acknowledged"
    } else {
        "established_pass"
    };
    let mut result = json!({
        "checked": true,
        "domain_scope": "first",
        "segments": verdicts,
        "disposition": disposition,
    });
    if !unknowns.is_empty() {
        result["teaching"] = json!(format!("新词认领在案：{}", unknowns.join(", ")));
        match verify_new_stem_claim(claim_zh, claim_code, claim_der, result["segments"].as_array().unwrap(), root) {
            Ok(receipt) => {
                result["new_coinage_claim"] = receipt;
            }
            Err(e) => die(2, &e, json!({"stem": stem})),
        }
    }
    result
}

/// 守卫安装（commitcore.install_hooks 对表）：向仓写 core.hooksPath 指仓内
/// hooks 目录，config 属环境态不入版控。
pub(crate) fn install_hooks(repos: &[String]) -> Vec<Value> {
    let hooks_dir = nomenclator_project_root()
        .parent()
        .map(|p| p.join("lease").join("hooks"))
        .unwrap_or_else(|| PathBuf::from("hooks"));
    let mut results = Vec::new();
    for repo in repos {
        let repo_path = py_resolve(&PathBuf::from(repo));
        let out = Command::new("git")
            .args(["config", "core.hooksPath", hooks_dir.to_str().unwrap_or("hooks")])
            .current_dir(&repo_path)
            .output();
        let (installed, detail) = match out {
            Ok(o) if o.status.success() => (true, None),
            Ok(o) => (
                false,
                Some(String::from_utf8_lossy(&o.stderr).trim().chars().take(200).collect::<String>()),
            ),
            Err(e) => (false, Some(e.to_string())),
        };
        results.push(json!({
            "hooks_path": hooks_dir.display().to_string(),
            "installed": installed,
            "repo": repo_path.display().to_string(),
            "detail": detail,
        }));
    }
    results
}

/// 守卫拆卸（commitcore.uninstall_hooks 对表）：unset 可逆，键缺席 rc5 计
/// already_absent 照常收敛。
pub(crate) fn uninstall_hooks(repos: &[String]) -> Vec<Value> {
    let mut results = Vec::new();
    for repo in repos {
        let repo_path = py_resolve(&PathBuf::from(repo));
        let out = Command::new("git")
            .args(["config", "--unset", "core.hooksPath"])
            .current_dir(&repo_path)
            .output();
        let rc = out.as_ref().ok().map(|o| o.status.code().unwrap_or(-1)).unwrap_or(-1);
        let state = if rc == 0 {
            "unset"
        } else if rc == 5 {
            "already_absent"
        } else {
            "failed"
        };
        let detail = if rc == 0 || rc == 5 {
            None
        } else {
            Some(
                out.map(|o| {
                    String::from_utf8_lossy(&o.stderr)
                        .trim()
                        .chars()
                        .take(200)
                        .collect::<String>()
                })
                .unwrap_or_else(|e| e.to_string()),
            )
        };
        results.push(json!({
            "repo": repo_path.display().to_string(),
            "uninstalled": rc == 0 || rc == 5,
            "state": state,
            "detail": detail,
        }));
    }
    results
}

pub(crate) fn cmd_install_hooks(m: &BTreeMap<String, Vec<String>>) -> ! {
    let root_flag = one(m, "root").map(|s| s.to_string()).filter(|s| !s.is_empty());
    let root = detect_domain_context(root_flag.as_deref());
    let repos: Vec<String> = m
        .get("repo")
        .cloned()
        .unwrap_or_else(|| {
            vec![
                root.join("sih-tools").display().to_string(),
                root.join("sih-engine").display().to_string(),
            ]
        });
    let results = install_hooks(&repos);
    let ok = results.iter().all(|r| r["installed"].as_bool().unwrap_or(false));
    print!("{}", emit(&json!({"hooks": results})));
    std::process::exit(if ok { 0 } else { 1 });
}

pub(crate) fn cmd_uninstall_hooks(m: &BTreeMap<String, Vec<String>>) -> ! {
    let root_flag = one(m, "root").map(|s| s.to_string()).filter(|s| !s.is_empty());
    let root = detect_domain_context(root_flag.as_deref());
    let repos: Vec<String> = m
        .get("repo")
        .cloned()
        .unwrap_or_else(|| {
            vec![
                root.join("sih-tools").display().to_string(),
                root.join("sih-engine").display().to_string(),
            ]
        });
    let results = uninstall_hooks(&repos);
    let ok = results.iter().all(|r| r["uninstalled"].as_bool().unwrap_or(false));
    print!("{}", emit(&json!({"hooks": results})));
    std::process::exit(if ok { 0 } else { 1 });
}
