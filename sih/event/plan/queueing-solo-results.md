# queueing-solo 结果档

> 批：queueing-solo 注意力预算排队化（三 WIP 读数与洪流前兆判据，路线档第一档件二，载体 PROB-015）
> 会话：cd541966a2c48e5c（sess-zcode-260904-queueing）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理（gauge 读路径亲写 + 机械链亲跑）
> 政策行：用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——本批零新裁决点（PROB-015 已在仓已裁定且语义实覆盖判据场景），机械链全绿自行收口，不设等你令节
> 载体：PROB-015 Little 定律与负载界（mapping.md:205 行号实取）；哲学对照 convergence P3.1 三段（ask3 三锚程序切片于 witness-archive/00-ai-coding-governance-principles.md 213/223/237 行）；推导档 sih-math/docs/queueing-derivation-2026-09-04.md

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| F-1 四件套 | 完成 | 载体引用（cli.py 源内注记锚点 + mapping.md:205 实取）、推导档 math 工地落地、代码接线（gqueue 只读子命令，承 gchart gc-1 先例）、金向量（前兆/平稳两态双跑 IDENTICAL 上链 1af84214） |
| F-2 零行为变更 | 完成 | record 与 gchart 与 read 判据零改动；既有测试零回归（20 passed 3 skipped 零改型），新读路径测试先红（10 failed）后绿（30 passed 3 skipped） |
| F-3 前兆可解释 | 完成 | 前兆输出载三 WIP 读数与 rates（λ̂ 与 μ̂ 与计数与窗）与 ρ̂ 与判据依据行 basis（定理二语义与净积压速率随行），判定面零裸布尔，测试断言在案 |
| F-4 写入仅 allow | 完成 | 写入仅任务包请求写入节五面加共享追加面短持，三仓 settle 见收口读数 |
| M-3 模型域缺口预案 | 未触发 | PROB-015 定理二 ρ≥1 在途无界增长实覆盖洪流前兆判据，公理三可测性实覆盖三台账只读取数，公理一稳态前提实覆盖判据只作前兆不作确定断言；零硬挂零自造载体 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 载体引用 | gauge 既有载体位形制：cli.py 注记锚点 + 数学仓 mapping 行引用，行号实取 | 过 | cli.py 模块 docstring 增 gq-1 载体锚点（entry 路径 + mapping.md:205）；mapping 行号 2026-09-04 grep 实取 |
| F-1 推导档 | WIP 队列语义与到达/服务率估计与前兆判据与窗口参数模型参数声明成档 | 过 | sih-math/docs/queueing-derivation-2026-09-04.md 七节，worked example 与金向量数值一致，checkcite pass（PROB-013/015 落书单闭包 33 件） |
| F-1 代码接线 | 新只读子命令或读路径模块，record 写路径零改动 | 过 | gauge gqueue 子命令（src/gauge/cli.py，cmd_gqueue 与 _gqueue 及四算半函数），read 与 record 与 gchart 函数零触碰，git diff 可核 |
| F-1 金向量 | fixture 台账三 WIP 读数双跑逐字节一致 + 前兆/平稳两态用例 | 过 | 两夹具双跑 cmp IDENTICAL 且与冻结金向量一致（worktree 与主树已提交树两轮复现），重放寻径约定（仓根 cwd 相对路径）在档，主树复现关过 |
| F-2 零行为变更 | 既有 gauge record 与 gchart 与三维读数路径零改动，既有测试零回归 | 过 | 既有 test_gauge.py 与 test_gchart.py 零改型全绿（20 passed 3 skip）；当日例行读数三维快照 ga-2 照常落链（convergence 0.0 与 adoption 0.0 与 mergeback 0.043478） |
| F-2 先红后绿 | 新读路径测试先红后绿 | 过 | 实现前 test_gqueue.py 10 failed 全红（gqueue 子命令缺席 argparse exit 2），实现后 30 passed 3 skipped 全绿 |
| F-3 前兆可解释 | 前兆输出载三 WIP 读数、到达/服务率、判据依据行，不裸报布尔 | 过 | 输出 wip 三块与 rates 两块与 precursor 块（verdict 与 rho 与 basis），basis 载 λ̂ 与 μ̂ 与 ρ̂ 与 rho_bound 与净积压速率与「视图告警处置归人节点不断言洪流必至」；测试断言输出面无 true/false 裸布尔 |
| F-4 写入仅 allow | 写入仅 allow 面 | 过 | 锁实录：sih-tools/gauge exclusive 长持一笔，trail 与 meter/counts 与 scribe/reports append 短持即取即放零撞锁；台账文件零写（sessions/locks/claims 三台账全只读）；三仓产出经工地 settle 归并，主树直写仅收约前置整备三笔（见越线节申报） |

