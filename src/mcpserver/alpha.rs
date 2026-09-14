//! alpha 相九读数具：只读投影（SPEC-023 契约面）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/server.py 缺省分支（stdio 形
//! 与匿名 HTTP 面承载形）。chain_query 与 chain_verify 与 retriever 的引擎面
//! 走进程内库调（消子进程缺陷族）；Python 工具族（critsweep 加 gauge 加 lease
//! 查册加 nomenclator）子进程包裹与现行 argv 逐一对应。零写入、零判定语义。

use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};

use super::runtime::{
    code_root, error_payload, lease_bin, retriever_bin, resolve_root, run_readonly, today_str,
    trail_path, valid_date, RunOutcome, CANON_LINE_PKG, CANON_SPEC_023,
};

const NAMING_TEACHING: &str = "立名程序摘要（序承先乙后名再甲）：一、乙前注入——立名动作前先拉 nomenclator map --concept 语义映射报告入上下文（概念锚查词典六态与既裁 code 形与近邻词，只报不判），防想错了；二、查册对表——命名动作查册对象是概念本尊非只造出的词形，nomenclator_query 查概念词与候选词形并查命名集与检词（DEC-017 修订四常设纪律）；三、看真材料取名后填甲表三件——概念锚 zh、既裁 code 形或显式申报无承、语素派生；四、甲机械兜底——lease open --new-stem 带甲表，stem 闸机械核对指称完整与派生对表，填不圆即拒（零 LLM 判词位），防真的想错了；五、立名程序终裁登记——语义忠实终裁归立名程序人节点，收敛后 nomenclator register 在册。正典：sih-engine/doc/decision/017-wengu-naming.md；.agents/skills/sihankor-naming/SKILL.md";

fn nom_project_dir() -> PathBuf {
    code_root().join("sih-tools").join("nomenclator")
}

fn nom_pack() -> PathBuf {
    code_root().join("sih-tools").join("nomenclator").join("packs").join("core")
}

fn argv_of(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}

// ---------------------------------------------------------------- chain_query

pub async fn chain_query(root: PathBuf, date: Option<String>, event_type: Option<String>) -> Value {
    let what = "查当日治理链事件清单（哈希、事件型、主体字段），只读投影 scribe query。";
    let params = ["date: YYYY-MM-DD，缺省实日", "event_type: 可选事件型过滤字符串"];
    let date = date.unwrap_or_else(today_str);
    if !valid_date(&date) {
        return error_payload(
            "chain_query",
            what,
            &params,
            &format!("date 非法: '{date}'，须 YYYY-MM-DD"),
        );
    }
    let trail = trail_path(&root, &date);
    if !trail.is_file() {
        return error_payload(
            "chain_query",
            what,
            &params,
            &format!("链文件缺席: {}（该日零链或日期拼写误）", trail.display()),
        );
    }
    // 进程内库调（消子进程缺陷族）：load_events + query（全量，matches 即全量数）
    let events = match crate::event_stream::load_events(&trail) {
        Ok(e) => e,
        Err(e) => {
            return error_payload(
                "chain_query",
                what,
                &params,
                &format!("scribe query 退出码 2: {e:?}"),
            );
        }
    };
    let list = crate::event_stream::query(&events, None, None);
    let matches = list.events.len();
    let mapped: Vec<Value> = list
        .events
        .iter()
        .filter(|ev| event_type.as_ref().map_or(true, |t| ev.event_type == *t))
        .map(|ev| {
            let raw = serde_json::to_value(ev).unwrap_or(Value::Null);
            json!({
                "event_hash": raw.get("event_hash"),
                "event_type": raw.get("event_type"),
                "subject": raw.get("details").and_then(|d| d.get("subject")).cloned().unwrap_or(Value::Null),
                "doc_id": raw.get("doc_id"),
                "timestamp": raw.get("timestamp"),
            })
        })
        .collect();
    json!({
        "date": date,
        "trail": trail.display().to_string(),
        "event_type_filter": event_type,
        "matches": matches,
        "returned": mapped.len(),
        "events": mapped,
    })
}

// --------------------------------------------------------------- chain_verify

