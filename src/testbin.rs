//! 测试专用二进制定位助手（testhard 批件二）。
//!
//! 病灶申报出处：testhard 批任务包件二（2026-09-18），修前红证为 worktree
//! 工地形（`CARGO_TARGET_DIR` 指共享主 target）跑 `cargo test --lib`，
//! `CARGO_MANIFEST_DIR/target/debug/<bin>` 旧寻址在 manifest 下无 target 目录
//! 即必红——红名单 30 件留档：src/scrutinator/tests.rs CLI 级 21 件、
//! src/event_stream/tdd_tests.rs 8 件（t2/t6/t7/t11/ga1/ga2/ga3/ga4）、
//! src/event_stream/chainstamp_tests.rs 1 件（confirm_three_states）。
//!
//! 机制注记：cargo 标准注入 `CARGO_BIN_EXE_<name>` 只覆盖 tests/ 集成测试与
//! benchmark 编译目标，lib 单元测试（src/ 内 #[cfg(test)]）不在注入面——
//! 编译期 `env!` 形在 lib 测试直接编译失败。故本助手运行期两级解析：
//! 一、`env::var("CARGO_BIN_EXE_<name>")` 在位即直用（迁集成测试零改动）；
//! 二、`current_exe` 同胞解析——lib 测试二进制居于 `<target>/<profile>/deps/`，
//! 上跳两级即 bins 同位目录，与 `CARGO_TARGET_DIR` 及 worktree 形无关，语义
//! 与 CARGO_BIN_EXE 等价（定位 cargo 实际产出二进制位）。

use std::path::PathBuf;

/// 定位本 crate 同 tree 二进制（profile 随测试载体 debug/release 同步）。
pub(crate) fn bin(name: &str) -> PathBuf {
    if let Ok(p) = std::env::var(format!("CARGO_BIN_EXE_{name}")) {
        return PathBuf::from(p);
    }
    let exe = std::env::current_exe().expect("测试进程 exe 路径可取");
    let dir = exe
        .parent()
        .and_then(|deps| deps.parent())
        .expect("测试 exe 应居于 <target>/<profile>/deps/ 两级目录下");
    let p = dir.join(name);
    assert!(
        p.is_file(),
        "{name} 二进制未找到（解析位 {}）。请先 cargo build --bins 再跑测试",
        p.display()
    );
    p
}

#[cfg(test)]
mod tests {
    use super::bin;

    /// testhard 件二自证：scribe 与 scrutinator 两 bin 经两级解析在共享
    /// CARGO_TARGET_DIR 工地形与主树裸跑形俱可定位为在位文件。
    #[test]
    fn resolves_sibling_bins_in_shared_target() {
        for name in ["scribe", "scrutinator"] {
            let p = bin(name);
            assert!(p.is_file(), "{name} 应解析为在位文件：{}", p.display());
        }
    }
}
