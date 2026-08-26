# gov004-commit-takeover-t6d：git commit 司衡接管立界批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户批准 GOV-004 建议方案含六待裁点、DEC-009 终裁时点已到、DEC-011 终裁一二已签、租约 1.0.0 在役
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：立界类，主线串行为主**
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- git commit 司衡接管方案已获批未立界即方向页缺位、终裁决策缺位、DEC-011 手动期条款升格缺位。
- 接管五面即通道唯一、message 机械生成、提交界与认证门同界、历史不可改起点、反绕行机制均停在对话层。
- 历史不可改起点未成立即签署前压缩窗口仍开放，立界时点即窗口关闭时点。

## 二、关键设计 {#design}

### 2.1 GOV-004 方向页 {#gov-page}

每线一向界即 git commit 司衡接管线一页方向页，承载向界定义、五钉位、分期、版本与固定。方向与边界在本页，终裁细节在 DEC-012，冲突以 DEC-012 为准。

### 2.2 DEC-012 七件裁决 {#dec-rulings}

裁决一接管域与通道唯一即租约在册域内 worktree 新建销毁唯一合法通道为 lease open 与 close，直改正本检出即违例，编辑位迁移并入此界。裁决二 message 机械生成三形态即检查点与段结算与收约归并，语义源为机械件，偏离行唯一半自由位。裁决三提交界与认证门同界即一段一结算，管线绿加认证落链为前置，段内检查点自由，禁 amend 与 rebase。裁决四手动期升格即提交执行权交 lease commit 与对表两子命令，显式修订 DEC-011 终裁一内在自限款。裁决五主线历史粒度即归并保留分支全史。裁决六历史不可改起点即双签时点为治理域历史改写终点。裁决七反绕行即正身路径唯一加三方对表。

### 2.3 DEC-011 修订四 {#dec011-revision}

终裁一手动期内在自限款升格为工具承载，随 DEC-012 签署联动生效，两期安排不变。

## 三、工作清单 {#work}

### Cluster 1：起草（副本内写）

GOV-004 方向页一页、DEC-012 决策集七件、DEC-011 修订四一行、结果档一页。

### Cluster 2：管线与认证（主线串行验证）

化格落笔、核阅 des-001、检词 core 三绿，报告落 scribe/reports 并逐件认证上链。

### Cluster 3：段结算提交（副本内写）

分支段结算提交用批准的机械 message 模板即首次自我实践，呈现待签。

## 四、判据 {#criteria}

- L1 GOV-004 一页承五钉位与分期与版本固定，核阅绿
- L2 DEC-012 七件裁决各含决策内容与决策理由，核阅绿
- L3 DEC-011 修订四一行随签联动表述准确，核阅绿
- L4 三件文档化格核阅检词零违例
- L5 认证事件三件上链且链 valid
- L6 段结算提交 message 用机械模板含会话号与认证哈希
- L7 全程租约在册即锁覆盖全部写点
- L8 待签标记在即签署前不生效

## 五、结果留档 {#results}

结果见 gov004-commit-takeover-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/task-packages/gov004-commit-takeover-t6d.md
- sih-engine/task-packages/gov004-commit-takeover-t6d-results.md
- sih-engine/doc/governance/GOV-004-commit-takeover-v1.md
- sih-engine/doc/decision/012-git-commit-takeover.md
- sih-engine/doc/decision/011-locks-and-package-binding.md
- sih-tools/scribe/
