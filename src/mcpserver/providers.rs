//! 腿四接线：内置 19 具工具的 ToolProvider 统一注册（SPEC-025 § 插件槽位
//! 协议 A3：内置工具全数 ToolProvider 注册，废弃散点 match 分派）。
//!
//! 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//! 行为零变锁三重：一即描述与 schema 与 defs 数组同源（provider 注册即吃
//! defs 三元组，文本零漂移）；二即注册序与 tool_defs 拼接序一致（alpha 九
//! 后 beta 矩阵暴露十）；三即错误通道 Execution(String) 承 McpError 序列化
//! 载荷，server 端逐字还原 JSON-RPC error 形（接线前后双跑对表锁，基线
//! 指纹 7e1c9328608f7438）。
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use rmcp::ErrorData as McpError;
use serde_json::Value;
use tokio::sync::Mutex;

use super::matrix::is_tool_exposed;
use super::server::{alpha_defs, beta_defs, get_bool, get_f64, get_i64, get_str, get_str_list};
use super::session::ConnectionSession;
use super::alpha;
use super::tools;
use crate::tools_registry::{ToolError, ToolProvider, ToolRegistry};

type Conn = Arc<Mutex<ConnectionSession>>;
type Handler = Box<
    dyn Fn(Value, Conn) -> Pin<Box<dyn Future<Output = Value> + Send>> + Send + Sync,
>;

struct ClosuredProvider {
    name: &'static str,
    description: String,
    schema: Value,
    conn: Conn,
    handler: Handler,
}

