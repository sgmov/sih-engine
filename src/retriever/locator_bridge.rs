//! 寻址底座只读桥即子进程调用 build 与 query 两步，承接 SPEC-008 组件边界节。
//! wengumcp-parallel 批增两根分离：canonical 城的检索数据取本域 root，locator
//! 码根自最近祖先供给（城零 sih-tools 也可检），记忆包用内嵌 canonical 表零城侧
//! 配置；first_domain 形路径与包逐字节零变。

use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::{layout_form, Layout, RecallError};

/// canonical 新城正典域内嵌记忆包：城档映射表同源 include 集（archives.rs
/// classify_path_canonical 对表），建至系统临时文件传 locator，零工作区落盘。
pub const CANONICAL_MEMORY_PACK: &str = r#"{
  "name": "memory-canonical",
  "version": "0.1.0",
  "include": [
    "sih/event/plan/*-results.md",
    "sih/state/plan/*.md",
    "sih/state/parking/materials/*.json",
    "sih/state/parking/PARKING-v1.md"
  ],
  "exclude": [],
  "stale_threshold": 0,
  "max_file_bytes": 2000000
}"#;

/// 寻址条目十字段的消费子集即 id 与路径与载体与行位与文本。
#[derive(Clone, Debug, Deserialize)]
pub struct LocatorEntry {
    pub id: String,
    pub path: String,
    pub carrier: String,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub text: String,
}

fn locator_dir(root: &Path) -> PathBuf {
    if layout_form(root) == Some(Layout::Canonical) {
        // 码根祖先回溯：canonical 城零 sih-tools，locator 自最近祖先供给；
        // 缺祖回落现形路径由缺席报文如实显形（MissingBase locator）。
        let mut cur = root.parent();
        while let Some(dir) = cur {
            let cand = dir.join("sih-tools").join("locator");
            if cand.is_dir() {
                return cand;
            }
            cur = dir.parent();
        }
    }
    root.join("sih-tools").join("locator")
}

fn run_locator(root: &Path, args: &[String]) -> Result<serde_json::Value, RecallError> {
    let dir = locator_dir(root);
    if !dir.is_dir() {
        return Err(RecallError::MissingBase("locator".to_string()));
    }
    let output = std::process::Command::new("uv")
        .arg("run")
        .arg("--project")
        .arg(&dir)
        .arg("locator")
        .args(args)
        .output()
        .map_err(|_| RecallError::MissingBase("locator".to_string()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(brace) = stdout.find('{') else {
        return Err(RecallError::Internal(format!(
            "locator 输出不可解析 status={}",
            output.status.code().unwrap_or(-1)
        )));
    };
    serde_json::from_str(&stdout[brace..])
        .map_err(|e| RecallError::Internal(format!("locator 输出非法 {e}")))
}

fn pack_path(root: &Path) -> PathBuf {
    locator_dir(root)
        .join("packs")
        .join("memory")
        .join("pack.json")
}

fn index_path() -> PathBuf {
    std::env::temp_dir().join(format!("retriever-index-{}.ndjson", std::process::id()))
}

/// 建索引于系统临时目录不落工作区，缺席或失败映射 MissingBase 即报缺席件名 locator。
/// canonical 形用内嵌记忆包（临时文件承载），first_domain 形用仓内既有包零变。
pub fn build_index(root: &Path) -> Result<PathBuf, RecallError> {
    let pack: PathBuf = if layout_form(root) == Some(Layout::Canonical) {
        let p = std::env::temp_dir().join(format!(
            "retriever-canonical-pack-{}.json",
            std::process::id()
        ));
        std::fs::write(&p, CANONICAL_MEMORY_PACK)
            .map_err(|_| RecallError::MissingBase("locator".to_string()))?;
        p
    } else {
        let p = pack_path(root);
        if !p.is_file() {
            return Err(RecallError::MissingBase(
                "locator packs/memory/pack.json".to_string(),
            ));
        }
        p
    };
    let out = index_path();
    let report = run_locator(
        root,
        &[
            "build".to_string(),
            "--pack".to_string(),
            pack.to_string_lossy().into_owned(),
            "--root".to_string(),
            root.to_string_lossy().into_owned(),
            "--out".to_string(),
            out.to_string_lossy().into_owned(),
        ],
    )?;
    if report.get("files").and_then(|f| f.as_u64()).is_none() {
        return Err(RecallError::Internal("locator 建索引报告无文件计数".to_string()));
    }
    Ok(out)
}

fn entries_of(report: serde_json::Value) -> Vec<LocatorEntry> {
    report
        .get("entries")
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default()
}

/// 加载索引内全量 entry 供文轴逐字子串扫，承接 SPEC-008 修订六。
pub fn load_entries(index: &Path) -> Result<Vec<LocatorEntry>, RecallError> {
    let content = std::fs::read_to_string(index)
        .map_err(|_| RecallError::TargetUnreadable(index.to_string_lossy().into_owned()))?;
    let mut out = Vec::new();
    for line in content.split_terminator('\n') {
        if line.is_empty() {
            continue;
        }
        let v: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if v.get("type").and_then(|t| t.as_str()) == Some("entry") {
            if let Ok(entry) = serde_json::from_value::<LocatorEntry>(v) {
                out.push(entry);
            }
        }
    }
    Ok(out)
}

/// 词查询即按词边界匹配名与文本取条目级命中，不取 occurrences。
pub fn query_word_entries(
    root: &Path,
    index: &Path,
    word: &str,
) -> Result<Vec<LocatorEntry>, RecallError> {
    let report = run_locator(
        root,
        &[
            "query".to_string(),
            "--index".to_string(),
            index.to_string_lossy().into_owned(),
            "--ref".to_string(),
            word.to_string(),
            "--pack".to_string(),
            pack_path(root).to_string_lossy().into_owned(),
            "--root".to_string(),
            root.to_string_lossy().into_owned(),
        ],
    )?;
    Ok(entries_of(report))
}

/// 按稳定标识直取即 json 载体 ref 的机械回验路径。
pub fn query_by_id(
    root: &Path,
    index: &Path,
    id: &str,
) -> Result<Option<LocatorEntry>, RecallError> {
    let report = run_locator(
        root,
        &[
            "query".to_string(),
            "--index".to_string(),
            index.to_string_lossy().into_owned(),
            "--id".to_string(),
            id.to_string(),
        ],
    )?;
    Ok(entries_of(report).into_iter().next())
}
