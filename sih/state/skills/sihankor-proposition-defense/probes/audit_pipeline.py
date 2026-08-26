"""治理工程审阅流水线（sihankor-proposition-defense 内嵌脚本，零 LLM 调用）。

承接任务包 T6D-01 子任务 B2：机械层全过 → 跳过 LLM 审 / 机械层失败
→ 报告漏层清单（默认不调 LLM，由用户决定）。

设计原则：
- **零 LLM 调用**：subprocess 跑 5 机械脚本
- **流水线化**：跑全部 5 脚本，输出综合报告
- **可选 LLM 触发**：默认不调，--with-llm-review 显式启用（CLI 占位，未来由用户接 LLM）
- **跨工具一致**：sih-engine / .agents/skills 双处都能跑
- **可证伪**：F7.x 机械判定
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

# 5 个机械脚本路径（默认 sih-engine/sih/state/skills/.../probes/）
_PROBES_DIR = Path(__file__).resolve().parent

SCRIPTS = {
    "A1_layer_proportion": "check_layer_proportion.py",
    "A2_yaml_factual": "check_yaml_factual_consistency.py",
    "A3_test_intent": "check_test_coverage_intent.py",
    "A4_verdict_consistency": "check_verdict_consistency.py",
    "A5_decision_authority": "check_decision_authority.py",
}

# 跨工具 hardlink 目录（与 .agents/skills/.../probes/ 同步）
_CROSS_TOOL = Path("/Users/moc/workspaces/SiHankor/.agents/skills/sihankor-proposition-defense/probes")


@dataclass
class ScriptResult:
    """单脚本运行结果。"""
    name: str
    exit_code: int
    stdout: str
    stderr: str
    duration_s: float
    verdict: str = "UNKNOWN"  # PASS / FAIL / ERROR


@dataclass
class PipelineReport:
    """流水线综合报告。"""
    trail_root: str
    scripts_run: list[ScriptResult] = field(default_factory=list)
    all_pass: bool = False
    needs_llm_review: bool = False
    summary: dict = field(default_factory=dict)

    def to_dict(self) -> dict:
        return {
            "trail_root": self.trail_root,
            "scripts_run": [
                {
                    "name": r.name,
                    "exit_code": r.exit_code,
                    "verdict": r.verdict,
                    "duration_s": round(r.duration_s, 2),
                    "stderr": r.stderr[:200] if r.stderr else "",
                }
                for r in self.scripts_run
            ],
            "all_pass": self.all_pass,
            "needs_llm_review": self.needs_llm_review,
            "summary": self.summary,
        }


# ---------- 跑单脚本 ----------

def _run_script(name: str, script_path: Path, args: list[str]) -> ScriptResult:
    """subprocess 跑单脚本，返回 ScriptResult。"""
    import time
    cmd = [sys.executable, str(script_path)] + args
    t0 = time.time()
    try:
        proc = subprocess.run(
            cmd, capture_output=True, text=True, timeout=60,
        )
        duration = time.time() - t0
        # 退出码语义：0=PASS, 1=FAIL, 2=ERROR
        if proc.returncode == 0:
            verdict = "PASS"
        elif proc.returncode == 1:
            verdict = "FAIL"
        else:
            verdict = "ERROR"
        return ScriptResult(
            name=name, exit_code=proc.returncode,
            stdout=proc.stdout, stderr=proc.stderr,
            duration_s=duration, verdict=verdict,
        )
    except subprocess.TimeoutExpired:
        return ScriptResult(
            name=name, exit_code=124, stdout="", stderr="timeout after 60s",
            duration_s=60.0, verdict="ERROR",
        )
    except Exception as e:
        return ScriptResult(
            name=name, exit_code=1, stdout="", stderr=str(e),
            duration_s=0.0, verdict="ERROR",
        )


# ---------- 核心流水线 ----------

def run_pipeline(trail_root: str, with_json: bool = True) -> PipelineReport:
    """跑 5 机械脚本流水线（无 LLM 调用）。

    各子脚本用各自默认路径（A1 默认 trail_root, A2 默认 yaml/code_root, 等）。
    B2 接收的 trail_root 只用于报告标识与 A1 子脚本的显式覆盖。
    """
    import time
    # 各子脚本参数矩阵（按子脚本期望的 CLI 形式）
    SCRIPT_ARGS = {
        # A1: check_layer_proportion.py --trail-root PATH [--json] [--threshold N]
        "A1_layer_proportion": (
            ["--json", "--trail-root", trail_root] if with_json
            else ["--trail-root", trail_root]
        ),
        # A2: check_yaml_factual_consistency.py [--json]
        "A2_yaml_factual": ["--json"] if with_json else [],
        # A3: check_test_coverage_intent.py [--json]
        "A3_test_intent": ["--json"] if with_json else [],
        # A4: check_verdict_consistency.py [--json]
        "A4_verdict_consistency": ["--json"] if with_json else [],
        # A5: check_decision_authority.py [--json]
        "A5_decision_authority": ["--json"] if with_json else [],
    }

    report = PipelineReport(trail_root=trail_root)
    t0 = time.time()
    for key, script_name in SCRIPTS.items():
        script_path = _PROBES_DIR / script_name
        if not script_path.exists():
            alt = _CROSS_TOOL / script_name
            if alt.exists():
                script_path = alt
            else:
                report.scripts_run.append(ScriptResult(
                    name=key, exit_code=2, stdout="", stderr=f"not found: {script_path}",
                    duration_s=0.0, verdict="ERROR",
                ))
                continue
        args = SCRIPT_ARGS.get(key, [])
        result = _run_script(key, script_path, args)
        report.scripts_run.append(result)

    total_duration = time.time() - t0
    n_pass = sum(1 for r in report.scripts_run if r.verdict == "PASS")
    n_fail = sum(1 for r in report.scripts_run if r.verdict == "FAIL")
    n_error = sum(1 for r in report.scripts_run if r.verdict == "ERROR")

    report.all_pass = (n_fail == 0 and n_error == 0)
    report.needs_llm_review = (n_fail > 0 or n_error > 0)
    report.summary = {
        "total": len(report.scripts_run),
        "pass": n_pass,
        "fail": n_fail,
        "error": n_error,
        "duration_s": round(total_duration, 2),
        "llm_calls": 0,
    }
    return report


# ---------- 呈现 ----------

def _print_table(report: PipelineReport) -> None:
    print("=" * 64)
    print("治理工程审阅流水线（sihankor-proposition-defense，零 LLM）")
    print("=" * 64)
    print(f"trail root: {report.trail_root}")
    print()
    print(f"{'脚本':<28} {'退出码':<8} {'verdict':<8} {'耗时(s)':<10}")
    print(f"{'-'*28} {'-'*8} {'-'*8} {'-'*10}")
    for r in report.scripts_run:
        print(f"{r.name:<28} {r.exit_code:<8} {r.verdict:<8} {r.duration_s:<10.2f}")
    print()
    s = report.summary
    print(f"汇总: {s['total']} 脚本 / {s['pass']} pass / {s['fail']} fail / {s['error']} error")
    print(f"总耗时: {s['duration_s']}s")
    print(f"LLM 调用: {s['llm_calls']}（机械检查不调 LLM）")
    print()
    if report.all_pass:
        print("✅ 机械层全过 — 无需 LLM 真审")
    else:
        print(f"❌ 机械层 {s['fail'] + s['error']} 失败 — 见各脚本输出")
        print("   → 默认不调 LLM（用户决定）")
    print()
    print("=" * 64)


# ---------- CLI ----------

def main() -> int:
    p = argparse.ArgumentParser(description="治理工程审阅流水线（T6D-01 B2，零 LLM）")
    p.add_argument("--trail-root",
                    default="/Users/moc/workspaces/SiHankor/sih-tools/proposition/DES",
                    help="trail 根目录")
    p.add_argument("--json", action="store_true", help="仅输出 JSON")
    args = p.parse_args()

    report = run_pipeline(args.trail_root)
    if args.json:
        print(json.dumps(report.to_dict(), ensure_ascii=False, indent=2))
    else:
        _print_table(report)

    # 退出码：0=全过, 1=有失败（仍不调 LLM, 由用户决定）
    return 0 if report.all_pass else 1


if __name__ == "__main__":
    raise SystemExit(main())
