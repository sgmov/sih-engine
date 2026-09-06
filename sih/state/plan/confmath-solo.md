# confmath-solo：置信度数学模型建模批

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「置信度数学模型先行，你出任务提示词和任务包」即 pk-073 出泊条件之立项裁落位；已定型要件与候选方向照录源即 pk-073 泊材料（主会 2026-09-06 置信度经济三轮设计对话用户系列裁定）
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 置信度经济制度六要件已跨三轮对话定型，但全部为散文形态，无数学形式化；既裁约束即引擎数学常数须有数学模型支撑，置信度数值面的一切开发被钉死在本批之后
- billwire 批已携三账单事件形与点值实跑（open_face_bill 3 pts／lock_free 0 pts／lock_charged 1 pt），属阶段一记账；其点值的模型地位（回溯支撑或 grandfather）待本批裁定
- pk-074 子项一超宽阈值与子项二未用罚口径的参数槽位须在本批定义，数据面留口待实证批

## 二、关键设计 {#design}

### 2.1 公理化输入（照录零改写）

A 族即 pk-073 六要件：A1 置信度即信用点即支付货币，单一台账按正身开户、追加流水上链、余额即支付能力、历史即信誉曲线、余额下限零、欠账未清不得抢占、支付 fail-closed；A2 铸币即上链工作（干净收约铸币、主动链笔小额、修复他因事故铸币；自产自修不铸币；纯机械触发器零主观打分；数值走常数定义化通道）；A3 罚金重于铸币（罚金与损害量级挂钩，事故级罚金须大于若干次清洁铸币之和，防高产能莽人刷分通道）；A4 抢占制闭环（付置信度不退、得一裁事实与理由、放锁落链、被抢方修复按证据报销或无罪回滚、余款充公铸给主动上链者）；A5 阶段一账单不追溯即 grandfather；A6 人节点置信度语义（照常计分但提醒零强制，不修正乃至负分者人类职责最高穿透机械锁，归现实世界管理，agent 只补救后果）。

B 族既裁补面：B1 锁面定价先记账后算分即阶段一零置信度依赖（GOV-002 v2.3）；B2 开工锁少与首次临时加锁免费（同上）；B3 候选方向照录即类股票市场规则含涨停跌停概念（用户原话）。

### 2.2 数学化范围

- 账户与流水：单一台账、正身开户（账户键位 core_hash 承 identity 契约修订八跨报告配对键，流水逐笔携 identity_hash 保追溯）、追加式上链 → 计数测度与可加性形态（载体候选 PROB-016 先例）
- 余额过程：下限零、余额即支付能力 → 非负受抑过程与序结构（载体候选 ORD-002）
- 置信度语义：余额与置信度同一或经映射二择一，须显式裁定并给依据；若映射，信念更新形态为候选（载体候选 PROB-005）
- 铸币与罚金：机械触发器形式化；罚金下界不等式（事故罚金 > N 次清洁铸币之和）与防刷分通道论证
- 涨停跌停：置信度变动的界机制形式化，参数化不预设数值（B3 候选方向的数学态）
- 抢占闭环：支付算子不可逆性与 fail-closed 形式化；抢占排队的负载面（载体候选 PROB-015）
- 参数槽位：超宽阈值（pk-074 子项一）与未用罚口径（pk-074 子项二）定义槽位形，数值留实证批

### 2.3 推导档与载体

推导档落 sih-math/docs/confmodel-derivation-2026-09-07.md（constmodel-derivation 先例形）。候选载体即 PROB-005／PROB-015／PROB-016／ORD-002 皆数学仓已建条目，按 mapping.md 消费面引用；数学仓无对应的新概念显式申报为提案，走数学仓自身演化程序，本批不代立。

### 2.4 一裁

命题 gid m-confmodel-1，单基线锚 baseline_4 可验证性（m-halfmerge-1 先例形防依据族三值分散）：模型成立性由常数表可机械复算与不变量可机械检验证成。九发 stable_clear 加执契机器终签。

### 2.5 阶段二禁入

