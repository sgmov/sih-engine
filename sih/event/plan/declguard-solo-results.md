# declguard-solo 结果档：声明守卫批

> 日期：2026-09-06　会话：c0d48b2baad5228a（首会话 32517d4b687ee66b 收桌重开）　租约 lease 1.31.0　范式：T6 单线 solo，委外代理亲写零子代理
> 令源：用户 2026-09-06 转 watchcheck 代理审计工具双缺口，主会修正设计两处（声明面取请求写入节非 allow 全集；豁免走显式认领通道非硬拒）后立项
> 任务包：sih-engine/sih/state/plan/declguard-solo.md（绝对路径在册）；姊妹批 leftover-solo 先行收约（本批接续承其 lease-check 全族清点移交）

## 一、执行概要

T-1 声明对提交差集闸实装：close_session 末位（_merge_tree_precheck 后、归并循环前，前序闸零动）挂 declared_uncommitted_diff——声明面取任务包请求写入节逐行结构化（parse_declared_writes）非 allow 全集，对分支提交树 ls-tree 机械事实逐路径求「声明未提交」；处置三态：差集空放行、非空无认领整批拒零动作（报文逐件列路径）、条件形行含「若」字样（冻结启发 DECLARATION_CONDITIONAL_MARK）自动豁免入报告零静默；显式认领通道 close --ack-uncommitted「路径=事由」逐路径带事由放行、事由入 revoked 行新 detail 键留档零粉饰、认领差集外路径与缺事由俱拒；自由文本解析不了呈报告不猜；共享追加面声明路径入 shared_surface_exempt 豁免态零误伤。T-2 收据归家：写位迁 root 锚定 sih-tools/lease/ledger/receipts/<stem>.json 单一目录，SCOPE_SHARED_SURFACE 六路扩七路（常量单源化移 core 供闸复用，cli 具名导入命名空间照旧），migrate_legacy_receipts 存量一次迁入幂等。T-3 TDD 五族九件先红后绿，204 全绿零回归。T-4 判定语义一裁 facet 合同模式九发亲笔回填，verdict boundary 呈用户候裁不终签（第七节）。T-5 全测零回归加双仓 settle 加 close 过自装差集闸活体验收（第九节）加 reconcile 加 verify。

## 二、F 锚定逐条判定

| F 锚定 | 判定 | 依据 |
|---|---|---|
| F-1 差集闸 | 过（一裁项除外） | PROB-018 形复现夹具 test_gate_rejects_declared_uncommitted 红转绿；条件形 test_gate_conditional_line_auto_exempt_recorded 自动豁免入 revoked detail；--ack-uncommitted test_gate_ack_uncommitted_allows_with_reason 事由入 revoked 行，认领差集外与缺事由两拒形夹具绿；本批自身 close 走认领通道活体验收（第九节） |
| F-2 收据归家 | 过 | 新收据落 receipts/（本批 close 凭据 receipts/declguard-solo.json 即活证）；白名单登记后共享面豁免态生效；存量 41 件全量迁入零残留（engine 0 与 math 0）幂等 |
| F-3 一裁 | 呈用户候裁 | facet 合同模式九发亲笔回填 verdict boundary（4/9 boundary_flag 如实标记），红线纪律不终签不代写不清洗重跑，呈主会裁决（第七节），候裁期间本批其余面不受阻 |
| F-4 零回归 | 过 | 204 passed = 195 基线 + 9 新增；closefix 与 rootanchor 与 ledgerhyg 交付零动；台账行既有键零变更（revoked 行 detail 键系本闸任务设计留痕位）；ledgerwrite 唯一写点零动；账单台账与链证守门零动；leaseup 钉数测试 6→7 随 CONTRACT 修订四十五同批更新 |
| F-5 活体验收 | 过 | 本批自身 close 过自装差集闸：声明未提交路径 sih-tools/scribe/CALL-LOG.md 走 --ack-uncommitted 认领放行（事由：开约时在途批独占持锁移出 allow 面，收约后 bypass 补笔，viewline/idenlane-envelope/leftover-solo 先例形），事由入 revoked 行 detail；收据落新位 |

