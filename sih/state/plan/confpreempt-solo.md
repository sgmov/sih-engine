# confpreempt-solo：置信度抢占业务环批（阶段二第二批）

> 治理任务包（测量类＋实装类合一，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「开，委外」即阶段二第二批开工令；规格基准即推导档 v3 与 confledger v0（在役，八子命令）；要件源 pk-073 A4 抢占制闭环；规则面未定件按用户既裁路由得一裁、数值出生走三通道
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- A4 钉死了环的形状（付不退、得一裁事实与理由、放锁落链、被抢方按证据报销或无罪回滚、余款充公铸给主动上链者），但三件规则面未定：触发条件（何种事实构成可抢占）、价格机制形（固定价／比例／竞价类）、报销证据标准
- confledger v0 已留支付算子接口位（付不退、fail-closed、配对守卫），lease 侧已有 takeover 死会话清理与 wait-turn 排队先例可循，对接面未通

## 二、关键设计 {#design}

### 2.1 规则面一裁（命题 m-confpreempt-1，单锚 baseline_4）

三判定点：①触发条件族——候选事实从在役机制取即锁龄、会话心跳停滞、PID 探针死活（lease takeover 先例）、排队候叫深度，何者构成可抢占事实；②价格机制形——付不退与余款充公语义相容的机制形（数值出生仍走三通道：机制可裁、数值零硬编码，需数值即挂槽位或推导）；③报销证据标准——何种证据可机械见证（链上 ref 形）。stable_clear 落据成规则，boundary 转人诊断。

### 2.2 环机械实装（公理钉死零自由度）

五步环：抢占方经 confledger pay 付置信度（欠账未清拒、余额不足整笔拒 fail-closed、即付不退）→ 得一裁事实与理由（裁定值按 2.1 落地）→ 放锁落链（锁释放加链笔，lease 台账与链对表）→ 被抢方二路处置（无罪回滚即机械回滚零罚金；报销挂 m_repair 槽位休眠记 IOU，D_acc 标定后兑付）→ 余款充公铸给主动上链者（active_pen 同类铸造通道）。

### 2.3 集成面

lease 侧新增子命令 additive（如 preempt 类），消费 confledger 支付证明与得一裁落据；零改在役子命令退出码与判据。confledger 侧新增环命令 additive，支付算子零改动。

### 2.4 贫账休眠申报

机制建成即休眠于贫账期（现实余额不可触发价格即拒），申报态非缺口；m_repair 报销槽位态同 v3。

## 三、工作清单 {#work}

- [ ] 命题 m-confpreempt-1 起草落命题区（三判定点，材料照录 A4 与在役机制原文，程序切片禁手打）
- [ ] 一裁测量九发携正身、机器终签落链
- [ ] 环机械五步实装＋lease 与 confledger 双侧 additive 对接
- [ ] acceptor 判定包验收（red_then_green、double_run、baseline_freeze）
- [ ] 管线三步、书单对表、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 规则面裁过 | 跨族治理 | m-confpreempt-1 stable_clear 加机器终签在链，三判定点裁定值随档；boundary 如实转呈不硬凑 |
| **F-2** 环闭合 | 数据治理 | 五步环测试夹具走通（付→裁→放锁落链→处置→余款充公），链笔与台账对表 |
| **F-3** 四守卫红绿 | 跨族治理 | 付不退、欠账拒抢占、余额不足整笔拒、无罪回滚零罚金——四守卫各有先红后绿证 |
| **F-4** 零越界零常数 | 治理 | 零改租约与 confledger 在役退出码（新增面 additive）；代码零新数值硬编码（价格数值走三通道，需槽位即槽位态） |
| **F-5** 休眠如实 | 治理 | 贫账期休眠与 m_repair 槽位态如实申报，激活路径显式 |

## 五、必读文件 {#read}

- 规格基准：`sih-math/docs/confmodel-derivation-2026-09-07.md`（v3）
- 要件源：`sih-engine/sih/state/parking/materials/pk-073.json`（A4 原文）
- 在役面：`sih-tools/confledger/`（v0 与支付算子）、`sih-tools/lease/CONTRACT.md`（takeover 与 wait-turn 与账单节）
- 一裁先例：`sih-engine/sih/event/plan/confrulegate-solo-results.md`（命题形与坑位）

## 六、约束 {#constraints}

1. 规则只从一裁出；数值只从三通道出；公理环机械零自由度
2. 采样从工地 facet 跑、计分携正身；boundary 不重采样凑共识
3. lease 与 confledger 在役面零改动，新增 additive；主树零直写经工地
4. 在泊件零触碰；遗留无主件不豁免不代清
5. 抢占真实触发前须双重门槛：裁定值生效＋余额充足，缺一即休眠

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过，判定包读数在档
- [ ] settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增，checkcite 拼接扫描形 pass
- [ ] 结果档 confpreempt-solo-results.md 落 event/plan，裁定值随档

## 八、风险点 {#risks}

- 裁定值与在役 takeover 语义冲突（死会话清理 vs 经济抢占边界）——冲突如实申报呈裁不硬并
- 贫账期无真实触发样例——夹具内全环走通为验收面，真实触发留待余额期
- 撞在飞批锁即排队候叫

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/attractor`（一裁）、`sih-engine/target/debug/scribe`（链笔）、`sih-tools/acceptor`（验收）
- 后继：标定触发批（S1/S2/S3 数据面与 D_acc，报销兑付前提）

## 十一、请求写入 {#requested-writes}

- `sih-tools/proposition/topics/`（m-confpreempt-1 命题）
- `sih-tools/proposition/DES/m-confpreempt-1/`
- `sih-tools/confledger/`（环命令 additive）
- `sih-tools/lease/`（preempt 类子命令 additive）
- `sih-engine/sih/state/plan/confpreempt-solo.md`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/confpreempt-solo-results.md`
- `sih-engine/sih/event/plan/confpreempt-solo-materials/`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 confpreempt-solo 工地
