//! 判定路由组件库（askroute），gap-askroute-port 引擎位移植件。
//!
//! 正典指针：围堰契约 sih-tools/askroute/CONTRACT.md（intents-v0 包 schema）、
//! sih-tools/askroute/tests/test_pack.py（drift 守卫三类硬事实）、DEC-006 三问载体、
//! SPEC-025 腿六长尾收口（askroute 67 行即 test_pack.py）。围堰为纯数据包形，
//! 无可执行件；本组件把「缩写→意图→动作链」判定做成引擎位库加 bin，
//! 供 MCP 强制触发协议（AGENTS.md § sih 强制触发协议）引擎侧承载：
//! 模型侧只产缩写，缩写到意图的映射与意图到 MCP 治理动作链的拉起全部由包数据
//! 承载，不经模型生成；表外缩写唯一出口是 unknown_action 三步兜底
//! （查册 → 语料召回 → 问人），禁即兴解释后执行。
//!
//! 两级判定面，对齐 test_pack.py 的 load 层与守卫层：
//! - 载入层 `parse_pack`：包结构 schema（pack=="intents"、version 以 v 开头、
//!   unknown_action 在场、chain 非空逐命令字符串）。载入失败即工具异常，
//!   对应 CLI 退出码 2。
//! - 守卫层 `check_pack`：drift 守卫（意图 id 唯一、缩写不跨意图冲突、
//!   unknown_action 步骤序固定、惯例缩写在册、write-chain 剧本对表）。
//!   有违例即红，对应 CLI 退出码 1。
//!
//! 引擎位判定包编译期内嵌（include_str!），与围堰包逐字节随迁（随迁对表由
//! tests/mergeall_t6_askroute.rs T0 承载）；`--pack` 可指围堰包路径做同参双跑。

use serde_json::Value;

/// 编译期内嵌引擎位判定包：src/askroute/packs/intents-v0.json，
/// 逐字节随迁自围堰 sih-tools/askroute/packs/intents-v0.json。
pub const DEFAULT_PACK: &str = include_str!("packs/intents-v0.json");

/// 包类型恒量（CONTRACT：`pack` 恒 "intents"）。
pub const PACK_KIND: &str = "intents";

/// 惯例缩写在册守卫表（test_pack.py test_convention_abbreviations_registered）。
pub const REQUIRED_ABBREVIATIONS: [&str; 6] = ["过得一裁", "得一裁", "温故", "体检", "泊一下", "排队"];

/// 单意图：id 全包唯一 kebab-case，缩写全包不跨意图重复，链非空逐命令。
#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    pub id: String,
    pub abbreviations: Vec<String>,
    pub chain: Vec<String>,
    pub manual: String,
}

/// 表外缩写唯一出口：查册 → 语料召回 → 问人，步骤顺序即纪律。
#[derive(Debug, Clone, PartialEq)]
pub struct UnknownAction {
    pub id: String,
    pub steps: Vec<String>,
}

/// intents-v0 判定包。
#[derive(Debug, Clone, PartialEq)]
pub struct Pack {
    pub version: String,
    pub updated: String,
    pub canonical_pointers: Vec<String>,
    pub unknown: UnknownAction,
    pub intents: Vec<Intent>,
}

/// 路由判定结果：命中在册意图，或表外缩写落 unknown_action 出口。
#[derive(Debug, Clone, PartialEq)]
pub enum RouteOutcome {
    Hit {
        intent: String,
        chain: Vec<String>,
        manual: String,
    },
    Unknown(UnknownAction),
}

/// 取字符串数组字段，缺键或元素非字符串即载入失败。
fn string_array(obj: &serde_json::Map<String, Value>, key: &str) -> Result<Vec<String>, String> {
    match obj.get(key) {
        Some(Value::Array(items)) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                match item.as_str() {
                    Some(s) => out.push(s.to_string()),
                    None => return Err(format!("{key} 数组元素须为字符串")),
                }
            }
            Ok(out)
        }
        _ => Err(format!("包缺 {key} 键或非字符串数组")),
    }
}

