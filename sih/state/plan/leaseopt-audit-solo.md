# leaseopt-audit-solo：租约撞车盘点批（leaseopt 线批一）

> 批：leaseopt-audit-solo（leaseopt 线批一，纯脚本零行为变更盘点，单线 solo 零子代理）
> 令源：用户 2026-09-04「租约的优化也要加入，多agents协调一直撞车」+「锁竞态导致多agents冲突，空转消耗token对我来说比较重要」；线总纲 sih-engine/sih/state/plan/leaseopt-line-v1.md 排序批一必先
> 先导检索：wikirecall recall --query 租约冲突多agent协调（2026-09-04 本批复跑）词面通道零命中、语义通道低分长尾 ≤0.062、图通道经 ORD-020/ORD-021 调度秩序载体扩散 17 件书单，无租约冲突专门命题，与线总纲「检索面未覆盖」记载一致；证据底座为台账与样本库与 2026-09-04 晚主会一手查证

## 一、问题陈述 {#problem}

多 agent 并行批持续撞车：收约失败台账累计 176 笔（sih-tools/lease/ledger/sessions.ndjson close_failed 事件）；pk-045 冲突样本库在泊二十余类；2026-09-04 晚 mainord-solo 批单批三连撞。用户点名两级痛点：锁竞态致批间冲突、撞锁空转重试消耗 token。本批是 leaseopt 线的盘点批，产出撞车分类账本，为批二至批六的次序重排提供数据基线。

## 二、病灶底座（线总纲第二节六项，本批逐件核验入账） {#evidence}

1. 收约守卫假阳性：lease/src/lease/core.py detect_merge_conflicts 以「盘上内容 ≠ 分支内容」判脏位不核真实 git 脏位（对 HEAD），2026-09-02 修订二十九「严义不看 base」后凡批期内基线前进过的文件（主树净态）被误判 diverged 整批拒
2. 无租约活写零实时告警：主树直写绕工地只在收约时被动撞见
3. 僵尸会话：会话撤销 ≠ 进程死亡，无回收程序，ledger 有 session_orphan 类在档
4. 锁竞态与空转 token：exclusive 撞锁有限重试上限十次逐次计数、共享追加面 --mode append 短持即取即放、并行批两批上限（现行补丁纪律），补丁存在本身即病灶证据
5. 无协调视图：谁持何锁、在途批施工面交集、冲突预测无可视面板
6. 空转无度量：meter counts 有逐日调用计数但未与撞车事件交叉

## 三、工作清单 {#work}

- [ ] 台账全量读取：sessions.ndjson close_failed 全量、locks.ndjson、bypass.ndjson、meter/counts 逐日计数、PARKING-v1 pk-045 样本库记载、mainord-solo-results.md 与 gov002v2-solo-results.md 撞车实录
- [ ] 盘点脚本落工地（纯读零写台账），产出撞车分类账本：每类频次、处置现状（已硬化在役 / 裸奔）、平均 meter 调用代价即 token 代理量、修复批次归属映射到批二至批六
- [ ] 已知病灶逐件核验：病灶一 detect_merge_conflicts 判脏位读源码核验；病灶二 attractor R5 缺围堰修订六核哈希优先配对（gov002v2-solo-results.md 缺陷节在案）
- [ ] 结果档 sih-engine/sih/event/plan/leaseopt-audit-solo-results.md：F 表、账本指针、复算读数、越线与误差申报、冲突样本节、批二至批六次序重排建议呈人节点
- [ ] 管线三步（化格→核阅→检词）→认证逐笔→双仓 settle→放锁→close→reconcile→当日链 verify

## 四、红线 {#redlines}

- 零行为变更零源码改动（本批纯盘点，台账与工具与文档主体零触碰）
- 判定语义不动；主树零直写，一切经工地
- 并行纪律照现行（两批上限、重试上限十次逐次计数、append 短持）直至批四替代
- 账本 token 代价列必须有数不许估；零人工填项，脚本可复算逐字节一致
- 批二至批六只出重排建议呈人节点，不自行开工

## 五、可证伪条件 {#falsifiable}

| F | 判据 |
|---|---|
| F-1 | 账本由盘点脚本产出，复算逐字节一致 |
| F-2 | 账本与台账 diff 对表零漏项：close_failed 全量逐笔归类 |
| F-3 | token 代价列每类有数（meter 计数实算），不许估 |
| F-4 | 病灶六项逐件核验结论在案，源码引证带行号 |
| F-5 | 处置现状二值标注（已硬化在役 / 裸奔）逐类落实 |
| F-6 | 修复批次归属映射到批二至批六逐类落实 |

## 六、请求写入 {#requested-writes}

- sih-tools/proposition/DES/leaseopt-audit/（盘点脚本与账本与复算材料）
- sih-tools/lease/CONTRACT.md（版本行随批，纯盘点零语义变更仅版本行注明批一盘点引用）
- sih-engine/sih/event/plan/leaseopt-audit-solo-results.md（结果档）
- sih-engine/sih/event/plan/leaseopt-audit-solo-materials/（例扫读数材料）
- sih-tools/scribe/reports/ 与 sih-tools/identity/reports/ 与 sih-tools/meter/counts/（管线例面）
