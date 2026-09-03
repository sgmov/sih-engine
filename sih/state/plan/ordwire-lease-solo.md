# ordwire-lease-solo：lease 锁机制载体接线批（mathpipe 批四起首件）

> task-packages 治理任务
> 承接：mathpipe-full-program-v1.md 批四节对挂表与 ledger-rev1 可指认未实例化件，用户令委外即冲突模式并发启动
> 队形：单线形 solo——确定性代码与主线亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-ordwire.json 双档零命中如实记
> 冲突测试：本批为 pk-045 多 agent 冲突测试样本库参与者，结果档须载冲突样本节

## 一、问题陈述 {#problem}

rev1 账本坐实 lease 为可指认未实例化件即程序档对挂 ORD-020 全序资源分配与死锁自由在数学仓在册而 lease 源码零引用。按 M-1 判定性常数与判据须携带载体引用与推导档，锁机制的判定面即撞锁拒绝与等待重试语义当前裸奔。

## 二、关键设计 {#design}

1. 载体接线：全读 mapping.md 定 ORD-020 条目，推导档承载 lease 锁判定语义的形式化即互斥即活锁死锁不自由即等待终止性，接线即推导档落 sih-math/docs/ 加 lease CONTRACT 增引用节加源码判定位注释锚点，逐件四件套即载体引用、推导档、接线、金向量（金向量即撞锁与让路两场景的机械重放）。
2. 范围纪律：不改 lease 判定行为即本批是接线与推导不是行为变更；若推导发现行为与载体不符即停批申报属实现偏离，不静默改行为。

## 三、工作清单 {#work}

- [ ] ORD-020 载体定位与推导档
- [ ] lease CONTRACT 引用节与源码锚点
- [ ] 金向量即撞锁与让路场景机械重放
- [ ] 推导档与变更件三步
- [ ] 认证上链多仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 载体四件套** | 工程 | 载体引用与推导档与接线与金向量四件齐，ORD-020 锚点磁盘实存 |
| **F-2 行为零变更** | 工程 | lease 既有测试全绿，判定行为 diff 为零或偏差即停批申报 |
| **F-3 金向量重放** | 工程 | 撞锁与让路两场景机械重放逐字节一致 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-math/llm-friendly-build/mapping.md 全读与 order 子仓 ORD-020 条目
- 必读 2：sih-tools/lease/ 契约与锁核心
- 必读 3：sih-engine/sih/event/plan/mathpipe-full-program-v1.md 批四节
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 认证一律先落主树活链，链文件只在 settle 前一次性拷入工地，严禁工地链副本追加（pk-045 教训）
2. 撞他会话锁即有限重试如实计数入冲突样本节，不绕行
3. 收约让位走备份对表法，非 identical 即停批上报
4. 词债不过夜，findings 亲读

## 七、请求写入 {#requested-writes}

- sih-tools/lease/CONTRACT.md
- sih-tools/lease/src/lease/
- sih-math/docs/
- sih-engine/sih/state/plan/ordwire-lease-solo.md
- sih-engine/sih/event/plan/ordwire-lease-solo-results.md
- sih-engine/sih/event/plan/ordwire-lease-solo-materials/
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

- [ ] F-1 至 F-4 全过
- [ ] 冲突样本节在结果档逐件
- [ ] 认证入链，多仓结算收约，对表读数在档
