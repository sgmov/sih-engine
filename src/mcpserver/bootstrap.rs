//! 域自举 bootstrap 全链：新城引入 MCP 的开域五段一令（sihmcp-solo 丙簇）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/bootstrap.py（五段固化编排正典）
//! 与 sih-tools/mcpline/src/mcpline/init.py（开域单步窄口：precheck 六项与
//! open_domain 落地五步加开域毕即验域）。DES-015 判词：序固定五段——前置检查、
//! 标识牌签发位（缺牌即签）、开域位、镜像位（中央 active 行逐字段恒等写域内
//! 登记册）、客户端注册旗标选入位。写盘点位四件是本模块 sanctioned 职责
//! （DES-015 修订四分面申报）：中央登记册签发行（tokens::append_row，flock 加
//! O_APPEND 唯一写点位）、域内镜像行（同函数）、开域落地（含 tempfile report
//! 用完删）、可选客户端配置件（--client-config 显式给参才写，合并写保既有键，
//! 写毕重读复核）。语汇循「标识牌」非「凭据」（DEC-010 已拒凭证语义）。
//!
//! 三层承载：核心 bootstrap_domain（五段单一 canonical 路径）、CLI run_cli
//! （argv 与退出码 0/1/2 语义照录）、台面 router（GET 开域表单 / POST 开域 /
//! POST 确认开域，两步确认防手滑，确认后调 bootstrap_domain 同一核心零二次
//! 实现）。开域本体照 init.py 对等移植（本批无独立 init 模块位，开域形唯一
//! 承载即本模块 init_precheck 与 open_domain，与 Python 逐字对表）。

use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

use super::runtime::{code_root, resolve_root, run_readonly, scribe_bin, today_str, valid_date};
use super::tokens::{self, TokenRow};

// ------------------------------------------------------------------ 常量定形

pub const DEFAULT_SCOPE: &str = "domain_write";
pub const DEFAULT_OPENED_BY: &str = "main-window-bootstrap";
/// 统一 web 面连接形（init.py CLIENT_FACE_URL 同源照录）。
pub const CLIENT_FACE_URL: &str = "http://127.0.0.1:8765/mcp";
pub const CLIENT_SERVER_KEY: &str = "sih";
pub const CLIENT_TIMEOUT_MS: i64 = 60000;

pub const DOMAIN_DECLARATION_RELPATH: &str = "sih/domain.json";
pub const DOMAIN_README_RELPATH: &str = "sih/README.md";
pub const TEMPLATE_PLAN_RELPATH: &str = "sih/state/plan/TEMPLATE-任务包.md";
pub const TEMPLATE_PARKING_RELPATH: &str = "sih/state/parking/TEMPLATE-停泊材料.json";
pub const TEMPLATE_INTENT_PLAIN_RELPATH: &str = "sih/state/plan/TEMPLATE-意图-plain.json";
pub const LAYOUT_FORM_CANONICAL: &str = "canonical";
pub const NO_SESSION_REASON: &str = "域自举无租约会话主会处置位";
pub const REPORT_BATCH: &str = "domain-open";
pub const OPEN_PEN_WHAT: &str = "域开域初始化：正典 sih 树落地与资产包与模板落位";

const SCRIBE_TIMEOUT: Duration = Duration::from_secs(120);

// 正典指针三条（init.py CANON_POINTERS 同源照录）。
pub const CANON_DES_015: &str = "sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md";
pub const CANON_DES_014: &str = "sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md";
pub const CANON_SPEC_023: &str = "sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md";
pub const CANON_POINTERS: [&str; 3] = [CANON_DES_015, CANON_DES_014, CANON_SPEC_023];

// 面说明节正典常量（init.py 域自述卡面说明节同源照录，验证行与测试请求形一字同源）。
pub const RESTART_SEMANTICS: &str = "客户端配置于新会话读取，既有会话不重载，须重开新会话方见 sih 工具面";
pub const VERIFY_LINE_MCP: &str = "验证法一：新会话中 MCP 面 locks_read 心跳可调，即客户端配置在役";
pub const VERIFY_LINE_HTTP: &str = concat!(
    "验证法二：HTTP 面同形 POST /mcp，头携 Accept: application/json 与 ",
    "Authorization: Bearer <标识牌>，请求体 {\"jsonrpc\": \"2.0\", \"method\": \"tools/list\", \"id\": 1}",
    "，响应 200 且工具面含 locks_read，即配置在役"
);
pub const CLIENT_PAYLOAD_SUMMARY: &str = concat!(
    "固定 payload 形：type http、url http://127.0.0.1:8765/mcp、",
    "Authorization Bearer <标识牌> 头、enabled true、timeoutMs 60000（承 DES-015 修订四）"
);

// 出参教学定形（DES-015 修订五：重启语义与在役验证两法与面卡指针）。
pub const NEXT_TEACHING: &str = concat!(
    "全链毕：MCP 连接头携 Authorization: Bearer <标识牌> 读本域（写操作只落所",
    "绑域）；客户端配置于新会话读取，既有会话不重载，须重开新会话方见 sih 工具面",
    "；在役验证：验证法一：新会话中 MCP 面 locks_read 心跳可调，即客户端配置在役",
    "；验证法二：HTTP 面同形 POST /mcp，头携 Accept: application/json 与 ",
    "Authorization: Bearer <标识牌>，请求体 {\"jsonrpc\": \"2.0\", \"method\": \"tools/list\", \"id\": 1}",
    "，响应 200 且工具面含 locks_read，即配置在役；域面接法详见面卡 sih/README.md"
);

pub const WHAT_THIS_TOOL_DOES: &str = concat!(
    "域自举 bootstrap：新城引入 MCP 开域全链一令。序固定五段——前置检查、",
    "标识牌签发位（缺牌即签）、开域位（init 既有形零重实现）、令牌双落位",
    "（域内镜像行恒等）、客户端注册旗标选入（--client-config 给了才写，",
    "合并写保既有键，写毕重读复核）；已开域补全形 --complete（面卡与镜像核对",
    "与教学补全，零域状态写零 reinit）"
);
pub const VALID_PARAMS: [&str; 7] = [
    "<域根>",
    "--by <事由>",
    "[--token-id <词形>]",
    "[--scope <三档之一>]",
    "[--date <YYYY-MM-DD>]",
    "[--client-config <路径>]",
    "[--complete]",
];

// ------------------------------------------------------------------- 错误形

/// bootstrap 错误形：exit_code 1=前置拒或验红（教学 JSON 载荷）、2=工具自身
/// 异常（载荷照 bootstrap.py _tool_payload 形，无 fix 字段）。
#[derive(Debug, Clone)]
pub struct BootstrapError {
    pub exit_code: i32,
    pub payload: Value,
}

/// 前置拒或验红载荷（照 bootstrap.py _rejection_payload 六字段形）。
fn rejection(message: impl std::fmt::Display, fix: impl std::fmt::Display) -> BootstrapError {
    BootstrapError {
        exit_code: 1,
        payload: json!({
            "ok": false,
            "error": message.to_string(),
            "fix": fix.to_string(),
            "what_this_tool_does": WHAT_THIS_TOOL_DOES,
            "valid_params": VALID_PARAMS,
            "canonical_pointers": CANON_POINTERS,
        }),
    }
}

/// 工具自身异常载荷（照 bootstrap.py _tool_payload 形，error 加前缀零 fix）。
fn tool_error(message: impl std::fmt::Display) -> BootstrapError {
    BootstrapError {
        exit_code: 2,
        payload: json!({
            "ok": false,
            "error": format!("工具自身异常：{message}"),
            "what_this_tool_does": WHAT_THIS_TOOL_DOES,
            "valid_params": VALID_PARAMS,
            "canonical_pointers": CANON_POINTERS,
        }),
    }
}

// ------------------------------------------------------------------- 工具件

/// resolve 加 normpath 归一（域根同路比对位，与 Python _norm 对等：canonicalize
/// 成功即真归一；不成功（缺席路径）即绝对化逐段 lexical 归一，零 symlink 展开）。
fn norm(p: &Path) -> PathBuf {
    if let Ok(c) = std::fs::canonicalize(p) {
        return c;
    }
    let abs = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(p)
    };
    let mut out = PathBuf::new();
    for comp in abs.components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

/// 客户端配置路径 expanduser（HOME 环境展开，与 Python expanduser 对等窄形）。
fn expand_home(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    if s == "~" {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home);
        }
    } else if let Some(rest) = s.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                return PathBuf::from(home).join(rest);
            }
        }
    }
    p.to_path_buf()
}

