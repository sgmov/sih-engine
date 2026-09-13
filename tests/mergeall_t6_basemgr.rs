//! lease-mergeleg6-parallel 簇I T6：basemgr 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/basemgr 0.1.0（cli.py、engine.py，登记件冻结形三操作）。
//! 三例金向量：正常（freeze 落件与 replay 只读比对）、拒绝（拒收、指纹前置、
//! refuse 双前置）、边界（六冻结对象零豁免、{ROOT} 寻径、index 去重、spec 先红
//! 后重录）。fixture 全部 temp 自建。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_basemgr")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn run_json(args: &[&str]) -> (i32, Value, String) {
    let (code, out, err) = run(args);
    (code, serde_json::from_str(&out).unwrap_or(Value::Null), err)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

struct Fx {
    dir: PathBuf,
    root: String,
    _guard: tempfile::TempDir,
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let dir = guard.path().to_path_buf();
    let root = dir.join("projroot");
    fs::create_dir_all(&root).unwrap();
    Fx { dir, root: root.display().to_string(), _guard: guard }
}

impl Fx {
    fn vec_path(&self, name: &str) -> String {
        self.dir.join("vectors").join(name).display().to_string()
    }

    fn freeze_args(&self, name: &str, cmd_json: &str) -> Vec<String> {
        vec![
            "freeze".into(),
            "--vector".into(), self.vec_path(name),
            "--command-json".into(), cmd_json.into(),
            "--kind".into(), "baseline".into(),
            "--program-version".into(), "p1".into(),
            "--pack-version".into(), "pk1".into(),
            "--root".into(), self.root.clone(),
        ]
    }
}

fn cjson(tokens: &[&str]) -> String {
    json!(tokens).to_string()
}

// ---------- 正常形：freeze 落件四元组与 replay 只读比对 ----------

