# corr012-t6d：修正批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户修正批开工令、DEC-012 修订二签署态、意图记录 2026-08-26-ask3-corr012
> 范式：T6-D 范式——本任务包偏离：单点更正类，主线串行，无子代理
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- 双签当晚「压缩」一词被误读为 git 历史压缩，DEC-012 裁决六内落下一句失实事实断言即「即用户现准备的 git 历史压缩」，用户本意为会话上下文压缩。
- 全仓 grep 核验失实句唯一即 sih-engine/doc/decision/012-git-commit-takeover.md 第 60 行单句，GOV-004 与 DEC-011 与 DEC-001 现行文本无同类断言。
- 签署裁定文本承载失实陈述破坏留痕可审计性。

## 二、关键设计 {#design}

### 2.1 单句更正 {#fix}

裁决六内删除「即用户现准备的 git 历史压缩」半句，句子落为「签署前压缩与整理为人类最后整理权。」。规则本体不动，裁决内容零变化。

### 2.2 修订三落字 {#revision}

修订记录新增修订三行，载明失实来源即误读与更正范围即单句单文件，修订一二原文不动。

## 三、工作清单 {#work}

### Cluster 1：更正编辑（主线写）

DEC-012 裁决六单句更正加修订三落字，单文件两写点。

### Cluster 2：管线与认证（主线串行验证）

化格核阅检词三绿，报告落盘逐件上链，结果档核阅域外如实记录。

### Cluster 3：段结算提交（主线写）

机械 message 提交即更正落链，会话收约。

## 四、判据 {#criteria}

- L1 失实句全仓清零即 grep 现准备 无命中
- L2 规则本体保留即签署前压缩与整理为人类最后整理权原样在文
- L3 修订一二原文零改动，修订三为纯新增行
- L4 管线绿认证上链链 valid
- L5 机械 message 动态取 cert 哈希
- L6 全程租约在册锁覆盖写点
- L7 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 corr012-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/task-packages/corr012-t6d.md
- sih-engine/task-packages/corr012-t6d-results.md
- sih-engine/doc/decision/012-git-commit-takeover.md
- sih-tools/scribe/
