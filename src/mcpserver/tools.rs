//! beta 写面工具函数层：十二写工具的透传执行（DES-014 矩阵逐行）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/writeface/tools.py。每工具流
//! 程：路由层矩阵复核（静态表）→ 一对一结构卫（非执法判定）→ CLI argv 构造
//! → 子进程透传（退出码原样）→ 结果或错误载荷（静态裁剪映射）。会话号
//! server 端注入逐写操作（客户端不可代填），结果载会话号供客户端记录。

use serde_json::Value;

use super::errors::error_payload;
use super::matrix::is_tool_exposed;
use super::passthrough::{
    lease_claim_argv, lease_close_argv, lease_commit_argv, lease_lock_argv, lease_open_argv,
    lease_unlock_argv, lease_unclaim_argv, lease_wait_turn_argv, parse_session_id,
    passthrough_result, run_cli, scribe_append_argv, scribe_direct_argv, scribe_intent_argv,
    scribe_park_argv, LEASE_BIN_TIMEOUT, SCRIBE_TIMEOUT,
};
use super::runtime::valid_date;
use super::session::ConnectionSession;

const VALID_MODES: [&str; 3] = ["exclusive", "append", "auto"];
const VALID_STAGES: [&str; 2] = ["wip", "settle"];

// scribe 缺件报文静态短语集（pk-094 其二实勘）：MCP 层教学补位，静态短语匹
// 配，零动态拼接调用方可控数据进错误文本。
const CLI_FILE_MISSING_PHRASES: [&str; 5] =
    ["记录不存在", "记录件缺失", "验证件缺失", "报告不存在", "读数件不存在"];
const CLI_FILE_PATH_HINT: &str = "（教学补位：相关文件参应为 json 文件路径——域根相对形或绝对形，示例 work/record.json；内联文本与 md 形拒）";

fn tool_error(
    conn: &ConnectionSession,
    tool: &str,
    what: &str,
    params: &[String],
    message: &str,
    rc: i64,
    reason_code: Option<&str>,
    gate: Option<&str>,
    suggested_action: Option<&str>,
) -> Value {
    error_payload(
        tool,
        what,
        params,
        message,
        &conn.root.display().to_string(),
        &conn.agent_class,
        conn.session_id.as_deref(),
        rc,
        reason_code,
        gate,
        suggested_action,
    )
}

/// 绑定查找单点位（pk-093 其一归一位）：三态 bound 加 implicit 加 unbound。
fn binding_state(conn: &ConnectionSession) -> &'static str {
    if conn.session_id.is_some() {
        "bound"
    } else if conn.auto_open_error.is_some() {
        "implicit"
    } else {
        "unbound"
    }
}

