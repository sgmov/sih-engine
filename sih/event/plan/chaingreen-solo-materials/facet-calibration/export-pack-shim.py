"""chaingreen-solo 批材料工具：退役 temp_probe 的 atom.yaml 寻径垫片。

退役脚本 _THIS_DIR.parent / "atom.yaml" 预期 facet/probes/atom.yaml，实件在
facet/atom.yaml（baseinject-solo 退役搬迁断裂，facefit-solo 先例以 PYTHONPATH
补齐同款如实记）。本垫片零改退役脚本，仅运行期改 _THIS_DIR 数据寻径，
LEDGER 维持原值（retired/calibration/ledger.jsonl）不动。
"""
import pathlib
import sys

WT = pathlib.Path("/Users/moc/workspaces/SiHankor/worktrees/sih-tools/chaingreen-solo")
sys.path.insert(0, str(WT / "facet" / "probes" / "retired"))
sys.path.insert(0, str(WT / "facet" / "probes"))
sys.path.insert(0, str(WT / "facet" / "src"))

import temp_probe  # noqa: E402

# 数据寻径垫片：atom.yaml 实件在 facet/atom.yaml
temp_probe._THIS_DIR = WT / "facet" / "probes"

if __name__ == "__main__":
    out = sys.argv[1]
    temp_probe.export_pack(out)
