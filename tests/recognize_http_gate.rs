//! recognize-solo 批 HTTP 读面域隔离端到端验收：真 axum 路由真 TCP 请求，
//! 闸面四用例（降级三因受控教学面、未域自举绑根读写硬拒、已域自举放行）。
//! 夹具 SIH_ROOT 指进程内临时中央根，零 8765 依赖零网络外部面。集成测试
//! 进程独占环境变量，单测试函数串行用例，无并行环境竞争。

use std::fs;
use std::sync::OnceLock;

use serde_json::Value;
use sih_engine::mcpserver::httpface::streamable_router;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// 进程级夹具端口：临时中央根（leak 即常驻，夹具生命期=进程生命期）加登记
/// 册三牌（opened/bare/stopped），ephemeral 端口起真闸面路由。
async fn fixture_port() -> u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    if let Some(p) = PORT.get() {
        return *p;
    }
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    // 夹具目录泄漏即常驻：测试进程短命，生命期与进程等长即正确语义。
    std::mem::forget(dir);
    fs::create_dir_all(root.join("sih-engine/doc")).unwrap();
    fs::create_dir_all(root.join("sih-tools")).unwrap();
    std::env::set_var("SIH_ROOT", root.display().to_string());

    // 登记册三牌：已域自举绑根、未域自举绑根、停行牌（末行为准形）。
    let opened = root.join("opened-domain");
    fs::create_dir_all(opened.join("sih/ledger")).unwrap();
    let bare = root.join("bare-domain");
    fs::create_dir_all(&bare).unwrap();
    let registry = root.join("sih-tools/mcpline/ledger/tokens.ndjson");
    fs::create_dir_all(registry.parent().unwrap()).unwrap();
    let mut rows = String::new();
    for row in [
        serde_json::json!({"token_id": "tok-opened", "domain_root": opened.display().to_string(),
            "scope": "domain_write", "status": "active", "issued_at": "2026-09-14T00:00:00+00:00", "issued_by": "test"}),
        serde_json::json!({"token_id": "tok-bare", "domain_root": bare.display().to_string(),
            "scope": "domain_write", "status": "active", "issued_at": "2026-09-14T00:00:00+00:00", "issued_by": "test"}),
        serde_json::json!({"token_id": "tok-stopped", "domain_root": opened.display().to_string(),
            "scope": "domain_write", "status": "active", "issued_at": "2026-09-14T00:00:00+00:00", "issued_by": "test"}),
        serde_json::json!({"token_id": "tok-stopped", "domain_root": opened.display().to_string(),
            "scope": "domain_write", "status": "stopped", "issued_at": "2026-09-14T00:01:00+00:00", "issued_by": "test"}),
    ] {
        rows.push_str(&format!("{row}\n"));
    }
    fs::write(&registry, rows).unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let app = streamable_router("/mcp");
    tokio::spawn(async move { axum::serve(listener, app).await });
    PORT.set(port).ok();
    port
}

/// 裸 HTTP/1.1 POST（零 HTTP 客户端依赖；Connection: close 即读尽即断）。
async fn post_json(port: u16, body: &str, bearer: Option<&str>) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let auth = bearer
        .map(|t| format!("Authorization: Bearer {t}\r\n"))
        .unwrap_or_default();
    let req = format!(
        "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n{auth}\r\n{body}",
        body.len()
    );
    stream.write_all(req.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    let raw = String::from_utf8_lossy(&buf).to_string();
    let status: u16 = raw
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let body = raw.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
    (status, body)
}

fn call_tool(id: i64, name: &str) -> String {
    serde_json::json!({
        "jsonrpc": "2.0", "id": id, "method": "tools/call",
        "params": {"name": name, "arguments": {}}
    })
    .to_string()
}

#[tokio::test(flavor = "current_thread")]
async fn http_read_gate_end_to_end() {
    let port = fixture_port().await;

    // 用例一（F-3）：无牌 critsweep → 受控教学面，isError false，因码
    // missing_authorization，identity_notice 在列，零域数据投影。
    let (status, body) = post_json(port, &call_tool(1, "critsweep"), None).await;
    assert_eq!(status, 200, "{body}");
    let v: Value = serde_json::from_str(&body).unwrap();
    let sc = &v["result"]["structuredContent"];
    assert_eq!(v["result"]["isError"], false);
    assert_eq!(sc["reason_code"], "missing_authorization");
    assert!(sc["identity_notice"].as_str().unwrap().contains("/tokens"));
    assert!(sc.get("root").is_none());

    // 用例二（F-3）：不在册牌 → 因码 token_unregistered。
    let (status, body) = post_json(port, &call_tool(2, "critsweep"), Some("tok-absent")).await;
    assert_eq!(status, 200, "{body}");
    let v: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(
        v["result"]["structuredContent"]["reason_code"],
        "token_unregistered"
    );

    // 用例三（F-3）：停行牌（末行为准）→ 因码 token_stopped。
    let (status, body) = post_json(port, &call_tool(3, "critsweep"), Some("tok-stopped")).await;
    assert_eq!(status, 200, "{body}");
    let v: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(
        v["result"]["structuredContent"]["reason_code"],
        "token_stopped"
    );

    // 用例四（F-2）：未域自举绑根 active 牌读 critsweep → 硬拒教学
    // domain_not_bootstrapped，零中央回退（出参零中央根数据投影）。
    let (status, body) = post_json(port, &call_tool(4, "critsweep"), Some("tok-bare")).await;
    assert_eq!(status, 200, "{body}");
    let v: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["result"]["isError"], true);
    assert_eq!(
        v["result"]["structuredContent"]["reason_code"],
        "domain_not_bootstrapped"
    );

    // 用例五（F-5 前网位）：未域自举绑根牌写 record_intent → 403 教学拒。
    let (status, body) = post_json(port, &call_tool(5, "record_intent"), Some("tok-bare")).await;
    assert_eq!(status, 403, "{body}");
    let v: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["reason_code"], "domain_not_bootstrapped");

    // 用例六（F-1 前闸位）：已域自举绑根牌读 critsweep → 闸放行（放行后请求
    // 进入 rmcp 会话层，回非闸面响应即证闸不拦；闸面载荷零出现）。
    let (status, body) = post_json(port, &call_tool(6, "critsweep"), Some("tok-opened")).await;
    assert!(
        !body.contains("domain_not_bootstrapped")
            && !body.contains("identity_notice")
            && !body.contains("missing_authorization"),
        "放行形混入闸面载荷: HTTP {status} {body}"
    );
}
