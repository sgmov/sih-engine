# checkerimpl-solo：检查器实装与包判定面机器化

> 治理任务包（工程类，单线形 solo，DEC-018；**委外执行**：单代理持本任务包与提示词连续执行全链）
> 承接：用户 2026-09-07 令「开工」即检查器实装批；上游输入件即孵化登记件 `sih-tools/incubation/regula-line-tools-2026-09-06.md`（接口契约与验收判据权威）、SDD 包族 `sih-tools/incubation/packs/sdd-v1/`（夹具与判定面三十条）、设计稿 regula-design-draft-2026-09-06.md §二§三
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 检查器（孵化登记件工作名）按契约须为空腹机器：规则全部来自格式包，引擎不内置。实核发现 sdd-v1 五包的三十条判定面谓词当前是中文描述句，无可执行机器字段——sddpacks 批的可验性由 validate.py 硬编码支撑，包自身不承载可执行性，与空腹机器的前提不合
- 本批双工作面：其一，包判定面机器化即给三十条规则补机器谓词字段（封闭操作词汇表），可执行性成为包数据；其二，检查器引擎实装即解释该词汇表的空腹判定机，按契约三值退出码与三态失败定位出报告
- 基线向量管理工具（登记件工作名）未建，本批 D-4 向量以围堰形态落包目录，工具建成后迁移申报

## 二、关键设计 {#design}

### 2.1 落位与命名

工具落 `sih-tools/checker/`，工作代号 `checker`（uv 工程形照 formatter 与 nomenclator 先例即 src 布局加 packs 装载加 tests 加 fixtures 加 CONTRACT.md）。代号刻意中性不带文规字面，名脉（承不承文规名）候正式立名裁，目录迁移属已裁定的连带改写面（toolincub 登记件命名次序节）。

### 2.2 包判定面机器化（契约面申报）

三十条规则逐条补 `check` 机器字段：封闭操作词汇表（如 exists_readable、heading_present、pattern_unique、pattern_all、reference_closure、field_triple_present、marker_section_present 类，v0 全集定义在工具 CONTRACT.md），参数显式（层级、正则、引用源、字段数）。原 `predicate` 中文句保留为义面，`check` 为械面，义械双载体。manifest schema 版本进位并申报；此为登记件契约判定面的补全非推翻，结果档专节申报（登记件为带日期不可变件不改文）。词汇表外的新操作即停批上报，不私扩。sdd-v1 五包机器化后随批过化格 json-canonical-v1。

### 2.3 引擎实装（按登记件契约）

- 空腹：引擎只实现词汇表通用操作，零文档形状知识；机械证明即 grep 测试——引擎 src 零 SDD 特异字面（SHALL、Scenario、R-、S- 等）零规则 ID 字面（SL-001 类）
- CLI 按登记件契约最小形；跨文档引用闭包检查（任务清单引场景清单）允许 --reference 目标形，as-built 形入 CONTRACT.md
- 三值退出码：零合规、一违规、二工具自身异常（含目标缺席依赖缺席），实测三态
- 报告 JSON：双版本戳（引擎版本加包版本）、逐条规则结果、失败带三态定位（缺件、违规、断链）与 hint
- 同参双跑 cmp 逐字节一致（决定论）；无网络；无隐式状态

### 2.4 先红后绿（本线自家纪律的首次实战）

测试先行：以 sdd-v1 五包与样例五件为夹具先写测试，空实现上跑红（红证归档），再实装至绿。红样例夹具须有：违规文档（如重复 S- 编号）产预期 findings 与退出码一、缺席目标产退出码二。

### 2.5 D-4 围堰向量与 validate.py 退役

样例五件加红样例的冻结期望报告落包目录（围堰基线向量），manifest 申报迁移点即基线向量管理工具建成后迁移。validate.py 按 sddpacks 批预设退役声明执行退役：manifest 指向检查器，脚本移入批材料留档。

### 2.6 命题与机器门

一命题 gid m-checker-impl-1，单锚即可验证性——契约条款与判据逐条只问能不能机械验。facet 三步加得一三步，stable_clear 即落据；boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门（三锚引文程序切片）、叩问、正身、watch 对表（在盘遗留如实转述不代清）、泊界心跳两线、例行读数

### Cluster 2：施工

