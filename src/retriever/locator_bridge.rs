//! 寻址底座只读桥即子进程调用 build 与 query 两步，承接 SPEC-008 组件边界节。

use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::RecallError;

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
    std::env::temp_dir().join(format!("retrievergate-index-{}.ndjson", std::process::id()))
}

/// 建索引于系统临时目录不落工作区，缺席或失败映射 MissingBase 即报缺席件名 locator。
pub fn build_index(root: &Path) -> Result<PathBuf, RecallError> {
    let pack = pack_path(root);
    if !pack.is_file() {
        return Err(RecallError::MissingBase(
            "locator packs/memory/pack.json".to_string(),
        ));
    }
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
