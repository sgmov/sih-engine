# closefix-t6d：pk-022 出泊修复批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户优先解决令、pk-022 白皮书批停泊缺陷、意图记录 2026-08-26-ask3-closefix
> 范式：T6-D 范式——本任务包偏离：缺陷修复类，主线串行，无子代理
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- close 归并腿 merge 调用返回码被吞，主线同名未跟踪文件阻塞归并时报 removed 实未归并未删支即假绿。
- 白皮书批真会话即现场，手工补归并 4adc597 后缺陷入泊 pk-022。
- 拆本先于归并的次序使失败后自愈面残缺。

## 二、关键设计 {#design}

### 2.1 改序与全检 {#reorder}

归并先行于拆本。每仓序即锁清零前置、ahead 计数、有提交即 merge --no-ff 返回码全检、失败计 merge_failed 不吊销、拆本返回码检、删支返回码检、全过方 removed。任一失败 revoked 为假留残自愈，missing 容忍承前判。

### 2.2 出泊 {#exit}

pk-022 出泊事件上链，材料移除投影改写，修复即处置形态。

## 三、工作清单 {#work}

### Cluster 1：出泊与立约（主线写）

pk-022 出泊记录、双仓立约、锁。

### Cluster 2：修复实施（主线写）

close_session 改序与全检、新增阻塞拦与自愈复跑测试、契约修订七、版本 1.1.1。

### Cluster 3：管线与认证与段结算（主线串行验证）

化格核阅检词对 CONTRACT 与包档，报告落盘逐件上链，双仓 lease commit 段结算，会话收约。

## 四、判据 {#criteria}

- L1 阻塞场景拦即主线未跟踪文件撞副本提交路径时 close 退出码一计 merge_failed 分支在工地在
- L2 自愈复跑即清障后重跑 close 归并成删支成吊销成
- L3 既有收约两态与脏工地自愈测试不破
- L4 契约修订七落字即归并先行与三失败态
- L5 版本 1.1.1
- L6 管线绿认证上链链 valid
- L7 双仓提交经 lease commit 正身路径
- L8 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 closefix-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-tools/lease/
- sih-tools/parking/
- sih-tools/scribe/
- sih-engine/sih/state/plan/closefix-t6d.md
- sih-engine/sih/state/plan/closefix-t6d-results.md