## 三、实现语义摘要（gq-1）

- 三 WIP 读数（点位计数截止 --at 当日，台账零写只读度量）：泊界在泊数（泊材料目录逐件分类 enter/exit，在泊 = 有进无出，孤出记 unmatched_exits、坏件记 unparsed 零虚构）、批在途数（会话台账 issued 未 revoked，载 distinct_packages 与 session_ids）、会话并发数（锁台账 acquired 未 released，载 holding_sessions 与 holders）
- 到达/服务率：窗 [at−(W−1), at] 内 issued 与 revoked 计数时平均 λ̂ 与 μ̂（sessions/day，计数随行），--window-days 缺省 7 与 --rho-bound 缺省 1.0 显式给参即模型参数（PROB-015 工程注意事项三窗口对价）
- 前兆判据（PROB-015 定理二）：ρ̂ = a/s ≥ rho_bound 报洪流前兆，basis 载净积压速率 λ̂−μ̂ 线性发散（定理二证明形态）；单服务台限制如实申报，多服务台排队模型不构造
- status 三值：ok／insufficient（窗内零到达零服务，率与 ρ̂ 与 little 全 null 零虚构，公理一瞬态不可外推）／degenerate（服务停走而到达为正，ρ 无上界即前兆成立形如实申报）
- little 反解（定理一）：仅 ok 且 stable 形出 Ŵ=L/λ̂（隐含平均逗留天数视图值），前兆与退化形不出 Ŵ 零虚构（ρ̂≥1 非稳态承公理一门控）
- 词表七件 established 入 core 包（在途数、到达率、服务率、利用率、前兆、排队化、洪流）随批入版控

## 四、管线读数（化格→核阅→检词，笔在核前）

| 目标 | 化格 | 核阅 | 检词 |
|---|---|---|---|
| sih-math/docs/queueing-derivation-2026-09-04.md | exit 0 无需改 | exit 2 域外如实记（des-001 与 des-001-mathe 域均不盖 docs 面） | exit 0 零违例 |
| gauge/CONTRACT.md | exit 0 无需改 | exit 2 域外如实记（sih-tools 不受治理引擎文档规范约束） | exit 0 零违例 |
| gauge/CALL-LOG.md | exit 0 无需改 | exit 2 域外如实记 | exit 0 零违例 |
| sih/state/plan/queueing-solo.md | exit 0 无需改 | exit 2 域外如实记（state/plan 不在 des-001 域） | exit 0 零违例 |
| sih/event/plan/queueing-solo-materials/dispatch.md | exit 0 无需改 | exit 2 域外如实记（event/plan 不在 des-001 域） | exit 0 零违例 |
| 本结果档 | exit 0 无需改 | exit 2 域外如实记（event/plan 不在 des-001 域） | exit 0 零违例 |

检词包：五目标用工工地 core 包（含本批七件新词）；本结果档用主树归并后 core 包（157 件）。叩问七信号（在途数/到达率/服务率/利用率/前兆/排队化/洪流，全轻）digest passed covered 7，处置即上列七件登记；温故检索 materials/recall-queueing.json「注意力预算排队」零命中如实记。

