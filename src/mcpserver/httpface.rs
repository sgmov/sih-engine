//! HTTP 识别写面：DES-015 识别写面的 rmcp 加 axum 承载（sihmcp-solo 段3 甲簇）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/httpface.py（识别与域绑定与
//! scope 分档与 Bearer 闸）与 writeface/domains.py（域布局两形）。判词照录：
//! 识别语义形是连接头带项目标识即可（Authorization Bearer 头承载位），查登记
//! 册得所绑域与 scope 绑定本连接；写面十具经既有 CLI 执法透传只落所绑域；
//! scope 三档（readonly 拒写、domain_write 缺省授出、custom 零缺省放行）；
//! 逐调用新读登记册（零缓存执法结果）；一标识牌至多一活跃会话（进程内会话表
//! 按 token_id 一对一承载，close 成即解绑可再立）；失败语义直给教学语。
//!
//! 承载形差异（如实申报）：rmcp StreamableHttpService 是会话制（每 MCP 会话
//! 工厂构造一个 SihMcpServer，连接一会话语义），Python FastMCP 是无会话制
//! （逐请求独立）。故标识绑定发生在 initialize 时（会话根就此锚定），而写
//! 工具的逐调用识别与 scope 裁决仍每笔在传输闸新读登记册执行（零缓存不变）。
//! 读工具域形式（按所绑域组装与 identity_notice 降级注记）须 alpha 读具收
//! root 参方可承载，本批未动 alpha.rs，如实申报为待主线最小改动位。
//!
//! 正典指针：DES-015；DES-014；SPEC-023。

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex as StdMutex};

use axum::body::{Body, to_bytes};
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use serde_json::{Value, json};
use tokio::sync::Mutex;

use super::errors::{CANON_DES_014, CANON_SPEC_023};
use super::matrix::AGENT_CLASS_EXTERNAL;
use super::runtime::resolve_root;
use super::server::SihMcpServer;
use super::session::ConnectionSession;
use super::tokens;

pub const CANON_DES_015: &str = "sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md";

/// 正典指针三元（httpface.py CANON_POINTERS 对等，序固定）。
pub const CANON_POINTERS: [&str; 3] = [CANON_DES_015, CANON_DES_014, CANON_SPEC_023];

/// HTTP 写面十具（DES-015 实装六件第五条；与 DES-014 外部冷 agent 最小写集
/// 同一十行——record_direct 与 lease_unclaim 是本地可信位，HTTP 面零注册）。
pub const HTTP_WRITE_TOOLS: [&str; 10] = [
    "lease_open",
    "record_intent",
    "record_append",
    "record_park",
    "lease_lock",
    "lease_unlock",
    "lease_wait_turn",
    "lease_claim",
    "lease_commit",
    "lease_close",
];

/// α 只读九名（SPEC-023 契约照录；HTTP 面结构 = 此九 + 写面十）。
pub const HTTP_ALPHA_TOOLS: [&str; 9] = [
    "chain_query",
    "chain_verify",
    "critsweep",
    "heartbeat",
    "locks_read",
    "naming_guide",
    "nomenclator_query",
    "nomenclator_check",
    "retriever_recall",
];

/// 写工具载荷重试纪律（DES-015 失败语义节：写工具载荷加 retry_discipline 字段）。
pub const RETRY_DISCIPLINE: &str = "撞锁改 lease_wait_turn 排队候轮；重试前查会话在册（会话号随 lease_open 出参载出，逐写操作结果回带）；record append 无幂等闸，超时重试即重复认证笔，由信封审计与 meter 交叉核对检出";

/// 降只读形教学注记（读工具出参附注位；读具域形式待 alpha 收 root 参后启用）。
pub const IDENTITY_NOTICE_DEGRADED: &str = "连接携标识但未识别为 active（不在册或已停行）：本连接降只读投影中央域；写工具一律拒透传（401）；管理台 /tokens 可查册或换牌";

// ============================================================ 域布局两形
// 行为对等基准：writeface/domains.py。一项目一治理域，域目录布局两形并存：
// 第一域历史布局映射形（链与治理状态住引擎仓 sih-engine/sih/ 下、租约台账住
// 工具仓 sih-tools/lease/ledger/ 下、标识登记册住 sih-tools/mcpline/ledger/
// tokens.ndjson）与新城正典单根布局形（域根下 sih 目录树承载全部治理状态）。
// 两形判别是静态确定性规则，零判定语义零 LLM。

pub const FORM_FIRST: &str = "first";
pub const FORM_CANONICAL: &str = "canonical";

// 第一域历史布局映射登记表（domains.py 判词照录，登记面数据不迁移不重写）。
const FIRST_DOMAIN_TRAIL: &str = "sih-engine/sih/event/trail";
const FIRST_DOMAIN_LEDGER: &str = "sih-tools/lease/ledger";
const FIRST_DOMAIN_TOKENS: &str = "sih-tools/mcpline/ledger/tokens.ndjson";
const FIRST_DOMAIN_PARKING: &str = "sih-engine/sih/state/parking/materials";
const FIRST_DOMAIN_REPOS: [&str; 2] = ["sih-tools", "sih-engine"];

