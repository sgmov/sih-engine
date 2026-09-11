//! mcpline 统一 web 台面（视图面板 + 令牌管理台 + 说明书）Rust 对等移植。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/web.py。路由集逐一移植：
//! GET /（视图面板入口页）、GET /tokens（令牌管理台：列表 + 签发 + 撤销 +
//! 开域三表单）、GET /manual（AI 使用说明书原文直出）、POST /tokens/issue 与
//! /tokens/confirm-issue（签发两步一步确认）、POST /tokens/revoke 与
//! /tokens/confirm-revoke（撤销两步）、POST /tokens/open 与
//! /tokens/confirm-open（开域两步）。
//!
//! 管理钥回环零防（web.py 判词照录）：无管理钥，本机回环面零防，管理台即
//! 人节点操作位；Python 面 BearerGateMiddleware 只承 /mcp 写面识别闸，台面
//! 路由零防护形照录（本模块零鉴权中间件）。
//!
//! 台账路径固定第一域形：<root>/sih-tools/mcpline/ledger/tokens.ndjson
//! （web.py central_registry_path 经 default_layout(resolve_root()) 数据根
//! 自锚即第一域映射形，同形固定；两形判别位在 httpface，本模块不依赖）。
//!
//! 边界：/mcp MCP streamable HTTP 识别写面不在本模块（httpface 载体位）；
//! 静态资产兜底挂载不在本模块（Rust 面只承载根路径入口页，缺席出教学页）；
//! 开域 confirm 的全链承载位是 bootstrap.run_chain，Rust 侧 bootstrap 载体
//! 未落位前出教学错误页（薄壳单一 canonical 路径零二次实现红线，禁本模块
//! 私铸全链）。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use axum::body::Body;
use axum::extract::{Form, State};
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;

use crate::mcpserver::tokens::{self, TokenRow};

/// 台面签发落款（任务指定 "sihmcp-console"，区分 Python 台面 "console" 落款笔）。
pub const CONSOLE_ISSUED_BY: &str = "sihmcp-console";

/// 视图面板静态目录相对位（web.py VISUAL_SUBPATH 照录，只读引用零写入）。
pub const VISUAL_SUBPATH: &str = "sih-visual/assets/viewer-dashboard-2026-09-06";

/// 根路径入口页文件名（web.py DEFAULT_ENTRY 照录）。
pub const DEFAULT_ENTRY: &str = "dashboard-v3-flow-console.html";

/// AI 使用说明书相对位（web.py manual_file 即包仓根 AI-MANUAL.md，对等工作
/// 区形 <root>/sih-tools/mcpline/AI-MANUAL.md）。
pub const MANUAL_REL: &str = "sih-tools/mcpline/AI-MANUAL.md";
pub const MANUAL_FILENAME: &str = "AI-MANUAL.md";

/// 中央标识登记册相对位（第一域 mcpline 工具面自治位，路径形固定）。
pub const LEDGER_REL: &str = "sih-tools/mcpline/ledger/tokens.ndjson";

/// 开域缺省档位与缺省事由（bootstrap 常量照录，确认页文案对等用）。
pub const DEFAULT_SCOPE: &str = "domain_write";
pub const DEFAULT_OPENED_BY: &str = "main-window-bootstrap";

/// 页面样式（web.py _PAGE_STYLE 照录，单行 CSS）。
const PAGE_STYLE: &str = "body{font-family:-apple-system,sans-serif;max-width:880px;margin:32px auto;\
padding:0 16px;color:#222}table{border-collapse:collapse;width:100%}td,th{border:1px solid #ccc;\
padding:6px 10px;text-align:left;font-size:14px}th{background:#f4f4f4}code{background:#f4f4f4;\
padding:1px 5px}form{margin:12px 0;padding:12px;border:1px solid #ddd;border-radius:6px}\
input,select{margin:2px 8px 2px 0;padding:4px}.btn{padding:6px 14px;cursor:pointer}.warn{color:#a00}\
h2{border-bottom:2px solid #eee;padding-bottom:6px}";

/// 档位下拉选项（签发与开域两表单共用，web.py 同段照录）。
const SCOPE_OPTIONS: &str = "<option value=\"domain_write\">domain_write（域写档）</option>\
<option value=\"readonly\">readonly（只读档）</option>\
<option value=\"custom\">custom（零缺省放行）</option>";

/// 台面态：数据根。台账与说明书与面板入口页皆自此解析。
#[derive(Clone, Debug)]
pub struct WebState {
    pub root: PathBuf,
}

// ------------------------------------------------------------------ 路径解析

/// 中央标识登记册路径（第一域形固定，解析单点）。
pub fn registry_path(root: &Path) -> PathBuf {
    root.join(LEDGER_REL)
}

/// AI 使用说明书路径。
pub fn manual_path(root: &Path) -> PathBuf {
    root.join(MANUAL_REL)
}

/// 根路径入口页路径（面板静态目录 + 入口文件名）。
pub fn entry_page_path(root: &Path) -> PathBuf {
    root.join(VISUAL_SUBPATH).join(DEFAULT_ENTRY)
}

