# mathpipe-a3-solo：gauge 期票清偿批（mathpipe-full 程序批三）

> task-packages 治理任务
> 承接：mathpipe-full-program-v1.md 批三节与 pk-041 路线档第一档，用户令委外即冲突模式并发启动
> 队形：单线形 solo——确定性代码与主线亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-a3.json 双档零命中如实记
> 冲突测试：本批为 pk-045 多 agent 冲突测试样本库参与者，结果档须载冲突样本节

## 一、问题陈述 {#problem}

gauge 秤星契约登记两张期票即 PROB-003 置信带与 PROB-005 贝叶斯语义，读数事件 schema 待增量扩展即公式版本升 ga-2。载体已在数学仓在册即 rev1 账本可指认未实例化件，属程序批三与路线档第一档三项中的秤星控制图前置。

## 二、关键设计 {#design}

1. 载体接线：全读 sih-math/llm-friendly-build/mapping.md 定 PROB-003 与 PROB-005 条目，推导档承载置信带公式与贝叶斯更新语义，接线 sih-tools/gauge 计算核，公式版本升 ga-2 即读数事件 schema 增量扩展，ga-1 旧读数回放兼容。
2. 金向量：ga-2 公式输出冻结与复算逐字节一致；ga-1 回放同判。
3. 判变申报：升版若改变既有读数判读逐件申报，零静默。

## 三、工作清单 {#work}

- [ ] 载体定位与推导档落 sih-math/docs/
- [ ] gauge 计算核接线与 ga-2 schema
- [ ] 金向量与 ga-1 回放兼容读数
- [ ] 推导档与变更件走化格核阅检词三步
- [ ] 认证上链多仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 载体推导在案** | 工程 | PROB-003 与 PROB-005 条目实存，推导档在册锚点磁盘实存 |
| **F-2 金向量一致** | 工程 | ga-2 输出冻结复算逐字节一致，ga-1 旧读数回放同判 |
| **F-3 判变零静默** | 治理 | 判变逐件申报或如实记零 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-math/llm-friendly-build/mapping.md 全读
- 必读 2：sih-tools/gauge/ 契约与计算核
- 必读 3：sih-engine/sih/event/plan/mathpipe-full-program-v1.md 批三节
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 认证一律先落主树活链即 meter 包裹引擎 scribe append 主链绝对路径，链文件只在 settle 前一次性拷入工地，严禁在工地链副本追加事件（2026-09-03 链分叉教训承 pk-045）
2. 撞他会话锁即有限重试并如实计数入冲突样本节，不绕行
3. 收约让位走备份让位归并对表法，diff 非 identical 即停批上报
4. 词债不过夜，findings 亲读

## 七、请求写入 {#requested-writes}

- sih-tools/gauge/
- sih-math/docs/
- sih-engine/sih/state/plan/mathpipe-a3-solo.md
- sih-engine/sih/event/plan/mathpipe-a3-solo-results.md
- sih-engine/sih/event/plan/mathpipe-a3-solo-materials/
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
- [ ] ga-2 在案且 ga-1 回放兼容
- [ ] 冲突样本节在结果档逐件
- [ ] 认证入链，多仓结算收约，对表读数在档
