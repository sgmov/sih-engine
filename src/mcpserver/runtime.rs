//! 运行时层：工作区根解析与承接 CLI 只读子进程封装。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/runtime.py。红线：本层零写入
//! ——只调任务包列明的现成只读 CLI 形，无状态、零重试改写、零判定语义。

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{json, Value};

pub const CANON_SPEC_023: &str = "sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md";
pub const CANON_LINE_PKG: &str = "sih-engine/sih/state/plan/mcpline-line-v1.md";

/// 工作区根解析：环境变量 SIH_ROOT 优先，缺省自当前目录向上找引擎仓标记
/// （sih-engine/Cargo.toml 即根），找不到回落当前目录。第一域双仓形由
/// legacy_first_domain_enabled 短路判定（SIH_LEGACY_FIRST_DOMAIN 显式控，
/// 未设时回落围堰存量 sihtools 标记检测作向后兼容）。
pub fn resolve_root() -> PathBuf {
    if let Ok(env) = std::env::var("SIH_ROOT") {
        if !env.trim().is_empty() {
            return PathBuf::from(env);
        }
    }
    let mut cur = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        if cur.join("sih-engine/Cargo.toml").is_file()
            && legacy_first_domain_enabled(&cur)
        {
            return cur;
        }
        if cur.join("sih-engine/Cargo.toml").is_file()
            && cur.join("sih/ledger").is_dir()
        {
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
/// 第一域检测短路于 SIH_LEGACY_FIRST_DOMAIN（env var 优先），未设时回落围堰
/// 存量 sihtools 标记检测作向后兼容（解耦绝拒后仍有 env 路径覆盖）。
pub fn detect_layout_form(root: &Path) -> Option<&'static str> {
    if root.join("sih-engine/Cargo.toml").is_file()
        && legacy_first_domain_enabled(root)
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

/// 引擎二进制位探针错误：debug 与 release 两档俱缺时返回（绝拒位——零静默
/// 回落，违背围堰退役批「缺即显形」纪律）。
#[derive(Debug)]
pub struct EngineBinError {
    pub name: String,
    pub debug: PathBuf,
    pub release: PathBuf,
}

impl std::fmt::Display for EngineBinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "engine binary `{}` 缺席：debug={} release={}（两档俱缺——cargo build \
             未产出或 target 目录错位；零静默回落，spawn 之前即显形拒）",
            self.name,
            self.debug.display(),
            self.release.display()
        )
    }
}

impl std::error::Error for EngineBinError {}

/// 引擎二进制位解析：debug 优先、release 次之、两档俱缺返回 Err（不静默
/// 回落不存在的路径——避免 spawn 127 与真错误混淆）。围堰 Python 工具线已冻
/// 结兼容只读（2026-09-14 围堰退役批），生产调用面全数切引擎 bin。
pub fn engine_bin(name: &str) -> Result<PathBuf, EngineBinError> {
    let base = code_root().join("sih-engine/target");
    let debug = base.join("debug").join(name);
    let release = base.join("release").join(name);
    if debug.is_file() {
        Ok(debug)
    } else if release.is_file() {
        Ok(release)
    } else {
        Err(EngineBinError {
            name: name.to_string(),
            debug,
            release,
        })
    }
}

/// 租约二进制位（lockface 已融回引擎，uv 裸名 spawn 失败 127 病灶随批根除）。
pub fn lease_bin() -> Result<PathBuf, EngineBinError> {
    engine_bin("lease")
}

/// 判据扫二进制位（lease-mergeall-parallel 簇A 融回件；围堰 sweep.py 实参
/// 形 --at <日期> --root <根> 对齐，行为对等基准 1.2.0）。
pub fn critsweep_bin() -> Result<PathBuf, EngineBinError> {
    engine_bin("critsweep")
}