## 五、认证清单

| 件 | event_hash 前八 |
|---|---|
| intent（ask3 三锚，双门过，digest covered 7） | 29534d04 |
| 2026-09-04-queueing-solo-pipeline.json | 62123ea3 |
| 2026-09-04-queueing-solo-golden.json | 1af84214 |
| 2026-09-04-queueing-solo-derivation.json | ada5ba70 |
| 2026-09-04-queueing-solo-checkcite.json | 2c8e2ea8 |

settle --cert 取链尾认证 2c8e2ea8 三仓同锚。例行读数三笔（75b708ea 与 ffba098e 与 bf6a41e5）随开机快照在链。

## 六、金向量与实跑读数

夹具（tests/fixtures/gqueue/，确定性台账与泊材料）：共用锁台账持锁 4 持锁会话 3；两泊目录进 6 配出 1 孤出 1 坏件 1，在泊 5。

| 态 | 夹具 | λ̂ | μ̂ | ρ̂ | verdict | 三 WIP | Ŵ |
|---|---|---|---|---|---|---|---|
| 平稳 | sessions-stable | 0.285714 | 0.571429 | 0.5 | stable | 泊 5／批 2／并发 4 | 7.0 |
| 前兆 | sessions-precursor | 1.0 | 0.142857 | 7.0 | precursor | 泊 5／批 7／并发 4 | null |

双跑 cmp 逐字节一致两轮（工地与主树已提交树），金向量冻结两件重放寻径约定（仓根 cwd 相对路径）在档。

实跑读数（真实台账与两泊材料目录，at=2026-09-04，窗 7 天）：在泊 16（进 24 出 10 配 8 孤出 2）、批在途 2（locatorwire-solo 与本批）、持锁 7 持锁会话 2；λ̂=26.428571（185 进）≥ μ̂=26.285714（184 出），ρ̂=1.005435，verdict precursor——视图告警如实申报：ρ̂ 微超 1 由窗末两在途会话未结承担（locatorwire 在途加本批自体在途），处置归人节点零自动动作；本批收约后复测该读数即回落，度量面如实反映排队态不修饰。

## 七、越线与误差申报

1. 收约前置整备三笔通道外提交（--no-verify 加 bypass 登记各一）：sih-engine a0a01c0（当日链 85 事件并集超集快照，scribe verify valid 后提交）、sih-tools 9cc5b4be（锁/会话台账与 meter counts 并集超集）、sih-tools 1e6eecf1（claims 台账并集快照）；三笔均共享追加面让位 close 的既定整备形，非链文件改写（链写入全经引擎 scribe 闸三）。
2. 工地并集归并一笔通道外提交（--no-verify 加 bypass 登记）：sih-tools b2584835，与 integral-stage-build 归并遇两 CALL-LOG 与词表三文件内容冲突（locatorwire 先并推进主树），按并集超集放行形消解；首并提交曾短暂含未消解冲突标记件，git diff --check 拦截后 amend 重做（2e13fc4b 仅存在于工地分支史，amend 后 b2584835 入归并，主树从未收到带标记件）。
3. 工具调用误差三笔如实记：elicit check 首调用以未定义变量传包路径报目标不存在 exit 1（纠正后零信号面重跑）；recall 首调用整串传 query 致词通道零命中（纠正为逐词重复传）；checkcite 首调用以内联 JSON 数组传 --cited 报文件缺席退出码一（纠正为文件传参）。三笔均调用面误差非数据面误差，纠正后全过。
4. gauge 源码 VERSION 常量随升 0.6.0，pyproject project.version 仍 0.2.0 未随（0.4.0 先例同款注记如实申报）。
5. 定时窗口观察：real-run verdict precursor 为视图告警非治理动作，本批未因前兆触发任何自动处置，符合工程基线第三条。