/// 词法归一：先试 canonicalize（对等 Python resolve 的 symlink 解析），缺席
/// 即退词法清理（剥 . 与 ..，对等 normpath）。零写盘。
fn normalize_path(p: &Path) -> PathBuf {
    if let Ok(c) = p.canonicalize() {
        return c;
    }
    let mut out = PathBuf::new();
    for comp in p.components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::RootDir => out = PathBuf::from("/"),
            other => out.push(other.as_os_str()),
        }
    }
    if out.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        out
    }
}

/// 域根解析（web.py _resolve_domain_root 对等）：空形回落数据根；相对形并
/// 到数据根下；末位归一。
pub fn resolve_domain_root(base: &Path, domain_root: &str) -> PathBuf {
    let dr = domain_root.trim();
    if dr.is_empty() {
        return base.to_path_buf();
    }
    let p = Path::new(dr);
    let joined = if p.is_absolute() { p.to_path_buf() } else { base.join(p) };
    normalize_path(&joined)
}

// ------------------------------------------------------------------ HTML 形

/// HTML 转义（Python html.escape 对等：& < > " ' 五符）。
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// 页壳（web.py _page 对等）：标题 + 样式 + 正文 + 三链接页脚。
fn html_response(status: StatusCode, title: &str, body: &str) -> Response {
    let html = format!(
        "<!doctype html><html lang=\"zh\"><head><meta charset=\"utf-8\">\
<title>{t}</title><style>{style}</style></head><body><h1>{t}</h1>{body}\
<p><a href=\"/manual\">AI 使用说明书</a> · <a href=\"/tokens\">返回管理台</a> · \
<a href=\"/\">返回面板</a></p></body></html>",
        t = escape_html(title),
        style = PAGE_STYLE,
        body = body,
    );
    (status, Html(html)).into_response()
}

/// 200 页。
fn page(title: &str, body: &str) -> Response {
    html_response(StatusCode::OK, title, body)
}

/// 错误页（web.py _error_page 对等：400 + 报文 + 处置语）。
fn error_page(message: &str, action: &str) -> Response {
    let body = format!(
        "<p class=\"warn\">{m}</p><p>处置：{a}</p>",
        m = escape_html(message),
        a = escape_html(action),
    );
    html_response(StatusCode::BAD_REQUEST, "管理台输入被拒", &body)
}

/// 独立教学页（无页壳形，web.py 说明书缺席页同构）：根路径面板缺席等请求位
/// 故障用（Python 在组装位 fail-fast，Rust 面组装零绑盘，缺席迁请求位如实出）。
fn bare_page(status: StatusCode, title: &str, body: &str) -> Response {
    let html = format!(
        "<!doctype html><html lang=\"zh\"><head><meta charset=\"utf-8\">\
<title>{t}</title></head><body>{body}</body></html>",
        t = escape_html(title),
        body = body,
    );
    (status, Html(html)).into_response()
}

/// 登记册末行读数直呈（web.py _render_rows 对等）：按 token_id 字典序，
/// 非 active 行载 warn 类标。
pub fn render_rows(rows: &BTreeMap<String, TokenRow>) -> String {
    if rows.is_empty() {
        return "<p>登记册空册（零标识在册）。</p>".to_string();
    }
    let mut lines = vec![
        "<table><tr><th>token_id</th><th>domain_root</th><th>scope</th>\
<th>status</th><th>issued_at</th><th>issued_by</th></tr>"
            .to_string(),
    ];
    for (tid, r) in rows {
        let mark = if r.status == tokens::STATUS_ACTIVE { "" } else { " class=\"warn\"" };
        lines.push(format!(
            "<tr{mark}><td><code>{tid}</code></td><td>{dom}</td><td>{scope}</td>\
<td>{status}</td><td>{iat}</td><td>{iby}</td></tr>",
            mark = mark,
            tid = escape_html(tid),
            dom = escape_html(&r.domain_root),
            scope = escape_html(&r.scope),
            status = escape_html(&r.status),
            iat = escape_html(&r.issued_at),
            iby = escape_html(&r.issued_by),
        ));
    }
    lines.push("</table>".to_string());
    lines.join("\n")
}

/// 管理台正文（web.py console_page body 对等）：登记册位 + 列表 + 三表单。
pub fn console_body(registry_display: &str, rows: &BTreeMap<String, TokenRow>) -> String {
    format!(
        "<p>登记册：<code>{reg}</code>（明文登记明文可查，读数取每 token_id 末行）</p>\
<h2>在册标识</h2>{rows_html}\
<h2>签发（一步确认）</h2>\
<form method=\"post\" action=\"/tokens/issue\">标识 \
<input name=\"token_id\" placeholder=\"可读短串，如 sinfer\">域根 \
<input name=\"domain_root\" size=\"48\" placeholder=\"/path/to/project（新城）或留空即中央第一域\">\
档位 <select name=\"scope\">{options}</select>\
<button class=\"btn\" type=\"submit\">签发→确认</button></form>\
<h2>撤销（一步确认，停行即追加行）</h2>\
<form method=\"post\" action=\"/tokens/revoke\">标识 \
<input name=\"token_id\" placeholder=\"待停行 token_id\">\
<button class=\"btn\" type=\"submit\">撤销→确认</button></form>\
<h2>开域（域自举全链一步确认：签发加开域加镜像落位）</h2>\
<form method=\"post\" action=\"/tokens/open\">域根 \
<input name=\"domain_root\" size=\"44\" placeholder=\"/path/to/project（新城 git 仓根）\">标识 \
<input name=\"token_id\" placeholder=\"缺 active 行时必填（签发新牌）\">\
档位 <select name=\"scope\">{options}</select>事由 \
<input name=\"by\" size=\"20\" placeholder=\"开域事由\">\
<button class=\"btn\" type=\"submit\">开域→确认</button></form>",
        reg = escape_html(registry_display),
        rows_html = render_rows(rows),
        options = SCOPE_OPTIONS,
    )
}

