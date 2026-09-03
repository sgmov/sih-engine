# m3clear-solo 处置清单（m3clear-disposition）

- 批：m3clear-solo（M-3 不可译清账：盘点、得一裁、三态分流，零施工）｜ 日期：2026-09-04
- 得一裁：facet 合同模式九发三态，判据 v3 闸，材料四件齐（topic/contract/responses/score）
- 命题区：sih-tools/facet/contracts/zc-glm53flash-20260904/（contract 与 responses 与 score-material 各十组）与 sih-engine/sih/event/plan/m3clear-solo-materials/topics/（topic 十件）与 sih-tools/proposition/DES/m3c-*/flywheel-trail.jsonl（飞轮 trail 十组）
- 政策行：用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——逐件裁决点全过得一裁机械分流，三态分流零人工代裁

## 逐件三态表

| 件 | 机制职责 | 判据位 | 命题 gid | 闸三态 | 去向 |
|---|---|---|---|---|---|
| elicit | 缺口信号检测与消化闸 | cli.py:66 隶属判定与 :116 覆盖判定 | m3c-elicit-1 | stable_clear（9 发零 void） | 实例化候选行：ORD-008 承接消化闸（候选，待接线批） |
| formatter | 包驱动确定性格式归一 | format.py:54-65 应用序与未知拒绝 | m3c-formatter-1 | stable_clear（9 发零 void） | 新载体候选条目行：重写系统确定性幂等（无哲学新桥，载体管线输入） |
| latex-helper | LaTeX 数学计算与档面辅助 | compute.py:4-14 运算枚举与 cli.py:33-35 退出码 | m3c-latexhelper-1 | stable_clear（9 发零 void） | 实例化候选行：LIM-007 与 INT-007 与 SER-001 与 ALG-001 条目族承载 compute（候选，待接线批） |
| locator | 多载体解析派生稳定标识 | identity.py:11-13 派生与 stale.py:20 对表 | m3c-locator-1 | stable_clear（9 发零 void） | 实例化候选行：ORD-019 承载寻址与陈旧检测（候选，待接线批） |
| locks | 租约锁面互斥让路 | lockcore.py:179/206/240/289 四判据位 | m3c-locks-1 | stable_clear（9 发零 void） | 实例化候选行（已实例化）：ordwire-lease-solo 已接线 ORD-020，账清零补施工 |
| meter | 调用计量与对账 | cli.py:41-43 逐调用一记录与 :130 对账 | m3c-meter-1 | stable_clear（9 发零 void） | 新载体候选条目行：计数测度可加性（无哲学新桥，载体管线输入） |
| nomenclator | 术语三态登记与词面核查 | matching.py:14 跨度迭代与 :49 截断 | m3c-nomenclator-1 | stable_clear（9 发零 void） | 新载体候选条目行：基准集串跨度匹配（无哲学新桥）；资源性行入环境参数登记面 |
| parser | 空腹 PEG 解析与条目投影 | lexer.py:72 码点算术与 lint.py:131 阈值 | m3c-parser-1 | stable_clear（9 发零 void） | 新载体候选条目行：形式文法解析确定性（无哲学新桥，载体管线输入） |
| scrutinator | 空腹谓词引擎包驱动核阅 | engine.py 谓词求值核与包 rules.toml:80 | m3c-scrutinator-1 | stable_clear（9 发零 void） | 实例化候选行（已实例化）：gatecap-solo 已在包级接线 ORD-008（包 0.2.0），包级账清；资源性行入环境参数登记面 |
| selector | 谓词路由三路归位 | route.py:24-45 与 pack.py:78 锚点位 | m3c-selector-1 | stable_clear（9 发零 void） | 实例化候选行（已实例化）：selwire-solo 已接线 ALG-002，账清零补施工 |

## 去向汇总

- 实例化候选行（已实例化，账清）：locks（ORD-020）、scrutinator（ORD-008 包级）、selector（ALG-002）——线索三件全部坐实，rev1 无可指认读数被三批推进清账。
- 实例化候选行（候选，待接线批输入）：elicit（ORD-008）、latex-helper（LIM-007/INT-007/SER-001/ALG-001）、locator（ORD-019）——接线形态即源码判定位注释锚点（selwire 先例），零行为变更。
- 新载体候选条目行（载体管线批次输入，不代建）：formatter（重写系统确定性幂等）、meter（计数测度可加性）、nomenclator（基准集串跨度匹配）、parser（形式文法解析确定性）——四条均为数学仓新条目需求且无哲学新桥，涉命题映射零（工程实证语义），不触发哲学仓演化候选输入登记。
- 环境参数登记面行（只增不改）：nomenclator matching.py:49 摘录截断 60、scrutinator engine.py:87 表格流计上限 3，两行走 sih-math/docs/mathpipe-coverage-2026-09-03/env-params-m3clear-2026-09-04.json 增量件。
- 冻结登记（boundary）：零件——十件闸读数全 stable_clear，泊界零新登。
- 哲学仓演化候选输入登记：零件——十件去向均不涉哲学新桥。

## 归类变动申报（逐件，零静默改判）

| 件 | 常数/判据位 | rev1 原判 | 本批判 | 依据 |
|---|---|---|---|---|
| latex-helper | cli.py:33-35 EXIT_OK/VIOLATION/ERROR | 待确认 | 判定性 | 退出码枚举直接判违规位，进档面管线 |
| latex-helper | cli.py 解析通道选型 | （未列） | 待确认 | 外部库行为不在源码判定面 |
| parser | lexer.py:72 代理对算术 | 待确认 | 判定性 | 构造性正确的码点归一算术，判定字符切分 |
| parser | lint.py:131 样例阈值 8 | 待确认 | 判定性 | 覆盖下限直接决定 lint 通拒 |
| nomenclator | matching.py:49 截断 60 | 待确认 | 资源性 | 输出降噪族，改判行入 env-params 增量件 |
| scrutinator | engine.py:87 流计 3 | 待确认 | 资源性 | findings 降噪族，退出码通拒不变，改判行入 env-params 增量件 |
| meter | cli.py:43 按日分册 | （未列） | 资源性（备注） | 存储分册形，无独立常数行不登 env-params |
| selector | 参照时间显式给参 | （未列） | 资源性（备注） | 环境参数形，无独立常数行不登 env-params |

## 载体管线批次输入（后续批清单，本批零施工）

1. 接线批候选三件：elicit 接 ORD-008（digest 判定位锚点）、latex-helper 接 LIM-007/INT-007/SER-001/ALG-001（compute 判定位锚点）、locator 接 ORD-019（identity 与 stale 判定位锚点）；形态照 selwire 注释锚点先例，零行为变更。
2. 新条目批候选四件：重写系统确定性幂等（formatter）、计数测度可加性（meter）、基准集串跨度匹配（nomenclator）、形式文法解析确定性（parser）；各命题 gid 与材料四件即本批在档测量证据。

## 温故与账本对表

- recall-m3clear.json：{"envelope":"recall","topics":["M-3清账"],"count":0} 零命中如实记。
- rev1 账本三件（summary-rev1.md 与 ledger-rev1.json 与 env-params-rev1.json）只读零改动；ledger.json 原账本只读作证据源。
- env-params 既有行只增不改：本批增量件独立成件，既有两件零触碰。
