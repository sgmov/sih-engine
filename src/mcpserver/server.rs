//! 服务器装配：rmcp ServerHandler 注册 alpha 九具与 beta 十二写具（分级裁剪）
//! （sihmcp-solo 段1 加段2 stdio 面）。
//!
//! 工具描述文与正典指针逐字承自 sih-tools/mcpline/src/mcpline/server.py
//! （工具描述自足是 SPEC-023 契约面，beta 描述冻结承 DES-014 第七节）；serverInfo
//! 名称承载体名 sihmcp，与旧载体名 mcpline 的差异是有意变更（载体替换，线与
//! 契约正典不变）。

use std::sync::Arc;

use rmcp::model::{
    CallToolRequestParam, CallToolResult, Content, Implementation, ListToolsResult,
    PaginatedRequestParam, ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use serde_json::{json, Map, Value};
use tokio::sync::Mutex;

use super::alpha;
use super::matrix::{is_tool_exposed, resolve_agent_class};
use super::session::ConnectionSession;
use super::providers;
use super::tools;
use crate::tools_registry::{ToolError, ToolRegistry};

const SERVER_NAME: &str = "sihmcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROTOCOL_VERSION: &str = "2025-11-25";
const CANON_SPEC_023: &str = "sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md";
const CANON_LINE_PKG: &str = "sih-engine/sih/state/plan/mcpline-line-v1.md";
const CANON_SPEC_007: &str = "sih-engine/doc/spec/SPEC-007-project-memory-component.md";
const CANON_DES_014: &str = "sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md";

pub struct SihMcpServer {
    /// 一连接一会话（DES-014 第一节）：stdio 形态一进程一连接，进程始即连接生。
    pub conn: Arc<Mutex<ConnectionSession>>,
    pub agent_class: &'static str,
    /// A3 统一注册形（SPEC-025 腿四接线）：19 具内置 provider 全数注册，
    /// list 与 call 经 registry 派生，散点 match 分派退役。
    pub registry: ToolRegistry,
}

impl SihMcpServer {
    /// 连接构造：分级静态解析一次；连接级自动开由 main 于 serve 前调
    /// auto_open 承载（缺省形客户端经 lease_open 显式立）。
    pub fn new() -> Self {
        let agent_class = resolve_agent_class();
        let conn = Arc::new(Mutex::new(ConnectionSession::new(agent_class)));
        let registry = providers::build_registry(agent_class, conn.clone());
        Self {
            conn,
            agent_class,
            registry,
        }
    }
}

impl Default for SihMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

fn protocol_version() -> ProtocolVersion {
    serde_json::from_value(Value::String(PROTOCOL_VERSION.to_owned()))
        .expect("protocol version string is valid")
}

fn canon_pair() -> String {
    format!("{CANON_SPEC_023}；{CANON_LINE_PKG}")
}

pub(crate) fn beta_defs(agent_class: &str) -> Vec<Tool> {
    // 工具描述冻结为契约文本（DES-014 第七节缓解位）：静态、随批评审、只载
    // 操作语义不含可执行指令面。_beta_desc 形：zh 加 en 加 设计正典指针。
    let canon_beta = format!("{CANON_DES_014}；{CANON_SPEC_023}");
    let bd = |zh: &str, en: &str| format!("{zh} {en} 设计正典：{canon_beta}");
    let defs: [(&str, String, Value); 12] = [
        ("lease_open", bd(
            "lease open：立约开工立本连接的 lease 会话（一连接恰好一会话），以 server 进程正身件签发，会话号经本工具结果透出记录；意图闸（ask3 验收）与同包活跃闸与施工面预检原位。",
            "lease open: establish this connection's single lease session with the server process identity; session id is returned in the result."),
         json!({"properties": {
             "package": {"title": "Package", "type": "string"},
             "intent": {"title": "Intent", "type": "string"},
             "repo": {"items": {"type": "string"}, "title": "Repo", "type": "array"},
             "allow": {"items": {"type": "string"}, "title": "Allow", "type": "array"},
         }, "required": ["package", "intent"], "type": "object"})),
        ("record_intent", bd(
            "record intent：追加意图精炼事件（写），闸二重意图拒（同 record 路径唯一意图笔）与闸三会话在册验原位；plain 形（零哲学引文）validation 豁免，ask3 形验证件必填（DES-016）。",
            "record intent: append an intent event via the existing scribe gates (re-intent and session-in-ledger checks in place); plain form (zero philosophy quotes) omits validation, ask3 form requires it (DES-016)."),
         json!({"properties": {
             "record": {"title": "Record", "type": "string"},
             "validation": {"title": "Validation", "type": "string"},
             "date": {"title": "Date", "type": "string"},
         }, "required": ["record", "validation"], "type": "object"})),
        ("record_append", bd(
            "record append：追加认证事件（写），闸三会话在册验原位；本操作无幂等闸，重放同 report 即重复认证笔（信封审计可检出）。",
            "record append: append a certification event via the existing session gate; no idempotency gate (declared)."),
         json!({"properties": {
             "report": {"title": "Report", "type": "string"},
             "exit_code": {"title": "Exit Code", "type": "integer"},
             "date": {"title": "Date", "type": "string"},
         }, "required": ["report", "exit_code"], "type": "object"})),
        ("record_park", bd(
            "record park：追加停泊事件（写），停泊记录 JSON 透传。",
            "record park: append a parking event via the existing scribe CLI."),
         json!({"properties": {
             "record": {"title": "Record", "type": "string"},
             "date": {"title": "Date", "type": "string"},
         }, "required": ["record"], "type": "object"})),
        ("record_direct", bd(
            "record direct：直改链笔（写，仅本地可信 agent 可调；直改车道免全套租约仪式但免不了治理，agent 笔强制挂 server 正身报告）。",
            "record direct: direct-pen chain event (local trusted agents only)."),
         json!({"properties": {
             "record": {"title": "Record", "type": "string"},
             "date": {"title": "Date", "type": "string"},
         }, "required": ["record"], "type": "object"})),
        ("lease_lock", bd(
            "lease lock：租内取锁，悲观五验（在册、择定、范围、身份、绑定）原位；撞锁即 locked_elsewhere 拒或 wait 入队受理返位次。",
            "lease lock: acquire a path lock; five in-place verifications; conflicts surface locked_elsewhere or queue position."),
         json!({"properties": {
             "path": {"title": "Path", "type": "string"},
             "mode": {"title": "Mode", "type": "string"},
             "wait": {"default": false, "title": "Wait", "type": "boolean"},
         }, "required": ["path"], "type": "object"})),
        ("lease_unlock", bd(
            "lease unlock：放锁，同一五验加持锁验原位。",
            "lease unlock: release a path lock with the in-place verifications."),
         json!({"properties": {"path": {"title": "Path", "type": "string"}},
                "required": ["path"], "type": "object"})),
        ("lease_wait_turn", bd(
            "lease wait-turn：排队阻塞至轮到并取锁（机械轮询，timeout 到即如实返退出码与出队事实）。",
            "lease wait-turn: block until your turn then acquire; timeout surfaces the dequeue fact as-is."),
         json!({"properties": {
             "path": {"title": "Path", "type": "string"},
             "timeout_seconds": {"title": "Timeout Seconds", "type": "number"},
             "interval_seconds": {"title": "Interval Seconds", "type": "number"},
             "mode": {"title": "Mode", "type": "string"},
         }, "required": ["path"], "type": "object"})),
        ("lease_claim", bd(
            "lease claim：领取登记（声明位非执法位），同包未过期在领即 PackageAlreadyClaimed 拒。",
            "lease claim: declaration-style claim on a package; duplicate active claims are rejected."),
         json!({"properties": {
             "package": {"title": "Package", "type": "string"},
             "ttl": {"title": "Ttl", "type": "integer"},
             "claimant": {"title": "Claimant", "type": "string"},
         }, "required": ["package", "ttl"], "type": "object"})),
        ("lease_unclaim", bd(
            "lease unclaim：领取释放（仅本地可信 agent 可调），无在领即拦。",
            "lease unclaim: release a claim (local trusted agents only)."),
         json!({"properties": {
             "package": {"title": "Package", "type": "string"},
             "claimant": {"title": "Claimant", "type": "string"},
         }, "required": ["package"], "type": "object"})),
        ("lease_commit", bd(
            "lease commit：提交即四验后机械 message 落笔；settle 须 seq 与 cert（链上认证哈希）。",
            "lease commit: mechanical commit after the in-place four verifications; settle requires seq and cert."),
         json!({"properties": {
             "repo": {"anyOf": [{"type": "string"}, {"items": {"type": "string"}, "type": "array"}], "title": "Repo"},
             "stage": {"title": "Stage", "type": "string"},
             "subject": {"title": "Subject", "type": "string"},
             "seq": {"title": "Seq", "type": "integer"},
             "cert": {"title": "Cert", "type": "string"},
             "note": {"title": "Note", "type": "string"},
             "trail": {"items": {"type": "string"}, "title": "Trail", "type": "array"},
             "root": {"title": "Root", "type": "string"},
         }, "required": ["repo", "stage", "subject"], "type": "object"})),
        ("lease_close", bd(
            "lease close：收约（锁清零、分支归并删支、拆本吊销），受 close 链闸与 closeguard 约束；强拆与绕行旗标不透传。",
            "lease close: close the session; chain gate and closeguard apply; force and bypass flags are never forwarded."),
         json!({"properties": {
             "package": {"title": "Package", "type": "string"},
             "reason": {"title": "Reason", "type": "string"},
         }, "required": ["package"], "type": "object"})),
    ];
    // β 写面分级路由：矩阵无行的工具不注册即结构性拒透传（DES-014 第三节）。
    defs.into_iter()
        .filter(|(name, _, _)| is_tool_exposed(name, agent_class))
        .map(|(name, description, schema)| Tool {
            name: name.to_string().into(),
            description: Some(description.into()),
            input_schema: schema.as_object().cloned().unwrap_or_default().into(),
            output_schema: None,
            annotations: None,
        })
        .collect()
}

pub fn tool_defs(agent_class: &str) -> Vec<Tool> {
    let mut all = alpha_defs();
    all.extend(beta_defs(agent_class));
    all
}

pub(crate) fn alpha_defs() -> Vec<Tool> {
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

pub(crate) fn get_str(args: &Value, key: &str) -> Option<String> {
    args.get(key).and_then(|v| v.as_str()).map(str::to_owned)
}

pub(crate) fn get_bool(args: &Value, key: &str) -> Option<bool> {
    args.get(key).and_then(|v| v.as_bool())
}

pub(crate) fn get_i64(args: &Value, key: &str) -> Option<i64> {
    args.get(key).and_then(|v| v.as_i64())
}

pub(crate) fn get_f64(args: &Value, key: &str) -> Option<f64> {
    args.get(key).and_then(|v| v.as_f64())
}

pub(crate) fn get_str_list(args: &Value, key: &str) -> Option<Vec<String>> {
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
        let tools: Vec<Tool> = self
            .registry
            .list()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .into_iter()
            .map(|d| Tool {
                name: d.name.into(),
                description: Some(d.description.into()),
                input_schema: d.input_schema.as_object().cloned().unwrap_or_default().into(),
                output_schema: None,
                annotations: None,
            })
            .collect();
        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let name = request.name.to_string();
        let args = Value::Object(request.arguments.unwrap_or_default());
        // β 写面：会话绑定卫在 tools 层 precheck（矩阵拒透传的未注册工具
        // 到不了这里；分级残留面以同表复核兜底）。
        let is_beta = super::matrix::MATRIX_ROWS.contains(&name.as_str());
        if is_beta && !is_tool_exposed(&name, self.agent_class) {
            return Err(McpError::invalid_params(
                format!("授权矩阵拒透传：分级 {} 无工具 {name} 行", self.agent_class),
                None,
            ));
        }
        // A3 接线（SPEC-025 腿四）：分派全量走 registry；执行级错误载荷经
        // Execution 通道承 McpError 序列化串，此处逐字还原保 JSON-RPC error
        // 形零变（双跑对表锁基线 7e1c9328608f7438）。
        let value = match self.registry.call(&name, args).await {
            Ok(v) => v,
            Err(ToolError::NotFound(n)) => {
                return Err(McpError::invalid_params(format!("未知工具 `{n}`"), None));
            }
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
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

/// beta 工具名单（矩阵行全集）供测试对表。
pub fn beta_tool_names() -> [&'static str; 12] {
    [
        "lease_open",
        "record_intent",
        "record_append",
        "record_park",
        "record_direct",
        "lease_lock",
        "lease_unlock",
        "lease_wait_turn",
        "lease_claim",
        "lease_unclaim",
        "lease_commit",
        "lease_close",
    ]
}

/// schema 构造对表面：schema Map 非空且含 type=object（注册面完整形）。
pub fn schemas_nonempty() -> bool {
    tool_defs("local").iter().all(|t| {
        let m: &Map<String, Value> = &t.input_schema;
        m.get("type").and_then(|v| v.as_str()) == Some("object")
    })
}
