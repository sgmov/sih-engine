//! mergeall_t5_registry：腿五插件槽位架构验收测试（批
//! lease-mergeall-parallel · 簇C）。
//!
//! 对表 SPEC-025 § 插件槽位协议 加 § 验收判据 A3 加 A4：
//! - T1 trait 注册与调用 roundtrip（register/list/call）
//! - T2 registry 守卫面（未知 NotFound 加重名 DuplicateTool）
//! - T3 manifest 解析正常形（fixture 扫描加 inline parse）
//! - T4 manifest 非法形拒（语义校验加未知字段加 Io 缺席三态）加空列绿态
//! - T5 演示迁移：HeartbeatProvider 元数据与在役 MCP 面 defs 冻结同形加
//!   registry 接线（MCP 面零变契约位；call 直委托零变换，运行读数演示在
//!   registrydemo bin，测试不 spawn python3 以免环境依赖）
//! - T6 外部插件桥 A4 申报缺席形：四面恒 BridgeUnsupported

use async_trait::async_trait;
use serde_json::{json, Value};
use sih_engine::tools_registry::{
    load_manifests, ExternalPluginBridge, HeartbeatProvider, PluginManifest, ToolDescriptor,
    ToolError, ToolProvider, ToolRegistry,
};
use std::path::PathBuf;

fn fixtures_plugins() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/plugins")
}

// ------------------------------------------------------------------- mocks

/// mock：echo 回声件——args 包层原样回传。
struct EchoProvider;

#[async_trait]
impl ToolProvider for EchoProvider {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "测试 mock：args 原样回传。"
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

/// mock：upper 大写件——读入参返输出参。
struct UpperProvider;

#[async_trait]
impl ToolProvider for UpperProvider {
    fn name(&self) -> &str {
        "upper"
    }

    fn description(&self) -> &str {
        "测试 mock：msg 转大写。"
    }

    fn input_schema(&self) -> Value {
        json!({
            "properties": {"msg": {"title": "Msg", "type": "string"}},
            "required": ["msg"],
            "type": "object",
        })
    }

    async fn call(&self, args: Value) -> Result<Value, ToolError> {
        let msg = args
            .get("msg")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidArguments {
                tool: "upper".to_string(),
                reason: "msg 须为字符串".to_string(),
            })?;
        Ok(json!({ "upper": msg.to_uppercase() }))
    }
}

// -------------------------------------------------------------------- tests

/// T1：trait 注册与调用 roundtrip——register 加 list 加 call 三面。
#[tokio::test]
async fn t1_registry_register_list_call_roundtrip() {
    let reg = ToolRegistry::new();
    reg.register(Box::new(EchoProvider)).unwrap();
    reg.register(Box::new(UpperProvider)).unwrap();

    let list: Vec<ToolDescriptor> = reg.list().unwrap();
    let names: Vec<&str> = list.iter().map(|d| d.name.as_str()).collect();
    assert_eq!(names, vec!["echo", "upper"], "列单须注册序");

    assert_eq!(list[0].description, "测试 mock：args 原样回传。");
    assert_eq!(
        list[0].input_schema,
        json!({
            "properties": {"msg": {"title": "Msg", "type": "string"}},
            "required": [],
            "type": "object",
        })
    );

    let out = reg.call("echo", json!({ "msg": "hi" })).await.unwrap();
    assert_eq!(out, json!({ "echoed": { "msg": "hi" } }));

    let out = reg.call("upper", json!({ "msg": "abc" })).await.unwrap();
    assert_eq!(out, json!({ "upper": "ABC" }));

    // 调用面错误三态透传：provider 报 InvalidArguments，registry 零变换。
    match reg.call("upper", json!({})).await {
        Err(ToolError::InvalidArguments { tool, .. }) => assert_eq!(tool, "upper"),
        other => panic!("expected InvalidArguments, got {other:?}"),
    }
}

/// T2：registry 守卫面——未知名 NotFound 加重名 DuplicateTool。
#[tokio::test]
async fn t2_registry_rejects_unknown_and_duplicate() {
    let reg = ToolRegistry::new();
    reg.register(Box::new(EchoProvider)).unwrap();

    match reg.call("nope", json!({})).await {
        Err(ToolError::NotFound(n)) => assert_eq!(n, "nope"),
        other => panic!("expected NotFound, got {other:?}"),
    }

    match reg.register(Box::new(EchoProvider)) {
        Err(ToolError::DuplicateTool(n)) => assert_eq!(n, "echo"),
        other => panic!("expected DuplicateTool, got {other:?}"),
    }
    // 拒后列单不变。
    assert_eq!(reg.list().unwrap().len(), 1);
}

