# confreopen-solo 任务包：置信度线重开（confpreempt 断点续作）

> 令源：用户 2026-09-08 令「拉子代理全开」承置信度线重开候令；续作锚即 09-07 意图笔 05762749 与 confpreempt 保留现场（adjudicate A4 席 stable_clear f4165e2b 裁定保留续作锚）。前置已齐：pk-073 数学模型 2026-09-07 经 confmath 批一裁过出泊（事件 c8a09b5e）。会话号 sess-zcode-260908-forkB-confreopen。

## 开工程式（顺序固定）{#procedure}

1. 盘点现场：读 sih-engine/sih/state/plan/confpreempt-solo.md（原任务包）与 09-07 当日链意图笔 05762749 全文；git 查 msh/confpreempt-solo 分支未归并提交清单（sih-tools 与 sih-engine 两仓）与 confpreempt 保留工地（worktrees/sih-engine/confpreempt-solo 等）实态；读 sih-tools/confledger/ 现状与 CONTRACT 与 sih-math/docs/ 下 confmath 推导档。产出现场盘点节（已毕/未毕/变更三清单）入结果档。
2. 续作承接：新开工地（lease 自建 msh/confreopen-solo），首笔将 msh/confpreempt-solo 分支归并或 cherry-pick 入本支（携带现场，归属链连续），或经对表确认旧提交可直接延续形；取舍在结果档申报理由。
3. 续作实施：按原任务包意图与 confmath 数学模型继续未毕件（抢占制闭环与置信度台账接线），TDD 先红后绿；判定语义变更（若有）须 facet 测量过闸才落（facefit 先例 m-facefit-stamp-1 同形），刀锋即转人诊断零硬凑。
4. 若盘点发现原意图已被后续批次（confmath、confledger、confconst）实质覆盖或矛盾：零硬跑，结果档如实呈报矛盾点与候裁选项，批只收口盘点不实施。

## 产出 {#outputs}

任务包本件、现场盘点节、结果档 sih-engine/sih/event/plan/confreopen-solo-results.md、实施件与测试、ask3 记录与验证件、正身件、租约与锁实录、意图链笔、认证清单、大白话节、机器可读投影 confreopen-readout.json 认证上链、越线与误差申报、双仓或三仓 settle 提交号、链 verify 与 reconcile 读数。

## 红线 {#redlines}

- 断点忠实：旧意图笔与已归并结论零改写，续作从现场出发不重起炉灶；矛盾即停如实呈报。
- 阶段边界：台账与抢占实装照 confmath 模型承载，模型零改动（改模型须另批过得一）。
- 并行窗面零触碰：entrydocs（doc/guide 与 README）、baselineexit（doc/governance 与 AGENTS.md 与泊材料 pk-070）、scrutpath（src 与核阅器源码）；trail 共享追加面照常写；.session-anchor.md 零触碰。
- 主树零直写（本任务包主窗预落除外），链文件只经引擎 scribe 写位，禁管道掩退出码，误差红证如实记档。
- 坑位必防：正身件在场直至 unlock 毕；tally 从工地根 cwd 装配；close 无主闸与 CALL-LOG 闸非本批活面双旗 bypass 留痕不代收；每命令即捕 RC。
