//! 共享错误信封
//!
//! 抽自核阅与得一两侧的求值通道错误承载。当前 glob 匹配两侧均 fail-closed
//! 返回 bool 吞掉编译失败（`scrutinator/rule.rs` 与 `attractor/route.rs` 原
//! 行为），未在两侧公开接口里暴露错误。
//!
//! 本件留位供后续扩出返回 `Result` 的统一求值接口时归集错误类型。当前仅
//! 承载 [`GlobError`] 一枚零字段结构，预留给可能新增的"严格模式"或
//! "诊断模式"使用。
//!
//! 边界：
//! - 核阅侧 `RuleError`（当前为 `String`）不并——外部接口冻结，跨仓 ABI 稳定。
//! - 得一侧 `PyError`（来自 `attractor::jsonc`）不并——已与 jsonc 共享。
//! - 新增错误类型须在本件登记，不在两侧重写。

/// 共享 glob 编译错误（当前未在 `match_*_glob` 中暴露，预留）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobError {
    pub message: String,
}

impl GlobError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for GlobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "glob error: {}", self.message)
    }
}

impl std::error::Error for GlobError {}