/// T3：manifest 正常形——fixture 扫描加 inline parse 两路。
#[test]
fn t3_manifest_parse_valid_fixture_and_inline() {
    // 扫描根 tests/fixtures/plugins/scan，其下 demo-ext/ 为插件目录。
    let loaded = load_manifests(&fixtures_plugins().join("scan")).unwrap();
    assert_eq!(loaded.len(), 1, "一插件子目录一清单");
    let lp = &loaded[0];
    assert!(lp.path.ends_with("demo-ext/plugin.json"));
    let m = &lp.manifest;
    assert_eq!(m.name, "demo-ext");
    assert_eq!(m.version, "0.1.0");
    assert_eq!(m.transport, "stdio");
    assert_eq!(m.command, "demo-plugin");
    assert_eq!(m.tools.len(), 2);
    assert_eq!(m.tools[0].name, "demo_greet");
    assert_eq!(m.tools[0].description, "演示外挂工具：greet。");
    assert_eq!(m.tools[1].name, "demo_ping");
    assert_eq!(m.tools[1].description, "", "description 可选缺省空串");

    // inline parse 同形（含可选 description 缺省）。
    let parsed = PluginManifest::parse(
        r#"{"name":"x","version":"1.2.3","transport":"stdio","command":["not", "a", "string"]}"#,
    );
    assert!(parsed.is_err(), "command 须为字符串，数组形拒");

    let ok = PluginManifest::parse(
        r#"{"name":"x","version":"1.2.3","transport":"stdio","command":"run-x","tools":[]}"#,
    )
    .unwrap();
    assert_eq!(ok.name, "x");
    assert!(ok.tools.is_empty(), "tools 空数组非非法形");
}

/// T4：manifest 非法形拒——语义校验加未知字段加 Io 缺席三态加空列绿态。
#[test]
fn t4_manifest_rejects_invalid_forms_and_io_absence() {
    // inline 非法形：transport 非 stdio 加必填键缺席加空 name 加空 command
    // 加未知字段加 tools 重名加 tools 空名加 name 路径分隔符。
    let bad_forms: [(&str, &str); 8] = [
        (
            r#"{"name":"x","version":"0.1.0","transport":"http","command":"c","tools":[]}"#,
            "transport 非 stdio",
        ),
        (
            r#"{"name":"x","transport":"stdio","command":"c","tools":[]}"#,
            "version 键缺席",
        ),
        (
            r#"{"version":"0.1.0","transport":"stdio","command":"c","tools":[]}"#,
            "name 键缺席",
        ),
        (
            r#"{"name":"","version":"0.1.0","transport":"stdio","command":"c","tools":[]}"#,
            "name 空串",
        ),
        (
            r#"{"name":"x","version":"0.1.0","transport":"stdio","command":" ","tools":[]}"#,
            "command 空白",
        ),
        (
            r#"{"name":"x","version":"0.1.0","transport":"stdio","command":"c","tools":[],"extra":1}"#,
            "未知字段拒",
        ),
        (
            r#"{"name":"x","version":"0.1.0","transport":"stdio","command":"c","tools":[{"name":"t"},{"name":"t"}]}"#,
            "tools 重名",
        ),
        (
            r#"{"name":"a/b","version":"0.1.0","transport":"stdio","command":"c","tools":[]}"#,
            "name 路径分隔符",
        ),
    ];
    for (raw, tag) in bad_forms {
        match PluginManifest::parse(raw) {
            Err(ToolError::ManifestInvalid { .. }) => {}
            other => panic!("{tag}: expected ManifestInvalid, got {other:?}"),
        }
    }

    // tools 条目空名。
    let bad_tools = PluginManifest::parse(
        r#"{"name":"x","version":"0.1.0","transport":"stdio","command":"c","tools":[{"name":" "}]}"#,
    );
    assert!(matches!(bad_tools, Err(ToolError::ManifestInvalid { .. })));

    // fixture 扫描非法形：transport http 件在扫描面被拒。
    match load_manifests(&fixtures_plugins().join("scan-invalid")) {
        Err(ToolError::ManifestInvalid { site, .. }) => {
            assert!(site.contains("bad-transport"), "site 载清单路径: {site}");
        }
        other => panic!("expected ManifestInvalid from fixture scan, got {other:?}"),
    }

    // 根目录缺席即 Io 错。
    match load_manifests(&fixtures_plugins().join("absent-root")) {
        Err(ToolError::ManifestIo { .. }) => {}
        other => panic!("expected ManifestIo for absent root, got {other:?}"),
    }

    // 根在而零插件子目录即空列绿态（.keep 文件非目录不参与）。
    let empty = load_manifests(&fixtures_plugins().join("scan-empty")).unwrap();
    assert!(empty.is_empty());

    // 子目录缺 plugin.json 即 Io 错（tempfile 临时形，dev-dep 在册）。
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir(tmp.path().join("bare-plugin")).unwrap();
    match load_manifests(tmp.path()) {
        Err(ToolError::ManifestIo { site, reason }) => {
            assert!(site.contains("bare-plugin"), "site 载清单路径: {site}");
            assert!(reason.contains("缺席"), "reason 申报缺席: {reason}");
        }
        other => panic!("expected ManifestIo for missing manifest, got {other:?}"),
    }
}

