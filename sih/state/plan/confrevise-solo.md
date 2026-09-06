# confrevise-solo：置信度裁定值落位修订批

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：m-confrule-1 三件裁定值落位（机器终签 18ba3476 stable_clear direction comply 即授权），本批纯机械执行零设计自由度；起草笔链笔排队声明即起草时 acceptorimpl-solo 持链锁，本批意图入链即承载体
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 三件裁定值已落据成规则但推导档 v2 仍是裁前文本：N_min 仍挂标定输入式、D6 仍两形并列、S3 输出面仍含形一参数
- 文本与规则不一致即契约失真，须升版落位

## 二、关键设计 {#design}

- 推导档升 v3 三处：P2 立则即 N_min 推导式改 ⌊U⁺/m_clean⌋ + 1 并撤提案四挂起态；P3 定形即 D6 定形二分段带 [−U⁻, U⁺]，形一与 U := max(U⁺, U⁻) 入版本史退场；S3 输出面随形二收窄
- S1 消费面变化如实申报：n_max* 不再喂 N_min，留监测读数位；去留属规则面变化，本批申报不代裁
- 版本史记令源（用户得一裁令）与终签号 18ba3476；零新一裁（终签即授权，本批不重裁不重采样）

## 三、工作清单 {#work}

- [ ] 推导档 v3 升版三处改式与版本史
- [ ] 全档交叉引用一致性核对（改式波及处 grep 复核）
- [ ] 确定性对表：改式结果与裁定值表逐项 diff
- [ ] 管线三步、书单对表、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 改式对表 | 数据治理 | 三处改式与 m-confrule-1 裁定值逐项 diff 零偏差 |
| **F-2** 公理零改动 | 治理 | A 族 B 族逐字节不变 |
| **F-3** 版本史完整 | 数据治理 | 升版记令源、终签号、差异申报三件齐 |
| **F-4** 零重裁零越界 | 治理 | 不跑新一裁不重采样；零代码零数值断言；S1 去留只申报 |

## 五、必读文件 {#read}

- 裁定源：`sih-engine/sih/event/plan/confrulegate-solo-results.md` 三件裁定值表
- 修订对象：`sih-math/docs/confmodel-derivation-2026-09-07.md`（v2）

## 六、约束 {#constraints}

1. 只改裁定覆盖面，三处之外零触碰
2. 边界情况（S1 去留、形一退场注记形态）如实申报不代裁
3. 主树零直写经工地 settle 通道；在泊件零触碰；遗留无主件不代清

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过，settle 在档，链 verify valid，reconcile 零新增
- [ ] 结果档 confrevise-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 改式波及交叉引用遗漏 → 全档 grep 一致性核对兜底
- acceptorimpl-solo 在飞持链锁 → 本批开工撞锁即排队候叫不绕行

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（写位）
- 后继：阶段二台账实装批（规格面以 v3 为准）

## 十一、请求写入 {#requested-writes}

- `sih-math/docs/confmodel-derivation-2026-09-07.md`（v3 升版）
- `sih-engine/sih/state/plan/confrevise-solo.md`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/confrevise-solo-results.md` 与 `confrevise-solo-materials/`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 confrevise-solo 工地
