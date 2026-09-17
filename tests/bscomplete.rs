//! bscomplete 批回归：开域补全链 --complete（pk-107 承载）。
//! 正典：sih-engine/sih/state/plan/bscomplete.md（SDD 载体）；语义对表围堰
//! sih-tools/mcpline/src/mcpline/bootstrap.py run_chain complete 分支
//! （DES-015 修订五，只读对表零改动）。
//!
//! 四件：健康域补全 in_place 零域状态写；缺镜像行补写恒等（appended）；
//! 域链缺席如实拒（退出码 1，无链不静默修复）；未开域给旗标无害走全链。

use std::path::PathBuf;
use std::process::Command;

struct Fixture {
    ws: PathBuf,
    dom: PathBuf,
}

impl Fixture {
    fn new(tag: &str) -> Self {
        let base = std::env::temp_dir().join(format!("bscomplete-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let ws = base.join("ws");
        let dom = base.join("proj");
        std::fs::create_dir_all(ws.join("sih-engine/doc")).unwrap();
        std::fs::create_dir_all(ws.join("sih-tools/lease")).unwrap();
        std::fs::create_dir_all(ws.join("sih-tools/mcpline/ledger")).unwrap();
        std::fs::create_dir_all(ws.join("sih-engine/target/debug")).unwrap();
        let mut wsroot = std::env::current_dir().unwrap().join("../../..").canonicalize().unwrap();
        while !wsroot.join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md").is_file() {
            if !wsroot.pop() {
                panic!("工作区根未寻得：{}", wsroot.display());
            }
        }
        std::fs::copy(
            wsroot.join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md"),
            ws.join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md"),
        ).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(
            wsroot.join("sih-engine/target/debug/scribe"),
            ws.join("sih-engine/target/debug/scribe"),
        )
        .unwrap();
        std::fs::create_dir_all(&dom).unwrap();
        assert!(Command::new("git").arg("init").arg("-q").arg(&dom).status().unwrap().success());
        Self { ws, dom }
    }

    /// 先开域（正常全链），返回 stdout 判词。
    fn open(&self, token: &str) -> String {
        let out = Command::new(env!("CARGO_BIN_EXE_sihmcp"))
            .args(["bootstrap", self.dom.to_string_lossy().as_ref(), "--by", "pre", "--token-id", token])
            .env_remove("SIH_ROOT")
            .current_dir(&self.ws)
            .output()
            .expect("spawn bootstrap");
        assert!(out.status.success(), "预开域失败：{}", String::from_utf8_lossy(&out.stdout));
        String::from_utf8_lossy(&out.stdout).to_string()
    }

    fn complete(&self) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_sihmcp"))
            .args(["bootstrap", self.dom.to_string_lossy().as_ref(), "--by", "complete-run", "--complete"])
            .env_remove("SIH_ROOT")
            .current_dir(&self.ws)
            .output()
            .expect("spawn bootstrap --complete")
    }
}

/// T1：健康域补全——in_place、零域状态写（domain.json 逐字节不变）、验域 valid。
#[test]
fn t1_complete_healthy_domain_in_place_zero_write() {
    let f = Fixture::new("t1");
    f.open("bc-t1-01");
    let decl_before = std::fs::read(f.dom.join("sih/domain.json")).unwrap();
    let out = f.complete();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(out.status.success(), "补全退出码非零：{stdout}");
    assert!(stdout.contains("\"in_place\""), "健康域应 in_place：{stdout}");
    assert!(stdout.contains("\"chain_verify\": \"valid\""), "验域应 valid：{stdout}");
    assert!(!stdout.contains("open_pen_hash"), "补全链出参不得含 open_pen_hash");
    assert!(!stdout.contains("flywheel_git_exclude"), "补全链出参不得含 flywheel_git_exclude");
    let decl_after = std::fs::read(f.dom.join("sih/domain.json")).unwrap();
    assert_eq!(decl_before, decl_after, "补全链零域状态写被破坏（domain.json 变更）");
}

/// T2：缺镜像行——补写中央 active 行恒等行（appended），补全后行数与中央对齐。
#[test]
fn t2_complete_missing_mirror_row_appended() {
    let f = Fixture::new("t2");
    f.open("bc-t2-01");
    let mirror = f.dom.join("sih/ledger/tokens.ndjson");
    assert!(mirror.is_file(), "开域链应已落镜像");
    std::fs::remove_file(&mirror).unwrap();
    let out = f.complete();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(out.status.success(), "补全退出码非零：{stdout}");
    assert!(stdout.contains("\"appended\""), "缺镜像应 appended：{stdout}");
    let mirror_text = std::fs::read_to_string(&mirror).unwrap();
    let rows: Vec<&str> = mirror_text.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(rows.len(), 1, "镜像应恰一行：{rows:?}");
    let central_text = std::fs::read_to_string(f.ws.join("sih-tools/mcpline/ledger/tokens.ndjson")).unwrap();
    let central_rows: Vec<&str> = central_text.lines().filter(|l| !l.trim().is_empty()).collect();
    let central_last: serde_json::Value = serde_json::from_str(central_rows.last().unwrap()).unwrap();
    let mirror_last: serde_json::Value = serde_json::from_str(rows.last().unwrap()).unwrap();
    assert_eq!(central_last, mirror_last, "镜像末行须与中央 active 行恒等");
    // 补全幂等：再跑一次应 in_place
    let out2 = f.complete();
    let stdout2 = String::from_utf8_lossy(&out2.stdout).to_string();
    assert!(stdout2.contains("\"in_place\""), "二次补全应 in_place：{stdout2}");
}

/// T3：域链缺席（半态即声明在链无）——如实拒，退出码 1，不静默造链。
#[test]
fn t3_complete_penless_half_state_rejected() {
    let f = Fixture::new("t3");
    f.open("bc-t3-01");
    std::fs::remove_dir_all(f.dom.join("sih/event/trail")).unwrap();
    let out = f.complete();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert_eq!(out.status.code(), Some(2), "域链缺席应工具异常退出码 2（对表围堰 InitToolError 形）：{stdout}");
    assert!(stdout.contains("域链缺席"), "拒语应点名域链缺席：{stdout}");
    assert!(!f.dom.join("sih/event/trail").exists(), "不得静默造链");
}

/// T4：未开域给 --complete——旗标无害，走正常全链。
#[test]
fn t4_complete_on_unopened_is_harmless_full_chain() {
    let f = Fixture::new("t4");
    let out = Command::new(env!("CARGO_BIN_EXE_sihmcp"))
        .args(["bootstrap", f.dom.to_string_lossy().as_ref(), "--by", "t4", "--token-id", "bc-t4-01", "--complete"])
        .env_remove("SIH_ROOT")
        .current_dir(&f.ws)
        .output()
        .expect("spawn");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(out.status.success(), "未开域加旗标应走全链：{stdout}");
    assert!(f.dom.join("sih/domain.json").is_file(), "域应已开");
    assert!(stdout.contains("open_pen_hash"), "全链出参应含开域首笔哈希");
}