fn precheck(conn: &ConnectionSession, tool: &str, what: &str, params: &[String]) -> Option<Value> {
    // 其一矩阵复核：分级不在矩阵行即拒透传（静态表，拒透传即不发起调用）。
    if !is_tool_exposed(tool, &conn.agent_class) {
        return Some(tool_error(
            conn,
            tool,
            what,
            params,
            &format!("授权矩阵拒透传：分级 {} 无该工具行", conn.agent_class),
            1,
            Some("matrix_not_exposed"),
            Some("路由层授权矩阵"),
            Some(
                "外部冷 agent 取最小写集（open、intent、append、park、lock、unlock、wait-turn、commit、close、claim）；直改与领取释放为本地可信位；takeover 与 bypass 对任何来源拒透传",
            ),
        ));
    }
    // 其二一对一结构卫：绑定态单点位同源查找；未绑定即教开法；已绑定再开即
    // 拒（禁多对一）；隐式占位态 close 放行透传（收约执法全归既有 CLI）。
    let state = binding_state(conn);
    if tool == "lease_open" {
        if state == "bound" {
            return Some(tool_error(
                conn,
                tool,
                what,
                params,
                &format!(
                    "本连接已绑定会话 {}，一连接恰好一会话（禁多对一）；若该会话即连接级隐式占位会话（连接级自动立会所绑），收约走 lease_close（close 能收隐式会话），收约后可显式重开",
                    conn.session_id.clone().unwrap_or_default()
                ),
                1,
                Some("session_already_bound"),
                Some("连接与会话一对一"),
                Some("沿用在册会话勿重开；会话号随每次写操作结果载出"),
            ));
        }
        if let Some(err) = &conn.auto_open_error {
            return Some(tool_error(
                conn,
                tool,
                what,
                params,
                &format!(
                    "连接级自动立会话未成（隐式占位事实在案）：{err}；可先 lease_close 按包收隐式占位会话（close 能收隐式会话）再显式重开"
                ),
                1,
                Some("auto_open_failed"),
                Some("连接级 lease open"),
                Some("lease_close 收隐式占位会话后经本工具显式重开，或候部署面修正配置"),
            ));
        }
        return None;
    }
    if state != "bound" {
        if tool == "lease_close" && state == "implicit" {
            // 隐式占位会话归一（pk-093 其一）：隐式占位态放行 close 透传。
            return None;
        }
        return Some(tool_error(
            conn,
            tool,
            what,
            params,
            "本连接尚无绑定会话：一切写操作须在 lease_open 立会话之后；若本连接处于隐式占位态（连接级自动立会话未成的既成事实），lease_close 可按包收隐式会话，收约后再 lease_open",
            1,
            Some("session_not_bound"),
            Some("连接与会话一对一"),
            Some("先调 lease_open（package、intent、repo、allow）立会话"),
        ));
    }
    None
}

fn date_or_error(conn: &ConnectionSession, tool: &str, what: &str, params: &[String], date: Option<&str>) -> Option<Value> {
    if let Some(d) = date {
        if !valid_date(d) {
            return Some(tool_error(
                conn,
                tool,
                what,
                params,
                &format!("date 非法: '{d}'，须 YYYY-MM-DD"),
                2,
                Some("invalid_param"),
                Some("参数校验（调用形教学位）"),
                Some("按 YYYY-MM-DD 传 date 或缺省取实日"),
            ));
        }
    }
    None
}

/// 透传执行与载荷：退出码原样，错误走静态裁剪映射，成功登记写迹。
/// what 与 params 喂满位（pk-091 其四）：CLI 失败路径载荷四字段禁喂空。
async fn finish(
    conn: &mut ConnectionSession,
    tool: &str,
    argv: &[String],
    timeout: f64,
    what: &str,
    params: &[String],
    is_write: bool,
    on_success: Option<fn(&mut ConnectionSession)>,
) -> Value {
    let out = run_cli(argv, timeout).await;
    if out.rc != 0 {
        let mut msg = format!(
            "CLI 退出码 {}: {}",
            out.rc,
            (if out.stderr.is_empty() { &out.stdout } else { &out.stderr })
                .trim()
                .chars()
                .take(600)
                .collect::<String>()
        );
        if CLI_FILE_MISSING_PHRASES.iter().any(|p| msg.contains(p)) {
            msg.push_str(CLI_FILE_PATH_HINT);
        }
        let fallback = "既有 CLI 写操作透传（参数透传、结果透传、退出码透传）";
        return tool_error(conn, tool, if what.is_empty() { fallback } else { what }, params, &msg, out.rc as i64, None, None, None);
    }
    if is_write {
        conn.record_write();
    }
    if let Some(hook) = on_success {
        hook(conn);
    }
    passthrough_result(tool, conn.session_id.as_deref(), out.rc, &out.stdout, &out.stderr)
}

