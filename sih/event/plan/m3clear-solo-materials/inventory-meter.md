# 盘点档：meter（计量）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）

## 机制职责一句话

调用计量载体：逐次包裹调用追加一记录（时间戳、命令、工具归因）到 counts 按日 ndjson，count 面出读数，crosscheck 面对书简 trail 漏计交叉核对（meter/src/meter/cli.py:3 模块docstring 与 :130）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 逐调用一记录 | meter/src/meter/cli.py:41-43 `_append_record` 按日文件追加一行 | 计数语义：一次包裹调用恰一记录 | 判定性 |
| 工具归因 | meter/src/meter/cli.py:34 `_attribute_tool(argv)` | 调用串到被包裹工具的归因判定 | 判定性 |
| 漏计交叉核对 | meter/src/meter/cli.py:130 `cmd_crosscheck` 对 trail 事件对表 | counts 记录对链事件的差集判定 | 判定性 |
| 按日分册 | meter/src/meter/cli.py:43 `f"{record['ts'][:10]}.ndjson"` | 记录按 UTC 日期分册 | 资源性（存储分册形，不改判定） |

## 既有归属判

判定性为主。计数与归因与对账直接产出治理读数（LLM 调用计量是注意力预算的读数源），按日分册属资源性存储形。

## 载体候选分析

- 计数语义：逐调用恰一记录的追加计数，日计数即逐记录累加，计数对账即两序列的差集核对。
- 数学仓已建条目对照：mapping.md 全表无计数测度/可加计数条目，零命中如实记（PROB-006 概率测度是归一化测度，计数测度是其非归一化亲属但条目本身不覆盖，不硬挂）。
- 需新数学条目（计数测度可加性或自由幺半群上的计数），无哲学新桥（工程计量语义，不涉哲学命题映射）→ 载体管线批次输入。
- 源码概念引用位实测：sih-tools/meter/src 零命中（本批 grep 实查 2026-09-04）。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 meter；ledger-rev1.json carrier_matching meter 条目 source_math_ids 空与 face_refs 三面空。
- 职能位：AGENTS.md § 工具层静态审计（meter 包裹引擎 scribe 是链写入的强制包裹位）。
