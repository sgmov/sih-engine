# critsweep-solo：判据扫 v1——对话框内治理态回算器与启动序接线

> 治理任务包（实装类，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：用户 2026-09-08 令「同意，你拉起子代理跑」即判据扫候选设计甲案开工令；上游思想源即 2026-09-07 与 09-08 主会话裁定——「这个要有一个完整的机械化流程，视图实装是另外一个概念」，判据沉底须机械召回不靠 LLM 语义撞见
> 日期：2026-09-08

## 一、问题陈述 {#problem}

- 对话框内启动节律四件（自检、回锚、例行读数、泊界心跳）全部机械化，唯判据与向界面零机械存在：GOV-002 v2.4 五条退出判据的实态无回算器，判据三 measure-poly 程序本体自 2026-09-04 立项后零批沉底四天，靠主会 LLM 语义盘点才撞见（2026-09-08 主会话实录）
- 活动扫描有语义陷阱：pk-044 出泊裁定（2026-09-07）明引 measure-poly-rev1 §7，全文匹配会把设计引用误计为程序活动——扫描须按批名命名空间即 record_path 与 package 字段，不扫裁定散文
- 主会话临场手写的内联提取脚本（会话汇总、泊件速览、trail 摘要）无版控无测试，仪器由 LLM 临场造，读数不可归版本责任

## 二、关键设计 {#design}

- 工具 sih-tools/critsweep/ 四件：sweep.py（纯标准库零 LLM 零网络，退出码恒零，失效降级可见，根判据承 anchor.py 上溯形）、registry.json（空腹判据登记：id、title、status_kind、evidence、token_scope 批名令牌、registered_at、blocked_by 可选）、CONTRACT.md、tests/（pytest 加 fixture 链切片加金向量）
- 回算三面：判据面（逐判据三态即达成／在飞／沉底，达成以证据指针在档为准，活动按批名令牌扫 trail 的 record_path 与 report_path 与 package 字段，沉底阈值默认三日 --threshold 可覆调，程序零批即自 registered_at 起计 gap）、泊界面（两线泊材料经 selector parking 包路由回算，单线超时十秒降级）、在飞面（会话与锁账本回算）
- 输出严格 JSON 单对象 stdout；接线两件：AGENTS.md 启动节三件套改五件套即自检、回锚、判据扫、例行读数、泊界心跳；BATCH-FACE.md 增判据扫挂点节（verbatim 命令形，watch 对表挂点节先例同形）
- anchor.py 本批零触碰（v1 独立成扫；锚接判据行归后继批；缘由即 anchor.py 载权限位无主变更候人节点裁决，零触碰纪律）

## 三、工作清单 {#work}

### Cluster 1：工具本体（tools 工地）

- [ ] sweep.py 与 registry.json 与 CONTRACT.md 与 tests 落 sih-tools/critsweep/
- [ ] registry v1 五判据登记，逐条对链核证据指针与令牌表，派生依据落批材料
- [ ] sih-tools/pyproject.toml members 收编

### Cluster 2：接线

- [ ] AGENTS.md 会话启动节改五件套（归档件落引擎工地）
- [ ] sih-tools/BATCH-FACE.md 增判据扫挂点节

### Cluster 3：机械链

- [ ] 温故检索（落包前主会已跑零命中在批材料；结果档起草前按事件与时间轴再跑一次）
- [ ] 三问双门、叩问、正身、租约、意图、施工、管线三步、checkcite、认证、双仓 settle、放锁 close、对账对表、链 verify
- [ ] 主树裸调验收（工地条件验证不算接线）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 件落位与接线 | 实装 | critsweep 四件在位，零 LLM 零网络静态扫描过，pyproject members 收编，主树裸调可跑；AGENTS.md 五件套与 BATCH-FACE 挂点节在档 |
| **F-2** 判据实态复算对链 | 数据治理 | --at 2026-09-08 读数：判据一与五报达成且证据指针在档；判据二报在飞；判据三报沉底且批名命名空间零批记录（gap 自 2026-09-04 起算）；判据四按令牌实报；逐判据 last-hit 与 gap 给出可重放对表命令形；pk-044 散文引用不计为活动的对照读数在档 |
| **F-3** 确定性纪律 | 跨族治理 | 同参同日双跑 cmp 逐字节 IDENTICAL、退出码全零；缺 trail 或账本对应面降级行可见不静默 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-5** 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 |

