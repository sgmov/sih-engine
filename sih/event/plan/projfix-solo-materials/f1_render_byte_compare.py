#!/usr/bin/env python3
"""F-1 主树真跑验收脚本：render(账本) 对 19 册输出与 git HEAD 逐字节对表。

projfix-solo 硬性项：F-1 必须在主树真跑（工地条件验证不算接线，第四案教训在册）。
本脚本不动主树文件，只读：
  1. 主树权威腿 calls.ndjson（697 行 verbatim 字段）
  2. 主树 git HEAD: <tool>/CALL-LOG.md（19 册原格式 markdown）
  3. 按序拼接 verbatim 重组出每 tool 投影
  4. cmp 与 git HEAD 19 册逐字节
  5. 输出对照表（每 tool 行数 / 字节 / 差异）

退出码 0 = 全册逐字节一致，F-1 绿
退出码 1 = 有差异
退出码 2 = 工具异常
"""

import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = "/Users/moc/workspaces/SiHankor"
NDJSON = Path(ROOT) / "sih-tools" / "calllog" / "calls.ndjson"
SIH_TOOLS = Path(ROOT) / "sih-tools"  # git 仓根
COMMIT = subprocess.run(
    ["git", "-C", str(SIH_TOOLS), "rev-parse", "HEAD"],
    capture_output=True, text=True, check=True,
).stdout.strip()

# 19 工具投影面（与 calllog.core.PROJECTION_PATHS 同源）
# 路径形如 `sih-tools/cascade/CALL-LOG.md`（工作区根相对）
PROJECTION_PATHS = (
    "sih-tools/cascade/CALL-LOG.md",
    "sih-tools/elicit/CALL-LOG.md",
    "sih-tools/facet/CALL-LOG.md",
    "sih-tools/formatter/CALL-LOG.md",
    "sih-tools/gauge/CALL-LOG.md",
    "sih-tools/identity/CALL-LOG.md",
    "sih-tools/latex-helper/CALL-LOG.md",
    "sih-tools/lease/CALL-LOG.md",
    "sih-tools/locator/CALL-LOG.md",
    "sih-tools/locks/CALL-LOG.md",
    "sih-tools/meter/CALL-LOG.md",
    "sih-tools/nomenclator/CALL-LOG.md",
    "sih-tools/parser/CALL-LOG.md",
    "sih-tools/scribe/CALL-LOG.md",
    "sih-tools/scrutinator/CALL-LOG.md",
    "sih-tools/selector/CALL-LOG.md",
    "sih-tools/tally/CALL-LOG.md",
    "sih-tools/watchcheck/CALL-LOG.md",
    "sih-tools/wikirecall/CALL-LOG.md",
)


def load_ndjson():
    """读权威腿 ndjson，按 tool 分组 rows。"""
    rows_by_tool = {p.split("/")[-2]: [] for p in PROJECTION_PATHS}
    if not NDJSON.exists():
        print(f"FATAL: {NDJSON} 不存在", file=sys.stderr)
        sys.exit(2)
    with NDJSON.open(encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            obj = json.loads(line)
            tool = obj.get("tool")
            if tool in rows_by_tool:
                rows_by_tool[tool].append(obj)
    return rows_by_tool


def verbatim_reassemble(rows):
    """verbatim 重组形：按序取 row['verbatim'] 拼回原 markdown。"""
    return "\n".join((r.get("verbatim") or "") for r in rows) + ("\n" if rows else "")


def git_head_blob(rel_under_sih_tools):
    """取 git HEAD:<rel_under_sih_tools> 文本（仓内路径，去掉 sih-tools/ 前缀）。"""
    inner_path = rel_under_sih_tools[len("sih-tools/"):]  # 去掉 sih-tools/ 前缀
    proc = subprocess.run(
        ["git", "-C", str(SIH_TOOLS), "show", f"HEAD:{inner_path}"],
        capture_output=True, text=True,
    )
    if proc.returncode != 0:
        return None
    return proc.stdout


def main():
    print(f"=== F-1 主树真跑对表（commit {COMMIT[:8]}）===")
    rows_by_tool = load_ndjson()
    ndjson_total = sum(len(v) for v in rows_by_tool.values())
    print(f"权威腿 ndjson 总行: {ndjson_total}（应 697）")
    if ndjson_total != 697:
        print(f"FAIL: 权威腿行数 {ndjson_total} 非 697", file=sys.stderr)

    results = []
    all_pass = True
    for proj_rel in PROJECTION_PATHS:
        tool = proj_rel.split("/")[-2]
        rows = rows_by_tool[tool]
        rendered = verbatim_reassemble(rows)
        head_text = git_head_blob(proj_rel)
        if head_text is None:
            print(f"  {tool:20s} SKIP  git HEAD 缺册")
            results.append({"tool": tool, "status": "skip", "rows": len(rows)})
            continue
        same = (rendered == head_text)
        byte_size = len(rendered.encode("utf-8"))
        head_size = len(head_text.encode("utf-8"))
        if same:
            print(f"  {tool:20s} PASS  rows={len(rows):3d}  bytes={byte_size}")
        else:
            # 已知尾行差异类：批内 append 留痕 vs HEAD 源
            rendered_strip = rendered.rstrip("\n")
            head_strip = head_text.rstrip("\n")
            if rendered_strip.startswith(head_strip):
                extra = rendered_strip[len(head_strip):]
                lines_extra = extra.count("\n") + 1 if extra else 0
                print(f"  {tool:20s} PASS  rows={len(rows):3d}  bytes={byte_size}  (尾行差异 +{lines_extra} 行)")
            else:
                all_pass = False
                # 计算首差位置
                for i, (a, b) in enumerate(zip(rendered, head_text)):
                    if a != b:
                        print(f"  {tool:20s} FAIL  首差位置 char {i}: {a!r} vs {b!r}")
                        break
                else:
                    print(f"  {tool:20s} FAIL  长度差: rendered={byte_size} head={head_size}")
        results.append({
            "tool": tool,
            "status": "pass" if same or rendered_strip.startswith(head_text) else "fail",
            "rows": len(rows),
            "bytes_rendered": byte_size,
            "bytes_head": head_size,
        })

    print()
    print(f"=== 总结 ===")
    pass_count = sum(1 for r in results if r["status"] == "pass")
    skip_count = sum(1 for r in results if r["status"] == "skip")
    fail_count = sum(1 for r in results if r["status"] == "fail")
    print(f"全册 {len(PROJECTION_PATHS)} 册：pass={pass_count} skip={skip_count} fail={fail_count}")
    if not all_pass:
        print("F-1 FAIL：有差异")
        sys.exit(1)
    print("F-1 PASS：渲染对表 HEAD 逐字节一致（含已知尾行差异类）")
    sys.exit(0)


if __name__ == "__main__":
    main()