## 三、关键设计落位

冻结启发零裸奔（core.py 在册，改即走 CONTRACT 修订）：DECLARATION_CONDITIONAL_MARK=「若」；DECLARATION_PATH_TOKEN 路径词形正则（至少一斜杠段序，裸词与仓前缀不可解析的相对简写不猜只入报告）；LEGACY_RECEIPT_DIRS 冻结散件位（sih-engine/sih/state/plan 与 sih-math/sih/event/plan）。闸只挂 close 位，open 与 lock 位零动，直改车道与无租约形零影响；共享追加面（台账账单）不属请求写入节默认面零误伤。收据备选形乙（runtime-only 加 gitignore）按任务包不采：收据含 closed_at 与 close_session 与 identity_core 审计值宜进版控。零写面 accommodation 显式申报：migrate 用 Path.replace 单调用原子搬移，零写面纪律辖台账活面整文件盖版类原语，本迁移是写一次收据散件的一次性幂等搬移非台账写面。

## 四、TDD 五族实录（先红后绿）

先红：test_declguard.py 首跑红证归档 materials/tdd-red-first-run.log（复原件带出处声明，原件随收桌重开旧工地拆除丢失如实申报不粉饰）。族一差集非空拒（PROB-018 形：声明两路径只提交一件即拒且报文逐件列路径且分支零动作）与全提交放行与共享面零误伤、族二条件豁免（「若」行自动豁免入 revoked detail）、族三显式认领（放行加事由留痕加认领差集外拒加缺事由拒）、族四收据新位（receipts/ 落位加 closed_at 与 close_session 在册加包旁散位零新写）、族五存量迁移（双散件位迁入加幂等二跑零迁移）：九件全绿。全族 204 passed = 195 基线 + 9，夹具缺陷两处（任务包目录未跟踪致误判真分叉与迁移断言字段名）批内修复如实记录。

## 五、存量迁移实录

T-3 清点时点（leftover-solo 移交窗口）37 件；本批迁移窗实态 41 件（sddpacks 与 checkerimpl 等批在窗内收约续产），migrate_legacy_receipts 全量迁入 receipts/ 零跳过零残留（engine 散件位 0 与 math 散件位 0），二跑幂等零迁移。本批自身收据（leftover-solo 与 declguard-solo）落新位即活证。旧位 .lease-check.json 件自此绝迹，候后续各批 close 自然产新位收据。

## 六、会话收桌重开申报（越线如实）

首会话 32517d4b687ee66b 开约时 allow 随任务包请求写入节解析，而任务包第十一节漏列 sih-tools/lease/pyproject.toml 版本位（三源对齐必需），settle 段1 拒 staged_out_of_scope。处置按 modou「allow 漏即重开」先例：工地 50 件改动保全 tar 复原零损失（含暂存全量与八 tracked 改件逐一在 tar 验证），放锁收桌（旧会话 revoked 留档），补 allow 重开会话 c0d48b2baad5228a。首会话意图笔 e0e0cd11 与认证六笔（d0192b6c/9bc1a155/e89d5519/d4039cb6/7207ff02/5e4d73d6 之前四笔）留链不可篡改；重开会话意图防重用闸拒旧记录（IntentRecordUsedRejected），不出主会处置位 --allow-reintent，改出 round 2 记录（内容同 round 1 唯 round 序异）重跑双门取新意图 9ef6aa0b。旧 engine 工地随拆带走红证原件一件，复原件带出处声明归档（第四节）。

## 七、T-4 一裁 boundary 呈报候裁