## 五、必读文件 {#read}

- 命令面正典：`sih-tools/BATCH-FACE.md`（全链 verbatim 与全部坑位勘误）
- 家族纪律：`sih-tools/attnanchor/CONTRACT.md` 与 `sih-tools/attnanchor/anchor.py`（根判据上溯形、降级可见、严格 JSON）
- 判据源：`sih-engine/doc/governance/GOV-002-mainline-lock-v1.md`（v2.4 五判据文本与证据指针）
- 程序源：`sih-engine/sih/event/plan/measure-poly-rev1-progdoc.md` 与泊材料 pk-054、pk-044
- 链批先例：`sih-engine/sih/event/plan/genpark-solo-results.md` 与 `archpark-solo-results.md` 与 `anchorskill-solo-results.md`（委外链全形与收约补笔与无主闸 bypass-orphan 先例）

## 六、约束 {#constraints}

1. 零子代理，委外代理亲写单线 solo；工具零 LLM 零网络
2. anchor.py 与 .zcode/config.json 零触碰（前者载权限位无主变更候人裁，如实记缘由）
3. 主树零直写即待提交件经工地 settle 通道；AGENTS.md 无仓版控原地改，归档件落引擎工地
4. 守卫在位禁 plain git commit；收约补笔经 --no-verify 加 lease bypass 登记
5. watch 无主件（anchor.py 权限位、calls.ndjson）不豁免不代清；close 无主闸拦即 --bypass-orphan 载事由如实（anchorskill 先例同形）
6. scribe append 只认 JSON 报告：md 件走内容哈希清单件认证形（anchorskill 先例）
7. 禁管道掩退出码；先红留痕；工地期 SIHANKOR_CALLLOG_DOGFOOD 不开
8. CALL-LOG 走 lease call-log append 三腿齐落；confpreempt-solo 在途会话与 confidence 面零触碰
9. 任务锚 .session-anchor.md 归主会管理，本批零触碰

## 七、验收标准 {#acceptance}

本任务包验收 = 五项：F-1 至 F-5 全过；双仓 settle 提交号在档且收约后零本批活跃锁零本批活跃会话；链 verify valid 且 reconcile 零新增；结果档 critsweep-solo-results.md 落 event/plan；主树裸调 sweep 双跑读数入结果档。

## 八、风险点 {#risks}

- close 无主闸撞已知二无主件即 bypass-orphan 通道（先例同形，事由须载明非本批活面）
- 令牌表过宽即误报活动（pk-044 对照例）：派生依据逐条落材料，宁窄勿宽，窄报沉底不谎报在飞

## 九、范畴排除 {#exclusions}

- 视图实装（人面 UI）不属本批；anchor 六行接线不属本批；判据增删改裁不属本批（registry 只登记 GOV-002 在役五条，判据本体变更走换版批）；measure-poly 是否继续不裁（只回算实态）

## 十、关联文件 {#related}

- 主会话裁定源：2026-09-07「判据该有机械召回」与 2026-09-08「对话框内完整机械化流程，视图实装另一概念」
- 上游泊件：pk-054（出泊条件挂 measure-poly 批四）、pk-077（自有运行时远景）

## 十一、请求写入 {#requested-writes}

- sih-tools/critsweep/
- sih-tools/pyproject.toml
- sih-tools/BATCH-FACE.md
- AGENTS.md
- sih-engine/sih/state/plan/critsweep-solo.md
- sih-engine/sih/event/plan/critsweep-solo-results.md
- sih-engine/sih/event/plan/critsweep-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/critsweep-solo
- worktrees/sih-engine/critsweep-solo
