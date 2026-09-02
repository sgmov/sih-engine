//! 截流谓词装配位——GOV-002 退出标准判据三「按轮判定的截流谓词族融回
//! 三问模块」的落位形，承 SPEC-015 截流装配位节。
//!
//! 接口宁窄勿宽只两公共函数：装配位即 load_intercept_pack 读谓词包目录
//! 返回已校验包，按轮判定即 round_interception 对材料逐谓词求值返回过败
//! 记录序列。判定语义不复实现即两函数全走 src/attractor/route.rs 谓词机，
//! 单一实现双接位。三问既有 validate 与 gate 行为零改，本装配位是接口与
//! 测试承载，按轮判定接入三问会话起点与逐轮闸位的调用归后续批承载。

use serde_json::Value;

use crate::attractor::route::{evaluate, Pack};
use crate::attractor::jsonc::PyError;

use std::path::Path;

/// 装配位：读谓词包目录装载入三问，校验失败即 PackError 信封。
///
/// 谓词包与路由谓词同形即 manifest.toml 加 routes.toml 成对纯数据，
/// 截流族按轮判定 kinds 在场即可装配，包内声明什么评什么，空腹纪律同源。
pub fn load_intercept_pack(pack_dir: &Path) -> Result<Pack, PyError> {
    crate::attractor::route::load_pack(pack_dir)
}

/// 按轮判定接口：对材料（轮记录或普通材料）逐谓词求值，返回过败记录
/// 序列（{id, pass}，按包内声明顺序），首败定路与三路归并由调用方承接
/// 路由谓词机面，本接口只产截流判定读数。
///
/// 参照时间显式给参不读系统钟，时间维度谓词缺参 fail-closed。
pub fn round_interception(material: &Value, pack: &Pack, reference_time: Option<&str>) -> Vec<Value> {
    let context = serde_json::json!({"reference_time": reference_time});
    pack.predicates
        .iter()
        .map(|p| serde_json::json!({"id": p.id, "pass": evaluate(material, p, Some(&context))}))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intercept_narrow_surface_smoke() {
        // 宁窄勿宽：公共面恰两函数，均经谓词机
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/attractor/parks-nonexistent");
        assert!(load_intercept_pack(&dir).is_err());
        let _ = round_interception(&serde_json::json!({}), &crate::attractor::route::load_pack(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/attractor/packs/core")).unwrap(), None);
    }
}