// 新城正典单根布局（DES-015 域目录布局规约定稿照录）。
const CANONICAL_TRAIL: &str = "sih/event/trail";
const CANONICAL_LEDGER: &str = "sih/ledger";
const CANONICAL_PARKING: &str = "sih/state/parking/materials";

/// 域布局投影：域根与形态与中央根三元，路径投影俱由本对象承载
/// （domains.py DomainLayout 对等）。
#[derive(Debug, Clone)]
pub struct DomainLayout {
    pub root: PathBuf,
    pub form: &'static str,
    pub central_root: PathBuf,
}

impl DomainLayout {
    pub fn is_first(&self) -> bool {
        self.form == FORM_FIRST
    }

    /// 该域逐日哈希链路径：<date>.ndjson。
    pub fn trail(&self, date: &str) -> PathBuf {
        self.trail_dir().join(format!("{date}.ndjson"))
    }

    /// 该域链目录。第一域锚域根本尊（自锚形即中央根，与 domains.py
    /// central_root 承载形等价；标记判别形锚各域自身根）。
    pub fn trail_dir(&self) -> PathBuf {
        if self.is_first() {
            self.root.join(FIRST_DOMAIN_TRAIL)
        } else {
            self.root.join(CANONICAL_TRAIL)
        }
    }

    /// 该域租约台账册族：sessions 与 locks 与 claims 与 bills 四路径（键与
    /// 既有 lease CLI 台账覆写旗标一一对应，零新增键）。
    pub fn ledger_paths(&self) -> BTreeMap<String, PathBuf> {
        let base = if self.is_first() {
            self.root.join(FIRST_DOMAIN_LEDGER)
        } else {
            self.root.join(CANONICAL_LEDGER)
        };
        BTreeMap::from([
            ("ledger".to_string(), base.join("sessions.ndjson")),
            ("locks".to_string(), base.join("locks.ndjson")),
            ("claims".to_string(), base.join("claims.ndjson")),
            ("bills".to_string(), base.join("lockface-bills.ndjson")),
        ])
    }

    /// 该域标识登记册路径：第一域 mcpline 工具面自治位，新城 sih/ledger 位。
    pub fn tokens_path(&self) -> PathBuf {
        if self.is_first() {
            self.root.join(FIRST_DOMAIN_TOKENS)
        } else {
            self.root.join(CANONICAL_LEDGER).join("tokens.ndjson")
        }
    }

    /// 该域泊界材料目录。
    pub fn parking_dir(&self) -> PathBuf {
        if self.is_first() {
            self.root.join(FIRST_DOMAIN_PARKING)
        } else {
            self.root.join(CANONICAL_PARKING)
        }
    }

    /// 该域仓集（写入面仓指向域）：第一域双仓根相对形，新城域根单仓。
    pub fn repos(&self) -> Vec<String> {
        if self.is_first() {
            FIRST_DOMAIN_REPOS.iter().map(|s| s.to_string()).collect()
        } else {
            vec![self.root.display().to_string()]
        }
    }
}

/// 路径归一：先 canonicalize（已存在路径），缺席回落 absolute（不存在路径
/// 也可判形，对等 Python Path.resolve() 非严格形）。
fn norm_path(p: &Path) -> PathBuf {
    p.canonicalize()
        .unwrap_or_else(|_| std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf()))
}

/// 两形标记判别（DES-015 判据扫 detect_layout_form 同款谓词）：双仓标记
/// （sih-engine/Cargo.toml 加 sih-tools/pyproject.toml）俱在即第一域形，
/// sih/ledger 在即新城正典形，俱缺即 None。
pub fn detect_form(root: &Path) -> Option<&'static str> {
    if root.join("sih-engine/Cargo.toml").is_file()
        && root.join("sih-tools/pyproject.toml").is_file()
    {
        return Some(FORM_FIRST);
    }
    if root.join("sih/ledger").is_dir() {
        return Some(FORM_CANONICAL);
    }
    None
}

/// 两形判别：域根等于中央根即第一域映射形（domains.py 自锚判词）；其余域根
/// 按标记判别（双仓标记俱在即第一域形，sih/ledger 在即新城正典形，俱缺回落
/// 新城正典形——新城一律此形）。
pub fn layout_for(domain_root: &Path, central_root: &Path) -> DomainLayout {
    let d = norm_path(domain_root);
    let c = norm_path(central_root);
    if d == c {
        return DomainLayout { root: c.clone(), form: FORM_FIRST, central_root: c };
    }
    let form = detect_form(&d).unwrap_or(FORM_CANONICAL);
    DomainLayout { root: d, form, central_root: c }
}

