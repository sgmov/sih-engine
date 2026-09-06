# acceptorimpl-solo：TDD 验收工具实装与判定包首例

> 治理任务包（工程类，单线形 solo，DEC-018；**委外执行**：单代理持本任务包与提示词连续执行全链）
> 承接：用户 2026-09-07 令「开工」即 TDD 验收工具实装批；上游输入件即孵化登记件 `sih-tools/incubation/regula-line-tools-2026-09-06.md`（验收工具节契约权威）、SPEC-021（四查与甲乙红线）、设计稿 regula-design-draft-2026-09-06.md §四、检查器先例 `sih-tools/checker/`（词汇表与 golden 与红证素材）
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- TDD 门的四查（先红后绿、场景覆盖、基线冻结、双跑一致）当前无执行机：绿基线是手工跑出来的，golden 是静态文件，没有一台机器按判定数据把四查跑成退出码
- 检查器实装给出了可复用的先例：封闭操作词汇表＋义械双载体＋空腹 grep 证明＋围堰向量＋先红后绿；本批同形推进
- 基线向量管理工具（登记件工作名）未建，全量漂移三态归因机制属其后继；本批四查取 v0 诚实形态，范围外显式申报

## 二、关键设计 {#design}

### 2.1 落位与命名

工具落 `sih-tools/acceptor/`，工作代号 `acceptor`（uv src 布局照 checker 先例）。代号中性不带文规字面，名脉候正式立名裁，目录迁移属已裁定连带改写面。受检文档用「TDD 验收工具」指称，规避懒波词面全称命中。

### 2.2 双工作面（承 checkerimpl 先例）

其一，**TDD 判定包数据形**：四查即数据——判定包 JSON 声明四查各用的操作与参数（命令串、冻结期望件路径、红证件路径、场景清单路径与判据程序寻径约定）。其二，**引擎实装**：解释判定包的空腹执行机，逐查跑、逐查记、聚合成总退出码。

### 2.3 四查 v0 形（诚实边界，逐查声明）

- 双跑一致：命令同参跑两遍，输出 cmp 逐字节一致且退出码一致。
- 基线冻结：命令输出对冻结期望件逐字节比对；有漂移且判定包申报影响集时做包含判定报原始事实；全量三态归因（纯期望过期／真回归／工装抖动）属基线向量管理工具后继，v0 申报范围外。
- 先红后绿：冻结红证哈希对表＋当前命令全绿；红证缺席即该查不过（三态之缺件）。
- 场景覆盖：场景清单（sdd-v1 同语法）逐 S- 编号的判据程序运行结果差集为零。
- 甲乙红线入报告：基线类查只主张没变不主张对，报告载声明范围字段。

### 2.4 首例判定包（自反狗粮闭环）

首例 TDD 判定包对 sdd-v1 与 checker 线成文：双跑与冻结吃 checker golden 六件；红证吃 checkerimpl 批材料红档（first-red-pytest.txt 哈希冻结）；覆盖吃 sdd-v1 场景清单样例的判据行（判据程序经 manifest 0.2.0 指向 checker，寻径核对在案）。验收工具验判定机，判定机的素材喂验收工具。

### 2.5 纪律承袭

先红后绿（测试先行、空实现红证归档、实装至绿）；封闭操作词汇表 v0 全集定义在 CONTRACT.md，词汇表外新操作即停批上报；空腹 grep 证明（引擎 src 零项目特异字面零命令字面零规则编号字面）；围堰基线向量（判定包全过与各查失败的冻结期望报告）；D-4 锚位机制在，包声明锚时激活。

### 2.6 命题与机器门

一命题 gid m-acceptor-impl-1，单锚即可验证性。facet 三步加得一三步，stable_clear 即落据；boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门（三锚引文程序切片）、叩问、正身、watch 对表（在盘遗留如实转述不代清）、泊界心跳两线、例行读数

### Cluster 2：施工

- [ ] 租约开工：--package acceptorimpl-solo，双仓工地（sih-engine base main、sih-tools base integral-stage-build），--allow 按请求写入节；零 sih-math 工地、零租约源码面、零引擎源码面
- [ ] 任务包与提示词拷入工地 state/plan（批输入件先例）
- [ ] CONTRACT.md 成文（v0 操作词汇表全集、判定包 schema、as-built CLI、v0 范围申报含三态归因后继声明）
- [ ] 首例判定包成文（§2.4，寻径核对入档）
- [ ] 测试先行写、空实现跑红、红证归档
- [ ] 引擎实装至全绿（含 grep 空腹证明测试、双跑决定论测试、四查逐查三态实测）

