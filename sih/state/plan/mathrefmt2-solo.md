# mathrefmt2-solo：数学仓重构格式归一波二（calculus 余 51 条概览归一）

> task-packages 治理任务
> 承接：mathrefmt-solo 波一（62 条 LIM/DIFF/INT 已归一）余件；release-audit-2026-09-02 遗产披露第二条；波一误差申报修正后盘面即 114 总、定义首节 63、概览首节余 51（APP10/HIS16/MUL10/NS3/SER5/SPEC7）
> 用户裁定 2026-09-03 原话照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——本件纯机械即机械链全绿即放行不扰人节点
> 队形：单线形 solo——确定性脚本加逐条改写亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-fmt2.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者；与 mathquote-calc-solo 写面重叠即同 51 条目文件并发即故意内容冲突样本，撞锁有限重试上限十次逐次计数

## 一、问题陈述 {#problem}

calculus 余 51 条首二级节仍为概览旧格式（纯目录锚点导航），与波一 62 条及 APP-011 的定义首节不齐。本波收尾即 114 条全仓定义先节归一。

## 二、关键设计 {#design}

1. 变换承波一逐字同款：概览导航块整体移除（六行均为自指锚点无正文）→ 原「## 话题」改「## 定义 {#definition}」→ 正文零改动；每条 diff 删行必须全为锚点导航行，正则全匹配，否则停批申报。
2. 枚举对表：51 条与前缀计数 APP10/HIS16/MUL10/NS3/SER5/SPEC7 双向对表，与 INDEX 声明对表。
3. 金向量联动（波一教训入判据）：本波含 MUL 前缀即 golden_des001mathe_mul001 大概率随内容漂移；本批全量跑 cargo test --lib，凡本波改写引发的期望漂移随批重冻（pk-036 与 pendsweep 先例，只刷期望零改消费逻辑，live 实跑全绿），禁止漏报。
4. 与 mathquote-calc 并发适配：本波改节头与导航块，mathquote-calc 在桥接节行尾追加引文，内容面兼容；撞锁让位后以当刻盘面重跑枚举再施工。

## 三、工作清单 {#work}

- [ ] 枚举 51 条与前缀计数对表
- [ ] 逐条变换与零信息丢失逐字节 diff 验证
- [ ] 全量 cargo test --lib 读数与漂移随批重冻
- [ ] 逐条三步管线读数
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 枚举零漏** | 工程 | 51 条与前缀计数及 INDEX 对表，改后 114 条全定义首节 |
| **F-2 块序归一** | 工程 | 51 条首二级节全为 ## 定义 {#definition}，S005 域内通过 |
| **F-3 信息零丢失** | 工程 | 删行全为锚点导航行（正则全匹配），正文逐字节一致 |
| **F-4 金向量零漂移残留** | 工程 | cargo test --lib 全绿，本波引发漂移随批重冻，双跑一致 |
| **F-5 写入仅 allow** | 治理 | 写入仅请求写入节所列，INDEX 与 VERSION 零触碰 |

## 五、必读文件 {#read}

- 必读 1：sih-math/calculus/llm-friendly-build/entries/ 波一已改条目即定义首节形态
- 必读 2：sih-engine/sih/event/plan/mathrefmt-solo-results.md 波一变换语义与误差申报
- 必读 3：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 只做波一同款变换，禁任何正文语义改写
2. 认证先落主树活链，链 settle 前一次性拷工地，严禁工地链副本追加
3. 撞锁有限重试如实计数，与并发批让位后重跑枚举
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-engine/src/scrutinator/tests.rs（仅金向量期望随冻）
- sih-engine/sih/state/plan/mathrefmt2-solo.md
- sih-engine/sih/event/plan/mathrefmt2-solo-results.md
- sih-engine/sih/event/plan/mathrefmt2-solo-materials/
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
- [ ] 冲突样本节在结果档（含与 mathquote-calc 写面重叠实测）
- [ ] 认证入链双仓结算收约对表读数在档
