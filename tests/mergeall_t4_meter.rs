//! lease-mergeleg23-parallel 簇F T4：meter 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/meter 0.2.0（cli.py：run 包裹执行按日落册、count
//! 三方对表汇总、crosscheck 链上事件漏计检出）。fixture 全部 temp 自建，
//! 围堰真实 counts 目录零触碰。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn bin_meter() -> &'static str {
    env!("CARGO_BIN_EXE_meter")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_meter()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

struct Fx {
    dir: PathBuf,
    counts: String,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let counts = dir.join("counts").to_string_lossy().into_owned();
    Fx { dir, counts, _guard: guard }
}

fn today() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

// ---------- 正常形：run 落册、count 三方对表 ----------

#[test]
fn t1_normal_run_and_count() {
    let fx = build_fx();
    // run 包裹 /bin/echo scribe：透传退出码 0，归因 scribe，落当日册
    // （echo 的 stdout 经 meter 继承透传，故 out 里应见被包裹命令输出）
    let (code, out, err) = run(&["run", "--counts", &fx.counts, "--", "/bin/echo", "scribe"]);
    assert_eq!(code, 0, "echo 透传须 0，stderr={err}");
    assert!(out.contains("scribe"), "被包裹命令输出须透传，out={out}");

    let counts_dir = std::path::Path::new(&fx.counts);
    let day_file = counts_dir.join(format!("{}.ndjson", today()));
    assert!(day_file.is_file(), "当日册须在：{}", day_file.display());
    let text = fs::read_to_string(&day_file).unwrap();
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 1, "一次包裹恰一行记录（单点测度不重不漏）");
    let rec: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(rec["tool"], "scribe", "归因扫描须命中 scribe");
    assert_eq!(rec["exit_code"], 0);
    assert!(rec["duration_ms"].is_number());
    assert_eq!(rec["command"][0], "/bin/echo");
    assert_eq!(rec["command"][1], "scribe");
    assert!(rec["ts"].as_str().unwrap().starts_with(&today()));

    // count：total 与 by_tool 与 by_date 三方对表
    let (code, out, _) = run(&["count", "--counts", &fx.counts]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["engine"]["name"], "meter");
    assert_eq!(rep["engine"]["version"], "0.2.0");
    assert_eq!(rep["total"], 1);
    assert_eq!(rep["by_tool"]["scribe"], 1);
    assert_eq!(rep["by_date"][today()], 1);

    // 归因 null 形：无已知工具记号 → "null" 键
    let (code, _, _) = run(&["run", "--counts", &fx.counts, "--", "/bin/echo", "plain"]);
    assert_eq!(code, 0);
    let (code, out, _) = run(&["count", "--counts", &fx.counts, "--tool", "scribe"]);
    assert_eq!(code, 0, "按工具过滤须 0");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["total"], 1, "过滤后只见 scribe 一笔");
}

// ---------- 拒绝形：空命令、trail 缺席、未知子命令 ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();

    // run 空命令：command_missing，退出码 2
    let (code, out, _) = run(&["run", "--counts", &fx.counts, "--"]);
    assert_eq!(code, 2, "空命令须 2，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["status"], "error");
    assert_eq!(rep["error"], "command_missing");

    // spawn 失败：退出码 127 且仍落册
    let (code, _, err) = run(&["run", "--counts", &fx.counts, "--", "/no/such/binary-t4"]);
    assert_eq!(code, 127, "spawn 失败透传 127");
    assert!(err.contains("命令不可执行"), "stderr 须报不可执行，err={err}");
    let (code, out, _) = run(&["count", "--counts", &fx.counts]);
    assert_eq!(code, 0);
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["total"], 1, "127 亦落册");
    let day_file = std::path::Path::new(&fx.counts).join(format!("{}.ndjson", today()));
    let text = fs::read_to_string(&day_file).unwrap();
    let rec: Value = serde_json::from_str(text.lines().last().unwrap()).unwrap();
    assert_eq!(rec["exit_code"], 127);

    // crosscheck trail 缺席：退出码 2 trail_not_found
    let (code, out, _) = run(&["crosscheck", "--counts", &fx.counts, "--trail", "/no/such/trail-t4.ndjson"]);
    assert_eq!(code, 2, "trail 缺席须 2，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["status"], "error");
    assert_eq!(rep["error"], "trail_not_found");

    // 缺子命令 / 未知子命令：退出码 2
    let (code, _, _) = run(&[]);
    assert_eq!(code, 2);
    let (code, _, _) = run(&["no-such-sub"]);
    assert_eq!(code, 2);
}

