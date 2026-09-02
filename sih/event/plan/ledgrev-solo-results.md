# ledgrev-solo 结果档

> 批：ledgrev-solo 覆盖账本修订小批（mathpipe 审阅承接件）
> 会话：c8744a784725b2b1
> 日期：2026-09-03
> 队形：单线形 solo，零子代理（确定性脚本 + 主线亲写）

## F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 修订可复算 | rev1 规则全脚本化，双跑 cmp 逐字节一致 | 过 | rev1_script.py 双跑输出三件 DEFINITIVE_IDENTICAL，输出与工地落档 MATCH（summary-rev1 仅差命题层补段段） |
| F-2 归类零静默 | 判定性与资源性与三态每一变动在申报表逐件附依据 | 过 | 归类变动申报 12 件逐件附 file/line/value/old_class/new_class/basis/context，零静默改判，见归类变动申报表 |
| F-3 对挂锚点实存 | 每对概念 ID 实存于 INDEX 且条目磁盘在，零命中如实记 | 过 | 对挂七条（六对一次取四）：lease/ORD-020、cascade/ORD-016、tally/ORD-006、scribe/ORD-019 实存，identity/ALG-002 与 identity/PROB-013 实存，selector 零命中如实记不代裁 |
| F-4 补段过检 | 四件补段后 check_three_proposition_audit all_covered true | 过 | 四件 all_covered 均 true，见三层检查读数 |
| F-5 源码零改动 | 三仓源码 diff 为零，写入仅请求写入节 | 过 | 三仓源码零触碰；engine 与 math 归并提交为批件（补段/rev1/materials），tool 源码零改动 |

## 载体三态（修订前后对表）

原账本：已实例化 11、可指认未实例化 0、无可指认载体 12。
rev1 修订：已实例化 1、可指认未实例化 12、无可指认载体 10。

误差说明：原账本"已实例化 11"为伪读数——引擎六件同构 ID 清单来自规格引用与金向量夹具而非数学载体；rev1 收紧为源码推导面引用白名单 ID，伪读数 10 件分出，3 件可指认未实例化浮出与 12 件可指认未实例化（重估），2 件无可指认载体合并至对应表。合计三态 1+12+10=23 机制与前一致。

## 分类分布（修订前后对表）

原账本：命名常数 判定性 39、资源性 4、待确认 66、合计 109；比较位字面量 判定性 76、待确认 477、合计 553。
rev1 修订：命名常数 判定性 27、资源性 16、待确认 66、合计 109；比较位字面量 判定性 76、待确认 477、合计 553。

判定性 39→27 即资源性 4→16 的改判净 12 件，见归类变动申报表。

## 归类变动申报表（判定性 → 资源性）

共 12 件，零静默改判，每件附依据。依据均为"改变该数只改资源并发度/额度/重试次数而不改治理判定"。

| # | 名 | new_class | 件数 |
|---|---|---|---|
| 1 | GLOBAL_MAX_CONCURRENT | 资源性 | 8 |
| 2 | DEFAULT_MAX_CONCURRENT | 资源性 | 1 |
| 3 | PROC_CAP | 资源性 | 1 |
| 4 | MAX_TOKENS | 资源性 | 1 |
| 5 | MAX_RETRIES | 资源性 | 1 |

归类变动申报逐件细目（file/line/basis）落 ledger-rev1.json `reclassification_declared` 数组，不于本档冗余复述。

## 对挂核验表（六对一次取四）

| 机制 | 概念 ID | 概念名 | 子仓 | in_index | on_disk | 结果 |
|---|---|---|---|---|---|---|
| lease | ORD-020 | 全序资源分配与死锁自由 | order | ✓ | ✓ | 实存 |
| cascade | ORD-016 | 良基关系与倒推终止 | order | ✓ | ✓ | 实存 |
| tally | ORD-006 | 闭包算子与后果算子 | order | ✓ | ✓ | 实存 |
| selector | （零命中） | 谓词划分 | — | — | — | 零命中如实记 |
| scribe | ORD-019 | 版本偏序与外化状态存储 | order | ✓ | ✓ | 实存 |
| identity | ALG-002 | 等价关系与商集隔离 | algebra | ✓ | ✓ | 实存 |
| identity | PROB-013 | 变点检测 | probability | ✓ | ✓ | 实存 |

selector 谓词划分无直接数学概念映射，零命中如实记，待确认清单呈报不代裁。

## spec 消歧

- 白名单概念 ID 全集 167（calculus 114 + order 19 + probability 15 + topology 8 + algebra 11）
- SPEC 双义交集（数学仓 ∩ 引擎规格档）：SPEC-001 至 SPEC-007 共 7
- 消歧规则：引擎 src 引用位默认引擎侧，除非白名单命中且引用上下文明示数学条目