/// 载入层：解析并校验 intents-v0 包结构 schema（退出码 2 形）。
pub fn parse_pack(text: &str) -> Result<Pack, String> {
    let value: Value = serde_json::from_str(text).map_err(|err| format!("包 JSON 不可解析 {err}"))?;
    let obj = value.as_object().ok_or("包顶层须为 JSON 对象")?;
    let kind = obj
        .get("pack")
        .and_then(Value::as_str)
        .ok_or("包缺 pack 键或非字符串")?;
    if kind != PACK_KIND {
        return Err(format!("包类型须为 {PACK_KIND}，得 {kind}"));
    }
    let version = obj
        .get("version")
        .and_then(Value::as_str)
        .ok_or("包缺 version 键或非字符串")?;
    if !version.starts_with('v') {
        return Err(format!("version 须以 v 开头，得 {version}"));
    }
    let updated = obj
        .get("updated")
        .and_then(Value::as_str)
        .ok_or("包缺 updated 键或非字符串")?
        .to_string();
    let canonical_pointers = string_array(obj, "canonical_pointers")?;
    let unknown_obj = obj
        .get("unknown_action")
        .and_then(Value::as_object)
        .ok_or("包缺 unknown_action 键或非对象（本键不可删除）")?;
    let unknown_id = unknown_obj
        .get("id")
        .and_then(Value::as_str)
        .ok_or("unknown_action 缺 id 键或非字符串")?
        .to_string();
    let steps = string_array(unknown_obj, "steps")?;
    if steps.is_empty() {
        return Err("unknown_action.steps 须为非空动作序列".to_string());
    }
    let intents_value = obj.get("intents").and_then(Value::as_array).ok_or("包缺 intents 键或非数组")?;
    let mut intents = Vec::with_capacity(intents_value.len());
    for intent_value in intents_value {
        let intent_obj = intent_value
            .as_object()
            .ok_or("intents[] 元素须为对象")?;
        let id = intent_obj
            .get("id")
            .and_then(Value::as_str)
            .ok_or("意图缺 id 键或非字符串")?
            .to_string();
        let abbreviations = string_array(intent_obj, "abbreviations")?;
        let chain = string_array(intent_obj, "chain")?;
        if chain.is_empty() {
            return Err(format!("意图 {id} 动作链非空（CONTRACT：chain 非空）"));
        }
        let manual = intent_obj
            .get("manual")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("意图 {id} 缺 manual 键或非字符串"))?
            .to_string();
        intents.push(Intent { id, abbreviations, chain, manual });
    }
    Ok(Pack {
        version: version.to_string(),
        updated,
        canonical_pointers,
        unknown: UnknownAction { id: unknown_id, steps },
        intents,
    })
}

/// 路由判定：缩写逐字精确匹配在册表，按文件序首中即返回；全表未中落
/// unknown_action 出口。包经守卫层放行后不存在跨意图冲突，首中即确定。
pub fn route(pack: &Pack, abbreviation: &str) -> RouteOutcome {
    for intent in &pack.intents {
        if intent.abbreviations.iter().any(|abbr| abbr == abbreviation) {
            return RouteOutcome::Hit {
                intent: intent.id.clone(),
                chain: intent.chain.clone(),
                manual: intent.manual.clone(),
            };
        }
    }
    RouteOutcome::Unknown(pack.unknown.clone())
}