// ------------------------------------------------------------------ 表单解析

/// 签发表单（web.py 表单参数形：token_id/domain_root/scope；缺字段按空串落）。
#[derive(Debug, Clone, Deserialize)]
pub struct IssueForm {
    #[serde(default)]
    pub token_id: String,
    #[serde(default)]
    pub domain_root: String,
    #[serde(default)]
    pub scope: String,
}

/// 签发表单复核通过值。
#[derive(Debug, Clone, PartialEq)]
pub struct IssueValues {
    pub token_id: String,
    pub domain_root: String,
    pub scope: String,
}

/// 撤销表单（web.py 参数形：token_id）。
#[derive(Debug, Clone, Deserialize)]
pub struct RevokeForm {
    #[serde(default)]
    pub token_id: String,
}

/// 开域表单（web.py 参数形：domain_root/token_id/scope/by）。
#[derive(Debug, Clone, Deserialize)]
pub struct OpenForm {
    #[serde(default)]
    pub domain_root: String,
    #[serde(default)]
    pub token_id: String,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub by: String,
}

/// 开域表单复核通过值。
#[derive(Debug, Clone, PartialEq)]
pub struct OpenValues {
    pub domain_root: String,
    pub token_id: String,
    pub scope: String,
    pub by: String,
}

/// 签发表单复核（web.py _validate_issue_form 对等）：标识必填，scope 在档。
pub fn validate_issue_form(form: &IssueForm) -> Result<IssueValues, (String, String)> {
    let token_id = form.token_id.trim();
    let domain_root = form.domain_root.trim();
    let scope = form.scope.trim();
    if token_id.is_empty() {
        return Err(("标识缺席".to_string(), "补 token_id（可读短串）".to_string()));
    }
    if !tokens::VALID_SCOPES.contains(&scope) {
        return Err((format!("scope 非法：'{scope}'"), "scope 须三档之一".to_string()));
    }
    Ok(IssueValues {
        token_id: token_id.to_string(),
        domain_root: domain_root.to_string(),
        scope: scope.to_string(),
    })
}

/// 开域表单复核（web.py _validate_open_form 对等）：域根必填，scope 在档，
/// 空形回落 domain_write 缺省档；标识与事由可空（空形由链教学）。
pub fn validate_open_form(form: &OpenForm) -> Result<OpenValues, (String, String)> {
    let domain_root = form.domain_root.trim();
    let token_id = form.token_id.trim();
    let scope = match form.scope.trim() {
        "" => DEFAULT_SCOPE,
        s => s,
    };
    let by = form.by.trim();
    if domain_root.is_empty() {
        return Err(("域根缺席".to_string(), "补 domain_root（新城 git 仓根目录）".to_string()));
    }
    if !tokens::VALID_SCOPES.contains(&scope) {
        return Err((format!("scope 非法：'{scope}'"), "scope 须三档之一".to_string()));
    }
    Ok(OpenValues {
        domain_root: domain_root.to_string(),
        token_id: token_id.to_string(),
        scope: scope.to_string(),
        by: by.to_string(),
    })
}

/// 本域根 active 末行读数位（bootstrap.active_row_for 台面读形对等，只读）：
/// 取首个 status active 且归一域根相合的行；零 active 即 None。
pub fn active_row_for(rows: &BTreeMap<String, TokenRow>, domain_root: &Path) -> Option<TokenRow> {
    let want = normalize_path(domain_root);
    rows.values()
        .find(|r| r.status == tokens::STATUS_ACTIVE && normalize_path(Path::new(&r.domain_root)) == want)
        .cloned()
}

// ------------------------------------------------------------------ 路由处理器

/// GET /：视图面板入口页（只读引用形，静态资产零写入）。
async fn root_page(State(state): State<WebState>) -> Response {
    let entry = entry_page_path(&state.root);
    let static_dir = state.root.join(VISUAL_SUBPATH);
    match std::fs::read_to_string(&entry) {
        Ok(text) => (StatusCode::OK, Html(text)).into_response(),
        Err(e) => {
            let msg = if !static_dir.is_dir() {
                format!("静态目录缺席（只读引用形，零创建）: {}", static_dir.display())
            } else {
                format!("根路径入口页缺席: {}（{e}）", entry.display())
            };
            bare_page(
                StatusCode::INTERNAL_SERVER_ERROR,
                "面板缺席",
                &format!(
                    "<p>{m}</p><p>处置：候面板静态资产位就绪后重试；现役路径 \
Python 台面 <code>python -m mcpline.web</code>。</p>",
                    m = escape_html(&msg),
                ),
            )
        }
    }
}