/// 中央登记册位：第一域布局投影（对等 layout_for(central, central).tokens_path()
/// 即 sih-tools/mcpline/ledger/tokens.ndjson；域根恒非中央根，第一域形单点）。
pub fn central_registry_path(central_root: &Path) -> PathBuf {
    central_root.join("sih-tools/mcpline/ledger/tokens.ndjson")
}

/// 新城镜像登记册位：DES-015 登记册形判词「新城正典位 <域根>/sih/ledger/
/// tokens.ndjson」。
pub fn mirror_tokens_path(dom: &Path) -> PathBuf {
    dom.join("sih/ledger/tokens.ndjson")
}

/// scribe 子进程运行位（主树二进制，runtime::run_readonly 承 _clean_env 形）。
async fn run_scribe(argv: &[String]) -> super::runtime::RunOutcome {
    run_readonly(argv, None, None, SCRIBE_TIMEOUT).await
}

/// 链文件末行 event_hash 回取（append stdout 不可解析时的零猜回退位，init 同形）。
fn last_event_hash(trail: &Path) -> Option<String> {
    let text = std::fs::read_to_string(trail).ok()?;
    for raw in text.lines().rev() {
        let stripped = raw.trim();
        if stripped.is_empty() {
            continue;
        }
        let row: Value = serde_json::from_str(stripped).ok()?;
        return row
            .get("event_hash")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(str::to_string);
    }
    None
}

/// indent=1 JSON 直出形（对等 json.dumps(obj, ensure_ascii=False, indent=1)）。
fn emit_json(v: &Value, depth: usize) -> String {
    let pad = " ".repeat(depth);
    let inner = " ".repeat(depth + 1);
    match v {
        Value::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            let items: Vec<String> = map
                .iter()
                .map(|(k, val)| {
                    format!(
                        "{inner}{}: {}",
                        serde_json::to_string(k).unwrap_or_default(),
                        emit_json(val, depth + 1)
                    )
                })
                .collect();
            format!("{{\n{}\n{pad}}}", items.join(",\n"))
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            let items: Vec<String> = arr
                .iter()
                .map(|val| format!("{inner}{}", emit_json(val, depth + 1)))
                .collect();
            format!("[\n{}\n{pad}]", items.join(",\n"))
        }
        other => other.to_string(),
    }
}

/// stdout 出参：严格 JSON 单对象，indent=1 尾换行（对等 _emit）。
fn emit(obj: &Value) {
    println!("{}", emit_json(obj, 0));
}

fn dump_pretty2(v: &Value) -> String {
    // 对等 json.dumps(..., indent=2) + "\n"（域声明卡与停泊模板与客户端配置件）。
    format!("{}\n", serde_json::to_string_pretty(v).unwrap_or_default())
}

fn dump_indent1(v: &Value) -> String {
    format!("{}\n", emit_json(v, 0))
}

// ------------------------------------------------------------------- 段一

/// 段一：域根面五检（init 六项之域根面项，先于签发位跑；检查项与教学语与
/// init 逐字对表）。返回域是否已开域（sih/domain.json 在位）。
fn precheck_bootstrap_face(dom: &Path, central_root: &Path) -> Result<bool, BootstrapError> {
    if !dom.exists() {
        return Err(rejection(
            format!("域根缺席：{}", dom.display()),
            "先建域根目录（域根须为 git 仓：先 git init 与 git config 用户件）",
        ));
    }
    if !dom.is_dir() {
        return Err(rejection(
            format!("域根非目录：{}", dom.display()),
            "域根须为目录（git 仓根目录）",
        ));
    }
    if !dom.join(".git").exists() {
        return Err(rejection(
            format!("域根下 .git 缺席：{}", dom.display()),
            "域根内先 git init 加 git config user.name 与 user.email（lease git 机制要求域根为 git 仓）",
        ));
    }
    if norm(dom) == norm(central_root) {
        return Err(rejection(
            format!("域根等于中央根：{}", dom.display()),
            "第一域是历史映射形不开域（司衡工作区本身即第一域，布局走映射表不迁移）；bootstrap 仅用于新城正典形域",
        ));
    }
    let decl = dom.join(DOMAIN_DECLARATION_RELPATH);
    if decl.exists() {
        return Ok(true);
    }
    let sih = dom.join("sih");
    if sih.is_dir()
        && std::fs::read_dir(&sih)
            .map(|mut it| it.next().is_some())
            .unwrap_or(false)
    {
        return Err(rejection(
            format!("半态：sih/ 目录存在且非空但 domain.json 缺席：{}", sih.display()),
            "半态位人节点处置（清 sih/ 下散件或与既有域声明卡对表）后重跑",
        ));
    }
    Ok(false)
}

// ------------------------------------------------------------------- 段二

/// 段二：签发位。返回 (active 行, issued 出参)。本域根无 active 行或末行
/// stopped 即签发（须 --token-id）；已有 active 行即复用不重签。落册经
/// tokens::append_row（flock 加 O_APPEND 唯一 sanctioned 写点位）。
fn issue_if_needed(
    registry: &Path,
    dom: &Path,
    token_id: Option<&str>,
    scope: &str,
    issued_by: &str,
) -> Result<(TokenRow, Value), BootstrapError> {
    if !tokens::VALID_SCOPES.contains(&scope) {
        return Err(rejection(
            format!("scope 违例：'{scope}'"),
            format!("scope 须三档之一 {:?}（缺省 {DEFAULT_SCOPE}）", tokens::VALID_SCOPES),
        ));
    }
    let rows = tokens::load_last_rows(registry)
        .map_err(|e| tool_error(format!("中央登记册行形损坏：{e}")))?;
    let want = norm(dom);
    let bound: Vec<&TokenRow> = rows
        .values()
        .filter(|r| norm(Path::new(&r.domain_root)) == want)
        .collect();
    let active: Vec<&TokenRow> = bound
        .iter()
        .copied()
        .filter(|r| r.status == tokens::STATUS_ACTIVE)
        .collect();
    if active.len() > 1 {
        let mut ids: Vec<String> = active.iter().map(|r| r.token_id.clone()).collect();
        ids.sort();
        return Err(rejection(
            format!("中央登记册本域根多标识牌 active 并存：{ids:?}"),
            "先经管理台 /tokens 撤销多余标识牌止一，再跑 bootstrap",
        ));
    }
    if let Some(row) = active.first() {
        let row = (*row).clone();
        return Ok((
            row.clone(),
            json!({
                "done": false,
                "token_id": row.token_id,
                "scope": row.scope,
                "note": "active_row_reused",
            }),
        ));
    }
    let token_id = match token_id {
        Some(t) if !t.trim().is_empty() => t,
        _ => {
            return Err(rejection(
                format!("中央登记册无本域根 active 标识牌行，全链签发须 --token-id：{}", dom.display()),
                format!(
                    "补 --token-id <可读短串>（词形循 TOKEN_ID_RE）与可选 --scope（缺省 {DEFAULT_SCOPE}）重跑；或先经管理台 /tokens 签发再跑"
                ),
            ));
        }
    };
    let row = tokens::issue_row(token_id, &dom.display().to_string(), scope, issued_by);
    tokens::append_row(registry, &row).map_err(|e| tool_error(format!("签发行落册拒：{e}")))?;
    let note = if bound.is_empty() { "fresh_issue" } else { "reissued_from_stopped" };
    Ok((
        row,
        json!({
            "done": true,
            "token_id": token_id,
            "scope": scope,
            "note": note,
        }),
    ))
}

// ------------------------------------------------------------------- 段三