#[test]
fn t1_normal_freeze_replay() {
    let fx = build_fx();
    let vp = fx.vec_path("v1.json");

    // freeze：echo hello 捕获为期望字节
    let args = fx.freeze_args("v1.json", &cjson(&["/bin/echo", "hello"]));
    let (code, out, err) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "freeze 须 0，err={err}");
    assert_eq!(out["op"], "freeze");
    assert_eq!(out["verdict"], "pass");
    assert_eq!(out["exit_code"], 0);
    let sha = sha256_hex(b"hello\n");
    assert_eq!(out["expected_sha256"], sha);
    assert_eq!(out["declaration"]["claim"], "unchanged-only");

    // 期望件与元数据件齐落，.bin 逐字节
    let base = fx.dir.join("vectors");
    let bin_path = base.join("v1.bin");
    assert_eq!(fs::read(&bin_path).unwrap(), b"hello\n");
    let meta_text = fs::read_to_string(&vp).unwrap();
    let meta: Value = serde_json::from_str(&meta_text).unwrap();
    assert_eq!(meta["kind"], "baseline");
    assert_eq!(meta["vector_version"], "0.1.0");
    assert_eq!(meta["expected_sha256"], sha);
    assert_eq!(meta["version_triple"]["program"], "p1");
    assert_eq!(meta["version_triple"]["target"], sha, "target 缺省取输出哈希");
    assert_eq!(meta["six_objects"]["program"], Value::Null);
    assert_eq!(meta["quadruple"]["expected_file"], "v1.bin");
    assert_eq!(meta["env_fingerprint"]["path_resolution"], "cli-cwd");
    assert!(meta_text.starts_with("{\n \""), "元数据 indent=1 金向量形");

    // index.json：单件登记
    let idx: Value = serde_json::from_str(&fs::read_to_string(base.join("index.json")).unwrap()).unwrap();
    assert_eq!(idx["vectors"].as_array().unwrap().len(), 1);
    assert_eq!(idx["vectors"][0]["vector"], "v1", "index 载 stem 名");
    assert_eq!(idx["vectors"][0]["expected_sha256"], sha);

    // replay 同命令：零漂移过（只读零写入）
    let mtime = fs::metadata(&bin_path).unwrap().modified().unwrap();
    let args = vec![
        "replay".to_string(),
        "--vector".to_string(), vp.clone(),
        "--command-json".to_string(), cjson(&["/bin/echo", "hello"]),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    assert_eq!(out["verdict"], "pass");
    assert_eq!(out["claim"], "unchanged-only");
    assert_eq!(fs::metadata(&bin_path).unwrap().modified().unwrap(), mtime, "replay 零写入");

    // replay 漂移：fail 退出码一，drift_fact 原始事实
    let args = vec![
        "replay".to_string(),
        "--vector".to_string(), vp,
        "--command-json".to_string(), cjson(&["/bin/echo", "world"]),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(out["verdict"], "fail");
    let fact = out["drift_fact"].as_str().unwrap();
    assert!(fact.contains("首差异偏移 0"), "首差异偏移入事实，fact={fact}");
    assert!(fact.contains("6 字节对期望 6 字节"));
}

// ---------- 拒绝形：冻结拒收、指纹前置、refuse 双前置 ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();

    // baseline 种命令非零退出 → 拒收 ExecError 2
    let args = fx.freeze_args("r1.json", &cjson(&["/bin/sh", "-c", "exit 3"]));
    let (code, _, err) = run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 2);
    assert!(err.contains("执行环境异常"), "err={err}");

    // baseline 种空输出 → 拒收 2（/bin/true 即零字节成功运行）
    let args = fx.freeze_args("r2.json", &cjson(&["/usr/bin/true"]));
    let (code, _, err) = run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 2);
    assert!(err.contains("冻结拒收"));

    // 向量件缺席 replay → 2
    let args = vec![
        "replay".to_string(),
        "--vector".to_string(), fx.vec_path("ghost.json"),
        "--command-json".to_string(), cjson(&["/bin/echo", "x"]),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, _, err) = run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 2);
    assert!(err.contains("向量件非法"));

    // 指纹前置拦截：异 root 重放 → 2
    let args = fx.freeze_args("r3.json", &cjson(&["/bin/echo", "hello"]));
    run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>()).0;
    let other_root = fx.dir.join("otherroot");
    fs::create_dir_all(&other_root).unwrap();
    let args = vec![
        "replay".to_string(),
        "--vector".to_string(), fx.vec_path("r3.json"),
        "--command-json".to_string(), cjson(&["/bin/echo", "hello"]),
        "--root".to_string(), other_root.display().to_string(),
    ];
    let (code, _, err) = run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 2);
    assert!(err.contains("指纹前置拦截"));

    // baseline 漂移无影响集 → refuse 1（未分类漂移）
    let args = fx.freeze_args("r4.json", &cjson(&["/bin/echo", "hello"]));
    run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>()).0;
    let args = vec![
        "refreeze".to_string(),
        "--vector".to_string(), fx.vec_path("r4.json"),
        "--command-json".to_string(), cjson(&["/bin/echo", "world"]),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(out["verdict"], "refuse");
    assert!(out["reason"].as_str().unwrap().contains("未分类漂移"));

    // spec 种漂移无红证 → refuse 1；有红证 → refroze 0
    let spec_args = vec![
        "freeze".to_string(),
        "--vector".to_string(), fx.vec_path("r5.json"),
        "--command-json".to_string(), cjson(&["/bin/echo", "spec-v1"]),
        "--kind".to_string(), "spec".to_string(),
        "--program-version".to_string(), "p1".to_string(),
        "--pack-version".to_string(), "pk1".to_string(),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, _, _) = run_json(&spec_args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "spec 种绿输出允许冻结");
    let args = vec![
        "refreeze".to_string(),
        "--vector".to_string(), fx.vec_path("r5.json"),
        "--command-json".to_string(), cjson(&["/bin/echo", "spec-v2"]),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert!(out["reason"].as_str().unwrap().contains("先红前置"));
    let red = fx.dir.join("red.log");
    fs::write(&red, "red evidence body").unwrap();
    let args = vec![
        "refreeze".to_string(),
        "--vector".to_string(), fx.vec_path("r5.json"),
        "--command-json".to_string(), cjson(&["/bin/echo", "spec-v2"]),
        "--red-evidence".to_string(), red.display().to_string(),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "红证在位重录过");
    assert_eq!(out["verdict"], "refroze");
    assert_eq!(fs::read(fx.dir.join("vectors/r5.bin")).unwrap(), b"spec-v2\n");

    // 用法错：缺 --command 与坏 --command-json → 2
    let (code, _, err) = run(&["replay", "--vector", &fx.vec_path("r4.json"), "--root", &fx.root]);
    assert_eq!(code, 2);
    assert!(err.contains("用法错"));
    let (code, _, err) = run(&[
        "replay", "--vector", &fx.vec_path("r4.json"),
        "--command-json", "{not json}", "--root", &fx.root,
    ]);
    assert_eq!(code, 2);
    assert!(err.contains("command-json"));
}

// ---------- 边界形：六冻结零豁免、{ROOT} 寻径、index 去重、--command 变参 ----------

#[test]
fn t3_edge_six_objects_and_root_token() {
    let fx = build_fx();

    // {ROOT} 寻径层：token 内 {ROOT} 展开为 root
    fs::write(fx.dir.join("projroot/in.txt"), "payload-x").unwrap();
    let args = fx.freeze_args("e1.json", &cjson(&["/bin/cat", "{ROOT}/in.txt"]));
    let (code, _out, err) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "err={err}");
    assert_eq!(fs::read(fx.dir.join("vectors/e1.bin")).unwrap(), b"payload-x");

    // index 去重：同名向量重冻即单登记
    let args = fx.freeze_args("e1.json", &cjson(&["/bin/cat", "{ROOT}/in.txt"]));
    run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    let idx: Value = serde_json::from_str(
        &fs::read_to_string(fx.dir.join("vectors/index.json")).unwrap(),
    )
    .unwrap();
    let vectors = idx["vectors"].as_array().unwrap();
    assert_eq!(vectors.len(), 1, "同名去重，got={vectors:?}");
    assert_eq!(vectors[0]["vector"], "e1", "同名去重载 stem 名");

    // 六冻结对象零豁免：声明位变更即 refuse 1
    let args = vec![
        "refreeze".to_string(),
        "--vector".to_string(), fx.vec_path("e1.json"),
        "--command-json".to_string(), cjson(&["/bin/cat", "{ROOT}/in.txt"]),
        "--six-now".to_string(), json!({"representative": "abc"}).to_string(),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 1);
    assert_eq!(out["verdict"], "refuse");
    assert!(out["reason"].as_str().unwrap().contains("六冻结对象变更零豁免"));
    assert!(out["reason"].as_str().unwrap().contains("representative"));

    // 六冻结对象全 null 申报：无变更，继续走（零漂移 pass）
    let args = vec![
        "refreeze".to_string(),
        "--vector".to_string(), fx.vec_path("e1.json"),
        "--command-json".to_string(), cjson(&["/bin/cat", "{ROOT}/in.txt"]),
        "--six-now".to_string(), json!({"normalizer": null}).to_string(),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    assert_eq!(out["verdict"], "pass");
    assert_eq!(out["note"], "零漂移无需重冻");

    // baseline 带 impact_set 漂移：refroze 0，attribution 载影响集
    // （impact_set 属冻结时元数据申报位，refreeze 读 meta 不收 CLI 旗标）。
    let args = vec![
        "freeze".to_string(),
        "--vector".to_string(), fx.vec_path("e2.json"),
        "--command-json".to_string(), cjson(&["/bin/echo", "before"]),
        "--kind".to_string(), "baseline".to_string(),
        "--program-version".to_string(), "p1".to_string(),
        "--pack-version".to_string(), "pk1".to_string(),
        "--impact-set".to_string(), "doc/a.md,doc/b.md".to_string(),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, _, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    let args = vec![
        "refreeze".to_string(),
        "--vector".to_string(), fx.vec_path("e2.json"),
        "--command".to_string(), "/bin/echo".to_string(), "after".to_string(),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, out, _) = run_json(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0, "out={out}");
    assert_eq!(out["verdict"], "refroze");
    assert_eq!(
        out["attribution"],
        "纯期望过期（申报影响集 ['doc/a.md', 'doc/b.md']）",
        "attribution Python 列表字面形"
    );
    assert_eq!(out["new_sha256"], sha256_hex(b"after\n"));

    // stdout 报告 indent=1 金向量形（含尾换行）
    let args = vec![
        "replay".to_string(),
        "--vector".to_string(), fx.vec_path("e2.json"),
        "--command".to_string(), "/bin/echo".to_string(), "after".to_string(),
        "--root".to_string(), fx.root.clone(),
    ];
    let (code, raw, _) = run(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(code, 0);
    assert!(raw.starts_with("{\n \"claim\": \"unchanged-only\",\n"), "indent=1 形，got={raw}");
}
