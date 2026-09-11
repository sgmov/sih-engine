//! beta 写面连接会话绑定：一对一生命周期（DES-014 第一节正典）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/writeface/session.py。一条
//! MCP stdio 连接对应恰好一个 lease 会话；身份不自报，server 进程自采正身件
//! 签发（identity verify 生产路径采集，本包唯一文件写位即正身报告 tempfile
//! 暂存，域外非台账非链非 git）；断开收约形零写轻收约有写全收约，失败显形
//! 不强拆。孤儿连接与僵尸会话由既有机械兜底承接，零新增清理机制。

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::passthrough::{
    lease_close_argv, lease_open_argv, parse_session_id, run_cli, LEASE_BIN_TIMEOUT,
};
use super::runtime::{code_root, resolve_root, run_readonly};
use super::runtime::RunOutcome;

const IDENTITY_TIMEOUT: f64 = 120.0;

/// 正身报告暂存：tempfile 域外落盘（本包唯一文件写位，非台账非链非 git；
/// 对等 session._write_temp_report，零写守卫白名单单点）。
fn write_temp_report(payload: &str) -> std::io::Result<PathBuf> {
    let name = format!(
        "mcpline-identity-{}.json",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, payload)?;
    Ok(path)
}

/// 一连接一会话：绑定态与写迹与正身件与收约形的进程内承载。
pub struct ConnectionSession {
    pub root: PathBuf,
    pub agent_class: String,
    pub connection_id: String,
    pub id_component: String,
    pub identity_report: Option<PathBuf>,
    pub identity_error: Option<String>,
    pub session_id: Option<String>,
    pub package: Option<String>,
    pub write_count: usize,
    pub closed_by_client: bool,
    pub auto_open_error: Option<String>,
}

impl ConnectionSession {
    pub fn new(agent_class: &str) -> Self {
        let connection_id = format!(
            "mcpconn-{}",
            uuid_tail()
        );
        Self {
            root: resolve_root(),
            agent_class: agent_class.to_string(),
            id_component: connection_id.clone(),
            connection_id,
            identity_report: None,
            identity_error: None,
            session_id: None,
            package: None,
            write_count: 0,
            closed_by_client: false,
            auto_open_error: None,
        }
    }

    /// 进程正身签发：identity verify 生产路径采集，签发即采集一次。
    pub async fn ensure_identity(&mut self) -> Option<PathBuf> {
        if self.identity_report.is_some() || self.identity_error.is_some() {
            return self.identity_report.clone();
        }
        let argv = vec![
            "uv".to_string(),
            "run".to_string(),
            "--project".to_string(),
            code_root().join("sih-tools/identity").display().to_string(),
            "identity".to_string(),
            "verify".to_string(),
        ];
        let mut env_extra: Vec<(&str, &str)> = vec![("SIH_SESSION_ID", self.id_component.as_str())];
        let sandbox = std::env::var("SIH_SANDBOX_ID").unwrap_or_default();
        if !sandbox.is_empty() {
            env_extra.push(("SIH_SANDBOX_ID", sandbox.as_str()));
        }
        let out: RunOutcome =
            run_readonly(&argv, Some(Path::new("/")), Some(&env_extra), std::time::Duration::from_secs_f64(IDENTITY_TIMEOUT)).await;
        if out.rc != 0 {
            self.identity_error = Some(format!(
                "identity verify 退出码 {}: {}",
                out.rc,
                out.stderr.trim().chars().take(300).collect::<String>()
            ));
            return None;
        }
        let report: Value = match serde_json::from_str(&out.stdout) {
            Ok(v) => v,
            Err(e) => {
                self.identity_error = Some(format!("identity verify 出参非 JSON: {e}"));
                return None;
            }
        };
        let anomalies = report.get("anomalies").and_then(|a| a.as_array()).cloned().unwrap_or_default();
        if !anomalies.is_empty() {
            self.identity_error = Some("正身采集异常非空，拒签发（身份验既有语义）".to_string());
            return None;
        }
        match write_temp_report(&out.stdout) {
            Ok(p) => {
                self.identity_report = Some(p);
                self.identity_report.clone()
            }
            Err(e) => {
                self.identity_error = Some(format!("正身报告暂存失败: {e}"));
                None
            }
        }
    }

    /// 一对一绑定：多对一与重开即结构性拒（调用方先行裁决）。
    pub fn bind(&mut self, session_id: String, package: String) {
        self.session_id = Some(session_id);
        self.package = Some(package);
    }

