# 盘点档：formatter（化格）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）

## 机制职责一句话

包驱动确定性格式归一：按 pack 声明顺序对目标文本应用行操作与 JSON 正典化操作，未知操作种类即拒绝（formatter/src/formatter/format.py:54-65 apply_ops）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 未知操作拒绝 | formatter/src/formatter/format.py:65 `raise FormatError(f"未知操作 kind")` | 包外操作种类即工具异常 | 判定性 |
| 操作序应用 | formatter/src/formatter/format.py:57 `for pack_name, op in ops` | 按包声明顺序逐操作改写，序决定结果 | 判定性 |
| 退出码枚举 | 化格退出码 0=无需改 1=已修改 2=工具异常（BATCH-FACE.md §7） | 幂等判读：重跑落 0 即不动点 | 判定性 |
| fence_aware 开关 | formatter/src/formatter/format.py:13 apply_line_ops 参数 | 代码围栏感知的行操作域 | 判定性 |

## 既有归属判

判定性。改写与否直接决定档面合规位；rev1 判 decision_face 空是指无命令级判定子命令，改写判据本体在包数据与 apply 序，仍属判定语义。

## 载体候选分析

- 改写幂等语义：formatter --write 重跑落 0 即不动点，这是确定性重写系统的合流与幂等性质。
- 数学仓已建条目对照：mapping.md 全表无重写系统/合流性/幂等性条目，零命中如实记。
- 需新数学条目（重写系统确定性幂等），无哲学新桥（工程实证语义，不涉哲学命题映射）→ 载体管线批次输入。
- 源码概念引用位实测：sih-tools/formatter/src 零命中（本批 grep 实查 2026-09-04）。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 formatter；ledger-rev1.json carrier_matching formatter 条目 source_math_ids 空与 face_refs 三面空。
- 调用面：BATCH-FACE.md §7 管线三步第一步化格（笔在核前）。
