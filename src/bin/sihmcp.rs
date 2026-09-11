//! sihmcp 二进制入口：MCP 线 Rust 载体（sihmcp-solo 批）。
//!
//! stdio 与 streamable HTTP 双传输（SIH_TRANSPORT=stdio|http，HTTP 绑
//! 127.0.0.1:8765 /mcp，SIH_HTTP_BIND 与 SIH_HTTP_ENDPOINT 可覆写）。工具注册面与投影逻辑在
//! sih_engine::mcpserver（DEC-001 源码位：MCP server 与内部模块的承载）。
//! 进程始即连接生：连接级自动开（部署配置形）于 serve 前承载；stdio 管道
//! 正常关闭即断开收约（零写轻收约有写全收约，失败显形不强拆）。

use rmcp::ServiceExt;
use sih_engine::mcpserver::server::SihMcpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let transport = std::env::var("SIH_TRANSPORT")
        .unwrap_or_else(|_| "stdio".to_string())
        .trim()
        .to_ascii_lowercase();
    match transport.as_str() {
        "stdio" => {
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
