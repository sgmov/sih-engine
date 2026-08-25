//! 三问记录类型，承接 SPEC-005#data-contract。
//!
//! 输入记录两必须字段加一可选字段，输出记录八字段，
//! 意图契约四子字段，认知域双域，锚点九字段含血统三件套。

use serde::{Deserialize, Serialize};

/// 输入记录，经说话口到达的意图载体，SPEC-005#input-record。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputRecord {
    /// 会话标识，与事件流会话语境同源。
    pub session_id: String,
    /// 原始输入，逐字快照，不作预处理。
    pub raw_input: String,
    /// 会话语境，承载指针不复制正文。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_context: Option<SessionContext>,
}

/// 会话语境，SPEC-005#input-record 可选字段。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionContext {
    /// 指向的治理文档标识。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<String>,
    /// 本次已加载的约束出处清单。
    #[serde(default)]
    pub rule_sources: Vec<String>,
}

/// 输出记录，八字段，SPEC-005#output-record。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputRecord {
    pub session_id: String,
    pub raw_input: String,
    /// 轮次标记，会话内序号从 1 起。
    pub round: u32,
    pub intent_contract: IntentContract,
    pub domain_contract: DomainContract,
    /// 推理锚点数组，长度下限一。
    pub anchors: Vec<Anchor>,
    pub calls_in: i64,
    pub calls_out: i64,
}

/// 意图契约，收窄后的精确生成契约，四子字段。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntentContract {
    /// 正向目标，一句话。
    pub goal: String,
    /// 显式排除项，无排除时空数组不得缺省。
    #[serde(default)]
    pub exclusions: Vec<String>,
    /// 任务格式，产出形态约束。
    pub output_format: String,
    /// 约束注入，每项一条约束的出处。
    #[serde(default)]
    pub injected_constraints: Vec<String>,
}

/// 单域，范围陈述加深上限。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Domain {
    pub scope: String,
    pub max_depth: u32,
}

/// 认知域契约，目标域与支撑域二层，支撑域深度不得超过目标域。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainContract {
    pub target_domain: Domain,
    pub support_domain: Domain,
}

/// 诘察阶段，三问内的更深递归仍记 third。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InquiryStage {
    First,
    Second,
    Third,
}

/// 域标签，与认知域契约对应。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DomainTag {
    Target,
    Support,
}

/// 哲学引文，血统第一件，摘录须与原文逐字一致。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhilosophyRef {
    /// 哲学仓出处，文件路径加节名或命题编号。
    pub source: String,
    /// 原文摘录，逐字。
    pub quote: String,
}

/// 推理锚点，九字段，每推演节点必产，禁熔断。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Anchor {
    pub anchor_seq: u32,
    /// 命题文本，一句话。
    pub text: String,
    pub inquiry_stage: InquiryStage,
    pub domain_tag: DomainTag,
    /// 显层命题记 0，每递归细化一层增一。
    pub depth: u32,
    pub philosophy_ref: PhilosophyRef,
    /// 理据，血统第二件。
    pub rationale: String,
    /// 证据行号，血统第三件，原始输入行号或仓内路径加行号。
    pub evidence: String,
    /// 置信度，禁熔断载体，低置信如实取低值。
    pub confidence: f64,
}
