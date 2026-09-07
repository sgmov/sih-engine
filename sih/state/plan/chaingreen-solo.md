# chaingreen-solo：四查链主树复绿收口

> 治理任务包（工程类，单线形 solo，DEC-018；**委外执行**：与 doorprep-solo 同一委外令、同一代理**串行**执行，本批在前；与在途 attnanchor 与 confpreempt 批并行，撞面排队）
> 承接：用户 2026-09-07 令「第一、第二优先出提示词委外」；缺陷证据即 vecfix-solo 收约后主树复验在案（本会话核验档）
> 日期：2026-09-07

## 一、问题陈述 {#problem}

vecfix 修复其申报面后，主树四查链仍红，根因双股：

- **股一，墓碑件未进版控**：basemgrimpl 批的退役件即 `sih-tools/incubation/packs/sdd-v1/golden/` 七件与 `sih-tools/acceptor/frozen/expected-scenarios-full.json` 一件，是主树工作区未提交修改（M 态）——主树直写未结算，活在差集闸盲区（闸只对批分支）。主树实测：检查器套 6 过 1 红（test_golden_replay，KeyError 'findings'，读墓碑件缺键）；判定包端到端退出码 1（TC-003 violation，绿腿即检查器套；TC-001/002/004 过）。
- **股二，旧位依赖第三漏角**：test_golden_replay 仍读 golden 旧位，与版本钉同类即迁移依赖闭包未闭全，应改读新家 `sih-tools/basemgr/vectors/checker-golden/`。
- **附带**：`sih-tools/nomenclator/packs/core/terms.json` 一笔未提交修改，查源归位。

## 二、关键设计 {#design}

### 2.1 墓碑归位（内容零改动）

八件墓碑经本批工地通道入版控：工地带入同内容，settle 归并后主树 M 态自然清零。close 归并遇主树脏件同名冲突按备份让位归并对表法（备份、让位、归并、逐件 diff 全数 IDENTICAL）。墓碑**内容零改动**——只入版控不改写；`.bin` 期望件零触碰。

### 2.2 旧位依赖改读新家

test_golden_replay 改从 `basemgr/vectors/checker-golden/` 读期望（对齐 manifest D-4 锚位的 vectors_home 指向），旧 golden 位此后只剩墓碑即历史档案。改后检查器套主树全绿为 F-2 判据。

### 2.3 terms.json 查源处置（禁默改禁默弃）

diff 逐行呈报入批材料；若可对证为某批已申报登记的词条（CALL-LOG 或批档可查）则经本批通道入版控；无法查证合法性即停批上报候裁——禁默改禁默弃禁回退。

### 2.4 收口判据（唯一）

主树 TDD 验收工具判定包端到端退出码零、TC-001 至 TC-004 全过——本批自身即复验载体，批内绿加收约后主树复跑绿双读数。

### 2.5 命题与机器门

一命题 gid m-chaingreen-1，单锚即可验证性。facet 三步加得一三步，stable_clear 即落据；boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门、叩问、正身、watch 对表（在盘遗留如实转述不代清）、泊界心跳两线、例行读数
- [ ] 缺陷复现三项留痕（检查器套 1 红、判定包退出码 1、git status 八件 M 加 terms.json M）即修复前红态基线

### Cluster 2：施工

- [ ] 租约开工：--package chaingreen-solo，双仓工地，--allow 按请求写入节（**逐路径分行**，承 facefit 模板条款）；零 sih-math、零租约与引擎源码面
- [ ] 任务包与提示词拷入工地 state/plan（批输入件先例）
- [ ] test_golden_replay 改读新家，工地内检查器套全绿
- [ ] terms.json diff 查源处置（§2.3）
- [ ] 工地带入八件墓碑同内容（零改动），批内验证判定包退出码零

### Cluster 3：管线、facet 与结算

- [ ] 化格（JSON 过 json-canonical-v1、Markdown 过 general-v1）、检词 core 零违例（懒波词面全称规避）、核阅域外 exit-2 如实记档
- [ ] m-chaingreen-1 facet 三步加得一三步，stable_clear 终签入当日链
- [ ] 温故 recall 底稿；结果档 chaingreen-solo-results.md 落 event/plan（含红态节与查源节）
- [ ] 双仓 settle（差集闸在位，声明件先提交后 close）→ 放锁收约（主树脏件同名冲突按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔
- [ ] **收约后主树复跑**：判定包端到端退出码零读数回填补笔节；git status 对应 M 清零读数在档
- [ ] 收约后向同一代理确认串行下一批即 doorprep-solo

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 墓碑归位 | 工程 | 八件经批通道入版控，主树对应 M 态清零，内容零改动（归并 diff IDENTICAL） |
| **F-2** 旧位依赖清零 | 工程 | test_golden_replay 读新家，检查器套主树全绿（7/7） |
| **F-3** terms 查源 | 治理 | diff 逐行呈报在档，处置留痕；无法查证即停批候裁不默断 |
| **F-4** 四查链复绿 | 治理 | 主树判定包退出码零 TC 全过，双读数在档 |
| **F-5** 终签在链 | 治理 | m-chaingreen-1 stable_clear 终签入当日链，verify valid |
| **F-6** 写入仅 allow 且不越权 | 治理 | 零引擎零租约零 sih-math 源码写入；`.bin` 零触碰；在泊件与在盘遗留零触碰；在途批撞面排队如实呈报 |

## 五、必读文件 {#read}

- 缺陷对象：`sih-tools/checker/tests/test_engine.py`（test_golden_replay）、`sih-tools/basemgr/vectors/checker-golden/`
- 判定包：`sih-tools/acceptor/packs/tdd-v0-sddchecker.json`
- 请求写分行条款：`sih-tools/lease/TASK-PACKAGE-TEMPLATE.md`（facefit 批落位）
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 墓碑内容零改动只入版控；`.bin` 期望件与向量 kind 零触碰
2. terms.json 禁默改禁默弃禁回退，无法查证即停批候裁
3. 最小改动面：检查器 tests、八件墓碑、terms.json（查源后）、批件面
4. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
5. 在途 attnanchor 与 confpreempt 批撞面即排队，链归并按纯追加并集超集通道，真分叉停批
6. 中文零新造正式词；懒波词面全称规避

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 chaingreen-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- close 归并遇主树 M 脏件同名：备份让位归并四步，diff 非 IDENTICAL 即真分叉停批上报
- terms.json 若属在途批申报面（撞锁即信号），让位排队不抢
- 主树复跑若遇环境差异红，如实分列不硬闯

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/chaingreen-solo.md`
- `sih-tools/checker/tests/test_engine.py`
- `sih-tools/incubation/packs/sdd-v1/golden/`（八件中之七件墓碑归位）
- `sih-tools/acceptor/frozen/expected-scenarios-full.json`（墓碑归位）
- `sih-tools/nomenclator/packs/core/terms.json`（查源后处置）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/chaingreen-solo-results.md`
- `sih-engine/sih/event/plan/chaingreen-solo-materials/`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- `sih-tools/facet/`
- `sih-tools/proposition/DES/`
- worktrees 双仓 chaingreen-solo 工地
