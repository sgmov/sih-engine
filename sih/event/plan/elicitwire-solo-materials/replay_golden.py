"""elicitwire-solo 金向量重放：两场景 elicit digest 双跑逐字节一致加冻结金向量对表。

用法：python3 replay_golden.py <elicit src 目录绝对路径> [--freeze]
场景一 digest_passed_covered：全处置过闸，digest passed covered 2，退出码零。
场景二 digest_blocked_orphan：未消化拦截，digest blocked missing 一枚，退出码一。
判据：同夹具同参双跑 stdout 逐字节一致（digest 确定性），且与冻结金向量
（归一化夹具根路径后）逐字节一致、退出码一致。

重放寻径约定（V6 教训）：夹具路径一律相对本脚本所在目录（MATERIALS）解析，
零 cwd 依赖零工地绝对路径冻结；stdout 经 <MATERIALS> 归一化后对表。
--freeze 由实跑程序化冻结金向量，禁手打输出。
"""
import json
import subprocess
import sys
from pathlib import Path

MATERIALS = Path(__file__).resolve().parent
CASES = [("digest_passed_covered", MATERIALS / "golden-pass"),
         ("digest_blocked_orphan", MATERIALS / "golden-block")]
FROZEN = MATERIALS / "elicitwire-solo-golden-vector.json"


def run_once(elicit_src: Path, signals: Path, contract: Path):
    code = ("import sys; sys.path.insert(0, %r); "
            "from elicit.cli import main; sys.exit(main(['digest', '--signals', %r, "
            "'--contract', %r]))"
            % (str(elicit_src), str(signals), str(contract)))
    p = subprocess.run([sys.executable, "-c", code], capture_output=True)
    return p.returncode, p.stdout


def normalize(text: str) -> str:
    return text.replace(str(MATERIALS), "<MATERIALS>")


def main() -> int:
    elicit_src = Path(sys.argv[1])
    freeze = "--freeze" in sys.argv
    if freeze:
        cases = []
        for name, dirpath in CASES:
            rc, out = run_once(elicit_src, dirpath / "signals.ndjson",
                               dirpath / "contract.md")
            cases.append({"case": name, "exit": rc,
                          "stdout_normalized": normalize(out.decode("utf-8"))})
        vector = {"batch": "elicitwire-solo", "carrier": ["ORD-008"],
                  "cases": cases, "date": "2026-09-04", "kind": "golden_vector",
                  "replay_convention": "python3 replay_golden.py <elicit src 绝对路径>；"
                                       "夹具路径经 MATERIALS=本脚本所在目录相对解析，"
                                       "零 cwd 依赖零绝对路径冻结；stdout 归一化 "
                                       "<MATERIALS> 后逐字节比对，退出码逐案比对"}
        FROZEN.write_text(json.dumps(vector, ensure_ascii=False, indent=2) + "\n",
                          encoding="utf-8")
        print(f"frozen {FROZEN}")
        return 0
    frozen = json.loads(FROZEN.read_text(encoding="utf-8"))
    results = {"carrier": ["ORD-008"], "cases": []}
    all_ok = True
    for name, dirpath in CASES:
        signals = dirpath / "signals.ndjson"
        contract = dirpath / "contract.md"
        rc1, out1 = run_once(elicit_src, signals, contract)
        rc2, out2 = run_once(elicit_src, signals, contract)
        double_identical = out1 == out2
        frozen_case = next(c for c in frozen["cases"] if c["case"] == name)
        golden_match = (normalize(out1.decode("utf-8")) == frozen_case["stdout_normalized"]
                        and rc1 == frozen_case["exit"])
        case_ok = double_identical and golden_match
        all_ok = all_ok and case_ok
        results["cases"].append({
            "case": name, "exit": rc1,
            "stdout_normalized": normalize(out1.decode("utf-8")),
            "double_run_byte_identical": double_identical,
            "golden_match": golden_match,
        })
    results["double_run_all_identical"] = all_ok
    print(json.dumps(results, ensure_ascii=False, sort_keys=True, indent=2))
    return 0 if all_ok else 1


if __name__ == "__main__":
    sys.exit(main())