/// 开域前置检查六项（init.py precheck 对等移植，序固定）。通过返回
/// (域根归一形, 登记 active 行)。pub 放宽（adoptface 批 sih init 薄壳跨 bin
/// 调用位，签名全 pub 类型零私有泄漏，零行为变更）。
pub fn init_precheck(dom: &Path, central_root: &Path, registry: &Path) -> Result<(PathBuf, TokenRow), BootstrapError> {
    // 一：域根存在且为目录
    if !dom.exists() {
        return Err(rejection(
            format!("域根缺席：{}", dom.display()),
            "先建域根目录（域根须为 git 仓：先 git init 与 git config 用户件）",
        ));
    }
    if !dom.is_dir() {
        return Err(rejection(
            format!("域根非目录：{}", dom.display()),
            "域根须为目录（git 仓根目录）",
        ));
    }
    // 二：域根下 .git 在位（lease git 机制要求）
    if !dom.join(".git").exists() {
        return Err(rejection(
            format!("域根下 .git 缺席：{}", dom.display()),
            "域根内先 git init 加 git config user.name 与 user.email（lease git 机制要求域根为 git 仓）",
        ));
    }
    // 三：中央登记册有 active 行且 domain_root 同路（取其 token_id 与 scope）
    let rows = tokens::load_last_rows(registry)
        .map_err(|e| tool_error(format!("中央登记册行形损坏：{e}")))?;
    let want = norm(dom);
    let active_matches: Vec<&TokenRow> = rows
        .values()
        .filter(|r| r.status == tokens::STATUS_ACTIVE && norm(Path::new(&r.domain_root)) == want)
        .collect();
    let row = if active_matches.is_empty() {
        let bound: Vec<&TokenRow> = rows
            .values()
            .filter(|r| norm(Path::new(&r.domain_root)) == want)
            .collect();
        if !bound.is_empty() {
            let mut ids: Vec<String> = bound.iter().map(|r| r.token_id.clone()).collect();
            ids.sort();
            return Err(rejection(
                format!("中央登记册绑定本域根的标识牌末行已停行：{ids:?}"),
                "先经管理台 /tokens 重签或换牌（签发或换牌绑定本域根）再跑 init",
            ));
        }
        return Err(rejection(
            format!("中央登记册无本域根的 active 标识牌行：{}", dom.display()),
            "先经管理台 /tokens 签发本域首标识牌（绑定本域根与 scope）再跑 init",
        ));
    } else if active_matches.len() > 1 {
        let mut ids: Vec<String> = active_matches.iter().map(|r| r.token_id.clone()).collect();
        ids.sort();
        return Err(rejection(
            format!("中央登记册本域根多标识牌 active 并存：{ids:?}"),
            "先经管理台 /tokens 撤销多余标识牌止一，再跑 init",
        ));
    } else {
        active_matches[0].clone()
    };
    // 四：域根不等于中央根（第一域映射形守卫——第一域是历史映射形不开域）
    if want == norm(central_root) {
        return Err(rejection(
            format!("域根等于中央根：{}", dom.display()),
            "第一域是历史映射形不开域（司衡工作区本身即第一域，布局走映射表不迁移）；init 仅用于新城正典形域",
        ));
    }
    // 五：sih/domain.json 缺席（幂等守卫——已开域即拒，无 reinit 旗标）
    let decl = dom.join(DOMAIN_DECLARATION_RELPATH);
    if decl.exists() {
        return Err(rejection(
            format!("域已开域（sih/domain.json 在位）：{}", decl.display()),
            "init 幂等守卫无 reinit 旗标；重开域候后继裁定（先人节点处置既有域状态）",
        ));
    }
    // 六：半态拒（sih/ 存在且非空但 domain.json 缺席，半态位人节点处置）
    let sih = dom.join("sih");
    if sih.is_dir()
        && std::fs::read_dir(&sih)
            .map(|mut it| it.next().is_some())
            .unwrap_or(false)
    {
        return Err(rejection(
            format!("半态：sih/ 目录存在且非空但 domain.json 缺席：{}", sih.display()),
            "半态位人节点处置（清 sih/ 下散件或与既有域声明卡对表）后重跑 init",
        ));
    }
    Ok((dom.to_path_buf(), row))
}

/// 停泊材料骨架（init.py PARKING_TEMPLATE 照录，字段形对表 pk-070-exit.json）。
fn parking_template() -> Value {
    json!({
        "action": "<占位：parking 动作词，enter 入泊或 exit 出泊>",
        "entry_id": "<占位：pk-NNN 泊界条目编号>",
        "context": "<占位：泊界上下文，入泊条件与证据或出泊条件达成事实>",
        "ruling": "<占位：裁定原文或裁定编号，日期与令源随附>",
        "state": "<占位：parked 在泊或 exited 已出泊>",
        "parking": {
            "entered_at": "<占位：YYYY-MM-DD 入泊日期>",
            "ttl_days": 30,
        },
    })
}

/// 写链意图骨架（init.py PLAIN_INTENT_TEMPLATE 照录，plain 形零哲学引文）。
fn plain_intent_template() -> Value {
    json!({
        "session_id": "<占位：会话标识 sess- 形>",
        "raw_input": "<占位：令源原文或任务来文>",
        "round": 1,
        "intent_contract": {
            "goal": "<占位：一句话目标>",
            "exclusions": ["<占位：红线与不做什么>"],
            "output_format": "<占位：产出物清单>",
        },
        "domain_contract": {
            "target_domain": {"scope": "<占位：写入面路径清单>", "max_depth": 1},
            "support_domain": {"scope": "<占位：只读依据路径清单>", "max_depth": 1},
        },
    })
}

/// 域自述卡单一正典生成函数（init.py domain_readme_text 逐字对表；修订三树图
/// 四行用法三条正典指针三条加修订五面说明节；幂等恒等重写）。
fn domain_readme_text(token_id: &str, domain_root: &Path, declaration: &Value) -> String {
    let field = |k: &str| declaration.get(k).and_then(|v| v.as_str()).unwrap_or("");
    let lines = vec![
        format!("# 域自述：{token_id}"),
        String::new(),
        format!("- 域名（标识牌）：{token_id}"),
        format!("- 域根：{}", domain_root.display()),
        format!(
            "- 布局形：{}（新城正典单根布局，DES-015 域目录布局规约）",
            field("layout_form")
        ),
        format!("- 开域时：{}", field("opened_at")),
        format!("- 开域方：{}", field("opened_by")),
        format!("- mcpline 版本：{}", field("mcpline_version")),
        String::new(),
        "## 树图".to_string(),
        String::new(),
        "- sih/event/trail/ 逐日哈希链（一日一链，ndjson append-only）".to_string(),
        "- sih/ledger/ 台账册族（sessions 会话册与 locks 锁册与 claims 领取册与 tokens 标识登记册同规约：flock 串行加 O_APPEND 原子整行）".to_string(),
        "- sih/state/plan/ 任务包区（新批任务包写此区，照 TEMPLATE-任务包.md 形；写链意图照 TEMPLATE-意图-plain.json 形）".to_string(),
        "- sih/state/parking/materials/ 泊界材料（泊界材料写此区，照 TEMPLATE-停泊材料.json 形）".to_string(),
        String::new(),
        "## 用法".to_string(),
        String::new(),
        format!(
            "1. MCP 连接头携 `Authorization: Bearer {token_id}` 读本域（标识牌是明文短标识，中央登记册登记，解析单点）。"
        ),
        "2. 新批任务包写 sih/state/plan，照 TEMPLATE-任务包.md 形（版本位必填、请求写入节逐路径分行）；写链意图照 TEMPLATE-意图-plain.json 形（plain 形零哲学引文，DES-016 意图形分派）。".to_string(),
        "3. 泊界材料写 sih/state/parking/materials，照 TEMPLATE-停泊材料.json 形。".to_string(),
        String::new(),
        "## 面接法（sih 工具面）".to_string(),
        String::new(),
        format!(
            "- 面 URL：{CLIENT_FACE_URL}（统一 web 面识别写面，DES-015）"
        ),
        format!(
            "- 标识牌：{token_id}（明文短标识防呆锚点，DEC-010）；scope：{}",
            field("token_scope")
        ),
        "- 中央登记册镜像：sih/ledger/tokens.ndjson（末行 active 为准，与中央登记册本域 active 行恒等）".to_string(),
        format!(
            "- 客户端注册：bootstrap --client-config <路径> 显式给参才写（旗标选入，缺省零触碰）；{CLIENT_PAYLOAD_SUMMARY}；合并写仅设 mcp.servers.sih，其余键保留"
        ),
        format!("- 生效条件：{RESTART_SEMANTICS}"),
        format!("- {VERIFY_LINE_MCP}"),
        format!("- {VERIFY_LINE_HTTP}"),
        "- 重开域禁令：已开域重跑 bootstrap / init 即拒（幂等守卫，无 reinit 旗标）；已开域补全形：bootstrap --complete（面卡与镜像核对与教学补全，零域状态写零 reinit）".to_string(),
        "- 典源：DES-015 修订五；DEC-010".to_string(),
        String::new(),
        "## 正典指针".to_string(),
        String::new(),
        format!(
            "- DES-015：{CANON_DES_015}（项目域识别形与域自举程序形与域目录布局规约）"
        ),
        format!("- DES-014：{CANON_DES_014}（MCP beta 写面安全模型）"),
        format!("- SPEC-023：{CANON_SPEC_023}（alpha 相只读面契约）"),
        String::new(),
    ];
    lines.join("\n")
}