pub async fn chain_verify(root: PathBuf, date: Option<String>) -> Value {
    let what = "验当日治理链：逐笔哈希校验与整链 valid 判词，只读投影 scribe verify。";
    let params = ["date: YYYY-MM-DD（必填）"];
    let Some(date) = date else {
        return error_payload("chain_verify", what, &params, "date 非法: ''，须 YYYY-MM-DD");
    };
    if !valid_date(&date) {
        return error_payload(
            "chain_verify",
            what,
            &params,
            &format!("date 非法: '{date}'，须 YYYY-MM-DD"),
        );
    }
    let trail = trail_path(&root, &date);
    if !trail.is_file() {
        return error_payload(
            "chain_verify",
            what,
            &params,
            &format!("链文件缺席: {}（该日零链或日期拼写误）", trail.display()),
        );
    }
    // 进程内库调（消子进程缺陷族）：load_events + verify 全程
    let events = match crate::event_stream::load_events(&trail) {
        Ok(e) => e,
        Err(e) => {
            return error_payload(
                "chain_verify",
                what,
                &params,
                &format!("scribe verify 退出码 2: {e:?}"),
            );
        }
    };
    match crate::event_stream::verify(&events, crate::event_stream::VerifyRange::Full) {
        Ok(ok) => {
            let status = "valid";
            json!({
                "date": date,
                "trail": trail.display().to_string(),
                "status": status,
                "valid": status == "valid",
                "events": ok.event_count,
                "first_hash": ok.first_hash,
                "last_hash": ok.last_hash,
            })
        }
        Err(_) => {
            // 现行形：scribe verify 拒即 rc=1 且报文走 stdout，err 面空（对等保真）
            error_payload("chain_verify", what, &params, "scribe verify 退出码 1: ")
        }
    }
}

// ------------------------------------------------------------------ critsweep

pub async fn critsweep(root: PathBuf, date: Option<String>) -> Value {
    let what = "判据扫：GOV-002 五判据三态与泊界路由与两账本在飞，严格 JSON 单对象，只读投影 sweep.py。";
    let params = ["date: YYYY-MM-DD（必填，--at 参照日）"];
    let Some(date) = date else {
        return error_payload("critsweep", what, &params, "date 非法: ''，须 YYYY-MM-DD");
    };
    if !valid_date(&date) {
        return error_payload(
            "critsweep",
            what,
            &params,
            &format!("date 非法: '{date}'，须 YYYY-MM-DD"),
        );
    }
    let argv = argv_of(&[
        "python3",
        &code_root().join("sih-tools/critsweep/sweep.py").display().to_string(),
        "--at",
        &date,
        "--root",
        &root.display().to_string(),
    ]);
    let out = run_readonly(&argv, None, None, std::time::Duration::from_secs(120)).await;
    if out.rc != 0 {
        return error_payload(
            "critsweep",
            what,
            &params,
            &format!("sweep.py 退出码 {}: {}", out.rc, truncate(&out.stderr, 400)),
        );
    }
    match serde_json::from_str::<Value>(&out.stdout) {
        Ok(v) if v.is_object() => v,
        Ok(_) => error_payload("critsweep", what, &params, "sweep.py 出参非单对象"),
        Err(e) => error_payload(
            "critsweep",
            what,
            &params,
            &format!("sweep.py 出参非 JSON: {e}"),
        ),
    }
}