/// 缺省域布局：数据根自锚即第一域映射形（writeface/passthrough.default_layout
/// 对等，stdio 面承载形）。
pub fn default_layout(root: &Path) -> DomainLayout {
    let r = norm_path(root);
    layout_for(&r, &r)
}

/// 识别解析位：中央标识登记册（第一域 mcpline 工具面自治位）。数据根逐请求
/// 解析（SIH_ROOT 可变承测试隔离形），全链只认此一册（解析单点律）。
pub fn central_registry_path() -> PathBuf {
    default_layout(&resolve_root()).tokens_path()
}

// ============================================================ Bearer 头解析

/// Authorization Bearer 头提取（HTTP 惯例连接头标准承载位，DES-015 传输形
/// 判词）。首条 Authorization 头定胜负：非 Bearer 形或缺头即 None；前七字节
/// 大小写不敏感含空格方为 Bearer 形，token 去首尾空白，空串视同缺。
pub fn bearer_token_from_headers(headers: &HeaderMap) -> Option<String> {
    // 首条定胜负：get_all 保插入序，只取首条 Authorization 头（Python 首中
    // 即返对等：非 Bearer 形同样终局，不落到后续头）。
    let value = headers.get_all(AUTHORIZATION).iter().next()?;
    let raw = value.to_str().ok()?.trim();
    if raw.len() >= 7 && raw[..7].eq_ignore_ascii_case("bearer ") {
        let token = raw[7..].trim();
        return if token.is_empty() { None } else { Some(token.to_string()) };
    }
    None
}

// ============================================================ 教学载荷

/// 传输层 401/403 教学载荷：四字段契约外加 retry_discipline 与管理台指引
/// （httpface.py _gate_payload 逐字段对等，字段序即 Python dict 序）。
fn gate_payload(status: StatusCode, reason: &str, message: &str, action: &str) -> Value {
    json!({
        "error": message,
        "reason_code": reason,
        "http_status": status.as_u16(),
        "gate": "HTTP 识别与域绑定路由层（DES-015）",
        "what_this_tool_does": "HTTP 写面工具经既有 CLI 执法透传：连接须携 Bearer 项目标识且在册 active，写操作只落所绑域",
        "valid_params": ["Authorization: Bearer <token_id>（连接头携项目标识）"],
        "suggested_action": action,
        "retry_discipline": RETRY_DISCIPLINE,
        "console": "管理台 /tokens（列表、签发、撤销，签发与撤销各带一步确认）",
        "canonical_pointers": CANON_POINTERS,
    })
}

fn missing_header_payload() -> Value {
    gate_payload(
        StatusCode::UNAUTHORIZED,
        "missing_authorization",
        "缺 Authorization 头：连接须带 Bearer 头携项目标识（Authorization: Bearer <token_id>）；无标识连接降只读投影中央域，写工具一律拒透传",
        "查管理台 /tokens 列表取在册标识后携头重连，或经管理台签发",
    )
}

fn unregistered_payload() -> Value {
    gate_payload(
        StatusCode::UNAUTHORIZED,
        "token_unregistered",
        "标识不在册（Bearer 头所携标识未经登记册登记）：查管理台列表对表",
        "经管理台 /tokens 签发后携新标识重连",
    )
}

fn stopped_payload() -> Value {
    gate_payload(
        StatusCode::UNAUTHORIZED,
        "token_stopped",
        "标识已停行（登记册末行 status=stopped）：撤销即停行，末行为准",
        "换牌重连（管理台 /tokens 签发新标识）；已立会话不被 server 强拆，旧会话由既有孤儿清理通道兜底",
    )
}

fn readonly_payload() -> Value {
    gate_payload(
        StatusCode::FORBIDDEN,
        "scope_violation",
        "现行档位 readonly 拒写：readonly 档只读五工具可调，写工具一律拒透传",
        "候管理台换发 domain_write 档标识牌后重连（域写档为缺省授出形）",
    )
}

fn custom_payload() -> Value {
    gate_payload(
        StatusCode::FORBIDDEN,
        "scope_violation",
        "现行档位 custom 零缺省放行：细粒度白名单逐工具显式登记，登记面即授权面；白名单登记形待后继批评审，本批无白名单行即写工具一律拒透传",
        "候管理台换发 domain_write 档标识牌，或候 custom 白名单登记面实装",
    )
}

/// 登记册读取失败载荷（执法解析单点不可读即 fail-closed 拒透传；Python 形
/// 为异常 500，此处收编为结构化载荷，如实申报差异）。
fn registry_error_payload(err: &tokens::TokensLedgerError) -> Value {
    json!({
        "error": "登记册读取失败（识别解析单点不可读，fail-closed 拒透传）",
        "reason_code": "registry_unreadable",
        "http_status": 500,
        "gate": "HTTP 识别与域绑定路由层（DES-015）",
        "detail": err.to_string(),
        "suggested_action": "候 server 侧对登记册账面完好性，账面损坏不静默",
        "canonical_pointers": CANON_POINTERS,
    })
}

