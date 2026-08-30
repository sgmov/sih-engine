#!/usr/bin/env python3
"""c006-sb 短注类改法脚本

模板: X（Y） → X，Y
- 删左全角括号「（」
- 删右全角括号「）」
- 前面是中文字符则前面加「，」
- 后面是中文字符则后面加「，」

跳过：9 处留人工（即称类 6 + 描述同位 3）

执行范围: sih-math/calculus/llm-friendly-build/entries/*.md
"""
import re
import sys
from pathlib import Path

# 9 处留人工清单（file:line 形式）
SKIP_LIST = {
    ("MUL-003-gabriels-horn.md", 18),
    ("MUL-001-partial-derivative.md", 26),
    ("MUL-008-divergence-theorem.md", 18),
    ("INT-021-solid-of-revolution.md", 20),
    ("DIFF-030-regiomontanus-angle-maximization.md", 18),
    ("INT-008-integration-by-parts.md", 18),
    # 3 处描述同位
    ("INT-016-proof-that-22-over-7-exceeds-pi.md", 18),
    ("LIM-003-one-sided-limit.md", 24),
    ("LIM-003-one-sided-limit.md", 26),
}

# 中文范围（CJK Unified Ideographs 基本区 + 扩展 A 区 + 符号）
CN_PATTERN = re.compile(r'[一-鿿　-〿＀-｠]')

def is_cn(ch):
    if not ch:
        return False
    return bool(CN_PATTERN.match(ch))

def transform_line(line):
    """单行变换：X（Y）→ X，Y

    规则（本批仅改长描述，括号内 ≥ 6 字符且为补充说明型）:
    - 仅处理"长描述"括号（括号内长度 ≥ 6 字符）
    - 短补注（≤5 字符如"y 轴"、公式内"（周长）"）跳过
    - 删左全角括号「（」和右全角括号「）」及中间内容
    - 紧跟括号后的标点（，。；：、）一并删除（避免双标点）
    - 前面是中文字符（不是逗号/句号/分号/冒号/引号）→ 前面加「，」
    """
    if '（' not in line and '）' not in line:
        return line, 0

    PUNCT_AFTER = set('，。；：、')
    PUNCT_BEFORE = set('，。；：、！？」』】）》')
    MIN_INNER = 6  # 短补注阈值：括号内字符数 < 6 跳过

    count = 0
    result = []
    i = 0
    while i < len(line):
        ch = line[i]
        if ch == '（':
            j = line.find('）', i)
            if j == -1:
                result.append(ch)
                i += 1
                continue
            inner = line[i+1:j]
            # 跳过短补注
            if len(inner) < MIN_INNER:
                result.append(ch)
                i += 1
                continue
            prev = line[i-1] if i > 0 else ''
            # 前面是中文且不是标点 → 加逗号
            if is_cn(prev) and prev not in PUNCT_BEFORE and (not result or result[-1] not in PUNCT_BEFORE):
                result.append('，')
            i = j + 1
            # 跳过紧跟的标点
            while i < len(line) and line[i] in PUNCT_AFTER:
                i += 1
            count += 1
        else:
            result.append(ch)
            i += 1
    return ''.join(result), count

def main():
    if len(sys.argv) < 2:
        print("用法: transform.py <md_file> [<md_file> ...]", file=sys.stderr)
        sys.exit(2)

    total_changes = 0
    files_changed = 0
    for f in sys.argv[1:]:
        path = Path(f)
        if not path.exists():
            print(f"SKIP 不存在: {f}", file=sys.stderr)
            continue
        with path.open('r', encoding='utf-8') as fp:
            lines = fp.readlines()
        new_lines = []
        file_changes = 0
        for ln_no, line in enumerate(lines, start=1):
            if (path.name, ln_no) in SKIP_LIST:
                new_lines.append(line)
                continue
            new_line, n = transform_line(line)
            new_lines.append(new_line)
            file_changes += n
        if file_changes > 0:
            with path.open('w', encoding='utf-8') as fp:
                fp.writelines(new_lines)
            files_changed += 1
            total_changes += file_changes
            print(f"OK {path.name}: {file_changes} 处改", file=sys.stderr)
    print(f"---", file=sys.stderr)
    print(f"总文件: {files_changed}", file=sys.stderr)
    print(f"总改数: {total_changes}", file=sys.stderr)

if __name__ == '__main__':
    main()
