# adjudisp-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 adjudisp-solo，任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/adjudisp-solo.md，先读全文，再读本指令。两步一体：立裁决分派令（DEC-021）+ 得一狗粮回填测量。

## 开工前置

1. lease status 核在途会话；锁操作一律显式 --session <本批会话号>，撞 secondcarr 或他会话锁即停批报告。
2. 正典调用面：/Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md 全程照用。注意核阅正形是 cd sih-engine 后 target/debug/scrutinator --pack des-001 <绝对路径>（包名裸名，引擎件按内嵌名解析）。

## 第一步：分派令立名（DEC-021）

1. 查档：检词 query 裁决分派/AdjudicationRouting/adjudisp 三串须零命中；DEC 清点即 020 为最新。
2. 推导落档 doc/decision/021-adjudication-dispatch.md，体例承 DEC-020，必备节：三态分派表（过得一类：验收打回裁定、融回门开关建议、封窗追认、命题类裁决，各带理由；人节点类：出泊、开门、封窗令本身与一切用户字令；代裁类：纯机械对表与管线读数；灰区默认过得一从严）、适用界即从本批起算既往不回溯、与 DEC-020 硬件钥匙模型的关系即本令立路由不改签署模型、备选方案即全过得一（成本过高弃）与维持现状（自证循环病根弃）。
3. 立名段：治理名裁决分派、代号 adjudisp、英文对 AdjudicationRouting，哲学锚引文从 07-on-assay（自证循环节）与 06-on-canon（损补）程序切片逐字节子串。
4. 检词 register 落册（state established、zh 裁决分派、en AdjudicationRouting、code adjudisp、source 本批、signed 今日），manifest 升 0.5.0；两 skill（sihankor-facet-measure、sihankor-naming）不动，投影核验仅核引用一致性。
5. 走管线：化格 → 核阅（des-001 域内必过零违规）→ 检词（零违例），findings 亲读。

## 第二步：狗粮测量（拒直提守卫命题）

命题：lease 应增 pre-commit 守卫拦无模板 plain git commit，--no-verify 显式留痕放行。

1. 基线先行：用温故或既有链材料导出既有判定基线（sealwin2 与 matcatch 先例的封窗追认材料、goldfix 直提六笔的 reconcile 读数），落材料件。
2. facet 契约模式：emit-contract 出题 → 以席位模型作答 9 发（seat 承 08-31 先例即 ZCode:GLM-5.3-Flash:self-reported）→ score 计分。禁钓样本禁改答，挂起即如实。
3. tally R1-R7 核对：核对得分材料与基线，出 stable_clear 或挂起；stable_clear 即机器终签经 scribe crosscheck --report --material 落 crosscheck_completed 事件，gid 用 adisp-guard-1（非 mbgate 前缀），disposition 按裁决通过或归人如实。
4. 终签材料与得分明细全部落 sih-tools/facet/facet_task_packages/ 或材料件。

## 机械链

ask3 记录（三锚引文程序切片，锚分派令与狗粮两线）→ 双门（工具件 ask3 包加引擎 ask3repeater --root 绝对）→ elicit check（--words 逐词重复）加 digest → 正身 → lease open --package adjudisp-solo（双仓绝对路径）→ 取锁逐路径 → meter 包裹引擎 scribe intent 上链（--trail 当日链绝对路径）→ 工地施工（一切待提交件先进 worktrees 双工地）→ 管线（核阅引擎件）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（主树碰撞按备份让位归并对表法，diff 必须 identical）→ reconcile 双仓保持 unrouted 0 cert_missing 0 → 两日链 verify → 调用册留痕（scribe、formatter、scrutinator、nomenclator、facet、tally、nomenclator 登记行）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## inputlog 补录

sih/event/inputlog/2026-09-02.ndjson 追加 seq 递增两笔，会话号 sess-zcode-260902-acceptor，逐字转录：
- 司衡现有的组件是否已经可以做全态收敛治理了？
- 得一裁1是否能融回
- 为什么得一没有参与到得一落地以后的所有决策？
- 两步立刻执行
（共四笔，seq 按现有尾行递增，ts 用当前 UTC。）

## 红线

判定器融回门不开即 facet 与 tally 围堰原位跑、引擎零改动；狗粮命题最终开权归用户，测量只出裁决材料；分派令既往不回溯；上链前必须等绿；findings 亲读；禁管道掩退出码；正规路径严禁直提；链尾 settle 前后 wc 与末哈希对表；identity/reports 不入册。

## 完工报告

意图哈希、DEC-021 节构、检词登记结果、facet 得分与 tally 终签哈希或挂起实录、认证清单、双仓 commit 号、链 verify、reconcile 读数、F 表、越线与误差申报。