// ============================================================ 识别与绑定

/// tools/call 写面十具提取（BearerGateMiddleware._verdict 前半对等）：非
/// tools/call 或 name 不在写面十具即 None（原样透传）；请求体非 JSON 对象
/// 视同放行。
pub fn write_call_tool(msg: &Value) -> Option<&'static str> {
    if msg.get("method").and_then(Value::as_str) != Some("tools/call") {
        return None;
    }
    let name = msg
        .get("params")
        .and_then(|p| p.get("name"))
        .and_then(Value::as_str)?;
    HTTP_WRITE_TOOLS.iter().copied().find(|t| *t == name)
}

/// 写工具逐调用静态裁决（零缓存执法：每笔新读登记册）：None 即放行，
/// Some((status, payload)) 即拒。scope 三档分形：readonly 与 custom 拒写
/// 透传，domain_write 缺省授出形放行。
fn write_gate_verdict(registry: &Path, token: Option<&str>) -> Option<(StatusCode, Value)> {
    let Some(t) = token else {
        return Some((StatusCode::UNAUTHORIZED, missing_header_payload()));
    };
    let row = match tokens::resolve_token(registry, t) {
        Ok(r) => r,
        Err(e) => return Some((StatusCode::INTERNAL_SERVER_ERROR, registry_error_payload(&e))),
    };
    let Some(row) = row else {
        return Some((StatusCode::UNAUTHORIZED, unregistered_payload()));
    };
    if row.status != tokens::STATUS_ACTIVE {
        return Some((StatusCode::UNAUTHORIZED, stopped_payload()));
    }
    if row.scope == tokens::SCOPE_READONLY {
        return Some((StatusCode::FORBIDDEN, readonly_payload()));
    }
    if row.scope == tokens::SCOPE_CUSTOM {
        return Some((StatusCode::FORBIDDEN, custom_payload()));
    }
    None
}

/// 请求标识：token_id 与所绑域根二元（读工具域形式与写工具识别的共用识别
/// 位，对等 _REQUEST_TOKEN ContextVar 的载荷形）。匿名或缺省形 token_id 为
/// None，root 走缺省中央根形（SIH_ROOT 解析，与 stdio 缺省形同源）。
#[derive(Clone, Debug)]
pub struct HttpIdent {
    pub token_id: Option<String>,
    pub root: PathBuf,
}

impl HttpIdent {
    pub fn anonymous() -> Self {
        Self { token_id: None, root: resolve_root() }
    }
}

/// 请求标识解析：无标识或缺省形（不在册或已停行）降只读投影中央域；active
/// 标识按所绑域组装（域内治理自足）。
fn ident_for_token(token: Option<&str>) -> HttpIdent {
    let Some(t) = token else {
        return HttpIdent::anonymous();
    };
    let registry = central_registry_path();
    match tokens::resolve_token(&registry, t) {
        Ok(Some(row)) if row.status == tokens::STATUS_ACTIVE => {
            let layout = layout_for(Path::new(&row.domain_root), &resolve_root());
            HttpIdent { token_id: Some(t.to_string()), root: layout.root }
        }
        _ => HttpIdent::anonymous(),
    }
}

// ============================================================ 会话一对一表

/// 标识牌会话一对一表（DES-015 令牌与会话映射判词的进程内承载，对等
/// SessionTable）：同时刻一标识牌至多一个活跃 lease 会话。同 token_id 复用
/// 同一连接会话对象；换绑域（重签行 domain_root 变更）即旧对象让位新域对象；
/// close 成即解绑可再立（懒清除形：收约标记在 conn 上，下表取用时重建）。
pub struct HttpShared {
    endpoint: String,
    sessions: StdMutex<HashMap<String, Arc<Mutex<ConnectionSession>>>>,
}

impl HttpShared {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            sessions: StdMutex::new(HashMap::new()),
        }
    }

    /// 表取会话：匿名连接每会话独立（写面 401 拒透传，会话对象零执法参与）；
    /// 有牌连接按 token_id 一对一复用，域根变更或已收约即重建。
    pub fn session_conn(&self, ident: &HttpIdent) -> Arc<Mutex<ConnectionSession>> {
        let Some(token_id) = &ident.token_id else {
            return Arc::new(Mutex::new(ConnectionSession::new(AGENT_CLASS_EXTERNAL)));
        };
        let mut table = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(existing) = table.get(token_id).cloned() {
            // 复用条件：域根未变且未收约。在飞借用（他 MCP 会话正在调用）即
            // 保守复用——域根不随会话漂移，竞窗口只覆盖重签换域瞬间。
            let reusable = match existing.try_lock() {
                Ok(guard) => guard.root == ident.root && !guard.closed_by_client,
                Err(_) => true,
            };
            if reusable {
                return existing;
            }
            // 旧对象让位（旧会话归既有孤儿清理通道，server 不强拆）。
            table.remove(token_id);
        }
        let id = format!(
            "mcpconn-{token_id}-{}",
            uuid::Uuid::new_v4().simple().to_string().chars().take(8).collect::<String>()
        );
        let conn = ConnectionSession {
            root: ident.root.clone(),
            agent_class: AGENT_CLASS_EXTERNAL.to_string(),
            connection_id: id.clone(),
            id_component: id,
            identity_report: None,
            identity_error: None,
            session_id: None,
            package: None,
            write_count: 0,
            closed_by_client: false,
            auto_open_error: None,
        };
        let arc = Arc::new(Mutex::new(conn));
        table.insert(token_id.clone(), arc.clone());
        arc
    }
}

