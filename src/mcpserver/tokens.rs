//! mcpline tokens 明文登记册（DES-015 项目域识别形：标识牌台账实装）。
//!
//! 行为对等基准：sih-tools/mcpline/src/mcpline/tokens.py。标识牌是项目在
//! HTTP 面的名字牌，明文短标识即可；台账行固定六字段，停行即追加新行同
//! token_id 载 status stopped，读数取每 token_id 末行（append-only 与链同构）。
//! 写点纪律：flock 串行加 O_APPEND 原子整行，禁整文件重写。本模块是 sihmcp
//! 面唯一 sanctioned 直写盘点位（登记册是标识面数据非执法账本，与 writeface
//! 零直写盘红线分域不冲突；零写守卫白名单单点同 session.rs 正身暂存形）。

use std::io::Write;
use std::path::Path;

use chrono::Local;
use serde::{Deserialize, Serialize};

/// 台账行六字段（DES-015 判词照录，固定序）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenRow {
    pub token_id: String,
    pub domain_root: String,
    pub scope: String,
    pub status: String,
    pub issued_at: String,
    pub issued_by: String,
}

pub const SCOPE_READONLY: &str = "readonly";
pub const SCOPE_DOMAIN_WRITE: &str = "domain_write";
pub const SCOPE_CUSTOM: &str = "custom";
pub const VALID_SCOPES: [&str; 3] = [SCOPE_READONLY, SCOPE_DOMAIN_WRITE, SCOPE_CUSTOM];
pub const STATUS_ACTIVE: &str = "active";
pub const STATUS_STOPPED: &str = "stopped";
pub const VALID_STATUSES: [&str; 2] = [STATUS_ACTIVE, STATUS_STOPPED];

/// 登记册行形违例（调用形教学位，调用方转错误载荷）。
#[derive(Debug, Clone)]
pub struct TokensLedgerError(pub String);

impl std::fmt::Display for TokensLedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// issued_at 时戳（本地时区 ISO 形，秒精度）。
pub fn now_stamp() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string()
}

/// token_id 词形：可读短串，人可读可辨认可手输。
pub fn token_id_valid(token_id: &str) -> bool {
    use std::sync::OnceLock;
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"^[A-Za-z0-9][A-Za-z0-9_.-]{0,63}$").unwrap());
    re.is_match(token_id)
}

/// 行形校验：恰六字段、scope 与 status 在档、可读短串词形。
pub fn validate_row(row: &TokenRow) -> Result<(), TokensLedgerError> {
    for (field, value) in [
        ("token_id", &row.token_id),
        ("domain_root", &row.domain_root),
        ("issued_at", &row.issued_at),
        ("issued_by", &row.issued_by),
    ] {
        if value.trim().is_empty() {
            return Err(TokensLedgerError(format!("登记行字段 {field} 须非空字符串")));
        }
    }
    if !token_id_valid(&row.token_id) {
        return Err(TokensLedgerError(format!(
            "token_id 词形违例：'{}' 须可读短串（字母数字起，限 [A-Za-z0-9_.-]，至多 64 位）",
            row.token_id
        )));
    }
    if !VALID_SCOPES.contains(&row.scope.as_str()) {
        return Err(TokensLedgerError(format!(
            "scope 违例：'{}' 须三档之一 {:?}",
            row.scope, VALID_SCOPES
        )));
    }
    if !VALID_STATUSES.contains(&row.status.as_str()) {
        return Err(TokensLedgerError(format!(
            "status 违例：'{}' 须 {:?}",
            row.status, VALID_STATUSES
        )));
    }
    Ok(())
}

/// 台账追加行：flock 串行加 append 单次 write 原子整行，禁整文件重写。
/// 文件缺席即建（目录须在位）；返回所追加行。
pub fn append_row(path: &Path, row: &TokenRow) -> Result<TokenRow, TokensLedgerError> {
    validate_row(row)?;
    let line = serde_json::to_string(row).map_err(|e| TokensLedgerError(e.to_string()))? + "\n";
    use std::fs::OpenOptions;
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|e| TokensLedgerError(format!("登记册打开失败 {}: {e}", path.display())))?;
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        unsafe {
            if libc::flock(f.as_raw_fd(), libc::LOCK_EX) != 0 {
                return Err(TokensLedgerError("登记册加锁失败".to_string()));
            }
        }
        let r = f.write_all(line.as_bytes()).and_then(|_| f.flush());
        unsafe {
            libc::flock(f.as_raw_fd(), libc::LOCK_UN);
        }
        r.map_err(|e| TokensLedgerError(format!("登记册写入失败: {e}")))?;
    }
    #[cfg(not(unix))]
    {
        f.write_all(line.as_bytes())
            .map_err(|e| TokensLedgerError(format!("登记册写入失败: {e}")))?;
    }
    Ok(row.clone())
}

/// 登记册读数：逐行解析，每 token_id 取末行；文件缺席即空册；行形违例显形
/// 带行号（账面损坏不静默跳过）。
pub fn load_last_rows(path: &Path) -> Result<std::collections::BTreeMap<String, TokenRow>, TokensLedgerError> {
    let mut rows = std::collections::BTreeMap::new();
    if !path.is_file() {
        return Ok(rows);
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| TokensLedgerError(format!("登记册读取失败: {e}")))?;
    for (idx, raw) in text.lines().enumerate() {
        let lineno = idx + 1;
        let stripped = raw.trim();
        if stripped.is_empty() {
            continue;
        }
        let row: TokenRow = serde_json::from_str(stripped)
            .map_err(|e| TokensLedgerError(format!("登记册第 {lineno} 行非 JSON：{e}")))?;
        validate_row(&row).map_err(|e| {
            TokensLedgerError(format!("登记册第 {lineno} 行违例：{e}"))
        })?;
        rows.insert(row.token_id.clone(), row);
    }
    Ok(rows)
}

/// 单标识解析：末行在档即行，不在册即 None（识别语义单点解析位）。
pub fn resolve_token(path: &Path, token_id: &str) -> Result<Option<TokenRow>, TokensLedgerError> {
    Ok(load_last_rows(path)?.get(token_id).cloned())
}

/// 签发行构造：status active，issued_at 取实时刻。
pub fn issue_row(token_id: &str, domain_root: &str, scope: &str, issued_by: &str) -> TokenRow {
    TokenRow {
        token_id: token_id.to_string(),
        domain_root: domain_root.to_string(),
        scope: scope.to_string(),
        status: STATUS_ACTIVE.to_string(),
        issued_at: now_stamp(),
        issued_by: issued_by.to_string(),
    }
}

/// 停行行构造：同 token_id 载 status stopped，域根与 scope 承末行。
pub fn revoke_row(last_row: &TokenRow, issued_by: &str) -> TokenRow {
    TokenRow {
        token_id: last_row.token_id.clone(),
        domain_root: last_row.domain_root.clone(),
        scope: last_row.scope.clone(),
        status: STATUS_STOPPED.to_string(),
        issued_at: now_stamp(),
        issued_by: issued_by.to_string(),
    }
}
