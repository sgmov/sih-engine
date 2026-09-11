//! mcpserver：MCP 线 Rust 载体（rmcp SDK），sihmcp-solo 批段1。
//!
//! DEC-001 源码位：MCP server 与内部模块的承载。行为对等基准是现役 Python
//! 载体 sih-tools/mcpline（段4 parity 电池判词位）。

pub mod alpha;
pub mod runtime;
pub mod server;

#[cfg(test)]
mod tests;
