//! defectwave 批缺陷五回归：attnanchor 域覆盖台账不做 last-row-wins，撤牌域
//! 告警不灭。
//! 病灶申报出处：2026-09-18 侦察批会话档案（sih-engine 侧候令簿）——
//! sih-tools/mcpline/ledger/tokens.ndjson 中 sim-aesthetic-workbench 两行：
//! 2026-09-10 active 行 + 2026-09-17T21:44:10 stopped 行（sihmcp-console 撤牌
//! 笔）。last-row-wins 语义下该域应已停止覆盖，但 src/bin/attnanchor.rs
//! registry_domains 逐行扫、凡 status=="active" 即收域根，撤牌后该域每次提交
//! 日仍报「越限告警查无此人」（该域无 sih/event/trail，用自有 voyage 账；
//! COVERAGE_WINDOW_DAYS=7）。
//! 修法：registry_domains 改 last-row-wins——按域（domain_root 即覆盖单元）
//! 取每域末行 status，仅最终态 active 的域入覆盖面；单行域不变；探测域行
//!（/private/tmp 探针）行为不变；stopped→active 反转域须仍覆盖（防「见
//! stopped 即弃」过杀形）。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

fn git(dir: &Path, args: &[&str]) {
    let o = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git spawn");
    assert!(
        o.status.success(),
        "git {:?}: {}",
        args,
        String::from_utf8_lossy(&o.stderr)
    );
}

/// 假工作区根：只承载 tokens 台账与各域 git 仓（attnanchor 其余行降级不阻）。
fn make_ws(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("dwanchor-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("sih-tools/mcpline/ledger")).unwrap();
    root
}

/// 域 git 仓：今日两笔提交、无 sih/event/trail（covered 恒 false，覆盖域必告警）。
fn make_domain_repo(path: &Path) {
    std::fs::create_dir_all(path).unwrap();
    let _ = Command::new("git").arg("init").arg("-b").arg("master").arg(path).output();
    git(path, &["config", "user.email", "t@t"]);
    git(path, &["config", "user.name", "t"]);
    git(path, &["commit", "--allow-empty", "-m", "d1"]);
    git(path, &["commit", "--allow-empty", "-m", "d2"]);
}

fn token_row(token: &str, domain_root: &Path, status: &str) -> Value {
    json!({
        "domain_root": domain_root.display().to_string(),
        "issued_at": "2026-09-18T00:00:00+08:00",
        "issued_by": "defectwave-fixture",
        "scope": "domain_write",
        "status": status,
        "token_id": token,
    })
}

fn write_ledger(root: &Path, rows: &[Value]) {
    let text = rows
        .iter()
        .map(|r| serde_json::to_string(r).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(
        root.join("sih-tools/mcpline/ledger/tokens.ndjson"),
        text + "\n",
    )
    .unwrap();
}

/// 跑 attnanchor（--root 显式传假工作区根，--at 传今日），回 additionalContext 全文。
fn run_anchor(root: &Path) -> String {
    let at = String::from_utf8(
        Command::new("date")
            .arg("+%F")
            .output()
            .expect("date spawn")
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();
    let out = Command::new(env!("CARGO_BIN_EXE_attnanchor"))
        .args(["--root", root.to_str().unwrap(), "--at", &at])
        .output()
        .expect("attnanchor spawn");
    assert!(
        out.status.success(),
        "attnanchor 退出码恒零承围堰契约: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: Value = serde_json::from_str(&String::from_utf8_lossy(&out.stdout)).unwrap();
    v["additionalContext"].as_str().unwrap_or("").to_string()
}

fn alarm_count(text: &str, name: &str) -> usize {
    text.matches(&format!("越限告警：{} ", name)).count()
}

/// 撤牌域（active 后 stopped）不入覆盖面：零告警（修复前红：撤牌后仍报
/// 「越限告警查无某人」即病灶本体）。
#[test]
fn defectwave_revoked_domain_excluded_from_coverage() {
    let root = make_ws("revoked");
    let dom = root.join("domain-revoked");
    make_domain_repo(&dom);
    write_ledger(
        &root,
        &[
            token_row("tok-revoked", &dom, "active"),
            token_row("tok-revoked", &dom, "stopped"),
        ],
    );
    let text = run_anchor(&root);
    assert_eq!(
        alarm_count(&text, "domain-revoked"),
        0,
        "撤牌域（末行 stopped）不得入覆盖面告警：{}",
        text
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// 单 active 行域照旧入覆盖面（既有行为对齐，兼作告警机器在证控制组）。
#[test]
fn defectwave_single_active_domain_still_covered() {
    let root = make_ws("single");
    let dom = root.join("domain-single");
    make_domain_repo(&dom);
    write_ledger(&root, &[token_row("tok-single", &dom, "active")]);
    let text = run_anchor(&root);
    assert!(
        alarm_count(&text, "domain-single") >= 1,
        "单 active 行域照旧入覆盖面：{}",
        text
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// 多域混合取各域末行：active→stopped 域零告警；单 active 域告警；
/// stopped→active 反转域仍告警（last-row-wins 双向钉，防见 stopped 即弃过杀）。
#[test]
fn defectwave_mixed_domains_last_row_wins_per_domain() {
    let root = make_ws("mixed");
    let dom_c = root.join("domain-c-stopped");
    let dom_d = root.join("domain-d-active");
    let dom_e = root.join("domain-e-revived");
    make_domain_repo(&dom_c);
    make_domain_repo(&dom_d);
    make_domain_repo(&dom_e);
    write_ledger(
        &root,
        &[
            token_row("tok-c", &dom_c, "active"),
            token_row("tok-c", &dom_c, "stopped"),
            token_row("tok-d", &dom_d, "active"),
            token_row("tok-e", &dom_e, "stopped"),
            token_row("tok-e", &dom_e, "active"),
        ],
    );
    let text = run_anchor(&root);
    assert_eq!(
        alarm_count(&text, "domain-c-stopped"),
        0,
        "末行 stopped 域不入覆盖面：{}",
        text
    );
    assert!(
        alarm_count(&text, "domain-d-active") >= 1,
        "末行 active 域入覆盖面：{}",
        text
    );
    assert!(
        alarm_count(&text, "domain-e-revived") >= 1,
        "stopped→active 反转域末行 active 仍覆盖（last-row-wins 双向）：{}",
        text
    );
    let _ = std::fs::remove_dir_all(&root);
}
