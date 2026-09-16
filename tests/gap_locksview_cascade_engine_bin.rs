//! gap-locksview-cascade-engine-bin：locksview 乐观级联腿缺省兄弟位直调真引擎 cascade bin 回归。
//!
//! 承 locksview.rs 落差六承载位变更（run_cascade_check 已改直调引擎 cascade bin，
//! CASCADE_BIN_OVERRIDE 仅测试缝）。既有 mergeall_t6_locksview t3 只以假 cascade
//! shim 经覆盖位注入，本件补缺省解析序正路径：不设覆盖位，经本 bin 同目录兄弟位
//! 解析取真引擎 cascade bin，全链 bin-to-bin——真建册、真链上认证哈希对表、真
//! 判词（clean→writable、dirty→blocked、unverified→writable、不在册→unknown_target）
//! 与 checked 行留痕。fixture 全 temp 自建，零真实账本写入。

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin_locksview() -> &'static str {
    env!("CARGO_BIN_EXE_locksview")
}

fn bin_cascade() -> &'static str {
    env!("CARGO_BIN_EXE_cascade")
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

struct Fx {
    root: PathBuf,
    _guard: tempfile::TempDir,
}

fn write(p: &Path, s: &str) {
    if let Some(d) = p.parent() {
        fs::create_dir_all(d).unwrap();
    }
    fs::write(p, s).unwrap();
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().join("ws");
    let doc = root.join("sih-engine/doc");
    // 引用词形 TOKEN_RE（PRO-101 三位数字）定路：目标三件各引一件上游，
    // DES-202 引无认证史上游 PRO-103，余引 PRO-101/PRO-102。
    write(&doc.join("PRO-101-clean.md"), "上游洁净件。\n");
    write(&doc.join("PRO-102-dirty.md"), "上游偏离件。\n");
    write(&doc.join("PRO-103-nocert.md"), "上游无认证史件。\n");
    write(&doc.join("DES-201-clean.md"), "洁净目标，依赖 PRO-101。\n");
    write(&doc.join("DES-202-unv.md"), "无认证史目标，依赖 PRO-103。\n");
    write(&doc.join("DES-203-dirty.md"), "脏目标，依赖 PRO-102。\n");

    // 真引擎 cascade bin 建册（locksview 缺省 registry 位）
    let registry = root.join("sih-engine/doc/CASCADE.json");
    let out = Command::new(bin_cascade())
        .args(["build", "--root", &doc.display().to_string(), "--out", &registry.display().to_string()])
        .output()
        .expect("cascade spawn");
    assert!(out.status.success(), "cascade build 败：{}", String::from_utf8_lossy(&out.stderr));

    // 链上认证：DES-201 带真哈希（clean），DES-203 带错哈希（dirty）；
    // DES-202 的上游 PRO-103 无认证（unverified）。报告落 locksview 缺省
    // reports_root=<root>/sih-tools。
    let clean_abs = fs::canonicalize(doc.join("PRO-101-clean.md")).unwrap();
    let clean_hash = sha256_hex(fs::read(&clean_abs).unwrap().as_slice());
    write(
        &root.join("sih-tools/r-clean.json"),
        &json!({"content_hashes": {clean_abs.display().to_string(): clean_hash}}).to_string(),
    );
    let dirty_abs = fs::canonicalize(doc.join("PRO-102-dirty.md")).unwrap();
    write(
        &root.join("sih-tools/r-dirty.json"),
        &json!({"content_hashes": {dirty_abs.display().to_string(): "f".repeat(64)}}).to_string(),
    );
    let trail = root.join("sih-engine/sih/event/trail/2026-09-13.ndjson");
    let ev = |doc_id: &str, report: &str| {
        json!({
            "event_id": format!("e-{doc_id}"),
            "event_type": "certification_completed",
            "report_type": "scrutinator",
            "doc_id": doc_id,
            "report": report,
        })
        .to_string()
    };
    write(
        &trail,
        &format!(
            "{}\n{}\n",
            ev("DES-201-clean", "r-clean.json"),
            ev("DES-203-dirty", "r-dirty.json")
        ),
    );
    Fx { root, _guard: guard }
}

impl Fx {
    /// locksview check：只传 --path --root --locks，registry/doc-root/trails/
    /// reports-root 与 cascade bin 全走缺省解析（兄弟位真引擎 bin）。
    fn check(&self, rel: &str) -> (Option<i32>, Value) {
        let out = Command::new(bin_locksview())
            .args([
                "check",
                "--path",
                &format!("sih-engine/doc/{rel}"),
                "--root",
                &self.root.display().to_string(),
                "--locks",
                &self.root.join("locks.ndjson").display().to_string(),
            ])
            .env_remove("CASCADE_BIN_OVERRIDE")
            .output()
            .unwrap();
        let v: Value =
            serde_json::from_slice(&out.stdout).expect("locksview check 出参须 JSON");
        (out.status.code(), v)
    }

    fn locks_lines(&self) -> Vec<String> {
        fs::read_to_string(self.root.join("locks.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|s| s.to_string())
            .collect()
    }
}

#[test]
fn t1_real_engine_cascade_bin_full_chain_verdicts() {
    let fx = build_fx();

    // clean：真哈希在档 → writable（退出码零）
    let (code, v) = fx.check("DES-201-clean.md");
    assert_eq!(code, Some(0), "clean 判词可写：{v}");
    assert_eq!(v["verdict"], "writable");
    assert_eq!(v["dirty"].as_array().unwrap().len(), 0);

    // dirty：错哈希偏离 → blocked（退出码一，dirty 清单点名 PRO-102-dirty.md）
    let (code, v) = fx.check("DES-203-dirty.md");
    assert_eq!(code, Some(1), "dirty 判词拦：{v}");
    assert_eq!(v["verdict"], "blocked");
    assert_eq!(v["error"], "dirty_upstream");
    assert_eq!(v["dirty"][0], "PRO-102-dirty.md");

    // unverified：上游无认证史 → 非脏即 writable（unverified 面不拦，判词消费面语义）
    let (code, v) = fx.check("DES-202-unv.md");
    assert_eq!(code, Some(0), "unverified 判词可写：{v}");
    assert_eq!(v["verdict"], "writable");

    // 不在册：unknown_target 过注记（退出码零）
    let (code, v) = fx.check("GHOST-999-missing.md");
    assert_eq!(code, Some(0), "不在册目标退出码零：{v}");
    assert_eq!(v["verdict"], "unknown_target");

    // checked 行留痕：四笔 checked 逐笔入沙箱锁台账
    let lines = fx.locks_lines();
    assert_eq!(lines.len(), 4, "四笔 checked 留痕：{lines:?}");
    assert!(lines.iter().all(|l| l.contains("\"event\":\"checked\"")));
}

/// 覆盖位空串即视为未设：缺省兄弟位解析仍取真引擎 cascade bin（解析序语义）。
#[test]
fn t2_empty_override_falls_through_to_sibling_bin() {
    let fx = build_fx();
    let out = Command::new(bin_locksview())
        .args([
            "check",
            "--path",
            "sih-engine/doc/DES-201-clean.md",
            "--root",
            &fx.root.display().to_string(),
            "--locks",
            &fx.root.join("locks.ndjson").display().to_string(),
        ])
        .env("CASCADE_BIN_OVERRIDE", "")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "空串覆盖位须回落兄弟位：{}", String::from_utf8_lossy(&out.stdout));
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["verdict"], "writable");
}