// ---------- 边界形：crosscheck 漏计检出、pre_meter 豁免、quiet 紧凑行 ----------

#[test]
fn t3_edge_crosscheck_flagged_pre_meter_quiet() {
    let fx = build_fx();
    // 手写当日册：scribe 在 2026-09-13 有一笔
    let counts_dir = std::path::Path::new(&fx.counts);
    fs::create_dir_all(counts_dir).unwrap();
    let rec = json_record();
    fs::write(counts_dir.join("2026-09-13.ndjson"), format!("{rec}\n")).unwrap();

    // trail1：当日 certification_completed，report_path 暗含 formatter（未计数 → 漏计旗）
    let trail1 = fx.dir.join("trail1.ndjson");
    fs::write(
        &trail1,
        r#"{"event_id": "e1", "event_type": "certification_completed", "timestamp": "2026-09-13T12:00:00+00:00", "details": {"report_path": "sih-tools/formatter/reports/r.json"}}
"#,
    )
    .unwrap();

    // trail2：2026-09-11 事件早于首笔册（pre_meter 豁免不 flag）
    let trail2 = fx.dir.join("trail2.ndjson");
    fs::write(
        &trail2,
        r#"{"event_id": "e2", "event_type": "intent_refined", "timestamp": "2026-09-11T09:00:00+00:00", "details": {"report_path": ""}}
"#,
    )
    .unwrap();

    let t1 = trail1.to_string_lossy().into_owned();
    let t2 = trail2.to_string_lossy().into_owned();
    let (code, out, _) = run(&["crosscheck", "--counts", &fx.counts, "--trail", &t1, "--trail", &t2]);
    assert_eq!(code, 1, "有漏计须 1，stdout={out}");
    let rep: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(rep["meter_records"], 1);
    assert_eq!(rep["checked_events"], 1);
    assert_eq!(rep["pre_meter_events"], 1);
    assert_eq!(rep["flagged_count"], 1);
    let flagged = rep["flagged"].as_array().unwrap();
    assert_eq!(flagged[0]["tool"], "formatter", "scribe 在册、formatter 漏计");
    assert_eq!(flagged[0]["event_id"], "e1");
    assert_eq!(flagged[0]["event_date"], "2026-09-13");
    assert_eq!(flagged[0]["reason"], "no_meter_record");

    // 隔日对表：册面在 D-1 亦认（days = {当日, 前日}）
    let trail3 = fx.dir.join("trail3.ndjson");
    fs::write(
        &trail3,
        r#"{"event_id": "e3", "event_type": "certification_completed", "timestamp": "2026-09-14T08:00:00+00:00", "details": {"report_path": ""}}
"#,
    )
    .unwrap();
    let t3 = trail3.to_string_lossy().into_owned();
    let (code, out, _) = run(&["crosscheck", "--counts", &fx.counts, "--trail", &t3]);
    assert_eq!(code, 0, "D-1 有册即不旗（隔日对表），stdout={out}");

    // quiet 紧凑行：单行 JSON code 1
    let (code, out, _) = run(&["crosscheck", "--counts", &fx.counts, "--trail", &t1, "--trail", &t2, "--quiet"]);
    assert_eq!(code, 1);
    let raw = out.trim();
    assert!(!raw.contains('\n'), "quiet 须单行，got={raw}");
    let line: Value = serde_json::from_str(raw).unwrap();
    assert_eq!(line["tool"], "meter");
    assert_eq!(line["command"], "crosscheck");
    assert_eq!(line["code"], 1);
    assert_eq!(line["summary"]["flagged_count"], 1);
    assert_eq!(line["summary"]["checked_events"], 1);
    assert_eq!(line["summary"]["pre_meter_events"], 1);
    assert_eq!(line["summary"]["meter_records"], 1);
}

fn json_record() -> String {
    // 与 bin 落册形同构：ts、command、tool、exit_code、duration_ms
    format!(
        r#"{{"ts": "2026-09-13T10:00:00+00:00", "command": ["/bin/echo", "scribe"], "tool": "scribe", "exit_code": 0, "duration_ms": 1}}"#
    )
}
