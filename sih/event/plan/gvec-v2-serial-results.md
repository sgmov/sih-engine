# gvec-v2-serial 结果档：金向量双种方法论重裁与两件出泊

> 承接：任务包 gvec-v2-serial.md 与修正稿 2026-09-06-gvec-v2-design.md。批四件：SPEC-021 成文、单锚重裁（gid gvec-v2-guard-1）、得一终签、pk-046 与 pk-068 出泊两笔。
> 队形串行 serial：主会（主线）起草与开约锁链与验收，串行子代理（ZCode/GLM，会话 c41ce2afaaa8cc45）持全量上下文连续执行链。
> 日期：2026-09-06，意图会话标识 sess-zcode-260906-gvecv2。

## 意图锚定

- 意图事件：intent_refined `2d1ea900`（当日链核实在位）
- ask3 记录：sih-tools/scribe/reports/2026-09-06-ask3-gvec-v2-serial-record.json，认证笔 `cde2f17f`
- ask3 验证件：sih-tools/scribe/reports/2026-09-06-ask3-gvec-v2-serial-validation.json（status ok，anchor_count 3），认证笔 `e8f48e53`
- 正身件：sih-tools/identity/reports/2026-09-06-gvec-v2-serial-identity.json（status attest，anomalies 0，identity_hash b0604b90），认证笔 `a0402fec`

## 前置读数

主会开工前读数（承任务包与开工面）：

- 双门过：ask3 记录过 scrutinator ask3 规则包门与引擎 ask3repeater 门（validation status ok）
- 叩问 digest：全信号 covered 6，零 blocked
- 正身：identity verify status attest，anomalies 0
- 泊界心跳：零在泊告警如实转述

## 件读数

### 件一 SPEC-021 成文与管线

- 落位：doc/spec/SPEC-021-golden-vector-dual-method.md（引擎工地，settle 通道入主树），正文即修正稿全部内容重组为规范文档形（一词两物、四层模型、机械承载纪律与六冻结对象清单、分块矩阵与自反性、退化良性质、T1 至 T11 条款组、数学载体、流程路由），零条款语义增删
- 化格 general-v1：exit 0（无需改）
- 核阅 des-001：工地物理路径对域 glob 报 exit 2 域外；按 agentslim-solo 与 adjudisp-solo 与 pk037impl-solo 先例以内容恒等域内校验拷贝（src/scrutinator/fixtures/corpus/check/ 下，SHA-256 与工地件逐字节一致）承检 exit 0 findings 0，拷贝验毕即删不入提交
- 检词 core：首跑 exit 1（dead_ban 禁词两处，该词唯一在册义项为粤语入狱坐监，词面不复现），改登记术语后重跑 exit 0 findings 0

### 件二 单锚重裁 gvec-v2-guard-1

- 命题：facet_task_packages/gvec-v2-guard-1/topic.md（工具工地），谱系披露双声明在场（元层自指与利益披露，两重不利都指向 comply 从严）
- 出题 emit-contract：exit 0，合同 sha256 `77b9e60d`，topic sha256 `cc98746a` 前位
- 回填九发：responses.jsonl 九行（sha256 `ec897f3b`），子代理即席逐发独立作答（同席采样承先例披露）
- 计分 score：exit 0，九发全 comply（合合合合合合合合合，变卦 0%），谨慎信号 0/9，规约引用单值 baseline_4（上回三值分散 baseline_1×1／baseline_4×6／baseline_5×2 由单锚归约封口），闸门裁决判据 v3：stable_clear
- 计分材料：proposition/DES/gvec-v2-guard-1/contract-score-material.json（单元格复制在位）

### 件三 得一终签

- 席位当日基线：温度探针标定四命题×五发（cal-pack 与 cal-responses 随批材料），score exit 0 判定可用（体温 0.0、准确性 true、零漂移告警、零解析失败），身份哈希与材料同值 b0604b90 承 pk-035 哈希优先配对；首试以 2026-09-05 旧账本尾行为基线致 R5 哈希不一致 sign refused（处置挂起），按 openhyg-solo 勘误批内自跑标定产当日基线后通过
- tally assemble：exit 0 产 tally-check-input 材料
- attractor check：exit 0，R1 至 R7 全过（R5 席位当日基线判定可用且身份哈希一致），verdict pass
- attractor verify：exit 0，identical（重放锚 facet/contracts/gvec-v2/sign/gvec-v2-guard-1-signcheck.json）
- attractor sign：exit 0，crosscheck_completed 落链，事件哈希 `dedc76e9`，disposition 裁决通过，direction comply

### 件四 出泊两笔