fn truncate(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

// ------------------------------------------------------------------ heartbeat

const GAUGE_DIMS: [&str; 3] = ["convergence", "adoption", "mergeback"];

pub async fn heartbeat(root: PathBuf) -> Value {
    let what = "心跳：秤星三维最新读数与距上快照间隔日，只读投影 gauge.cli read（只读不落链）。";
    let params: [&str; 0] = [];
    let date = today_str();
    let trail = trail_path(&root, &date);
    if !trail.is_file() {
        return error_payload(
            "heartbeat",
            what,
            &params,
            &format!("当日链文件缺席: {}（心跳读数以当日链为准）", trail.display()),
        );
    }
    let gauge_dir = root.join("sih-tools/gauge");
    let sessions_ledger = root.join("sih-tools/lease/ledger/sessions.ndjson");
    let src_root = root.join("sih-engine");
    let tools_root = root.join("sih-tools");
    // 域链全日展开：trail 目录全日 sorted glob（pk-090 件六根因修复形）
    let mut trails: Vec<PathBuf> = match std::fs::read_dir(trail.parent().unwrap()) {
        Ok(rd) => rd
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map_or(false, |x| x == "ndjson"))
            .collect(),
        Err(_) => vec![],
    };
    trails.sort();
    let trail_strs: Vec<String> = trails.iter().map(|p| p.display().to_string()).collect();
    let env_extra = [("PYTHONPATH", "src")];
    let mut readings = Map::new();
    let mut snapshot_dates: Vec<String> = vec![];
    for dim in GAUGE_DIMS {
        let mut argv = vec![
            "python3".to_string(),
            "-m".to_string(),
            "gauge.cli".to_string(),
            "read".to_string(),
            "--dimension".to_string(),
            dim.to_string(),
            "--at".to_string(),
            date.clone(),
        ];
        for t in &trail_strs {
            argv.push("--trail".to_string());
            argv.push(t.clone());
        }
        argv.push("--sessions-ledger".to_string());
        argv.push(sessions_ledger.display().to_string());
        argv.push("--src-root".to_string());
        argv.push(src_root.display().to_string());
        argv.push("--tools-root".to_string());
        argv.push(tools_root.display().to_string());
        let out = run_readonly(&argv, Some(&gauge_dir), Some(&env_extra), std::time::Duration::from_secs(120)).await;
        if out.rc != 0 {
            return error_payload(
                "heartbeat",
                what,
                &params,
                &format!("gauge read({dim}) 退出码 {}: {}", out.rc, truncate(&out.stderr, 400)),
            );
        }
        let raw: Value = match serde_json::from_str(&out.stdout) {
            Ok(v) => v,
            Err(e) => {
                return error_payload(
                    "heartbeat",
                    what,
                    &params,
                    &format!("gauge read({dim}) 出参非 JSON: {e}"),
                );
            }
        };
        let reading = raw.get("reading").cloned().unwrap_or(Value::Null);
        let value = reading.get("value").cloned().unwrap_or(Value::Null);
        let computed_at = reading.get("computed_at").cloned().unwrap_or(Value::Null);
        let sequence = raw.get("sequence").cloned().unwrap_or(Value::Null);
        readings.insert(
            dim.to_string(),
            json!({"value": value, "computed_at": computed_at, "sequence": sequence}),
        );
        if let Some(history) = raw.get("history").and_then(|h| h.as_array()) {
            for h in history {
                if let Some(ca) = h.get("computed_at").and_then(|c| c.as_str()) {
                    snapshot_dates.push(ca.to_string());
                }
            }
        }
    }
    let last_snapshot = snapshot_dates.iter().max().cloned();
    let mut days_since = Value::Null;
    if let Some(ls) = &last_snapshot {
        let d1 = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok();
        let d2 = chrono::NaiveDate::parse_from_str(&ls[..ls.len().min(10)], "%Y-%m-%d").ok();
        if let (Some(d1), Some(d2)) = (d1, d2) {
            days_since = json!((d1 - d2).num_days());
        }
    }
    json!({
        "date": date,
        "readings": readings,
        "last_snapshot_at": last_snapshot,
        "days_since_last_snapshot": days_since,
    })
}

// ----------------------------------------------------------------- locks_read

pub async fn locks_read(root: PathBuf) -> Value {
    let what = "锁面读数：未释放锁成对核算与活跃会话数，只读投影 lease status（查册）。";
    let params: [&str; 0] = [];
    // 引擎 bin 位直调（recognize-solo：uv 裸名在 launchd 极简 PATH 下 spawn
    // 失败 127 病灶根除；台账路径经域布局投影显式传参，两形域各归其位）。
    let argv = {
        let central = resolve_root();
        let lp = super::httpface::layout_for(&root, &central).ledger_paths();
        argv_of(&[
            &lease_bin().display().to_string(),
            "status",
            "--ledger",
            &lp["ledger"].display().to_string(),
            "--locks",
            &lp["locks"].display().to_string(),
        ])
    };
    let out = run_readonly(&argv, Some(&root), None, std::time::Duration::from_secs(120)).await;
    if out.rc != 0 {
        return error_payload(
            "locks_read",
            what,
            &params,
            &format!("lease status 退出码 {}: {}", out.rc, truncate(&out.stderr, 400)),
        );
    }
    let raw: Value = match serde_json::from_str(&out.stdout) {
        Ok(v) => v,
        Err(e) => {
            return error_payload(
                "locks_read",
                what,
                &params,
                &format!("lease status 出参非 JSON: {e}"),
            );
        }
    };
    let held = raw.pointer("/locks/held").and_then(|h| h.as_array()).cloned().unwrap_or_default();
    let active = raw.pointer("/sessions/active").and_then(|s| s.as_array()).cloned().unwrap_or_default();
    json!({
        "held_count": held.len(),
        "held_locks": held.iter().map(|h| json!({
            "path": h.get("path"), "session_id": h.get("session_id"),
        })).collect::<Vec<_>>(),
        "active_sessions": active.len(),
        "active_session_ids": active.iter().map(|s| s.get("session_id").cloned().unwrap_or(Value::Null)).collect::<Vec<_>>(),
    })
}