/// T5：演示迁移——HeartbeatProvider 元数据与在役 MCP 面 defs 冻结同形
/// （同名同描述同 schema，MCP 面零变契约位）加 registry 接线。
#[tokio::test]
async fn t5_heartbeat_migration_metadata_frozen_and_wired() {
    let p = HeartbeatProvider;
    assert_eq!(p.name(), "heartbeat");
    // 与 src/mcpserver/server.rs desc_heart（canon_pair 展开）逐字同形。
    assert_eq!(
        p.description(),
        "心跳：秤星三维最新读数与距上快照间隔日（只读不落链）。Heartbeat: latest gauge \
         tri-dimension readings and days since last snapshot (read-only). 承接 CLI：cd \
         sih-tools/gauge 且 PYTHONPATH=src python3 -m gauge.cli read。正典：\
         sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md；\
         sih-engine/sih/state/plan/mcpline-line-v1.md"
    );
    assert_eq!(
        p.input_schema(),
        json!({ "properties": {}, "required": [], "type": "object" })
    );

    // registry 接线：注册后列单可见，元数据经 ToolDescriptor 原样透出。
    let reg = ToolRegistry::new();
    reg.register(Box::new(HeartbeatProvider)).unwrap();
    let list = reg.list().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "heartbeat");
    assert_eq!(list[0].description, p.description());
    assert_eq!(list[0].input_schema, p.input_schema());
}

/// T6：外部插件桥 A4 申报缺席形——四面缺省体恒 BridgeUnsupported。
#[tokio::test]
async fn t6_external_bridge_declared_absent() {
    struct AbsentBridge;
    #[async_trait]
    impl ExternalPluginBridge for AbsentBridge {}

    let manifest = PluginManifest::parse(
        r#"{"name":"demo-ext","version":"0.1.0","transport":"stdio","command":"demo-plugin","tools":[{"name":"demo_greet"}]}"#,
    )
    .unwrap();
    let bridge = AbsentBridge;

    match bridge.spawn(&manifest).await {
        Err(ToolError::BridgeUnsupported(s)) => assert!(s.contains("demo-ext"), "{s}"),
        other => panic!("expected BridgeUnsupported from spawn, got {other:?}"),
    }
    match bridge.send_request("demo-ext", json!({"jsonrpc":"2.0"})).await {
        Err(ToolError::BridgeUnsupported(s)) => assert!(s.contains("send_request"), "{s}"),
        other => panic!("expected BridgeUnsupported from send_request, got {other:?}"),
    }
    match bridge.recv("demo-ext").await {
        Err(ToolError::BridgeUnsupported(s)) => assert!(s.contains("recv"), "{s}"),
        other => panic!("expected BridgeUnsupported from recv, got {other:?}"),
    }
    match bridge.shutdown("demo-ext").await {
        Err(ToolError::BridgeUnsupported(s)) => assert!(s.contains("shutdown"), "{s}"),
        other => panic!("expected BridgeUnsupported from shutdown, got {other:?}"),
    }
}
