//! stdio-auto 批端到端回归：stdio 自动开域与开域链 exclude 写入（pk-105）。
//! 正典：sih-engine/sih/state/plan/stdio-auto.md（SDD 载体）；用户 2026-09-17
//! 裁定「stdio应该是自动签发令牌，令牌最重要的目的是选定治理空间」。
//!
//! 三族：T1 exclude 幂等单元；T2 CLI bootstrap 集成（argv 分发 + exclude +
//! 中央登记册落工作区）；T3 stdio 首连自动开域端到端（登记册不落项目内）。

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

/// 隔离夹具工作区：resolve_root 标记双目录 + 真任务包模板 + scribe 软链。
/// 域根：git init 的临时项目。
struct Fixture {
    ws: PathBuf,
    dom: PathBuf,
}

impl Fixture {
    fn new(tag: &str) -> Self {
        let base = std::env::temp_dir().join(format!("stdio-auto-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let ws = base.join("ws");
        let dom = base.join("proj");
        std::fs::create_dir_all(ws.join("sih-engine/doc")).unwrap();
        std::fs::create_dir_all(ws.join("sih-tools/lease")).unwrap();
        std::fs::create_dir_all(ws.join("sih-tools/mcpline/ledger")).unwrap();
        std::fs::create_dir_all(ws.join("sih-engine/target/debug")).unwrap();
        // 中央模板与 scribe 二进制取自工作区根（主树与 worktree 两形通吃：
        // 自 manifest 向上走，首个带 sih-tools 的目录即工作区根）
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let mut wsroot = repo.clone();
        while !wsroot.join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md").is_file() {
            if !wsroot.pop() {
                panic!("工作区根未寻得（sih-tools 缺席）：{}", repo.display());
            }
        }
        let tpl = wsroot.join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md");
        std::fs::copy(tpl, ws.join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md")).unwrap();
        // scribe 主树二进制：软链（open_domain 步五硬依赖）
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
}

fn exclude_line_count(dom: &Path) -> usize {
    let p = dom.join(".git/info/exclude");
    std::fs::read_to_string(p)
        .map(|s| s.lines().filter(|l| l.trim_end() == "sih/").count())
        .unwrap_or(0)
}

fn registry_rows(ws: &Path) -> Vec<String> {
    let p = ws.join("sih-tools/mcpline/ledger/tokens.ndjson");
    std::fs::read_to_string(p)
        .map(|s| s.lines().filter(|l| !l.trim().is_empty()).map(str::to_string).collect())
        .unwrap_or_default()
}

/// T1：exclude 幂等——同一域根连开两次只留一行（pk-105 核心断言）。
#[test]
fn t1_flywheel_exclude_idempotent() {
    let f = Fixture::new("t1");
    let v1 = sih_engine::mcpserver::bootstrap::flywheel_exclude(&f.dom);
    let v2 = sih_engine::mcpserver::bootstrap::flywheel_exclude(&f.dom);
    assert_eq!(exclude_line_count(&f.dom), 1, "两次只留一行 sih/");
    assert!(v1["verified"].as_bool().unwrap(), "第一次即验证过：{v1}");
    assert!(v2["verified"].as_bool().unwrap(), "第二次幂等仍验证过：{v2}");
}

/// T2：CLI bootstrap 集成——argv 分发、全链落位、exclude 行、登记册在册。
#[test]
fn t2_bootstrap_cli_opens_domain_with_exclude() {
    let f = Fixture::new("t2");
    let out = Command::new(env!("CARGO_BIN_EXE_sihmcp"))
        .args(["bootstrap", f.dom.to_string_lossy().as_ref(), "--by", "t2-cli", "--token-id", "t2-cli-01"])
        .env_remove("SIH_ROOT")
        .current_dir(&f.ws)
        .output()
        .expect("spawn sihmcp bootstrap");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(out.status.success(), "bootstrap 退出码非零：{stdout}");
    assert!(f.dom.join("sih/domain.json").is_file(), "域声明卡缺席");
    assert!(f.dom.join("sih/README.md").is_file(), "域自述缺席");
    assert_eq!(exclude_line_count(&f.dom), 1, "exclude 行缺席或重复");
    assert!(stdout.contains("sih/event/trail"), "开域首笔链文件未见于出参");
    let rows = registry_rows(&f.ws);
    assert_eq!(rows.len(), 1, "中央登记册应恰一行：{rows:?}");
    assert!(rows[0].contains("t2-cli-01"), "登记册行应含所签 token_id");
    assert!(stdout.contains("\"flywheel_git_exclude\""), "出参应载隔离步判词");
}

/// T3：stdio 首连自动开域——未开域域根触发全链，中央登记册落工作区不落项目。
#[test]
fn t3_stdio_first_connect_auto_opens() {
    let f = Fixture::new("t3");
    let mut child = Command::new(env!("CARGO_BIN_EXE_sihmcp"))
        .env("SIH_TRANSPORT", "stdio")
        .env("SIH_ROOT", &f.dom)
        .env("SIH_MCPLINE_CODE_ROOT", &f.ws)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn sihmcp stdio");
    use std::io::Write;
    let stdin = child.stdin.as_mut().unwrap();
    stdin
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{},\"clientInfo\":{\"name\":\"t3\",\"version\":\"0\"}}}\n")
        .unwrap();
    stdin.flush().unwrap();
    // 自动开域在 serve 前承载：给子进程链（bootstrap 孙进程含 scribe 两跑）足够墙钟。
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if f.dom.join("sih/domain.json").is_file() && exclude_line_count(&f.dom) == 1 {
            break;
        }
        std::thread::sleep(Duration::from_millis(300));
    }
    assert!(f.dom.join("sih/domain.json").is_file(), "stdio 首连未自动开域");
    assert_eq!(exclude_line_count(&f.dom), 1, "自动开域应写 exclude 行");
    assert!(!f.dom.join("sih-tools").exists(), "中央登记册不得落进项目内（根解析陷阱回归钉）");
    assert_eq!(registry_rows(&f.ws).len(), 1, "登记册应落工作区且恰一行");
    let _ = child.kill();
    let _ = child.wait();
}

/// T3b：已开域幂等——stdio 二连零重开（domain.json 不被改写、登记册不增行）。
#[test]
fn t3b_stdio_second_connect_no_reopen() {
    let f = Fixture::new("t3b");
    // 先以 CLI 形开域
    let out = Command::new(env!("CARGO_BIN_EXE_sihmcp"))
        .args(["bootstrap", f.dom.to_string_lossy().as_ref(), "--by", "t3b-pre", "--token-id", "t3b-pre-01"])
        .env_remove("SIH_ROOT")
        .current_dir(&f.ws)
        .output()
        .unwrap();
    assert!(out.status.success());
    let decl_before = std::fs::read_to_string(f.dom.join("sih/domain.json")).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_sihmcp"))
        .env("SIH_TRANSPORT", "stdio")
        .env("SIH_ROOT", &f.dom)
        .env("SIH_MCPLINE_CODE_ROOT", &f.ws)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{},\"clientInfo\":{\"name\":\"t3b\",\"version\":\"0\"}}}\n").unwrap();
    stdin.flush().unwrap();
    std::thread::sleep(Duration::from_secs(3));
    let decl_after = std::fs::read_to_string(f.dom.join("sih/domain.json")).unwrap();
    assert_eq!(decl_before, decl_after, "已开域域根被重开（幂等破坏）");
    assert_eq!(registry_rows(&f.ws).len(), 1, "二连不得增登记册行");
    assert_eq!(exclude_line_count(&f.dom), 1);
    let _ = child.kill();
    let _ = child.wait();
}
