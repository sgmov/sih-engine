# basemgrimpl-solo：基线向量管理工具实装与双机围堰向量迁入

> 治理任务包（工程类，单线形 solo，DEC-018；**委外执行**：单代理持本任务包与提示词连续执行全链）
> 承接：用户 2026-09-07 令「下一步的工作出提示词和任务包，委外」；上游输入件即孵化登记件 `sih-tools/incubation/regula-line-tools-2026-09-06.md` 基线向量管理工具节（契约权威，五子模块细目明文授权本批设计）、SPEC-021 金向量双种方法论、gvec-v2 修订稿 `sih-tools/scribe/reports/2026-09-06-gvec-v2-design.md`、检查器与 TDD 验收工具两先例及其围堰向量
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 第三台登记工具即基线向量管理工具（金向量双种共享家位）未实装：检查器的 golden 六件与 TDD 验收工具的 frozen 件都以围堰形态落在自家目录，manifest 申报的迁移点全部指着它
- TDD 验收工具申报的范围外项（漂移三态归因全量机制、申报影响集包含判定）按其 CONTRACT 声明归本工具后继承接
- 登记件明文：共享家位五子模块细目未在已立文本中枚举，以实装批设计为准——本批即设计承载批，设计申报入 CONTRACT

## 二、关键设计 {#design}

### 2.1 落位与命名

工具落 `sih-tools/basemgr/`，工作代号 `basemgr`（uv src 布局照 checker 与 acceptor 先例）。代号中性不带文规字面，名脉候正式立名裁，目录迁移属已裁定连带改写面。受检文档用「基线向量管理工具」指称，规避懒波词面全称命中。

### 2.2 五子模块设计承载（登记件授权）

候选分解五子模块：其一存储与 manifest（向量件家位与六对象声明位）；其二冻结（capture 跑出冻结四元组与环境指纹）；其三重放比对（重放加逐字节比对加指纹前置探针）；其四漂移归因（申报影响集包含判定与三态映射）；其五重冻分种（基线种归因前置、规约种先红前置）。细目设计以 gvec-v2 修订稿为底成文于 CONTRACT.md 并申报，不越登记件已裁边界。

### 2.3 三操作契约（登记件冻结，不得私改）

CLI 三子命令即 freeze、replay、refreeze 各带显式向量件参。重放只读零写入；冻结与重冻落笔仅限显式指名向量件。向量件 JSON 形：冻结四元组（输入、调用上下文、寻径约定、期望输出，非二元组）加环境指纹位（cwd 约定、路径解析根、环境变量白名单、时钟类型）加版本三元积（程序版本、规则包版本、目标内容版本）。manifest 载六冻结对象声明位（程序、清洗程序、变异清单、分类方式、代表元选择、探针集）并随冻结件哈希。

### 2.4 v0 深度与诚实边界

- 六冻结对象全数据位在；消费变异清单与分类方式与代表元与探针的覆盖机（探针跑、分块矩阵、覆盖率报告）属后继批，CONTRACT 与结果档双申报不伪装已全
- 漂移归因：有申报影响集时做包含判定映射三态即纯期望过期（允许随冻重录）／真回归（阻断）／工装非决定论（报工装层）；无申报影响集即未分类漂移，refuse 重冻
- 规约种重冻先红前置：无新鲜红证即 refuse（三态之缺件）
- 甲乙红线入报告声明字段；现役逐字节门优先条款入 CONTRACT
- 三值退出码与三现役机器一致并冻结；双版本戳

### 2.5 双机迁移与 D-4 锚位激活

- 检查器 golden 六件与 TDD 验收工具 frozen 件迁入新家：旧位退役移批材料留档，检查器 manifest 与验收判定包的寻径更新并版本进位申报
- 迁移以重放证零漂移：迁入后验收判定包重放退出码零、检查器 golden 重放退出码零
- 检查器包族出处面 D-4 锚位回填：五包 provenance 声明指向新家向量，required 由 false 翻真（checkerimpl 批申报的激活路径就此走通）

### 2.6 自托管

管理工具自身 CLI 的金向量冻结入自身家——首个真家位的自托管，冻结加重放对自身跑通。

### 2.7 纪律承袭与命题

先红后绿（测试先行红证归档后实装至绿）；操作词汇封闭（词汇表外即停批）；空腹 grep 证明（引擎 src 零项目特异字面零命令字面零规则编号字面）；一命题 gid m-basemgr-impl-1，单锚即可验证性，stable_clear 即落据，boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门（三锚引文程序切片）、叩问、正身、watch 对表（在盘遗留如实转述不代清）、泊界心跳两线、例行读数

### Cluster 2：施工

- [ ] 租约开工：--package basemgrimpl-solo，双仓工地（sih-engine base main、sih-tools base integral-stage-build），--allow 按请求写入节；零 sih-math 工地、零租约源码面、零引擎源码面
- [ ] 任务包与提示词拷入工地 state/plan（批输入件先例）
- [ ] CONTRACT.md 成文（五子模块设计申报、v0 范围申报、词汇表全集、逐字节门优先与甲乙红线条款）
- [ ] 向量件与 manifest 数据形成文
- [ ] 测试先行写、空实现跑红、红证归档
- [ ] 引擎实装至全绿（含 grep 空腹证明、三子命令三态实测、指纹不匹配前置拦实测）
- [ ] 双机迁移与重放零漂移实证；D-4 锚位回填；自托管冻结

