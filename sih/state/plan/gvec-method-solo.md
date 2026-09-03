# gvec-method-solo：金向量方法论立项批（全量审计加命题先裁后行）

> task-packages 治理任务
> 承接：用户 2026-09-03 质询原话照录「但是我们司衡很多地方唯金向量标准，这个机制没有数学依据，没有方法论，如何保证它的工程经验能够拖底？」；ORD-007 锚集形式化即锚绿不等于链真；SPEC-013 修订三至六金向量三事故史；pk-036 与 pendsweep 重冻判例堆；主会盘点 28 份结果档用金向量、数学仓覆盖与等价侧零载体（mapping 查询如实记）
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——件一纯机械直接做；件二裁量类先过得一裁，stable_clear 即执行未过进泊界，承 autoflow2 件二三态分流先例
> 队形：单线形 solo
> 日期：2026-09-03
> 温故检索：materials/recall-gvec.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，结果档须载冲突样本节

## 一、问题陈述 {#problem}

金向量在批机械链与测试面 pervasive 承重（结果档 28 份引用、scrutinator fixtures、各工具 vectors 目录），但其担保力从未被精确划界：数学侧无覆盖与等价载体，工程侧生命周期规程散落为判例堆（SPEC-013 修订四脏目标条款、修订五同参形、修订六冻错重录、pk-036 与 pendsweep 重冻先例）。历史三事故即三失效模式：goldfix 即净目标盲区即覆盖缺口、双前缀即冻错即锚真值缺口、lim001 即期望过期即锚生命周期缺口。

## 二、件一：全量金向量用法审计（纯机械） {#work-one}

1. 枚举：脚本扫三仓全部金向量位点即 cargo 测试 golden 系、scrutinator fixtures/golden、各工具 vectors 目录、结果档 F 表金向量断言行。
2. 分类：逐断言判两类——甲类变更检测即重放一致或行为零变更或漂移报警（在其声明范围内健全），乙类隐含正确性主张即由重放一致推出判定正确或语义无误（越权即漏洞）。
3. 读数：甲乙计数与乙类逐件清单，乙类即本批产出之漏洞清单。

## 三、件二：金向量方法论命题先裁后行（得一裁） {#work-two}

1. 起草命题入 facet：gid gvec-method-guard-1，n 九发，锚 baseline_4 可验证性。命题：「金向量生命周期与覆盖方法论应立为引擎向界层规范 SPEC-017——真值依据登记开链前置、覆盖要求即净脏双目标加输入类边界含入、漂移归因三态机械规程即内容合法变重冻与判定逻辑变查载体与向量本身错双侧重审、重冻权限归批不归人手改，四条款成文并挂 ORD-007 锚集条目扩展即有限向量集上等价断言的健全性边界与覆盖度量形式化」。
2. **对己不利声明双倍显式**：其一元层自指即 facet 裁决机制自身用金向量冻结 gate 输出，本命题攻击的是裁决尺子自身的地基，若立则先例回溯适用全部既有金向量；其二利益披露即本席与后续所有批的 F 表均依赖金向量，方法论收紧即本席后续每批负担加重。
3. 三态分流：stable_clear 即签核执行落 SPEC-017 加数学仓 ORD-007 扩展草案（扩条目批另开走数学仓管线）；boundary 或不过即不签不硬来进泊界 pk 位登记。

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 枚举零漏** | 工程 | 金向量位点全量清单可 grep 复算，对表 fixtures 与测试计数 |
| **F-2 分类可复核** | 工程 | 甲乙分类逐断言附原文行，乙类清单即漏洞清单零含糊 |
| **F-3 命题合规裁** | 治理 | 件二全流程引擎件出裁即 emit 至 check 至 sign 或泊，对己不利双声明入档 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-math/order/entries/ORD-007-derivation-soundness-and-mirror.md 锚集节
- 必读 2：sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md 修订三至六
- 必读 3：sih-engine/sih/event/plan/autoflow2-solo-materials/dispatch.md 件二三态分流流程先例
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 审计只分类不改任何金向量
2. 命题不过不硬签，泊界登记如实
3. 认证先落主树活链，链 settle 前一次性拷工地
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-tools/facet/facet_task_packages/gvec-method-guard-1/
- sih-engine/doc/spec/SPEC-017-golden-vector-lifecycle.md（件二过裁后）
- sih-engine/sih/state/plan/gvec-method-solo.md
- sih-engine/sih/event/plan/gvec-method-solo-results.md
- sih-engine/sih/event/plan/gvec-method-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/facet/CALL-LOG.md
- sih-tools/tally/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 乙类漏洞清单在档可逐件复核
- [ ] 件二三态实态在档（SPEC-017 落位或泊界事件哈希）
- [ ] 冲突样本节在结果档
- [ ] 认证入链结算收约对表读数在档
