# anchorcover-results：回锚链面覆盖越限告警批（debtclear-parallel 簇B）

> 批：anchorcover——attnanchor/anchor.py chain_line 扩域覆盖段，debtclear-parallel 清账并联批簇B
> 会话：0e2dce760f796692（sess-zcode-260911-clusterB-anchorcover）
> 日期：2026-09-11
> 队形：委外并联 parallel——debtclear-parallel 四簇之一，簇B 代理持全量上下文亲写零子代理，本工作区租约下并联形随批申报
> 链笔：intent_refined 67ac9c22；认证笔与 settle cert 见当日链与回执
> 温故：materials/recall-alarm.json（先例 debtclear-parallel-materials/recall-alarm.json 同族，topic 告警 word 越限）

## 改动清单 {#changes}

- sih-tools/attnanchor/anchor.py：chain_line 扩域覆盖段——新增 COVERAGE_WINDOW_DAYS=7 确定性常数、_registry_domains（中央登记册 active 域根去重）、_commit_days（git log --pretty=%cs 逐日笔数）、_covered（域链当日 trail 在场且有实文）、coverage_alarms（洞出告警段）；chain_line 增 today 可缺省参（缺省 date.today() 行为零变）并在告警非空时以分隔符追加；模块 docstring 补覆盖段纪律行
- sih-tools/attnanchor/tests/test_anchor.py：新增八测（洞告警、纯读数零变、册缺零段零崩、零在役零段、stopped 忽略与域去重、窗口边界、降级链仍追加、多域册序日升序）加只读静态扫一测（subprocess 首词白名单 git 与既有 uv）
- sih-tools/attnanchor/CONTRACT.md：修订三（覆盖语义、告警段形、降级与只读三件）
- sih-engine/sih/state/plan/anchorcover.md：开约包件（主树面申报写入，lease resolve_package 机械要件，正典任务文本锚接 debtclear-parallel.md 簇B 节）
- sih-engine/sih/event/plan/anchorcover-materials/：tdd-red-green.log、recall-alarm.json、live-worktree-run.json、pipeline 读数、scrutinator rc2 域外报文两件
- 本结果档

## F 表 {#f-table}

| F | 判据 | 实态 |
|---|---|---|
| F1 | fixture 域有提交无 trail 告警段出 | 通过——test_coverage_hole_alarms：洞日出「越限告警：sim-aesthetic-workbench <洞日> 5笔提交查无此人」，覆盖日零告警 |
| F2 | 域链齐纯读数形零变 | 通过——test_coverage_full_keeps_pure_form_byte_identical 逐字节断言等值旧形「[链] 今日 1 笔 末笔 abcdef12 certification_completed」 |
| F3 | 册缺零段零崩 | 通过——test_registry_missing_no_section_no_crash 与 test_registry_zero_active_no_section 两测，无越限告警段且行等旧形 |
| F4 | attnanchor 全套绿 | 通过——工地 17 passed（9 旧 + 8 新 + 只读静态扫在 8 新内计），先红 8 failed 9 passed 留档 tdd-red-green.log 禁清洗 |
| F5 | 只读约束 | 通过——coverage 段实现仅 git log 子进程与文件读，零网络零 LLM；test_coverage_readonly_static_scan 钉 subprocess 首词白名单（git 与既有泊路由 uv） |

## 机器链实录 {#chain-log}

叩问 elicit check 四词全轻 rc1，digest passed covered 4；ask3 双门 scrutinator rc0 零发现加 ask3repeater ok anchor_count 3；identity attest 零异常；lease open --new-stem rc0 会话 0e2dce760f796692，stem_check new_coinage_acknowledged，双工地 branch msh/anchorcover；scribe intent rc0 事件 67ac9c22；TDD 红 8 failed 9 passed 后绿 17 passed；CONTRACT 管线 formatter rc0 零改、scrutinator rc2 域外如实记、nomenclator rc0 零违例；settle 双仓与链 verify 与 reconcile 见当日链。

## 实弹读数 {#live}

- 工地实弹（materials/live-worktree-run.json）：链面行出「[链] 今日 N 笔 … · 越限告警：InferServer 2026-09-05 1笔提交查无此人 · 越限告警：InferServer 2026-09-07 2笔提交查无此人 · 越限告警：InferServer 2026-09-08 14笔提交查无此人 · 越限告警：InferServer 2026-09-11 1笔提交查无此人」；sim-aesthetic-workbench 近 7 日链面全覆盖零告警（任务书示例形中该域名日笔数为形例非实判）；SiInfer 非 git 仓零数据零虚构零告警
- 主树重跑读数见收约后段（收约后补录）

## 语义裁定申报 {#declarations}

- 告警段笔数 N 取十进制（任务书「N 笔」正典形）；令文示例「七笔」读作形例不读作汉字数词强制
- 域名取域根 basename；域非 git 仓或 git 不可读即空表不虚构不告警（降级可见以零虚构优先，册内坏域呈零数据事实）
- 当日引擎链不可读（降级）时覆盖段仍独立回算可追加，覆盖核对不依赖当日链
- 开约包件 sih-engine/sih/state/plan/anchorcover.md 为簇B 请求写入清单外唯一主树面写入：lease open 对裸 stem 无包件机械必拒（resolve_package must_exist），包件为开约必要件，内容纯锚接 debtclear-parallel.md 簇B 节零新裁，随批显式申报
- 意图落链在 lease open 之后（任务书序文 record_intent 先于 open 与之不符）：scribe 闸三要求会话在册活跃，BATCH-FACE 全序即 open 在 intent 前，按机械可行性承正典序执行

## 未决项 {#pending}

- anchorcover 与回锚与查无此人查册 unknown：术语登记候立名供给批（pk-090 面）消解，本批零写 packs/core
- InferServer 域四洞（09-05/07/08/11）与在役域补链与否归该域治理与人节点，本批只报不裁
- CALL-LOG 追加走 lease call-log append 于收约后批尾执行（calllog 纪律先例：留痕归收约后批尾主树补）
