//! 项目记忆组件库面，承接 SPEC-007 冻结契约与 SPEC-008 落差规格。
//!
//! 五档已沉淀物的检索面激活，单操作 recall 即按主题或事件或时段取切面清单带出处。
//! 只报不判即无评分无建议字段，确定性即同参同语料双跑逐字节一致，零 LLM。

pub mod archives;
pub mod axes;
pub mod facet;
pub mod locator_bridge;

pub use archives::Layout;

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

/// 四轴枚举，排序即 topic 加 word 加 event 加 time 末位决胜。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    Topic,
    Word,
    Event,
    Time,
}

impl Axis {
    pub fn as_str(&self) -> &'static str {
        match self {
            Axis::Topic => "topic",
            Axis::Word => "word",
            Axis::Event => "event",
            Axis::Time => "time",
        }
    }
}

/// recall 参数组：原七件承 SPEC-007 冻结加 word 文轴词列与 miss_log 可选参承 SPEC-008 修订六。
#[derive(Clone, Debug)]
pub struct RecallArgs {
    pub root: PathBuf,
    pub topics: Vec<String>,
    pub events: Vec<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub archives: Vec<String>,
    pub at: String,
    /// 文轴词列：每个词对寻址索引 entry.text 做逐字子串 contains 匹配，产 axis=word 切面行。
    pub words: Vec<String>,
    /// 零命中账可选参：带参时对每个产零行的查询词追加一行 ndjson 四字段 at/axis/word/rows=0。
    pub miss_log: Option<PathBuf>,
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

/// md 原始行区间窗口即按路径缓存整档行取行区间逐字词面，出处可机械回验由构造保证。
///
/// 档不可开档或行区间越界回 None，调用位不录活引用；窗口词面含行首标记，
/// 与重开比对位（md 行区间重开比对）同词面零语义视图加工。
fn md_raw_window(
    root: &Path,
    cache: &mut std::collections::HashMap<String, Option<Vec<String>>>,
    path: &str,
    line_start: u32,
    line_end: u32,
) -> Option<String> {
    let lines = cache.entry(path.to_string()).or_insert_with(|| {
        std::fs::read_to_string(root.join(path))
            .ok()
            .map(|text| text.lines().map(|line| line.to_string()).collect())
    });
    let lines = lines.as_ref()?;
    let ls = line_start as usize;
    let le = line_end as usize;
    if ls < 1 || le < ls || le > lines.len() {
        return None;
    }
    Some(lines[ls - 1..le].join("\n"))
}

/// 单操作 recall 即四轴编排取切面行序列，排序机械承 SPEC-008 确定性与排序节。
///
/// md 活引用抽取规则即出处可机械回验由构造保证：md 行摘录取原始行区间窗口词面
/// （逐字含行首标记），窗口不可开档或行区间越界即不录活引用。
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
    if args.topics.is_empty()
        && args.events.is_empty()
        && !has_time_axis
        && args.words.is_empty()
    {
        return Err(RecallError::Blocked("轴全缺".to_string()));
    }

    let mut rows: Vec<FacetRow> = Vec::new();

    // 布局宽判：canonical 形走城档面，非两形根回落 first_domain 路径由既有
    // 降级命名报错如实显形（F-8 语义逐字节承袭）；严判只在 bin --root 显式面。
    let layout = layout_form(&args.root).unwrap_or(Layout::FirstDomain);

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