/// 落地五步加开域毕即验域（init.py open_domain 对等移植）。返回出参对象
/// （成功形字段全集）。验红链留笔不删如实；全程零整文件重写台账。pub 放宽
/// （adoptface 批 sih init 薄壳跨 bin 调用位，签名全 pub 类型零私有泄漏，
/// 零行为变更）。
pub async fn open_domain(dom: &Path, row: &TokenRow, opened_by: &str, date: &str) -> Result<Value, BootstrapError> {
    let opened_at = tokens::now_stamp();
    // 步一：建目录四件
    for rel in ["sih/event/trail", "sih/ledger", "sih/state/plan", "sih/state/parking/materials"] {
        std::fs::create_dir_all(dom.join(rel))
            .map_err(|e| tool_error(format!("目录落地拒 {}：{e}", dom.join(rel).display())))?;
    }
    // 步二：写域声明卡（七字段固定形）
    let mut declaration = Map::new();
    declaration.insert("domain_id".into(), json!(row.token_id));
    declaration.insert("domain_root".into(), json!(dom.display().to_string()));
    declaration.insert("token_scope".into(), json!(row.scope));
    declaration.insert("layout_form".into(), json!(LAYOUT_FORM_CANONICAL));
    declaration.insert("opened_at".into(), json!(opened_at));
    declaration.insert("opened_by".into(), json!(opened_by));
    declaration.insert(
        "mcpline_version".into(),
        json!(env!("CARGO_PKG_VERSION")),
    );
    let declaration = Value::Object(declaration);
    let decl_bytes = dump_pretty2(&declaration).into_bytes();
    let decl_path = dom.join(DOMAIN_DECLARATION_RELPATH);
    std::fs::write(&decl_path, &decl_bytes)
        .map_err(|e| tool_error(format!("域声明卡落地拒 {}：{e}", decl_path.display())))?;
    // 步三：落模板两件（任务包模板字节复制中央单一源；停泊骨架模块内置）
    let template_src = code_root().join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md");
    if !template_src.is_file() {
        return Err(tool_error(format!("中央任务包模板源缺席：{}", template_src.display())));
    }
    let tpl_bytes = std::fs::read(&template_src)
        .map_err(|e| tool_error(format!("中央任务包模板源不可读：{e}")))?;
    let plan_path = dom.join(TEMPLATE_PLAN_RELPATH);
    std::fs::write(&plan_path, tpl_bytes)
        .map_err(|e| tool_error(format!("任务包模板落地拒：{e}")))?;
    let parking_path = dom.join(TEMPLATE_PARKING_RELPATH);
    std::fs::write(&parking_path, dump_pretty2(&parking_template()))
        .map_err(|e| tool_error(format!("停泊模板落地拒：{e}")))?;
    let intent_path = dom.join(TEMPLATE_INTENT_PLAIN_RELPATH);
    std::fs::write(&intent_path, dump_indent1(&plain_intent_template()))
        .map_err(|e| tool_error(format!("意图模板落地拒：{e}")))?;
    // 步四：写域自述（单一正典生成函数）
    let readme_path = dom.join(DOMAIN_README_RELPATH);
    std::fs::write(&readme_path, domain_readme_text(&row.token_id, dom, &declaration))
        .map_err(|e| tool_error(format!("域自述落地拒：{e}")))?;
    // 步五：开域首笔（report 域外暂存用完删，经中央 scribe 无会话主会处置位）
    let scribe = scribe_bin();
    if !scribe.is_file() {
        return Err(tool_error(format!("scribe 主树二进制缺席：{}", scribe.display())));
    }
    let trail = dom.join("sih/event/trail").join(format!("{date}.ndjson"));
    let report = json!({
        "batch": REPORT_BATCH,
        "domain_id": row.token_id,
        "domain_root": dom.display().to_string(),
        "at": date,
        "what": OPEN_PEN_WHAT,
        "opened_by": opened_by,
        "domain_declaration_sha256": hex::encode(Sha256::digest(&decl_bytes)),
        "templates": [TEMPLATE_PLAN_RELPATH, TEMPLATE_PARKING_RELPATH, TEMPLATE_INTENT_PLAIN_RELPATH],
    });
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_path = std::env::temp_dir().join(format!("mcpline-init-report-{}-{nanos}.json", std::process::id()));
    std::fs::write(&tmp_path, dump_indent1(&report))
        .map_err(|e| tool_error(format!("开域首笔报告暂存拒：{e}")))?;
    let argv = vec![
        scribe.display().to_string(),
        "append".to_string(),
        "--report".to_string(),
        tmp_path.display().to_string(),
        "--exit-code".to_string(),
        "0".to_string(),
        "--trail".to_string(),
        trail.display().to_string(),
        "--no-session-reason".to_string(),
        NO_SESSION_REASON.to_string(),
    ];
    let out = run_scribe(&argv).await;
    let _ = std::fs::remove_file(&tmp_path); // 域外暂存用完删
    if out.rc != 0 {
        let payload = if out.stdout.trim().is_empty() { &out.stderr } else { &out.stdout };
        return Err(tool_error(format!(
            "开域首笔 scribe append 拒：退出码 {} 载荷 {}",
            out.rc,
            payload.trim()
        )));
    }
    let mut pen_hash = serde_json::from_str::<Value>(&out.stdout)
        .ok()
        .and_then(|v| v.get("event_hash").and_then(|h| h.as_str()).map(str::to_string))
        .filter(|h| !h.is_empty());
    if pen_hash.is_none() {
        pen_hash = last_event_hash(&trail);
    }
    let pen_hash = match pen_hash {
        Some(h) => h,
        None => {
            return Err(tool_error(format!(
                "开域首笔 event_hash 不可得（append stdout 不可解析且链末行无 event_hash）：{}",
                out.stdout.trim().chars().take(400).collect::<String>()
            )));
        }
    };
    // 开域毕即验域：verify 退出码零且判词 valid
    let argv = vec![
        scribe.display().to_string(),
        "verify".to_string(),
        "--trail".to_string(),
        trail.display().to_string(),
    ];
    let out = run_scribe(&argv).await;
    let verdict = serde_json::from_str::<Value>(&out.stdout)
        .ok()
        .and_then(|v| v.get("status").and_then(|s| s.as_str()).map(str::to_string));
    if out.rc != 0 || verdict.as_deref() != Some("valid") {
        let payload = if out.stdout.trim().is_empty() { &out.stderr } else { &out.stdout };
        return Err(rejection(
            format!(
                "开域毕验红：scribe verify 退出码 {} 判词 {} 载荷 {}",
                out.rc,
                verdict.unwrap_or_else(|| "不可解析".to_string()),
                payload.trim()
            ),
            format!(
                "链文件留开域首笔不删：{}；人节点对表后重跑 scribe verify --trail <链文件> 复核",
                trail.display()
            ),
        ));
    }
    let files: Vec<Value> = [
        DOMAIN_DECLARATION_RELPATH,
        DOMAIN_README_RELPATH,
        TEMPLATE_PLAN_RELPATH,
        TEMPLATE_PARKING_RELPATH,
        TEMPLATE_INTENT_PLAIN_RELPATH,
    ]
    .iter()
    .map(|p| json!(p))
    .chain(std::iter::once(json!(format!("sih/event/trail/{date}.ndjson"))))
    .collect();
    // 步六：飞轮隔离步（pk-105 收口，stdio-auto 批）：幂等把 sih/ 写入域根
    // .git/info/exclude 并以 git check-ignore 机械验证。与 sih.rs 窄口
    // flywheel_git_exclude 同语义零二账；三入口（stdio 自动、CLI bootstrap、
    // 控制台 open）经 open_domain 全覆盖。非致命：失败降级为 warning 出参，
    // 域数据已落盘不回滚。
    let flywheel_git_exclude = flywheel_exclude(dom);
    Ok(json!({
        "ok": true,
        "domain_id": row.token_id,
        "domain_root": dom.display().to_string(),
        "opened_at": opened_at,
        "files": files,
        "open_pen_hash": pen_hash,
        "chain_verify": "valid",
        "flywheel_git_exclude": flywheel_git_exclude,
        "next": "读面经标识牌绑定本域可用：MCP 连接头携 Authorization: Bearer <标识牌> 读本域",
    }))
}