// --------------------------------------------------------- nomenclator 两具

pub async fn nomenclator_query(word: Option<String>) -> Value {
    let what = "查词：术语六态查询（已立、懒波、死档、候补、未知），只读投影 nomenclator query。";
    let params = ["word: 待查词（必填）"];
    let w = word.unwrap_or_default().trim().to_string();
    if w.is_empty() {
        let mut out = error_payload("nomenclator_query", what, &params, "word 缺席");
        out["naming_teaching"] = json!(NAMING_TEACHING);
        return out;
    }
    let nom_dir = nom_project_dir();
    let argv = argv_of(&[
        "uv",
        "run",
        "--project",
        &nom_dir.display().to_string(),
        "nomenclator",
        "query",
        "--pack",
        &nom_pack().display().to_string(),
        "--word",
        &w,
    ]);
    let out = run_readonly(&argv, Some(&nom_dir), None, std::time::Duration::from_secs(60)).await;
    if out.rc != 0 {
        let mut o = error_payload(
            "nomenclator_query",
            what,
            &params,
            &format!(
                "nomenclator query 退出码 {}: {}",
                out.rc,
                truncate(if out.stderr.is_empty() { &out.stdout } else { &out.stderr }, 200)
            ),
        );
        o["naming_teaching"] = json!(NAMING_TEACHING);
        return o;
    }
    let report: Value = match serde_json::from_str(&out.stdout) {
        Ok(v) => v,
        Err(_) => {
            let mut o = error_payload("nomenclator_query", what, &params, "出参不可解析（非 JSON）");
            o["naming_teaching"] = json!(NAMING_TEACHING);
            return o;
        }
    };
    let state = report.get("state").cloned().unwrap_or(Value::Null);
    let mut result = json!({
        "word": w,
        "state": state,
        "states": report.get("states").cloned().unwrap_or(json!([])),
        "hits": report.get("hits").cloned().unwrap_or(json!([])),
    });
    if state == json!("unknown") {
        result["naming_teaching"] = json!(NAMING_TEACHING);
        result["next_action"] = json!(format!(
            "词 '{w}' 查册 unknown：命名动作前先 nomenclator map --concept 拉概念本尊语义映射报告（乙前注入，只报不判）；立名收敛前不得凭空造词入文"
        ));
    }
    result
}