/// GET /tokens：令牌管理台列表动作（登记册末行读数直呈 + 三表单）。
async fn console_page(State(state): State<WebState>) -> Response {
    let registry = registry_path(&state.root);
    let rows = match tokens::load_last_rows(&registry) {
        Ok(rows) => rows,
        Err(e) => return error_page(&e.to_string(), "登记册行形损坏候人节点处置，勿手工改册"),
    };
    let rel = registry
        .strip_prefix(&state.root)
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|_| registry.clone());
    let body = console_body(&rel.display().to_string(), &rows);
    page("令牌管理台", &body)
}

/// GET /manual：AI 使用说明书只读 GET，原文 markdown 直出；缺席出教学语指
/// 仓内位（web.py manual_page 对等）。
async fn manual_page(State(state): State<WebState>) -> Response {
    let mf = manual_path(&state.root);
    match std::fs::read_to_string(&mf) {
        Ok(text) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/markdown; charset=utf-8")
            .body(Body::from(text))
            .unwrap_or_else(|e| bare_page(StatusCode::INTERNAL_SERVER_ERROR, "响应构造异常", &escape_html(&e.to_string()))),
        Err(_) => bare_page(
            StatusCode::NOT_FOUND,
            "说明书缺席",
            &format!(
                "<p>AI 使用说明书文件缺席（预期位 <code>{fname}</code>，\
仓内位 <code>sih-tools/mcpline/{fname}</code>)。\
候部署面补件后重试，勿手工另立副本。</p>",
                fname = MANUAL_FILENAME,
            ),
        ),
    }
}

/// POST /tokens/issue：签发第一步确认面（显示标识与所绑域与档位，零写入）。
async fn issue_action(State(state): State<WebState>, Form(form): Form<IssueForm>) -> Response {
    let v = match validate_issue_form(&form) {
        Ok(v) => v,
        Err((m, a)) => return error_page(&m, &a),
    };
    let resolved = resolve_domain_root(&state.root, &v.domain_root);
    let body = format!(
        "<h2>签发确认（一步确认防手滑）</h2>\
<p>标识 <code>{tid}</code> · 所绑域根 <code>{res}</code> · 档位 <code>{scope}</code></p>\
<p>确认即向登记册追加一行（签发即台账追加行，明文可查）。</p>\
<form method=\"post\" action=\"/tokens/confirm-issue\">\
<input type=\"hidden\" name=\"token_id\" value=\"{tid}\">\
<input type=\"hidden\" name=\"domain_root\" value=\"{res}\">\
<input type=\"hidden\" name=\"scope\" value=\"{scope}\">\
<button class=\"btn\" type=\"submit\">确认签发</button>\
<a href=\"/tokens\"><button class=\"btn\" type=\"button\">取消</button></a></form>",
        tid = escape_html(&v.token_id),
        res = escape_html(&resolved.display().to_string()),
        scope = escape_html(&v.scope),
    );
    page("签发确认", &body)
}

/// POST /tokens/confirm-issue：签发第二步落笔，动作即台账追加行（status
/// active，已 active 重发即拒）。
async fn confirm_issue_action(State(state): State<WebState>, Form(form): Form<IssueForm>) -> Response {
    let v = match validate_issue_form(&form) {
        Ok(v) => v,
        Err((m, a)) => return error_page(&m, &a),
    };
    let resolved = resolve_domain_root(&state.root, &v.domain_root).display().to_string();
    let registry = registry_path(&state.root);
    let rows = match tokens::load_last_rows(&registry) {
        Ok(rows) => rows,
        Err(e) => return error_page(&e.to_string(), "按报文改表单后重试"),
    };
    if rows.get(&v.token_id).map(|r| r.status == tokens::STATUS_ACTIVE).unwrap_or(false) {
        return error_page(
            &format!("标识 {} 已在册 active", v.token_id),
            "已在册标识勿重发；撤销后可重发，或径用现牌",
        );
    }
    let row = match tokens::append_row(
        &registry,
        &tokens::issue_row(&v.token_id, &resolved, &v.scope, CONSOLE_ISSUED_BY),
    ) {
        Ok(row) => row,
        Err(e) => return error_page(&e.to_string(), "按报文改表单后重试"),
    };
    let body = format!(
        "<p class=\"warn\">签发完成（台账已追加一行）。</p>\
<table><tr><th>token_id</th><th>domain_root</th><th>scope</th>\
<th>status</th><th>issued_at</th><th>issued_by</th></tr>\
<tr><td><code>{tid}</code></td><td>{dom}</td><td>{scope}</td><td>{status}</td>\
<td>{iat}</td><td>{iby}</td></tr></table>\
<p>客户端连接形：<code>Authorization: Bearer {tid}</code>（写操作只落所绑域）</p>",
        tid = escape_html(&row.token_id),
        dom = escape_html(&row.domain_root),
        scope = escape_html(&row.scope),
        status = escape_html(&row.status),
        iat = escape_html(&row.issued_at),
        iby = escape_html(&row.issued_by),
    );
    page("签发完成", &body)
}

