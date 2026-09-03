# 盘点档：locator（寻址）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）

## 机制职责一句话

多载体结构化解析派生稳定标识：路径加载体种类加序号加内容哈希四元组 sha256 派生 stable_id，build/query/stale/vectors 四面承载索引构建与查询与陈旧检测与金向量（locator/src/locator/identity.py:7-13）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 稳定标识派生 | locator/src/locator/identity.py:11-13 `stable_id(path, carrier, kind, seq, chash)` 四元组 `\x00` 连接后 sha256 | 确定性寻址唯一性 | 判定性 |
| 内容哈希 | locator/src/locator/identity.py:7-9 `content_hash` sha256 十六进制 | 载体内容指纹 | 判定性 |
| 陈旧检测 | locator/src/locator/stale.py:20 重算哈希对表 `current[rel]` | 存储哈希与现算哈希不等即陈旧 | 判定性 |
| 原始哈希登记 | locator/src/locator/pack.py:36 `raw_hash=hashlib.sha256(raw).hexdigest()` | 包原始字节指纹 | 判定性 |

## 既有归属判

判定性。标识派生与陈旧对表直接决定寻址唯一性与索引新鲜度，零资源性参数。

## 载体候选分析

- 稳定标识派生即外化载体的确定性寻址：内容哈希指纹加结构化四元组即外化状态的一致标识。
- 陈旧检测即外化存储态与现算态的哈希对表（版本一致性判定）。
- 已建条目对照：ORD-019 版本偏序与外化状态存储（mapping.md:196，承 P3.2 外化管理三性质即持久性与版本化与可审计性，已建实存）。
- 同构旁证：parser/src/parser/entries.py:7-11 同款 stable_id（docstring 承 DES-009），两件同构派生语义。
- 源码概念引用位实测：sih-tools/locator/src 零命中（本批 grep 实查 2026-09-04）。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 locator；ledger-rev1.json carrier_matching locator 条目 source_math_ids 空与 face_refs 三面空。
- 消费面：sih-math/llm-friendly-build/mapping.md:196 ORD-019 状态已建。
