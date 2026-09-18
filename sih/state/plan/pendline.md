# pendline：候裁处置范式编排件实装批

> 令源：DES-017 候裁处置范式编排设计 v1（2026-09-18 des017 批入正典提交态）加用户 2026-09-18 令「同意，我希望你作为主编排，拉起子代理顺序完成一 二 三」第三件即本实装批
> 范式正典：sih-engine/doc/design/DES-017-pending-adjudication-orchestration-v1.md；人工活证：sih-engine/sih/event/plan/mcpdual-parallel-results.md 第九节
> stem 认领：pendline，甲表三件即 zh 候裁处置范式编排线、code 无承、派生 pendline:new

## 问题陈述 {#problem}

候裁材料（泊位发现与批候裁单与普查推荐）现由主会人工串联：逐件分派得一裁、逐件读判词、逐件定执行或入泊。DES-017 已钉编排语义：候选收集归 critsweep 出参与泊界名册与批候裁单汇总，分派按档位调采样裁定，处置路由三态即过者走执行位、未过者走泊位通道、boundary 者走人重写处置位；编排层零新判定语义，critsweep 与 attractor 与 lease 三件零触碰。

## 关键设计 {#design}

- 落位（实装批裁定，承设计档授权）：独立引擎件 `pendline`（src/bin/pendline.rs 与 src/pendline/ 模块），不挂 critsweep 出参旁——critsweep 契约为零写回算器，编排件有落链写动作，独立件职责边界清。
- 三段编排：collect（汇总判据扫出参与泊界在泊名册与候裁单材料）加 dispatch（按档位生成采样合同）加 route（判词三态路由与落链）。
- 档位面：3/6/9 三档可关，缺省 3，档位即采样发数语义；关即零采样候裁单直呈人节点全人工。档位读写面取 CLI 旗标加配置件持存两形齐备，值域与缺省冻结承设计档。
- 采样回填腿插拔形：本批实装 collect 加合同生成加路由加落链全机械链；模型回填腿（围堰两段式的回填半）保持插拔接口即编排件出合同收回填件，活模型回填不进本批（人节点序），测试以夹具回填件承载。
- 落链形：过者执行位标记（建议批承载或直改笔两通道留痕）、未过者 scribe park 停泊事件（title 与 exit_condition 与 ttl 与 context 照录判词要害）、boundary 者人重写标记不自动重测不自动入泊；全路由留痕可对账。
- 零新执法位：编排层只收集分派留痕，三既有件判定正典与退出码语义零触碰。

## 工作清单 {#work}

- [x] pl-01：collect 面即三源汇总与候裁单解析（src/pendline/collect.rs，判据沉底机械召回加泊界 trail 只读重放加候裁单 md/json）
- [x] pl-02：dispatch 面即档位配置与采样合同生成与回填插拔接口（src/pendline/dispatch.rs 与 config.rs，facet 合同模式经 attractor contract_mode 内核；backfill 子命令严格对表 shots 判词上游直读）
- [x] pl-03：route 面即三态路由与 scribe park 落链与执行位标记（src/pendline/route.rs，写链全走 scribe 子进程零直接写 trail）
- [x] pl-04：CLI 面即子命令与配置件持存（src/bin/pendline.rs 五子命令 collect/dispatch/backfill/route/config）
- [x] pl-05：测试族即夹具回填件驱动全路由与档位与落链断言（tests/pendline.rs 十五件全绿：三态全链各一加混批加档位 off/3/6/9/缺省/非法/配置件两面加零触碰 git 断言）
- [ ] pl-06：settle 加 close 加结果档

## 验收 {#acceptance}

夹具回填件驱动下三态路由各至少一件绿且落链断言过；档位 off/3/6/9 旗标面与缺省 3 在役；critsweep 与 attractor 与 lease 三件生产码零 diff；主树全量回归零新红；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/src/bin/pendline.rs
- sih-engine/src/pendline/
- sih-engine/tests/pendline.rs
- sih-engine/sih/state/plan/pendline.md