/// POST /tokens/revoke：撤销第一步确认面（显示待停行标识与其现行绑域与档位）。
async fn revoke_action(State(state): State<WebState>, Form(form): Form<RevokeForm>) -> Response {
    let token_id = form.token_id.trim();
    if token_id.is_empty() {
        return error_page("标识缺席", "补待停行 token_id");
    }
    let registry = registry_path(&state.root);
    let rows = match tokens::load_last_rows(&registry) {
        Ok(rows) => rows,
        Err(e) => return error_page(&e.to_string(), "登记册行形损坏候人节点处置，勿手工改册"),
    };
    let row = match rows.get(token_id) {
        Some(row) => row.clone(),
        None => return error_page("标识不在册", "查管理台列表对表"),
    };
    if row.status == tokens::STATUS_STOPPED {
        return error_page("标识末行已停行", "无需重复撤销；换牌签发走签发动作");
    }
    let body = format!(
        "<h2>撤销确认（一步确认防手滑）</h2>\
<p>标识 <code>{tid}</code> · 所绑域根 <code>{dom}</code> · 档位 <code>{scope}</code></p>\
<p>确认即追加停行行（同 token_id 载 status stopped，末行为准）；\
停行后该标识下笔写操作即拒透传，已立会话不被 server 强拆。</p>\
<form method=\"post\" action=\"/tokens/confirm-revoke\">\
<input type=\"hidden\" name=\"token_id\" value=\"{tid}\">\
<button class=\"btn\" type=\"submit\">确认撤销</button>\
<a href=\"/tokens\"><button class=\"btn\" type=\"button\">取消</button></a></form>",
        tid = escape_html(&row.token_id),
        dom = escape_html(&row.domain_root),
        scope = escape_html(&row.scope),
    );
    page("撤销确认", &body)
}

/// POST /tokens/confirm-revoke：撤销第二步落笔，动作即台账追加停行行。
async fn confirm_revoke_action(State(state): State<WebState>, Form(form): Form<RevokeForm>) -> Response {
    let token_id = form.token_id.trim();
    if token_id.is_empty() {
        return error_page("标识缺席", "补待停行 token_id");
    }
    let registry = registry_path(&state.root);
    let rows = match tokens::load_last_rows(&registry) {
        Ok(rows) => rows,
        Err(e) => return error_page(&e.to_string(), "按报文处置后重试"),
    };
    let row = match rows.get(token_id) {
        Some(row) => row.clone(),
        None => return error_page("标识不在册", "查管理台列表对表"),
    };
    let stopped = match tokens::append_row(&registry, &tokens::revoke_row(&row, CONSOLE_ISSUED_BY)) {
        Ok(row) => row,
        Err(e) => return error_page(&e.to_string(), "按报文处置后重试"),
    };
    let body = format!(
        "<p class=\"warn\">撤销完成（停行行已追加，末行为准）。</p>\
<p>标识 <code>{tid}</code> 现行 status <code>{status}</code>；持牌连接的下笔写操作\
即拒透传（401 教学语），客户端换牌重连。</p>",
        tid = escape_html(&stopped.token_id),
        status = escape_html(&stopped.status),
    );
    page("撤销完成", &body)
}

/// 退役壳（承 bootstrap 簇真形保留教学面）：开域两步原壳，直调测试仍在，
/// 注册面已让位 bootstrap::router 单一 canonical 路径。
#[allow(dead_code)]
/// POST /tokens/open：开域第一步确认面（域根与签发计划与档位待复核，确认前
/// 零写入）。
async fn open_action(State(state): State<WebState>, Form(form): Form<OpenForm>) -> Response {
    let v = match validate_open_form(&form) {
        Ok(v) => v,
        Err((m, a)) => return error_page(&m, &a),
    };
    let resolved = resolve_domain_root(&state.root, &v.domain_root);
    let registry = registry_path(&state.root);
    let rows = match tokens::load_last_rows(&registry) {
        Ok(rows) => rows,
        Err(e) => return error_page(&e.to_string(), "登记册行形损坏候人节点处置，勿手工改册"),
    };
    let plan = match active_row_for(&rows, &resolved) {
        Some(active) => format!("复用现牌 <code>{}</code>（不重签）", escape_html(&active.token_id)),
        None if !v.token_id.is_empty() => {
            format!(
                "将签发新牌 <code>{}</code>（档位 {}）",
                escape_html(&v.token_id),
                escape_html(&v.scope)
            )
        }
        None => {
            return error_page(
                "本域根无 active 标识牌行，全链签发须标识词形",
                "表单补 token_id（可读短串）后重提交",
            )
        }
    };
    let by_display = if v.by.is_empty() { DEFAULT_OPENED_BY } else { v.by.as_str() };
    let body = format!(
        "<h2>开域确认（一步确认防手滑）</h2>\
<p>域根 <code>{res}</code> · 档位 <code>{scope}</code> · 事由 <code>{by}</code></p>\
<p>签发计划：{plan}</p>\
<p>确认即执行域自举全链（签发行落册加正典树落地加开域首笔落链加\
验域加域内镜像行），真链写入。</p>\
<form method=\"post\" action=\"/tokens/confirm-open\">\
<input type=\"hidden\" name=\"domain_root\" value=\"{res}\">\
<input type=\"hidden\" name=\"token_id\" value=\"{tid}\">\
<input type=\"hidden\" name=\"scope\" value=\"{scope}\">\
<input type=\"hidden\" name=\"by\" value=\"{by}\">\
<button class=\"btn\" type=\"submit\">确认开域</button>\
<a href=\"/tokens\"><button class=\"btn\" type=\"button\">取消</button></a></form>",
        res = escape_html(&resolved.display().to_string()),
        scope = escape_html(&v.scope),
        by = escape_html(by_display),
        plan = plan,
        tid = escape_html(&v.token_id),
    );
    page("开域确认", &body)
}

