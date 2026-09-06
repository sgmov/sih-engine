# anchwave-solo：向泊二界数学回锚登记

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「本次会话的向界是向泊二界数学回锚波和泊界升级（带锚落地），另外向界也应该要升级吧？」批一即回锚波；讨论源即本会话向界失效诊断与 mapping.md 全读核验与停车包裸常数实证
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 向、泊二界作为治理域，其判定承载机制无域级数学载体：mapping.md 全读核验即邻接件在册而域级零命中
- 停车包即 selector/packs/parking 在役四常数为无锚裸常数：老化阈值 45 日、侧线富余阈值 2、主线饥饿阈值 0、ttl 缺省 14 与 30，既无数学锚亦无纯约定登记，违背文规 D-1 唯一归因精神
- 两笔失效案例在档无下界登记：pk-070 假告警即不可裁件报到期、pk-063 触发器点火无人知即可裁件不上浮，正是 mapping 双源交集法所缺的 AI 失败案例下界材料
- 泊界升级即批二将新增 gate 机制，若回锚不先行即新增第五个裸常数

## 二、关键设计 {#design}

### 2.1 三态处置

每件判定承载机制落三态之一：邻接锚即挂已在册条目（经书单召回核实，不自造语义）；零命中申报即显式登记缺口作数学仓候选输入，工程不越俎代庖立条目；纯约定登记即诚实登记为约定无数学归因。

### 2.2 登记面落位

登记面落 sih-math/docs/anchorwave-parking-course-2026-09-07.md，承 asset-anchor-registry-2026-09-02.md 先例形态。

### 2.3 批序切割

泊界升级实装归批二带锚落地，向界文档瘦身与罗盘件归批三；本批零代码改动零在役判据与常数值触碰。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 书单召回即 wikirecall recall 出应读书单，锚定条目原文逐件加载核实
- [ ] 登记面文档落 sih-math 工地即盘点表与三态处置与零命中申报与失败案例下界材料
- [ ] 关键报告件认证入链

### Cluster 2：主线串行验证

- [ ] 管线三步跑任务包与结果档与登记面
- [ ] checkcite 书单对表即登记面全部引用落书单及图闭包
- [ ] 三仓 settle、放锁收约、reconcile 与链 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 盘点完备 | 数据治理 | 登记面盘点表覆盖泊界机制七件与向界机制四件共十一件，每件三态落位，三态之外零残留 |
| **F-2** 引用落书单 | 跨族治理 | checkcite exit 0 即登记面全部条目引用落书单及图闭包 |
| **F-3** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-4** 真值不越权 | 治理 | 登记面只列事实与候选不预设数学仓立条目，停车包常数值零改动，在泊件零触碰 |

## 五、必读文件 {#read}

- 消费面：`sih-math/llm-friendly-build/mapping.md`（全读即消费面验收线）
- 机制事实源：`sih-tools/selector/packs/parking/routes.toml` 与 `manifest.toml`
- 判据事实源：`sih-engine/doc/governance/GOV-002-mainline-lock-v1.md` 与 `GOV-003-fullstate-course-v1.md` 与 `PARKING-v1.md`
- 失败案例源：`sih-engine/sih/state/parking/materials/pk-070.json` 与 `pk-063.json`（只读零触碰）
- 先例：`sih-math/docs/asset-anchor-registry-2026-09-02.md`

## 六、约束 {#constraints}

1. 零 LLM 调用、零代码改动、零在役判据与退出码语义与停车包常数值触碰
2. 数学仓不新增条目，候选经登记面呈报走数学仓自身演化程序
3. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
4. 守卫在位禁 plain git commit，close 通道提交
5. 在盘遗留无主件不豁免不代清，本批不新增无主写
6. 唯一桥梁即引用只经 mapping 消费面与条目原文，产出引用落书单及图闭包

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-4 全过
- [ ] 三仓 settle 提交号在档，收约后本批零活跃锁
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 anchwave-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- CALL-LOG 与 scribe 报告面与在飞 basemgrimpl 会话共享，append 态可共存，exclusive 撞锁即如实申报候锁不绕行
- sih-math 无守卫钩，settle 走 lease commit 通道即 confrevise 先例形
- checkcite exit 1 即有引用不在书单，拦收口须重召回补书单

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（intent 与 append 写位）
- 跨仓引用：`sih-math/docs/` 与 `sih-tools/selector/packs/parking/`
- 后继批：批二泊界升级（gate 远景廊带锚落地）、批三向界瘦身与罗盘接口契约

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/anchwave-solo.md`
- `sih-math/docs/anchorwave-parking-course-2026-09-07.md`
- `sih-engine/sih/event/plan/anchwave-solo-results.md`
- `sih-engine/sih/event/plan/anchwave-solo-materials/`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- worktrees 三仓 anchwave-solo 工地
