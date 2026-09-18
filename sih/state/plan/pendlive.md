# pendlive：pendline 首活即泊界出泊材料处置形复审

> 令源：用户 2026-09-18 裁「b」即甲案复审走 pendline 第一次真活重测；前史即 09-08 总令「所有候裁过得一」下 m-adjudicate2-a1 机器裁甲（出泊件留册计 mainline），hygwave 批 09-18 执行丙形挪位构成事实修订候追认
> 范式：DES-017 编排件首活，collect 加 dispatch 加 backfill 加 route 四子命令全链，判词上游承 facet 机械评分 v3 闸
> stem 认领：pendlive，甲表三件即 zh pendline 首活复审批、code 无承、派生 pendlive:new

## 问题陈述 {#problem}

hygwave 批两线 127 件已出泊材料挪位 parking/historic（零删除零改名）与 09-08 m-adjudicate2-a1 甲案（出泊件留册计入 mainline，机器终签在链）构成事实修订；甲案前提已变即裁时出泊件量少 TTL 多未到期，其后 09-17 集中出泊 23 笔叠加 TTL 到期致陈料滞留计数面心跳告警两线常报。复审命题 m-parkdisp2-a1 送九发采样机械评分，stable_clear 即追认丙形并落名册语义注，boundary 即呈人重写，未过即入泊。

## 关键设计 {#design}

候裁单 JSON 形控 id 即 gid m-parkdisp2-a1；dispatch 档位 9 承裁决先例九发采样形（DES-017 缺省 3 的显式上调，复审机器终签配高档）；回填腿插拔即采样作答九发独立诚实填 raw；判词上游 facet measure score 机械闸 v3 出 gate_verdict 零人工断言；route 三态机械路由落链。facet 执行位在 sih-tools 租约工地（飞轮 trail 落工地 proposition/DES 不触主树）。谱系披露入命题文即起草与作答同席对己不利声明两向负担如实。

## 工作清单 {#work}

- [x] pl-01：命题与候裁单与 topic 落盘 T6
- [x] pl-02：collect 加 dispatch 九发出合同
- [x] pl-03：九发回填加 facet 评分出 gate_verdict
- [x] pl-04：backfill 加 route 三态落链
- [x] pl-05：执行腿按判词（stable_clear 即名册语义注 T6；boundary 呈人；未过入泊）
- [ ] pl-06：settle 加 close 加结果档

## 验收 {#acceptance}

四子命令全链实跑读数在档；gate_verdict 机械出零人工断言；route 落链事件在链可对账；执行腿与判词一致；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/sih/event/plan/pendlive-materials/
- sih-engine/doc/governance/PARKING-v1.md
- sih-tools/proposition/DES/
- sih-engine/sih/state/plan/pendlive.md
