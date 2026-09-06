//! 共享 glob 匹配
//!
//! 两条语义分支不重叠，分别承两侧既有实现：
//! - [`match_path_glob`]：path-anchored 语义（核阅域 include/exclude），承
//!   Python `(?:^|/)` 锚定
//! - [`match_fnmatch_glob`]：fnmatch 兼容语义（得一域文件名匹配），承 Python
//!   `(?s:...)\Z` dotall
//!
//! 两侧原实现均在编译失败时 fail-closed 返回 `false`，本件保留此行为。

/// path-anchored glob 匹配
/// 承工具件 Python `(?:^|/)` 锚定语义：双星跨目录段，单星段内任意，模式可
/// 自任一路径边界锚定
pub fn match_path_glob(pattern: &str, path: &str) -> bool {
    if pattern == "**" || pattern == "**/*" {
        return true;
    }
    // 编译模式为正则：(?:^|/) 前缀 + 模式体
    // 模式体：`**/` → `(?:[^/]+/)*`、`*` → `[^/]*`、`?` → `[^/]`、其他字面量
    let mut body = String::new();
    let mut i = 0;
    let n = pattern.len();
    let bytes = pattern.as_bytes();
    while i < n {
        if i + 3 <= n && &bytes[i..i + 3] == b"**/" {
            body.push_str("(?:[^/]+/)*");
            i += 3;
        } else if bytes[i] == b'*' {
            body.push_str("[^/]*");
            i += 1;
        } else if bytes[i] == b'?' {
            body.push_str("[^/]");
            i += 1;
        } else {
            // ASCII 字面量用 regex::escape，非 ASCII 直接 push
            let c = pattern[i..].chars().next().unwrap();
            let clen = c.len_utf8();
            // escape 只对 ASCII 安全字母元字符；非 ASCII 视为字面
            body.push_str(&regex::escape(&pattern[i..i + clen]));
            i += clen;
        }
    }
    let full = format!("(?:^|/){body}");
    match regex::Regex::new(&full) {
        Ok(re) => re.is_match(path),
        Err(_) => false,
    }
}

/// Python fnmatchcase 语义 glob 匹配
/// 承 Python `fnmatch.translate`：`*`→`.*`、`?`→`.`、`[seq]`/`[!seq]`
/// 字符类（集内 `!` 首位转 `^`、首 `]` 字面、`]` 截止、`^` 首位转义）、
/// 其余字符 re.escape；未闭合 `[` 按字面。全串匹配大小写敏感
/// （fnmatchcase），dotall 承 Python `(?s:...)\Z` 即 Rust `(?s:...)\z`。
pub fn match_fnmatch_glob(text: &str, pattern: &str) -> bool {
    // 逐次编译小而稀疏；包内模式有限，确定性优先
    let re = match build_fnmatch_regex(pattern) {
        Ok(re) => re,
        Err(_) => return false,
    };
    re.is_match(text)
}

fn build_fnmatch_regex(pattern: &str) -> Result<regex::Regex, String> {
    if pattern == "*" {
        return regex::Regex::new("(?s:.*)\\z").map_err(|e| e.to_string());
    }
    let chars: Vec<char> = pattern.chars().collect();
    let n = chars.len();
    let mut res = String::new();
    let mut i = 0usize;
    while i < n {
        let c = chars[i];
        i += 1;
        match c {
            '*' => res.push_str(".*"),
            '?' => res.push('.'),
            '[' => {
                let mut j = i;
                if j < n && chars[j] == '!' {
                    j += 1;
                }
                if j < n && chars[j] == ']' {
                    j += 1;
                }
                while j < n && chars[j] != ']' {
                    j += 1;
                }
                if j >= n {
                    res.push_str("\\[");
                } else {
                    let mut stuff: String = chars[i..j].iter().collect();
                    if stuff.starts_with('!') {
                        stuff = format!("^{}", &stuff[1..]);
                    } else if stuff.starts_with('^') {
                        stuff = format!("\\{stuff}");
                    }
                    res.push('[');
                    res.push_str(&stuff);
                    res.push(']');
                    i = j + 1;
                }
            }
            _ => res.push_str(&regex::escape(&c.to_string())),
        }
    }
    regex::Regex::new(&format!("(?s:{res})\\z")).map_err(|e| e.to_string())
}
