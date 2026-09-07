# doorprep-solo：两道门备料——流程包数据形与 SDD 形任务包模板

> 治理任务包（立文类，单线形 solo，DEC-018；**委外执行**：与 chaingreen-solo 同一委外令、同一代理**串行**执行，本批在后即 chaingreen-solo 收约后方可开工）
> 承接：用户 2026-09-07 令「第一、第二优先出提示词委外」；上游输入件即设计稿 regula-design-draft-2026-09-06.md §一§三§四、孵化登记件、sdd-v1 包族、TDD 验收工具判定包先例、租约线 TASK-PACKAGE-TEMPLATE.md（facefit 批落位）
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 两道门实装批的两件备料缺位：其一，流程包数据形未成文——门规须是数据（任务类到 SDD 件族与 TDD 查族的映射），门批实装时直接接线，没有它门只能硬编码；其二，任务包模板未 SDD 化——现有模板与 SDD 五件天然同构（问题陈述≈变更提案、关键设计≈技术方案、工作清单≈任务清单、F 锚≈场景雏形、缺规格差分一件），升形即可但未做
- 协调点：租约线已落 TASK-PACKAGE-TEMPLATE.md（逐路径分行、版本位必填等硬规则），SDD 形模板须吸收合一，不造两模板

## 二、关键设计 {#design}

### 2.1 流程包数据形与首例

流程包声明任务类的门规映射：SDD 门件族（用哪些格式包）与 TDD 门查族（用哪些判定包）。数据形与格式包及判定包同构（manifest 风即名、版本、映射条目）。落 `sih-tools/incubation/packs/flow-v1/`。首例流程包即治理批类：SDD 门吃 sdd-v1 五包，TDD 门吃 tdd-v0 判定包形。门实装批的接线对象即此件。

### 2.2 SDD 形任务包模板

升形映射落定：变更提案（问题陈述加令源加范畴排除）、规格差分（本批增量件清单即 ADDED 形）、场景清单（F 锚升 R-／S- 编号形，判据行三要素绑定检验程序与期望退出码）、技术方案（关键设计）、任务清单（工作清单加每步证据位）；保留租约结构节（必读、约束、验收、风险、关联、请求写入）。**硬规则全吸收**：逐路径分行、版本位必填等条款显式引用 TASK-PACKAGE-TEMPLATE 并同文，不改彼件（租约线资产，合一归属注记候两线协调裁）。

### 2.3 模板自证（红绿双证）

以本批自身为变更对象按模板生成样例五件（自反狗粮承先例）：好样例过 sdd-v1 五包判定全绿；坏样例（缺判据行或重号）检查器红证在档。模板声称能过门形，就用门形机器验它。

### 2.4 后续批即用

本批收约后，文规线任务包一律按 SDD 形模板书写（自愿先行），两道门实装批的任务包是首个强制食客。

### 2.5 命题与机器门

一命题 gid m-doorprep-1，单锚即可验证性。facet 三步加得一三步，stable_clear 即落据；boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门、叩问、正身、watch 对表、泊界心跳两线、例行读数

### Cluster 2：备料成文

- [ ] 租约开工：--package doorprep-solo，双仓工地，--allow 按请求写入节（逐路径分行）；零 sih-math、零源码面
- [ ] 任务包与提示词拷入工地 state/plan（批输入件先例）
- [ ] 流程包数据形成文加首例（治理批类映射）
- [ ] SDD 形任务包模板成文（硬规则吸收加引用）
- [ ] 样例五件按模板生成（自反狗粮），好样例过检查器全绿留读数，坏样例红证归档

### Cluster 3：管线、facet 与结算

- [ ] 化格（JSON 过 json-canonical-v1、Markdown 过 general-v1）、检词 core 零违例（懒波词面全称规避）、核阅域外 exit-2 如实记档
- [ ] m-doorprep-1 facet 三步加得一三步，stable_clear 终签入当日链
- [ ] 温故 recall 底稿；结果档 doorprep-solo-results.md 落 event/plan
- [ ] 双仓 settle（差集闸在位，声明件先提交后 close）→ 放锁收约 → 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 流程包在位 | 工程 | flow-v1 数据形成文，首例治理批类映射（SDD 五包加 TDD 判定包）条目齐 |
| **F-2** 模板在位且自证 | 工程 | SDD 形模板成文，硬规则条款显式引用 TASK-PACKAGE-TEMPLATE；好样例过 sdd-v1 五包全绿读数在档，坏样例红证在档 |
| **F-3** 终签在链 | 治理 | m-doorprep-1 stable_clear 终签入当日链，verify valid |
| **F-4** 写入仅 allow 且不越权 | 治理 | 不改 TASK-PACKAGE-TEMPLATE 原件；零源码写入；在泊件与在盘遗留零触碰；串行序即 chaingreen-solo 收约后才开工 |

## 五、必读文件 {#read}

- 上游设计稿：工作区根 `regula-design-draft-2026-09-06.md` §一§三§四
- 包族与判定包先例：`sih-tools/incubation/packs/sdd-v1/`、`sih-tools/acceptor/packs/tdd-v0-sddchecker.json`
- 硬规则源：`sih-tools/lease/TASK-PACKAGE-TEMPLATE.md`
- 任务包现状形：`sih-engine/sih/state/plan/` 任一近期任务包
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 纯数据零源码改动（检查器调用是使用不是修改）
2. 不改 TASK-PACKAGE-TEMPLATE 原件（租约线资产），合一归属注记候裁
3. 模板与流程包的判定条款全部机械可验，语义层显式申报范围外
4. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
5. 在泊件与在盘遗留零触碰不并批；在途批撞面排队
6. 中文零新造正式词；懒波词面全称规避（检查器、TDD 验收工具、基线向量管理工具）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 doorprep-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 模板升形与既有任务包节的兼容缝：以映射表逐节对表入结果档，缝大即停批上报
- 坏样例构造不得污染正式包目录（落批材料或 fixtures 隔离位）
- 与在途批共享面：撞锁排队如实呈报，链归并按纯追加并集超集通道，真分叉停批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`、`sih-tools/checker`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/doorprep-solo.md`
- `sih-tools/incubation/packs/flow-v1/`
- `sih-tools/incubation/`（SDD 形任务包模板件）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/doorprep-solo-results.md`
- `sih-engine/sih/event/plan/doorprep-solo-materials/`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- `sih-tools/facet/`
- `sih-tools/proposition/DES/`
- worktrees 双仓 doorprep-solo 工地
