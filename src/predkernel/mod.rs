//! 引擎侧共享求值内核（predkernel）
//!
//! 承接 DES-007#cargo-layout 与 kernelmerge-solo 批：把核阅
//! （`scrutinator/rule.rs`）与得一（`attractor/route.rs`）两侧求值辅助归并
//! 为引擎内一个共享模块。
//!
//! 抽取范围：glob 匹配——path-anchored 与 fnmatch 兼容两条语义不重叠的分支。
//! 谓词域不并：核阅 12 kinds 谓词与得一 10 kinds 谓词各自保留，重叠处以
//! 包装适配不以内核吞并（承 m-halfmerge-1 stable_clear 终签
//! 2a7d0e0e 的归并口径）。
//!
//! 公开子件：
//! - [`glob`]：glob 匹配两条分支（`match_path_glob` 与 `match_fnmatch_glob`）
//! - [`error`]：共享错误信封（`GlobError`，当前 glob 匹配 fail-closed 吞编
//!   译失败返回 bool，error 件留位供后续扩出 Result 接口使用）

pub mod error;
pub mod glob;