/// 守卫层：drift 守卫违例清单（test_pack.py 守卫面移植），空即全绿。
pub fn check_pack(pack: &Pack) -> Vec<String> {
    let mut findings = Vec::new();

    // 意图 id 全包唯一（test_intent_ids_unique）
    let mut seen_ids: Vec<&str> = Vec::new();
    for intent in &pack.intents {
        if seen_ids.contains(&intent.id.as_str()) {
            findings.push(format!("意图 id 重复：{}", intent.id));
        }
        seen_ids.push(&intent.id);
    }

    // 缩写不跨意图冲突（test_abbreviations_no_cross_conflict）
    let mut seen_abbrs: Vec<(&str, &str)> = Vec::new();
    for intent in &pack.intents {
        for abbr in &intent.abbreviations {
            if let Some((_, prior)) = seen_abbrs.iter().find(|(seen, _)| *seen == abbr.as_str()) {
                findings.push(format!("缩写跨意图冲突：{abbr} 在 {prior} 与 {}", intent.id));
            } else {
                seen_abbrs.push((abbr, &intent.id));
            }
        }
    }

    // unknown_action 步骤序固定：查册 → 语料召回 → 问人（本键步骤序即裁判面）
    let steps = &pack.unknown.steps;
    if steps.len() != 3 {
        findings.push(format!("unknown_action.steps 须三步，得 {}", steps.len()));
    } else {
        if !steps[0].starts_with("nomenclator_query") {
            findings.push(format!("unknown_action 第一步须查册（nomenclator_query），得 {}", steps[0]));
        }
        if !steps[1].starts_with("retriever_recall") {
            findings.push(format!("unknown_action 第二步须语料召回（retriever_recall），得 {}", steps[1]));
        }
        if !steps[2].starts_with("ask_human") {
            findings.push(format!("unknown_action 第三步须问人（ask_human），得 {}", steps[2]));
        }
    }

    // 惯例缩写在册，不得静默消失（test_convention_abbreviations_registered）
    let all_abbrs: Vec<&str> = pack.intents.iter().flat_map(|i| i.abbreviations.iter().map(String::as_str)).collect();
    for required in REQUIRED_ABBREVIATIONS {
        if !all_abbrs.contains(&required) {
            findings.push(format!("惯例缩写缺失：{required}"));
        }
    }

    // write-chain 剧本对表：写入链七步逐名在案（test_chains_match_manual_scripts）
    match pack.intents.iter().find(|i| i.id == "write-chain") {
        None => findings.push("write-chain 意图缺席".to_string()),
        Some(intent) => {
            if intent.chain.len() < 4 {
                findings.push(format!("write-chain 链长不足七步形，得 {}", intent.chain.len()));
            } else {
                if intent.chain[0] != "lease_open" || intent.chain[1] != "lease_lock" {
                    findings.push("write-chain 前两步须 lease_open 与 lease_lock".to_string());
                }
                let last = intent.chain.len();
                if intent.chain[last - 2] != "lease_unlock" || intent.chain[last - 1] != "lease_close" {
                    findings.push("write-chain 末两步须 lease_unlock 与 lease_close".to_string());
                }
                if !intent.chain.iter().any(|step| step == "record_intent") {
                    findings.push("write-chain 缺 record_intent".to_string());
                }
                if !intent.chain.iter().any(|step| step == "record_append") {
                    findings.push("write-chain 缺 record_append".to_string());
                }
            }
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn embedded_pack() -> Pack {
        parse_pack(DEFAULT_PACK).expect("内嵌包须可载入")
    }

    #[test]
    fn embedded_pack_loads_and_guard_green() {
        let pack = embedded_pack();
        assert_eq!(pack.version, "v0");
        assert_eq!(pack.updated, "2026-09-11");
        assert_eq!(pack.canonical_pointers.len(), 3);
        assert_eq!(pack.intents.len(), 7);
        assert_eq!(pack.unknown.id, "askroute-unknown");
        assert!(check_pack(&pack).is_empty(), "内嵌包守卫须全绿");
    }

    #[test]
    fn route_hits_golden_abbreviations() {
        let pack = embedded_pack();
        let hit = route(&pack, "体检");
        assert_eq!(
            hit,
            RouteOutcome::Hit {
                intent: "coldstart-check".to_string(),
                chain: vec![
                    "locks_read".to_string(),
                    "critsweep".to_string(),
                    "heartbeat".to_string(),
                    "chain_verify 今日".to_string(),
                ],
                manual: "AI-MANUAL §4 剧本 A".to_string(),
            }
        );
        assert!(matches!(route(&pack, "温故"), RouteOutcome::Hit { intent, .. } if intent == "recall"));
        if let RouteOutcome::Hit { intent, chain, .. } = route(&pack, "过得一裁") {
            assert_eq!(intent, "deyi-adjudicate");
            assert_eq!(chain.len(), 7, "得一裁链须七步");
        } else {
            panic!("过得一裁须命中");
        }
        assert!(matches!(route(&pack, "开租约"), RouteOutcome::Hit { intent, .. } if intent == "write-chain"));
    }

    #[test]
    fn unknown_falls_to_three_step_exit() {
        let pack = embedded_pack();
        let outcome = route(&pack, "表外缩写");
        match outcome {
            RouteOutcome::Unknown(unknown) => {
                assert_eq!(unknown.id, "askroute-unknown");
                assert_eq!(unknown.steps.len(), 3);
                assert!(unknown.steps[0].starts_with("nomenclator_query"));
                assert!(unknown.steps[1].starts_with("retriever_recall"));
                assert!(unknown.steps[2].starts_with("ask_human"));
            }
            other => panic!("表外缩写须落 unknown_action，得 {other:?}"),
        }
    }

    #[test]
    fn guard_flags_cross_conflict_and_missing_conventions() {
        let text = r#"{
            "pack": "intents", "version": "v0", "updated": "2026-09-11",
            "canonical_pointers": [],
            "unknown_action": {"id": "askroute-unknown", "steps": [
                "nomenclator_query <词>", "retriever_recall word=<词>", "ask_human：求裁"]},
            "intents": [
                {"id": "a", "abbreviations": ["同词"], "chain": ["x"], "manual": "m"},
                {"id": "b", "abbreviations": ["同词"], "chain": ["y"], "manual": "m"}
            ]
        }"#;
        let pack = parse_pack(text).expect("结构合法须可载入");
        let findings = check_pack(&pack);
        assert!(findings.iter().any(|f| f.contains("缩写跨意图冲突")), "findings={findings:?}");
        assert!(findings.iter().any(|f| f.contains("惯例缩写缺失")), "findings={findings:?}");
        assert!(findings.iter().any(|f| f.contains("write-chain 意图缺席")), "findings={findings:?}");
    }

    #[test]
    fn parse_rejects_schema_breaks() {
        assert!(parse_pack("not json").is_err());
        assert!(parse_pack(r#"{"pack": "other"}"#).is_err());
        // chain 空即载入失败（CONTRACT：chain 非空）
        let empty_chain = r#"{
            "pack": "intents", "version": "v0", "updated": "d", "canonical_pointers": [],
            "unknown_action": {"id": "u", "steps": ["s"]},
            "intents": [{"id": "a", "abbreviations": ["x"], "chain": [], "manual": "m"}]
        }"#;
        let err = parse_pack(empty_chain).unwrap_err();
        assert!(err.contains("非空"), "err={err}");
        // unknown_action 缺席即载入失败（CONTRACT：本键不可删除）
        let no_unknown = r#"{
            "pack": "intents", "version": "v0", "updated": "d", "canonical_pointers": [],
            "intents": []
        }"#;
        assert!(parse_pack(no_unknown).unwrap_err().contains("unknown_action"));
    }
}
