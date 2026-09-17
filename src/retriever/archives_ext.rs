//! 温故档案面扩容索引件 —— retrieverline 批（rl-02）SETSP 名录与旧仓 sihankor/doc
//! 索引入检索档案面。
//! 正典指针：retrieverline 批立项包（sih-engine/sih/state/plan/retrieverline.md）。
//!
//! 机制承 locator_bridge 现形（运行期临时索引：locator build --pack --root --out，
//! 包件内嵌零工作区落盘同 canonical 内嵌记忆包形）：首域 memory 主索引零动，扩容
//! 面另建平行索引合并消费。检索可见面非引用权威面：SETSP 与旧仓只作盘点源不作引
//! 用源（工程基线与资产回锚登记先例），来源域标注经 archives::source_domain_of 随
//! 件出行零伪装正典。canonical 城形无此两域即零扩容（调用位 layout 判定承载）。
//! 语料面：两域 *.md 载体；SETSP yaml 五件为关系边集（指向目标不在本域）在册未入
//! 索引，.DS_Store 非载体不入。

use std::path::{Path, PathBuf};

use super::RecallError;

/// 扩容面记忆包：两域 include 面相对工作区根（first_domain 形），包名随批登记。
pub const EXT_MEMORY_PACK: &str = r#"{
  "name": "memory-archives-ext",
  "version": "0.1.0",
  "include": [
    "sihankor/doc/**/*.md",
    ".tmp/SihEngineeringTechnologySelectionPrecedent/**/*.md"
  ],
  "exclude": [],
  "stale_threshold": 0,
  "max_file_bytes": 2000000
}"#;

/// 临时件序号：进程号加调用序号双料，并行测试线程各建各索零互踩。
fn temp_seq() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    SEQ.fetch_add(1, Ordering::Relaxed)
}

/// 通用扩容索引构建：locator_root 供 locator 码根定位（sih-tools/locator），
/// corpus_root 为语料根（include 相对其解析），pack_json 内嵌包件临时落盘。
/// 缺席或失败映射 MissingBase locator 与主索引同报文（F-8 退化形一致）。
pub fn build_index_with_pack(
    locator_root: &Path,
    corpus_root: &Path,
    pack_json: &str,
    tag: &str,
) -> Result<PathBuf, RecallError> {
    let dir = locator_root.join("sih-tools").join("locator");
    if !dir.is_dir() {
        return Err(RecallError::MissingBase("locator".to_string()));
    }
    let seq = temp_seq();
    let pack = std::env::temp_dir().join(format!(
        "retriever-ext-pack-{tag}-{}-{seq}.json",
        std::process::id()
    ));
    std::fs::write(&pack, pack_json)
        .map_err(|_| RecallError::MissingBase("locator".to_string()))?;
    let out = std::env::temp_dir().join(format!(
        "retriever-ext-index-{tag}-{}-{seq}.ndjson",
        std::process::id()
    ));
    let output = std::process::Command::new("uv")
        .arg("run")
        .arg("--project")
        .arg(&dir)
        .arg("locator")
        .arg("build")
        .arg("--pack")
        .arg(&pack)
        .arg("--root")
        .arg(corpus_root)
        .arg("--out")
        .arg(&out)
        .output()
        .map_err(|_| RecallError::MissingBase("locator".to_string()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(brace) = stdout.find('{') else {
        return Err(RecallError::Internal(format!(
            "locator 扩容建索引输出不可解析 status={}",
            output.status.code().unwrap_or(-1)
        )));
    };
    let report: serde_json::Value = serde_json::from_str(&stdout[brace..])
        .map_err(|e| RecallError::Internal(format!("locator 扩容建索引输出非法 {e}")))?;
    if report.get("files").and_then(|f| as_u64(f)).is_none() {
        return Err(RecallError::Internal(
            "locator 扩容建索引报告无文件计数".to_string(),
        ));
    }
    Ok(out)
}

fn as_u64(v: &serde_json::Value) -> Option<u64> {
    v.as_u64()
}

/// 扩容面索引构建：first_domain 形语料根即工作区根，包用内嵌 EXT_MEMORY_PACK。
pub fn build_ext_index(root: &Path) -> Result<PathBuf, RecallError> {
    build_index_with_pack(root, root, EXT_MEMORY_PACK, "retrline")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 扩容包内嵌件可解析且两域 include 在面，零伪装：包名随批登记。
    #[test]
    fn ext_pack_embedded_shape() {
        let pack: serde_json::Value = serde_json::from_str(EXT_MEMORY_PACK).expect("包件合法");
        assert_eq!(pack["name"], "memory-archives-ext");
        let includes = pack["include"].as_array().expect("include 数组");
        let inc: Vec<&str> = includes.iter().filter_map(|v| v.as_str()).collect();
        assert!(inc.contains(&"sihankor/doc/**/*.md"), "旧仓 doc 面缺席");
        assert!(
            inc.contains(&".tmp/SihEngineeringTechnologySelectionPrecedent/**/*.md"),
            "SETSP 名录面缺席"
        );
    }

    /// 缺席退化：locator 码根缺席报 MissingBase locator，与主索引同报文。
    #[test]
    fn missing_locator_reports_missing_base() {
        let tmp = tempfile::tempdir().unwrap();
        let err = build_index_with_pack(tmp.path(), tmp.path(), EXT_MEMORY_PACK, "retrtest");
        assert!(matches!(err, Err(RecallError::MissingBase(name)) if name.contains("locator")));
    }
}
