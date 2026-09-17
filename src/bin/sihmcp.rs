//! sihmcp 二进制入口：MCP 线 Rust 载体（sihmcp-solo 批）。
//!
//! stdio 与 streamable HTTP 双传输（SIH_TRANSPORT=stdio|http，HTTP 绑
//! 127.0.0.1:8765 /mcp，SIH_HTTP_BIND 与 SIH_HTTP_ENDPOINT 可覆写）。工具注册面与投影逻辑在
//! sih_engine::mcpserver（DEC-001 源码位：MCP server 与内部模块的承载）。
//! 进程始即连接生：连接级自动开（部署配置形）于 serve 前承载；stdio 管道
//! 正常关闭即断开收约（零写轻收约有写全收约，失败显形不强拆）。
//! stdio 自动开域（stdio-auto 批，用户 2026-09-17 裁定「stdio应该是自动签发
//! 令牌，令牌最重要的目的是选定治理空间」）：未开域域根首连自动走 bootstrap
//! 正典全链；argv 形 `sihmcp bootstrap <域根> --by <事由>` 为其子进程载体
//! （正典：sih_engine::mcpserver::bootstrap::run_cli）。

use rmcp::ServiceExt;
use sih_engine::mcpserver::server::SihMcpServer;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // argv 分发先于传输形：bootstrap 子命令是 stdio 自动开域的子进程载体，
    // 也是文档既载的 CLI 形（此前未接线，本批接线）。
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if matches!(argv.first().map(String::as_str), Some("bootstrap") | Some("mcpline.bootstrap")) {
        let code = sih_engine::mcpserver::bootstrap::run_cli(&argv)
            .await
            .unwrap_or(2);
        std::process::exit(code);
    }
    let transport = std::env::var("SIH_TRANSPORT")
        .unwrap_or_else(|_| "stdio".to_string())
        .trim()
        .to_ascii_lowercase();
    match transport.as_str() {
        "stdio" => {
            // stdio 自动开域（令牌即域绑定锚点，自动签发，不走管理台仪式）。
            if let Some(dom) = stdio_auto_domain() {
                stdio_auto_bootstrap(&dom);
            }
            let server = SihMcpServer::new();
            let conn = server.conn.clone();
            {
                let mut guard = conn.lock().await;
                let err = guard.auto_open_from_config().await;
                if err.is_none() && !guard.is_bound() {
                    // 部署未配置连接级自动开：连接转未绑形态，客户端经 lease_open 显式立
                    eprintln!(
                        "[sihmcp] 连接未绑定会话（部署未配置自动开或配置不齐）：写操作前先调 lease_open"
                    );
                }
            }
            let service = server
                .serve((tokio::io::stdin(), tokio::io::stdout()))
                .await?;
            service.waiting().await?;
            // 断开收约形（DES-014 第一节）：零写轻收约，有写全收约，失败显形。
            let mut guard = conn.lock().await;
            let _ = guard.disconnect_close().await;
            guard.cleanup();
            Ok(())
        }
        "http" => {
            let bind = std::env::var("SIH_HTTP_BIND")
                .unwrap_or_else(|_| "127.0.0.1:8765".to_string());
            let endpoint =
                std::env::var("SIH_HTTP_ENDPOINT").unwrap_or_else(|_| "/mcp".to_string());
            sih_engine::mcpserver::httpface::serve_http(&bind, &endpoint).await
        }
        other => anyhow::bail!(
            "unsupported SIH_TRANSPORT value `{other}`; expected `stdio` or `http`"
        ),
    }
}

/// stdio 自动开域判定：SIH_ROOT 指向未开域的 git 域根（.git 在位且
/// sih/domain.json 缺席）即触发。已开域幂等跳过，非 git 根零触碰
/// （避免给任意目录种域）。
fn stdio_auto_domain() -> Option<PathBuf> {
    let env = std::env::var("SIH_ROOT").ok()?;
    let env = env.trim().to_string();
    if env.is_empty() {
        return None;
    }
    let dom = PathBuf::from(&env);
    if dom.join(".git").is_dir() && !dom.join("sih/domain.json").is_file() {
        Some(dom)
    } else {
        None
    }
}

/// stdio 自动签发词形：自域根确定性派生（basename 净化 + 路径哈希四 hex），
/// 重试稳定（同根同 id）；已有 active 行时 bootstrap 侧复用旧牌、本 id 忽略。
/// 词形循 tokens::TOKEN_ID_RE（字母数字起首，字母数字_.- 至多 64 字符）。
fn stdio_token_id(dom: &std::path::Path) -> String {
    use std::hash::{Hash, Hasher};
    let base: String = dom
        .file_name()
        .map(|s| {
            s.to_string_lossy()
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
                .take(24)
                .collect()
        })
        .filter(|s: &String| !s.is_empty())
        .unwrap_or_else(|| "root".to_string());
    let mut h = std::collections::hash_map::DefaultHasher::new();
    dom.to_string_lossy().hash(&mut h);
    format!("stdio-{base}-{:04x}", (h.finish() & 0xffff) as u16)
}

/// stdio 自动开域执行：以子进程跑正典 bootstrap 全链。必须子进程隔离——
/// runtime.resolve_root 把 SIH_ROOT 当中央工作区根，而 stdio 客户端以它指
/// 域根，同进程直调会把中央登记册落进项目内（adoption-guide 第六问同族病）；
/// 子进程 env 卸 SIH_ROOT、cwd 取 code_root()，根解析自然正确。失败如实
/// stderr，以未开域形态继续（读面教学照常），不静默不阻断。
fn stdio_auto_bootstrap(dom: &std::path::Path) {
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("sihmcp"));
    let ws = sih_engine::mcpserver::runtime::code_root();
    let token_id = stdio_token_id(dom);
    let out = std::process::Command::new(&exe)
        .args([
            "bootstrap",
            dom.to_string_lossy().as_ref(),
            "--by",
            "stdio-auto",
            "--token-id",
            token_id.as_str(),
        ])
        .env_remove("SIH_ROOT")
        .current_dir(&ws)
        .output();
    match out {
        Ok(o) if o.status.success() => eprintln!(
            "[sihmcp] stdio 自动开域完成：{}（令牌自动签发即域绑定锚点；sih/ 已写入 .git/info/exclude）",
            dom.display()
        ),
        Ok(o) => {
            let stdout_s = String::from_utf8_lossy(&o.stdout);
            let stderr_s = String::from_utf8_lossy(&o.stderr);
            let payload = if stdout_s.trim().is_empty() { stderr_s } else { stdout_s };
            eprintln!(
                "[sihmcp] stdio 自动开域未成：{}（{}）——以未开域形态继续，读面教学可用；可手动跑 sihmcp bootstrap <域根> --by <事由>",
                dom.display(),
                payload.trim().chars().take(300).collect::<String>()
            );
        }
        Err(e) => eprintln!(
            "[sihmcp] stdio 自动开域子进程不可起：{e}——以未开域形态继续"
        ),
    }
}
