//! pendline 编排层公共件：档位类型、参数解析、错误信封、用法文本。
//!
//! 正典指针：DES-017 候裁处置范式编排设计 v1
//! （doc/design/DES-017-pending-adjudication-orchestration-v1.md）；
//! 任务包 sih/state/plan/pendline.md。本模块是 pendline bin 的私有模块树
//!（经 src/bin/pendline.rs 的 #[path] 引入，lib 不导出），承载五子命令共享
//! 的机械底座：档位值域冻结（3/6/9/off，缺省 3 承设计档档位配置面）、
//! attractor 形旗标解析（重复旗标与布尔旗标扩形）、退出码三值信封
//!（0 成功／1 违规或落链失败／2 用法或环境错误）。
//!
//! 编排层零新判定语义：本模块只收集、分派、路由、留痕，不新增第二个执行者。

use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub mod collect;
pub mod config;
pub mod dispatch;
pub mod route;

pub const USAGE: &str = "用法 pendline <collect|dispatch|backfill|route|config> <子命令参数>\n\
collect: --out <候选清单.json> [--critsweep-json <判据扫出参.json>...] [--parking-json <泊界在泊.json>...] \\\n\
         [--parking-trail <trail.ndjson>...] [--covenant <候裁单.md|json>...]\n\
  判据扫源取 status=sunk 沉底判据入候选（判据沉底机械召回）；泊界 trail 只读扫\n\
  parking_entered 未出泊事件取 entry_id/title/exit_condition；候裁单 md 取列表项。\n\
dispatch: --candidates <候选清单.json> --out <合同目录> [--gears <3|6|9|off>] [--config <配置.json>] [--seat <框架:模型>]\n\
  档位即采样发数，CLI 旗标 > 配置件 > 缺省 3；off 即零采样候裁单直呈人节点全人工。\n\
backfill: --contract <合同.json> --responses <回填.jsonl> (--verdict <判词> | --verdict-material <判词材料.json>) --out <回填评分材料.json>\n\
  回填件严格对表合同 shots（facet 纪律），判词上游直读零判定。\n\
route: --materials <回填评分材料.json>... --scribe-binary <scribe> --trail <链文件> [--out <路由报告.json>] [--ttl-days <n>] \\\n\
       [--identity-report <正身报告.json>] [--session <会话号> --sessions <台账> --locks <锁册>] [--no-session-reason <事由>] [--dry-run]\n\
  三态路由：stable_clear=过（执行位标记）／boundary=boundary（人重写标记，禁自动重测禁自动入泊）／其余=未过（scribe park 入泊）。\n\
config: --config <配置.json> (--get | --set <3|6|9|off>)\n\
退出码：0 成功／1 违规或落链失败／2 用法或环境错误";

/// 档位类型：值域冻结 3/6/9/off 承 DES-017 档位配置面，缺省 3 低档。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gear {
    Off,
    Shots(i64),
}

impl Gear {
    /// CLI 串解析：值域外即 Err（调用方落退出码 2）。
    pub fn parse_cli(s: &str) -> Result<Gear, String> {
        match s {
            "3" => Ok(Gear::Shots(3)),
            "6" => Ok(Gear::Shots(6)),
            "9" => Ok(Gear::Shots(9)),
            "off" => Ok(Gear::Off),
            _ => Err(format!("档位值域为 3/6/9/off，得到 {}", s)),
        }
    }

    /// 配置件 JSON 值解析：{"gears": N} 形，N 为数 3/6/9 或串 "off"。
    pub fn from_json(v: &Value) -> Result<Gear, String> {
        match v {
            Value::Number(n) => match n.as_i64() {
                Some(3) => Ok(Gear::Shots(3)),
                Some(6) => Ok(Gear::Shots(6)),
                Some(9) => Ok(Gear::Shots(9)),
                _ => Err(format!("档位值域为 3/6/9/off，得到 {}", v)),
            },
            Value::String(s) if s == "off" => Ok(Gear::Off),
            _ => Err(format!("档位值域为 3/6/9/off，得到 {}", v)),
        }
    }

    /// 采样发数：off 即 None（零采样直呈）。
    pub fn shots(&self) -> Option<i64> {
        match self {
            Gear::Off => None,
            Gear::Shots(n) => Some(*n),
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            Gear::Off => json!("off"),
            Gear::Shots(n) => json!(n),
        }
    }
}

