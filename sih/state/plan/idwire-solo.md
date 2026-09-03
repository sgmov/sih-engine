# idwire-solo：identity 正身载体接线批（mathpipe-full 程序批四起）

> task-packages 治理任务
> 承接：mathpipe-full-program-v1.md 批四节即 identity 对挂 ALG-002 商集隔离加 PROB-013 变点检测；批一账本 rev1 可指认未实例化件；四件套形制全承 ordwire-lease-solo 先例
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械即机械链全绿即放行
> 用户裁定 2026-09-03 冲突行照录「链上冲突，可以插队，并不存在太大的冲突点」——链追加面插队合法，收约 trail 冲突取并集超集机械解，严禁非书简通道改写链文件
> 队形：单线形 solo
> 日期：2026-09-03
> 温故检索：materials/recall-idwire.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，与并行批共享链/台账/调用册面

## 一、问题陈述 {#problem}

identity 即身份串 v3 十二件组件加盐 SHA-256，判定面即身份等价判定（canonical 代表）与漂移监控（跨时点身份串比对），当前裸奔无载体引用。ALG-002 承载等价关系与商集隔离即身份串为等价类代表且组件换序不变性；PROB-013 承载平稳性与变点检测即漂移监控的检测语义。本批将正身判定语义与双载体接线。

## 二、关键设计（四件套，形制承 ordwire-lease-solo，双载体各接各面） {#design}

1. 载体引用：mapping.md:197「选择性隔离与防污」→ ALG-002、mapping.md:203「未形之变的微兆检测」→ PROB-013，双条目磁盘实存核验。
2. 推导档：sih-math/docs/idwire-identity-derivation-<实日>.md，承载身份串语义形式化即组件集上等价关系定义与 canonical 代表选取即商集隔离（换序同串）与漂移比对即变点检测语义（何谓身份平稳与何谓变点即组件哈希逐位对表），随批入版控。
3. 接线：identity CONTRACT 增载体引用节（双载体分面即等价判定面挂 ALG-002、漂移监控面挂 PROB-013）+ 源码判定位注释锚点（哈希合成位、组件序归一位、漂移比对位）。
4. 金向量：两场景机械重放逐字节一致即同组件换序同串（等价类代表稳定）与异组件异串且漂移位指认（变点定位）。

## 三、工作清单 {#work}

- [ ] 载体四件套齐（双载体）
- [ ] identity 既有测试全绿零行为变更
- [ ] 金向量双跑逐字节一致
- [ ] 三步管线读数
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 载体四件套** | 工程 | 双载体引用与推导与接线与金向量齐，ALG-002 与 PROB-013 锚点磁盘实存 |
| **F-2 行为零变更** | 工程 | identity 既有测试全绿，change_type=wiring_only_no_behavior_change |
| **F-3 金向量重放** | 工程 | 两场景机械重放逐字节一致 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-math/algebra/entries/ALG-002-equivalence-relation-and-quotient-isolation.md 与 probability/entries/PROB-013-stationarity-and-change-point.md
- 必读 2：sih-engine/sih/event/plan/ordwire-lease-solo-results.md 四件套形制先例
- 必读 3：sih-tools/identity/CONTRACT.md
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 只加引用注释与契约节，不改哈希与判定逻辑
2. 认证先落主树活链即插队合法，链 settle 前一次性拷工地，严禁工地链副本追加
3. 收约 trail 冲突取并集超集且 scribe verify 过为唯一放行形
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-math/docs/
- sih-tools/identity/
- sih-engine/sih/state/plan/idwire-solo.md
- sih-engine/sih/event/plan/idwire-solo-results.md
- sih-engine/sih/event/plan/idwire-solo-materials/
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