/// 飞轮隔离步：幂等写 `sih/` 一行进域根 .git/info/exclude，git check-ignore
/// 机械验证。写失败或验证不过降级 warning 不拒开域（域数据已落盘不回滚），
/// 出参形对齐 sih.rs 窄口（method 加 verified 加可选 warning）。
pub fn flywheel_exclude(dom: &Path) -> Value {
    const LINE: &str = "sih/";
    let exclude = dom.join(".git/info/exclude");
    let write_result = (|| -> Result<(), String> {
        if let Some(parent) = exclude.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("info/ 目录创建拒：{e}"))?;
        }
        let cur = if exclude.is_file() {
            std::fs::read_to_string(&exclude).map_err(|e| format!("exclude 不可读：{e}"))?
        } else {
            String::new()
        };
        if cur.lines().any(|l| l.trim_end() == LINE) {
            return Ok(()); // 已有该行，幂等跳过
        }
        let mut next = cur;
        if !next.is_empty() && !next.ends_with('\n') {
            next.push('\n');
        }
        next.push_str(LINE);
        next.push('\n');
        std::fs::write(&exclude, next).map_err(|e| format!("exclude 写入拒：{e}"))
    })();
    if let Err(msg) = write_result {
        return json!({"method": ".git/info/exclude", "verified": false, "warning": msg});
    }
    match std::process::Command::new("git")
        .arg("-C")
        .arg(dom)
        .args(["check-ignore", "-q", LINE])
        .status()
    {
        Ok(st) if st.code() == Some(0) => json!({"method": ".git/info/exclude", "verified": true}),
        Ok(st) => json!({
            "method": ".git/info/exclude",
            "verified": false,
            "warning": format!(
                "git check-ignore 退出码 {:?}，sih/ 未确认被忽略（域数据已落盘不回滚）",
                st.code()
            ),
        }),
        Err(e) => json!({
            "method": ".git/info/exclude",
            "verified": false,
            "warning": format!("git spawn 拒：{e}（域数据已落盘不回滚）"),
        }),
    }
}

// ------------------------------------------------------------------- 段四

/// 段四：镜像位。中央 active 行逐字段恒等镜像写域内登记册（DES-015 登记册形
/// 判词「落位两形」正典实装；append_row 既有函数，flock 加 O_APPEND 原子整行）。
fn write_mirror_row(dom: &Path, row: &TokenRow) -> Result<PathBuf, BootstrapError> {
    let mirror = mirror_tokens_path(dom);
    tokens::append_row(&mirror, row).map_err(|e| tool_error(format!("镜像行落册拒：{e}")))?;
    Ok(mirror)
}

// ------------------------------------------------------------------- 段五

/// 客户端注册固定 payload（.zcode/config.json 的 mcp.servers.sih 值形）。
pub fn client_entry(token_id: &str) -> Value {
    json!({
        "type": "http",
        "url": CLIENT_FACE_URL,
        "headers": {"Authorization": format!("Bearer {token_id}")},
        "enabled": true,
        "timeoutMs": CLIENT_TIMEOUT_MS,
    })
}

/// 段一随行：客户端配置件形先验（零写入期）。旗标缺席即 (None, flag_absent)；
/// 既有件非合法 JSON 或顶层非对象形即拒教学（零触碰）——畸形件在签发与开域前
/// 即拒，全链零写入可修复重跑。
fn load_client_config(path: Option<&Path>) -> Result<(Option<Value>, &'static str), BootstrapError> {
    let Some(path_str) = path else {
        return Ok((None, "flag_absent"));
    };
    let path = expand_home(path_str);
    if !path.exists() {
        return Ok((Some(json!({})), "created"));
    }
    let text = std::fs::read_to_string(&path).map_err(|e| {
        rejection(
            format!("客户端配置件非合法 JSON：{}（{e}）", path.display()),
            "先人节点处置既有件（备份或修形）后重跑；本工具零触碰畸形件",
        )
    })?;
    let obj: Value = serde_json::from_str(&text).map_err(|e| {
        rejection(
            format!("客户端配置件非合法 JSON：{}（{e}）", path.display()),
            "先人节点处置既有件（备份或修形）后重跑；本工具零触碰畸形件",
        )
    })?;
    if !obj.is_object() {
        return Err(rejection(
            format!("客户端配置件顶层非对象形：{}", path.display()),
            "顶层须 JSON 对象；先人节点处置既有件后重跑",
        ));
    }
    Ok((Some(obj), "merged_preserved"))
}

/// 段五：客户端注册位。合并写，仅设 mcp.servers.sih 固定 payload；mcp 与
/// mcp.servers 缺席即建，其余键原样保留；indent=2 尾换行；幂等可重跑。
fn write_client_config(path: &Path, token_id: &str, mut obj: Value, note: &str) -> Result<Value, BootstrapError> {
    let path = expand_home(path);
    let entry = client_entry(token_id);
    {
        let map = obj.as_object_mut().ok_or_else(|| {
            rejection(
                format!("客户端配置件顶层非对象形：{}", path.display()),
                "顶层须 JSON 对象；先人节点处置既有件后重跑",
            )
        })?;
        if !map
            .get("mcp")
            .map(|m| m.is_object())
            .unwrap_or(false)
        {
            if map.get("mcp").is_some() {
                return Err(rejection(
                    format!("客户端配置件 mcp 键非对象形：{}", path.display()),
                    "mcp 键须 JSON 对象；先人节点处置既有件后重跑",
                ));
            }
            map.insert("mcp".into(), json!({}));
        }
        let mcp = map.get_mut("mcp").and_then(|m| m.as_object_mut()).unwrap();
        if !mcp
            .get("servers")
            .map(|s| s.is_object())
            .unwrap_or(false)
        {
            if mcp.get("servers").is_some() {
                return Err(rejection(
                    format!("客户端配置件 mcp.servers 键非对象形：{}", path.display()),
                    "mcp.servers 键须 JSON 对象；先人节点处置既有件后重跑",
                ));
            }
            mcp.insert("servers".into(), json!({}));
        }
        let servers = mcp.get_mut("servers").and_then(|s| s.as_object_mut()).unwrap();
        servers.insert(CLIENT_SERVER_KEY.into(), entry);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| tool_error(format!("客户端配置目录落地拒 {}：{e}", parent.display())))?;
    }
    std::fs::write(&path, dump_pretty2(&obj))
        .map_err(|e| tool_error(format!("客户端配置件写入拒 {}：{e}", path.display())))?;
    Ok(json!({
        "written": true,
        "path": path.display().to_string(),
        "note": note,
    }))
}

/// 客户端配置出参教学定形（DES-015 修订五）：note 载生效条件与配置路径。
fn teaching_note(note: &str, path: &str) -> String {
    format!("{note}；生效条件：{RESTART_SEMANTICS}；配置路径：{path}")
}

/// 段五写后复核位：客户端配置写毕重读，断言 mcp.servers.sih 固定 payload 恒等
/// 在位（type http 与 url 与 Bearer 头与 enabled 真值与 timeoutMs 六万）。缺键
/// 或漂移即拒教学零静默（畸形结果零放行，DES-015 修订五）。
fn verify_client_config(path: &Path, token_id: &str) -> Result<(), BootstrapError> {
    let path = expand_home(path);
    let obj: Value = std::fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .ok_or_else(|| {
            rejection(
                format!("客户端配置写后复核拒：配置件不可重读或非合法 JSON：{}", path.display()),
                "写后复核检出畸形结果，先人节点处置该件后重跑（已开域补全加 --complete）",
            )
        })?;
    let entry = obj
        .get("mcp")
        .and_then(|m| m.get("servers"))
        .and_then(|s| s.get(CLIENT_SERVER_KEY))
        .cloned();
    if entry.as_ref() != Some(&client_entry(token_id)) {
        return Err(rejection(
            format!(
                "客户端配置写后复核拒：mcp.servers.{CLIENT_SERVER_KEY} 固定 payload 缺席或漂移：{}",
                path.display()
            ),
            format!(
                "复核断言形即 {CLIENT_PAYLOAD_SUMMARY}；先人节点处置该件后重跑（已开域补全加 --complete）"
            ),
        ));
    }
    Ok(())
}

// ------------------------------------------------------------- 全链编排核心