    if !args.topics.is_empty() || !args.words.is_empty() {
        let index = locator_bridge::build_index(&args.root)?;
        let entries = locator_bridge::load_entries(&index)?;
        let mut md_windows: std::collections::HashMap<String, Option<Vec<String>>> =
            std::collections::HashMap::new();
        if !args.topics.is_empty() {
            for topic in &args.topics {
                for entry in locator_bridge::query_word_entries(&args.root, &index, topic)? {
                    let Some(archive) = archives::classify_path_in(layout, &entry.path) else {
                        continue;
                    };
                    if !in_keep(archive) {
                        continue;
                    }
                    let carrier: &'static str = if entry.carrier == "json" { "json" } else { "md" };
                    let (reference, excerpt) = match (carrier, entry.line_start, entry.line_end) {
                        ("json", _, _) => (
                            format!("{}@{}", entry.path, entry.id),
                            facet::excerpt_of_text(&entry.text),
                        ),
                        ("md", Some(ls), Some(le)) => {
                            let Some(window) =
                                md_raw_window(&args.root, &mut md_windows, &entry.path, ls, le)
                            else {
                                continue;
                            };
                            (
                                format!("{}@{ls}-{le}", entry.path),
                                facet::excerpt_of_text(&window),
                            )
                        }
                        _ => continue,
                    };
                    rows.push(FacetRow {
                        archive,
                        carrier,
                        reference,
                        axis: Axis::Topic,
                        excerpt,
                        matched: topic.clone(),
                        at: args.at.clone(),
                    });
                }
            }
        }
        if !args.words.is_empty() {
            // miss_log 账：带参且产零行即每词一行，缺参零写。
            let mut per_word_rows: Vec<(String, usize)> = Vec::new();
            for word in &args.words {
                let mut count = 0usize;
                for entry in &entries {
                    let Some(archive) = archives::classify_path_in(layout, &entry.path) else {
                        continue;
                    };
                    if !in_keep(archive) {
                        continue;
                    }
                    let carrier: &'static str =
                        if entry.carrier == "json" { "json" } else { "md" };
                    if !entry.text.contains(word.as_str()) {
                        continue;
                    }
                    let (reference, excerpt) = match (carrier, entry.line_start, entry.line_end) {
                        ("json", _, _) => (
                            format!("{}@{}", entry.path, entry.id),
                            facet::excerpt_of_text(&entry.text),
                        ),
                        ("md", Some(ls), Some(le)) => {
                            let Some(window) =
                                md_raw_window(&args.root, &mut md_windows, &entry.path, ls, le)
                            else {
                                continue;
                            };
                            (
                                format!("{}@{ls}-{le}", entry.path),
                                facet::excerpt_of_text(&window),
                            )
                        }
                        _ => continue,
                    };
                    count += 1;
                    rows.push(FacetRow {
                        archive,
                        carrier,
                        reference,
                        axis: Axis::Word,
                        excerpt,
                        matched: word.clone(),
                        at: args.at.clone(),
                    });
                }
                per_word_rows.push((word.clone(), count));
            }
            if let Some(log_path) = &args.miss_log {
                if let Some(parent) = log_path.parent() {
                    if !parent.as_os_str().is_empty() {
                        std::fs::create_dir_all(parent).map_err(|_| {
                            RecallError::OutUnwritable(log_path.to_string_lossy().into_owned())
                        })?;
                    }
                }
                let mut buf = String::new();
                for (word, count) in &per_word_rows {
                    if *count == 0 {
                        let row = serde_json::json!({
                            "at": args.at,
                            "axis": Axis::Word.as_str(),
                            "word": word,
                            "rows": 0u32,
                        });
                        buf.push_str(&serde_json::to_string(&row).unwrap_or_default());
                        buf.push('\n');
                    }
                }
                if !buf.is_empty() {
                    use std::io::Write;
                    let mut f = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(log_path)
                        .map_err(|_| {
                            RecallError::OutUnwritable(log_path.to_string_lossy().into_owned())
                        })?;
                    f.write_all(buf.as_bytes()).map_err(|_| {
                        RecallError::OutUnwritable(log_path.to_string_lossy().into_owned())
                    })?;
                }
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

/// 零命中信封：recall 零命中时写 JSON 信封件而非零字节，使命中与零命中在文件层可辨。
pub fn write_output_envelope(
    path: &Path,
    rows: &[FacetRow],
    topics: &[String],
) -> Result<(), RecallError> {
    let body = if rows.is_empty() {
        let envelope = serde_json::json!({
            "envelope": "recall",
            "topics": topics,
            "count": 0,
        });
        format!("{envelope}\n")
    } else {
        rows_to_ndjson(rows)
    };
    std::fs::write(path, body).map_err(|_| {
        RecallError::OutUnwritable(path.to_string_lossy().into_owned())
    })
}

#[cfg(test)]
mod envelope_tests {
    use super::*;

    #[test]
    fn zero_hit_writes_envelope_not_empty_file() {
        let dir = std::env::temp_dir().join(format!("retr-env-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("out.ndjson");
        write_output_envelope(&p, &[], &["注意力 预算".to_string()]).unwrap();
        let body = std::fs::read_to_string(&p).unwrap();
        assert!(!body.trim().is_empty(), "envelope must not be zero bytes");
        let v: serde_json::Value = body.trim_end().parse().expect("json");
        assert_eq!(v["count"], 0);
        assert_eq!(v["envelope"], "recall");
        std::fs::remove_dir_all(&dir).ok();
    }
}

/// 根定位即从给定目录上溯找 sih-engine 与 sih-tools 并存的目录层，名为 worktrees 的租约工地目录恒非根即跳过。
pub fn derive_root(from: &Path) -> Option<PathBuf> {
    let mut cur = Some(from);
    while let Some(dir) = cur {
        let is_scaffold = dir
            .file_name()
            .map(|name| name == "worktrees")
            .unwrap_or(false);
        if !is_scaffold && layout_form(dir).is_some() {
            return Some(dir.to_path_buf());
        }
        cur = dir.parent();
    }
    None
}

/// 布局判别（wengumcp-parallel 批件二）：root 下 sih-engine 与 sih-tools 目录俱在
/// 即 first_domain 中央双仓形；root 下 sih/ledger 为目录即 canonical 新城正典域形
/// （与 mcpline 开域落地形及 lease detect_domain_context 同构）；两形俱缺即 None。
pub fn layout_form(root: &Path) -> Option<Layout> {
    if root.join("sih-engine").is_dir() && root.join("sih-tools").is_dir() {
        return Some(Layout::FirstDomain);
    }
    if root.join("sih").join("ledger").is_dir() {
        return Some(Layout::Canonical);
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

#[cfg(test)]
mod md_window_tests {
    use super::*;

    /// md 窗口即原始行词面逐字承载：行首标记保留在窗内，与重开比对位同词面。
    #[test]
    fn md_raw_window_verbatim_includes_line_markers() {
        let tmp = tempfile::tempdir().unwrap();
        let rel = "doc/note.md";
        std::fs::create_dir_all(tmp.path().join("doc")).unwrap();
        std::fs::write(tmp.path().join(rel), "# t\n\n> 批：a 批\n> 会话：s1\n\n后文\n").unwrap();
        let mut cache = std::collections::HashMap::new();
        let window = md_raw_window(tmp.path(), &mut cache, rel, 3, 4).unwrap();
        assert_eq!(window, "> 批：a 批\n> 会话：s1");
        let again = md_raw_window(tmp.path(), &mut cache, rel, 3, 4).unwrap();
        assert_eq!(window, again, "同路径二查缓存同读数");
    }

    /// 行区间越界与逆区间与档缺席即不录：None 不 panic 不误录。
    #[test]
    fn md_raw_window_oob_or_missing_is_none() {
        let tmp = tempfile::tempdir().unwrap();
        let rel = "doc/note.md";
        std::fs::create_dir_all(tmp.path().join("doc")).unwrap();
        std::fs::write(tmp.path().join(rel), "一行\n二行\n").unwrap();
        let mut cache = std::collections::HashMap::new();
        assert_eq!(md_raw_window(tmp.path(), &mut cache, rel, 0, 1), None, "行号零越下界");
        assert_eq!(md_raw_window(tmp.path(), &mut cache, rel, 2, 5), None, "越上界");
        assert_eq!(md_raw_window(tmp.path(), &mut cache, rel, 3, 2), None, "逆区间");
        assert_eq!(
            md_raw_window(tmp.path(), &mut cache, "doc/absent.md", 1, 1),
            None,
            "档缺席"
        );
    }
}

/// 集成测试：依赖真实工作区索引，跑慢，故 #[ignore]；实装后用 cargo test -- --ignored 跑。
///
/// 工地根解析：测试在 worktree 内跑时 derive_root 必须能上溯到真根。
fn integration_root() -> PathBuf {
    let cwd = std::env::current_dir().expect("cwd");
    derive_root(&cwd).expect("root 不可定位")
}

#[cfg(test)]
mod wenguobs_tests {
    use super::*;

    /// 真实工作区根解析恒存在。
    #[test]
    fn integration_root_resolves() {
        let root = integration_root();
        assert!(root.join("sih-engine").is_dir());
        assert!(root.join("sih-tools").is_dir());
    }

    /// F-1.1 文轴：六落空词经 --word 全命中且 ref 可回查原档。
    #[test]
    #[ignore]
    fn f1_word_axis_hits_six_words() {
        let root = integration_root();
        let args = RecallArgs {
            root,
            topics: vec![],
            events: vec![],
            since: None,
            until: None,
            archives: vec![],
            at: "2026-08-31".to_string(),
            words: vec![
                "令牌".to_string(),
                "use 边".to_string(),
                "边账".to_string(),
                "期票".to_string(),
                "句读".to_string(),
                "级联".to_string(),
            ],
            miss_log: None,
        };
        let rows = recall(&args).expect("recall 失败");
        let six: Vec<&str> = vec!["令牌", "use 边", "边账", "期票", "句读", "级联"];
        for word in &six {
            let hits: Vec<&FacetRow> =
                rows.iter().filter(|r| r.matched == *word && r.axis == Axis::Word).collect();
            assert!(!hits.is_empty(), "文轴未命中 {word}");
            let hit = hits[0];
            // ref 形如 path@Lstart-Lend 即 md 载体回原档行区间
            assert!(
                hit.reference.contains('@'),
                "ref 缺 @ 分隔即 {ref}",
                ref = hit.reference
            );
        }
    }

    /// F-1.2 文轴对 name 轴零变：RecallArgs 仅传 topics 不传 words 时行为与 v1.4 退路一致。
    #[test]
    #[ignore]
    fn f1_topic_axis_unchanged_when_words_absent() {
        let root = integration_root();
        let baseline = RecallArgs {
            root: root.clone(),
            topics: vec!["期票".to_string()],
            events: vec![],
            since: None,
            until: None,
            archives: vec![],
            at: "2026-08-31".to_string(),
            words: vec![],
            miss_log: None,
        };
        let baseline_rows = recall(&baseline).expect("baseline 失败");
        let baseline_count = baseline_rows
            .iter()
            .filter(|r| r.axis == Axis::Topic)
            .count();
        assert!(baseline_count > 0, "主轴基线本应有命中");

        // 加 words 但不命中任一新词时，topic 行集应完全一致
        let with_words = RecallArgs {
            words: vec!["句读".to_string()],
            ..baseline.clone()
        };
        let with_rows = recall(&with_words).expect("扩展 失败");
        let topic_rows: Vec<&FacetRow> =
            with_rows.iter().filter(|r| r.axis == Axis::Topic).collect();
        let baseline_topic: Vec<&FacetRow> =
            baseline_rows.iter().filter(|r| r.axis == Axis::Topic).collect();
        assert_eq!(topic_rows.len(), baseline_topic.len(), "topic 行数变化");
        for (a, b) in topic_rows.iter().zip(baseline_topic.iter()) {
            assert_eq!(a.reference, b.reference, "topic ref 漂移");
        }
    }

    /// F-1.3 --word 与 --topic 同词并用时两轴行并出。
    #[test]
    #[ignore]
    fn f1_word_and_topic_same_word_both_axes() {
        let root = integration_root();
        let args = RecallArgs {
            root,
            topics: vec!["期票".to_string()],
            events: vec![],
            since: None,
            until: None,
            archives: vec![],
            at: "2026-08-31".to_string(),
            words: vec!["期票".to_string()],
            miss_log: None,
        };
        let rows = recall(&args).expect("recall 失败");
        let has_topic = rows.iter().any(|r| r.axis == Axis::Topic && r.matched == "期票");
        let has_word = rows.iter().any(|r| r.axis == Axis::Word && r.matched == "期票");
        assert!(has_topic && has_word, "两轴应并存：topic={has_topic} word={has_word}");
    }

    /// F-2.1 miss_log 缺参零写：未带参时 miss_log 不应被写。
    #[test]
    #[ignore]
    fn f2_miss_log_absent_keeps_default_readonly() {
        let root = integration_root();
        let target = std::env::temp_dir().join("wenguobs-miss-log-absent.ndjson");
        let _ = std::fs::remove_file(&target);
        let args = RecallArgs {
            root,
            topics: vec![],
            events: vec![],
            since: None,
            until: None,
            archives: vec![],
            at: "2026-08-31".to_string(),
            words: vec!["wenguobsnonexistent".to_string()],
            miss_log: None,
        };
        let _ = recall(&args).expect("recall 失败");
        assert!(!target.exists(), "缺参形态不应写 miss_log");
    }

    /// F-2.2 miss_log 带参对零命中词追加 ndjson 一行四字段。
    /// 选词用 `zzz_wenguobs_unique_20260831_xyz`——本批"wenguobsnonexistent" 已被
    /// wenguobs-solo-results.md 第 78/79/81/89 行实测收录，逐字子串包含即命中即非零，
    /// 选此词真零命中且工作区无碰撞。
    #[test]
    #[ignore]
    fn f2_miss_log_records_zero_hit_axis_word() {
        let root = integration_root();
        let target = std::env::temp_dir().join("wenguobs-miss-log-word.ndjson");
        let _ = std::fs::remove_file(&target);
        let args = RecallArgs {
            root,
            topics: vec![],
            events: vec![],
            since: None,
            until: None,
            archives: vec![],
            at: "2026-08-31".to_string(),
            words: vec!["zzz_wenguobs_unique_20260831_xyz".to_string()],
            miss_log: Some(target.clone()),
        };
        let _ = recall(&args).expect("recall 失败");
        assert!(target.exists(), "带参形态应写 miss_log");
        let content = std::fs::read_to_string(&target).expect("miss_log 读失败");
        let lines: Vec<&str> = content.split_terminator('\n').filter(|l| !l.is_empty()).collect();
        assert_eq!(lines.len(), 1, "应恰一行账");
        let row: serde_json::Value = serde_json::from_str(lines[0]).expect("行非 json");
        assert_eq!(row["at"], "2026-08-31");
        assert_eq!(row["axis"], "word");
        assert_eq!(row["word"], "zzz_wenguobs_unique_20260831_xyz");
        assert_eq!(row["rows"], 0);
    }

    /// F-2.3 miss_log 重复查询词重复记行即 append-only 不去重。
    /// 选词同 F-2.2，理由同上。
    #[test]
    #[ignore]
    fn f2_miss_log_repeats_each_call() {
        let root = integration_root();
        let target = std::env::temp_dir().join("wenguobs-miss-log-rep.ndjson");
        let _ = std::fs::remove_file(&target);
        let mk = || RecallArgs {
            root: root.clone(),
            topics: vec![],
            events: vec![],
            since: None,
            until: None,
            archives: vec![],
            at: "2026-08-31".to_string(),
            words: vec!["zzz_wenguobs_unique_20260831_xyz".to_string()],
            miss_log: Some(target.clone()),
        };
        let _ = recall(&mk()).expect("first 失败");
        let _ = recall(&mk()).expect("second 失败");
        let content = std::fs::read_to_string(&target).expect("miss_log 读失败");
        let lines: Vec<&str> = content.split_terminator('\n').filter(|l| !l.is_empty()).collect();
        assert_eq!(lines.len(), 2, "重复查询应双份账行");
    }
}

/// 跨日链加载即 trail 目录文件名字典序逐文件拼接。
pub fn load_chain(root: &Path) -> Result<Vec<crate::event_stream::Event>, RecallError> {
    let dir = match layout_form(root) {
        Some(Layout::Canonical) => root.join("sih").join("event").join("trail"),
        _ => root.join("sih-engine").join("sih").join("event").join("trail"),
    };
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
