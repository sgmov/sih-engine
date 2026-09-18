//! config 面：档位配置件持存，读写两面齐备。
//!
//! 配置件形 JSON {"gears": N}，N 为数 3/6/9 或串 "off"；值域冻结承
//! DES-017 档位配置面，值域外即退出码 2。读面（--get）缺文件如实报缺省
//! 3 与来源；写面（--set）机械落盘。

use serde_json::json;
use std::path::Path;

use super::{write_json, Args, Gear, PendError};

pub const CONFIG_VERSION: i64 = 1;

/// 配置件读面：文件缺席即 Ok(None)；存在但非法（非 JSON、缺 gears、
/// 值域外）即 Err（退出码 2）。
pub fn load(path: &Path) -> Result<Option<Gear>, PendError> {
    if !path.is_file() {
        return Ok(None);
    }
    let v = super::read_json(path)?;
    let gears = v
        .get("gears")
        .ok_or_else(|| PendError::usage(format!("配置件缺 gears 字段：{}", path.display())))?;
    Gear::from_json(gears)
        .map(Some)
        .map_err(|e| PendError::usage(format!("配置件档位非法 {}：{}", path.display(), e)))
}

/// 档位解析序：CLI 旗标 > 配置件 > 缺省 3（承设计档缺省低档令源）。
pub fn resolve(cli_gears: Option<String>, config_path: Option<String>) -> Result<Gear, PendError> {
    if let Some(raw) = cli_gears {
        return Gear::parse_cli(&raw).map_err(PendError::usage);
    }
    if let Some(cp) = config_path {
        if let Some(g) = load(Path::new(&cp))? {
            return Ok(g);
        }
    }
    Ok(Gear::Shots(3))
}

/// 子命令入口：--get 读面或 --set 写面。
pub fn run(args: &Args) -> Result<i32, PendError> {
    let cfg_path = args.get("config")?;
    let path = Path::new(&cfg_path);
    if args.flag("get") {
        match load(path)? {
            Some(g) => {
                println!(
                    "{}",
                    serde_json::to_string(&json!({"gears": g.to_json(), "source": "config"}))
                        .unwrap_or_default()
                );
            }
            None => {
                println!(
                    "{}",
                    serde_json::to_string(&json!({
                        "gears": Gear::Shots(3).to_json(),
                        "source": "default",
                    }))
                    .unwrap_or_default()
                );
            }
        }
        return Ok(0);
    }
    if let Some(raw) = args.opt("set") {
        let gear = Gear::parse_cli(&raw).map_err(PendError::usage)?;
        let out = json!({"kind": "pendline-config", "version": CONFIG_VERSION, "gears": gear.to_json()});
        write_json(path, &out)?;
        println!(
            "{}",
            serde_json::to_string(&json!({"config": cfg_path, "gears": gear.to_json()}))
                .unwrap_or_default()
        );
        return Ok(0);
    }
    Err(PendError::usage("config 须 --get 或 --set <3|6|9|off> 二择一"))
}
