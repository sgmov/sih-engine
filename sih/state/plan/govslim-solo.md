# govslim-solo：向界瘦身两层拆分

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「另外向界也应该要升级吧？」经拆半定夺即批三文档瘦身半；诊断源即本会话向界失效三实证即当前段冻结 08-25、真相沉版本史底部、零机械读者
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- GOV-003 编排节承诺当前段唯一详列，实际冻结在 2026-08-25 即结算追加制无更新通道，正文系统性腐烂误导读者
- GOV-002 与 GOV-003 版本史即沉淀层与判据层同页混载，单页超限，真相沉底，注意力成本反噬
- 状态描述类正文与链账实态必然漂移，无回算机制承载即投影腐烂是结构性的

## 二、关键设计 {#design}

### 2.1 两层拆分

每页两层即稳定层与沉淀层。稳定层即定义、判据、纪律，手写、版本化、走管线。沉淀层即版本史，整体迁出单页至独立 history 文件，只增不减，追记纪律随迁，承 GOV-003 增长纪律超限即拆结算记录另存句。

### 2.2 状态描述退役

GOV-003 编排节改编排节律即只述结构事实即结算追加、前瞻深度一、当前态由链与账本与心跳回算承载不再手写快照，08-25 编排快照退役归 git 历史与沉淀层。GOV-002 版本与固定节改沉淀层指针。

### 2.3 判据零改

退出标准五条、冻结清单排序标准句、范畴排除、全态定义、泊界节、增长纪律逐字节零改即本批不动任何判据文本。

## 三、工作清单 {#work}

### Cluster 1：工地写

- [ ] GOV-002 版本史整体迁 GOV-002-history-v1.md，正文留沉淀层指针，v2.5 换版条目入 history 首位
- [ ] GOV-003 版本史整体迁 GOV-003-history-v1.md，编排节改编排节律，v2.0 换版条目入 history 首位
- [ ] des-001-gov002 与 des-001-gov003 金向量随冻重录即重跑刷 content_hashes 与报告，findings 须零

### Cluster 2：主线串行验证

- [ ] 稳定层判据零改对表即批前批后 sha 对照
- [ ] 沉淀零删除对表即版本史条目逐字节迁入对照
- [ ] des-001 对四件域内核阅 exit 0，化格检词过
- [ ] cargo test 金向量相关测全绿
- [ ] checkcite、认证、双仓 settle、放锁收约、reconcile 与链 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 判据零改 | 治理 | GOV-002 概览与主线与退出标准与冻结清单与范畴排除、GOV-003 概览与全态定义与展开清单与泊界与增长纪律各节批前批后逐字节一致 |
| **F-2** 沉淀零删除 | 数据治理 | 两页版本史全部条目逐字节迁入 history 文件零删改 |
| **F-3** 状态描述退役 | 数据治理 | GOV-003 编排节不再含快照式当前段描述即改编排节律携回算指针，GOV-002 版本节改指针 |
| **F-4** 金向量随冻重录 | 跨族治理 | 两金向量重录 findings 零且 cargo test 相关测全绿 |
| **F-5** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |

## 五、必读文件 {#read}

- 在役两页：`sih-engine/doc/governance/GOV-002-mainline-lock-v1.md` 与 `GOV-003-fullstate-course-v1.md`
- 增长纪律源：GOV-003 增长纪律节即一页为限超限即拆
- 先例：pk-036 即 deyimerge-switch-solo 金向量随冻重录先例只刷期望断言逻辑零改
- 批一登记面：`sih-math/docs/anchorwave-parking-course-2026-09-07.md` 即前瞻深度一锚位

## 六、约束 {#constraints}

1. 判据文本零字节改动，换版条目只述结构变更不触碰判据语义
2. 版本史零删除，迁移逐字节承三层固定即 git 历史可回查
3. 换版签署裁量归人节点即 v2.1 至 v2.4 换版先例均未附得一签，本批同形申报候裁
4. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
5. 守卫在位禁 plain git commit，收约补笔走 bypass 登记
6. 在盘遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后本批零活跃锁
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 govslim-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- history 新文件首入核阅域，若版本史文本带文规违例即就地归零承 v2.4 补笔先例内容词零增删
- cargo test 范围裁至金向量相关测即不跑全量，如实记档
- GOV-004 不在本批范围，同形拆分归后继批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe` 与 `sih-engine/target/debug/scrutinator`
- 后继：罗盘件即回算件消费面，接视图定形后合设计
- 关联泊件：无触碰

## 十一、请求写入 {#requested-writes}

- `sih-engine/doc/governance/GOV-002-mainline-lock-v1.md`
- `sih-engine/doc/governance/GOV-003-fullstate-course-v1.md`
- `sih-engine/doc/governance/GOV-002-history-v1.md`
- `sih-engine/doc/governance/GOV-003-history-v1.md`
- `sih-engine/src/scrutinator/fixtures/golden/des-001-gov002.json`
- `sih-engine/src/scrutinator/fixtures/golden/des-001-gov003.json`
- `sih-engine/sih/state/plan/govslim-solo.md`
- `sih-engine/sih/event/plan/govslim-solo-results.md`
- `sih-engine/sih/event/plan/govslim-solo-materials/`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- worktrees 双仓 govslim-solo 工地