## 八、收约读数

### close 三跑与冲突样本节

| 跑 | 结果 | 处置 |
|---|---|---|
| 第一跑 | 拒：tools merge_diverge 三件（锁/会话台账与 meter counts）与 engine merge_diverge 一件（当日链），共享面脏整批拒零部分动作 | 收约前置整备三笔（上节申报 1）加 bypass 三笔 |
| 第二跑 | 拒：engine merge_diverge locatorwire-solo-results.md（并行批正写结果档在途） | 轮询让位（20 秒粒度），locatorwire 收约归并后自净 |
| 第三跑 | 拒：merge-tree 预检内容冲突三件（两 CALL-LOG 与词表） | 工地并 integral-stage-build 并集消解（上节申报 2）加 bypass 一笔 |
| 第四跑 | 拒：engine 归并撞主树同名未跟踪件两件（任务包与 materials） | 备份让位归并对表法四步 |
| 第五跑 | 成：三仓归并（tools already_gone 快路径与 engine 归并删除支与 math already_gone）、三工地拆除、会话吊销 | 备份对表三件 cmp IDENTICAL（BACKUP-IDENTICAL），让位零丢失 |

### 三仓提交号

| 仓 | settle 段1 | 归并 | 整备 |
|---|---|---|---|
| sih-tools（integral-stage-build） | a0a38552 | 8509440 | 9cc5b4be 与 1e6eecf1 |
| sih-engine（main） | d4f0549 | 940e143 | a0a01c0 |
| sih-math（main） | 3c3abc2 | d18ceac | 无需 |

### reconcile 读数（三仓，异常均旧账零新增）

| 仓 | unrouted | unbypassed | cert_missing | 异常归属 |
|---|---|---|---|---|
| sih-engine | 0 | 0 | 0 | 全净 |
| sih-tools | 0 | 0 | 1 | entryunique-solo 段2（526e2be，2026-09-03 旧账，非本批新增） |
| sih-math | 0 | 1 | 2 | baseline 零号 c556abb 与 mathfix2 段2（93c4f0b）与 fmtfix 段2（d561f17），均 2026-08-30 前旧账，非本批新增 |

### 链 verify（close 后）

status valid，85 事件，首哈希 05a8a75e，末哈希 2c8e2ea8（本批 checkcite 认证居链尾）；例行读数三笔加 intent 一笔加认证四笔全数在链，与 locatorwire-solo 并行共笔零丢失（85 事件含其九笔）。

### 共享面对表与主树复现

- scribe/CALL-LOG.md 与 lease/CALL-LOG.md：并集版本随 tools 归并在案（本批各一笔留痕在档尾），与 locatorwire 并行条目并序零丢失。
- 词表：主树 terms.json 157 件归一（本批七件 established 在册；gchart 六件与 latexwire 四件与 locatorwire 八件并行并集收编），主树结果档检词零违例。
- 主树 gauge 全测试族复跑：30 passed 3 skipped（gq-1 与 gc-1 与 ga-2 三代共存在役），金向量两态主树重放 cmp 逐字节一致（已提交树复现关过）。
- 本笔回填提交：close 通道外 wip 形（--no-verify 加 bypass 登记），locatorwire 973578d 先例同形。
- 任务包与 dispatch 与 recall 零命中件随批入版控（备份让位归并对表三件 IDENTICAL）。

## 九、队形验证

单线形 solo 成立：本批全部写入由会话 cd541966a2c48e5c（sess-zcode-260904-queueing）亲写，零 Agent/Task 子代理调用，零 LLM 直改链文件（链写入全经引擎 scribe 闸三 --session 加 --sessions），台账文件零写（三 WIP 读数全只读取数），主树直写仅收约前置整备三笔与回填一笔（均 --no-verify 加 bypass 登记，其余产出经 math/engine/tools 三工地 settle 归并）。
