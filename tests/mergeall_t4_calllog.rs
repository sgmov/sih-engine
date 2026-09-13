//! lease-mergeleg23-parallel 簇F T4：calllogtool 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/calllog 0.1.0（core.py append 三腿齐落与 render
//! 投影再生与 reconcile 对账、cli.py 接线）。sqlite 索引腿未实装（A6 申报，
//! bin 头注落差一），本件对其余两腿金向量。fixture 全部 temp 自建。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_calllogtool() -> &'static str {
    env!("CARGO_BIN_EXE_calllogtool")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_calllogtool()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// 与 bin 同形的 Python json.dumps(sort_keys=True, separators=(",", ":")) 规范行。
fn py_compact_sorted(v: &Value) -> String {
    fn esc(s: &str) -> String {
        let mut out = String::new();
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
        out
    }
    fn rec(v: &Value, out: &mut String) {
        match v {
            Value::Object(m) => {
                let mut kvs: Vec<(&String, &Value)> = m.iter().collect();
                kvs.sort_by(|a, b| a.0.cmp(b.0));
                out.push('{');
                for (i, (k, val)) in kvs.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&esc(k));
                    out.push(':');
                    rec(val, out);
                }
                out.push('}');
            }
            Value::Array(a) => {
                out.push('[');
                for (i, val) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    rec(val, out);
                }
                out.push(']');
            }
            Value::Null => out.push_str("null"),
            Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Value::Number(n) => out.push_str(&n.to_string()),
            Value::String(s) => out.push_str(&esc(s)),
        }
    }
    let mut s = String::new();
    rec(v, &mut s);
    s
}

struct Fx {
    root: String,
    dir: PathBuf,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let root = dir.to_string_lossy().into_owned();
    Fx { root, dir, _guard: guard }
}

fn ndjson_path(fx: &Fx) -> PathBuf {
    fx.dir.join("sih-tools/calllog/calls.ndjson")
}

fn projection_path(fx: &Fx, tool: &str) -> PathBuf {
    fx.dir.join("sih-tools").join(tool).join("CALL-LOG.md")
}

fn append_args(fx: &Fx, tool: &str, occasion: &str, commands: &str) -> Vec<String> {
    vec![
        "--root".into(),
        fx.root.clone(),
        "append".into(),
        "--tool".into(),
        tool.into(),
        "--occasion".into(),
        occasion.into(),
        "--commands".into(),
        commands.into(),
        "--exit".into(),
        "0".into(),
    ]
}

// ---------- 正常形：append 两腿齐落、权威腿行形、投影 verbatim 重组 ----------

#[test]
fn t1_normal_append_two_legs() {
    let fx = build_fx();
    let args = append_args(&fx, "scribe", "单测一", "scribe verify --trail t.ndjson");
    let extra = ["--session", "s1", "--note", "n1"];
    let mut all = args.clone();
    all.extend(extra.iter().map(|s| s.to_string()));
    let (code, out, err) = run(&all.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "append 须 0，stderr={err}");
    let ev: Value = serde_json::from_str(&out).unwrap();
    assert!(ev["event_id"].as_str().unwrap().starts_with("clog-"), "事件号前缀");
    assert_eq!(ev["event_id"].as_str().unwrap().len(), 5 + 8, "clog- 加 8 位短号");
    let at = ev["at"].as_str().unwrap();
    assert!(at.ends_with("+00:00") && at.len() == 25, "秒级 UTC iso，at={at}");
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let verbatim = format!("| {today} | 单测一 | scribe verify --trail t.ndjson | 0 | s1 | n1 |");
    assert_eq!(ev["verbatim"], verbatim, "表格行渲染形");
    assert_eq!(ev["tool"], "scribe");
    assert_eq!(ev["exit"], "0");
    assert_eq!(ev["note"], "n1");
    assert_eq!(ev["session"], "s1");

    // 权威腿：一行、sort_keys 紧凑形逐字节
    let text = fs::read_to_string(ndjson_path(&fx)).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 1);
    let row: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(lines[0], py_compact_sorted(&row), "权威腿行须 sort_keys 紧凑形");
    assert_eq!(row["event_id"], ev["event_id"]);

    // 投影腿：scribe 册子按序 verbatim 重组
    let proj = fs::read_to_string(projection_path(&fx, "scribe")).unwrap();
    assert_eq!(proj, format!("{verbatim}\n"), "投影腿 verbatim 重组加尾换行");

    // 第二笔异 tool：各册各归其位
    let args2 = append_args(&fx, "formatter", "单测二", "formatter --pack p doc.md");
    let (code, out2, _) = run(&args2.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    let ev2: Value = serde_json::from_str(&out2).unwrap();
    let text = fs::read_to_string(ndjson_path(&fx)).unwrap();
    assert_eq!(text.lines().count(), 2, "权威腿两笔");
    let proj_scribe = fs::read_to_string(projection_path(&fx, "scribe")).unwrap();
    assert_eq!(proj_scribe.lines().count(), 1, "scribe 册不受 formatter 笔影响");
    let proj_formatter = fs::read_to_string(projection_path(&fx, "formatter")).unwrap();
    assert_eq!(proj_formatter, format!("{}\n", ev2["verbatim"].as_str().unwrap()));
}