/// 全链五段编排（CLI 与台面单一 canonical 路径）。序固定：段一前置检查（域根
/// 面五检先于签发；已开域即拒幂等守卫；客户端配置件形先验随行）→ 段二签发位
/// → 段三开域位（init precheck 六项复跑加 open_domain 落地五步加验域收口）→
/// 段四镜像位 → 段五客户端注册位加写后复核位。出参严格 JSON 单对象。
pub async fn bootstrap_domain(
    root: &std::path::Path,
    token_id: Option<&str>,
    scope: Option<&str>,
    by: &str,
    date: Option<&str>,
    client_config: Option<&std::path::Path>,
) -> Result<Value, BootstrapError> {
    let date = date.map(str::to_string).unwrap_or_else(today_str);
    if !valid_date(&date) {
        return Err(tool_error(format!("--date 词形违例：{date}")));
    }
    let central_root = resolve_root();
    let registry = central_registry_path(&central_root);
    let dom = norm(root);
    // 段一：前置检查（域根面检先于签发；已开域分流；客户端配置件形先验随行
    // ——畸形件在签发与开域前即拒，全链零写入可修复重跑）
    let opened = precheck_bootstrap_face(&dom, &central_root)?;
    if opened {
        return Err(rejection(
            format!(
                "域已开域（sih/domain.json 在位）：{}",
                dom.join(DOMAIN_DECLARATION_RELPATH).display()
            ),
            "幂等守卫无 reinit 旗标；已开域补全重跑加 --complete（面卡与镜像核对与教学补全，零域状态写零 reinit）；重开域候后继裁定（先人节点处置既有域状态）",
        ));
    }
    let (client_obj, client_note) = load_client_config(client_config)?;
    // 段二：签发位（缺牌即签，有牌复用）
    let scope = scope.unwrap_or(DEFAULT_SCOPE);
    // row（签发行）在开域位由 init_precheck 复读的 row2 承接（init 同形），
    // 本载体无补全分流故 row 本身不再直读，出参经 issued 承载。
    let (_row, issued) = issue_if_needed(&registry, &dom, token_id, scope, by)?;
    // 段三：开域位（init 既有形零重实现；precheck 六项在签发后复跑全过）
    let (dom2, row2) = init_precheck(&dom, &central_root, &registry)?;
    let opened_r = open_domain(&dom2, &row2, by, &date).await?;
    // 段四：镜像位（验域 valid 后恒等落域内登记册）
    let mirror = write_mirror_row(&dom2, &row2)?;
    let mirror_action = "appended";
    let token_used = row2.token_id.clone();
    let readme_path = dom2.join(DOMAIN_README_RELPATH);
    // 段五：客户端注册位（旗标选入，缺省零触碰 .zcode）加写后复核位
    let mut client = json!({"written": false, "path": null, "note": client_note});
    if let Some(obj) = client_obj {
        let cfg_path = expand_home(client_config.expect("client_obj 在位即路径在位"));
        let written = write_client_config(&cfg_path, &token_used, obj, client_note)?;
        let note = teaching_note(
            written["note"].as_str().unwrap_or(client_note),
            written["path"].as_str().unwrap_or_default(),
        );
        client = json!({
            "written": true,
            "path": written["path"],
            "note": note,
        });
        verify_client_config(&cfg_path, &token_used)?;
    }
    let mut result = json!({
        "ok": true,
        "domain_id": opened_r["domain_id"],
        "domain_root": dom.display().to_string(),
        "opened_at": opened_r["opened_at"],
        "issued": issued,
        "chain_verify": opened_r["chain_verify"],
        "mirror_row_path": mirror.display().to_string(),
        "mirror_action": mirror_action,
        "readme_path": readme_path.display().to_string(),
        "client_config": client,
        "next": NEXT_TEACHING,
    });
    // 出参承开域链（全链恒含：本载体无补全分流，开域恒跑）。
    result["files"] = opened_r["files"].clone();
    result["open_pen_hash"] = opened_r["open_pen_hash"].clone();
    result["flywheel_git_exclude"] = opened_r["flywheel_git_exclude"].clone();
    Ok(result)
}

// ------------------------------------------------------------------- CLI 形

/// CLI 承载（argv 形照 bootstrap.py main）：sihmcp bootstrap <域根> --by <事由>
/// [--token-id 词形] [--scope domain_write] [--date] [--client-config 路径]。
/// 退出码 0 成功；1 前置拒或验红（教学 JSON 出 stdout，拒时加 files_kept 如实
/// 注记）；2 工具自身异常（教学 JSON 出 stdout）与用法错（usage 出 stderr）。
pub async fn run_cli(args: &[String]) -> anyhow::Result<i32> {
    const USAGE: &str = "usage: mcpline.bootstrap <域根> --by <事由> [--token-id <词形>] [--scope <三档之一>] [--date <YYYY-MM-DD>] [--client-config <路径>] [--complete]";
    let mut domain_root: Option<String> = None;
    let mut by: Option<String> = None;
    let mut token_id: Option<String> = None;
    let mut scope: Option<String> = None;
    let mut date: Option<String> = None;
    let mut client_config: Option<String> = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "bootstrap" | "mcpline.bootstrap" => {}
            "--by" => match it.next() {
                Some(v) => by = Some(v.clone()),
                None => {
                    eprintln!("{USAGE}");
                    return Ok(2);
                }
            },
            "--token-id" => match it.next() {
                Some(v) => token_id = Some(v.clone()),
                None => {
                    eprintln!("{USAGE}");
                    return Ok(2);
                }
            },
            "--scope" => match it.next() {
                Some(v) => scope = Some(v.clone()),
                None => {
                    eprintln!("{USAGE}");
                    return Ok(2);
                }
            },
            "--date" => match it.next() {
                Some(v) => date = Some(v.clone()),
                None => {
                    eprintln!("{USAGE}");
                    return Ok(2);
                }
            },
            "--client-config" => match it.next() {
                Some(v) => client_config = Some(v.clone()),
                None => {
                    eprintln!("{USAGE}");
                    return Ok(2);
                }
            },
            // 补全旗标显式拒（如实教学，零静默吞旗标）。
            "--complete" => {
                eprintln!("--complete 补全形候后继批承载：Rust 载体本批零补全链，已开域补全请走 Python 载体 mcpline.bootstrap --complete");
                return Ok(2);
            }
            other => {
                if domain_root.is_none() {
                    domain_root = Some(other.to_string());
                } else {
                    eprintln!("{USAGE}");
                    return Ok(2);
                }
            }
        }
    }
    let Some(domain_root) = domain_root else {
        eprintln!("{USAGE}");
        return Ok(2);
    };
    match bootstrap_domain(
        Path::new(&domain_root),
        token_id.as_deref(),
        scope.as_deref(),
        by.as_deref().unwrap_or(DEFAULT_OPENED_BY),
        date.as_deref(),
        client_config.as_deref().map(Path::new),
    )
    .await
    {
        Ok(result) => {
            emit(&result);
            Ok(0)
        }
        Err(e) => {
            let mut payload = e.payload;
            if e.exit_code == 1 {
                payload["files_kept"] = json!("验红时链文件留开域首笔不删，镜像行零写（如实）");
            }
            emit(&payload);
            Ok(e.exit_code)
        }
    }
}

// ------------------------------------------------------------------- 台面形

use axum::extract::Form;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};

const PAGE_STYLE: &str = concat!(
    "body{font-family:-apple-system,sans-serif;max-width:880px;margin:32px auto;",
    "padding:0 16px;color:#222}",
    "table{border-collapse:collapse;width:100%}td,th{border:1px solid #ccc;",
    "padding:6px 10px;text-align:left;font-size:14px}",
    "th{background:#f4f4f4}code{background:#f4f4f4;padding:1px 5px}",
    "form{margin:12px 0;padding:12px;border:1px solid #ddd;border-radius:6px}",
    "input,select{margin:2px 8px 2px 0;padding:4px}",
    ".btn{padding:6px 14px;cursor:pointer}.warn{color:#a00}",
    "h2{border-bottom:2px solid #eee;padding-bottom:6px}"
);

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// 台面页骨架（web.py _page 形照录：标题加正文加页脚三链接）。
fn page(title: &str, body: &str) -> String {
    format!(
        "<!doctype html><html lang=\"zh\"><head><meta charset=\"utf-8\">\
         <title>{}</title><style>{PAGE_STYLE}</style></head>\
         <body><h1>{}</h1>{body}\
         <p><a href=\"/manual\">AI 使用说明书</a> · <a href=\"/tokens\">返回管理台</a> · <a href=\"/\">返回面板</a></p>\
         </body></html>",
        html_escape(title),
        html_escape(title),
    )
}

fn html_page(title: &str, body: &str) -> Response {
    Html(page(title, body)).into_response()
}

/// 错误页（web.py _error_page 形：400 加拒语加处置语）。
fn html_error_page(message: &str, action: &str) -> Response {
    (
        axum::http::StatusCode::BAD_REQUEST,
        Html(page(
            "管理台输入被拒",
            &format!(
                "<p class=\"warn\">{}</p><p>处置：{}</p>",
                html_escape(message),
                html_escape(action)
            ),
        )),
    )
        .into_response()
}

/// 台面三路由：GET /tokens/open 开域表单、POST /tokens/open 开域确认面（一步
/// 确认防手滑）、POST /tokens/confirm-open 确认落笔。台面是薄壳，执行位调
/// bootstrap_domain 同一核心函数零二次实现；台面路径不含客户端注册。
pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/tokens/open", get(open_form_page).post(open_action))
        .route("/tokens/confirm-open", post(confirm_open_action))
}

/// 开域表单页（域根与标识词形与档位与事由四件，提交入确认面）。
async fn open_form_page() -> Response {
    let body = concat!(
        "<h2>域自举开域（两步确认）</h2>",
        "<form method=\"post\" action=\"/tokens/open\">",
        "域根（新城 git 仓根目录）：<input name=\"domain_root\" size=\"60\" required><br>",
        "标识词形（域根无 active 行时必填）：<input name=\"token_id\"><br>",
        "档位：<select name=\"scope\">",
        "<option value=\"domain_write\">domain_write</option>",
        "<option value=\"readonly\">readonly</option>",
        "<option value=\"custom\">custom</option></select><br>",
        "事由：<input name=\"by\" placeholder=\"main-window-bootstrap\"><br>",
        "<button class=\"btn\" type=\"submit\">下一步（确认面）</button>",
        "</form>"
    );
    html_page("开域（域自举全链）", body)
}

