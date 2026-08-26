# lease11-t6d：lease 1.1 实施批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户 lease 实施批开工令、DEC-012 裁决四与裁决七、DEC-011 修订四、意图记录 2026-08-26-ask3-lease11
> 范式：T6-D 范式——本任务包偏离：单工具实施类，主线串行，无子代理
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- DEC-012 已签即手动期提交执行须有正身路径，lease 1.0 无 commit 子命令，提交仍经人手 git commit。
- 反绕行机制须有三方对表载体，git log 对会话台账对 trail 的分类器缺位。
- 归位批后 resolve_package 缺省位指已退役的顶层 task-packages。

## 二、关键设计 {#design}

### 2.1 commit 四验 {#commit}

会话在册验、范围验即暂存件全落 allow、认证在链验即 settle 必挂 cert 前缀命中 trail 的 certification_completed event_hash、信息机械生成即 build_message 模板单源三形态。拦截理由码五件。base 由工具自算。

### 2.2 reconcile 三方对表 {#reconcile}

分类五态即 routed、routed_merge、session_orphan、cert_missing、unrouted。只报不拦，first_routed 分界存量与增量，污界退出码一。

### 2.3 自举 {#bootstrap}

本批提交首次经 lease commit 正身路径落地即工具承载自身的段结算，先 wip 后 settle。

## 三、工作清单 {#work}

### Cluster 1：实施（已先行落笔即偏离一）

commitcore 模块、CLI 两子命令、resolve_package 缺省位、版本 1.1.0。

### Cluster 2：测试（已先行落笔）

二十六测全绿含新增六件即 wip 路由、settle 认证门、范围与暂存拦、他仓拦、对表分类、净界零退出。

### Cluster 3：契约与留痕（主线写）

契约修订六、lease CALL-LOG、scribe CALL-LOG、根 AGENTS 两处即 pk-006 旧句同步与文件索引租约条目。

### Cluster 4：管线与认证与段结算（主线串行验证）

化格核阅检词对 CONTRACT 与包档，报告落盘逐件上链，lease commit 自举两提交，会话收约。

## 四、判据 {#criteria}

- L1 commit 四验拦放正确即五理由码各就位与绿路径放行
- L2 reconcile 分类正确即五态判类与净污退出码
- L3 测试二十六全绿
- L4 契约修订六落字即八子命令与两段机器形态与 L9 L10
- L5 本批提交经 lease commit 即 message 含 session 与 cert 与 base 机械三件
- L6 管线绿认证上链链 valid
- L7 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 lease11-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-tools/lease/
- sih-engine/sih/state/plan/lease11-t6d.md
- sih-engine/sih/state/plan/lease11-t6d-results.md
- sih-tools/scribe/
- AGENTS.md
