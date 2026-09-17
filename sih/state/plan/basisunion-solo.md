# basisunion-solo:依据族并集判据修订与 watchcheck F-5 复算落定

> 令源:用户 2026-09-06 裁定原话「watchcheck 改题文本不需要改，只要改判定，依据只要是基线，无论是哪个，都是同一个语义的判决。」即依据族语义同一裁定：引用落合法基线集内任一基线俱为同一语义判决，基线集内分散不构成不共识
> 范式:T6 单线 solo,委外代理亲写零子代理(主会拉子代理执行)
> 域:sih-tools/facet 判据面

## 一、问题陈述 {#problem}

- maturation_gate.py basis_consensus 判据现为 distinct_basis <= 1 硬共识,机制类命题天然多条基线同时成立(确定性程序即基线一、只呈报零 LLM 即基线五、读台账链面即基线四),考卷强制三选一即产生合法分散
- 实证三件:pk-058 在泊承载连续五件 near_threshold 成因恒为依据族双源;pk-046 件二 gvec-method-guard-1 依据族三值分散 boundary 打回;watchcheck round1 九发全 comply 零变卦而 3/3/3 分散挂 basis_consensus 落 boundary,F-5 停批候裁即用户本裁定之由
- 用户已裁语义:题目不改,判定改——依据只要是基线(合法集内),无论哪个,同一语义判决

## 二、关键设计 {#design}

### 2.1 判据语义修订

- 旧:distinct_basis <= 1 即九发引用唯一基线才过
- 新:引用基线全部落在合同声明的 basis_regulation 合法枚举集内即过(集内语义同一,承用户裁定);引用落集外、缺失、或无法解析仍挂
- 合法枚举集以合同 contract.json 各发 system_prompt 声明的 basis_regulation 枚举为准(现状 baseline_1/4/5),不新造常数不裸奔
- 边界旗判据、decision 稳定判据、foregrounding 判据零动

### 2.2 watchcheck round1 复算落定

- 九发原始 responses 零改动零重采样,以新判据复算 gate verdict(预期 boundary 翻 stable_clear:3/3/3 全落合法集)
- 复算翻转即 watchcheck-solo F-5 落定,执契链(attractor check 与 verify 与 sign)随即跑,终签在链
- 复算不翻转(预期外)即停批如实申报,不得回滚判据迁就

### 2.3 判据变更自裁

- 判据语义变更按 gateswitch 先例过得一裁(facet 合同模式),near_threshold 呈用户转主会
- 判据版本号升版在档(criteria_version 注记),金向量若涉刷新按 findings-unchanged-only-hashes 先例

## 三、工作清单 {#work}

- [ ] T-1 basis_consensus 新判据实装(集合成员语义,枚举源合同声明)
- [ ] T-2 TDD 先红后绿:集内分散过(红:旧判据挂)、集外引用挂、缺失挂
- [ ] T-3 watchcheck round1 复算(材料只读消费,零重采样),verdict 翻转在档
- [ ] T-4 watchcheck F-5 执契链三步(check/verify/sign),终签在链
- [ ] T-5 判据变更自裁(facet 合同模式)
- [ ] T-6 既有判据族测试全绿零回归(含 gateswitch test 判据族)

## 四、可证伪条件(跑前立文) {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 新判据 | 集内分散即过、集外即挂、缺失即挂,三态夹具先红后绿 |
| **F-2** | 复算翻转 | watchcheck round1 按新判据复算 boundary 翻 stable_clear,原始 responses 逐字节零改动 |
| **F-3** | F-5 落定 | watchcheck 执契终签在链,结果档补记 F-5 过 |
| **F-4** | 自裁 | 判据变更 facet 合同模式过得一裁 stable_clear 过执契,near_threshold 呈用户 |
| **F-5** | 零回归 | facet 判据族与既有测试全绿 |

## 五、必读文件 {#read}

- 用户裁定:本包令源行(原话在档)
- 判据源位:sih-tools/facet/probes/maturation_gate.py(basis_consensus 现实现与 criteria_version)
- 复算对象:sih-engine/sih/event/plan/watchcheck-solo-materials/facet-round1/(contract.json 与 responses.jsonl 与 gate-recompute.json)
- 先例:gateswitch-solo 结果档(判据切换全链)与 pk-058 泊材料(机制类命题依据族观察)
- 机械链:sih-tools/BATCH-FACE.md(含全部勘误节)

## 六、约束 {#constraints}

1. 判据变更自裁过得一裁,near_threshold 呈用户转主会,不自行终签
2. watchcheck 原始 responses 零改动零重采样,复算只读消费
3. 合法基线枚举以合同声明为准,不新造判定常数,零裸奔
4. 与 gvec-v2-serial 并行避让:facet 面现被他批独占,open 预检拦即 wait-turn 排队候位不抢不绕
5. TDD 先红后绿;scribe 写入裸调逐笔 grep 验证禁 meter 包裹掩败
6. 判据只改 basis_consensus 一处,其余四判据零动

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle + close + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/basisunion-solo-results.md 与 materials/
- [ ] watchcheck 结果档 F-5 行补记与 CONTRACT 修订与 CALL-LOG 双笔随批

## 八、风险点 {#risks}

- 复算不翻转(集外引用或解析缺失混入)即停批申报,不回滚判据
- 金向量刷新若涉,按 findings 零变化只刷哈希先例,断言逻辑零改
- facet 面锁排队时长不定,wait-turn 超时即如实回报不硬等

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游停批:watchcheck-solo 结果档 F-5 行(boundary 候裁)
- 同族观察:pk-058(在泊)、pk-046 件二(在泊)
- 判据族先例:gateswitch-solo

## 十一、请求写入 {#requested-writes}

- sih-tools/facet/probes/maturation_gate.py —— basis_consensus 判据修订与 criteria_version 升版注记
- sih-tools/facet/tests/ —— 新判据三态测试
- sih-engine/sih/event/plan/basisunion-solo-results.md 与 materials/
- sih-engine/sih/event/plan/watchcheck-solo-results.md —— F-5 行补记(复算与终签读数)
- sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/2026-09-06.ndjson 与后续日链
