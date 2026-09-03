# 盘点档：nomenclator（检词）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）

## 机制职责一句话

术语三态登记与文档核查：登记面 terms.json 三态词册，check 面对目标文本做已登记词的词面跨度匹配出违例，query 面查词条（nomenclator/src/nomenclator/matching.py 匹配核）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| CJK/ASCII 词判定 | nomenclator/src/nomenclator/matching.py:6-10 `is_cjk` / `is_ascii_word` | 字符类别判定决定词跨度切分 | 判定性 |
| 词跨度迭代 | nomenclator/src/nomenclator/matching.py:14 `iter_spans(text, word)` | 已登记词在文本中的出现跨度枚举 | 判定性 |
| 摘录截断常数 | nomenclator/src/nomenclator/matching.py:49 `if len(excerpt) > 60` | 违例摘录超 60 字符截断 | 资源性（输出降噪形，不改违例判定本身） |
| 三态登记面 | nomenclator/packs/core/terms.json（established 等状态字段） | 术语三态（在册/候审/退役）判定基准集 | 判定性（登记面数据即判定基准） |

## 既有归属判

判定性为主，一处资源性。词面违例判定直接拦收口（判在书简前），摘录 60 字截断是展示降噪不改判定结论，本批改判资源性走环境参数登记面。

## 载体候选分析

- 词面违例判定语义：已登记术语集上的子串跨度匹配与出现判定。
- 数学仓已建条目对照：mapping.md 全表无串匹配/形式语言条目，零命中如实记。
- 需新数学条目（已登记集上的串跨度匹配判定语义），无哲学新桥 → 载体管线批次输入。
- 摘录截断常数 60 资源性行走环境参数登记面增量件（只增不改）。
- 源码概念引用位实测：sih-tools/nomenclator/src 零命中（本批 grep 实查 2026-09-04）；packs/core/terms.json 含 15 处概念 ID 字样是登记词条内容不是源码引用位，如实区分。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 nomenclator；ledger.json pending_confirmation 一条（matching.py:49 截断 60，原判待确认，本批改判资源性）。
- 调用面：BATCH-FACE.md §7 管线三步第三步检词（判在书简前）。
