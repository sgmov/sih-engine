# 盘点档：parser（句读）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）

## 机制职责一句话

空腹 PEG 解析与条目投影：grammar 驱动的 tokenize+parse 纯数据解析，entries 面投影条目带 content_hash 与 stable_id，lint 面查样例覆盖，vectors 面金向量冻结（parser/src/parser/engine.py:7 与 entries.py:7-11）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 代理对解码算术 | parser/src/parser/lexer.py:72 `code = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00)` | UTF-16 代理对到码点的解码算术 | 判定性（构造性正确的数学算术，判定字符归一） |
| lint 样例覆盖阈值 | parser/src/parser/lint.py:131 `if len(examples) < 8` | 规则样例少于八条即 lint 失败 | 判定性（覆盖下限阈值直接决定 lint 通拒） |
| 条目寻址派生 | parser/src/parser/entries.py:7-11 `content_hash` / `stable_id` 四元组 sha256 | 投影条目确定性寻址 | 判定性 |
| 解析判定 | parser/src/parser/engine.py:7 `parse_text(text, tokens_spec, grammar)` | 文法驱动的解析成功/失败判定 | 判定性 |

## 既有归属判

判定性。解析与 lint 与寻址三面全属判定语义，零资源性参数。

## 载体候选分析

- 解析面判定语义：PEG 形式文法的解析确定性（同输入同 parse 树）。
- 寻址面与 locator 同构（stable_id 四元组派生，locator 盘点档已对照 ORD-019）。
- lint 覆盖阈值八条是判定阈值不是资源参数。
- 数学仓已建条目对照：mapping.md 全表无形式文法/解析确定性条目，零命中如实记。
- 需新数学条目（形式文法解析确定性），无哲学新桥 → 载体管线批次输入；寻址面与 locator 同构可同批承接。
- 源码概念引用位实测：sih-tools/parser/src 零命中（本批 grep 实查 2026-09-04）。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 parser；ledger.json pending_confirmation 两条（lexer.py:72 代理对算术与 lint.py:131 阈值 8，原判待确认，本批改判判定性）。
- 文件索引：sih-tools/parser/ 职能「句读／parser，空腹 PEG 解析与条目投影，纯数据零引擎改动」。
