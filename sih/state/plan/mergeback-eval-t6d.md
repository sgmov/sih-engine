# mergeback-eval-t6d：融回评估批

> T6D task-packages 治理任务
> 承接：2026-08-27 用户批令即融回评估批以三问轮回评估、意图记录 2026-08-27-ask3-mergeback 即 hash 759a46e5
> 范式：T6-D 范式——本任务包偏离：评估研究类，主线串行，无子代理
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 围堰工具十五件服役中即围堰期已历真实治理负载，融回门判据已立于 COURSE-v2 即两重门槛体量与时间加贡献度主判，逐件评估材料缺位，融回首件裁决待人节点而无材料可裁。
- 日常对表跑需手输 --base 即封线在裁定档不在工具，错输风险在即 audit-049 机械形态未单源。

## 二、关键设计 {#design}

### 2.1 三问轮回评估 {#ask3wheel}

逐件三问即体量即真实调用量与写入量从调用册与 trail 与 git 史取数、时间即服役窗自首调用至今、贡献度即拦下的真实问题与被依赖的强制位实走与影响过的决策走向，空转调用量不计入。产出研究档与建议次序，融回门开关与首件裁决归用户，不以判据全绿而提前。

### 2.2 去向核对 {#direction}

按 COURSE-v2 向界投影即路择谓词融判定器、按轮判定族融三问、facet 转模块接口，对表 GOV-002 五条退出标准，逐件标去向与回迁债，不另造去向。

### 2.3 封线入工具 {#seallines}

lease 内置两仓封线即 sih-engine 取 d2b4a24 父与 sih-tools 取 6b5855c 父，reconcile 缺 --base 时按仓名套用并在报告标注来源，非表内仓不套用保持显式。契约修订与测试随行。

## 三、工作清单 {#work}

### Cluster 1：三问与开工链（主线）

意图双腿验与上链、包档登记 wip、正身、open、锁、双仓副本。

### Cluster 2：证据采集与研究档（副本流）

逐件三问证据采集即调用册行数与 trail 事件数与 git 史计数与测试面，研究档落 doc/research/，词债随批登记。

### Cluster 3：封线小活（副本流）

lease 封线常量与 reconcile 默认套用与测试与契约修订。

### Cluster 4：管线与段结算与收约

化格核阅检词、认证上链、双仓段结算、收约归并。

## 四、判据 {#criteria}

- L1 研究档覆盖围堰全部工具即逐件三问与去向核对与建议次序
- L2 证据从留痕取数即逐件数据引用可回溯复演
- L3 封线入工具即免 --base 对表 engine 退出码零、tools 露形 1592b4a 即裁定后首笔增量如实呈报处置归人节点
- L4 管线绿认证上链链 valid
- L5 双仓段结算经正身路径、收约归并成
- L6 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 mergeback-eval-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/doc/research/
- sih-engine/sih/state/plan/mergeback-eval-t6d.md
- sih-engine/sih/state/plan/mergeback-eval-t6d-results.md
- sih-tools/lease/
- sih-tools/nomenclator/
- sih-tools/scribe/