### Cluster 3：管线、facet 与结算

- [ ] 化格：JSON 过 json-canonical-v1、Markdown 过 general-v1；检词 core 零违例（懒波词面全称规避）
- [ ] 核阅 des-001：域外 exit-2 如实记档
- [ ] 登记件验收判据三查逐条对表（重放零漂移、环境指纹、重冻分种两通道），读数入结果档
- [ ] m-basemgr-impl-1 facet 三步加得一三步，stable_clear 终签入当日链
- [ ] 温故 recall 底稿；结果档 basemgrimpl-solo-results.md 落 event/plan
- [ ] 双仓 settle（差集闸在位，声明件先提交后 close）→ 放锁收约（撞主树同名未跟踪件按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 三查实测 | 工程 | 重放零漂移退出码零；环境指纹不匹配样本前置探针拦截退出码二；重冻分种两通道退出码逐一对表（基线种漂移三态映射、规约种无红证 refuse） |
| **F-2** 迁移零漂移 | 工程 | 双机围堰向量迁入新家后，验收判定包重放退出码零、检查器 golden 重放退出码零；旧位退役留档；寻径更新与版本进位申报在档 |
| **F-3** 自托管 | 工程 | 管理工具自身金向量冻结入自身家且重放退出码零 |
| **F-4** 先红后绿 | 治理 | 测试先行红证在批材料，实装后全绿 |
| **F-5** 空腹证明 | 工程 | grep 测试在档且绿：引擎 src 零项目特异字面零命令字面零规则编号字面 |
| **F-6** 终签在链 | 治理 | m-basemgr-impl-1 stable_clear 终签入当日链，verify valid |
| **F-7** 写入仅 allow 且不越权 | 治理 | 零租约零引擎零 sih-math 源码写入；登记件验收判据对表入结果档；在泊件与在盘遗留零触碰 |

## 五、必读文件 {#read}

- 契约权威：`sih-tools/incubation/regula-line-tools-2026-09-06.md` 基线向量管理工具节
- 方法论定案：`sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md`（四层模型与 T1 至 T11）
- 修订稿底本：`sih-tools/scribe/reports/2026-09-06-gvec-v2-design.md`
- 迁移对象：`sih-tools/incubation/packs/sdd-v1/golden/` 六件、`sih-tools/acceptor/frozen/`、`sih-tools/acceptor/packs/tdd-v0-sddchecker.json`
- 工程先例：`sih-tools/checker/`、`sih-tools/acceptor/`（CONTRACT、布局、测试形）
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 闸不过即停批，不硬闯不改判据不重采样凑收敛
2. CLI 三子命令形是登记件冻结契约不得私改；五子模块设计申报不越登记件已裁边界（一词两物、四元组、指纹位、重冻分种、甲乙红线、逐字节门优先）
3. v0 范围诚实申报：覆盖机（探针、分块矩阵、覆盖率）属后继批，双申报不伪装已全
4. 零租约源码写入（租约线在修收官）、零引擎源码写入、零 sih-math 写入
5. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
6. 在泊件与在盘遗留零触碰不并批；checkerimpl 红证原档只读
7. 中文零新造正式词；受检文档用基线向量管理工具指称规避懒波词面

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-7 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 basemgrimpl-solo-results.md 落 event/plan 含五子模块设计申报节与 v0 范围申报节与验收判据对表节

## 八、风险点 {#risks}

- gvec-v2 修订稿与 v0 实装的缝：以 v0 范围申报承载，缝大即停批上报
- 迁移同时动检查器 manifest 与验收判定包两件数据：版本进位逐件申报，迁移前后重放对表在档
- 环境指纹白名单的取舍（env 变量集）以最小白名单起步并申报，不完备性承 T11 条款声明
- 与在途批共享面：撞锁排队如实呈报，链归并按纯追加并集超集通道，真分叉停批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`
- 跨仓引用：`sih-tools/basemgr/`、`sih-tools/incubation/`、`sih-tools/checker/`、`sih-tools/acceptor/`、`sih-tools/proposition/DES/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/basemgrimpl-solo.md` 与 `basemgrimpl-solo-prompt.md`（批输入件经工地落位）
- `sih-tools/basemgr/`（src、tests、fixtures、CONTRACT.md、pyproject、自托管向量家）
- `sih-tools/incubation/packs/sdd-v1/`（golden 迁出与 manifest 版本进位与 provenance 锚位回填）
- `sih-tools/acceptor/`（frozen 迁出与判定包寻径更新）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/basemgrimpl-solo-results.md` 与 `basemgrimpl-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`、`sih-tools/facet/`、`sih-tools/proposition/DES/`
- worktrees 双仓 basemgrimpl-solo 工地