/// 秤星二进制位（簇A 融回件；heartbeat read --dimension 三维快照入口；围堰
/// PYTHONPATH=src python3 -m gauge.cli 改 spawn 引擎 bin，cwd 锚解）。
pub fn gauge_bin() -> Result<PathBuf, EngineBinError> {
    engine_bin("gauge")
}

/// 检词二进制位（簇A 融回件；query/check/map 三子命令；围堰 uv run --project
/// 改 spawn 引擎 bin，--pack 由 SIH_NOMENCLATOR_PACK_DIR 强制配置）。
pub fn nomenclator_bin() -> Result<PathBuf, EngineBinError> {
    engine_bin("nomenclator")
}

/// 正身二进制位（簇E 融回件；围堰 uv run --project 改 spawn 引擎 bin，盐与声
/// 明对表语义同源）。
pub fn identity_bin() -> Result<PathBuf, EngineBinError> {
    engine_bin("identity")
}

/// 旧仓依赖显式拒：env var 缺位即返回绝拒位哨（绝对路径前缀，任何 join 后
/// 仍为非真实路径，触发后续 IO ENOENT 显形）。绝不静默降级为零对象、空串或
/// 当前目录——这些会让上游错把空当真，违背围堰退役批的「旧仓转冻结只读
/// 兼容态，生产调用面切零运行期依赖」纪律。
pub fn deny_path(env_var: &str, contract_anchor: &str) -> PathBuf {
    eprintln!(
        "[sihmcp] {env_var} 未配置：{contract_anchor} 已冻结兼容只读（旧仓依赖绝拒）。\
         绝拒位哨返回——任何后续 IO 会以 ENOENT 显形，非静默降级。"
    );
    PathBuf::from(format!(
        "/__sihmcp_deny__/{env_var}/unset-contract={}",
        contract_anchor.replace('/', "_")
    ))
}

/// 第一域（legacy 双仓布局）开关：env var 强制配置，缺位与非法值一律回落
/// 新城正典形（绝不允许任何运行期 sihtools 路径拼接——旧仓依赖绝拒）。
/// 生产部署方设 SIH_LEGACY_FIRST_DOMAIN=1 显式启用第一域，设 0 显式拒。
pub fn legacy_first_domain_enabled(_root: &Path) -> bool {
    match std::env::var("SIH_LEGACY_FIRST_DOMAIN") {
        Ok(v) => match v.trim() {
            "1" | "true" | "yes" => true,
            "0" | "false" | "no" => false,
            // 非法值显式回落新城正典形（旧仓路径永不拼接）
            _ => false,
        },
        // 缺位回落新城正典形（旧仓路径永不拼接）
        Err(_) => false,
    }
}

/// 测试共享 env var 守卫：进程级 env 互斥串行，离开作用域即还原。
/// 仅测试编译（`#[cfg(test)]`），生产代码零引入。
#[cfg(test)]
pub mod test_helpers {
    use std::sync::{Mutex, MutexGuard, OnceLock};

    /// 全模块互斥：env 是进程级共享量，跨测试串行防互踩。
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    fn env_lock() -> &'static Mutex<()> {
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    /// 临时设一组环境变量，Drop 时还原。env var 是相对域身份（迁移前既
    /// 存；迁移后由部署方自指）。本守卫覆写 env 段；作用域结束即还原，不
    /// 污染其他测试。
    pub struct EnvGuard {
        saved: Vec<(String, Option<String>)>,
        _lock: MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        pub fn set(vars: &[(&str, &str)]) -> Self {
            let lock = env_lock().lock().unwrap_or_else(|e| e.into_inner());
            let mut saved = Vec::with_capacity(vars.len());
            for (k, v) in vars {
                saved.push((k.to_string(), std::env::var(k).ok()));
                std::env::set_var(k, v);
            }
            Self { saved, _lock: lock }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, v) in self.saved.drain(..) {
                match v {
                    Some(val) => std::env::set_var(&k, val),
                    None => std::env::remove_var(&k),
                }
            }
        }
    }
}

