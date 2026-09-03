#!/usr/bin/env python3
# mathrefmt-solo 确定性转换脚本
# 职责：将 calculus 条目旧格式「## 概览 {#overview}」（纯目录导航块）归一为标准块序首块「## 定义」。
# 方法：移除概览导航块（六项目录均为自指锚点，目标节全部原位保留→零弃段），
#       将紧随的话题节改为「## 定义 {#definition}」（其首段即定义引入，正文零改动）。
# 确定性：同输入同输出，可复算。工程基线第一条：确定性程序是治理操作唯一执行者。
import re
import sys
import os

OVERVIEW_HEADER = re.compile(r'^##\s*概览\s*\{#overview\}\s*$')
TOPIC_HEADER = re.compile(r'^##\s*话题\s*\{#topic\}\s*$')
ANY_HEADER = re.compile(r'^##\s')

def transform(text):
    lines = text.splitlines(keepends=True)
    out = []
    i = 0
    in_overview = False
    encountered_topic = False
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        if OVERVIEW_HEADER.search(stripped):
            # 跳过概览头；后续删除概览导航行直到下一个 ## 头
            i += 1
            while i < len(lines):
                if ANY_HEADER.match(lines[i]):
                    break
                i += 1
            continue
        if TOPIC_HEADER.search(stripped):
            encountered_topic = True
            # 改写话题节头为定义节头（正文保持原位）
            out.append("## 定义 {#definition}\n")
            i += 1
            continue
        out.append(line)
        i += 1
    new_text = "".join(out)
    return new_text, encountered_topic

def main():
    if len(sys.argv) < 2:
        print("usage: refmt.py <file.md> [--check]")
        return 2
    path = sys.argv[1]
    text = open(path, encoding="utf-8").read()
    new_text, hit_topic = transform(text)
    if not hit_topic:
        print(f"WARN: {path} 无话题节头，估算异常")
    if new_text == text:
        print(f"UNCHANGED: {path}")
        return 0
    if "--check" in sys.argv:
        # check：只报告是否需改
        print(f"WOULD_CHANGE: {path}")
        return 1 if new_text != text else 0
    open(path, "w", encoding="utf-8").write(new_text)
    print(f"CHANGED: {path}")
    return 0

if __name__ == "__main__":
    sys.exit(main())