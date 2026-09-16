//! sih init 薄壳黑箱测试（adoptface 批，SPEC-026 测试设计规范承载位：
//! sih-engine/doc/spec/SPEC-026-engine-test-design-v1.md；被测正典指针见
//! src/bin/sih.rs 头注：SPEC-025 与 DEC-023 与 DES-015）。
//!
//! 黑箱法：CARGO_BIN_EXE_sih 运行时定位产物，临时沙箱全密闭——SIH_ROOT 与
//! SIH_MCPLINE_CODE_ROOT 钉临时中央根；中央根预置三件（任务包模板源、中央
//! 登记册本域 active 行、伪 scribe 脚本于 sih-engine/target/debug/scribe）；
//! 断言 sih 树生成与重复运行幂等守卫（明确拒非静默）与逐项判词整形。
//! 伪 scribe 只承 open_domain 两步出参形（append 出 event_hash 落链一行、
//! verify 出 valid 判词），零真链校验。unix 专属（脚本执行位）。
#![cfg(unix)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

const TOKEN_ID: &str = "sandboxdom";
const FAKE_SCRIBE: &str = r#"#!/bin/sh
# 伪 scribe（沙箱黑箱测试位，零真链校验）：
# append：读 --report 文件内容出定形哈希，落 --trail 一行，stdout 出 event_hash
# verify：stdout 出 {"status":"valid"}
cmd=""; trail=""; report=""; prev=""
for a in "$@"; do
  case "$prev" in
    --trail) trail=$a ;;
    --report) report=$a ;;
  esac
  case "$a" in
    append|verify) cmd=$a ;;
  esac
  prev=$a
done
if [ "$cmd" = append ]; then
  hash="fake$(shasum -a 256 < "$report" 2>/dev/null | cut -c1-16)"
  printf '{"event_hash":"%s"}\n' "$hash" >> "$trail"
  printf '{"event_hash":"%s"}\n' "$hash"
elif [ "$cmd" = verify ]; then
  printf '{"status":"valid"}\n'
fi
exit 0
"#;

struct Sandbox {
    _dir: tempfile::TempDir,
    central: PathBuf,
    dom: PathBuf,
}

fn sandbox() -> Sandbox {
    let dir = tempfile::tempdir().expect("沙箱临时目录建立拒");
    let central = dir.path().join("central");
    let dom = dir.path().join("dom");
    fs::create_dir_all(central.join("sih-tools/lease")).unwrap();
    fs::create_dir_all(central.join("sih-tools/mcpline/ledger")).unwrap();
    fs::create_dir_all(central.join("sih-engine/target/debug")).unwrap();
    fs::create_dir_all(dom.join(".git")).unwrap();
    // 中央任务包模板源（open_domain 步三字节复制源，在位即可）
    fs::write(
        central.join("sih-tools/lease/TASK-PACKAGE-TEMPLATE.md"),
        "# 任务包模板（沙箱占位）\n",
    )
    .unwrap();
    // 伪 scribe
    let scribe = central.join("sih-engine/target/debug/scribe");
    fs::write(&scribe, FAKE_SCRIBE).unwrap();
    make_executable(&scribe);
    // 中央登记册：本域 active 行（domain_root 用 canonicalize 形对表 norm 比对）
    let dom_canon = fs::canonicalize(&dom).unwrap();
    let row = json!({
        "token_id": TOKEN_ID,
        "domain_root": dom_canon.display().to_string(),
        "scope": "domain_write",
        "status": "active",
        "issued_at": "2026-09-17T00:00:00+08:00",
        "issued_by": "sih-init-test",
    });
    fs::write(
        central.join("sih-tools/mcpline/ledger/tokens.ndjson"),
        format!("{row}\n"),
    )
    .unwrap();
    Sandbox { _dir: dir, central, dom }
}

fn make_executable(p: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perm = fs::metadata(p).unwrap().permissions();
    perm.set_mode(0o755);
    fs::set_permissions(p, perm).unwrap();
}

