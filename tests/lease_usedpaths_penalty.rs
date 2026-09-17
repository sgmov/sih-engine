//! pk-074 承 faceprecise-analysis.md §2.5：close 收约未用锁罚口径精确化集成测试。
//! 四条实装的面：used_paths 改源实际改动文件集（git diff --name-only
//! <base>..<branch>，归并前在会话工地取样）、比对形改前缀覆盖匹配（watchcheck
//! covered 同形）、集合运算 rstrip('/') 归一去重、git 取数失败回退 allow 近似
//! 口径且罚单 detail 带 used_source=allow_fallback 不静默；事件 schema 既有
//! 字段不删不改义只增 used_source。
//! harness 形承 tests/lease_cascadeclose_projection.rs（临时域双仓 + lease
//! 二进制三台账显式全传 + --trail 缺席即链证与 SDDG 双 skip 最短收约路径）。

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

const STEM: &str = "usedpaths";

fn git(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let o = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git spawn");
    (
        o.status.success(),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

fn init_repo(dir: &Path) {
    let _ = Command::new("git").arg("init").arg("-b").arg("master").arg(dir).output();
    git(dir, &["config", "user.email", "t@t"]);
    git(dir, &["config", "user.name", "t"]);
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-m", "base"]);
}

struct Domain {
    root: PathBuf,
    ledger: PathBuf,
    locks: PathBuf,
}

fn make_domain(tag: &str) -> Domain {
    let root = std::env::temp_dir().join(format!("usedpaths-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let engine = root.join("sih-engine");
    std::fs::create_dir_all(engine.join("sih/state/plan")).unwrap();
    std::fs::create_dir_all(engine.join("sih/event/trail")).unwrap();
    std::fs::write(engine.join("README.md"), "base\n").unwrap();
    std::fs::write(
        engine.join(format!("sih/state/plan/{}.md", STEM)),
        "# usedpaths：pk-074 fixture 任务包\n\n## 请求写入 {#requested-writes}\n\n（本批零声明）\n",
    )
    .unwrap();
    init_repo(&engine);
    let tools = root.join("sih-tools");
    std::fs::create_dir_all(&tools).unwrap();
    std::fs::write(tools.join("placeholder.txt"), "base\n").unwrap();
    init_repo(&tools);
    let dir = root.join("sih-tools/lease/ledger");
    std::fs::create_dir_all(&dir).unwrap();
    let ledger = dir.join("sessions.ndjson");
    let locks = dir.join("locks.ndjson");
    let bills = dir.join("bills.ndjson");
    for p in [&ledger, &locks, &bills] {
        std::fs::write(p, "").unwrap();
    }
    Domain { root, ledger, locks }
}

fn run_lease(domain: &Domain, args: &[String]) -> (i32, String, String) {
    let base = [
        "--root".to_string(),
        domain.root.display().to_string(),
        "--ledger".to_string(),
        domain.ledger.display().to_string(),
        "--locks".to_string(),
        domain.locks.display().to_string(),
        "--bills".to_string(),
        domain.locks.parent().unwrap().join("bills.ndjson").display().to_string(),
    ];
    let o = Command::new(env!("CARGO_BIN_EXE_lease"))
        .args(args)
        .args(&base)
        .output()
        .expect("lease spawn");
    (
        o.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
    )
}

/// 开约（无链笔：close 走 --trail 缺席文件即链证与 SDDG 双 skip 最短路径），
/// allow 走 --allow 显式旗标（包档零声明），返回会话号。
fn open_session(domain: &Domain, tag: &str, allow: &[&str]) -> String {
    let identity_path = domain.root.join("identity.json");
    std::fs::write(
        &identity_path,
        json!({
            "anomalies": [],
            "identity": {
                "core_hash": "fixturecorehash0000000000000000000000000000000000000000000000000000000000",
                "file_sha256": format!("fixturefsha{}", tag),
                "identity_hash": "fixtureidentityhash00000000000000000000000000000000000000000000000000000"
            }
        })
        .to_string(),
    )
    .unwrap();
    let intent_path = domain.root.join(format!("intent-{}.json", tag));
    std::fs::write(&intent_path, json!({"anchors": []}).to_string()).unwrap();
    let mut args: Vec<String> = vec![
        "open".into(),
        "--package".into(),
        STEM.into(),
        "--at".into(),
        "2026-09-13T00:00:00+00:00".into(),
        "--identity".into(),
        identity_path.display().to_string(),
        "--intent".into(),
        intent_path.display().to_string(),
    ];
    for a in allow {
        args.push("--allow".into());
        args.push((*a).to_string());
    }
    args.push("--repo".into());
    args.push("sih-engine".into());
    let (code, out, err) = run_lease(domain, &args);
    assert_eq!(code, 0, "open: {} {}", out, err);
    serde_json::from_str::<Value>(&out).unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string()
}

/// 会话工地随批写：在 worktree（分支 msh/<STEM>）落件并提交，即实际改动集。
fn worktree_commit(domain: &Domain, files: &[(&str, &str)]) {
    let wt = domain
        .root
        .join(format!("worktrees/sih-engine/{}", STEM));
    for (p, text) in files {
        let full = wt.join(p);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(full, text).unwrap();
    }
    let (ok, _, err) = git(&wt, &["add", "-A"]);
    assert!(ok, "worktree add: {}", err);
    let (ok, _, err) = git(&wt, &["commit", "-m", "fixture write"]);
    assert!(ok, "worktree commit: {}", err);
}

fn lock_path(domain: &Domain, sid: &str, path: &str) {
    let (code, out, err) = run_lease(
        domain,
        &[
            "lock".into(),
            "--path".into(),
            path.to_string(),
            "--session".into(),
            sid.to_string(),
        ],
    );
    assert_eq!(code, 0, "lock {}: {} {}", path, out, err);
}

fn unlock_path(domain: &Domain, sid: &str, path: &str) {
    let (code, out, err) = run_lease(
        domain,
        &[
            "unlock".into(),
            "--path".into(),
            path.to_string(),
            "--session".into(),
            sid.to_string(),
        ],
    );
    assert_eq!(code, 0, "unlock {}: {} {}", path, out, err);
}

fn close(domain: &Domain) -> (i32, String, String) {
    // --trail 指向缺席文件：链证守门与 SDDG 双 skip（workspace_chainless），
    // 罚金路径不受闸影响，本件只验账单口径。
    run_lease(
        domain,
        &[
            "close".into(),
            "--package".into(),
            STEM.into(),
            "--trail".into(),
            domain
                .root
                .join("absent-trail.ndjson")
                .display()
                .to_string(),
        ],
    )
}

fn penalty_events(domain: &Domain) -> Vec<Value> {
    let bills = domain.locks.parent().unwrap().join("lockface-bills.ndjson");
    std::fs::read_to_string(&bills)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .filter(|r| r.get("event_type").and_then(|v| v.as_str()) == Some("unused_lock_penalty"))
        .collect()
}

/// 过罚退罚主路径（§2.2：52.4% 罚点实写过）：allow 只声明 B，锁 A 与 B，
/// 工地实写 A 与 B。旧口径 used≈allow 罚 A（A 不在声明集），新口径 used=diff
/// 查 A 实写在案即零罚。
#[test]
fn written_outside_allow_not_penalized() {
    let d = make_domain("writtenfree");
    let sid = open_session(&d, "writtenfree", &["sih-engine/src/b.rs"]);
    worktree_commit(
        &d,
        &[
            ("src/a.rs", "// fixture a\n"),
            ("src/b.rs", "// fixture b\n"),
        ],
    );
    lock_path(&d, &sid, "sih-engine/src/a.rs");
    lock_path(&d, &sid, "sih-engine/src/b.rs");
    unlock_path(&d, &sid, "sih-engine/src/a.rs");
    unlock_path(&d, &sid, "sih-engine/src/b.rs");
    let (code, out, err) = close(&d);
    assert_eq!(code, 0, "收约须过：{} {}", out, err);
    let penalties = penalty_events(&d);
    assert!(
        penalties.is_empty(),
        "实写路径（即使在 allow 外）不得罚：{:?}",
        penalties
    );
}

/// 漏罚转罚反向面（任务反向例：声明 A 锁 A 没写且不在 diff → 罚）：allow 声明
/// A，锁 A，分支零提交（diff 空）。新口径 A 锁未用即罚，used_source=worktree_diff。
#[test]
fn unwritten_within_allow_penalized() {
    let d = make_domain("unwritten");
    let sid = open_session(&d, "unwritten", &["sih-engine/src/a.rs"]);
    lock_path(&d, &sid, "sih-engine/src/a.rs");
    unlock_path(&d, &sid, "sih-engine/src/a.rs");
    let (code, out, err) = close(&d);
    assert_eq!(code, 0, "收约须过：{} {}", out, err);
    let penalties = penalty_events(&d);
    assert_eq!(penalties.len(), 1, "锁而未写即罚：{:?}", penalties);
    let p = &penalties[0];
    assert_eq!(
        p["detail"]["unused_paths"],
        json!(["sih-engine/src/a.rs"])
    );
    assert_eq!(p["detail"]["used_source"], json!("worktree_diff"));
    assert_eq!(p["unused_count"], json!(1));
    assert_eq!(p["bill_points"], json!(1));
    assert_eq!(p["event_type"], json!("unused_lock_penalty"));
}

/// §2.3 类三钉（「锁了且在 allow 内但无写证——精确口径下应转罚」）：allow 声明
/// A 与 B、锁 A、工地实写仅 B。A 锁未用（不在 diff）即罚——allow 全集不再是
/// used 近似面，声明而未写不豁免。
#[test]
fn declared_but_unwritten_lock_penalized_despite_other_declared_write() {
    let d = make_domain("declaredunwritten");
    let sid = open_session(
        &d,
        "declaredunwritten",
        &["sih-engine/src/a.rs", "sih-engine/src/b.rs"],
    );
    worktree_commit(&d, &[("src/b.rs", "// fixture b\n")]);
    lock_path(&d, &sid, "sih-engine/src/a.rs");
    unlock_path(&d, &sid, "sih-engine/src/a.rs");
    let (code, out, err) = close(&d);
    assert_eq!(code, 0, "收约须过：{} {}", out, err);
    let penalties = penalty_events(&d);
    assert_eq!(penalties.len(), 1, "声明而未写不豁免：{:?}", penalties);
    assert_eq!(
        penalties[0]["detail"]["unused_paths"],
        json!(["sih-engine/src/a.rs"])
    );
    assert_eq!(penalties[0]["detail"]["used_source"], json!("worktree_diff"));
}

/// 前缀覆盖（§2.5 其二，watchcheck covered 同形）：锁 dir/ 与 dir/other.txt，
/// 工地实写 dir/file.txt。目录锁 dir/ 为祖先目录命中即算用（不罚），未写件
/// other.txt 照罚。
#[test]
fn dir_lock_prefix_cover_matches_diff_file() {
    let d = make_domain("prefixcover");
    let sid = open_session(&d, "prefixcover", &["sih-engine/dir/"]);
    worktree_commit(&d, &[("dir/file.txt", "fixture\n")]);
    lock_path(&d, &sid, "sih-engine/dir/");
    lock_path(&d, &sid, "sih-engine/dir/other.txt");
    unlock_path(&d, &sid, "sih-engine/dir/");
    unlock_path(&d, &sid, "sih-engine/dir/other.txt");
    let (code, out, err) = close(&d);
    assert_eq!(code, 0, "收约须过：{} {}", out, err);
    let penalties = penalty_events(&d);
    assert_eq!(penalties.len(), 1, "{:?}", penalties);
    assert_eq!(
        penalties[0]["detail"]["unused_paths"],
        json!(["sih-engine/dir/other.txt"]),
        "目录锁经前缀覆盖算用，唯未写件入罚"
    );
    assert_eq!(penalties[0]["detail"]["used_source"], json!("worktree_diff"));
}

/// 斜杠归一（§2.5 其三）：同路径带/不带尾斜杠两锁归一去重，未用差分只计
/// 一笔（旧形字符串全等两笔重复计罚）；归一形罚路径不带尾斜杠。
#[test]
fn slash_variants_dedup_in_unused_differential() {
    let d = make_domain("slashdedup");
    let sid = open_session(&d, "slashdedup", &[]);
    lock_path(&d, &sid, "sih-engine/src/dup.rs");
    lock_path(&d, &sid, "sih-engine/src/dup.rs/");
    unlock_path(&d, &sid, "sih-engine/src/dup.rs");
    unlock_path(&d, &sid, "sih-engine/src/dup.rs/");
    let (code, out, err) = close(&d);
    assert_eq!(code, 0, "收约须过：{} {}", out, err);
    let penalties = penalty_events(&d);
    assert_eq!(penalties.len(), 1, "斜杠变体归一后单笔罚单：{:?}", penalties);
    assert_eq!(
        penalties[0]["detail"]["unused_paths"],
        json!(["sih-engine/src/dup.rs"]),
        "归一形去重且剥尾斜杠"
    );
    assert_eq!(penalties[0]["unused_count"], json!(1), "不重复计罚");
    assert_eq!(penalties[0]["bill_points"], json!(1));
}

/// 回退路径（§2.5 其一备选）：repo 指向非 git 目录，git diff 取数失败整体
/// 回退旧 allow 近似口径，罚单 detail 带 used_source=allow_fallback 不静默；
/// 回退面下比对形仍前缀覆盖（allow 声明的 B 豁免其锁，未声明未写的 A 入罚）。
#[test]
fn nongit_repo_falls_back_to_allow_approximation() {
    let d = make_domain("fallback");
    // 非 git 工地：plain 目录（有文件无 .git）
    let nongit = d.root.join("nongit");
    std::fs::create_dir_all(&nongit).unwrap();
    std::fs::write(nongit.join("scratch.txt"), "not a repo\n").unwrap();
    // 手工会话行：open 需真 git 仓，此处直落 issued 形（worktree 指缺席路径，
    // 卫生检查 missing 即洁，归并走 missing 分支）
    let sid = "fallbacksid00000001";
    let issued = json!({
        "allow": ["fb/B"],
        "event": "issued",
        "issued_at": "2026-09-13T00:00:00+00:00",
        "package": STEM,
        "repos": [{
            "base_branch": "master",
            "branch": "msh/usedpaths",
            "repo": nongit.display().to_string(),
            "worktree": d.root.join("absent-worktree").display().to_string(),
        }],
        "session_id": sid,
    });
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(&d.ledger)
        .unwrap();
    writeln!(f, "{}", issued).unwrap();
    lock_path(&d, sid, "fb/A");
    lock_path(&d, sid, "fb/B");
    unlock_path(&d, sid, "fb/A");
    unlock_path(&d, sid, "fb/B");
    let (code, out, err) = close(&d);
    assert_eq!(code, 0, "非 git 工地收约须过（回退非拒）：{} {}", out, err);
    let penalties = penalty_events(&d);
    assert_eq!(penalties.len(), 1, "{:?}", penalties);
    assert_eq!(
        penalties[0]["detail"]["used_source"],
        json!("allow_fallback"),
        "git 取数失败如实标注不静默"
    );
    assert_eq!(
        penalties[0]["detail"]["unused_paths"],
        json!(["fb/A"]),
        "回退近似口径：allow 内 B 豁免，未声明未写 A 入罚"
    );
}
