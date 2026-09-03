# mathclose-solo：数学仓重构收尾批（括注扫尾 + 金向量重冻 + 主会 riders 入库）

> task-packages 治理任务
> 承接：数学仓重构两波格式归一与两波引文补强本体已闭项，收尾三件即 lim001 加 mul001 金向量双红遗留（用户 2026-09-03 裁定转遗留）与 VERSION.md 状态行括注债（补强波重写条目应记（波名 增补）括注未记）与主会验收 riders 未入库（PARKING 七项名册与 pk-046/pk-047 材料与台账活写件）
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——本批纯机械即机械链全绿即放行不扰人节点
> 队形：单线形 solo
> 日期：2026-09-03
> 温故检索：materials/recall-mathclose.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，结果档须载冲突样本节

## 一、问题陈述 {#problem}

其一，cargo test --lib 现况 139 过 2 败即 golden_des001mathe_lim001 与 mul001 期望哈希过期（mathrefmt 波一与 mathrefmt2 波二合法改写加 mathquote-calc 引文追加叠变）。其二，VERSION.md 状态语义立规矩即补强波重写条目记（波名 增补）括注，四波（mathrefmt、mathquote2、mathquote-calc、mathrefmt2）改写条目均未记。其三，主会验收处置 riders（名册七项与撞号披露与 pk-046 加 pk-047 材料）以未提交态在主树等待随批入库。

## 二、关键设计 {#design}

1. **时序硬约束（本批命门）**：件一括注扫尾必在件二金向量重冻之前完成并留证——扫尾会改 LIM-001 与 MUL-001 内容哈希，先冻后扫即重演跨批锚竞态（pk-045 第十三类）。同批有序即竞态根除。禁止倒序。
2. 括注枚举机械归因：基线 mathrepo-v1（mathver-solo 0eab3ae）以来四波提交即 a4372c1（mathrefmt）、e4d5724（mathquote2）、bd60afd（mathquote-calc）、56e90c8（mathrefmt2），逐文件 git 归因到波，改写条目集合预期 147 件即 calculus 113 加四子仓 34（APP-011 零触碰零括注）。
3. 括注形：状态行改「已建，哲学到工程桥梁条目（<波名> 增补）」，多波并置波名以「加」连接，波名取批名去 -solo（mathrefmt、mathquote2、mathquote-calc、mathrefmt2）。只动状态行一行，正文零改。
4. 金向量重冻：件一完成后取盘上实哈希，刷 sih-engine/src/scrutinator/fixtures/golden/des-001-mathe-lim001.json 与 des-001-mathe-mul001.json 各恰一行 content_hash，消费逻辑零改，cargo test --lib 全绿两跑一致（pk-036 与 pendsweep 随冻先例）。
5. riders 入库：PARKING-v1.md 现未提交态逐字节入库（七项名册与 pk-046 金向量方法论行与 pk-047 OTel 行含撞号披露）、sih/state/parking/materials/pk-046.json 与 pk-047.json 入库、sih-tools 台账活写件（locks 与 sessions 与 meter counts）随批入库，全部 identical 对表零改字。

## 三、工作清单 {#work}

- [ ] 括注枚举脚本与 147 件对表
- [ ] 逐件状态行括注与正文零动逐字节验证
- [ ] 时序留证（件一完成读数先于件二取哈希）
- [ ] 金向量双件重冻与 cargo 全绿双跑
- [ ] 逐件三步管线读数
- [ ] riders identical 入库
- [ ] 认证上链三仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 括注枚举零漏** | 工程 | 四波提交机械归因清单可 grep 复算，147 件对表，APP-011 零括注 |
| **F-2 括注形合规** | 工程 | 逐件状态行恰改一行，波名归因与 git 历史一致，正文逐字节一致 |
| **F-3 金向量转绿** | 工程 | 重冻后 cargo test --lib 全绿（含全部金向量）双跑一致 |
| **F-4 时序不可倒** | 工程 | 件一完成与件二取哈希时序读数在档，重冻哈希即扫尾后盘面 |
| **F-5 riders 入库** | 治理 | 名册与泊材料与台账件 identical 对表入库零改字 |
| **F-6 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-math/VERSION.md 状态语义节与重绑纪律节
- 必读 2：sih-engine/sih/event/plan/mathrefmt2-solo-results.md 金向量处置节（用户转遗留裁定在案）
- 必读 3：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 件一件二时序硬约束，倒序即停批
2. allow 面须完整覆盖 fixtures/golden/（goldlim 教训即包面指错文件致静默扩锁，本包已列全，禁再扩）
3. 认证先落主树活链，链 settle 前一次性拷工地，严禁工地链副本追加
4. 撞锁有限重试如实计数，收约前先放锁后拆工地
5. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-math/topology/entries/
- sih-math/probability/entries/
- sih-math/order/entries/
- sih-math/algebra/entries/
- sih-engine/src/scrutinator/fixtures/golden/des-001-mathe-lim001.json
- sih-engine/src/scrutinator/fixtures/golden/des-001-mathe-mul001.json
- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/parking/materials/pk-046.json
- sih-engine/sih/state/parking/materials/pk-047.json
- sih-engine/sih/state/plan/mathclose-solo.md
- sih-engine/sih/event/plan/mathclose-solo-results.md
- sih-engine/sih/event/plan/mathclose-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/lease/ledger/
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 时序读数与冲突样本节在结果档
- [ ] 认证入链三仓结算收约对表读数在档
