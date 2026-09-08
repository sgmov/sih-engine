# 引擎路择契约（attractor route）

> 家位 sih-engine/src/attractor/route.rs，承 SPEC-015 谓词融回规格与 autoflow2-solo 批实装；本册随 parktune-solo 批（2026-09-08）新建，承载引擎侧路由谓词机对表基准与修订记档。围堰 sih-tools/selector/CONTRACT.md 是谓词语义的权威正典，本册只记引擎侧对表实态与两侧同步义务，语义冲突以围堰册为准。

## 对表基准 {#baseline}

- 围堰现行文即 sih-tools/selector/src/selector/{predicates,route,pack,cli}.py 四件源码与两包四件纯数据，十一 kinds 与 fail-closed 语义与 route_on_fail 缺省与批级告警逐条对表，零语义漂移。
- 报告头 tool.version 承围堰 __version__ 即 0.4.0，工件工具名字段是契约面标识非二进制身份。
- 错误信封承围堰 f-string 原文形；toml 与 json 解析失败的异常报文是运行时差异面，拒收退出码与信封形态对表，此为显式范畴排除（SPEC-015 原条款）。
- 包纯数据随迁 src/attractor/packs/{core,parking}/ 与围堰逐字节一致即同参形构成条件，T0 测试钉死。

## 有余告警豁免语义 {#surplus-exempt}

承围堰修订七（parktune-solo 批 2026-09-08）：[alarms] 节可选键 siding_surplus_exempt_kinds（缺省空即零豁免，值须为合法谓词 kind 字符串数组否则拒包 fail-closed），首败谓词 kind 属豁免清单的侧轨件不计入有余告警的积压计数，summary 的 siding 仍为全量侧轨数即豁免不隐藏停计事实。parking 谓词包 0.4.0 声明 gate_hold 豁免即 gate 远景件入侧轨是设计行为非积压，点火后 gate 因 time_deadline 首败落侧轨不豁免即真事件计入。误报灭真报留承工程基线三注意力只投异常。

## 修订记录 {#revisions}

2026-09-08 修订一：本册新建随 parktune-solo 批。三件同步落地：其一有余告警豁免语义与围堰修订七逐条对表（parse_exempt_kinds 与 route_batch 积压计数排除段）；其二追平 parkgate-solo 批遗留债即 gate_hold 谓词（ALL_KINDS 十一件、check_gate_hold 结构判定、evaluate 分发、无参白名单）与 parking_aging 对未点火 gate 停计（围堰修订六语义引擎侧补齐）；其三金向量按 SPEC-021 T9 基线种随冻重录即期望由围堰 Python 原件跑出重冻、消费逻辑零改，重冻依据即 parking 包 0.3.0 升 0.4.0 与 ALL_KINDS 报文扩 gate_hold 与告警 count 语义三面合法变更的纯期望过期。t6 装配位谓词数断言随包四谓词适配（3 至 4，声明变更非违规）。
