"""tallywire2-solo 金向量重放：两场景 tally check 双跑逐字节一致加冻结金向量对表。

用法：python3 replay_golden.py <tally src 目录绝对路径>
场景一 pass_all_closure：全过材料核对闭合，处置裁决通过。
场景二 suspend_r5_drift：拒材料 R5 条目挂起定位，处置挂起定位行 R5。
判据：同夹具同参双跑 stdout 逐字节一致（闭包幂等），且与冻结金向量
（归一化 fixtures 根路径后）逐字节一致、退出码一致（归约确定）。
"""
import json
import subprocess
import sys
from pathlib import Path

MATERIALS = Path(__file__).resolve().parent
CASES = [("pass_all_closure", MATERIALS / "golden-pass"),
         ("suspend_r5_drift", MATERIALS / "golden-suspend")]
FROZEN = MATERIALS / "tallywire2-solo-golden-vector.json"


def run_once(tally_src: Path, material: Path):
    code = ("import sys; sys.path.insert(0, %r); "
            "from tally.cli import main; sys.exit(main(['check', '--material', %r]))"
            % (str(tally_src), str(material)))
    p = subprocess.run([sys.executable, "-c", code], capture_output=True)
    return p.returncode, p.stdout


def normalize(text: str) -> str:
    return text.replace(str(MATERIALS), "<MATERIALS>")


def main() -> int:
    tally_src = Path(sys.argv[1])
    frozen = json.loads(FROZEN.read_text(encoding="utf-8"))
    results = {"carrier": ["ORD-006", "ORD-011"], "cases": []}
    all_ok = True
    for name, dirpath in CASES:
        material = dirpath / "material.json"
        rc1, out1 = run_once(tally_src, material)
        rc2, out2 = run_once(tally_src, material)
        double_identical = out1 == out2
        frozen_case = next(c for c in frozen["cases"] if c["case"] == name)
        golden_match = (normalize(out1.decode("utf-8")) == frozen_case["stdout_normalized"]
                        and rc1 == frozen_case["exit"])
        case_ok = double_identical and golden_match
        all_ok = all_ok and case_ok
        results["cases"].append({
            "case": name, "exit": rc1,
            "disposition": json.loads(out1.decode("utf-8"))["disposition"],
            "double_run_byte_identical": double_identical,
            "golden_match": golden_match,
        })
    results["double_run_all_identical"] = all_ok
    print(json.dumps(results, ensure_ascii=False, sort_keys=True, indent=2))
    return 0 if all_ok else 1


if __name__ == "__main__":
    sys.exit(main())
