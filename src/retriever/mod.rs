//! 项目记忆组件库面，承接 SPEC-007 冻结契约与 SPEC-008 落差规格。
//!
//! 五档已沉淀物的检索面激活，单操作 recall 即按主题或事件或时段取切面清单带出处。
//! 只报不判即无评分无建议字段，确定性即同参同语料双跑逐字节一致，零 LLM。

pub mod archives;
pub mod axes;
pub mod facet;
pub mod locator_bridge;

use std::path::{Path, PathBuf};

use serde::Serialize;

/// 五档枚举，排序即档序 fact 加 conclusion 加 experience 加 parked 加 intent。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Archive {
    Fact,
    Conclusion,
    Experience,
    Parked,
    Intent,
}

impl Archive {
    pub fn as_str(&self) -> &'static str {
        match self {
            Archive::Fact => "fact",
            Archive::Conclusion => "conclusion",
            Archive::Experience => "experience",
            Archive::Parked => "parked",
            Archive::Intent => "intent",
        }
    }

    pub fn parse(value: &str) -> Option<Archive> {
        match value {
            "fact" => Some(Archive::Fact),
            "conclusion" => Some(Archive::Conclusion),
            "experience" => Some(Archive::Experience),
            "parked" => Some(Archive::Parked),
            "intent" => Some(Archive::Intent),
            _ => None,
        }
    }
}

/// 三轴枚举，排序即 topic 加 event 加 time 末位决胜。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    Topic,
    Event,
    Time,
}

impl Axis {
    pub fn as_str(&self) -> &'static str {
        match self {
            Axis::Topic => "topic",
            Axis::Event => "event",
            Axis::Time => "time",
        }
    }
}

/// recall 参数组，七件承 SPEC-007 接口签名。
#[derive(Clone, Debug)]
pub struct RecallArgs {
    pub root: PathBuf,
    pub topics: Vec<String>,
    pub events: Vec<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub archives: Vec<String>,
    pub at: String,
}

/// 切面记录七字段，键序固定即 archive 加 carrier 加 ref 加 axis 加 excerpt 加 matched 加 at。
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FacetRow {
    pub archive: Archive,
    pub carrier: &'static str,
    #[serde(rename = "ref")]
    pub reference: String,
    pub axis: Axis,
    pub excerpt: String,
    pub matched: String,
    pub at: String,
}

/// recall 错误面，Blocked 拦即退出码一，余四类异常即退出码二。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecallError {
    Blocked(String),
    MissingBase(String),
    TargetUnreadable(String),
    OutUnwritable(String),
    Internal(String),
}

/// 单操作 recall 即三轴编排取切面行序列，排序机械承 SPEC-008 确定性与排序节。
pub fn recall(args: &RecallArgs) -> Result<Vec<FacetRow>, RecallError> {
    let mut keep: Vec<Archive> = Vec::new();
    for name in &args.archives {
        let parsed = Archive::parse(name)
            .ok_or_else(|| RecallError::Blocked(format!("档名非枚举值 {name}")))?;
        keep.push(parsed);
    }
    let keep_all = keep.is_empty();
    let in_keep = |archive: Archive| keep_all || keep.contains(&archive);

    let has_time_axis = args.since.is_some() || args.until.is_some();
    if args.topics.is_empty() && args.events.is_empty() && !has_time_axis {
        return Err(RecallError::Blocked("轴全缺".to_string()));
    }

    let mut rows: Vec<FacetRow> = Vec::new();

    if !args.events.is_empty() || has_time_axis {
        let store = load_chain(&args.root)?;
        if !args.events.is_empty() {
            rows.extend(event_axis_rows(&store, &args.events, &keep, &args.at));
        }
        if has_time_axis {
            rows.extend(time_axis_rows(
                &store,
                args.since.as_deref(),
                args.until.as_deref(),
                &keep,
                &args.at,
            )?);
        }
    }

    if !args.topics.is_empty() {
        let index = locator_bridge::build_index(&args.root)?;
        for topic in &args.topics {
            for entry in locator_bridge::query_word_entries(&args.root, &index, topic)? {
                let Some(archive) = archives::classify_path(&entry.path) else {
                    continue;
                };
                if !in_keep(archive) {
                    continue;
                }
                let carrier: &'static str = if entry.carrier == "json" { "json" } else { "md" };
                let reference = match (carrier, entry.line_start, entry.line_end) {
                    ("json", _, _) => format!("{}@{}", entry.path, entry.id),
                    ("md", Some(ls), Some(le)) => format!("{}@{ls}-{le}", entry.path),
                    _ => continue,
                };
                rows.push(FacetRow {
                    archive,
                    carrier,
                    reference,
                    axis: Axis::Topic,
                    excerpt: facet::excerpt_of_text(&entry.text),
                    matched: topic.clone(),
                    at: args.at.clone(),
                });
            }
        }
    }

    facet::sort_rows(&mut rows);
    Ok(rows)
}

