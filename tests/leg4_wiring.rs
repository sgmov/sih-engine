//! 腿四接线回归锁：registry 派生 list 与 defs 零漂移加调用路由（SPEC-025 A3）。
use sih_engine::mcpserver::server::{SihMcpServer, tool_defs, tool_names, beta_tool_names};

#[test]
fn t1_registry_list_matches_defs_exactly() {
    let server = SihMcpServer::new();
    let listed = server.registry.list().expect("registry list");
    let defs = tool_defs(server.agent_class);
    assert_eq!(listed.len(), defs.len(), "具数一致");
    for (d, t) in listed.iter().zip(defs.iter()) {
        assert_eq!(d.name, t.name.as_ref(), "名单序一致");
        assert_eq!(d.description, t.description.clone().unwrap_or_default(), "描述逐字一致");
        let m: serde_json::Map<String, serde_json::Value> =
            d.input_schema.as_object().cloned().unwrap_or_default();
        assert_eq!(
            serde_json::Value::Object(m.clone()),
            serde_json::Value::Object((*t.input_schema).clone()),
            "schema 逐字一致 {}",
            d.name
        );
    }
}

#[test]
fn t2_registry_call_routes_read_tool() {
    let server = SihMcpServer::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let v = rt.block_on(server.registry.call("locks_read", serde_json::json!({}))).expect("call ok");
    assert!(v.get("held_count").is_some() || v.get("error").is_some(), "locks_read 出参键面");
}

#[test]
fn t3_unknown_tool_not_found() {
    let server = SihMcpServer::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let err = rt.block_on(server.registry.call("no_such_tool_xyz", serde_json::json!({}))).unwrap_err();
    assert!(err.to_string().contains("no_such_tool_xyz"));
}

#[test]
fn t4_alpha_nine_registered_and_full_family() {
    let server = SihMcpServer::new();
    let names: Vec<String> = server.registry.list().unwrap().into_iter().map(|d| d.name).collect();
    for n in tool_names() {
        assert!(names.iter().any(|x| x == n), "alpha {n} 在册");
    }
    let exposed_beta: Vec<&str> = beta_tool_names()
        .iter()
        .filter(|n| sih_engine::mcpserver::matrix::is_tool_exposed(n, server.agent_class))
        .copied()
        .collect();
    for n in exposed_beta {
        assert!(names.iter().any(|x| x == n), "beta 暴露 {n} 在册");
    }
}
