# mathquote2-solo：数学仓重构逐字引文补强波一重发（四子仓，承 mathquote-solo 中途死亡处置）

> task-packages 治理任务
> 承接：mathquote-solo 原任务包全 scope 不变（sih-engine/sih/state/plan/mathquote-solo.md）；原批会话 c725f413e88bc177 于施工与管线全绿后、上链前中途死亡，主会验收处置即抢救 diff 落材料目录、强拆工地、吊销会话，账见 bypass 与本行
> 队形：单线形 solo——程序切片加逐条补写亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-quote.json 双档零命中如实记（承原批读数）
> 冲突测试：pk-045 样本库参与者，结果档须载冲突样本节，含原批死亡样本引用

## 一、问题陈述 {#problem}

与原包同：四子仓（topology/probability/order/algebra）哲学桥接节缺原文逐字引文条目共 34 件（枚举 47 桥接条目、13 件已锚），逐条回哲学仓原文程序切片引文入桥接段。原批已把 34 件补写做完且管线三步全绿、主会复算 34 件引文全部为原文逐字节子串，但零链上事件零结算即死亡。本批重发即把这批已验证施工合法落链。

## 二、关键设计 {#design}

1. 施工源二选一：甲即以抢救 diff（sih-engine/sih/event/plan/mathquote2-solo-materials/salvage-34quotes-2026-09-03.diff，442 行，自原批工地逐字节导出）为盘点源 apply 后逐件机械复验；乙即照原包脚本从头重做。无论甲乙，本批全部机械验证（子串断言、管线三步、金向量对表）必须本批重跑，读数不继承原批。
2. 抢救 diff 地位申报：盘点源不是引用源。apply 后以 reverify 脚本子串断言逐件过为唯一放行判据，断言不过之件零容忍即重做该件。
3. 其余承原包：切片逐字节子串禁手打、落形原文「引文」见 文件:行号、命题对照保留不替换、calculus 零触碰、已锚 13 件零触碰。

## 三、工作清单 {#work}

- [ ] apply 或重做 34 件补写
- [ ] 复验脚本全绿读数（本批重跑）
- [ ] 逐条三步管线读数（本批重跑）
- [ ] 认证上链双仓收约对表（原批死在此步之前，本批必须走完）
- [ ] 冲突样本节含原批死亡样本引用

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 枚举零漏** | 工程 | 清单可 grep 复算，34 件与 47/13 计数对表 |
| **F-2 引文逐字节** | 工程 | 复验脚本全绿即全部引文为原文子串，重跑一致 |
| **F-3 对照保留** | 治理 | 原命题对照零删改，引文为增不换 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列，calculus 与已锚 13 件零触碰 |
| **F-5 链上完整** | 工程 | 意图与逐件认证落主树活链，close 后链 verify valid，reconcile 零新增（原批即缺此件） |

## 五、必读文件 {#read}

- 必读 1：sih-philosophy/emanation/proodos/ 原文即切片源
- 必读 2：sih-engine/sih/state/plan/mathquote-solo.md 原任务包
- 必读 3：sih-math/order/entries/ORD-015 即引文落形先例
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 哲学仓只读，引文程序切片逐字节子串禁手打
2. 认证先落主树活链，链 settle 前一次性拷工地，严禁工地链副本追加（护栏一已在役即 scribe 会拒，勿触发）
3. 撞锁有限重试如实计数，不绕行
4. 词债不过夜 findings 亲读
5. 收约前先放锁后拆工地（原批处置序教训即反序致 close 拒）

## 七、请求写入 {#requested-writes}

- sih-math/topology/entries/
- sih-math/probability/entries/
- sih-math/order/entries/
- sih-math/algebra/entries/
- sih-engine/sih/state/plan/mathquote2-solo.md
- sih-engine/sih/event/plan/mathquote2-solo-results.md
- sih-engine/sih/event/plan/mathquote2-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 复验脚本与清单在档
- [ ] 冲突样本节在结果档（含原批死亡样本引用）
- [ ] 认证入链双仓结算收约对表读数在档
