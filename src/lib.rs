//! sih-engine 库入口，承接 DES-007#cargo-layout
//!
//! 库导出事件流公共接口。
//!
//! 事件流是 sih-engine 第一阶段的核心模块，提供追加写入、哈希链校验、事件检索
//! 三个入口，按确定性程序规则运行。

pub mod ask3repeater;
pub mod attractor;
pub mod event_stream;
pub mod mcpserver;
pub mod retriever;
pub mod snapline;
pub mod scrutinator;
pub mod tools_registry;
pub mod view;

// Re-export the public API
pub use ask3repeater::{
    intent_event_input, validate, Anchor, Ask3Error, Domain, DomainContract, DomainTag,
    InputRecord, InquiryStage, IntentContract, INTENT_EVENT_CLASS, INTENT_EVENT_TYPE,
    OutputRecord, PhilosophyRef, SessionContext,
};
pub use attractor::{compiler, contract_mode, jsonc, model_utils, paradigm_loader, route, stats, tally, validators, anchors as attractor_anchors};
pub use event_stream::{
    append, compute_event_hash, load_events, query, verify, AggregationResult, AppendError,
    AppendSuccess, Actor, ActorType, Event, EventAggregate, EventFilter, EventInput, EventList,
    GENESIS_PREV_HASH, VerifyError, VerifyRange, VerifySuccess,
};
