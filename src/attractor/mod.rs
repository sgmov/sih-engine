//! attractor——得一：判定器席实例的机械核对腿（融回自 sih-tools facet/tally，
//! 承 SPEC-014 得一融回落差规格与 DEC-020 立名）。
//!
//! 腿分层（SPEC-014 腿切分清单）：采样观察腿留围堰 sih-tools/facet（组件层
//! 零 LLM 纪律的物理承载）；机械核对腿落本模块，全确定性：零网络、零 LLM、
//! 零 key 读取、零目标仓写入。融回件依赖闭包全落在融回件互依，围堰密钥面
//! 与采样面（LLM 客户端、key 探针、env 装载等留堰件）不在本模块依赖闭包内。
//!
//! 子模块（融回十一行对应物）：
//! - [`contract_mode`]：采样腿契约机械内核（合同 emit/load、响应严格对表、
//!   原文解析、dc 装配、飞轮追加、计分材料落盘、规范席位串）
//! - [`stats`]：严格统计量门面（metrics 距离四函数、inf 推断校正五函数、
//!   conv 收敛二函数）
//! - [`compiler`]：确定性聚合器（指标计算、发散发谱、决策收敛装配）
//! - [`validators`]：PRO-001 内置检验器函数库
//! - [`anchors`]：锚定三步验证
//! - [`model_utils`]：模型家族与认知框架提取
//! - [`paradigm_loader`]：范式 yaml 装载与编排校验
//! - [`tally`]：执契机械核对五子命令逻辑（check/verify/assemble/watch/sign）
//!
//! 判定规约形态（des-011 随迁）：R1 至 R7 规则与三态映射为代码承载的核对
//! 逻辑，`rules_version` 字段随材料声明代际（缺省 des-011-r1），规则增改走
//! 版本管理即 rules_version 形变更为唯一入口，承 SPEC-014 机械腿零规则内嵌
//! 纪律。

pub mod anchors;
pub mod compiler;
pub mod contract_mode;
pub mod jsonc;
pub mod model_utils;
pub mod paradigm_loader;
pub mod route;
pub mod stats;
pub mod tally;
pub mod validators;
