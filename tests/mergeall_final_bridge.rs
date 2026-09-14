//! 收尾批：进程外插件桥四面实装回归锁（A4 转正）。
use sih_engine::tools_registry::{
    load_manifests, ExternalPluginBridge, StdioPluginBridge, ToolRegistry,
};
use std::path::PathBuf;

#[test]
fn t0_static_manifest_still_loads() {
    let manifests = load_manifests(&fixture_dir()).expect("静态 manifest 装载");
    assert_eq!(manifests.len(), 1);
}

fn fixture_dir() -> PathBuf {
    PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/plugins/bridge")
}

fn abs_manifest() -> sih_engine::tools_registry::PluginManifest {
    let cmd = fixture_dir().join("demo-ext").join("demo-plugin.sh");
    serde_json::from_value(serde_json::json!({
        "name": "demo-ext", "version": "0.1.0", "transport": "stdio",
        "command": cmd.to_str().unwrap(),
        "tools": [{"name": "ext_echo", "description": "回显入参（fixture 假插件）"}]
    })).expect("manifest")
}

#[tokio::test]
async fn t1_bridge_full_roundtrip() {
    let manifest = abs_manifest();
    let bridge = StdioPluginBridge::new();
    let registry = ToolRegistry::new();
    let out = bridge
        .register_plugin(&registry, &manifest)
        .await
        .expect("注册");
    assert_eq!(out["registered"][0]["name"], "ext_echo");
    let listed = registry.list().unwrap();
    assert!(listed.iter().any(|d| d.name == "ext_echo"), "tools/list 显形");
    let v = registry
        .call("ext_echo", serde_json::json!({"hello": "world"}))
        .await
        .expect("桥往返");
    assert_eq!(v["echo"], true, "假插件回显");
    let down = bridge.shutdown("demo-ext").await.expect("shutdown");
    assert_eq!(down["shutdown"], true);
}

#[tokio::test]
async fn t2_bridge_send_without_spawn_not_found() {
    let bridge = StdioPluginBridge::new();
    let err = bridge
        .send_request("ghost", serde_json::json!({"method": "call", "params": {}}))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("未启动"));
}

#[tokio::test]
async fn t3_bridge_recv_session_sentinel() {
    let manifest = abs_manifest();
    let bridge = StdioPluginBridge::new();
    bridge.spawn(&manifest).await.expect("spawn");
    let sent = bridge.recv("demo-ext").await.expect("recv 哨");
    assert_eq!(sent["state"], "session-open");
    bridge.shutdown("demo-ext").await.expect("shutdown");
    assert!(bridge.recv("demo-ext").await.is_err(), "shutdown 后不在场");
}
