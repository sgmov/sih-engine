//! 三问组件本体，承接 DEC-006 七项决策、DES-013 组件设计、SPEC-005 接口规格。
//!
//! 三问即 ask3repeater，命名承 DEC-006 五件套即中文正式名三问、
//! 代码标识符小写 ask3repeater、内部代号 ASK3。
//! 诘察认知本体是 LLM 使用框架不在本模块，本模块承载确定性面：
//! 记录类型、错误四类验收、意图事件构建。写入下游即 event_stream，
//! 写入权归书简承 DEC-006 责权拆分，本模块不直接落盘。

pub mod gate;
pub mod intercept;
pub mod record;
pub mod validate;

pub use gate::{intent_event_input, INTENT_EVENT_CLASS, INTENT_EVENT_TYPE};
pub use intercept::{load_intercept_pack, round_interception};
pub use record::{
    Anchor, Domain, DomainContract, DomainTag, InputRecord, InquiryStage, IntentContract,
    OutputRecord, PhilosophyRef, SessionContext,
};
pub use validate::{validate, Ask3Error};
