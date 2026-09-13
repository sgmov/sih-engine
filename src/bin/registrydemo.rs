//! registrydemo：腿五插件槽位演示 bin（SPEC-025 § 插件槽位协议 · 批
//! lease-mergeall-parallel · 簇C）。
//!
//! 在役面改造让位后批申报：sihmcp 分派面（src/mcpserver/server.rs）零触碰，
//! 本 bin 只演示 ToolRegistry 统一注册形——真件 HeartbeatProvider（在役
//! heartbeat 只读投影直委托，同参同出参）加 mock echo 加 mock fail 加
//! manifest 扫描读数。MCP 面接线改造（defs 加 dispatch 改走 registry）候
//! 腿五收口批。
//!
//! 零 LLM 零网络（SPEC-025 A9 承袭）：全调用为本地投影或纯 mock。

use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use sih_engine::tools_registry::{
    load_manifests, HeartbeatProvider, ToolError, ToolProvider, ToolRegistry,
};
use std::path::PathBuf;

/// mock：echo 回声件——args 包层原样回传（registrydemo 演示件）。
struct EchoProvider;

#[async_trait]
impl ToolProvider for EchoProvider {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "演示件：args 原样回传（registrydemo mock，非治理面）。"
    }

    fn input_schema(&self) -> Value {
        json!({
            "properties": {"msg": {"title": "Msg", "type": "string"}},
            "required": [],
            "type": "object",
        })
    }

    async fn call(&self, args: Value) -> Result<Value, ToolError> {
        Ok(json!({ "echoed": args }))
    }
}

/// mock：fail 失败件——恒 Execution 错（错误面读数演示）。
struct FailProvider;

#[async_trait]
impl ToolProvider for FailProvider {
    fn name(&self) -> &str {
        "fail"
    }

    fn description(&self) -> &str {
        "演示件：恒 Execution 错（registrydemo mock，非治理面）。"
    }

    fn input_schema(&self) -> Value {
        json!({ "properties": {}, "required": [], "type": "object" })
    }

    async fn call(&self, _args: Value) -> Result<Value, ToolError> {
        Err(ToolError::Execution {
            tool: "fail".to_string(),
            reason: "演示性失败：fail 件恒错".to_string(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let registry = ToolRegistry::new();
    // 真件先行：在役只读工具经统一注册形（A3 预览）。
    registry.register(Box::new(HeartbeatProvider))?;
    registry.register(Box::new(EchoProvider))?;
    registry.register(Box::new(FailProvider))?;

    println!("== registry list ==");
    for d in registry.list()? {
        println!("- {} :: {}", d.name, d.description);
        println!("  schema: {}", d.input_schema);
    }

    println!("\n== registry call ==");
    match registry.call("heartbeat", json!({})).await {
        Ok(v) => println!("heartbeat -> {}", serde_json::to_string_pretty(&v)?),
        Err(e) => println!("heartbeat -> err: {e}"),
    }
    let echoed = registry.call("echo", json!({ "msg": "registry demo" })).await?;
    println!("echo -> {echoed}");
    match registry.call("fail", json!({})).await {
        Ok(v) => println!("fail -> unexpected ok: {v}"),
        Err(e) => println!("fail -> err: {e}"),
    }
    match registry.call("nope", json!({})).await {
        Ok(v) => println!("nope -> unexpected ok: {v}"),
        Err(e) => println!("nope -> err: {e}"),
    }

    println!("\n== plugin manifest scan ==");
    let plugins_root = std::env::var("SIH_PLUGINS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("sih/plugins"));
    match load_manifests(&plugins_root) {
        Ok(loaded) => {
            if loaded.is_empty() {
                println!("({plugins_root:?} 下零插件子目录：空列绿态)");
            }
            for lp in loaded {
                let m = &lp.manifest;
                let tools: Vec<&str> = m.tools.iter().map(|t| t.name.as_str()).collect();
                println!(
                    "- {} v{} transport={} command={} tools={tools:?}",
                    m.name, m.version, m.transport, m.command
                );
                println!("  manifest: {}", lp.path.display());
            }
        }
        Err(e) => println!("manifest scan -> err: {e}"),
    }

    Ok(())
}