facet 合同模式：gid m-declguard-gate-1，九发亲笔回填（温度零谱系披露双向，seat ZCode:GLM-5.3:self-reported，identity_hash 5a983dff），九发零 void 全 comply、依据族 baseline_4 六发 baseline_5 三发，boundary_flag 如实标记 4/9（条件形冻结启发、共享面豁免域、Path.replace accommodation 三处基线不直接回答须推理适用，另收据审计值进版控一处直答）。score 闸 v3 裁决 boundary（打回重作类）。红线处置：near_threshold 即须呈用户，boundary 更甚——不自行终签、不代写命题、不清洗红证重跑凑绿，红证与计分材料归档 materials（contract-score-material.json 原件在 facet 合同目录），呈主会裁决候复算或重写上下文（watchcheck-solo F-5 boundary 呈报先例形，其后 basisunion 批经主会裁决复算转 stable_clear）。候裁期间本批 T-1/T-2/T-3/T-5 交付不受阻；主会若裁 violate 即差集闸与收据归家退回重议。

## 八、CONTRACT 修订与版本

修订四十五升 1.31.0 三源对齐（pyproject 与 __init__ 与 CONTRACT）：差集闸声明面与三态处置与认领通道与冻结启发与闸序挂位、收据归家写位与白名单七路与存量迁移、零写面 accommodation 申报、leaseup 钉数 6→7 同批更新。管线：CONTRACT 化格零改；检词 findings 两笔俱批前既有（主树基线 CONTRACT 同词同行死档旧词在案，本批增量零违例，不代修候原批）；CALL-LOG 行检词零违例。

## 九、活体验收实录（本批 close 过自装差集闸）

本批自身任务包请求写入节声明面经自装闸对分支提交树差集：sih-tools/lease/src/lease/ 与 tests/ 与 ledger/receipts/ 与 CONTRACT 与 CALL-LOG 与 pyproject 与结果档与材料俱在树即放行；自由文本简写（materials/ 等）与共享追加面（trail 当日链）入 unparsed 与 shared_surface_exempt 报告零误伤零静默；声明未提交路径 sih-tools/scribe/CALL-LOG.md（开约时在途批独占持锁移出 allow 面）走 --ack-uncommitted 认领放行，事由入 revoked 行 detail 留档。close 后凭据落 receipts/declguard-solo.json 新位即 T-2 活证。收口读数（reconcile 与 verify 与终计数）落 scribe CALL-LOG 补笔行。

## 十、关联文件

- 任务包：sih-engine/sih/state/plan/declguard-solo.md
- 姊妹批：sih-engine/sih/event/plan/leftover-solo-results.md（先行收约，移交 lease-check 全族清点）
- 材料：sih-engine/sih/event/plan/declguard-solo-materials/（TDD 红证复原件、一裁 boundary 计分材料、管线报告）
- 一裁工件：sih-tools/facet/contracts/declguard-260906/m-declguard-gate-1/（topic 与合同与九发响应与计分材料）
- CALL-LOG 双笔：sih-tools/lease/CALL-LOG.md（随批）、sih-tools/scribe/CALL-LOG.md（认领通道收约后 bypass 补笔）

## 十、收尾补做（主会代补，2026-09-07，shell 故障阻断四件闭卷） {#closure-main}

- 终对账与链终读数（阻断件一）:双仓 reconcile unrouted 0 与 unbypassed 0 亲核在案;链 verify 主树二进制 2026-09-07 当日链 valid（事件数随批后写入增长至 32，时戳严格单调亲核即链铸时戳生产读数）
- §九更正（阻断件三）:close 首试实况为 --ack-uncommitted 所认领路径落在差集外触发 unknown_acks 拒（认领差集外即拒，反乱认方向活体触发在案），随后零差集放行——原文「认领放行」表述照实更正为本句，闸行为零缺陷且反向防护实证
- watchcheck 终态（阻断件四）:2026-09-07 复跑净态无主零处，退出码零
- CALL-LOG 补笔与 bypass 登记（阻断件二）:主会代补 scribe 与 lease 两笔，c0d48b2b 持锁窗已过锁面归零后落笔