pub async fn nomenclator_check(root: PathBuf, target: Option<String>) -> Value {
    let what = "文档核查：死档禁字级与懒波词两规则字符串级检出，只读投影 nomenclator check。";
    let params = ["target: 目标文档路径（工作区根相对或绝对形）"];
    let t = target.unwrap_or_default().trim().to_string();
    if t.is_empty() {
        let mut out = error_payload("nomenclator_check", what, &params, "target 缺席");
        out["naming_teaching"] = json!(NAMING_TEACHING);
        return out;
    }
    let mut tp = std::path::PathBuf::from(&t);
    if !tp.is_absolute() {
        tp = root.join(tp);
    }
    if !tp.is_file() {
        let mut out = error_payload(
            "nomenclator_check",
            what,
            &params,
            &format!("目标文档缺席: {}", tp.display()),
        );
        out["naming_teaching"] = json!(NAMING_TEACHING);
        return out;
    }
    let nom_dir = nom_project_dir();
    let argv = argv_of(&[
        "uv",
        "run",
        "--project",
        &nom_dir.display().to_string(),
        "nomenclator",
        "check",
        "--pack",
        &nom_pack().display().to_string(),
        &tp.display().to_string(),
    ]);
    let out = run_readonly(&argv, Some(&nom_dir), None, std::time::Duration::from_secs(60)).await;
    if out.rc != 0 && out.rc != 1 {
        let mut o = error_payload(
            "nomenclator_check",
            what,
            &params,
            &format!(
                "nomenclator check 退出码 {}: {}",
                out.rc,
                truncate(if out.stderr.is_empty() { &out.stdout } else { &out.stderr }, 200)
            ),
        );
        o["naming_teaching"] = json!(NAMING_TEACHING);
        return o;
    }
    let report: Value = match serde_json::from_str(&out.stdout) {
        Ok(v) => v,
        Err(_) => {
            let mut o = error_payload("nomenclator_check", what, &params, "出参不可解析（非 JSON）");
            o["naming_teaching"] = json!(NAMING_TEACHING);
            return o;
        }
    };
    let violations = report
        .get("violations")
        .cloned()
        .or_else(|| report.get("findings").cloned())
        .unwrap_or(json!([]));
    json!({
        "target": tp.display().to_string(),
        "exit_code": out.rc,
        "violations": violations,
        "summary": report.get("summary").cloned().unwrap_or(json!({})),
    })
}

// --------------------------------------------------------------- naming_guide

pub async fn naming_guide() -> Value {
    json!({
        "tool": "naming_guide",
        "title": "立名指引：司衡命名动作的程序与纪律（只读教学面，零裁决）",
        "sections": {
            "naming_five_step_form": [
                "一、乙前注入：立名动作前先拉 nomenclator map --concept 语义映射报告（概念锚查词典六态与既裁 code 形与近邻词，只报不判）入上下文，防想错了",
                "二、查册对表：命名动作查册对象是概念本尊非只造出的词形——nomenclator_query 查概念词与候选词形，并查命名集与检词（DEC-017 修订四常设纪律）",
                "三、取名填甲表：看真材料取名后申报甲表三件——概念锚 zh、既裁 code 形或显式申报无承、语素派生",
                "四、甲机械兜底：lease open --new-stem 带甲表，stem 闸机械核对指称完整与派生对表，填不圆即拒（零 LLM 判词位），防真的想错了",
                "五、立名程序终裁与登记：语义忠实终裁归立名程序人节点；收敛后 nomenclator register 在册，血统档与连带改写清单随批",
            ],
            "dec_017_pointers": [
                "DEC-017：sih-engine/doc/decision/017-wengu-naming.md",
                "修订四：工程命名动作前查命名集与检词为常设纪律（触发面即工程命名动作，不含临时描述用语）",
                "修订五：纪律执行位落机械闸面——lease open stem 查册闸加nomenclator_query 与 nomenclator_check 两具上 MCP 面；批次标签经 --new-stem 认领形为合法通道，实体正名仍走立名全程序",
                "修订六：乙前注入语义映射报告加 --new-stem 甲表认领（nomsupply 批落位）",
            ],
            "nomenclator_six_states": [
                "established 已立：terms.json 在册词条，stem 闸过，可入文",
                "lazy 懒波：临时词登记在案，事后网非事前闸，新产文档禁入，候立名消化",
                "dead/strict 死档字级：字级禁入文档，档外不复述防死词回流入文",
                "dead/name_only 死档名级：禁作名不禁史述，复用即撞闸拒",
                "candidate 候补：候选在案，重推被既有条目阻断，呈报候裁",
                "unknown 未知：未立名，命名动作须走立名程序或 --new-stem 带甲表认领",
            ],
            "stem_gate_reject_teach_claim": [
                "拒：任务包名与词典既有条目相撞即拒教学——dead 翻案须显式推翻原死因、lazy 升格走 nomenclator register、candidate 呈报既有条目阻断重推",
                "教：unknown 未立名即拒教学指立名程序；新城正典域零代强制纯教学（DES-016 只教不拒）",
                "认领：--new-stem 显式新词认领，认领须带甲表三件（--claim-zh 概念锚 zh、--claim-code 既裁 code 形或字面 无承、--claim-derivation 语素派生表 段:形），裸认领拒，指称完整与派生对表机械核，填不圆即拒",
            ],
            "dead_archive_rules": [
                "dead/strict 死档字级禁入：字级检出承 nomenclator_check 文档核查机械位",
                "dead/name_only 禁作名不禁史述：词形不得复用为名，史述引用豁免",
                "翻案须显式推翻原死因并留痕，层级复活与词场淹没死因入档",
                "血统档：死名与死因全录死档，档外不复述",
            ],
        },
        "canonical_pointers": [
            "sih-engine/doc/decision/017-wengu-naming.md",
            ".agents/skills/sihankor-naming/SKILL.md",
            "sih-engine/sih/state/parking/materials/pk-090.json",
            "sih-tools/lease/CONTRACT.md（stem 查册闸修订）",
        ],
        "disclaimer": "本具只读教学零裁决零 LLM：语义忠实终裁归立名程序人节点（pk-090 甲乙结合定案）",
    })
}