fn plist(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

// ------------------------------------------------------------------ lease_open

pub async fn tool_lease_open(
    conn: &mut ConnectionSession,
    package: Option<String>,
    intent: Option<String>,
    repo: Option<Vec<String>>,
    allow: Option<Vec<String>>,
) -> Value {
    let what = "lease open：立约开工立连接会话（一连接恰好一会话），以 server 进程正身件签发。";
    let params = plist(&[
        "package: 任务包 stem 或路径（必填）",
        "intent: 意图件路径（ask3 或 plain 意图记录 JSON 文件，必填，闸三意图验原位；自由文本或内联散文即 intent record unreadable 拒）",
        "repo: 目标仓工作区相对路径列表（可缺省，缺省 sih-engine）",
        "allow: 写入范围路径列表（可缺省）",
    ]);
    if let Some(pre) = precheck(conn, "lease_open", what, &params) {
        return pre;
    }
    let package = package.unwrap_or_default();
    let intent = intent.unwrap_or_default();
    if package.is_empty() || intent.is_empty() {
        return tool_error(
            conn,
            "lease_open",
            what,
            &params,
            "缺必填参数：package 与 intent",
            2,
            Some("invalid_param"),
            Some("参数校验（调用形教学位）"),
            Some("补 package（任务包 stem 或路径）与 intent（ask3 记录路径）"),
        );
    }
    let report = conn.ensure_identity().await;
    if report.is_none() {
        return tool_error(
            conn,
            "lease_open",
            what,
            &params,
            &format!("正身签发未成：{}", conn.identity_error.clone().unwrap_or_default()),
            2,
            Some("identity_issue_failed"),
            Some("进程正身签发"),
            Some("候 server 重签正身件后重试"),
        );
    }
    let argv = lease_open_argv(
        &conn.root,
        &report.unwrap(),
        &package,
        &intent,
        &repo.unwrap_or_default(),
        &allow.unwrap_or_default(),
    );
    let out = run_cli(&argv, LEASE_BIN_TIMEOUT).await;
    if out.rc != 0 {
        return tool_error(
            conn,
            "lease_open",
            what,
            &params,
            &format!(
                "lease open 退出码 {}: {}",
                out.rc,
                (if out.stderr.is_empty() { &out.stdout } else { &out.stderr })
                    .trim()
                    .chars()
                    .take(600)
                    .collect::<String>()
            ),
            out.rc as i64,
            None,
            None,
            None,
        );
    }
    let sid = parse_session_id(&out.stdout);
    if sid.is_none() {
        return tool_error(
            conn,
            "lease_open",
            what,
            &params,
            "lease open 成而出参未载会话号（出参形漂移即异常）",
            2,
            Some("open_output_shape"),
            Some("出参投影"),
            Some("候 server 侧对表出参形漂移"),
        );
    }
    let sid = sid.unwrap();
    conn.bind(sid.clone(), package);
    passthrough_result("lease_open", Some(&sid), out.rc, &out.stdout, &out.stderr)
}

// --------------------------------------------------------------- record 三具

pub async fn tool_record_intent(
    conn: &mut ConnectionSession,
    record: Option<String>,
    validation: Option<String>,
    date: Option<String>,
) -> Value {
    let what = "record intent：书简意图笔透传 scribe intent，闸二重意图拒与闸三会话在册验原位；plain 形（零哲学引文）validation 豁免，ask3 形验证件必填（DES-016）。";
    let params = plist(&[
        "record: 意图记录 json 路径（必填，闸二幂等键；应为 json 文件路径，域根相对形或绝对形，示例 work/intent-2026-09-11.json；内联散文拒）",
        "validation: 验证件 json 路径（ask3 形必填；plain 形可缺省；应为 json 文件路径，示例 work/validation-2026-09-11.json）",
        "date: 链日 YYYY-MM-DD（可缺省取实日）",
    ]);
    if let Some(pre) = precheck(conn, "record_intent", what, &params) {
        return pre;
    }
    if let Some(de) = date_or_error(conn, "record_intent", what, &params, date.as_deref()) {
        return de;
    }
    let record = record.unwrap_or_default();
    if record.is_empty() {
        return tool_error(
            conn,
            "record_intent",
            what,
            &params,
            "缺必填参数：record",
            2,
            Some("invalid_param"),
            Some("参数校验（调用形教学位）"),
            Some("补 record 路径；ask3 形另补 validation 路径"),
        );
    }
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = scribe_intent_argv(
        &conn.root,
        &session_id,
        &record,
        validation.as_deref().filter(|s| !s.is_empty()),
        date.as_deref(),
    );
    finish(conn, "record_intent", &argv, SCRIBE_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_record_append(
    conn: &mut ConnectionSession,
    report: Option<String>,
    exit_code: Option<i64>,
    date: Option<String>,
) -> Value {
    let what = "record append：认证笔透传 scribe append，闸三会话在册验原位；append 无幂等闸如实申报。";
    let params = plist(&[
        "report: JSON 报告文件路径（必填；应为 json 文件路径，域根相对形或绝对形，示例 work/report-2026-09-11.json；报告须是 JSON 文件，md 与内联文本两连拒）",
        "exit_code: 所载退出码整数（必填）",
        "date: 链日 YYYY-MM-DD（可缺省取实日）",
    ]);
    if let Some(pre) = precheck(conn, "record_append", what, &params) {
        return pre;
    }
    if let Some(de) = date_or_error(conn, "record_append", what, &params, date.as_deref()) {
        return de;
    }
    let report = report.unwrap_or_default();
    if report.is_empty() || exit_code.is_none() {
        return tool_error(
            conn,
            "record_append",
            what,
            &params,
            "缺必填参数：report 与 exit_code",
            2,
            Some("invalid_param"),
            Some("参数校验（调用形教学位）"),
            Some("补 report 路径与 exit_code 整数"),
        );
    }
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = scribe_append_argv(&conn.root, &session_id, &report, exit_code.unwrap(), date.as_deref());
    finish(conn, "record_append", &argv, SCRIBE_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_record_park(
    conn: &mut ConnectionSession,
    record: Option<String>,
    date: Option<String>,
) -> Value {
    let what = "record park：停泊事件笔透传 scribe park。";
    let params = plist(&[
        "record: 停泊记录 json 路径（必填；应为 json 文件路径，域根相对形或绝对形，示例 work/park-pk-094.json；内联文本拒）",
        "date: 链日 YYYY-MM-DD（可缺省取实日）",
    ]);
    if let Some(pre) = precheck(conn, "record_park", what, &params) {
        return pre;
    }
    if let Some(de) = date_or_error(conn, "record_park", what, &params, date.as_deref()) {
        return de;
    }
    let record = record.unwrap_or_default();
    if record.is_empty() {
        return tool_error(
            conn,
            "record_park",
            what,
            &params,
            "缺必填参数：record",
            2,
            Some("invalid_param"),
            Some("参数校验（调用形教学位）"),
            Some("补停泊记录 json 路径"),
        );
    }
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = scribe_park_argv(&conn.root, &session_id, &record, date.as_deref());
    finish(conn, "record_park", &argv, SCRIBE_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_record_direct(
    conn: &mut ConnectionSession,
    record: Option<String>,
    date: Option<String>,
) -> Value {
    let what = "record direct：直改笔透传 scribe direct（仅本地可信行；免全套租约仪式但免不了治理，agent 笔强制挂正身报告）。";
    let params = plist(&[
        "record: 直改链笔 json 路径（必填，pen_form=direct 且 pen_kind=agent；应为 json 文件路径，域根相对形或绝对形，示例 work/direct-pen.json）",
        "date: 链日 YYYY-MM-DD（可缺省取实日）",
    ]);
    if let Some(pre) = precheck(conn, "record_direct", what, &params) {
        return pre;
    }
    if let Some(de) = date_or_error(conn, "record_direct", what, &params, date.as_deref()) {
        return de;
    }
    let record = record.unwrap_or_default();
    if record.is_empty() {
        return tool_error(
            conn,
            "record_direct",
            what,
            &params,
            "缺必填参数：record",
            2,
            Some("invalid_param"),
            Some("参数校验（调用形教学位）"),
            Some("补直改链笔 json 路径（pen_form=direct 且 pen_kind=agent）"),
        );
    }
    let report = conn.ensure_identity().await;
    if report.is_none() {
        return tool_error(
            conn,
            "record_direct",
            what,
            &params,
            &format!("正身签发未成：{}", conn.identity_error.clone().unwrap_or_default()),
            2,
            Some("identity_issue_failed"),
            Some("进程正身签发"),
            Some("候 server 重签正身件后重试"),
        );
    }
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = scribe_direct_argv(
        &conn.root,
        &session_id,
        &report.unwrap(),
        &record,
        date.as_deref(),
    );
    finish(conn, "record_direct", &argv, SCRIBE_TIMEOUT, what, &params, true, None).await
}

// ------------------------------------------------------------------ lease 余具

pub async fn tool_lease_lock(
    conn: &mut ConnectionSession,
    path: Option<String>,
    mode: Option<String>,
    wait: Option<bool>,
) -> Value {
    let what = "lease lock：租内取锁，悲观五验（在册、择定、范围、身份、绑定）原位；撞锁拒或 --wait 入队受理。";
    let params = plist(&[
        "path: 工作区根相对路径（必填）",
        "mode: 锁型 exclusive|append|auto（可缺省，auto 为既有缺省语义）",
        "wait: 撞锁入队受理返位次（可缺省 false）",
    ]);
    if let Some(pre) = precheck(conn, "lease_lock", what, &params) {
        return pre;
    }
    let path = path.unwrap_or_default();
    if path.is_empty() {
        return tool_error(conn, "lease_lock", what, &params, "缺必填参数：path", 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("补工作区根相对路径"));
    }
    let mode_s = mode.clone();
    if let Some(m) = &mode_s {
        if !VALID_MODES.contains(&m.as_str()) {
            return tool_error(conn, "lease_lock", what, &params,
                &format!("mode 非法: '{m}'，须 exclusive|append|auto"), 2,
                Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("改 mode 为三值之一或缺省"));
        }
    }
    // 锁类五验的逐写正身复核位：身份报告缺席即先签发（签发即采集，缓存复用）。
    let report = match conn.ensure_identity().await {
        Some(r) => r,
        None => {
            return tool_error(conn, "lease_lock", what, &params,
                &format!("正身签发未成：{}", conn.identity_error.clone().unwrap_or_default()), 2,
                Some("identity_issue_failed"), Some("进程正身签发"), Some("候 server 重签正身件后重试"));
        }
    };
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = lease_lock_argv(&conn.root, &report, &session_id, &path, mode_s.as_deref(), wait.unwrap_or(false));
    finish(conn, "lease_lock", &argv, LEASE_BIN_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_lease_unlock(conn: &mut ConnectionSession, path: Option<String>) -> Value {
    let what = "lease unlock：放锁，同一五验加持锁验原位。";
    let params = plist(&["path: 工作区根相对路径（必填）"]);
    if let Some(pre) = precheck(conn, "lease_unlock", what, &params) {
        return pre;
    }
    let path = path.unwrap_or_default();
    if path.is_empty() {
        return tool_error(conn, "lease_unlock", what, &params, "缺必填参数：path", 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("补工作区根相对路径"));
    }
    let report = match conn.ensure_identity().await {
        Some(r) => r,
        None => {
            return tool_error(conn, "lease_unlock", what, &params,
                &format!("正身签发未成：{}", conn.identity_error.clone().unwrap_or_default()), 2,
                Some("identity_issue_failed"), Some("进程正身签发"), Some("候 server 重签正身件后重试"));
        }
    };
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = lease_unlock_argv(&conn.root, &report, &session_id, &path);
    finish(conn, "lease_unlock", &argv, LEASE_BIN_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_lease_wait_turn(
    conn: &mut ConnectionSession,
    path: Option<String>,
    timeout_seconds: Option<f64>,
    interval_seconds: Option<f64>,
    mode: Option<String>,
) -> Value {
    let what = "lease wait-turn：排队阻塞至轮到并取锁（机械轮询零 LLM 零 token 消耗语义）；timeout 到即如实返退出码与出队事实。";
    let params = plist(&[
        "path: 工作区根相对路径（必填）",
        "timeout_seconds: 等待上限秒（可缺省，缺省无限）",
        "interval_seconds: 机械查询间隔秒（可缺省，资源性参数）",
        "mode: 锁型（可缺省）",
    ]);
    if let Some(pre) = precheck(conn, "lease_wait_turn", what, &params) {
        return pre;
    }
    let path = path.unwrap_or_default();
    if path.is_empty() {
        return tool_error(conn, "lease_wait_turn", what, &params, "缺必填参数：path", 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("补工作区根相对路径"));
    }
    let report = match conn.ensure_identity().await {
        Some(r) => r,
        None => {
            return tool_error(conn, "lease_wait_turn", what, &params,
                &format!("正身签发未成：{}", conn.identity_error.clone().unwrap_or_default()), 2,
                Some("identity_issue_failed"), Some("进程正身签发"), Some("候 server 重签正身件后重试"));
        }
    };
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = lease_wait_turn_argv(&conn.root, &report, &session_id, &path,
        mode.as_deref(), timeout_seconds, interval_seconds);
    finish(conn, "lease_wait_turn", &argv, LEASE_BIN_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_lease_claim(
    conn: &mut ConnectionSession,
    package: Option<String>,
    ttl: Option<i64>,
    claimant: Option<String>,
) -> Value {
    let what = "lease claim：领取登记（声明位非执法位），同包未过期在领即拒 PackageAlreadyClaimed。";
    let params = plist(&[
        "package: 任务包 stem 或路径（必填）",
        "ttl: 领取时效分钟数正整数（必填）",
        "claimant: 领取人标识（可缺省，缺省 anon）",
    ]);
    if let Some(pre) = precheck(conn, "lease_claim", what, &params) {
        return pre;
    }
    let package = package.unwrap_or_default();
    if package.is_empty() || ttl.is_none() {
        return tool_error(conn, "lease_claim", what, &params, "缺必填参数：package 与 ttl", 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("补 package 与 ttl（分钟数正整数）"));
    }
    let argv = lease_claim_argv(&conn.root, &package, ttl.unwrap(), claimant.as_deref());
    finish(conn, "lease_claim", &argv, LEASE_BIN_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_lease_unclaim(
    conn: &mut ConnectionSession,
    package: Option<String>,
    claimant: Option<String>,
) -> Value {
    let what = "lease unclaim：领取释放（仅本地可信行），无在领即拦。";
    let params = plist(&[
        "package: 任务包 stem 或路径（必填）",
        "claimant: 领取人标识（可缺省，给参即验与在领相符）",
    ]);
    if let Some(pre) = precheck(conn, "lease_unclaim", what, &params) {
        return pre;
    }
    let package = package.unwrap_or_default();
    if package.is_empty() {
        return tool_error(conn, "lease_unclaim", what, &params, "缺必填参数：package", 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("补任务包 stem 或路径"));
    }
    let argv = lease_unclaim_argv(&conn.root, &package, claimant.as_deref());
    finish(conn, "lease_unclaim", &argv, LEASE_BIN_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_lease_commit(
    conn: &mut ConnectionSession,
    repo: Option<Value>,
    stage: Option<String>,
    subject: Option<String>,
    seq: Option<i64>,
    cert: Option<String>,
    note: Option<String>,
    trail: Option<Vec<String>>,
    root: Option<String>,
) -> Value {
    let what = "lease commit：提交四验后机械 message 落笔；settle 须 seq 与 cert（链上认证哈希）。";
    let params = plist(&[
        "repo: 目标仓路径（必填；工位相对路径形为成功形，点号形会撞主检出直提拒；str 与列表双形接受，列表归一单仓——CLI --repo 单值形（lease/cli.py commit_cmd 非 action=append 实勘），多仓须逐仓各调一笔）",
        "stage: wip|settle（必填）",
        "subject: 一段事述（必填）",
        "seq: 段序（settle 必填）",
        "cert: 认证哈希（settle 必填即该段认证在链）",
        "note: 附注（可缺省，偏离声明等）",
        "trail: 链路径列表（可缺省，逐条透传 CLI --trail；缺席即 CLI 缺省全链按日序发现，cert 四验收敛域内链须显式传——pk-091 其三透传补位）",
        "root: 工作区根覆写（可缺省，缺省域根，透传 CLI --root）",
    ]);
    if let Some(pre) = precheck(conn, "lease_commit", what, &params) {
        return pre;
    }
    let stage = stage.unwrap_or_default();
    let subject = subject.unwrap_or_default();
    // repo 双形归一（pk-094 其一）：str 与单元素列表双形接受，多条教学拒。
    let repos: Vec<String> = match repo {
        Some(Value::String(s)) => vec![s],
        Some(Value::Array(a)) => a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect(),
        Some(Value::Null) | None => vec![],
        Some(other) => vec![other.to_string()],
    };
    if repos.is_empty() || subject.is_empty() {
        return tool_error(conn, "lease_commit", what, &params, "缺必填参数：repo 与 subject", 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("补 repo 路径与 subject 事述"));
    }
    if repos.len() != 1 {
        return tool_error(conn, "lease_commit", what, &params,
            &format!("repo 列表形须恰单仓（CLI commit --repo 单值形，一笔提交落一仓）：收 {} 条", repos.len()),
            2, Some("invalid_param"), Some("参数校验（调用形教学位）"),
            Some("多仓逐仓各调一笔 lease_commit，每笔 repo 传单仓工位相对路径"));
    }
    if !VALID_STAGES.contains(&stage.as_str()) {
        return tool_error(conn, "lease_commit", what, &params,
            &format!("stage 非法: '{stage}'，须 wip|settle"), 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("改 stage 为 wip 或 settle"));
    }
    let session_id = conn.session_id.clone().unwrap_or_default();
    let argv = lease_commit_argv(&conn.root, &session_id, &repos[0], &stage, &subject,
        seq, cert.as_deref(), note.as_deref(), &trail.unwrap_or_default(), root.as_deref());
    finish(conn, "lease_commit", &argv, LEASE_BIN_TIMEOUT, what, &params, true, None).await
}

pub async fn tool_lease_close(
    conn: &mut ConnectionSession,
    package: Option<String>,
    reason: Option<String>,
) -> Value {
    let what = "lease close：收约（锁清零、分支归并、拆本吊销），受 close 链闸与 closeguard 约束；强拆与绕行旗标不透传。";
    let params = plist(&[
        "package: 任务包 stem 或路径（必填）",
        "reason: 吊销事由（可缺省）",
    ]);
    if let Some(pre) = precheck(conn, "lease_close", what, &params) {
        return pre;
    }
    let package = package.unwrap_or_default();
    if package.is_empty() {
        return tool_error(conn, "lease_close", what, &params, "缺必填参数：package", 2,
            Some("invalid_param"), Some("参数校验（调用形教学位）"), Some("补任务包 stem 或路径"));
    }
    let argv = lease_close_argv(&conn.root, &package, reason.as_deref().filter(|s| !s.is_empty()), None);
    finish(
        conn,
        "lease_close",
        &argv,
        LEASE_BIN_TIMEOUT,
        what,
        &params,
        true,
        Some(|c: &mut ConnectionSession| {
            // 隐式占位归一（pk-093 其一）：收约成即隐式占位事实清零。
            c.mark_client_closed();
            c.auto_open_error = None;
        }),
    )
    .await
}