/// 切面行序列化为 ndjson 逐行，键序随结构体声明固定。
pub fn rows_to_ndjson(rows: &[FacetRow]) -> String {
    let mut out = String::new();
    for row in rows {
        out.push_str(&serde_json::to_string(row).unwrap_or_default());
        out.push('\n');
    }
    out
}

/// 错误到退出码映射即拦一异常二。
pub fn exit_code(err: &RecallError) -> i32 {
    match err {
        RecallError::Blocked(_) => 1,
        _ => 2,
    }
}

/// 输出落 --out 文件，失败映射 OutUnwritable。
pub fn write_output(path: &Path, rows: &[FacetRow]) -> Result<(), RecallError> {
    std::fs::write(path, rows_to_ndjson(rows)).map_err(|_| {
        RecallError::OutUnwritable(path.to_string_lossy().into_owned())
    })
}

/// 根定位即从给定目录上溯找 sih-engine 与 sih-tools 并存的目录层，名为 worktrees 的租约工地目录恒非根即跳过。
pub fn derive_root(from: &Path) -> Option<PathBuf> {
    let mut cur = Some(from);
    while let Some(dir) = cur {
        let is_scaffold = dir
            .file_name()
            .map(|name| name == "worktrees")
            .unwrap_or(false);
        if !is_scaffold && dir.join("sih-engine").is_dir() && dir.join("sih-tools").is_dir() {
            return Some(dir.to_path_buf());
        }
        cur = dir.parent();
    }
    None
}

#[cfg(test)]
mod derive_tests {
    use super::*;

    /// 工地名跳过即 worktrees 汇集层不被误认成根，上溯到真根。
    #[test]
    fn worktrees_layer_never_root() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("real");
        let batch = root.join("worktrees").join("sih-engine").join("batch");
        std::fs::create_dir_all(&batch).unwrap();
        std::fs::create_dir_all(root.join("worktrees").join("sih-tools")).unwrap();
        std::fs::create_dir_all(root.join("sih-engine")).unwrap();
        std::fs::create_dir_all(root.join("sih-tools")).unwrap();
        assert_eq!(derive_root(&batch), Some(root));
    }
}

/// 跨日链加载即 trail 目录文件名字典序逐文件拼接。
pub fn load_chain(root: &Path) -> Result<Vec<crate::event_stream::Event>, RecallError> {
    let dir = root.join("sih-engine").join("sih").join("event").join("trail");
    if !dir.is_dir() {
        return Err(RecallError::TargetUnreadable(
            dir.to_string_lossy().into_owned(),
        ));
    }
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .map_err(|_| RecallError::TargetUnreadable(dir.to_string_lossy().into_owned()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |x| x == "ndjson"))
        .collect();
    files.sort();
    let mut store = Vec::new();
    for file in &files {
        let events = crate::event_stream::load_events(file).map_err(|_| {
            RecallError::TargetUnreadable(file.to_string_lossy().into_owned())
        })?;
        store.extend(events);
    }
    Ok(store)
}

/// 事件轴取行即三路精确匹配按事件档属归档。
pub fn event_axis_rows(
    store: &[crate::event_stream::Event],
    tokens: &[String],
    keep: &[Archive],
    at: &str,
) -> Vec<FacetRow> {
    let keep_all = keep.is_empty();
    let mut rows = Vec::new();
    for event in store {
        let archive = archives::classify_event(&event.event_type);
        if !keep_all && !keep.contains(&archive) {
            continue;
        }
        for token in tokens {
            if axes::event_token_hit(event, token) {
                rows.push(FacetRow {
                    archive,
                    carrier: "event",
                    reference: facet::event_reference(event),
                    axis: Axis::Event,
                    excerpt: facet::excerpt_of_event(event),
                    matched: token.clone(),
                    at: at.to_string(),
                });
            }
        }
    }
    rows
}

/// 时间轴取行即东八区日界含即含。
pub fn time_axis_rows(
    store: &[crate::event_stream::Event],
    since: Option<&str>,
    until: Option<&str>,
    keep: &[Archive],
    at: &str,
) -> Result<Vec<FacetRow>, RecallError> {
    let keep_all = keep.is_empty();
    let mut rows = Vec::new();
    for event in store {
        let archive = archives::classify_event(&event.event_type);
        if !keep_all && !keep.contains(&archive) {
            continue;
        }
        if !axes::time_hit(event.timestamp, since, until)? {
            continue;
        }
        rows.push(FacetRow {
            archive,
            carrier: "event",
            reference: facet::event_reference(event),
            axis: Axis::Time,
            excerpt: facet::excerpt_of_event(event),
            matched: event.timestamp.to_rfc3339(),
            at: at.to_string(),
        });
    }
    Ok(rows)
}
