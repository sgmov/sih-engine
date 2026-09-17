//! 引擎共享退出封套基座：CLI bin 侧错误报文发射与终退出的唯一实现位。
//!
//! 承接 pk-062（引擎共享单元层家位）与 pk-060 既裁口径（包装适配不吞并、行为零漂移）：
//! critsweep、gauge、lease 三 bin 逐字节一致的 `sort_json`／`emit`／`die` 副本归并于此，
//! meter 的 `die_uncaught` 与 calllogtool 的 `die_value_error` 以 [`die_plain`] 参数化承载，
//! attnanchor 的同字节 `sort_json`／`emit` 对同承此位。各 bin 保留薄包装或直接引用，
//! 报文格式逐字节不变。
//!
//! 边界：
//! - 纯发射原语，零治理语义，无包消费；本模块是库件，自身不设命令面。
//! - 谓词求值归墨斗（`snapline`），退出封套归本件，两域不混。
//! - 未并族照留各 bin：流（stdout/stderr）与格式（紧凑单行／不排序 pretty／py_json）
//!   与信封构造各异的 fail/emit 族（retriever、askroute、nomenclator、incubation、
//!   checker、basemgr、commitlaw、scrutinator、scribe、viewer、ask3repeater、
//!   confledger、locksview）强并即行为漂移，不并。

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

/// JSON 键递归排序（BTreeMap 归一），原三 bin 副本逐字节移植。
pub fn sort_json(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let bt: BTreeMap<&String, &Value> = m.iter().collect();
            Value::Object(
                bt.into_iter()
                    .map(|(k, val)| (k.clone(), sort_json(val)))
                    .collect(),
            )
        }
        Value::Array(a) => Value::Array(a.iter().map(sort_json).collect()),
        other => other.clone(),
    }
}

/// 排序键 pretty JSON 加尾换行，报文格式与原副本逐字节一致。
pub fn emit(v: &Value) -> String {
    let mut s = serde_json::to_string_pretty(&sort_json(v)).unwrap();
    s.push('\n');
    s
}

/// 错误信封终退出：stderr 排序 pretty JSON（error 必带，detail 非 null 才带），退出码 rc。
pub fn die(rc: i32, err: &str, detail: Value) -> ! {
    let mut m = Map::new();
    m.insert("error".into(), json!(err));
    if !detail.is_null() {
        m.insert("detail".into(), detail);
    }
    eprintln!("{}", emit(&Value::Object(m)));
    std::process::exit(rc);
}

/// 纯文本终退出：stderr 单行加退出码，报文由调用方组好（围堰未捕获路径近似形）。
pub fn die_plain(rc: i32, msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(rc);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 零漂移锚测：排序键、两空格缩进、尾换行，形与归并前副本一致。
    #[test]
    fn emit_sorted_pretty_with_trailing_newline() {
        let v = json!({"b": 1, "a": {"d": 3, "c": 2}});
        assert_eq!(
            emit(&v),
            "{\n  \"a\": {\n    \"c\": 2,\n    \"d\": 3\n  },\n  \"b\": 1\n}\n"
        );
    }

    /// 数组元素保序，仅对象键排序。
    #[test]
    fn sort_json_orders_object_keys_only() {
        let v = json!([{"z": 1, "a": 2}]);
        assert_eq!(emit(&v), "[\n  {\n    \"a\": 2,\n    \"z\": 1\n  }\n]\n");
    }
}