// ------------------------------------------------------------ retriever_recall

#[allow(clippy::too_many_arguments)]
pub async fn retriever_recall(
    root: PathBuf,
    topic: Option<Vec<String>>,
    word: Option<Vec<String>>,
    event: Option<Vec<String>>,
    since: Option<String>,
    until: Option<String>,
    archive: Option<String>,
    at: Option<String>,
) -> Value {
    // 根参即连接实效根（providers conn_root 传入）。
    let what = "温故检索：治理档案四轴检索 stdout NDJSON 透传，只读投影 retriever recall（只报不判）。";
    let params = [
        "topic: 主题词数组（可选）",
        "word: 文轴词数组（可选）",
        "event: 事件记号数组（可选）",
        "since: YYYY-MM-DD 时间轴下界（可选）",
        "until: YYYY-MM-DD 时间轴上界（可选）",
        "archive: 档名过滤（可选）",
        "at: YYYY-MM-DD 参照日（可选）",
        "四轴即 topic 与 word 与 event 与 time（since/until），至少给一轴",
    ];
    let clean = |v: &Option<Vec<String>>| -> Vec<String> {
        v.as_ref().map_or(vec![], |xs| {
            xs.iter().filter(|s| !s.trim().is_empty()).cloned().collect()
        })
    };
    let topics = clean(&topic);
    let words = clean(&word);
    let events = clean(&event);
    let since_s = since.unwrap_or_default();
    let until_s = until.unwrap_or_default();
    let has_time_axis = !since_s.trim().is_empty() || !until_s.trim().is_empty();
    if topics.is_empty() && words.is_empty() && events.is_empty() && !has_time_axis {
        return error_payload(
            "retriever_recall",
            what,
            &params,
            "轴全缺（镜像 CLI 轴全缺报文，退出码一拦）：四轴即 topic 与 word 与 event 与 time（since/until），至少给一轴；at 与 archive 是参照与过滤位非轴",
        );
    }
    let mut argv = vec![retriever_bin().display().to_string(), "recall".to_string()];
    for t in &topics {
        argv.push("--topic".to_string());
        argv.push(t.clone());
    }
    for w in &words {
        argv.push("--word".to_string());
        argv.push(w.clone());
    }
    for e in &events {
        argv.push("--event".to_string());
        argv.push(e.clone());
    }
    if !since_s.trim().is_empty() {
        argv.push("--since".to_string());
        argv.push(since_s.trim().to_string());
    }
    if !until_s.trim().is_empty() {
        argv.push("--until".to_string());
        argv.push(until_s.trim().to_string());
    }
    if let Some(a) = archive.as_deref().filter(|s| !s.trim().is_empty()) {
        argv.push("--archive".to_string());
        argv.push(a.trim().to_string());
    }
    if let Some(a) = at.as_deref().filter(|s| !s.trim().is_empty()) {
        argv.push("--at".to_string());
        argv.push(a.trim().to_string());
    }
    argv.push("--root".to_string());
    argv.push(root.display().to_string());
    let out: RunOutcome =
        run_readonly(&argv, None, None, std::time::Duration::from_secs(120)).await;
    json!({
        "tool": "retriever_recall",
        "root": root.display().to_string(),
        "exit_code": out.rc,
        "stdout": out.stdout,
        "stderr": out.stderr,
    })
}

// ---------------------------------------------------------------- 供测试对表

pub fn canon_pointers() -> [&'static str; 2] {
    [CANON_SPEC_023, CANON_LINE_PKG]
}
