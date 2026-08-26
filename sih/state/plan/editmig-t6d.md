# editmig-t6d：编辑位迁移批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户批准令、DEC-011 终裁一、DEC-009 终裁时点、意图记录 2026-08-27-ask3-editmig
> 范式：T6-D 范式——本任务包偏离：工作流迁移类，主线串行，无子代理，本批文档编辑首走副本流
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 每批结果档重复登记同一偏离即编辑位在主检出副本空转，两把锁对主分支文件的保护半兑现。
- lease commit 不认副本路径即正身提交到副本会吃他仓拦，副本流断在提交段。
- 副本流的归并依赖即 close 归并腿已于 1.1.1 修复，前置件已备。

## 二、关键设计 {#design}

### 2.1 副本别名 {#alias}

commit_staged 认会话仓条目的 worktree 路径为别名，范围验按其正身仓映射，外会话副本仍拦。base 即副本提交取 base_branch 加 merge-base 短号。

### 2.2 副本流首走 {#dogfood}

本批两仓文档编辑在副本落笔，管线核副本件，段结算经 lease commit 从副本提交，收约归并回家即首笔真实归并。

## 三、工作清单 {#work}

### Cluster 1：实施（先于立约落笔如实登记）

commit_staged 别名与 base 自算、新增副本提交与外副本拒两测试、契约修订八、版本 1.2.0。

### Cluster 2：副本流文档（主线写在副本）

契约与包档与结果档在副本编辑，管线自副本路径核。

### Cluster 3：段结算与收约（正身路径在副本）

双仓 lease commit 从副本提交，解锁收约归并回家。

## 四、判据 {#criteria}

- L1 自家副本提交放行即 message 三件机械生成且 base 为 base_branch 加 merge-base 短号
- L2 外会话副本拦即 repo_not_in_session
- L3 既有测试不破即全套绿
- L4 契约修订八落字升 1.2.0
- L5 本批两仓文档编辑零主检出写即 diff 只在副本分支
- L6 管线绿认证上链链 valid
- L7 收约归并成即两仓 merge 提交落基线
- L8 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 editmig-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-tools/lease/
- sih-tools/scribe/
- sih-engine/sih/state/plan/editmig-t6d.md
- sih-engine/sih/state/plan/editmig-t6d-results.md
