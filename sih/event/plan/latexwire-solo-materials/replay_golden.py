# latexwire-solo 金向量机械重放：compute 代表用例两场景双跑。
# 场景一 convergence_carriers：极限 lim(x→∞)(1−1/x)=1 与无穷级数 sum(1/2^k)=1
#   两收敛判定位机械重放（推导档 §3.1 与 §3.3，LIM-007 与 SER-001）。
# 场景二 accumulation_and_matrix_carriers：定积分 ∫[0,1]x²dx=1/3 与
#   行列式 det([[1,2],[3,4]])=−2 两判定位机械重放（推导档 §3.2 与 §3.4，
#   INT-007 与 ALG-001）。
# 用法（emit 模式，须在 latex-helper 项目环境下跑即 sympy 在役）：
#   cd sih-tools/latex-helper && env -u PYTHONHOME -u PYTHONPATH \
#     uv run --project . python <materials>/replay_golden.py <materials>/golden_cases.json \
#     --tools-src sih-tools/latex-helper/src > payload.json
# 用法（verify 模式）：
#   ... 同上 ... replay_golden.py golden_cases.json latexwire-solo-golden-vector.json --tools-src sih-tools/latex-helper/src
# 重放寻径约定（V6 教训，禁工地绝对路径冻结）：--tools-src 显式传参优先
# （相对路径按重放 cwd 解析），缺省按本件自身位置相对解析
# ../../../../../sih-tools/latex-helper/src（引擎仓 materials 目录五级上跳至
# 工作区根再入 sih-tools），已提交树任意检出位可复现；
# 运行环境约定为 sih-tools/latex-helper 项目环境（uv run --project），系统裸 python 无 sympy 不作重放环境。
# 判据：同参双跑逐字节一致（承 SPEC-015 双跑同参形条款），且与冻结金向量一致。
import argparse
import hashlib
import json
import sys
from pathlib import Path


def default_tools_src():
    # materials 目录 → plan → event → sih → 引擎仓根 → 工作区根，再入 sih-tools
    return (Path(__file__).resolve().parent / ".." / ".." / ".." / ".." / ".." / "sih-tools" / "latex-helper" / "src").resolve()


def run_case(compute, case):
    result = compute(case["operation"], case["expression"], **case["kwargs"])
    return {k: result.get(k) for k in sorted(result)}


def build_payload(cases_path, tools_src):
    sys.path.insert(0, str(tools_src))
    from latex_helper.compute import compute

    cases = json.loads(Path(cases_path).read_text(encoding="utf-8"))
    runs = {name: run_case(compute, case) for name, case in cases["cases"].items()}
    exp = cases["expectations"]
    s1_ok = (
        runs["limit"]["result_value"] == exp["scenario_1_convergence_carriers"]["limit_result_value"]
        and runs["sum"]["result_value"] == exp["scenario_1_convergence_carriers"]["sum_result_value"]
    )
    s2_ok = (
        runs["integral"]["result_value"] == exp["scenario_2_accumulation_and_matrix_carriers"]["integral_result_value"]
        and runs["matrix_det"]["result_value"] == exp["scenario_2_accumulation_and_matrix_carriers"]["matrix_det_result_value"]
    )
    payload = {
        "carrier": cases["carrier"],
        "formula_version": cases["formula_version"],
        "runs": runs,
        "scenario_1_convergence_carriers": {
            "limit_result_value": runs["limit"]["result_value"],
            "sum_result_value": runs["sum"]["result_value"],
            "limit_is_number": runs["limit"]["is_number"],
            "sum_is_number": runs["sum"]["is_number"],
            "expect": exp["scenario_1_convergence_carriers"],
            "scenario_ok": s1_ok,
        },
        "scenario_2_accumulation_and_matrix_carriers": {
            "integral_result_value": runs["integral"]["result_value"],
            "matrix_det_result_value": runs["matrix_det"]["result_value"],
            "integral_is_number": runs["integral"]["is_number"],
            "expect": exp["scenario_2_accumulation_and_matrix_carriers"],
            "scenario_ok": s2_ok,
        },
        "scenarios_ok": s1_ok and s2_ok,
    }
    return payload


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cases")
    ap.add_argument("frozen", nargs="?")
    ap.add_argument("--tools-src", default=None)
    args = ap.parse_args()
    tools_src = Path(args.tools_src).resolve() if args.tools_src else default_tools_src()
    payload = build_payload(args.cases, tools_src)
    out = json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=1) + "\n"
    sys.stdout.write(out)
    if args.frozen:
        frozen = Path(args.frozen).read_text(encoding="utf-8")
        same = out == frozen
        print(f"replay_vs_frozen: {'IDENTICAL' if same else 'DIVERGENT'}", file=sys.stderr)
        sys.exit(0 if same else 1)


if __name__ == "__main__":
    main()
