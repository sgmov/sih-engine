# reconfirst-t6d：对表首跑批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户批准令、DEC-012 裁决七、意图记录 2026-08-27-ask3-reconfirst
> 范式：T6-D 范式——本任务包偏离：审计清点类，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 反绕行闸门已立即 commit 正身路径在役，但对表从未正式跑过，存量基线缺位，任何一笔新提交说不清是否绕行。
- 两仓全史的存量规模与首路由界只有冒烟数无落档数。

## 二、关键设计 {#design}

### 2.1 两仓全史对表 {#run}

lease reconcile 对 sih-engine main 与 sih-tools integral-stage-build 各跑全史，五态分类，报告 JSON 落 scribe reports。

### 2.2 清点报告 {#report}

audit-049 承 audit-048 序，载两仓数字、首路由界、存量口径声明、封存裁定请求。

## 三、工作清单 {#work}

### Cluster 1：对表执行（主线读）

两仓 reconcile 全史，报告落盘。

### Cluster 2：清点报告与管线（主线写与串行验证）

audit-049 落 sih/event/audit，化格核阅检词，报告落盘逐件上链。

### Cluster 3：段结算与收约（主线写）

lease commit 段结算，会话收约，台面呈数与裁定请求。

## 四、判据 {#criteria}

- L1 两仓 reconcile 报告落盘且五态数字与 git 实态一致
- L2 audit-049 落 sih/event/audit 即序承 audit-048
- L3 首路由界两仓各钉一笔
- L4 管线绿认证上链链 valid
- L5 提交经 lease commit 正身路径
- L6 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 reconfirst-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/sih/event/audit/
- sih-engine/sih/state/plan/reconfirst-t6d.md
- sih-engine/sih/state/plan/reconfirst-t6d-results.md
- sih-tools/scribe/