// 请求标识上下文（对等 _REQUEST_TOKEN ContextVar）：Bearer 闸于转发前置入，
// 会话工厂读取。task 局部形沿 await 链传播至工厂同步调用点（initialize 时
// 会话创建在请求处理栈内），跨 tokio::spawn 不传播（服务任务持既有实例，
// 无需再读）。
tokio::task_local! {
    static REQUEST_IDENTITY: HttpIdent;
}

// ============================================================ Bearer 闸

/// 纯 axum 识别闸（对等 BearerGateMiddleware：路由层静态裁剪，DES-015 失败
/// 语义传输层映射承载位）。仅拦端点 POST 的 tools/call 写面十具：401 未识别
/// 三因直说（缺头/不在册/已停行）、403 分档拒（readonly 与 custom），载荷直
/// 给教学语；其余请求原样透传。放行前置入请求标识上下文。
async fn bearer_gate(
    State(shared): State<Arc<HttpShared>>,
    req: Request,
    next: Next,
) -> Response {
    let is_target = req.method() == Method::POST
        && req.uri().path().trim_end_matches('/') == shared.endpoint;
    if !is_target {
        return next.run(req).await;
    }
    let token = bearer_token_from_headers(req.headers());
    let (parts, body) = req.into_parts();
    // 请求体收满后以回放 body 转发（下游可整读），整读无上限与 Python 形同源
    // （回环管理面承载边界，DES-015）。
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, format!("请求体读取失败: {e}")).into_response();
        }
    };
    let msg: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    if write_call_tool(&msg).is_some() {
        let registry = central_registry_path();
        if let Some((status, payload)) = write_gate_verdict(&registry, token.as_deref()) {
            return (status, Json(payload)).into_response();
        }
    }
    let ident = ident_for_token(token.as_deref());
    let req = Request::from_parts(parts, Body::from(bytes));
    REQUEST_IDENTITY.scope(ident, next.run(req)).await
}

// ============================================================ MCP 实例装配

/// 启动复核（fail-closed 拒启，对等 assert_http_face）：HTTP 面工具恰 α 九
/// 加写面十；record_direct 与 lease_unclaim 与 takeover 与 bypass 零在列
/// （结构性拒透传承载位）。返回排序去重后的全工具面名单。
pub fn assert_http_face() -> anyhow::Result<Vec<&'static str>> {
    use super::matrix::{EXTERNAL_ALLOWED, MATRIX_ROWS, is_tool_exposed};
    let mut names: Vec<&'static str> = super::server::tool_names().to_vec();
    names.extend(
        MATRIX_ROWS
            .iter()
            .copied()
            .filter(|n| is_tool_exposed(n, AGENT_CLASS_EXTERNAL)),
    );
    names.sort_unstable();
    names.dedup();
    let mut expected: Vec<&'static str> = super::server::tool_names()
        .into_iter()
        .chain(EXTERNAL_ALLOWED)
        .collect();
    expected.sort_unstable();
    expected.dedup();
    let absent_local = ["record_direct", "lease_unclaim", "takeover", "bypass"];
    let present_absent: Vec<&str> = names.iter().copied().filter(|n| absent_local.contains(n)).collect();
    if !present_absent.is_empty() || names != expected {
        anyhow::bail!(
            "HTTP 面工具结构违例，拒启：本地可信位在列 {present_absent:?}；全工具面 {names:?}"
        );
    }
    Ok(names)
}

/// 端点路由装配：nest_service 承 /mcp streamable 端点，StreamableHttpService
/// 工厂每 MCP 会话构造一个 SihMcpServer（连接一会话语义，agent_class 恒
/// external 最小写集十行）；闸层挂 Router 级，路径自筛（非端点 POST 原样
/// 透传）。路由自成一体（乙丙簇可直接并装或经 serve_http 独立起服）。
pub fn streamable_router(endpoint: &str) -> Router {
    let shared = Arc::new(HttpShared::new(endpoint));
    let factory = {
        let shared = shared.clone();
        move || -> Result<SihMcpServer, std::io::Error> {
            let ident = REQUEST_IDENTITY
                .try_with(Clone::clone)
                .unwrap_or_else(|_| HttpIdent::anonymous());
            Ok(SihMcpServer {
                conn: shared.session_conn(&ident),
                agent_class: AGENT_CLASS_EXTERNAL,
            })
        }
    };
    let service = StreamableHttpService::new(
        factory,
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );
    Router::new()
        .nest_service(endpoint, service)
        .layer(middleware::from_fn_with_state(shared, bearer_gate))
        .with_state(())
}

