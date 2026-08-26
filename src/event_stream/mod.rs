//! 事件流模块入口，承接 DES-007#module-organization 与 SPEC-004#interface-signature
//!
//! 五子模块按职责分离：
//! - event：事件数据结构，含 Actor 子结构
//! - hash：哈希链计算，归属确定性程序
//! - append：追加写入入口，含四态校验
//! - verify：哈希链校验入口，含区间校验
//! - query：事件检索入口，含过滤与聚合

pub mod append;
pub mod certify;
pub mod intent;
pub mod park;
pub mod event;
pub mod hash;
pub mod query;
pub mod verify;

#[cfg(test)]
mod tdd_tests;

// Re-export key types
pub use append::{append, load_events, AppendError, AppendSuccess};
pub use certify::{certification_event, CertifyError};
pub use event::{Actor, ActorType, Event, EventInput};
pub use intent::{intent_event, IntentError};
pub use park::{park_event, ParkError};
pub use hash::{compute_event_hash, verify_chain, GENESIS_PREV_HASH};
pub use query::{query, EventAggregate, EventFilter, EventList, AggregationResult};
pub use verify::{verify, VerifyError, VerifyRange, VerifySuccess};