/// 黑箱跑 sih init：SIH_ROOT 与 SIH_MCPLINE_CODE_ROOT 钉沙箱中央根。
fn run_init(dom: &Path, central: &Path, extra: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_sih"))
        .args(["init", "--root"])
        .arg(dom)
        .args(extra)
        .env("SIH_ROOT", central)
        .env("SIH_MCPLINE_CODE_ROOT", central)
        .output()
        .expect("sih 产物 spawn 拒");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

fn run_init_dom(dom: &Path, central: &Path) -> (i32, String, String) {
    run_init(dom, central, &[])
}

fn parse_json(stdout: &str) -> Value {
    serde_json::from_str(stdout).unwrap_or_else(|e| {
        panic!("stdout 非合法 JSON 单对象：{e}\n{stdout}")
    })
}

fn items(v: &Value) -> Vec<Value> {
    v["precheck"]["items"]
        .as_array()
        .expect("precheck.items 缺席")
        .clone()
}

fn verdict_of(item: &Value) -> &str {
    item["verdict"].as_str().expect("verdict 非字符串")
}

#[test]
fn init_ok_creates_canonical_sih_tree() {
    let sb = sandbox();
    let (code, stdout, stderr) = run_init_dom(&sb.dom, &sb.central);
    assert_eq!(code, 0, "退出码非零：stdout={stdout} stderr={stderr}");
    let v = parse_json(&stdout);
    assert_eq!(v["ok"], json!(true), "ok 假：{stdout}");
    assert_eq!(v["stage"], json!(null), "成功形不带 stage");
    // 六项预检逐项 pass
    let its = items(&v);
    assert_eq!(its.len(), 6, "六项判词形破坏");
    for it in &its {
        assert_eq!(verdict_of(it), "pass", "预检项意外非 pass：{it}");
    }
    // 开域出参形
    assert_eq!(v["domain"]["domain_id"], json!(TOKEN_ID));
    assert_eq!(v["domain"]["chain_verify"], json!("valid"));
    assert_eq!(
        v["domain"]["files"].as_array().expect("files 缺席").len(),
        6,
        "files 六件形破坏（四模板加自述加首笔链）"
    );
    // sih 树断言（canonical 布局）
    let domc = fs::canonicalize(&sb.dom).unwrap();
    assert!(domc.join("sih/domain.json").is_file(), "域声明卡缺席");
    assert!(domc.join("sih/README.md").is_file(), "域自述卡缺席");
    assert!(
        domc.join("sih/state/plan/TEMPLATE-任务包.md").is_file(),
        "任务包模板缺席"
    );
    assert!(
        domc.join("sih/state/parking/TEMPLATE-停泊材料.json").is_file(),
        "停泊模板缺席"
    );
    assert!(
        domc.join("sih/state/plan/TEMPLATE-意图-plain.json").is_file(),
        "意图模板缺席"
    );
    // 开域首笔落链：trail 目录恰一 ndjson 且带 event_hash
    let trail_dir = domc.join("sih/event/trail");
    let entries: Vec<PathBuf> = fs::read_dir(&trail_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(entries.len(), 1, "trail 目录应恰一链文件：{entries:?}");
    let trail_text = fs::read_to_string(&entries[0]).unwrap();
    assert!(
        trail_text.contains("event_hash"),
        "开域首笔无 event_hash：{trail_text}"
    );
    // 域声明卡字段形
    let decl: Value = serde_json::from_str(&fs::read_to_string(domc.join("sih/domain.json")).unwrap())
        .expect("域声明卡非合法 JSON");
    assert_eq!(decl["domain_id"], json!(TOKEN_ID));
    assert_eq!(decl["layout_form"], json!("canonical"));
    assert_eq!(decl["opened_by"], json!("main-window-bootstrap"));
}

#[test]
fn init_rerun_explicitly_rejected_and_idempotent() {
    let sb = sandbox();
    let (c1, s1, e1) = run_init_dom(&sb.dom, &sb.central);
    assert_eq!(c1, 0, "首次 init 失败：{s1} {e1}");
    let domc = fs::canonicalize(&sb.dom).unwrap();
    let decl_path = domc.join("sih/domain.json");
    let decl_before = fs::read(&decl_path).unwrap();
    let trail_dir = domc.join("sih/event/trail");
    let trail_path = fs::read_dir(&trail_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .next()
        .unwrap();
    let trail_before = fs::read(&trail_path).unwrap();

    // 重跑：幂等守卫明确拒（退出码 1 前置拒），非静默成功非半态写
    let (c2, s2, _e2) = run_init_dom(&sb.dom, &sb.central);
    assert_eq!(c2, 1, "重跑应前置拒退出码 1：stdout={s2}");
    let v = parse_json(&s2);
    assert_eq!(v["ok"], json!(false));
    assert_eq!(v["stage"], json!("precheck"));
    let err = v["error"].as_str().unwrap_or_default();
    assert!(err.contains("域已开域"), "拒语缺幂等守卫教学：{err}");
    assert!(v["fix"].as_str().unwrap_or_default().contains("reinit"), "fix 缺处置教学");
    // 逐项判词：第五项 fail，一至四 pass，六 unverified
    let its = items(&v);
    assert_eq!(its.len(), 6);
    assert_eq!(verdict_of(&its[4]), "fail", "第五项应为 fail：{its:?}");
    assert!(
        its[4]["detail"].as_str().unwrap_or_default().contains("域已开域"),
        "fail 项缺原拒语"
    );
    for it in &its[0..4] {
        assert_eq!(verdict_of(it), "pass", "前序项应 pass：{it}");
    }
    assert_eq!(verdict_of(&its[5]), "unverified", "后序项应 unverified");
    // 幂等零副作用：域声明卡与链文件字节恒等
    assert_eq!(fs::read(&decl_path).unwrap(), decl_before, "域声明卡被重写");
    assert_eq!(fs::read(&trail_path).unwrap(), trail_before, "链文件被追加");
}

#[test]
fn init_rejects_gitless_root_at_item_two_with_zero_writes() {
    let sb = sandbox();
    fs::remove_dir_all(sb.dom.join(".git")).unwrap();
    let (code, stdout, _stderr) = run_init_dom(&sb.dom, &sb.central);
    assert_eq!(code, 1, ".git 缺席应前置拒：stdout={stdout}");
    let v = parse_json(&stdout);
    assert!(v["error"].as_str().unwrap_or_default().contains(".git 缺席"));
    let its = items(&v);
    assert_eq!(verdict_of(&its[0]), "pass", "第一项应 pass");
    assert_eq!(verdict_of(&its[1]), "fail", "第二项应 fail");
    for it in &its[2..] {
        assert_eq!(verdict_of(it), "unverified", "后序项应 unverified：{it}");
    }
    assert!(!sb.dom.join("sih").exists(), "预检拒后零域写入");
}

#[test]
fn init_rejects_absent_root_at_item_one() {
    let sb = sandbox();
    let absent = sb._dir.path().join("nonexistent-root");
    let (code, stdout, _stderr) = run_init_dom(&absent, &sb.central);
    assert_eq!(code, 1, "域根缺席应前置拒：stdout={stdout}");
    let v = parse_json(&stdout);
    let its = items(&v);
    assert_eq!(verdict_of(&its[0]), "fail", "第一项应 fail");
    for it in &its[1..] {
        assert_eq!(verdict_of(it), "unverified", "后序项应 unverified：{it}");
    }
}

#[test]
fn usage_errors_exit_two() {
    let sb = sandbox();
    // 无参：usage 出 stderr 退出码 2
    let out = Command::new(env!("CARGO_BIN_EXE_sih"))
        .env("SIH_ROOT", &sb.central)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2), "无参应退出码 2");
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("usage: sih init"),
        "stderr 缺 usage 行"
    );
    // 未知子命令
    let out = Command::new(env!("CARGO_BIN_EXE_sih"))
        .arg("bogus")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    // --root 缺值
    let out = Command::new(env!("CARGO_BIN_EXE_sih"))
        .args(["init", "--root"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    // --help：stdout usage 退出码 0
    let out = Command::new(env!("CARGO_BIN_EXE_sih"))
        .arg("--help")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("usage: sih init"));
}
