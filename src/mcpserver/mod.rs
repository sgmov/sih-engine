//! mcpserver：MCP 线 Rust 载体（rmcp SDK），sihmcp-solo 批。
//!
//! DEC-001 源码位：MCP server 与内部模块的承载。行为对等基准是现役 Python
//! 载体 sih-tools/mcpline（段4 parity 电池判词位）。段2 起 beta 写面六件
//! （matrix 加 errors 加 passthrough 加 session 加 tools）照 DES-014 对等移植。

pub mod alpha;
pub mod errors;
pub mod matrix;
pub mod passthrough;
pub mod runtime;
pub mod server;
pub mod session;
pub mod tokens;
pub mod tools;

#[cfg(test)]
mod tests;
