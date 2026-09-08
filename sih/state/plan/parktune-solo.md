# parktune-solo：泊界侧轨积压告警调优——gate 远景件豁免计数

> 治理任务包（实装类，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：parkgate-solo 批（2026-09-07 落 gate 远景廊）后遗留配置性误报——带 gate 的远景件 pk-070 与 pk-077 落 siding 轨触发 siding_surplus 告警（阈值二，现侧二即告警），gate 件入侧轨是设计行为非积压，主会 2026-09-08 编排令即 FORK-2 窗口段
> 并行协调：reroute-solo（FORK-1）在飞或即将开租约，本批写入面与其零交集；废轨修复与 confpreempt 收口不属本批
> 日期：2026-09-08

## 一、问题陈述 {#problem}

- 路择 parking 包（双载体逐字节同源即 sih-tools/selector/packs/parking/ 与 sih-engine/src/attractor/packs/parking/，SPEC-015）告警语义：siding_surplus_threshold=2 把侧轨件数超阈当积压告警，但 gate 远景件（出泊条件挂未来事件者）入侧轨是 parkgate 设计行为，非积压——每日心跳误报一条，稀释告警可信度（狼来了效应，违基线三注意力只投异常）
- 修复语义须保真：真积压（非 gate 件侧轨超阈）仍须告警；豁免只及 gate 件

## 二、关键设计 {#design}

- **先勘后改**：实读 routes.toml 与 manifest 与 pk-070/pk-077 材料实态字段（gate 形态以盘面为准），确定 gate 件豁免可否由包数据表达（如谓词重排或 alarm 节扩展）；若包数据不可表达即改引擎告警逻辑（围堰 selector 与引擎 attractor/route 两侧同改），数据形与代码形选择及依据逐条落批材料，不猜
- **双载体纪律**：SPEC-015 双跑对表——围堰包与引擎包改动后须逐字节一致（cmp），引擎包经 include_str! 编译期内嵌须 cargo build 重编主树后再验；金向量八场景若因包数据合法变更而变，按 GV3 随冻重录（期望重冻、消费逻辑零改），重冻依据落材料
- **验收读数**：改后 --reference-time 2026-09-08 路由两线，pk-070/pk-077 仍在侧轨（或其设计轨）但 siding_surplus 告警零；构造真积压夹具（三件非 gate 侧轨件）证明告警仍触发——红绿证俱在档

## 三、工作清单 {#work}

- [x] 勘察：routes.toml 与 manifest 现文、pk-070/pk-077 材料字段、双载体 cmp 基线、金向量现状
- [x] 温故检索（落包前主会编排令在档；本批自跑 recall --topic 补档，零命中如实记）
- [x] 三问双门→叩问→正身→租约→意图→工地施工（数据形或代码形按勘察定）
- [ ] 管线三步→checkcite→认证→双仓 settle→放锁 close→对账对表→verify→回填 bypass→CALL-LOG
- [ ] 结果档 parktune-solo-results.md 落 event/plan

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 误报灭真报留 | 实装 | 改后实跑两线路由：pk-070/pk-077 在侧轨零 siding_surplus 告警；真积压夹具≥三件仍触发告警，红绿证在档 |
| **F-2** 双载体一致 | 跨族治理 | 围堰与引擎包 cmp 逐字节一致（或两侧代码同改后同参双跑输出一致），引擎重编后验证，金向量按 GV3 处置在档 |
| **F-3** 双跑确定性 | 跨族治理 | 改后同参同日双跑 cmp 逐字节 IDENTICAL 退出码一致 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径，reroute-solo 面（GOV-002、critsweep、facet、DES、引擎泊材料）零触碰 |
| **F-5** 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 |

## 五、必读文件 {#read}

- `sih-tools/selector/packs/parking/routes.toml` 与 `manifest.toml`；`sih-engine/src/attractor/packs/parking/`（双载体）
- `sih-engine/sih/state/parking/materials/pk-070.json` 与 `pk-077.json`（gate 字段实态）
- `sih-tools/BATCH-FACE.md`（路由调用面节、坑位勘误全节、金向量重放 cwd 约定）
- 先例：`sih-engine/sih/event/plan/parkgate-solo-results.md` 与 `critsweep-solo-results.md`

## 六、约束 {#constraints}

1. 零子代理；.session-anchor.md 与 .zcode/config.json 与 anchor.py 零触碰（跨窗口竞态在案）
2. reroute-solo 在飞面零触碰；撞锁不绕行走 wait-turn 或候批重试，禁 bypass 抢锁
3. 主树零直写；禁 plain commit；禁管道掩退出码；先红留痕；md 认证走内容哈希清单形
4. 泊材料本体（pk-070/pk-077 等任何在泊件 JSON）零改动——本批只调告警语义不改泊件
5. 真积压判定阈值与 gate 判定字段名以勘察为准，语义变更落两册 CONTRACT（selector 与 attractor）修订记档

## 七、验收标准 {#acceptance}

F-1 至 F-5 全过；收约后零本批活跃锁零活跃会话；结果档落 event/plan；改后心跳读数（引擎线告警零）入结果档。

## 八、风险点 {#risks}

- 引擎代码改动牵金向量重冻与 cargo 重编，批期变长——数据形可表达即优先数据形，取舍依据落材料
- reroute-solo 若与本批争报告目录锁即 append 形互不憋或排队候叫，如实记等待

## 九、范畴排除 {#exclusions}

- 废轨 9 件修复（撞 reroute 泊材料面）、confpreempt 收口（候 W1 后）、gate 谓词语义扩展新功能、critsweep 术语收编，皆不属本批

## 十、请求写入 {#requested-writes}

- sih-tools/selector/packs/parking/
- sih-tools/selector/src/selector/
- sih-tools/selector/pyproject.toml
- sih-tools/selector/CONTRACT.md
- sih-tools/selector/tests/
- sih-engine/src/attractor/packs/parking/
- sih-engine/src/attractor/route.rs
- sih-engine/src/attractor/CONTRACT.md
- sih-engine/src/attractor/fixtures/route/
- sih-engine/tests/attractor_route.rs
- sih-engine/sih/state/plan/parktune-solo.md
- sih-engine/sih/event/plan/parktune-solo-results.md
- sih-engine/sih/event/plan/parktune-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/parktune-solo
- worktrees/sih-engine/parktune-solo

> 2026-09-08 勘察修订（批内修订，承第二节先勘后改条款）：原表漏列四路径即 sih-tools/selector/src/selector/（告警消费逻辑 route.py 与 pack.py 在此）、sih-tools/selector/pyproject.toml（版本进位惯例）、sih-engine/src/attractor/route.rs（引擎侧同改面）、sih-engine/tests/attractor_route.rs（t6 谓词数断言随包四谓词适配）。勘察定夺数据驱动的代码形即纯包数据不可表达豁免语义，代码路径为代码形必写面，修订依据落批材料与结果档越线申报节。