/// POST /tokens/confirm-open：开域第二步落笔位。Python 薄壳调
/// bootstrap.run_chain 单一 canonical 路径；Rust 侧 bootstrap 载体未落位
/// （段3 编组 stub），零二次实现红线禁本模块私铸全链，故出教学错误页如实
/// 指现役路径。
#[allow(dead_code)]
async fn confirm_open_action(State(_state): State<WebState>, Form(form): Form<OpenForm>) -> Response {
    if let Err((m, a)) = validate_open_form(&form) {
        return error_page(&m, &a);
    }
    error_page(
        "开域全链在 Rust 面承载位未就绪（bootstrap 载体待落位）",
        "候 bootstrap 载体就绪；现役路径 Python 台面 python -m mcpline.web 的 /tokens/open",
    )
}

// ------------------------------------------------------------------ 应用组装

/// 台面路由组装（web.py create_app 台面段对等）：七路由，管理钥回环零防。
/// 开域两路由（/tokens/open 与 /tokens/confirm-open）承 bootstrap 簇真形
/// （router() 同名路由，单一 canonical 路径零二次实现），本簇两步壳退役；
/// /mcp 识别写面与静态资产兜底挂载在组装层另位（httpface/server 载体）。
pub fn router(state: WebState) -> axum::Router {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/", get(root_page))
        .route("/tokens", get(console_page))
        .route("/manual", get(manual_page))
        .route("/tokens/issue", post(issue_action))
        .route("/tokens/confirm-issue", post(confirm_issue_action))
        .route("/tokens/revoke", post(revoke_action))
        .route("/tokens/confirm-revoke", post(confirm_revoke_action))
        .with_state(state)
}