- [ ] 租约开工：--package checkerimpl-solo，双仓工地（sih-engine base main、sih-tools base integral-stage-build），--allow 按请求写入节；零 sih-math 工地、零租约源码面、零引擎源码面
- [ ] 任务包与提示词拷入工地 state/plan（批输入件先例）
- [ ] CONTRACT.md 成文（v0 词汇表全集、CLI as-built、围堰向量家位申报）
- [ ] 五包机器化（三十条 check 字段，manifest 版本进位申报）
- [ ] 测试先行写、空实现跑红、红证归档
- [ ] 引擎实装至全绿（含 grep 空腹证明测试、双跑决定论测试、三态退出码实测）
- [ ] 冻结期望报告（绿五件加红样例）落包目录；validate.py 退役执行

### Cluster 3：管线、facet 与结算

- [ ] 化格：JSON 过 json-canonical-v1、Markdown 过 general-v1；检词 core 零违例（受检文档避免懒波词面全称，用检查器简称）
- [ ] 核阅 des-001：sih-tools 与 state/plan 与 event/plan 域外 exit-2 如实记档
- [ ] 登记件验收判据逐条对表，读数入结果档
- [ ] m-checker-impl-1 facet 三步加得一三步，stable_clear 终签入当日链
- [ ] 温故 recall 底稿；结果档 checkerimpl-solo-results.md 落 event/plan
- [ ] 双仓 settle → 放锁收约（撞主树同名未跟踪件按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 工具可跑三态实测 | 工程 | uv run 检查器按包跑样例五件全零违规退出码零；红样例退出码一且 findings 与期望一致；缺席目标退出码二；同参双跑 cmp 逐字节一致 |
| **F-2** 包机器化 | 工程 | 五包三十条规则全带 check 字段且词汇表内，manifest 版本进位与契约补全申报在结果档 |
| **F-3** 先红后绿 | 治理 | 测试先行红证在批材料，实装后全绿；红样例夹具产预期 findings |
| **F-4** 空腹证明 | 工程 | grep 测试在档且绿：引擎 src 零 SDD 特异字面零规则 ID 字面 |
| **F-5** 终签在链 | 治理 | m-checker-impl-1 stable_clear 终签入当日链，verify valid |
| **F-6** 写入仅 allow 且不越权 | 治理 | 零租约源码零引擎源码零 sih-math 写入；validate.py 退役完成；登记件验收判据对表入结果档；在泊件与在盘遗留零触碰 |

## 五、必读文件 {#read}

- 契约权威：`sih-tools/incubation/regula-line-tools-2026-09-06.md`（检查器节即接口契约与验收判据）
- 夹具与判定面：`sih-tools/incubation/packs/sdd-v1/`（manifest、五包、样例五件、validate.py）
- 上游设计稿：工作区根 `regula-design-draft-2026-09-06.md` §二§三
- 工程形先例：`sih-tools/formatter/`（uv 布局与空腹纪律）
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 闸不过即停批，不硬闯不改判据不重采样凑收敛
2. 空腹是构成性承诺：词汇表外新操作即停批上报；引擎零文档形状知识（F-4 机械证明）
3. 契约补全走申报通道：登记件不改文，机器化与版本进位在结果档专节申报
4. 零租约源码写入（租约线在修）、零引擎源码写入、零 sih-math 写入
5. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
6. 在泊件与在盘遗留零触碰不并批
7. 中文零新造正式词；受检文档避免懒波词面全称用检查器简称

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 checkerimpl-solo-results.md 落 event/plan 含契约补全申报节与验收判据对表节

## 八、风险点 {#risks}

- 词汇表覆盖不足：三十条里有规则落不进 v0 词汇表即停批上报，不私扩词汇
- 跨文档闭包的 CLI 形与登记件契约的最小形有出入：以 as-built 披露入 CONTRACT.md，不改登记件
- 围堰向量与基线家位迁移是已知债，manifest 申报在案
- 与在途批共享当日链：close 归并按纯追加并集超集通道，真分叉停批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`
- 跨仓引用：`sih-tools/checker/`、`sih-tools/incubation/`、`sih-tools/proposition/DES/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/checkerimpl-solo.md` 与 `checkerimpl-solo-prompt.md`（批输入件经工地落位）
- `sih-tools/checker/`（src、tests、fixtures 红样例、CONTRACT.md、pyproject）
- `sih-tools/incubation/packs/sdd-v1/`（五包机器化、manifest 版本进位、冻结期望报告、validate.py 退役处置）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/checkerimpl-solo-results.md` 与 `checkerimpl-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`、`sih-tools/facet/`、`sih-tools/proposition/DES/`
- worktrees 双仓 checkerimpl-solo 工地