/// HTTP 识别写面起服：fail-closed 启动复核先行，绑址即候请求。
pub async fn serve_http(bind: &str, endpoint: &str) -> anyhow::Result<()> {
    assert_http_face()?;
    // 组装层：/mcp 识别写面加台面（乙簇七路由）加域接入链三路由（丙簇真形），
    // 单进程单端口多协议形（DES-020 部署拓扑判词）。
    let app = streamable_router(endpoint)
        .merge(crate::mcpserver::webface::router(crate::mcpserver::webface::WebState {
            root: crate::mcpserver::runtime::resolve_root(),
        }))
        .merge(crate::mcpserver::bootstrap::router());
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// ============================================================ 测试

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use crate::mcpserver::tokens::{issue_row, revoke_row};

    #[test]
    fn domain_layout_two_forms() {
        let dir = tempfile::tempdir().unwrap();
        let central = dir.path().join("central");
        std::fs::create_dir_all(central.join("sih-engine")).unwrap();
        std::fs::create_dir_all(central.join("sih-tools")).unwrap();
        // 自锚即第一域映射形（default_layout 判词），tokens 落 mcpline 自治位。
        let l = default_layout(&central);
        assert!(l.is_first());
        assert_eq!(l.form, FORM_FIRST);
        assert_eq!(
            l.tokens_path(),
            l.root.join("sih-tools/mcpline/ledger/tokens.ndjson")
        );
        assert_eq!(
            l.trail("2026-09-11"),
            l.root.join("sih-engine/sih/event/trail/2026-09-11.ndjson")
        );
        assert_eq!(
            l.ledger_paths()["locks"],
            l.root.join("sih-tools/lease/ledger/locks.ndjson")
        );
        assert_eq!(l.parking_dir(), l.root.join("sih-engine/sih/state/parking/materials"));
        // 标记判别形：双仓标记俱在即 first（判别形照 domains.py 判据），仓集
        // 双仓根相对形。
        std::fs::write(central.join("sih-engine/Cargo.toml"), "[package]").unwrap();
        std::fs::write(central.join("sih-tools/pyproject.toml"), "").unwrap();
        let l2 = layout_for(&central, Path::new("/nonexistent-central-root"));
        assert!(l2.is_first());
        assert_eq!(l2.repos(), vec!["sih-tools".to_string(), "sih-engine".to_string()]);
        // 新城正典形：sih/ledger 在即 canonical，tokens 落 sih/ledger 位，
        // 仓集域根单仓。
        let dir2 = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir2.path().join("sih/ledger")).unwrap();
        let l3 = layout_for(dir2.path(), &central);
        assert!(!l3.is_first());
        assert_eq!(l3.form, FORM_CANONICAL);
        assert_eq!(l3.tokens_path(), l3.root.join("sih/ledger/tokens.ndjson"));
        assert_eq!(
            l3.ledger_paths()["ledger"],
            l3.root.join("sih/ledger/sessions.ndjson")
        );
        assert_eq!(l3.repos(), vec![l3.root.display().to_string()]);
        // 俱缺标记回落新城正典形（新城一律此形）。
        let dir3 = tempfile::tempdir().unwrap();
        let l4 = layout_for(dir3.path(), &central);
        assert!(!l4.is_first());
        assert_eq!(detect_form(dir3.path()), None);
    }

    #[test]
    fn token_resolution_two_forms() {
        // 第一域形：中央根 sih-tools/mcpline/ledger/tokens.ndjson。
        let dir = tempfile::tempdir().unwrap();
        let first_dir = dir.path().join("sih-tools/mcpline/ledger");
        std::fs::create_dir_all(&first_dir).unwrap();
        let first_path = first_dir.join("tokens.ndjson");
        let row = issue_row("tok-first", dir.path().display().to_string().as_str(), tokens::SCOPE_DOMAIN_WRITE, "tester");
        append_ok(&first_path, &row);
        let got = tokens::resolve_token(&first_path, "tok-first").unwrap().unwrap();
        assert_eq!(got.status, tokens::STATUS_ACTIVE);
        assert_eq!(got.scope, tokens::SCOPE_DOMAIN_WRITE);
        // 末行为准：追加 stopped 行后解析取停行。
        append_ok(&first_path, &revoke_row(&got, "tester"));
        let last = tokens::resolve_token(&first_path, "tok-first").unwrap().unwrap();
        assert_eq!(last.status, tokens::STATUS_STOPPED);
        // 新城正典形：域根 sih/ledger/tokens.ndjson。
        let dir2 = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir2.path().join("sih/ledger")).unwrap();
        let canon_path = dir2.path().join("sih/ledger/tokens.ndjson");
        append_ok(&canon_path, &issue_row("tok-canon", dir2.path().display().to_string().as_str(), tokens::SCOPE_DOMAIN_WRITE, "tester"));
        let got2 = tokens::resolve_token(&canon_path, "tok-canon").unwrap().unwrap();
        assert_eq!(got2.status, tokens::STATUS_ACTIVE);
        // 不在册即 None。
        assert!(tokens::resolve_token(&canon_path, "tok-absent").unwrap().is_none());
    }

    fn append_ok(path: &Path, row: &crate::mcpserver::tokens::TokenRow) {
        tokens::append_row(path, row).unwrap();
    }

    #[test]
    fn bearer_header_parsing() {
        let mut hm = HeaderMap::new();
        assert!(bearer_token_from_headers(&hm).is_none());
        hm.insert(AUTHORIZATION, HeaderValue::from_static("Bearer tok-1"));
        assert_eq!(bearer_token_from_headers(&hm).as_deref(), Some("tok-1"));
        // 小写 bearer 形同认。
        hm.insert(AUTHORIZATION, HeaderValue::from_static("bearer tok-2"));
        assert_eq!(bearer_token_from_headers(&hm).as_deref(), Some("tok-2"));
        // token 首尾空白剥除。
        hm.insert(AUTHORIZATION, HeaderValue::from_static("Bearer   tok-3  "));
        assert_eq!(bearer_token_from_headers(&hm).as_deref(), Some("tok-3"));
        // 非 Bearer 形即 None。
        hm.insert(AUTHORIZATION, HeaderValue::from_static("Basic abc"));
        assert!(bearer_token_from_headers(&hm).is_none());
        // 裸 Bearer 无 token 即 None。
        hm.insert(AUTHORIZATION, HeaderValue::from_static("Bearer "));
        assert!(bearer_token_from_headers(&hm).is_none());
        hm.insert(AUTHORIZATION, HeaderValue::from_static("Bearer"));
        assert!(bearer_token_from_headers(&hm).is_none());
        // 首条 Authorization 定胜负（首条非 Bearer 即 None）。
        let mut hm2 = HeaderMap::new();
        hm2.append(AUTHORIZATION, HeaderValue::from_static("Basic x"));
        hm2.append(AUTHORIZATION, HeaderValue::from_static("Bearer y"));
        assert!(bearer_token_from_headers(&hm2).is_none());
    }

    #[test]
    fn write_gate_verdict_stopped_and_scopes() {
        let dir = tempfile::tempdir().unwrap();
        let registry = dir.path().join("tokens.ndjson");
        for (tid, scope) in [
            ("tok-ok", tokens::SCOPE_DOMAIN_WRITE),
            ("tok-ro", tokens::SCOPE_READONLY),
            ("tok-custom", tokens::SCOPE_CUSTOM),
        ] {
            append_ok(&registry, &issue_row(tid, dir.path().display().to_string().as_str(), scope, "tester"));
        }
        let stopped_base = issue_row("tok-stopped", dir.path().display().to_string().as_str(), tokens::SCOPE_DOMAIN_WRITE, "tester");
        append_ok(&registry, &stopped_base);
        append_ok(&registry, &revoke_row(&stopped_base, "tester"));

        // 缺头即 401 missing_authorization。
        let (status, payload) = write_gate_verdict(&registry, None).unwrap();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(payload["reason_code"], "missing_authorization");
        // 不在册即 401 token_unregistered。
        let (status, payload) = write_gate_verdict(&registry, Some("tok-absent")).unwrap();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(payload["reason_code"], "token_unregistered");
        // 已停行即 401 token_stopped（末行为准）。
        let (status, payload) = write_gate_verdict(&registry, Some("tok-stopped")).unwrap();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(payload["reason_code"], "token_stopped");
        assert_eq!(payload["http_status"], 401);
        assert_eq!(
            payload["error"],
            "标识已停行（登记册末行 status=stopped）：撤销即停行，末行为准"
        );
        // readonly 档即 403 scope_violation。
        let (status, payload) = write_gate_verdict(&registry, Some("tok-ro")).unwrap();
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(payload["reason_code"], "scope_violation");
        assert!(payload["error"].as_str().unwrap().contains("readonly"));
        // custom 档零缺省放行即 403。
        let (status, payload) = write_gate_verdict(&registry, Some("tok-custom")).unwrap();
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(payload["error"].as_str().unwrap().contains("custom"));
        // domain_write 缺省授出形放行。
        assert!(write_gate_verdict(&registry, Some("tok-ok")).is_none());
    }

    #[test]
    fn gate_payload_teaching_shape() {
        let (_, payload) = write_gate_verdict(Path::new("/nonexistent-registry"), None).unwrap();
        // 教学载荷契约面：四字段外加 retry_discipline 与管理台指引（字段逐一
        // 在列，正典指针首位 DES-015）。
        for field in [
            "error",
            "reason_code",
            "http_status",
            "gate",
            "what_this_tool_does",
            "valid_params",
            "suggested_action",
            "retry_discipline",
            "console",
            "canonical_pointers",
        ] {
            assert!(payload.get(field).is_some(), "载荷缺字段 {field}");
        }
        assert_eq!(payload["gate"], "HTTP 识别与域绑定路由层（DES-015）");
        assert_eq!(payload["canonical_pointers"][0], CANON_DES_015);
        assert_eq!(payload["canonical_pointers"][1], CANON_DES_014);
        assert_eq!(payload["canonical_pointers"][2], CANON_SPEC_023);
        assert_eq!(payload["retry_discipline"], RETRY_DISCIPLINE);
        assert!(payload["console"].as_str().unwrap().contains("/tokens"));
    }

    #[test]
    fn write_call_tool_extraction() {
        let call = json!({
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": {"name": "lease_lock", "arguments": {"path": "a"}}
        });
        assert_eq!(write_call_tool(&call), Some("lease_lock"));
        // 读工具不进裁决。
        let read_call = json!({"method": "tools/call", "params": {"name": "chain_query"}});
        assert_eq!(write_call_tool(&read_call), None);
        // 本地可信位与无行名不进裁决。
        let takeover = json!({"method": "tools/call", "params": {"name": "takeover"}});
        assert_eq!(write_call_tool(&takeover), None);
        // 非 tools/call 与非 JSON 放行。
        assert_eq!(write_call_tool(&json!({"method": "initialize"})), None);
        assert_eq!(write_call_tool(&json!({"method": "tools/call"})), None);
        assert_eq!(write_call_tool(&Value::Null), None);
    }

    #[test]
    fn session_table_reuse_and_rebind() {
        let shared = HttpShared::new("/mcp");
        let ident_a = HttpIdent {
            token_id: Some("tok-t".to_string()),
            root: PathBuf::from("/tmp/domain-a"),
        };
        // 同牌复用同一连接会话对象（一对一表）。
        let c1 = shared.session_conn(&ident_a);
        let c2 = shared.session_conn(&ident_a);
        assert!(Arc::ptr_eq(&c1, &c2));
        // 连接标识形：mcpconn-{token_id}-{hex8}。
        let idc = c1.try_lock().unwrap().id_component.clone();
        assert!(idc.starts_with("mcpconn-tok-t-"));
        assert_eq!(c1.try_lock().unwrap().agent_class, AGENT_CLASS_EXTERNAL);
        assert_eq!(c1.try_lock().unwrap().root, PathBuf::from("/tmp/domain-a"));
        // close 成即解绑可再立：收约标记后同牌取新对象（懒清除形）。
        c1.try_lock().unwrap().mark_client_closed();
        let c3 = shared.session_conn(&ident_a);
        assert!(!Arc::ptr_eq(&c1, &c3));
        // 换绑域（重签行 domain_root 变更）即旧对象让位新域对象。
        let ident_b = HttpIdent {
            token_id: Some("tok-t".to_string()),
            root: PathBuf::from("/tmp/domain-b"),
        };
        let c4 = shared.session_conn(&ident_b);
        assert!(!Arc::ptr_eq(&c3, &c4));
        assert_eq!(c4.try_lock().unwrap().root, PathBuf::from("/tmp/domain-b"));
        // 匿名连接走缺省中央根形（每会话独立，表零参与）。
        let anon = shared.session_conn(&HttpIdent::anonymous());
        let guard = anon.try_lock().unwrap();
        assert_eq!(guard.agent_class, AGENT_CLASS_EXTERNAL);
        assert_eq!(guard.root, resolve_root());
    }

    #[test]
    fn http_face_structure_fail_closed() {
        // 启动复核：恰 α 九加写面十；本地可信位四名零在列。
        let names = assert_http_face().unwrap();
        assert_eq!(names.len(), 19);
        for absent in ["record_direct", "lease_unclaim", "takeover", "bypass"] {
            assert!(!names.contains(&absent), "{absent} 不应在列");
        }
        assert_eq!(HTTP_WRITE_TOOLS.len(), 10);
        assert_eq!(HTTP_ALPHA_TOOLS.len(), 9);
        assert_eq!(CANON_POINTERS.len(), 3);
    }

    #[test]
    fn http_face_router_builds() {
        // 装配冒烟（不绑端口，测试零 8765 依赖）：端点路由可构造即可起服。
        let _app = streamable_router("/mcp");
        let _app2 = streamable_router("/mcp-nested/");
    }
}