// ------------------------------------------------------------------ 测试

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::Response;

    /// 临时台面根：唯一目录 + 台账父目录在位（append_row 目录须在位形）。
    fn temp_root(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("sihmcp-webface-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("sih-tools/mcpline/ledger")).unwrap();
        d
    }

    async fn body_text(resp: Response) -> String {
        let (_parts, body) = resp.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    #[test]
    fn escape_html_basics() {
        assert_eq!(escape_html("a<b>&\"'"), "a&lt;b&gt;&amp;&quot;&#x27;");
        assert_eq!(escape_html("plain"), "plain");
    }

    #[tokio::test]
    async fn page_contains_title_style_and_footer_links() {
        let resp = page("测试页", "<p>正文锚</p>");
        assert_eq!(resp.status(), StatusCode::OK);
        let text = body_text(resp).await;
        assert!(text.contains("<title>测试页</title>"));
        assert!(text.contains("<h1>测试页</h1>"));
        assert!(text.contains(PAGE_STYLE));
        assert!(text.contains("href=\"/manual\""));
        assert!(text.contains("href=\"/tokens\""));
        assert!(text.contains("href=\"/\""));
        assert!(text.contains("AI 使用说明书"));
        assert!(text.contains("<p>正文锚</p>"));
    }

    #[tokio::test]
    async fn error_page_carries_message_and_action() {
        let resp = error_page("坏输入", "改成好的");
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let text = body_text(resp).await;
        assert!(text.contains("管理台输入被拒"));
        assert!(text.contains("坏输入"));
        assert!(text.contains("处置：改成好的"));
    }

    #[test]
    fn validate_issue_form_rejects_missing_token_and_bad_scope() {
        let err = validate_issue_form(&IssueForm {
            token_id: "  ".into(),
            domain_root: "".into(),
            scope: "readonly".into(),
        })
        .unwrap_err();
        assert_eq!(err, ("标识缺席".into(), "补 token_id（可读短串）".into()));

        let err = validate_issue_form(&IssueForm {
            token_id: "t1".into(),
            domain_root: "".into(),
            scope: "root".into(),
        })
        .unwrap_err();
        assert_eq!(err.0, "scope 非法：'root'");
        assert_eq!(err.1, "scope 须三档之一");

        let v = validate_issue_form(&IssueForm {
            token_id: " t1 ".into(),
            domain_root: " /tmp/p ".into(),
            scope: "domain_write".into(),
        })
        .unwrap();
        assert_eq!(v.token_id, "t1");
        assert_eq!(v.domain_root, "/tmp/p");
        assert_eq!(v.scope, "domain_write");
    }

    #[test]
    fn validate_open_form_defaults_and_rejects() {
        let err = validate_open_form(&OpenForm {
            domain_root: "".into(),
            token_id: "".into(),
            scope: "".into(),
            by: "".into(),
        })
        .unwrap_err();
        assert_eq!(err, ("域根缺席".into(), "补 domain_root（新城 git 仓根目录）".into()));

        let v = validate_open_form(&OpenForm {
            domain_root: "/tmp/p".into(),
            token_id: "".into(),
            scope: "".into(),
            by: "  ".into(),
        })
        .unwrap();
        assert_eq!(v.scope, DEFAULT_SCOPE);
        assert_eq!(v.by, "");

        let err = validate_open_form(&OpenForm {
            domain_root: "/tmp/p".into(),
            token_id: "".into(),
            scope: "nope".into(),
            by: "".into(),
        })
        .unwrap_err();
        assert_eq!(err.0, "scope 非法：'nope'");
    }

    #[test]
    fn resolve_domain_root_forms() {
        let base = PathBuf::from("/tmp/sih-root");
        // 空形回落数据根
        assert_eq!(resolve_domain_root(&base, ""), base);
        assert_eq!(resolve_domain_root(&base, "  "), base);
        // 相对形并到数据根下
        assert_eq!(resolve_domain_root(&base, "proj/sub"), base.join("proj/sub"));
        // 绝对形词法归一剥 ..
        assert_eq!(resolve_domain_root(&base, "/tmp/sih-root/./x/../y"), PathBuf::from("/tmp/sih-root/y"));
    }

    #[test]
    fn render_rows_empty_and_filled() {
        let empty: BTreeMap<String, TokenRow> = BTreeMap::new();
        assert_eq!(render_rows(&empty), "<p>登记册空册（零标识在册）。</p>");

        let mut rows = BTreeMap::new();
        rows.insert(
            "t1".to_string(),
            tokens::issue_row("t1", "/tmp/p", "readonly", "tester"),
        );
        let mut stopped = tokens::issue_row("t0", "/tmp/q", "custom", "tester");
        stopped.status = tokens::STATUS_STOPPED.to_string();
        rows.insert("t0".to_string(), stopped);

        let html = render_rows(&rows);
        assert!(html.contains("<th>token_id</th><th>domain_root</th><th>scope</th>"));
        assert!(html.contains("<th>status</th><th>issued_at</th><th>issued_by</th>"));
        assert!(html.contains("<td><code>t1</code></td>"));
        assert!(html.contains("<td><code>t0</code></td>"));
        // 非 active 行载 warn 类标（t0 在字典序先位）
        let t0_pos = html.find("<code>t0</code>").unwrap();
        let warn_pos = html.find("class=\"warn\"").unwrap();
        assert!(warn_pos < t0_pos);
        // active 行无 warn 标
        let t1_seg = &html[t0_pos..];
        assert!(!t1_seg.contains("class=\"warn\""));
    }

    #[test]
    fn console_body_contains_registry_and_three_forms() {
        let rows: BTreeMap<String, TokenRow> = BTreeMap::new();
        let html = console_body("sih-tools/mcpline/ledger/tokens.ndjson", &rows);
        assert!(html.contains("<code>sih-tools/mcpline/ledger/tokens.ndjson</code>"));
        assert!(html.contains("登记册空册"));
        // 三表单动作路由与字段名逐一在档
        assert!(html.contains("action=\"/tokens/issue\""));
        assert!(html.contains("action=\"/tokens/revoke\""));
        assert!(html.contains("action=\"/tokens/open\""));
        assert!(html.contains("name=\"token_id\""));
        assert!(html.contains("name=\"domain_root\""));
        assert!(html.contains("name=\"scope\""));
        assert!(html.contains("name=\"by\""));
        assert!(html.contains("domain_write（域写档）"));
    }

    #[test]
    fn paths_are_first_domain_form() {
        let root = PathBuf::from("/tmp/sih");
        assert_eq!(registry_path(&root), root.join("sih-tools/mcpline/ledger/tokens.ndjson"));
        assert_eq!(manual_path(&root), root.join("sih-tools/mcpline/AI-MANUAL.md"));
        assert_eq!(
            entry_page_path(&root),
            root.join("sih-visual/assets/viewer-dashboard-2026-09-06/dashboard-v3-flow-console.html")
        );
    }

    #[tokio::test]
    async fn issue_then_duplicate_then_revoke_roundtrip() {
        let root = temp_root("roundtrip");
        let state = WebState { root: root.clone() };

        // 签发确认落笔：追加一行 active，落款 sihmcp-console
        let resp = confirm_issue_action(
            State(state.clone()),
            Form(IssueForm {
                token_id: "sinfer".into(),
                domain_root: "proj".into(),
                scope: "domain_write".into(),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let text = body_text(resp).await;
        assert!(text.contains("签发完成"));
        assert!(text.contains("Authorization: Bearer sinfer"));
        assert!(text.contains(CONSOLE_ISSUED_BY));
        let registry = registry_path(&root);
        let rows = tokens::load_last_rows(&registry).unwrap();
        let row = rows.get("sinfer").unwrap();
        assert_eq!(row.status, tokens::STATUS_ACTIVE);
        assert_eq!(row.domain_root, resolve_domain_root(&root, "proj").display().to_string());
        assert_eq!(row.issued_by, CONSOLE_ISSUED_BY);

        // 已 active 重发即拒（400 教学页）
        let resp = confirm_issue_action(
            State(state.clone()),
            Form(IssueForm {
                token_id: "sinfer".into(),
                domain_root: "".into(),
                scope: "readonly".into(),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let text = body_text(resp).await;
        assert!(text.contains("已在册 active"));

        // 撤销确认面：显示现行绑域与档位，零写入
        let resp = revoke_action(State(state.clone()), Form(RevokeForm { token_id: "sinfer".into() })).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let text = body_text(resp).await;
        assert!(text.contains("撤销确认"));
        assert!(text.contains("action=\"/tokens/confirm-revoke\""));
        assert_eq!(tokens::load_last_rows(&registry).unwrap()["sinfer"].status, tokens::STATUS_ACTIVE);

        // 撤销落笔：追加停行行，末行为准
        let resp =
            confirm_revoke_action(State(state.clone()), Form(RevokeForm { token_id: "sinfer".into() })).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let text = body_text(resp).await;
        assert!(text.contains("撤销完成"));
        assert!(text.contains("status <code>stopped</code>"));
        let rows = tokens::load_last_rows(&registry).unwrap();
        assert_eq!(rows["sinfer"].status, tokens::STATUS_STOPPED);
        assert_eq!(rows["sinfer"].issued_by, CONSOLE_ISSUED_BY);

        // 在册外标识撤销即拒
        let resp = confirm_revoke_action(State(state.clone()), Form(RevokeForm { token_id: "ghost".into() }))
            .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let text = body_text(resp).await;
        assert!(text.contains("标识不在册"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn manual_page_serves_markdown_or_teaches_absence() {
        let root = temp_root("manual");
        let state = WebState { root: root.clone() };

        // 缺席：404 教学语指仓内位
        let resp = manual_page(State(state.clone())).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let text = body_text(resp).await;
        assert!(text.contains("说明书缺席"));
        assert!(text.contains("sih-tools/mcpline/AI-MANUAL.md"));

        // 在位：原文 text/markdown 直出
        let mf = manual_path(&root);
        std::fs::write(&mf, "# AI 手册\n正文").unwrap();
        let resp = manual_page(State(state)).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/markdown; charset=utf-8"
        );
        let text = body_text(resp).await;
        assert!(text.contains("# AI 手册"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn root_page_serves_entry_or_teaches_absence() {
        let root = temp_root("rootpage");
        let state = WebState { root: root.clone() };

        // 静态目录缺席：500 教学页（Python 在组装位 fail-fast，Rust 面迁请求位）
        let resp = root_page(State(state.clone())).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let text = body_text(resp).await;
        assert!(text.contains("静态目录缺席"));

        // 入口页在位：原文直出
        let entry = entry_page_path(&root);
        std::fs::create_dir_all(entry.parent().unwrap()).unwrap();
        std::fs::write(&entry, "<html>面板</html>").unwrap();
        let resp = root_page(State(state)).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let text = body_text(resp).await;
        assert!(text.contains("面板"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn open_confirm_teaches_carrier_not_ready() {
        let root = temp_root("open");
        let state = WebState { root: root.clone() };

        // 表单违例仍走对等校验错误
        let resp = confirm_open_action(
            State(state.clone()),
            Form(OpenForm { domain_root: "".into(), token_id: "".into(), scope: "".into(), by: "".into() }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let text = body_text(resp).await;
        assert!(text.contains("域根缺席"));

        // 表单合规但全链承载位未落位：教学错误页指现役路径
        let resp = confirm_open_action(
            State(state.clone()),
            Form(OpenForm {
                domain_root: "/tmp/newdom".into(),
                token_id: "nd".into(),
                scope: "domain_write".into(),
                by: "test".into(),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let text = body_text(resp).await;
        assert!(text.contains("承载位未就绪"));
        assert!(text.contains("/tokens/open"));

        // 开域第一步确认面（零写入）：新牌计划形
        let resp = open_action(
            State(state.clone()),
            Form(OpenForm {
                domain_root: "/tmp/newdom".into(),
                token_id: "nd".into(),
                scope: "domain_write".into(),
                by: "test".into(),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let text = body_text(resp).await;
        assert!(text.contains("开域确认"));
        assert!(text.contains("将签发新牌 <code>nd</code>"));
        assert!(!registry_path(&root).is_file(), "确认前零写入");

        // 无 active 且缺标识即拒教学
        let resp = open_action(
            State(state.clone()),
            Form(OpenForm { domain_root: "/tmp/newdom".into(), token_id: "".into(), scope: "".into(), by: "".into() }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let text = body_text(resp).await;
        assert!(text.contains("本域根无 active 标识牌行"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn active_row_for_matches_normalized_domain() {
        let mut rows = BTreeMap::new();
        rows.insert("a".to_string(), tokens::issue_row("a", "/tmp/dom", "readonly", "t"));
        let mut stopped = tokens::issue_row("b", "/tmp/dom", "readonly", "t");
        stopped.status = tokens::STATUS_STOPPED.to_string();
        rows.insert("b".to_string(), stopped);
        // active 且域根相合（词法归一对表）
        let hit = active_row_for(&rows, Path::new("/tmp/dom/../dom"));
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().token_id, "a");
        // 零 active 即 None
        let mut only_stopped = BTreeMap::new();
        only_stopped.insert("b".to_string(), rows["b"].clone());
        assert!(active_row_for(&only_stopped, Path::new("/tmp/dom")).is_none());
    }

    #[test]
    fn router_builds_with_state() {
        // Router 零列举 API，构造即形验（真实绑端口禁，此处只组不启）。
        let _router = router(WebState { root: std::env::temp_dir() });
    }
}
