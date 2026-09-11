# askroute-solo 任务包：三问缩写意图判定包与固定拉起派发

> 令源：用户 2026-09-11 「过得一裁，过了即可开租约修复」；前置一裁 m-askroute-1 stable_clear 九发终签 crosscheck-m-askroute-1 落链（726c1405 同日链面，裁决方向 comply）。
> 队形：单线 solo，零子代理。会话：sess-zcode-260911-askroute。域：sih-tools 主。
> 设计正典：三问（ask3repeater，DEC-006，DES-013）语义判定产出意图标识；映射由版本化判定包承载；unknown 唯一出口为查册加语料召回加问人。

## 交付面

1. `sih-tools/askroute/packs/intents-v0.json`：判定包 v0。schema：version、unknown_action（固定兜底动作序列 nomenclator_query → retriever_recall → ask_human）、intents 数组（id、缩写表、固定动作链逐命令）。首批收录本日实测在用惯例：过得一裁、温故、体检、泊一下及冷启动体检、命名、写入全链、撞锁、泊界五剧本。
2. `sih-tools/askroute/CONTRACT.md`：包 schema 契约与变更纪律（数据行走批、drift 守卫钉缩写表在册）。
3. `sih-tools/askroute/tests/test_pack.py`：schema 校验（意图 id 唯一、动作链非空、unknown_action 在册、缩写表无跨意图冲突）。
4. `sih-tools/mcpline/AI-MANUAL.md` 新节「会话惯例缩写」：表列缩写与所指，指回判定包（事实清单形，随包版本）。
5. 接线两段（域外申报）：sihankor-intent-refine SKILL.md 加派发段；AGENTS.md MCP 节加停顿闸纪律一行（治理缩写查表 unknown 即走兜底，禁即兴解释）。

## 明确不做

引擎 ask3repeater Rust 本体扩展（候后继批）；critsweep 阈值封印（属 m-referee-seal-r2 落规批）；HTTP 面注册表变更；手册既有正文改笔。

## F 锚定

| F | 判据 | 验法 |
|---|---|---|
| F-1 | 判定包 schema 全校验过 | pytest 绿 |
| F-2 | 缩写表含本日实测惯例且动作链与 AI-MANUAL 剧本逐命令一致 | 对表 |
| F-3 | unknown 兜底动作序列在包内且唯一 | 包查询 |
| F-4 | 手册惯例节与包表一致 | 对表 |
| F-5 | 双仓 settle：intent 笔与认证笔在链、commit 落、双零 | 链 verify |
