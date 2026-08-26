# dual-sign-t6d：双组签署批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户签署令、GOV-004 v1 与 DEC-012 与 DEC-011 修订四待签组、DEC-001 修订二三四待签组
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：签署类，主线串行为主**
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- 两组待签文档待人节点令已到达即 GOV-004 v1 与 DEC-012 七件裁决与 DEC-011 修订四联动、DEC-001 修订二三四。
- 待签标记未清除即签署生效无文档形态，历史不可改起点未落链。
- structrev 结果档提交号占位待补记，AGENTS 文件索引缺接管与结构条目。

## 二、关键设计 {#design}

### 2.1 标记清除与生效记录 {#sign}

GOV-004 版本节改签署生效并记历史不可改起点。DEC-012 引言改签署生效加修订二转裁定。DEC-001 加修订五即修订二三四转裁定。DEC-011 修订四自declare联动生效不改正文。

### 2.2 连带同步 {#sync}

structrev 结果档补记提交号与消费闭环声明。AGENTS 文件索引增接管条目一行。

## 三、工作清单 {#work}

### Cluster 1：签署编辑（主线写）

五处编辑即 GOV-004 版本节、DEC-012 引言与修订二、DEC-001 修订五、structrev 结果档补记、AGENTS 增行。

### Cluster 2：管线与认证（主线串行验证）

化格核阅检词三绿，报告落盘逐件上链。

### Cluster 3：段结算提交（主线写）

机械 message 提交即签署落链，会话收约。

## 四、判据 {#criteria}

- L1 两组待签标记全清即无残留待签字样
- L2 历史不可改起点在 GOV-004 与 DEC-012 双处落字
- L3 DEC-011 正文零改动即联动生效条款自持
- L4 管线绿认证上链链 valid
- L5 机械 message 动态取 cert 哈希
- L6 全程租约在册锁覆盖写点含 AGENTS 行
- L7 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 dual-sign-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/task-packages/dual-sign-t6d.md
- sih-engine/task-packages/dual-sign-t6d-results.md
- sih-engine/doc/governance/GOV-004-commit-takeover-v1.md
- sih-engine/doc/decision/012-git-commit-takeover.md
- sih-engine/doc/decision/001-repository-structure.md
- sih-engine/task-packages/structrev-001-t6d-results.md
- AGENTS.md
- sih-tools/scribe/
