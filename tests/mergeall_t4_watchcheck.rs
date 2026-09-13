//! lease-mergeleg23-parallel 簇F T4：watchcheck 引擎 bin 金向量。
//!
//! 行为面对表围堰 sih-tools/watchcheck 0.2.0（core.py 脏文件集与三面差集谓词、
//! constants.py 判定常数冻结登记）。porcelain 解析对表 git status --porcelain
//! -uall -z 实际输出：fixture 以真 git init/commit 构造，围堰真实双仓零触碰。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin_watchcheck() -> &'static str {
    env!("CARGO_BIN_EXE_watchcheck")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(bin_watchcheck()).args(args).output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("git 可用");
    assert!(
        out.status.success(),
        "git {args:?} 失败: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

struct Fx {
    root: PathBuf,
    _guard: tempfile::TempDir,
}

fn init_repos(dir: &Path) {
    let st = dir.join("sih-tools");
    let en = dir.join("sih-engine");
    fs::create_dir_all(&st).unwrap();
    fs::create_dir_all(&en).unwrap();
    git(&st, &["init"]);
    git(&en, &["init"]);
}

fn build_fx() -> Fx {
    let guard = tempfile::TempDir::new().unwrap();
    let root = guard.path().to_path_buf();
    init_repos(&root);
    Fx { root, _guard: guard }
}

fn default_trail(root: &Path, at: &str) -> PathBuf {
    root.join("sih-engine/sih/event/trail").join(format!("{at}.ndjson"))
}

const AT: &str = "2026-09-13";

// ---------- 正常形：无主清单呈报、声明笔覆盖转净态 ----------

#[test]
fn t1_normal_unowned_then_declared_clean() {
    let fx = build_fx();
    // 围堰实证在盘形：sih-tools 下一个未跟踪活写件，当日 trail 空册
    fs::write(fx.root.join("sih-tools/foo.txt"), "活写件\n").unwrap();
    fs::create_dir_all(default_trail(&fx.root, AT).parent().unwrap()).unwrap();
    fs::write(default_trail(&fx.root, AT), "").unwrap();

    let r = fx.root.to_string_lossy().into_owned();
    let (code, out, err) = run(&["check", "--at", AT, "--root", &r]);
    assert_eq!(code, 1, "有无主修改须 1，stderr={err}");
    assert!(out.contains("稽 watchcheck 对表读数 2026-09-13（v0.2.0）"), "头行，out={out}");
    assert!(out.contains("判定式：脏文件集 −（租约锁面 ∪ 直改链笔声明面 ∪ 豁免面）= 无主清单"));
    assert!(out.contains("无主修改 1 件"), "结论行，out={out}");
    assert!(out.contains("无主清单："));
    assert!(out.contains("- sih-tools/foo.txt | mtime "), "无主行带 mtime，out={out}");
    assert!(out.contains("未跟踪新件"), "?? 码标签，out={out}");
    assert!(out.contains("豁免面冻结登记 16 面"), "constants 冻结登记数，out={out}");

    // 直改链笔声明面覆盖 foo.txt → 净态
    fs::write(
        default_trail(&fx.root, AT),
        r#"{"event_type": "direct_edit_completed", "details": {"files": ["sih-tools/foo.txt"]}}
"#,
    )
    .unwrap();
    let (code, out, _) = run(&["check", "--at", AT, "--root", &r]);
    assert_eq!(code, 0, "声明笔覆盖后须净态，out={out}");
    assert!(out.contains("结论：净态，无主修改零处"));
    assert!(out.contains("声明 1 件"), "声明计数，out={out}");
}

// ---------- 拒绝形：非 git 仓、当日 trail 缺席（fail-closed） ----------

#[test]
fn t2_reject_forms() {
    let fx = build_fx();

    // root 下 sih-tools 非 git 仓：git status 失败 → 工具异常退出码 2
    let plain = tempfile::TempDir::new().unwrap();
    fs::create_dir_all(plain.path().join("sih-tools")).unwrap();
    fs::create_dir_all(plain.path().join("sih-engine")).unwrap();
    let (code, out, err) = run(&["check", "--at", AT, "--root", &plain.path().to_string_lossy()]);
    assert_eq!(code, 2, "非 git 仓 fail-closed 须 2，stderr={err}");
    assert!(out.is_empty());
    assert!(err.contains("稽 watchcheck 工具异常"), "stderr 报工具异常，err={err}");
    assert!(err.contains("git status 失败"), "stderr 报 git 失败，err={err}");

    // 当日 trail 缺席：挂点序保证 trail 先行的挂点语义，退出码 2
    let r = fx.root.to_string_lossy().into_owned();
    let (code, _, err) = run(&["check", "--at", "2026-09-01", "--root", &r]);
    assert_eq!(code, 2, "trail 缺席须 2，stderr={err}");
    assert!(err.contains("当日 trail 缺席"), "stderr 报缺席，err={err}");

    // 缺 --at / --root：用法错退出码 2
    let (code, _, _) = run(&["check", "--root", &r]);
    assert_eq!(code, 2, "缺 --at 须 2");
    let (code, _, _) = run(&["check", "--at", AT]);
    assert_eq!(code, 2, "缺 --root 须 2");
}

// ---------- 边界形：锁面覆盖、豁免面覆盖、已跟踪修改标签 ----------

#[test]
fn t3_edge_lockface_exemption_tracked_label() {
    let fx = build_fx();
    let st = fx.root.join("sih-tools");
    let en = fx.root.join("sih-engine");

    // 已跟踪件提交后修改 → porcelain " M" → 标签 已跟踪修改，未声明 → 无主
    fs::write(st.join("track.txt"), "one\n").unwrap();
    git(&st, &["add", "track.txt"]);
    git(&st, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-m", "init"]);
    fs::write(st.join("track.txt"), "two\n").unwrap();

    // 未跟踪件走锁面覆盖（默认锁台账路径）
    fs::create_dir_all(st.join("lease/ledger")).unwrap();
    fs::write(
        st.join("lease/ledger/locks.ndjson"),
        r#"{"event": "acquired", "path": "sih-tools/foo2.txt", "mode": "exclusive", "session_id": "s1"}
"#,
    )
    .unwrap();
    fs::write(st.join("foo2.txt"), "锁内活写\n").unwrap();

    // 声明面 trail 在豁免面路径内（自身不被判无主），声明 track.txt 之外的 nothing
    fs::create_dir_all(en.join("sih/event/trail")).unwrap();
    fs::write(
        en.join("sih/event/trail").join(format!("{AT}.ndjson")),
        r#"{"event_type": "direct_edit_completed", "details": {"files": []}}
"#,
    )
    .unwrap();

    let r = fx.root.to_string_lossy().into_owned();
    let (code, out, _) = run(&["check", "--at", AT, "--root", &r]);
    assert_eq!(code, 1, "track.txt 无主须 1，out={out}");
    assert!(out.contains("无主修改 1 件"), "锁内与豁免均被扣住，out={out}");
    assert!(out.contains("- sih-tools/track.txt | mtime "), "无主行，out={out}");
    assert!(out.contains("已跟踪修改"), " M 码标签，out={out}");
    assert!(out.contains("现势 1 面"), "锁面计数，out={out}");
    assert!(!out.contains("foo2.txt"), "锁面覆盖件不入无主清单，out={out}");
    assert!(!out.contains("- sih-tools/lease/ledger/locks.ndjson"), "豁免面件不入无主清单，out={out}");

    // 锁 release 后 foo2.txt 浮出无主
    let locks = st.join("lease/ledger/locks.ndjson");
    let mut text = fs::read_to_string(&locks).unwrap();
    text.push_str(r#"{"event": "released", "path": "sih-tools/foo2.txt", "session_id": "s1"}
"#);
    fs::write(&locks, text).unwrap();
    let (code, out, _) = run(&["check", "--at", AT, "--root", &r]);
    assert_eq!(code, 1);
    assert!(out.contains("无主修改 2 件"), "release 后两件无主，out={out}");
    assert!(out.contains("- sih-tools/foo2.txt | mtime "));
}
