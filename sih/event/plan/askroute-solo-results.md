# askroute-solo 结果档：三问缩写意图判定包与固定拉起派发批

> 批：askroute-solo（单线 solo，零子代理）。会话：sess-zcode-260911-askroute（session_id b9c493ca586714ce）。
> 令源：用户 2026-09-11 「过得一裁，过了即可开租约修复」。前置一裁 m-askroute-1 stable_clear 九发合同模式同席回填，终签 crosscheck-m-askroute-1 落链，裁决方向 comply。

## 一、交付面

1. `sih-tools/askroute/packs/intents-v0.json`：判定包 v0。unknown_action 三步兜底（nomenclator_query → retriever_recall → ask_human）+ 七意图（coldstart-check、naming、write-chain、lock-wait、park、deyi-adjudicate、recall），缩写表含本日实测惯例七条（过得一裁、得一裁、过得一、温故、体检、泊一下、排队、开约、立约、开租约、冷启动、查册、立名、入泊）。
2. `sih-tools/askroute/CONTRACT.md`：包 schema 契约与变更纪律（unknown_action steps 视为裁判面，变更须独立认证笔载事由）。
3. `sih-tools/askroute/tests/test_pack.py`：drift 守卫七测（id 唯一、链非空、缩写不跨意图冲突、unknown 三步序固定、惯例在册、写入链与剧本对表）。
4. `sih-tools/mcpline/AI-MANUAL.md` §10 会话惯例缩写节：表列缩写与所指，指回判定包（204 行）。
5. 接线两段（域外申报）：sihankor-intent-refine SKILL.md 派发位段；AGENTS.md sih 强制触发协议段停顿闸纪律句。

## 二、F 锚定

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 | 数据治理 | 判定包 schema 全校验过 | 通过（pytest 7 passed） |
| F-2 | 数据治理 | 缩写表含本日实测惯例且写入链与剧本逐命令对表 | 通过（test_convention_abbreviations_registered 加 test_chains_match_manual_scripts） |
| F-3 | 裁判面 | unknown 兜底动作序列在包内且三步序固定唯一 | 通过（test_unknown_action_present_and_ordered） |
| F-4 | 漂移守卫 | 手册惯例节与既有手册守卫共处不破 | 通过（test_ai_manual.py 10 passed） |
| F-5 | 结算治理 | intent 笔与认证笔在链、settle commit 落、双零 | 通过（intent c2a28364、cert 见链、close 后双零） |

## 三、边界申报

1. 前置一裁直跑两笔 boundary（m-dualpath-gate、m-referee-seal）留痕不抹，r2 改写重立谱系披露在 topic authored 行。
2. 标定账本（facet/probes/calibration/ledger.jsonl）444 只读态临时开合追加本日基线行后复原，承 chaingreen-solo 先例。
3. 两段接线文件（.agents/skills/sihankor-intent-refine/SKILL.md、AGENTS.md）在仓外面，按声明道改笔不入双仓 commit。
4. 引擎 ask3repeater Rust 本体扩展与 critsweep 阈值封印均不在本批（候后继批），任务包明确不做节在案。
