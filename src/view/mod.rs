//! 视图组件纯函数簇，承接 DEC-007#decision-component 聚合输出组件边界。
//!
//! 视图零写零 LLM，承接 SPEC-004#event-classification 仅可消费类进人类视图、
//! 承接 PARKING-v1#heartbeat 心跳与结算必经栏。
//! 同参双跑逐字节一致是构成性纪律。
//!
//! 三子模块即 alarms 异常视图、heartbeat 心跳视图、settle 结算视图。
//! 缺省缺席字段如实标 unknown，不虚构趋势与数值承秤星纪律。

pub mod alarms;
pub mod heartbeat;
pub mod settle;
