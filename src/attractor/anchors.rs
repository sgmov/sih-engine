//! anchors——锚定三步验证（融回自围堰 facet/src/anchors.py，承 DES-003）。
//!
//! 三步：一文件存在（path 须存在，含 basename 反查处理路径缩写）；二行号
//! 合法（range 须落在文件行数内）；三声明白名单（anchor 须在 topic.md
//! anchors 列表）。
//!
//! range 三形：单行 `L42`、多行 `L42-L100`、开放区间 `L42-`（兼容无 L 前缀）。

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// 解析单条 anchor 的 range 字段，返回 (start, end)，end 为 None 表示开放区间。
pub fn parse_range(range_str: &str) -> Option<(i64, Option<i64>)> {
    let s = range_str.trim();
    let pattern = regex::Regex::new(r"^L?([0-9]+)(?:-(L?([0-9]*)))?$").unwrap();
    let caps = pattern.captures(s)?;
    let start: i64 = caps.get(1)?.as_str().parse().ok()?;
    let end = caps.get(2).and_then(|m| {
        if m.as_str().is_empty() {
            None
        } else {
            caps.get(3).and_then(|n| n.as_str().parse().ok())
        }
    });
    Some((start, end))
}

/// 白名单单条目。
#[derive(Debug, Clone)]
pub struct WhitelistEntry {
    pub path: String,
    pub line_start: i64,
    pub line_end: Option<i64>,
    pub note: String,
}

fn entry_from_dict(d: &[(String, String)]) -> Option<WhitelistEntry> {
    let get = |k: &str| d.iter().find(|(kk, _)| kk == k).map(|(_, v)| v.clone());
    let path = get("path")?;
    let range = get("range")?;
    let (line_start, line_end) = parse_range(&range)?;
    Some(WhitelistEntry {
        path,
        line_start,
        line_end,
        note: get("note").unwrap_or_default(),
    })
}

/// 从 topic.md 的 `## anchors` 段解析白名单条目列表。
pub fn parse_topic_anchors(topic_path: &Path) -> Result<Vec<WhitelistEntry>, super::jsonc::PyError> {
    let content = std::fs::read_to_string(topic_path)
        .map_err(|e| super::jsonc::io_to_py(&e))?;
    let mut in_anchors = false;
    let mut current: Vec<(String, String)> = Vec::new();
    let mut entries: Vec<WhitelistEntry> = Vec::new();

    let entry_re = regex::Regex::new(r"^-\s+path:\s*(.+)$").unwrap();
    let kv_re = regex::Regex::new(r"^(\w+):\s*(.+)$").unwrap();

    for line in content.split('\n') {
        let stripped = line.trim();
        if stripped.starts_with("## ") {
            if in_anchors {
                if !current.is_empty() {
                    if let Some(e) = entry_from_dict(&current) {
                        entries.push(e);
                    }
                    current = Vec::new();
                }
                break;
            }
            if stripped == "## anchors" || stripped.starts_with("## anchors") {
                in_anchors = true;
                continue;
            }
            if in_anchors {
                break;
            }
            continue;
        }
        if !in_anchors {
            continue;
        }
        if stripped.starts_with("- ") {
            if !current.is_empty() {
                if let Some(e) = entry_from_dict(&current) {
                    entries.push(e);
                }
                current = Vec::new();
            }
            if let Some(m) = entry_re.captures(stripped) {
                current.push(("path".to_string(), m.get(1).unwrap().as_str().trim().to_string()));
            }
        } else if stripped.contains(':') {
            if let Some(m) = kv_re.captures(stripped) {
                current.push((
                    m.get(1).unwrap().as_str().to_string(),
                    m.get(2).unwrap().as_str().trim().to_string(),
                ));
            }
        }
    }
    if in_anchors && !current.is_empty() {
        if let Some(e) = entry_from_dict(&current) {
            entries.push(e);
        }
    }
    Ok(entries)
}

fn count_lines(path: &Path) -> Option<usize> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(text.lines().count())
}

/// 路径词汇绝对化（围堰 Path.resolve 的确定性近似：连根 + 词典化归一，
/// 不追符号链接）。
fn abs_norm(root: &Path, rel: &str) -> PathBuf {
    let joined = if Path::new(rel).is_absolute() {
        PathBuf::from(rel)
    } else {
        root.join(rel)
    };
    let mut out = PathBuf::from("/");
    for comp in joined.components() {
        use std::path::Component;
        match comp {
            Component::RootDir | Component::Prefix(_) => {}
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            Component::Normal(c) => out.push(c),
        }
    }
    out
}

fn rel_display(root: &Path, abs: &Path) -> String {
    match abs.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().into_owned(),
        Err(_) => abs.to_string_lossy().into_owned(),
    }
}

