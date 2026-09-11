//! 运行时层：工作区根解析与承接 CLI 只读子进程封装。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/runtime.py。红线：本层零写入
//! ——只调任务包列明的现成只读 CLI 形，无状态、零重试改写、零判定语义。

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{json, Value};

pub const CANON_SPEC_023: &str = "sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md";
pub const CANON_LINE_PKG: &str = "sih-engine/sih/state/plan/mcpline-line-v1.md";

/// 工作区根解析：环境变量 SIH_ROOT 优先，缺省自当前目录向上找标记
/// （sih-engine/doc 与 sih-tools 同现即根），找不到回落当前目录。
pub fn resolve_root() -> PathBuf {
    if let Ok(env) = std::env::var("SIH_ROOT") {
        if !env.trim().is_empty() {
            return PathBuf::from(env);
        }
    }
    let mut cur = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        if cur.join("sih-engine/doc").is_dir() && cur.join("sih-tools").is_dir() {
            return cur;
        }
        if !cur.pop() {
            break;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// CLI 代码根解析：环境变量 SIH_MCPLINE_CODE_ROOT 优先，缺省与数据根同源。
pub fn code_root() -> PathBuf {
    if let Ok(env) = std::env::var("SIH_MCPLINE_CODE_ROOT") {
        if !env.trim().is_empty() {
            return PathBuf::from(env);
        }
    }
    resolve_root()
}

/// 实日（本地日期，YYYY-MM-DD）。
pub fn today_str() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// 日期形校验：YYYY-MM-DD 字面形（镜像 DATE_RE，零正则依赖）。
pub fn valid_date(date: &str) -> bool {
    let b = date.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return false;
    }
    b.iter().enumerate().all(|(i, c)| {
        if i == 4 || i == 7 {
            true
        } else {
            c.is_ascii_digit()
        }
    })
}

/// 错误载荷四字段：报错即教学。
pub fn error_payload(_tool: &str, what: &str, params: &[&str], message: &str) -> Value {
    json!({
        "error": message,
        "what_this_tool_does": what,
        "valid_params": params,
        "canonical_pointers": [CANON_SPEC_023, CANON_LINE_PKG],
    })
}

/// 只读子进程结果：(returncode, stdout, stderr) 三值对等形。
pub struct RunOutcome {
    pub rc: i32,
    pub stdout: String,
    pub stderr: String,
}

/// 只读子进程封装：捕获 stdout/stderr，超时与 spawn 失败收编为非零退出码形。
/// 零重试零改写。环境按 BATCH-FACE 坑位剥 PYTHONHOME 与 PYTHONPATH 与
/// VIRTUAL_ENV 污染后按需覆写（runtime._clean_env 对等）。
pub async fn run_readonly(
    argv: &[String],
    cwd: Option<&Path>,
    env_extra: Option<&[(&str, &str)]>,
    timeout: Duration,
) -> RunOutcome {
    let mut cmd = tokio::process::Command::new(&argv[0]);
    cmd.args(&argv[1..])
        .env_remove("PYTHONHOME")
        .env_remove("PYTHONPATH")
        .env_remove("VIRTUAL_ENV");
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    if let Some(extra) = env_extra {
        cmd.envs(extra.iter().copied());
    }
    let child = cmd.stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).stdin(std::process::Stdio::null()).spawn();
    let child = match child {
        Ok(c) => c,
        Err(e) => {
            return RunOutcome { rc: 127, stdout: String::new(), stderr: format!("{e}") };
        }
    };
    match tokio::time::timeout(timeout, child.wait_with_output()).await {
        Ok(Ok(out)) => RunOutcome {
            rc: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
        },
        Ok(Err(e)) => RunOutcome { rc: 127, stdout: String::new(), stderr: format!("{e}") },
        Err(_) => RunOutcome { rc: 124, stdout: String::new(), stderr: "subprocess timeout".to_string() },
    }
}

/// 布局判别：first_domain / canonical / None（critsweep detect_layout 先例）。
pub fn detect_layout_form(root: &Path) -> Option<&'static str> {
    if root.join("sih-engine/Cargo.toml").is_file()
        && root.join("sih-tools/pyproject.toml").is_file()
    {
        return Some("first_domain");
    }
    if root.join("sih/ledger").is_dir() {
        return Some("canonical");
    }
    None
}

/// 读面缺省分支的链路径解析：canonical 域根命中域内 sih/event/trail；
/// first_domain 与未知形回落第一域历史形 sih-engine/sih/event/trail。
pub fn trail_path(root: &Path, date: &str) -> PathBuf {
    if detect_layout_form(root) == Some("canonical") {
        root.join("sih/event/trail").join(format!("{date}.ndjson"))
    } else {
        root.join("sih-engine/sih/event/trail").join(format!("{date}.ndjson"))
    }
}

/// scribe 二进制位：debug 与 release 兜底序，两档俱缺回落 debug 位。
pub fn scribe_bin() -> PathBuf {
    let base = code_root().join("sih-engine/target");
    let debug = base.join("debug/scribe");
    let release = base.join("release/scribe");
    if debug.is_file() {
        debug
    } else if release.is_file() {
        release
    } else {
        debug
    }
}

/// 温故二进制位：主树 target/debug 位（debug 固定，对等 retriever_bin）。
pub fn retriever_bin() -> PathBuf {
    code_root().join("sih-engine/target/debug/retriever")
}
