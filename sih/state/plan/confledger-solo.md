# confledger-solo：置信度台账实装批（阶段二第一步）

> 治理任务包（实装类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「下一步工作出提示词和任务包」即阶段二开工令；规格基准即 sih-math/docs/confmodel-derivation-2026-09-07.md（v3，终签链 18ba3476 与 70af572e）；已定型要件源 pk-073 出泊材料；起草笔链笔排队声明即起草时在飞批持链锁，本批意图入链即承载体
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 阶段一记账已在跑（billwire 三类账单事件，租约 1.29.0）但无账户形态：有流水无余额、有账单无置信度
- 台账是标定通道（S1/S2/S3）的数据源，不立台账则涨跌界与罚金参数永无标定之日
- v3 规格已闭环，实装缺位

## 二、关键设计 {#design}

### 2.1 工具位与工作名

sih-tools 围堰孵化（判据四条即通用、纯确定性、可跨仓复用、可带退出码）。工作名 confledger 不立名（meter 先例），立名归后继命名裁决。

### 2.2 台账本体（承 v3 L(t) 与 ORD-019）

单一账本 ndjson 追加式，写点纪律承 pk057fix（flock 串行＋原子整行）；事件五字段 e = (ts, kind, account, amount, ref)，ref 挂链上事件哈希；账户键 core_hash（identity 契约修订八跨报告配对键），流水笔携 identity_hash 保追溯。

### 2.3 铸币触发器（纯机械，A2）

三类：干净收约铸币 +m_clean（=1 归一值）；主动链笔铸币 +m_active（=1 推导值）；修复他因事故铸币 m_repair 槽位态休眠（D_acc 待实证）。自产自修不铸币（判别规则照 v3）。零 LLM 零网络零主观打分。

### 2.4 余额与支付算子

余额纯算术重算，不变量下限零；支付算子本批实装即付不退、fail-closed（余额不足即拒零部分支付）——抢占业务环（lease 锁面与得一裁对接）归阶段二第二批，本批只留算子接口位。

### 2.5 grandfather 与 T₀

T₀ 由本批链上启用笔定义（非人选数，v3 归一通道）；T₀ 前账单零铸币零追溯（A5）。

### 2.6 休眠门控

结算、罚金、D6 分段带与递延队列、I3 全建接口与门控位、激活态全关（B5 阶段一零结算零罚金），激活归标定触发后继批。

### 2.7 验收载体

acceptor 判定包（空腹判定包执行机，2026-09-07 在役）为本批验收载体候选：red_then_green（先红后绿）、double_run（同参双跑逐字节一致）、baseline_freeze（不变量冻结）三查承载 F 表。

## 三、工作清单 {#work}

- [ ] confledger 工具实装：台账、开户、铸币触发器、余额、支付算子、grandfather、休眠门控
- [ ] 账单只读对接：消费 lockface-bills 三类事件（零改租约在役退出码）
- [ ] T₀ 启用笔上链；台账数据面开跑（标定数据自此攒量）
- [ ] acceptor 判定包承载验收，红绿证与冻结件随批材料
- [ ] 管线三步、书单对表、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 台账形态承规格 | 数据治理 | 事件五字段与追加式与 ref 挂链逐项对 v3 L(t)，账户键 core_hash 流水携 identity_hash 双键位测试在档 |
| **F-2** 铸币纯机械 | 治理 | 三类触发判别规则与 v3 一致，自产自修不铸币有测试，grep 零 LLM 零网络零主观打分字面 |
| **F-3** 不变量红绿 | 跨族治理 | 余额下限零（任意重算不出负）、支付 fail-closed（不足即拒零部分支付）、grandfather（T₀ 前零铸币）三不变量先红后绿证在档 |
| **F-4** 常数零新 | 治理 | 消费面仅 v3 三通道已定值（m_clean=1、m_active=1、T₀ 启用笔），槽位态如实申报零新数值 |
| **F-5** 休眠不越权 | 治理 | 结算罚金 D6 递延 I3 全休眠态，激活位零触发；抢占业务环零实装（支付算子除外）；零改租约在役退出码 |

## 五、必读文件 {#read}

- 规格基准：`sih-math/docs/confmodel-derivation-2026-09-07.md`（v3 全档）
- 要件源：`sih-engine/sih/state/parking/materials/pk-073.json`（出泊材料照录）
- 账单先例：`sih-tools/lease/CONTRACT.md` 1.29.0 账单节与 lockface-bills.ndjson 事件形
- 验收载体：`sih-tools/acceptor/CONTRACT.md`（判定包 schema 与词汇表）
- 写点纪律：BATCH-FACE.md 台账写点纪律节（pk057fix）

## 六、约束 {#constraints}

1. 账单面只读消费，零改租约与引擎在役判据与退出码
2. 抢占业务环不在本批（支付算子接口位除外）；标定触发不在本批；激活任何休眠门不在本批
3. 常数消费面仅 v3 三通道表，零新数值零裸常数
4. TDD 先红后绿留痕；acceptor 判定包词汇表内操作，不私扩
5. 主树零直写经工地 settle 通道；链文件只经引擎 scribe 写位
6. 在泊件零触碰；遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过，判定包读数在档
- [ ] T₀ 启用笔上链，台账首日数据面开跑
- [ ] settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增，checkcite 拼接扫描形 pass
- [ ] 结果档 confledger-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- core_hash 取解依赖正身件形——消费 identity 报告先例（lease 与 facet），开工前实测一件
- 账单事件字段漂移（1.30.0 在飞线）——对接面以 CONTRACT 在役版为准，漂移即如实申报
- 台账并发写——写点纪律承 flock 先例，撞锁排队不绕行

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（启用笔与认证写位）、`sih-tools/acceptor`（验收载体）
- 后继：阶段二第二批（抢占业务环：lease 锁面与得一裁对接）；标定触发批（S1/S2/S3 数据面）

## 十一、请求写入 {#requested-writes}

- `sih-tools/confledger/`
- `sih-engine/sih/state/plan/confledger-solo.md`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/confledger-solo-results.md` 与 `confledger-solo-materials/`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 confledger-solo 工地