## 三层检查读数（四件 all_covered）

| 目标 | 立题 | 应用 | 贡献 | all_covered |
|---|---|---|---|---|
| mathpipe-a1-solo-results.md | ✓ | ✓ | ✓ | true |
| parkreplay-solo-results.md | ✓ | ✓ | ✓ | true |
| mathpipe-full-program-v1.md | ✓ | ✓ | ✓ | true |
| summary-rev1.md | ✓ | 0.833 | ✓ | true |

四件补段均 all_covered true（F-4），各补一节含立题立场一句 + PRO-07 应用映射（A-A3.1 破自证循环 / A-A4.1 候选建议生成器 / A-A4.2 裁决权归确定性引擎）+ 治理贡献自述（信息洪流降维与权责归一）。

## 管线读数

化格四目标 exit 0 无需改；核阅四目标经 des-001（event/plan 与 math docs 域外 exit 2 如实记，零 findings）；检词四目标 core exit 0 零违例零新词登记。

## 认证读数

| 件 | event_id 前八 | event_hash 前八 |
|---|---|---|
| ledgrev-solo-pipeline.json | a0d6fc0e | 4d857be4 |
| ledger-rev1.json | 186cb8bb | 32db7e01 |
| env-params-rev1.json | 421ebbdd | af04bff4 |
| ledgrev-solo-script.json（链末） | 7688692b | 46600c10 |

链 verify valid，56 事件，末哈希 46600c10（前 46600c10）。

## 三仓 settle 与 close

- engine：commit 80e26748（归并主树，含补段三件 + materials）
- math：commit 3b494090（归并主树，含 rev1 三件）
- tools：无批件 settle（本批 tool 源码零改动、零新词、留痕走主树 CALL-LOG）——见误差申报
- close：三工地移除，session revoked；主树同名未跟踪件按备份让位归并对表法处理，归并 diff 中 materials 三件 identical、补段件差异即本批补段内容

## 切面检索

recall-certification（2026-09-03/2026-09-03）落 sih/event/plan/ledgrev-solo-materials/recall-results.json 作为链上事实底稿。

## 误差与越线申报

1. **tools 仓 settle 误判暂无批件**：本批工具侧源码零改动、零新词，化格/核阅/检词零发现，故 lease commit 报 nothing_staged。但请求写入节含 sih-tools/scribe/reports/ 六件（ask3 record/validation、elicit-signals、pipeline、script）与六件 CALL-LOG 各 +1——mathpipe-a2 先例将批报告与调用册随批入版控，本批因 close 先行 revoked 无法补 settle，未做主树 plain commit（守卫在位禁提）。此批件收编缺口待后续批或主会话处置。identity/reports 按红线零收编。
2. **recall 输出目录重建**：sih-engine/materials 目录不存在，切面检索 out 转落 sih/event/plan/ledgrev-solo-materials/，该文件随归并在主树（标记：recall 底稿系链上引文，未纳入认证）。
3. 其余红线全部遵守：原账本三件字节不动（时间戳 04:11 未变）、待确认清单（66 命名 + 477 字面量）只呈报不代裁、守卫在位全链 settle 走 lease 无 plain git commit、撞锁即处理。

## 承接

rev1 修订把"已实例化 11"伪读数修正为 1 实实例化 + 12 可指认未实例化 + 10 无可指认，判定性 39→27 净 -12 归资源性，可指认未实例化 12 件与无可指认 10 件为 M-1/M-3 后续清账工作量的真实信号，批序待主会话按账本定。工具侧 reports 收编缺口并入 mathpipe 后续批承接。

## 命题层

立题立场：本结果档的立题是"覆盖账本修订的审计合法性"——三态语义修订与判定性复核触及本体命题的载体归因准确性，不把脚本可复算本身当立题，而把可复算当作归因可信的承载。

应用命题映射：PRO-07 鉴要求检验由可重复程序承载，rev1_script.py 双跑逐字节一致即其应用。A-A3.1 鉴层破自证循环——修订以独立白名单与源码扫描证伪原账本伪读数，不以枚举结果左证机械核验；A-A4.1 候选建议生成器——对挂核验只产实存/缺失结果，不代立未映射概念；A-A4.2 裁决权归确定性引擎——归类改判由脚本按规则机械执行，裁决权不下放给人工主观判断。

治理贡献：本批把"已实例化 11"的伪读数修正为可核验的三态分布，是对上下文注意力的精准降维——人类只聚焦归类变动申报与零命中待确认清单，不复核全量枚举，治理贡献即信息洪流降维与权责归一（修订结果跟可复算脚本走，不跟人工记忆走）。
