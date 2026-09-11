//! 服务器装配：rmcp ServerHandler 注册 alpha 九具（sihmcp-solo 段1 stdio 面）。
//!
//! 工具描述文与正典指针逐字承自 sih-tools/mcpline/src/mcpline/server.py
//! （工具描述自足是 SPEC-023 契约面）；serverInfo.name 承载体名 sihmcp，
//! 与旧载体名 mcpline 的差异是有意变更（载体替换，线与契约正典不变）。

use rmcp::model::{
    CallToolRequestParam, CallToolResult, Content, Implementation, ListToolsResult,
    PaginatedRequestParam, ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use serde_json::{json, Map, Value};

use super::alpha;

const SERVER_NAME: &str = "sihmcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROTOCOL_VERSION: &str = "2025-11-25";
const CANON_SPEC_023: &str = "sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md";
const CANON_LINE_PKG: &str = "sih-engine/sih/state/plan/mcpline-line-v1.md";
const CANON_SPEC_007: &str = "sih-engine/doc/spec/SPEC-007-project-memory-component.md";
const CANON_DES_014: &str = "sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md";

pub struct SihMcpServer;

fn protocol_version() -> ProtocolVersion {
    serde_json::from_value(Value::String(PROTOCOL_VERSION.to_owned()))
        .expect("protocol version string is valid")
}

fn canon_pair() -> String {
    format!("{CANON_SPEC_023}；{CANON_LINE_PKG}")
}

fn tool_defs() -> Vec<Tool> {
    let canon = canon_pair();
    let desc_query = format!(
        "查当日治理链事件清单（哈希、事件型、主体字段）。Query one day's governance chain events \
         (hash, event type, subject). 承接 CLI：sih-engine/target/debug/scribe query --trail。正典：{canon}"
    );
    let desc_verify = format!(
        "验当日治理链：逐笔哈希校验与整链 valid 判词。Verify one day's chain: per-event hash \
         checks and whole-chain valid verdict. 承接 CLI：scribe verify --trail。正典：{canon}"
    );
    let desc_sweep = format!(
        "判据扫：GOV-002 五判据三态、泊界两线路由、两账本在飞，严格 JSON 单对象。Criterion sweep: \
         five GOV-002 criteria tri-state, parking routing, two in-flight ledgers. \
         承接 CLI：python3 sih-tools/critsweep/sweep.py --at <date> --root <root>。正典：{canon}"
    );
    let desc_heart = format!(
        "心跳：秤星三维最新读数与距上快照间隔日（只读不落链）。Heartbeat: latest gauge tri-dimension \
         readings and days since last snapshot (read-only). 承接 CLI：\
         cd sih-tools/gauge 且 PYTHONPATH=src python3 -m gauge.cli read。正典：{canon}"
    );
    let desc_locks = format!(
        "锁面读数：未释放锁成对核算与活跃会话数。Locks reading: unreleased lock accounting and \
         active session count. 承接 CLI：uv run --project sih-tools/lease lease status（查册）。正典：{canon}"
    );
    let desc_recall = format!(
        "温故检索：治理档案四轴检索（topic 主题轴、word 文轴、event 事件轴、time 时间轴），\
         stdout NDJSON 原文透传与退出码三值透传，只读投影 retriever recall，只报不判。\
         Retrieve governance archive facets on four axes (topic/word/event/time) with verbatim \
         NDJSON and exit-code passthrough; read-only projection, reports without judging. \
         承接 CLI：sih-engine/target/debug/retriever recall ... --root <域根>，\
         cwd 不承根以 --root 显式传。正典：{CANON_SPEC_023} 与 {CANON_SPEC_007}；{CANON_LINE_PKG}"
    );
    let desc_naming = "立名指引：司衡命名动作程序与纪律静态教学五段（立名五步形、DEC-017 修订指针、\
        检词六态语义、stem 闸拒教认领三语义、死档禁条），只读零裁决。Naming guide: static \
        five-section teaching for naming actions, read-only, zero adjudication. \
        正典：DEC-017；立名 skill；pk-090。"
        .to_string();
    let desc_nom_query = "查词：术语六态查询（已立、懒波、死档、候补、未知）附出处指针，\
        只读投影 nomenclator query。命名动作前查册义务的机械承载位。"
        .to_string();
    let desc_nom_check = "文档核查：死档禁字级与懒波词两规则字符串级检出，行号与摘录入报告，\
        只读投影 nomenclator check。"
        .to_string();

    let defs: [(&str, String, Value); 9] = [
        ("chain_query", desc_query, json!({
            "properties": {
                "date": {"title": "Date", "type": "string"},
                "event_type": {"title": "Event Type", "type": "string"},
            },
            "required": [],
            "type": "object",
        })),
        ("chain_verify", desc_verify, json!({
            "properties": {"date": {"title": "Date", "type": "string"}},
            "required": ["date"],
            "type": "object",
        })),
        ("critsweep", desc_sweep, json!({
            "properties": {"date": {"title": "Date", "type": "string"}},
            "required": ["date"],
            "type": "object",
        })),
        ("heartbeat", desc_heart, json!({
            "properties": {},
            "required": [],
            "type": "object",
        })),
        ("locks_read", desc_locks, json!({
            "properties": {},
            "required": [],
            "type": "object",
        })),
        ("naming_guide", desc_naming, json!({
            "properties": {},
            "required": [],
            "type": "object",
        })),
        ("nomenclator_query", desc_nom_query, json!({
            "properties": {"word": {"title": "Word", "type": "string"}},
            "required": ["word"],
            "type": "object",
        })),
        ("nomenclator_check", desc_nom_check, json!({
            "properties": {"target": {"title": "Target", "type": "string"}},
            "required": ["target"],
            "type": "object",
        })),
        ("retriever_recall", desc_recall, json!({
            "properties": {
                "topic": {"items": {"type": "string"}, "title": "Topic", "type": "array"},
                "word": {"items": {"type": "string"}, "title": "Word", "type": "array"},
                "event": {"items": {"type": "string"}, "title": "Event", "type": "array"},
                "since": {"title": "Since", "type": "string"},
                "until": {"title": "Until", "type": "string"},
                "archive": {"title": "Archive", "type": "string"},
                "at": {"title": "At", "type": "string"},
            },
            "required": [],
            "type": "object",
        })),
    ];
    defs.into_iter()
        .map(|(name, description, schema)| Tool {
            name: name.to_string().into(),
            description: Some(description.into()),
            input_schema: schema.as_object().cloned().unwrap_or_default().into(),
            output_schema: None,
            annotations: None,
        })
        .collect()
}

fn get_str(args: &Value, key: &str) -> Option<String> {
    args.get(key).and_then(|v| v.as_str()).map(str::to_owned)
}

fn get_str_list(args: &Value, key: &str) -> Option<Vec<String>> {
    args.get(key).and_then(|v| v.as_array()).map(|a| {
        a.iter()
            .filter_map(|x| x.as_str().map(str::to_owned))
            .collect()
    })
}

impl ServerHandler for SihMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: protocol_version(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: SERVER_NAME.to_owned(),
                version: SERVER_VERSION.to_owned(),
            },
            instructions: Some(
                "SiHankor mcpline MCP 服务器（alpha 只读面九工具 + beta 写面按分级裁剪）。\
                 SiHankor mcpline MCP server (alpha read-only nine tools + beta write face by class). \
                 冷启动先体检三具 locks_read 与 critsweep 与 heartbeat；写入走五步链 \
                 lease_open 加 lease_lock 加 lease_commit 加 lease_unlock 加 lease_close，\
                 意图笔与认证笔先于收约；被拒是教学：读错误载荷 reason_code 与 suggested_action \
                 改做法，勿原样重试。AI 使用说明书：HTTP 面 http://127.0.0.1:8765/manual，\
                 仓内 sih-tools/mcpline/AI-MANUAL.md。正典指针："
                    .to_owned()
                    + CANON_DES_014
                    + " 与 "
                    + CANON_SPEC_023
                    + " 与 "
                    + CANON_LINE_PKG
                    + "。",
            ),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult::with_all_items(tool_defs()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let name = request.name.to_string();
        let args = Value::Object(request.arguments.unwrap_or_default());
        let value = match name.as_str() {
            "chain_query" => {
                alpha::chain_query(get_str(&args, "date"), get_str(&args, "event_type")).await
            }
            "chain_verify" => alpha::chain_verify(get_str(&args, "date")).await,
            "critsweep" => alpha::critsweep(get_str(&args, "date")).await,
            "heartbeat" => alpha::heartbeat().await,
            "locks_read" => alpha::locks_read().await,
            "naming_guide" => alpha::naming_guide().await,
            "nomenclator_query" => alpha::nomenclator_query(get_str(&args, "word")).await,
            "nomenclator_check" => alpha::nomenclator_check(get_str(&args, "target")).await,
            "retriever_recall" => {
                alpha::retriever_recall(
                    get_str_list(&args, "topic"),
                    get_str_list(&args, "word"),
                    get_str_list(&args, "event"),
                    get_str(&args, "since"),
                    get_str(&args, "until"),
                    get_str(&args, "archive"),
                    get_str(&args, "at"),
                )
                .await
            }
            other => {
                return Err(McpError::invalid_params(format!("未知工具 `{other}`"), None));
            }
        };
        Ok(CallToolResult {
            content: Some(vec![Content::text(
                serde_json::to_string(&value).unwrap_or_default(),
            )]),
            structured_content: Some(value),
            is_error: Some(false),
        })
    }
}

/// 工具名单供测试对表（零写入守卫与冒烟用）。
pub fn tool_names() -> [&'static str; 9] {
    [
        "chain_query",
        "chain_verify",
        "critsweep",
        "heartbeat",
        "locks_read",
        "naming_guide",
        "nomenclator_query",
        "nomenclator_check",
        "retriever_recall",
    ]
}

/// schema 构造对表面：schema Map 非空且含 type=object（注册面完整形）。
pub fn schemas_nonempty() -> bool {
    tool_defs().iter().all(|t| {
        let m: &Map<String, Value> = &t.input_schema;
        m.get("type").and_then(|v| v.as_str()) == Some("object")
    })
}