### Cluster 3：管线、facet 与结算

- [ ] 化格：JSON 过 json-canonical-v1、Markdown 过 general-v1；检词 core 零违例（懒波词面全称规避）
- [ ] 核阅 des-001：域外 exit-2 如实记档
- [ ] 登记件验收判据逐条对表，读数入结果档
- [ ] m-acceptor-impl-1 facet 三步加得一三步，stable_clear 终签入当日链
- [ ] 温故 recall 底稿；结果档 acceptorimpl-solo-results.md 落 event/plan
- [ ] 双仓 settle → 放锁收约（撞主树同名未跟踪件按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 三态实测 | 工程 | 判定包全过退出码零；构造漂移（改一字节期望）与坏红证各产退出码一且 findings 命中对应查；缺判定包或缺红证件产退出码二 |
| **F-2** 首例跑绿 | 工程 | 首例判定包对 sdd-v1 与 checker 线四查全过退出码零，报告含声明范围字段 |
| **F-3** 先红后绿 | 治理 | 测试先行红证在批材料，实装后全绿 |
| **F-4** 空腹证明 | 工程 | grep 测试在档且绿：引擎 src 零项目特异字面（checker、cargo、pytest 等）零命令字面零规则编号字面 |
| **F-5** 终签在链 | 治理 | m-acceptor-impl-1 stable_clear 终签入当日链，verify valid |
| **F-6** 写入仅 allow 且不越权 | 治理 | 零租约零引擎零 sih-math 源码写入；登记件验收判据对表入结果档；在泊件与在盘遗留零触碰 |

## 五、必读文件 {#read}

- 契约权威：`sih-tools/incubation/regula-line-tools-2026-09-06.md`（验收工具节）
- 四查定案：`sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md`（T2、T8、甲乙红线）
- 设计稿：工作区根 `regula-design-draft-2026-09-06.md` §四
- 素材：`sih-tools/incubation/packs/sdd-v1/`（golden 六件、场景清单样例、manifest 0.2.0）、`sih-engine/sih/event/plan/checkerimpl-solo-materials/red-evidence/first-red-pytest.txt`
- 工程先例：`sih-tools/checker/`（CONTRACT、布局、测试形）
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 闸不过即停批，不硬闯不改判据不重采样凑收敛
2. 空腹是构成性承诺：词汇表外新操作即停批上报；命令与项目知识只进判定包数据不进引擎（F-4 机械证明）
3. v0 范围诚实申报：三态归因全量机制属基线向量管理工具后继，CONTRACT 与结果档双申报，不伪装已全
4. 零租约源码写入（租约线在修）、零引擎源码写入、零 sih-math 写入
5. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
6. 在泊件与在盘遗留零触碰不并批
7. 中文零新造正式词；受检文档用 TDD 验收工具指称规避懒波词面

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 acceptorimpl-solo-results.md 落 event/plan 含 v0 范围申报节与验收判据对表节

## 八、风险点 {#risks}

- 判定包 schema 与登记件契约的缝：以 as-built 披露入 CONTRACT.md，不改登记件
- 覆盖查的判据程序寻径（manifest 指向 checker 的改指核对）：寻径不通即停批申报不私改 manifest
- 红证哈希冻结跨批引用（checkerimpl 材料在 engine 仓）：只读引用，零触碰原档
- 与在途批（chainstamp、declguard 等）共享面：撞锁排队如实呈报，链归并按纯追加并集超集通道，真分叉停批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`
- 跨仓引用：`sih-tools/acceptor/`、`sih-tools/incubation/`、`sih-tools/proposition/DES/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/acceptorimpl-solo.md` 与 `acceptorimpl-solo-prompt.md`（批输入件经工地落位）
- `sih-tools/acceptor/`（src、tests、fixtures 坏样例、CONTRACT.md、pyproject、首例判定包）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/acceptorimpl-solo-results.md` 与 `acceptorimpl-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`、`sih-tools/facet/`、`sih-tools/proposition/DES/`
- worktrees 双仓 acceptorimpl-solo 工地