    pub fn is_bound(&self) -> bool {
        self.session_id.is_some()
    }

    /// 写迹登记：收约轻重形分流依据。
    pub fn record_write(&mut self) {
        self.write_count += 1;
    }

    pub fn mark_client_closed(&mut self) {
        self.closed_by_client = true;
    }

    /// 连接级自动开（部署配置形）：SIH_MCPLINE_PACKAGE 与 SIH_MCPLINE_INTENT
    /// 齐备即连接建立时以本进程正身件调 lease open；失败不静默（stderr 显形）。
    pub async fn auto_open_from_config(&mut self) -> Option<String> {
        let package = std::env::var("SIH_MCPLINE_PACKAGE").unwrap_or_default();
        let intent = std::env::var("SIH_MCPLINE_INTENT").unwrap_or_default();
        let package = package.trim().to_string();
        let intent = intent.trim().to_string();
        if package.is_empty() || intent.is_empty() {
            return None;
        }
        let repos: Vec<String> = split_env_list("SIH_MCPLINE_REPO");
        let allow: Vec<String> = split_env_list("SIH_MCPLINE_ALLOW");
        let report = self.ensure_identity().await;
        if report.is_none() {
            let err = self.identity_error.clone().unwrap_or_default();
            self.auto_open_error = Some(err.clone());
            return Some(err);
        }
        let report = report.unwrap();
        let argv = lease_open_argv(&self.root, &report, &package, &intent, &repos, &allow);
        let out = run_cli(&argv, LEASE_BIN_TIMEOUT).await;
        let sid = if out.rc == 0 { parse_session_id(&out.stdout) } else { None };
        match sid {
            Some(sid) => {
                self.bind(sid, package);
                None
            }
            None => {
                let msg = format!(
                    "连接级 lease open 退出码 {}: {}",
                    out.rc,
                    if out.stderr.is_empty() { &out.stdout } else { &out.stderr }
                        .trim()
                        .chars()
                        .take(300)
                        .collect::<String>()
                );
                eprintln!("[sihmcp] {msg}");
                self.auto_open_error = Some(msg.clone());
                Some(msg)
            }
        }
    }

    /// 断开收约形：零写轻收约，有写全收约，失败显形不强拆。
    pub async fn disconnect_close(&mut self) -> Option<Value> {
        if !self.is_bound() || self.closed_by_client || self.package.is_none() {
            return None;
        }
        let package = self.package.clone().unwrap_or_default();
        let (light_trail, form) = if self.write_count == 0 {
            (Some(self.default_trail_for(&self.connection_id)), "light")
        } else {
            (None, "full")
        };
        let argv = lease_close_argv(&self.root, &package, None, light_trail.as_deref());
        let out = run_cli(&argv, LEASE_BIN_TIMEOUT).await;
        if out.rc != 0 {
            eprintln!(
                "[sihmcp] 收约未成即会话转挂起候人节点 takeover 裁决（server 不强拆不静默弃置）：exit={} {}",
                out.rc,
                if out.stderr.is_empty() { &out.stdout } else { &out.stderr }
                    .trim()
                    .chars()
                    .take(400)
                    .collect::<String>()
            );
        }
        Some(json!({
            "exit_code": out.rc,
            "form": form,
            "stdout": out.stdout,
            "stderr": out.stderr,
        }))
    }

    fn default_trail_for(&self, name: &str) -> PathBuf {
        self.root
            .join("sih-engine/sih/event/trail")
            .join(format!("{name}.ndjson"))
    }

    /// 正身暂存件清理（tempfile 域外，非治理面）。
    pub fn cleanup(&mut self) {
        if let Some(p) = self.identity_report.take() {
            let _ = std::fs::remove_file(p);
        }
    }
}

fn uuid_tail() -> String {
    // 连接派生值：十六位十六进制尾（对等 uuid4().hex[:12] 长度量级，进程内
    // 唯一即可，不做跨进程唯一承诺——会话唯一性归 lease 层）。
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id() as u128;
    format!("{:012x}", (nanos ^ (pid << 64)) & 0xffff_ffff_ffff)
}

fn split_env_list(key: &str) -> Vec<String> {
    // 对等 os.pathsep 分隔（macOS 与 Linux 俱 ':'），非路径分隔符。
    std::env::var(key)
        .unwrap_or_default()
        .split(':')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

/// 连接构造：根解析与分级定形。
pub fn make_connection(agent_class: &str) -> ConnectionSession {
    ConnectionSession::new(agent_class)
}