/// 开域表单复核：域根必填，scope 在档；标识与事由可空（空形由链教学）。
type OpenFormValues = (String, Option<String>, String, String);

fn validate_open_form(form: &HashMap<String, String>) -> Result<OpenFormValues, (String, String)> {
    let domain_root = form.get("domain_root").map(|s| s.trim().to_string()).unwrap_or_default();
    let token_id = form.get("token_id").map(|s| s.trim().to_string()).unwrap_or_default();
    let scope = form.get("scope").map(|s| s.trim().to_string()).unwrap_or_default();
    let scope = if scope.is_empty() { DEFAULT_SCOPE.to_string() } else { scope };
    let by = form.get("by").map(|s| s.trim().to_string()).unwrap_or_default();
    if domain_root.is_empty() {
        return Err(("域根缺席".to_string(), "补 domain_root（新城 git 仓根目录）".to_string()));
    }
    if !tokens::VALID_SCOPES.contains(&scope.as_str()) {
        return Err((format!("scope 非法：'{scope}'"), "scope 须三档之一".to_string()));
    }
    Ok((domain_root, if token_id.is_empty() { None } else { Some(token_id) }, scope, by))
}

/// 本域根 active 末行读数位（bootstrap.py active_row_for 对等，只读；多 active
/// 并存返 None 由调用方分流）。
pub fn active_row_for(rows: &std::collections::BTreeMap<String, TokenRow>, domain_root: &Path) -> Option<TokenRow> {
    let want = norm(domain_root);
    rows.values()
        .filter(|r| r.status == tokens::STATUS_ACTIVE && norm(Path::new(&r.domain_root)) == want)
        .next()
        .cloned()
}

/// 开域第一步：确认面（域根与签发计划与档位待复核，确认前零写入）。
async fn open_action(Form(form): Form<HashMap<String, String>>) -> Response {
    let (domain_root, token_id, scope, by) = match validate_open_form(&form) {
        Ok(v) => v,
        Err((m, a)) => return html_error_page(&m, &a),
    };
    let resolved = norm(Path::new(&domain_root)).display().to_string();
    let rows = match tokens::load_last_rows(&central_registry_path(&resolve_root())) {
        Ok(r) => r,
        Err(e) => return html_error_page(&e.to_string(), "登记册行形损坏候人节点处置，勿手工改册"),
    };
    let active = active_row_for(&rows, Path::new(&resolved));
    let plan = if let Some(row) = &active {
        format!("<p>复用现牌 <code>{}</code>（不重签）</p>", html_escape(&row.token_id))
    } else if let Some(tid) = &token_id {
        format!(
            "<p>将签发新牌 <code>{}</code>（档位 {}）</p>",
            html_escape(tid),
            html_escape(&scope)
        )
    } else {
        return html_error_page(
            "本域根无 active 标识牌行，全链签发须标识词形",
            "表单补 token_id（可读短串）后重提交",
        );
    };
    let body = format!(
        "<h2>开域确认（一步确认防手滑）</h2>\
         <p>域根 <code>{resolved}</code> · 档位 <code>{scope}</code> · 事由 <code>{by}</code></p>\
         {plan}\
         <p>确认即执行域自举全链（签发行落册加正典树落地加开域首笔落链加验域加域内镜像行），真链写入。</p>\
         <form method=\"post\" action=\"/tokens/confirm-open\">\
         <input type=\"hidden\" name=\"domain_root\" value=\"{resolved}\">\
         <input type=\"hidden\" name=\"token_id\" value=\"{token_id}\">\
         <input type=\"hidden\" name=\"scope\" value=\"{scope}\">\
         <input type=\"hidden\" name=\"by\" value=\"{by}\">\
         <button class=\"btn\" type=\"submit\">确认开域</button>\
         <a href=\"/tokens\"><button class=\"btn\" type=\"button\">取消</button></a></form>",
        resolved = html_escape(&resolved),
        scope = html_escape(&scope),
        by = html_escape(if by.is_empty() { DEFAULT_OPENED_BY } else { &by }),
        plan = plan,
        token_id = html_escape(token_id.as_deref().unwrap_or("")),
    );
    html_page("开域确认", &body)
}

/// 开域第二步：确认落笔，薄壳调 bootstrap_domain 单一 canonical 路径。
async fn confirm_open_action(Form(form): Form<HashMap<String, String>>) -> Response {
    let (domain_root, token_id, scope, by) = match validate_open_form(&form) {
        Ok(v) => v,
        Err((m, a)) => return html_error_page(&m, &a),
    };
    let by_owned = if by.is_empty() { DEFAULT_OPENED_BY.to_string() } else { by };
    let result = bootstrap_domain(
        Path::new(&domain_root),
        token_id.as_deref(),
        Some(&scope),
        &by_owned,
        None,
        None,
    )
    .await;
    let result = match result {
        Ok(r) => r,
        Err(e) => {
            let error = e.payload["error"].as_str().unwrap_or_default().to_string();
            let fix = e.payload["fix"].as_str().unwrap_or("按报文处置后重试").to_string();
            return html_error_page(&error, &fix);
        }
    };
    let issued = &result["issued"];
    let issued_done = issued["done"].as_bool().unwrap_or(false);
    let field = |k: &str| result[k].as_str().unwrap_or_default().to_string();
    let body = format!(
        "<p class=\"warn\">开域完成（全链落毕，链验 {chain}）。</p>\
         <table>\
         <tr><th>域名</th><td><code>{domain_id}</code></td></tr>\
         <tr><th>域根</th><td>{domain_root}</td></tr>\
         <tr><th>开域时</th><td>{opened_at}</td></tr>\
         <tr><th>签发</th><td>{issued_verb} <code>{issued_token}</code>（{issued_note}）</td></tr>\
         <tr><th>镜像行</th><td>{mirror}</td></tr>\
         <tr><th>开域首笔</th><td><code>{pen}</code></td></tr>\
         </table>\
         <p>客户端连接形：<code>Authorization: Bearer {domain_id}</code>（写操作只落所绑域）</p>",
        chain = html_escape(&field("chain_verify")),
        domain_id = html_escape(&field("domain_id")),
        domain_root = html_escape(&field("domain_root")),
        opened_at = html_escape(&field("opened_at")),
        issued_verb = if issued_done { "新签" } else { "复用现牌" },
        issued_token = html_escape(issued["token_id"].as_str().unwrap_or_default()),
        issued_note = html_escape(issued["note"].as_str().unwrap_or_default()),
        mirror = html_escape(&field("mirror_row_path")),
        pen = html_escape(&field("open_pen_hash")),
    );
    html_page("开域完成", &body)
}