- pk-046-exit.json 与 pk-068-exit.json 落工地 parking/materials（正形参照 pk-057-exit 字段形，json-canonical 化格 exit 0），载裁决指向即用户 2026-09-06 开工令承裁＋单锚重裁 stable_clear 终签 dedc76e9
- scribe park exit 两笔入链：pk-046 `4672e40e`、pk-068 `7718d15f`

### 温故切面

- 结果档起草前检索：retriever recall --event parking_entered --since 2026-09-03 --until 2026-09-06 --at 2026-09-06，exit 0，输出件随批材料 recall-event-pre-results.md

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 重裁终签 | 治理 | 九发经引擎 attractor，stable_clear 即 crosscheck_completed 在链；闸不过即泊位保持（双态可证伪） | 通过（stable_clear，crosscheck_completed dedc76e9 在链核实在位） |
| F-2 SPEC-021 成文 | 工程 | doc/spec/ 下 021 新件落位，化格核阅检词全过即 des-001 域内退出码 0，零绕行 | 通过（化格 0、核阅域内拷贝 0 findings 0、检词 0；工地直路径域外 exit 2 如实记不属违规） |
| F-3 出泊两笔 | 数据治理 | stable_clear 后 pk-046 与 pk-068 出泊事件在链且 exit 件载裁决指向；闸不过则改判泊位保持且零出泊写入 | 通过（parking_exited 4672e40e 与 7718d15f 在链，exit 件载开工令承裁＋终签哈希） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（写入面见越线申报节，全落 allow 十一路径与双仓工地） |
| F-5 子代理不越权 | 编组治理 | 零人节点代行、零绕行、零 plain commit、零预设重裁结论 | 通过（出泊唯人节点由开工令承裁在档，机器门 stable_clear 后才写 exit；重裁逐发独立作答；settle 走 lease commit 通道） |

## 越线与误差申报

- 无越线项。误差申报三笔，均如实：
- 其一：检词首跑 2 违例（dead_ban 禁词两处，词面不复现，详见读数件），系术语疏失非条款语义变更，改登记术语重跑 0，修订稿原词形保留在域外批输入件不动。
- 其二：attractor 首验拒收 facet-contract-score 形材料（材料 kind 不符），按 openhyg-solo 勘误经 tally assemble 零判断装配转 tally-check-input 形后全过；首跑 check 材料误用计分材料属调用形误差非工具异常。
- 其三：sign 首试 refused（处置挂起），根因即席位当日基线身份哈希与材料不一致（旧账本尾行为 2026-09-05 异日身份），按 openhyg-solo 先例批内自跑温度探针标定产当日基线（判定可用）后 sign exit 0；期间零链写入零出泊写入，refused 读数如实留档。
- 附注一：核阅工地件域 glob 不匹配属固有路径形态（先例三批同款），域内校验拷贝法承检非绕行，拷贝哈希恒等且验毕即删。
- 附注二：正身件输出形 summary.identity_hash 与 facet 件读取位 identity.hash 不同构，经工具自带 --identity-hash 二选一通道同值直给（b0604b90），非杜撰。
- 主树直写零笔：链写入全经引擎 scribe 写位（认证三笔、终签一笔、出泊两笔）；主树其余件零触碰。

## 结算读数

- 双仓 settle：engine 工地提交 d55f518（base main@1164de0，三查过），tools 工地提交 f16d16a5（base integral-stage-build@e4c77b05，三查过）；cert 取 cde2f17f 即 ask3 记录认证哈希前八位。
- 放锁收约：十一路径 unlock 毕全部 exit 0；close 首跑即成功零碰撞（closeguard 预提交 05ea572 吸收主树未跟踪件后归并 fa14d39，双工地与分支清除、会话 c41ce2afaaa8cc45 revoked、零失败）。
- 链 verify：2026-09-06 当日链 valid，37 事件，末笔即出泊 pk-068 7718d15f。
- reconcile（无管道真码）：双仓真实退出码均 1，驱动项全为历史账面即 unbypassed 38/72、session_orphan 29/22、工具仓 cert_missing 1 笔属 2026-09-03 entryunique-solo 旧账（archpark 档已登记同一笔）；本批对照批前零新增成立（unrouted 双仓 0、engine cert_missing 0、无本批 sha 落违规类）。
- 越线与误差申报追记（主线）：genpark-solo 结果档「reconcile：双仓 exit 0」系管道掩码误差（管道尾取了 tail 的退出码），当时真实退出码亦为 1 且同为历史账面零新增；勘误行已随本笔补笔加注于 genpark 档尾。
- 心跳复算：引擎线 mainline 39、告警零、六件存量废轨不变；出泊两件 enter 件留册、exit 件入废轨属常态。
- 收约补笔：本结果档回填与 genpark 勘误即本笔，经 --no-verify 加 lease bypass 登记通道入版控（archpark-solo 与 openhyg-solo 先例同形）。