#[async_trait::async_trait]
impl ToolProvider for ClosuredProvider {
    fn name(&self) -> &str {
        self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn input_schema(&self) -> Value {
        self.schema.clone()
    }
    async fn call(&self, args: Value) -> Result<Value, ToolError> {
        // 行为零变锁（原 call_tool 形）：工具层错误以载荷内 error 键承载
        // （Value），恒 Ok 进 CallToolResult；registry 通道只承载调度级
        // NotFound（未注册名）。
        let conn = self.conn.clone();
        Ok((self.handler)(args, conn).await)
    }
}

/// 连接实效根：连接会话绑定的域根（HTTP active 牌即所绑域根，stdio 与匿名
/// 形即中央缺省根）。锁竞争即回落中央根（根不随会话漂移，竞窗口仅重签换
/// 域瞬间）。recognize-solo：读具根由连接承载，task_local 不跨 rmcp spawn
/// 任务（2026-09-14 活体验收实证），故弃 REQUEST_IDENTITY 直取形。
fn conn_root(c: &Conn) -> PathBuf {
    c.try_lock()
        .map(|g| g.root.clone())
        .unwrap_or_else(|_| super::runtime::resolve_root())
}

fn reg(
    registry: &ToolRegistry,
    conn: &Conn,
    def: (&'static str, String, Value),
    handler: Handler,
) {
    let (name, description, schema) = def;
    let _ = registry.register(Box::new(ClosuredProvider {
        name,
        description,
        schema,
        conn: conn.clone(),
        handler,
    }));
}

/// 内置全量注册（A3）：alpha 九读数加 beta 矩阵暴露写面，注册序与
/// tool_defs 拼接序一致；调用方持 registry 锁面零竞争（构造期单线程）。
pub fn build_registry(agent_class: &str, conn: Conn) -> ToolRegistry {
    let registry = ToolRegistry::new();
    // —— alpha 九读数（描述与 schema 逐字承 alpha_defs 同源 defs）——
    for (name, description, schema) in alpha_defs().into_iter().map(|t| {
        (
            leak_name(&t.name),
            t.description.unwrap_or_default().to_string(),
            Value::Object((*t.input_schema.clone()).clone()),
        )
    }) {
        let h: Handler = match name {
            "chain_query" => Box::new(|a, c| Box::pin(alpha::chain_query(conn_root(&c), get_str(&a, "date"), get_str(&a, "event_type")))),
            "chain_verify" => Box::new(|a, c| Box::pin(alpha::chain_verify(conn_root(&c), get_str(&a, "date")))),
            "critsweep" => Box::new(|a, c| Box::pin(alpha::critsweep(conn_root(&c), get_str(&a, "date")))),
            "heartbeat" => Box::new(|_a, c| Box::pin(alpha::heartbeat(conn_root(&c)))),
            "locks_read" => Box::new(|_a, c| Box::pin(alpha::locks_read(conn_root(&c)))),
            "naming_guide" => Box::new(|_a, _c| Box::pin(alpha::naming_guide())),
            "nomenclator_query" => Box::new(|a, _c| Box::pin(alpha::nomenclator_query(get_str(&a, "word")))),
            "nomenclator_check" => Box::new(|a, c| Box::pin(alpha::nomenclator_check(conn_root(&c), get_str(&a, "target")))),
            "retriever_recall" => Box::new(|a, c| {
                Box::pin(alpha::retriever_recall(conn_root(&c), 
                    get_str_list(&a, "topic"),
                    get_str_list(&a, "word"),
                    get_str_list(&a, "event"),
                    get_str(&a, "since"),
                    get_str(&a, "until"),
                    get_str(&a, "archive"),
                    get_str(&a, "at"),
                ))
            }),
            _ => unreachable!("alpha defs name drift"),
        };
        reg(&registry, &conn, (name, description, schema), h);
    }
    // —— beta 矩阵暴露写面（conn 闭包捕获，分级过滤与 tool_defs 同滤）——
    for (name, description, schema) in beta_defs(agent_class).into_iter().map(|t| {
        (
            leak_name(&t.name),
            t.description.unwrap_or_default().to_string(),
            Value::Object((*t.input_schema.clone()).clone()),
        )
    }) {
        let h: Handler = match name {
            "lease_open" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_lease_open(
                        &mut guard,
                        get_str(&a, "package"),
                        get_str(&a, "intent"),
                        get_str_list(&a, "repo"),
                        get_str_list(&a, "allow"),
                    )
                    .await
                })
            }),
            "record_intent" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_record_intent(
                        &mut guard,
                        get_str(&a, "record"),
                        get_str(&a, "validation"),
                        get_str(&a, "date"),
                    )
                    .await
                })
            }),
            "record_append" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_record_append(
                        &mut guard,
                        get_str(&a, "report"),
                        get_i64(&a, "exit_code"),
                        get_str(&a, "date"),
                    )
                    .await
                })
            }),
            "record_park" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_record_park(&mut guard, get_str(&a, "record"), get_str(&a, "date"))
                        .await
                })
            }),
            "lease_lock" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_lease_lock(
                        &mut guard,
                        get_str(&a, "path"),
                        get_str(&a, "mode"),
                        get_bool(&a, "wait"),
                    )
                    .await
                })
            }),
            "lease_unlock" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_lease_unlock(&mut guard, get_str(&a, "path")).await
                })
            }),
            "lease_wait_turn" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_lease_wait_turn(
                        &mut guard,
                        get_str(&a, "path"),
                        get_f64(&a, "timeout_seconds"),
                        get_f64(&a, "interval_seconds"),
                        get_str(&a, "mode"),
                    )
                    .await
                })
            }),
            "lease_claim" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_lease_claim(
                        &mut guard,
                        get_str(&a, "package"),
                        get_i64(&a, "ttl"),
                        get_str(&a, "claimant"),
                    )
                    .await
                })
            }),
            "lease_commit" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_lease_commit(
                        &mut guard,
                        a.get("repo").cloned(),
                        get_str(&a, "stage"),
                        get_str(&a, "subject"),
                        get_i64(&a, "seq"),
                        get_str(&a, "cert"),
                        get_str(&a, "note"),
                        get_str_list(&a, "trail"),
                        get_str(&a, "root"),
                    )
                    .await
                })
            }),
            "lease_close" => Box::new(|a, c| {
                Box::pin(async move {
                    let mut guard = c.lock().await;
                    tools::tool_lease_close(&mut guard, get_str(&a, "package"), get_str(&a, "reason"))
                        .await
                })
            }),
            _ => unreachable!("beta defs name drift"),
        };
        reg(&registry, &conn, (name, description, schema), h);
    }
    let _ = is_tool_exposed("heartbeat", agent_class); // 矩阵面引用保温（beta 过滤在 beta_defs 内完成）
    registry
}

fn leak_name(n: &str) -> &'static str {
    // defs 名字面量本为 'static；Tool.name 是 rmcp SharedString，map 回 &'static
    // 经字面量表直查（19 具名单有限，drift 即 unreachable）。
    match n {
        "chain_query" => "chain_query",
        "chain_verify" => "chain_verify",
        "critsweep" => "critsweep",
        "heartbeat" => "heartbeat",
        "locks_read" => "locks_read",
        "naming_guide" => "naming_guide",
        "nomenclator_query" => "nomenclator_query",
        "nomenclator_check" => "nomenclator_check",
        "retriever_recall" => "retriever_recall",
        "lease_open" => "lease_open",
        "record_intent" => "record_intent",
        "record_append" => "record_append",
        "record_park" => "record_park",
        "lease_lock" => "lease_lock",
        "lease_unlock" => "lease_unlock",
        "lease_wait_turn" => "lease_wait_turn",
        "lease_claim" => "lease_claim",
        "lease_commit" => "lease_commit",
        "lease_close" => "lease_close",
        other => unreachable!("def name drift: {other}"),
    }
}