// ---------- 拒绝形：缺必填旗标、import 与 rebuild 未实装位 ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();

    // append 缺 --occasion：用法错退出码 2
    let mut args = append_args(&fx, "scribe", "占位", "cmd");
    // 移除 --occasion 与其值（第 5、6 个元素：--root root append --tool .. --occasion .. --commands ..）
    let pos = args.iter().position(|a| a == "--occasion").unwrap();
    args.remove(pos);
    args.remove(pos);
    let (code, _, err) = run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 2, "缺 --occasion 须 2，err={err}");
    assert!(err.contains("用法错") || err.contains("--occasion"), "stderr 报用法，err={err}");

    // 缺 --root：用法错退出码 2
    let (code, _, _) = run(&["append", "--tool", "scribe", "--occasion", "o", "--commands", "c", "--exit", "0"]);
    assert_eq!(code, 2, "缺 --root 须 2");

    // import：与围堰同形占位，退出码 2
    let (code, _, err) = run(&["--root", &fx.root, "import"]);
    assert_eq!(code, 2, "import 占位须 2");
    assert!(err.contains("暂不实现"), "stderr 报占位，err={err}");

    // rebuild：sqlite 索引腿未实装（A6 申报），退出码 2
    let (code, _, err) = run(&["--root", &fx.root, "rebuild"]);
    assert_eq!(code, 2, "rebuild 未实装须 2");
    assert!(err.contains("未实装"), "stderr 报未实装，err={err}");

    // 未知子命令：退出码 2
    let (code, _, _) = run(&["--root", &fx.root, "no-such-sub"]);
    assert_eq!(code, 2);
}

// ---------- 边界形：render 再生幂等、空册创建、reconcile 对账 ----------

#[test]
fn t3_edge_render_reconcile() {
    let fx = build_fx();
    let mut args = append_args(&fx, "scribe", "边界", "scribe crosscheck --report r.json");
    args.extend(["--note", "n"].iter().map(|s| s.to_string()));
    let (code, out, _) = run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    let ev: Value = serde_json::from_str(&out).unwrap();
    let verbatim = ev["verbatim"].as_str().unwrap();

    // 删投影再 render：从权威腿再生，逐字节同前
    let proj = projection_path(&fx, "scribe");
    fs::remove_file(&proj).unwrap();
    let (code, _, _) = run(&["--root", &fx.root, "render", "--tool", "scribe"]);
    assert_eq!(code, 0, "render 须 0");
    assert_eq!(fs::read_to_string(&proj).unwrap(), format!("{verbatim}\n"), "再生逐字节还原");

    // 无行 tool 的 render：派生创建空册
    let (code, _, _) = run(&["--root", &fx.root, "render", "--tool", "gauge"]);
    assert_eq!(code, 0);
    let gauge_proj = projection_path(&fx, "gauge");
    assert!(gauge_proj.is_file(), "空册须派生创建");
    assert_eq!(fs::read_to_string(&gauge_proj).unwrap(), "", "空册零内容");

    // reconcile：db 腿未实装（db_only 恒空），ndjson_only 即全量权威腿 id；
    // per_tool 19 册齐；投影行为 markdown 非 JSON → projection_ids 恒空（围堰同形）
    let (code, out, _) = run(&["--root", &fx.root, "reconcile"]);
    assert_eq!(code, 0, "reconcile 须 0，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["db_only"].as_array().unwrap().len(), 0);
    let nd_only = rep["ndjson_only"].as_array().unwrap();
    assert_eq!(nd_only.len(), 1, "一笔在册");
    assert_eq!(nd_only[0], ev["event_id"]);
    let per_tool = rep["per_tool"].as_object().unwrap();
    assert_eq!(per_tool.len(), 19, "19 册投影面齐");
    let scribe_entry = &per_tool["scribe"];
    assert_eq!(scribe_entry["ndjson_ids_for_tool"].as_array().unwrap().len(), 1);
    assert_eq!(scribe_entry["projection_ids"].as_array().unwrap().len(), 0, "投影行非 JSON 恒空");
    assert_eq!(scribe_entry["missing_in_projection"].as_array().unwrap().len(), 0);
}