/// 从 topic.md 解析白名单并展开为 (path, line) 集合。开放区间展开到该文件
/// 实际行数；路径按 project_root 解析。
pub fn load_whitelist(topic_path: &Path, project_root: &Path) -> Result<HashSet<(String, i64)>, super::jsonc::PyError> {
    let entries = parse_topic_anchors(topic_path)?;
    let mut whitelist: HashSet<(String, i64)> = HashSet::new();
    for entry in entries {
        let anchor_path = abs_norm(project_root, &entry.path);
        let rel_path = rel_display(&project_root.to_path_buf(), &anchor_path);
        match entry.line_end {
            None => {
                if let Some(n) = count_lines(&anchor_path) {
                    for line in entry.line_start..=(n as i64) {
                        whitelist.insert((rel_path.clone(), line));
                    }
                }
            }
            Some(end) => {
                for line in entry.line_start..=end {
                    whitelist.insert((rel_path.clone(), line));
                }
            }
        }
    }
    Ok(whitelist)
}

/// 单条 anchor 验证结果。
#[derive(Debug, Clone)]
pub struct AnchorValidation {
    pub result: &'static str, // Keep / Drop
    pub path: String,
    pub range: String,
    pub reason: Option<String>, // not_in_whitelist / file_not_found / line_out_of_range / ambiguous_path
}

impl AnchorValidation {
    fn drop(reason: &str) -> Self {
        AnchorValidation { result: "Drop", path: String::new(), range: String::new(), reason: Some(reason.to_string()) }
    }
    fn drop_at(path: &str, range: &str, reason: &str) -> Self {
        AnchorValidation { result: "Drop", path: path.to_string(), range: range.to_string(), reason: Some(reason.to_string()) }
    }
}

/// 锚定三步验证。anchor_ref 形 `<path>#L<line>` 或 `<path>#<line>`，多行
/// range 支持 `#L42-L100` / `#42-100` / `#42-`。
pub fn validate(anchor_ref: &str, whitelist: &HashSet<(String, i64)>, project_root: &Path) -> AnchorValidation {
    if !anchor_ref.contains('#') {
        return AnchorValidation::drop("not_in_whitelist");
    }
    let (path_part, line_part) = match anchor_ref.rsplit_once('#') {
        Some((p, l)) => (p, l),
        None => return AnchorValidation::drop("not_in_whitelist"),
    };
    if path_part.is_empty() || line_part.is_empty() {
        return AnchorValidation::drop("not_in_whitelist");
    }
    let line_num_str = line_part.strip_prefix('L').unwrap_or(line_part);
    let start_line: i64 = if line_num_str.contains('-') {
        let start_str = line_num_str.split('-').next().unwrap_or("");
        match start_str.parse() {
            Ok(v) => v,
            Err(_) => return AnchorValidation::drop("not_in_whitelist"),
        }
    } else {
        match line_num_str.parse() {
            Ok(v) => v,
            Err(_) => return AnchorValidation::drop("not_in_whitelist"),
        }
    };

    let mut anchor_path = abs_norm(project_root, path_part);
    let mut rel_path = rel_display(&project_root.to_path_buf(), &anchor_path);

    // 第一步：文件存在（含 basename 反查，处理路径缩写）
    if !anchor_path.exists() {
        let basename = Path::new(path_part)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let whitelist_paths: HashSet<&String> = whitelist.iter().map(|(p, _)| p).collect();
        let matches: Vec<&String> = whitelist_paths
            .into_iter()
            .filter(|p| Path::new(p.as_str()).file_name().map(|f| f.to_string_lossy() == basename).unwrap_or(false))
            .collect();
        if matches.len() == 1 {
            let matched_path = matches[0].clone();
            anchor_path = abs_norm(project_root, &matched_path);
            rel_path = matched_path;
            if !anchor_path.exists() {
                return AnchorValidation::drop_at(&rel_path, line_part, "file_not_found");
            }
        } else if matches.len() > 1 {
            return AnchorValidation::drop_at(path_part, line_part, "ambiguous_path");
        } else {
            return AnchorValidation::drop_at(&rel_path, line_part, "file_not_found");
        }
    }

    // 第二步：行号合法
    let file_lines = match count_lines(&anchor_path) {
        Some(n) => n as i64,
        None => return AnchorValidation::drop_at(&rel_path, line_part, "line_out_of_range"),
    };
    if start_line < 1 || start_line > file_lines {
        return AnchorValidation::drop_at(&rel_path, line_part, "line_out_of_range");
    }

    // 第三步：声明白名单
    if !whitelist.contains(&(rel_path.clone(), start_line)) {
        return AnchorValidation::drop_at(&rel_path, line_part, "not_in_whitelist");
    }

    AnchorValidation { result: "Keep", path: rel_path, range: line_part.to_string(), reason: None }
}
