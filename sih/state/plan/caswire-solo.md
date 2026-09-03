# caswire-solo：cascade 级联检查载体接线批（mathpipe-full 程序批四起）

> task-packages 治理任务
> 承接：mathpipe-full-program-v1.md 批四节即 cascade 对挂 ORD-016 良基终止；批一账本 rev1 可指认未实例化件；四件套形制全承 ordwire-lease-solo 先例（lease 对挂 ORD-020 已毕）
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械即机械链全绿即放行
> 用户裁定 2026-09-03 冲突行照录「链上冲突，可以插队，并不存在太大的冲突点」——链追加面插队合法，收约 trail 冲突取并集超集机械解，严禁非书简通道改写链文件
> 队形：单线形 solo
> 日期：2026-09-03
> 温故检索：materials/recall-caswire.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，与并行批共享链/台账/调用册面

## 一、问题陈述 {#problem}

rev1 账本坐实 cascade 级联检查为可指认未实例化件：cascade 即路径即 id 不改名即删除新建与上游洁净不变式的乐观链验基线，判定面即上游洁净判定与基线比较当前裸奔无载体引用。本批将级联判定语义与 ORD-016 载体接线。

## 二、关键设计（四件套，形制承 ordwire-lease-solo） {#design}

1. 载体引用：mapping.md:190「倒推终止与良基」→ sih-math/order/entries/ORD-016-well-founded-relation-and-backward-termination.md 磁盘实存核验。
2. 推导档：sih-math/docs/caswire-cascade-derivation-<实日>.md，承载级联判定语义形式化即上游依赖倒推遍历的良基性与终止性（> 关系无无穷下降链即遍历必停）与洁净判定逐层传播，随批入版控。
3. 接线：cascade CONTRACT 增载体引用节（## 载体引用 {#carrier}）+ 源码判定位注释锚点（上游洁净判定、基线比较、倒推遍历各位，落位以 CONTRACT 与源码实读为准）。
4. 金向量：两场景机械重放逐字节一致即洁净链全过与污染链在污染上游处拒。

## 三、工作清单 {#work}

- [ ] 载体四件套齐
- [ ] cascade 既有测试全绿零行为变更
- [ ] 金向量双跑逐字节一致
- [ ] 三步管线读数
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 载体四件套** | 工程 | 引用与推导与接线与金向量四件齐，ORD-016 锚点磁盘实存 |
| **F-2 行为零变更** | 工程 | cascade 既有测试全绿，change_type=wiring_only_no_behavior_change |
| **F-3 金向量重放** | 工程 | 两场景机械重放逐字节一致 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-math/order/entries/ORD-016-well-founded-relation-and-backward-termination.md
- 必读 2：sih-engine/sih/event/plan/ordwire-lease-solo-results.md 四件套形制先例
- 必读 3：sih-tools/cascade/CONTRACT.md
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 只加引用注释与契约节，不改判定谓词
2. 认证先落主树活链即插队合法，链 settle 前一次性拷工地，严禁工地链副本追加
3. 收约 trail 冲突取并集超集且 scribe verify 过为唯一放行形
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-math/docs/
- sih-tools/cascade/
- sih-engine/sih/state/plan/caswire-solo.md
- sih-engine/sih/event/plan/caswire-solo-results.md
- sih-engine/sih/event/plan/caswire-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 冲突样本节在结果档（含插队实测读数）
- [ ] 认证入链双仓结算收约对表读数在档