/// 错误信封：code 即退出码（1 违规／2 用法或环境错误），落 stderr JSON。
#[derive(Debug)]
pub struct PendError {
    pub code: i32,
    pub message: String,
}

impl PendError {
    pub fn usage(msg: impl std::fmt::Display) -> PendError {
        PendError { code: 2, message: msg.to_string() }
    }
    pub fn violate(msg: impl std::fmt::Display) -> PendError {
        PendError { code: 1, message: msg.to_string() }
    }
}

/// 子命令参数集：values 单值旗标、lists 重复旗标（保序）、bools 布尔旗标。
#[derive(Debug, Default)]
pub struct Args {
    pub values: HashMap<String, String>,
    pub lists: HashMap<String, Vec<String>>,
    pub bools: HashSet<String>,
}

impl Args {
    pub fn get(&self, name: &str) -> Result<String, PendError> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| PendError::usage(format!("缺 --{}", name)))
    }

    pub fn opt(&self, name: &str) -> Option<String> {
        self.values.get(name).cloned()
    }

    pub fn list(&self, name: &str) -> Vec<String> {
        self.lists.get(name).cloned().unwrap_or_default()
    }

    pub fn flag(&self, name: &str) -> bool {
        self.bools.contains(name)
    }
}

/// attractor 形旗标解析扩形：布尔旗标不带值，其余旗标必带值（支持
/// --key=value 与 --key value 两形），重复旗标保序累积，位置参数拒收。
pub fn parse_args(argv: &[String], bool_flags: &[&str]) -> Result<Args, PendError> {
    let mut args = Args::default();
    let bool_set: HashSet<&str> = bool_flags.iter().copied().collect();
    let mut i = 0;
    while i < argv.len() {
        let raw = &argv[i];
        let rest = match raw.strip_prefix("--") {
            Some(r) if !r.is_empty() => r,
            _ => return Err(PendError::usage(format!("意外位置参数 {}", raw))),
        };
        let (name, inline): (String, Option<String>) = match rest.split_once('=') {
            Some((k, v)) => (k.to_string(), Some(v.to_string())),
            None => (rest.to_string(), None),
        };
        if bool_set.contains(name.as_str()) {
            if inline.is_some() {
                return Err(PendError::usage(format!("布尔旗标 --{} 不取值", name)));
            }
            args.bools.insert(name);
            i += 1;
            continue;
        }
        let val = match inline {
            Some(v) => v,
            None => {
                i += 1;
                match argv.get(i) {
                    Some(v) => v.clone(),
                    None => return Err(PendError::usage(format!("旗标 --{} 缺值", name))),
                }
            }
        };
        args.values.insert(name.clone(), val.clone());
        args.lists.entry(name).or_default().push(val);
        i += 1;
    }
    Ok(args)
}

/// JSON 落盘：pretty + 尾换行，父目录自动创建。
pub fn write_json(path: &Path, value: &Value) -> Result<(), PendError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| PendError::usage(format!("目录创建失败 {}：{}", parent.display(), e)))?;
        }
    }
    let text = format!("{}\n", serde_json::to_string_pretty(value).unwrap_or_default());
    std::fs::write(path, text)
        .map_err(|e| PendError::usage(format!("写入失败 {}：{}", path.display(), e)))
}

/// 读文件为字符串：缺席或不可读即环境错误（退出码 2）。
pub fn read_text(path: &Path) -> Result<String, PendError> {
    std::fs::read_to_string(path)
        .map_err(|e| PendError::usage(format!("文件不可读 {}：{}", path.display(), e)))
}

/// 解析 JSON 文件：非法即环境错误（退出码 2）。
pub fn read_json(path: &Path) -> Result<Value, PendError> {
    let text = read_text(path)?;
    serde_json::from_str(&text)
        .map_err(|e| PendError::usage(format!("文件非合法 JSON {}：{}", path.display(), e)))
}

/// 文件名安全化：保留字母数字与 . _ -，其余替换为 _。
pub fn sanitize(name: &str) -> String {
    let out: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if out.is_empty() {
        "unnamed".to_string()
    } else {
        out
    }
}

/// 当日时刻 ISO 串（编排留痕用，不进确定性合同）。
pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}
