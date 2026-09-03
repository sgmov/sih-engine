# mathquote-calc-solo：数学仓重构逐字引文补强波二（calculus 缺锚条目）

> task-packages 治理任务
> 承接：mathquote-solo 与 mathquote2-solo 四子仓 34 件已闭环；calculus 波即 release-audit-2026-09-02 遗产披露第一条的 calculus 部分；主会盘面枚举 calculus 桥接节缺任何原文「锚条目 107 件（LIM8/DIFF26/INT22/APP10/HIS16/MUL10/NS3/SER5/SPEC7），宽形锚口径承 mathquote2 枚举注记
> 用户裁定 2026-09-03 原话照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——本件纯机械即机械链全绿即放行不扰人节点
> 队形：单线形 solo——程序切片加逐条补写亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-quote-calc.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者；与 mathrefmt2-solo 写面重叠即同 51 条目文件并发即故意内容冲突样本，撞锁有限重试上限十次逐次计数

## 一、问题陈述 {#problem}

calculus 107 条哲学桥接以命题对照形态承载而无原文逐字引文锚。逐条回哲学仓原文程序切片引文入桥接段，恢复可机械核验的原文锚。

## 二、关键设计 {#design}

1. 枚举：宽形口径脚本扫 calculus 已建条目哲学桥接节缺锚清单（严正则加嵌套引号宽形），与主会盘面 107 件及前缀计数对表，位移如实申报。
2. 切片：逐条按其命题对照 PRO 编号回 sih-philosophy/emanation/proodos 原文定位引文，程序切片逐字节子串禁手打，落形原文「引文」见 文件:行号，命题对照保留不替换；已有宽形锚条目如有遗漏标准锚照补。
3. 机械复验：补后复验脚本即全部新增引文对原文子串断言全过，脚本入 materials 可重跑。
4. 与 mathrefmt2 并发适配：本批在桥接节 bullet 行尾追加引文，mathrefmt2 改节头与导航块，内容面兼容；施工脚本按内容定位不按行号，撞锁让位后以当刻盘面重跑。

## 三、工作清单 {#work}

- [ ] 枚举脚本与缺锚清单对表
- [ ] 逐条程序切片补写
- [ ] 复验脚本全绿读数
- [ ] 逐条三步管线读数
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 枚举零漏** | 工程 | 清单可 grep 复算，与 107 件及前缀计数对表 |
| **F-2 引文逐字节** | 工程 | 复验脚本全绿即全部引文为原文子串，重跑一致 |
| **F-3 对照保留** | 治理 | 原命题对照零删改，引文为增不换 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列，四子仓零触碰 |
| **F-5 链上完整** | 工程 | 意图与逐件认证落主树活链，close 后链 verify valid，reconcile 零新增 |

## 五、必读文件 {#read}

- 必读 1：sih-philosophy/emanation/proodos/ 原文即切片源
- 必读 2：sih-engine/sih/event/plan/mathquote2-solo-results.md 宽形口径与复验先例
- 必读 3：sih-math/order/entries/ORD-015 即引文落形先例
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 哲学仓只读，引文程序切片逐字节子串禁手打
2. 认证先落主树活链，链 settle 前一次性拷工地，严禁工地链副本追加
3. 撞锁有限重试如实计数，与并发批让位后重跑枚举
4. 词债不过夜 findings 亲读
5. 收约前先放锁后拆工地

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-engine/sih/state/plan/mathquote-calc-solo.md
- sih-engine/sih/event/plan/mathquote-calc-solo-results.md
- sih-engine/sih/event/plan/mathquote-calc-solo-materials/
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
- [ ] 冲突样本节在结果档（含与 mathrefmt2 写面重叠实测）
- [ ] 认证入链双仓结算收约对表读数在档