本批零引擎代码、零台账实装、零抢占用例实装；阶段二件（置信度台账与付费抢占启动）只在文档面以规格草案态出现。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 推导档落 sih-math/docs（公理照录、定义、不变量与论证、常数候选表、参数槽位、可证伪条件）
- [ ] 常数候选表逐值附模型依据行，数值呈用户裁不代裁
- [ ] 命题区材料 m-confmodel-1 落 sih-tools/proposition/DES/

### Cluster 2：主线串行验证

- [ ] 一裁测量与执契终签落链
- [ ] 管线三步（化格核阅检词，math 仓与命题区域外如实记档）
- [ ] 书单对表即 recall 加 checkcite，引用载体 ID 全落书单及图闭包
- [ ] pk-073 出泊材料落位（唯 F-1 至 F-4 全过后）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 公理完备照录 | 数据治理 | A 族六要件与 B 族既裁逐条进推导档，引文照录零改写零遗漏 |
| **F-2** 零裸常数 | 治理 | 常数候选表与推导档内每个数值或参数槽位都落到定义或论证行，无既裁约束外的裸数值 |
| **F-3** 载体合法 | 跨族治理 | 引用概念 ID 全在数学仓书单及图闭包内即 checkcite pass；新概念显式申报提案态不冒充已立 |
| **F-4** 一裁过 | 跨族治理 | m-confmodel-1 stable_clear 加机器终签在链 |
| **F-5** 零越界 | 治理 | 零引擎代码零台账实装零抢占实装，阶段二件只出现在文档面 |

## 五、必读文件 {#read}

- 要件源：`sih-engine/sih/state/parking/materials/pk-073.json`（六要件与候选方向照录，只读）
- 既裁面：`sih-engine/doc/governance/GOV-002-mainline-lock-v1.md` v2.3 节（锁面定价与免费裁定）
- 实跑面：`sih-tools/lease/CONTRACT.md` 1.29.0 账单节与 lockface-bills.ndjson 事件形
- 载体面：`sih-math/llm-friendly-build/mapping.md`（推理问题先全读再定位概念 ID）
- 先例：`sih-math/docs/constmodel-derivation-2026-09-05.md`（推导档形）；`sih-engine/sih/event/plan/halfmerge-solo-results.md`（单锚一裁形）

## 六、约束 {#constraints}

1. 零代码改动（引擎与工具线零触碰），零在役判据与退出码语义触碰
2. 不代裁：一切数值出候选表呈用户裁；涨停跌停参数同；模型不预设终值
3. 唯一桥梁：哲学散文不入推导档正文；公理引文照录自 pk-073 链上材料
4. 数学仓写入面即 docs/ 推导档；mapping.md 与条目改动走数学仓自身程序不裸奔
5. pk-073 出泊唯在 F-1 至 F-4 全过后，exit 载裁决指向即用户 2026-09-07 立项令
6. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
7. 在泊件零触碰不并批（除 pk-073 出泊）；在盘遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增，checkcite pass
- [ ] 结果档 confmath-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 模型方向发散（市场机制候选 vs 余额即置信度直取）→ 以单锚可验证性收口，二择一显式裁定
- 公理与 billwire 实跑点值冲突 → 回溯支撑或 grandfather 二路显式处置，不静默改数
- 一裁 boundary 不收敛 → 转人诊断呈报，不硬凑

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（写位）、`sih-engine/target/debug/attractor`（一裁与终签）
- 跨仓引用：`sih-math/docs/`、`sih-tools/proposition/DES/`、`sih-tools/facet/contracts/`（测量合同）
- 接口泊件：pk-074（参数槽位对表）、pk-070（基线正典迁离后常数表居所联动）

## 十一、请求写入 {#requested-writes}

- `sih-math/docs/confmodel-derivation-2026-09-07.md`
- `sih-tools/proposition/DES/m-confmodel-1/`
- `sih-engine/sih/state/plan/confmath-solo.md`
- `sih-engine/sih/state/parking/materials/pk-073-exit.json`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/confmath-solo-results.md` 与 `confmath-solo-materials/`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 confmath-solo 工地
