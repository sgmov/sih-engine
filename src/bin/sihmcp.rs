//! sihmcp 二进制入口：MCP 线 Rust 载体（sihmcp-solo 批段1）。
//!
//! stdio 传输；HTTP streamable 面随段3 落地。工具注册面与投影逻辑在
//! sih_engine::mcpserver（DEC-001 源码位：MCP server 与内部模块的承载）。

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
            let service = SihMcpServer
                .serve((tokio::io::stdin(), tokio::io::stdout()))
                .await?;
            service.waiting().await?;
            Ok(())
        }
        other => anyhow::bail!(
            "unsupported SIH_TRANSPORT value `{other}`; expected `stdio`（HTTP streamable 面随 sihmcp-solo 段3 落地）"
        ),
    }
}
