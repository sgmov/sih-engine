# lease-Reintegration-solo：租约融回三步曲第一步——SDD 落差规格批

> T6D-XX task-packages 治理任务
> 承接：用户 2026-09-13 令「租约启动融回」；DEC-013 融回门机制（复用条款：下一件融回即租约）；SPEC-013/014/015 三件融回落差规格先例
> 范式：T6-D 范式 —— 本任务包偏离：实施类单线 solo，材料取经双子代理并行（ Explore 两路取材已毕），规格写件与治理链主会亲写
> 日期：2026-09-13

## 一、问题陈述 {#problem}

- 用户 2026-09-13 令租约启动融回。融回门开关归治理程序与用户批令留痕（DEC-013 门开条款），本批即批令落痕与三步曲第一步。
- 围堰件现状：sih-tools/lease 1.46.0（CONTRACT 修订六十二），约 7800 行源码、381 测、19 顶层子命令加 call-log 五子命令，远超前四件融回体量。
- 已知待清债九处散档（closeidem、lockguard、mem-impl、redtest、scribeback、wenguwire 六结果档），从未归集。
- 观测面缺陷（如实申报）：meter 计量面 2026-09-11 后断流且历史只包 scribe，gd-2 承重计数对租约为零——融回评估的判据面数据不完整，本批在规格中如实登记此缺陷，不宣称判据全绿。

## 二、关键设计 {#design}

### 2.1 交付物

SPEC-024-lease-mergeback-gap-v1.md，沿 SPEC-013/014/015 共同骨架：概览／家位与模块形／接口契约对表（19+5 子命令逐件）／双模并存条款／腿切分清单／验收判据 A1-An／金向量同参形条款／回迁债（含 DEC-001 围堰归位映射行）／测试计划／边界／修订记录／内容充分性。

### 2.2 本批只做 SDD，不做实装

融回三步曲每步独立开批（DEC-013 复用条款）。本批交付落差规格，TDD 批与切换批候另令。

### 2.3 两个必答抉择点的处置

- 台账家位：DEC-001 已裁方向（「状态层登记族……归位动作融回接引擎状态层登记」），具体落位由本规格提案（候选 sih/state/ledger/），标候选待裁不宣称已裁。
- 实装形态：DEC-013 第一步「引擎侧是开发非移植」+「组件以 Rust 落 src」已裁 Rust 原生；mcpserver passthrough 层去留入双模并存条款处置。

## 三、工作清单 {#work}

### Cluster 1：主会亲写

- [x] 双子代理取材（围堰盘点、待清债归集、SDD 模板、承接面、CONTRACT 调用册）
- [ ] SPEC-024 落差规格写件
- [ ] 管线三步（化格→核阅→检词）
- [ ] 认证上链、结算收约

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 待清债归集完备 | 数据治理 | 六结果档九处归租约融回批表述逐条入规格回迁债节，漏一处即败 |
| **F-2** 骨架对表 | 元层工具 | SPEC-024 节标题覆盖 SPEC-013/014/015 共同骨架全节 |
| **F-3** 管线零违规 | 元层工具 | 化格 0/1、核阅 0、检词 0 三步退出码全过 |

## 五、必读文件 {#read}

- sih-engine/doc/decision/013-mergeback-gate.md（融回门正典）
- sih-engine/doc/spec/SPEC-013/014/015（骨架先例）
- sih-tools/lease/CONTRACT.md（围堰契约 1.46.0）

## 六、约束 {#constraints}

1. 本批零实装：sih-engine/src 零触碰，TDD 批另开。
2. 观测缺陷如实申报：gd-2 计量断流不粉饰。
3. 命名零新增：租约已立名，名随物走（DEC-013），不另起。

## 七、验收标准 {#acceptance}

本任务包验收 = 3 项：

- [ ] SPEC-024 在档且管线三步全过
- [ ] 认证链笔 valid
- [ ] lease close 收约过 SDDG 门

## 八、请求写入 {#requested-writes}

- sih-engine/doc/spec/SPEC-024-lease-mergeback-gap-v1.md
- sih-engine/sih/state/plan/lease-Reintegration-solo.md
- sih-engine/sih/event/plan/lease-Reintegration-solo-results.md
- sih-engine/sih/event/trail/（append）