// ------------------------------------------------------------------- 测试

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试基準位：中央 scribe 真二进制与模板源走真工作区根（code root），
    /// SIH_ROOT 指临时根（数据根）——runtime 两根分离；链追加只落临时根。
    const REAL_WS_ROOT: &str = "/Users/moc/workspaces/SiHankor";

    /// 环境变量是进程全局量：本模块涉 env 测试互斥串行，退出即还原。
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct EnvGuard {
        saved: Vec<(String, Option<String>)>,
        _lock: std::sync::MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        fn set(vars: &[(&str, &str)]) -> Self {
            let lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let mut saved = Vec::new();
            for (k, v) in vars {
                saved.push((k.to_string(), std::env::var(k).ok()));
                std::env::set_var(k, v);
            }
            Self { saved, _lock: lock }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, v) in self.saved.drain(..) {
                match v {
                    Some(val) => std::env::set_var(&k, val),
                    None => std::env::remove_var(&k),
                }
            }
        }
    }

    /// 临时新城根：中央根带中央登记册目录，域根带 .git（前置检查即过）。
    fn make_roots(tag: &str) -> (PathBuf, PathBuf) {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let base = std::env::temp_dir().join(format!("sihmcp-boot-{tag}-{}-{nanos}", std::process::id()));
        let central = base.join("central");
        let dom = base.join("dom");
        std::fs::create_dir_all(central.join("sih-tools/mcpline/ledger")).unwrap();
        std::fs::create_dir_all(dom.join(".git")).unwrap();
        (central, dom)
    }

    fn cleanup(base: &Path) {
        let _ = std::fs::remove_dir_all(base);
    }

    fn env_pair(central: &Path) -> Vec<(&'static str, String)> {
        vec![
            ("SIH_ROOT", central.to_string_lossy().to_string()),
            ("SIH_MCPLINE_CODE_ROOT", REAL_WS_ROOT.to_string()),
        ]
    }

    #[tokio::test]
    async fn happy_path_five_segments() {
        let (central, dom) = make_roots("happy");
        let vars = env_pair(&central);
        let _env = EnvGuard::set(
            &vars.iter().map(|(k, v)| (*k, v.as_str())).collect::<Vec<_>>(),
        );
        let result = bootstrap_domain(&dom, Some("dom-a"), None, "test-window", Some("2026-09-11"), None)
            .await
            .expect("全链应绿");
        // 出参键全集（files 与 open_pen_hash 承开域链）。
        for key in [
            "ok", "domain_id", "domain_root", "opened_at", "issued", "files", "open_pen_hash",
            "chain_verify", "mirror_row_path", "mirror_action", "readme_path", "client_config", "next",
        ] {
            assert!(result.get(key).is_some(), "出参缺键 {key}");
        }
        assert_eq!(result["ok"], json!(true));
        assert_eq!(result["domain_id"], "dom-a");
        assert_eq!(result["issued"]["done"], json!(true));
        assert_eq!(result["issued"]["token_id"], "dom-a");
        assert_eq!(result["issued"]["note"], "fresh_issue");
        assert_eq!(result["chain_verify"], "valid");
        assert_eq!(result["mirror_action"], "appended");
        assert_eq!(result["files"].as_array().unwrap().len(), 6);
        assert!(result["open_pen_hash"].as_str().unwrap().len() > 8);
        assert_eq!(result["client_config"]["written"], json!(false));
        // 段一至段五落地件俱在。
        assert!(dom.join("sih/domain.json").is_file());
        assert!(dom.join("sih/README.md").is_file());
        assert!(dom.join("sih/state/plan/TEMPLATE-任务包.md").is_file());
        assert!(dom.join("sih/state/parking/TEMPLATE-停泊材料.json").is_file());
        assert!(dom.join("sih/state/plan/TEMPLATE-意图-plain.json").is_file());
        assert!(dom.join("sih/event/trail/2026-09-11.ndjson").is_file());
        // 镜像位：中央行与镜像行六字段恒等。
        let central_rows = tokens::load_last_rows(&central_registry_path(&central)).unwrap();
        let mirror_rows = tokens::load_last_rows(Path::new(result["mirror_row_path"].as_str().unwrap())).unwrap();
        let c = central_rows.get("dom-a").unwrap();
        let m = mirror_rows.get("dom-a").unwrap();
        assert_eq!(c, m, "中央行与镜像行六字段恒等");
        assert_eq!(m.token_id, "dom-a");
        assert_eq!(m.scope, "domain_write");
        assert_eq!(m.status, "active");
        // 幂等守卫：已开域重跑即拒（零 reinit）。
        let err = bootstrap_domain(&dom, None, None, "test-window", Some("2026-09-11"), None)
            .await
            .unwrap_err();
        assert_eq!(err.exit_code, 1);
        assert!(err.payload["error"].as_str().unwrap().contains("域已开域"));
        cleanup(central.parent().unwrap());
    }

    #[tokio::test]
    async fn active_row_reused_no_reissue() {
        let (central, dom) = make_roots("reuse");
        // 预置中央 active 行（seed-a 绑本域根）：全链应复用不重签。
        let seed = tokens::issue_row("seed-a", &dom.to_string_lossy(), "readonly", "seed");
        tokens::append_row(&central_registry_path(&central), &seed).unwrap();
        let vars = env_pair(&central);
        let _env = EnvGuard::set(
            &vars.iter().map(|(k, v)| (*k, v.as_str())).collect::<Vec<_>>(),
        );
        let result = bootstrap_domain(&dom, None, None, "test-window", Some("2026-09-11"), None)
            .await
            .expect("复用现牌全链应绿");
        assert_eq!(result["issued"]["done"], json!(false));
        assert_eq!(result["issued"]["token_id"], "seed-a");
        assert_eq!(result["issued"]["note"], "active_row_reused");
        assert_eq!(result["domain_id"], "seed-a");
        assert_eq!(result["chain_verify"], "valid");
        // 中央册仍一行（零重签），镜像行恒等复用行。
        let central_text = std::fs::read_to_string(central_registry_path(&central)).unwrap();
        assert_eq!(central_text.lines().filter(|l| !l.trim().is_empty()).count(), 1);
        let mirror_rows =
            tokens::load_last_rows(Path::new(result["mirror_row_path"].as_str().unwrap())).unwrap();
        assert_eq!(mirror_rows.get("seed-a").unwrap(), &seed);
        cleanup(central.parent().unwrap());
    }

    #[tokio::test]
    async fn missing_token_id_rejects_zero_write() {
        let (central, dom) = make_roots("notoken");
        let vars = env_pair(&central);
        let _env = EnvGuard::set(
            &vars.iter().map(|(k, v)| (*k, v.as_str())).collect::<Vec<_>>(),
        );
        let err = bootstrap_domain(&dom, None, None, "test-window", Some("2026-09-11"), None)
            .await
            .unwrap_err();
        assert_eq!(err.exit_code, 1);
        assert!(err.payload["error"].as_str().unwrap().contains("--token-id"));
        assert!(err.payload["fix"].as_str().unwrap().contains("--token-id"));
        assert_eq!(err.payload["ok"], json!(false));
        assert!(err.payload["canonical_pointers"].as_array().unwrap().len() == 3);
        // 零写入：中央登记册缺席、域根 sih/ 未建（签发只在可开域教学后发生）。
        assert!(!central_registry_path(&central).exists());
        assert!(!dom.join("sih").exists());
        cleanup(central.parent().unwrap());
    }

    #[tokio::test]
    async fn client_config_merge_preserves_and_idempotent() {
        let (central, dom) = make_roots("clientcfg");
        let cfg = central.parent().unwrap().join("zcode-config.json");
        std::fs::write(
            &cfg,
            r#"{"other":{"keep":true},"mcp":{"servers":{"other-srv":{"type":"http","url":"http://example"}}}}"#,
        )
        .unwrap();
        let vars = env_pair(&central);
        let _env = EnvGuard::set(
            &vars.iter().map(|(k, v)| (*k, v.as_str())).collect::<Vec<_>>(),
        );
        let result = bootstrap_domain(
            &dom,
            Some("dom-c"),
            None,
            "test-window",
            Some("2026-09-11"),
            Some(&cfg),
        )
        .await
        .expect("全链应绿");
        assert_eq!(result["client_config"]["written"], json!(true));
        assert!(result["client_config"]["note"]
            .as_str()
            .unwrap()
            .contains("生效条件"));
        let text = std::fs::read_to_string(&cfg).unwrap();
        let obj: Value = serde_json::from_str(&text).unwrap();
        // 合并写：其余键原样保留，仅设 mcp.servers.sih 固定 payload。
        assert_eq!(obj["other"]["keep"], json!(true));
        assert_eq!(obj["mcp"]["servers"]["other-srv"]["url"], "http://example");
        assert_eq!(obj["mcp"]["servers"]["sih"]["type"], "http");
        assert_eq!(obj["mcp"]["servers"]["sih"]["url"], CLIENT_FACE_URL);
        assert_eq!(obj["mcp"]["servers"]["sih"]["headers"]["Authorization"], "Bearer dom-c");
        assert_eq!(obj["mcp"]["servers"]["sih"]["enabled"], json!(true));
        assert_eq!(obj["mcp"]["servers"]["sih"]["timeoutMs"], json!(60000));
        // 文件形：indent 2 尾单换行。
        assert!(text.ends_with("}\n"));
        // 幂等可重跑：读回再写逐字节恒等，写后复核过。
        let obj2: Value = serde_json::from_str(&text).unwrap();
        write_client_config(&cfg, "dom-c", obj2, "merged_preserved").unwrap();
        assert_eq!(std::fs::read_to_string(&cfg).unwrap(), text);
        verify_client_config(&cfg, "dom-c").unwrap();
        cleanup(central.parent().unwrap());
    }

    #[tokio::test]
    async fn date_invalid_is_tool_error() {
        // 词形违例在段一之前即拒（exit 2 工具异常形，零触碰任何面）。
        let err = bootstrap_domain(Path::new("/tmp/sihmcp-boot-absent"), None, None, "t", Some("2026-9-1"), None)
            .await
            .unwrap_err();
        assert_eq!(err.exit_code, 2);
        assert!(err.payload["error"].as_str().unwrap().contains("词形违例"));
        assert!(err.payload.get("fix").is_none(), "工具异常载荷零 fix 字段");
    }
}
