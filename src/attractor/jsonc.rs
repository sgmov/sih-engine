//! jsonc——围堰 Python 序列化形的逐字节兼容层。
//!
//! 四类契约工件（采样合同 json、计分材料 json、核对报告 json、signcheck
//! json）的围堰落盘形为 `json.dumps(ensure_ascii=False, sort_keys=True,
//! indent=2) + 尾换行`。本模块提供同形序列化：
//! - 键序：递归按键升序（UTF-8 字节序与 Python 码点序一致）
//! - 缩进：两空格，空对象 `{}`、空数组 `[]` 紧凑形
//! - 非 ASCII：原样 UTF-8 不转义
//! - [`py_compact_sorted`]：`json.dumps` 缺省分隔符形（`", "` 与 `": "`），
//!   供 dc 指纹复算等紧凑载荷
//!
//! 另含围堰错误信封类型 [`PyError`]（类型名对齐 Python 异常族，报文文本
//! 承围堰消息原文）。

use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fmt;

/// 围堰形域错误：kind 对齐 Python 异常类型名（ValueError/OSError/ KeyError/
/// JSONDecodeError），msg 承围堰消息原文，Display 即 `"kind: msg"`。
#[derive(Debug, Clone)]
pub struct PyError {
    pub kind: &'static str,
    pub msg: String,
}

impl PyError {
    pub fn message(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for PyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl std::error::Error for PyError {}

/// ValueError 构造。
pub fn verr(msg: impl Into<String>) -> PyError {
    PyError { kind: "ValueError", msg: msg.into() }
}

/// OSError 构造（文件面）。
pub fn oserr(msg: impl Into<String>) -> PyError {
    PyError { kind: "OSError", msg: msg.into() }
}

/// JSONDecodeError 构造。
pub fn jsonerr(msg: impl Into<String>) -> PyError {
    PyError { kind: "JSONDecodeError", msg: msg.into() }
}

/// KeyError 构造。
pub fn keyerr(msg: impl Into<String>) -> PyError {
    PyError { kind: "KeyError", msg: msg.into() }
}

/// io::Error 转 OSError 信封。
pub fn io_to_py(e: &std::io::Error) -> PyError {
    oserr(format!("{}", e))
}

/// 递归按键升序重排对象（数组递归，标量原样）。
pub fn sort_value(v: Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut bt: BTreeMap<String, Value> = BTreeMap::new();
            for (k, val) in m {
                bt.insert(k, sort_value(val));
            }
            Value::Object(bt.into_iter().collect::<Map<_, _>>())
        }
        Value::Array(a) => Value::Array(a.into_iter().map(sort_value).collect()),
        other => other,
    }
}

/// 围堰落盘形：`json.dumps(ensure_ascii=False, sort_keys=True, indent=2)`，
/// 不含尾换行（落盘处自行追加）。
pub fn canonical_json(v: &Value) -> String {
    serde_json::to_string_pretty(&sort_value(v.clone())).unwrap_or_default()
}

/// 围堰紧凑形：`json.dumps(ensure_ascii=False, sort_keys=True)`（缺省分隔符
/// 即 `", "` 与 `": "`）。dc 指纹复算承此形。
pub fn py_compact_sorted(v: &Value) -> String {
    match v {
        Value::Object(m) => {
            let mut bt: BTreeMap<&String, &Value> = BTreeMap::new();
            for (k, val) in m {
                bt.insert(k, val);
            }
            let parts: Vec<String> = bt
                .into_iter()
                .map(|(k, val)| {
                    format!("{}: {}", serde_json::to_string(k).unwrap_or_default(), py_compact_sorted(val))
                })
                .collect();
            format!("{{{}}}", parts.join(", "))
        }
        Value::Array(a) => {
            let parts: Vec<String> = a.iter().map(py_compact_sorted).collect();
            format!("[{}]", parts.join(", "))
        }
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

/// Python 单引号列表字面（repr 形），拒收消息缺发明细承此形：
/// `['a', 'b']`。
pub fn py_list_repr(items: &[String]) -> String {
    let parts: Vec<String> = items
        .iter()
        .map(|s| format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")))
        .collect();
    format!("[{}]", parts.join(", "))
}

/// Python 整数列表字面（repr 形）：`[1, 2]`。
pub fn py_int_list_repr(items: &[usize]) -> String {
    let parts: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

/// 逐行读文本为 json 行列表，跳过空行；坏行按调用方语义处置。
pub fn read_lines(path: &std::path::Path) -> Result<Vec<String>, PyError> {
    let text = std::fs::read_to_string(path).map_err(|e| io_to_py(&e))?;
    Ok(text.lines().map(|s| s.to_string()).collect())
}

/// 解析 json 文本为 Value，坏形归 JSONDecodeError 信封。
pub fn parse_json(text: &str) -> Result<Value, PyError> {
    serde_json::from_str(text).map_err(|e| jsonerr(format!("{}", e)))
}

/// fsum：Shewchuk 精确和（对齐 math.fsum 的逐位精确舍入语义）。
pub fn fsum(values: &[f64]) -> f64 {
    let mut partials: Vec<f64> = Vec::new();
    for &x0 in values {
        let mut x = x0;
        let mut i = 0usize;
        for j in 0..partials.len() {
            let mut y = partials[j];
            if x.abs() < y.abs() {
                std::mem::swap(&mut x, &mut y);
            }
            let hi = x + y;
            let lo = y - (hi - x);
            if lo != 0.0 {
                partials[i] = lo;
                i += 1;
            }
            x = hi;
        }
        partials.truncate(i);
        partials.push(x);
    }
    let mut s = 0.0f64;
    for p in &partials {
        s += p;
    }
    s
}

/// Python round(x, n) 对齐形：ties-to-even 后回除。
pub fn py_round(x: f64, n: i32) -> f64 {
    let m = 10f64.powi(n);
    (x * m).round_ties_even() / m
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn canonical_sorts_keys_and_keeps_shape() {
        let v = json!({"b": 1, "a": {"d": [1, 2], "c": null}});
        assert_eq!(canonical_json(&v), "{\n  \"a\": {\n    \"c\": null,\n    \"d\": [\n      1,\n      2\n    ]\n  },\n  \"b\": 1\n}");
    }

    #[test]
    fn compact_matches_python_default_separators() {
        let v = json!({"b": 1, "a": [1, 2]});
        assert_eq!(py_compact_sorted(&v), "{\"a\": [1, 2], \"b\": 1}");
    }

    #[test]
    fn list_repr_forms() {
        assert_eq!(py_list_repr(&["x".into(), "y".into()]), "['x', 'y']");
        assert_eq!(py_int_list_repr(&[1, 2, 3]), "[1, 2, 3]");
    }
}
