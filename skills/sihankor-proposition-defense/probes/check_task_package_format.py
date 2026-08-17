"""task-packages 文档格式机械化校验（T6D-07 F-D 5 条）。

跑 `python3 check_task_package_format.py` 验 task-packages/*.md：

- F-D.1 一级标题 1 个
- F-D.2 二级及以上标题均带 {#anchor}
- F-D.3 必填节（§一-§七）齐
- F-D.4 em dash 0 命中
- F-D.5 三级标题 100% 带 {#anchor}

退出码：
- 0 = 全过
- 1 = 至少 1 条 FAIL
- 2 = 异常（无 task-packages 目录等）
"""
from __future__ import annotations

import re
import sys
from pathlib import Path


# task-packages 目录（相对仓库根）
TASK_PACKAGES_DIR = Path("task-packages")

# 必填节（一级中文数字 §一-§七）
REQUIRED_SECTIONS = [
    ("§一", r"^##\s+一、"),
    ("§二", r"^##\s+二、"),
    ("§三", r"^##\s+三、"),
    ("§四", r"^##\s+四、"),
    ("§五", r"^##\s+五、"),
    ("§六", r"^##\s+六、"),
    ("§七", r"^##\s+七、"),
]

# em dash（含 U+2014 em dash + U+2013 en dash）
EM_DASH_PATTERN = re.compile(r"[—–]")


def check_one(file_path: Path) -> dict:
    """对一个 task-packages 文档跑 F-D 5 条校验。

    返回 {f-d-id: {"status": "PASS"/"FAIL", "detail": str}}
    """
    results: dict = {}
    content = file_path.read_text()
    lines = content.splitlines()

    # F-D.1 一级标题 1 个
    h1_count = sum(1 for line in lines if re.match(r"^#\s+", line) and not line.startswith("##"))
    if h1_count == 1:
        results["F-D.1"] = {"status": "PASS", "detail": f"1 个一级标题"}
    else:
        results["F-D.1"] = {"status": "FAIL", "detail": f"{h1_count} 个一级标题（应 1 个）"}

    # F-D.2 二级及以上标题均带 {#anchor}
    headings_with_anchor = 0
    headings_without_anchor: list[str] = []
    for line in lines:
        m = re.match(r"^(#{2,})\s+", line)
        if m:
            if "{#" in line:
                headings_with_anchor += 1
            else:
                headings_without_anchor.append(line.strip())

    if not headings_without_anchor:
        results["F-D.2"] = {"status": "PASS", "detail": f"{headings_with_anchor} 个标题全带锚点"}
    else:
        results["F-D.2"] = {
            "status": "FAIL",
            "detail": f"{len(headings_without_anchor)} 个标题缺锚点: " + "; ".join(headings_without_anchor[:3]),
        }

    # F-D.3 必填节（§一-§七）齐
    missing_sections: list[str] = []
    for section_name, pattern in REQUIRED_SECTIONS:
        if not re.search(pattern, content, flags=re.MULTILINE):
            missing_sections.append(section_name)

    if not missing_sections:
        results["F-D.3"] = {"status": "PASS", "detail": "§一-§七 必填节齐"}
    else:
        results["F-D.3"] = {"status": "FAIL", "detail": f"缺: {', '.join(missing_sections)}"}

    # F-D.4 em dash 0 命中
    em_dash_count = len(EM_DASH_PATTERN.findall(content))
    if em_dash_count == 0:
        results["F-D.4"] = {"status": "PASS", "detail": "0 em dash"}
    else:
        results["F-D.4"] = {"status": "FAIL", "detail": f"{em_dash_count} 个 em dash 命中"}

    # F-D.5 三级标题 100% 带 {#anchor}
    h3_count = 0
    h3_with_anchor = 0
    h3_without: list[str] = []
    for line in lines:
        if re.match(r"^###\s+", line):
            h3_count += 1
            if "{#" in line:
                h3_with_anchor += 1
            else:
                h3_without.append(line.strip())

    if h3_count == 0:
        results["F-D.5"] = {"status": "PASS", "detail": "0 三级标题（豁免）"}
    elif h3_with_anchor == h3_count:
        results["F-D.5"] = {"status": "PASS", "detail": f"{h3_count} 个三级标题全带锚点"}
    else:
        results["F-D.5"] = {
            "status": "FAIL",
            "detail": f"{len(h3_without)}/{h3_count} 三级标题缺锚点: " + "; ".join(h3_without[:3]),
        }

    return results


def main() -> int:
    """主函数：扫 task-packages/*.md 跑 F-D 5 条，输出汇总。

    退出码 0 = 全过 / 1 = 至少 1 FAIL / 2 = 异常
    """
    if not TASK_PACKAGES_DIR.exists():
        print(f"❌ task-packages/ 目录不存在: {TASK_PACKAGES_DIR.resolve()}")
        return 2

    md_files = sorted(TASK_PACKAGES_DIR.glob("*.md"))
    if not md_files:
        print(f"❌ task-packages/ 目录无 .md 文件")
        return 2

    # 跳过 TEMPLATE（模板自身不算任务包）
    md_files = [f for f in md_files if f.name != "TEMPLATE.md"]
    # 跳过 README（目录说明文档）
    md_files = [f for f in md_files if f.name != "README.md"]

    print("=" * 72)
    print("task-packages 文档格式机械化校验（T6D-07 F-D 5 条）")
    print("=" * 72)
    print(f"扫描目录: {TASK_PACKAGES_DIR.resolve()}")
    print(f"扫到: {len(md_files)} 任务包文档（跳过 TEMPLATE.md + README.md）")
    print()

    total_pass = 0
    total_fail = 0
    failed_files: list[str] = []

    for f in md_files:
        print(f"### {f.name}")
        results = check_one(f)
        file_pass = 0
        file_fail = 0
        for fd_id, r in results.items():
            mark = "✓" if r["status"] == "PASS" else "✗"
            print(f"  {mark} {fd_id}: {r['status']} — {r['detail']}")
            if r["status"] == "PASS":
                file_pass += 1
            else:
                file_fail += 1
        if file_fail > 0:
            failed_files.append(f.name)
        total_pass += file_pass
        total_fail += file_fail
        print()

    print("=" * 72)
    print(f"汇总: {total_pass + total_fail} F 锚定 / {total_pass} pass / {total_fail} fail")
    if failed_files:
        print(f"❌ {len(failed_files)} 文件未全过 F-D 5 条")
        for fn in failed_files:
            print(f"   - {fn}")
        return 1
    else:
        print(f"✓ 全部 {len(md_files)} 任务包文档 F-D 5 条全过")
        return 0


if __name__ == "__main__":
    sys.exit(main())
