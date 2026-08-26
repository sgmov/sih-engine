# ratify-t6d：追认纳入批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户追认令与 latex-helper 纳版控令、意图记录 2026-08-27-ask3-ratify 即 hash 0adef047
> 范式：T6-D 范式——本任务包偏离：修订与纳控类，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 封字令机械形态口径冲突即界后存量二十一笔永呈未路由使日常对表退出码一常态化，用户裁定以追认表机械豁免处置。
- latex-helper 立名已毕而未入版控即承诺无载体，61 兆中 60 兆为 venv 与缓存即真实内容未纳。

## 二、关键设计 {#design}

### 2.1 追认表 {#ratify-table}

lease 升 1.4.0 即 SEAL_EXEMPTS 按仓名钉死二十一笔全量号，engine 三笔与 tools 十八笔，facet 两笔即 3451c6f 与 1592b4a 不入表。reconcile 识别表内提交归 sealed 类不计违规不入尾单，summary 增 sealed 计数，退出码语义不变。表变更须升版本。

### 2.2 封字令修订 {#ruling-amend}

audit-049 补追认令节即界后存量二十一笔经追认表机械豁免、不追认条款对此让位、表变更须升工具版本、facet 两笔仍露形处置归人节点。

### 2.3 latex-helper 纳版控 {#admit}

加 gitignore 排 .venv 与 .pytest_cache 与 __pycache__，经 lease commit 首次正身提交，调用留痕册随入。

## 三、工作清单 {#work}

### Cluster 1：三问与开工链（主线）

意图双腿验与上链、包档登记 wip、正身、open、锁、双仓副本。

### Cluster 2：追认表（副本流）

SEAL_EXEMPTS 与 sealed 类与测试与契约修订十，升 1.4.0。

### Cluster 3：封字令修订（副本流）

audit-049 补追认令节于引擎副本。

### Cluster 4：latex-helper 纳控（主检出）

gitignore 与首次正身提交。

### Cluster 5：管线与段结算与收约

化格核阅检词、认证上链、双仓段结算、收约归并、终烟测。

## 四、判据 {#criteria}

- L1 追认表钉死二十一笔即引擎三工具十八，facet 两笔不在表
- L2 reconcile 对表 engine 退出码零、tools 退出码一且尾单仅 facet 两笔
- L3 封字令修订入档与用户裁定一致
- L4 latex-helper 首次正身提交成即版控内可见且忽略规则排除 venv
- L5 管线绿认证上链链 valid
- L6 双仓段结算经正身路径、收约归并成、零在役锁

## 五、结果留档 {#results}

结果见 ratify-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/sih/event/audit/
- sih-engine/sih/state/plan/ratify-t6d.md
- sih-engine/sih/state/plan/ratify-t6d-results.md
- sih-tools/lease/
- sih-tools/latex-helper/
- sih-tools/nomenclator/
- sih-tools/scribe/
