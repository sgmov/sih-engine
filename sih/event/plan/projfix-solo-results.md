# projfix-solo 完工结果档

> 令源：用户 2026-09-07 令「开」projfix-solo 批
> 范式：T6 单线 solo，委外代理亲写零子代理
> 判定语义：m-calllog-proj-1 九发 stable_clear 机器终签 2564e16b 在链（markdown 表格原格式形认可，jsonl 行堆形不取）
> 验收背景：calllog-solo 六项不实或缺陷实录（CONTRACT 修订四十八追记）的第四件（03:02 dogfooding 跨根直写真根）与第五件（import 投影再生步把源册覆写为 jsonl 行堆）
> 批内：T-1 至 T-5 落地，F-1 至 F-7 验收门全过，双仓 settle + close + reconcile + verify 全绿

## 一、F 表读数 {#f-table}

| F 锚定 | 类别 | 判据 | 验收读数 | 结论 |
|---|---|---|---|---|
| **F-1** | 渲染忠实 | render(账本) 对 19 册输出与 git HEAD 逐字节一致（已知尾行差异类外零差） | 主树真跑（commit c11b2c5a）：19/19 册 PASS，权威腿 698 行（697 行 + 1 行 dogfooding 留痕印证 T-3 跨根卫必要性）；lease 册尾行差异 +3 行（含 calllog-solo 残账结构化回位 1 行 + 跨根卫未装前 dogfooding 复发 2 行） | **PASS** |
| **F-2** | 迁移零自毁 | import 全流程后册子与源逐字节一致（临时根夹具 + one-shot 守卫保持） | 1 测 test_run_import_projection_byte_identical_to_source 先红后绿，19 册 5 行原文 verbatim 重组 = 源 markdown 字节相等 | **PASS** |
| **F-3** | 跨根卫 | 伪工地形拒零写入、主树形过、显式根放行三夹具红转绿 | 9 测 test_dogfood_crossroot 先红后绿（_is_worktree_cwd + _resolve_dogfood_target + 工地 CWD 拒 + 显式根放行 + 缺省关 + 主树 CWD 走 CWD 推断根） | **PASS** |
| **F-4** | 追加渲染 | append 后投影含新表格行且历史行零变动 | 1 测 test_append_call_writes_verbatim_table_row 先红后绿，新行 6 列形态（7 个 \|）含 occasion 字段，verbatim 字段写表格渲染形 | **PASS** |
| **F-5** | 双跑一致 | 全子命令同参双跑逐字节一致 | 1 测 test_append_double_run_byte_identical（既有 8 测零回归），event_id/at 不同是设计内，结构与值一致 | **PASS** |
| **F-6** | 零 LLM | 命令族全程零模型调用零网络 | 1 测 test_no_llm_import（既有 8 测零回归），calllog 包导入链不引 LLM 库（openai/anthropic/google.generativeai/transformers/torch/llama_cpp） | **PASS** |
| **F-7** | 语义承证 | 按 m-calllog-proj-1 落地零偏离，遇边界停批呈报 | markdown 表格原格式形认可（verbatim 重组形落定），jsonl 行堆形不取（旧 _render_projection_for_tool 重写 + 旧 test_append_three_legs_atomic 与 test_projection_contains_tool_only 投影不再含 event_id 断言更新到 verbatim 重组形语义）；零边界停批 | **PASS** |

## 二、测试计数 {#tests}

| 包 | 测数 | 旧测 | 新增 | 零回归 |
|---|---|---|---|---|
| calllog 包 | **12** | 8 | 4（T-1 verbatim 重组 4 件） | 绿 |
| lease 包 | **248** | 235 | 13（T-2 import 对表 1 件 + T-3 跨根卫 9 件 + 既有 3 件更新） | 绿 |
| 合计 | **260** | 243 | 17 | 绿 |

新增测名：
- calllog: test_render_verbatim_reassembles_markdown, test_render_all_19_tools_verbatim_reassemble, test_append_call_writes_verbatim_table_row, test_no_jsonl_line_stack_in_projection
- lease: test_run_import_projection_byte_identical_to_source, test_is_worktree_cwd_true, test_is_worktree_cwd_false, test_resolve_target_worktree_no_explicit, test_resolve_target_explicit_overrides, test_resolve_target_main_cwd, test_dogfood_crossroot_guard_refuses_worktree, test_dogfood_crossroot_guard_allows_explicit_root, test_dogfood_off_by_default, test_dogfood_no_explicit_no_worktree_cwd

## 三、F-1 主树真跑逐册对表 {#f1-detail}

> 工地条件验证不算接线（第四案教训在册），F-1 必须在主树真跑。
> 脚本：sih-engine/sih/event/plan/projfix-solo-materials/f1_render_byte_compare.py
> 输出：sih-engine/sih/event/plan/projfix-solo-materials/f1-verify.stdout.txt

| 工具 | rows | 字节 | 状态 |
|---|---|---|---|
| cascade | 7 | 796 | PASS |
| elicit | 10 | 1587 | PASS |
| facet | 17 | 7775 | PASS |
| formatter | 111 | 19569 | PASS |
| gauge | 7 | 3357 | PASS |
| identity | 15 | 2191 | PASS |
| latex-helper | 44 | 2535 | PASS |
| lease | 139 | 62847 | PASS（尾行差异 +3 行：残账结构化回位 1 行 + 跨根卫未装前 dogfooding 复发 2 行） |
| locator | 5 | 947 | PASS |
| locks | 11 | 1968 | PASS |
| meter | 11 | 2501 | PASS |
| nomenclator | 125 | 27997 | PASS |
| parser | 6 | 1520 | PASS |
| scribe | 26 | 1461 | PASS |
| scrutinator | 116 | 25318 | PASS |
| selector | 17 | 3651 | PASS |
| tally | 15 | 6277 | PASS |
| watchcheck | 3 | 605 | PASS |
| wikirecall | 13 | 2284 | PASS |
| **合计** | **698** | — | **19/19 PASS** |

权威腿 ndjson 总行 698 行 = calllog-solo 重建 697 行 + 1 行 dogfooding 留痕（印证 T-3 跨根卫必要性，跨根卫未装前事故复发的实证）。

## 四、链笔哈希 {#chain}

| 事件 | event_id | event_hash |
|---|---|---|
| intent_refined | 5d9a4888-bc98-4aa9-8508-33f28dc005a9 | 86f578aba9da75dd61afd325f30653858c274395e4794b2136ae4833c526fe7c |

链位：sih-engine/sih/event/trail/2026-09-07.ndjson

## 五、CONTRACT 修订四十九 + BATCH-FACE 坑位勘误 2026-09-07 {#contract}

- CONTRACT 修订四十九：升版本位 1.34.0 零变更（缺陷修复非语义升级），三源对齐零动；增 calllog/core._render_projection_for_tool verbatim 重组形、lease/calllog_import.run_import 投影步去自毁、lease/cli._dogfood_calllog 跨根卫；行为变更先红后绿
- BATCH-FACE 坑位勘误 2026-09-07 节：投影腿 verbatim 重组（calllog-solo 第五件修复）、import 投影步去自毁（第五件修复）、跨根 dogfood 卫（第四件修复）、dogfooding 解冻申报、F-1 主树真跑硬性项

## 六、批内 lease 治理调用 dogfooding 留痕说明 {#dogfooding-statement}

> 任务包硬性项：工地期不开 SIHANKOR_CALLLOG_DOGFOOD（跨根卫是要装的东西不是要开的环境变量）
> 留痕归收约后主会补或批尾主树补，批内如实申报

**批内 dogfooding 留痕状态**：本批批内 `SIHANKOR_CALLLOG_DOGFOOD` 全程未开（跨根卫已装 + 任务包硬性项双约束）。批内 lease 治理命令（open / lock / commit / close）的留痕归收约后由主会在主树手动补（照 calllog-solo T-8 先例 + append 命令；非工地期无跨根风险，可安全走主树补笔）。

## 七、待办（批内可不做） {#todo}

- [ ] 书单对表（recall + checkcite）— 批内引用件无外部书单依赖，0 跳过
- [ ] 主树复跑判定包（merge 后）— 工地归并后由主会代收
- [ ] 解 dogfooding 留痕（callog-solo 后某批在工地开 dogfooding 跨根直写 1 行）— 跨根卫已装后应不再复发，但 1 行留痕需主会审裁

## 八、风险点与边界声明 {#risk}

- lease 册 schema 异构（两表头摞立：5 列 + 6 列）：verbatim 重组形天然规避，append 新行统一走 6 列「日期 | 事由 | 调用 | 退出码 | 会话号 | 备注」批内定形
- 跨根卫仅拦截 dogfooding 路径，其他留痕路径（如显式 SIHANKOR_CALLLOG_DOGFOOD_ROOT）仍可自担放行
- F-1 主树权威腿 698 行 = 697 + 1（dogfooding 留痕）— 跨根卫未装前事故复发实证，跨根卫本批装后应根治

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 F 锚定、得一裁红线、双仓同步；判定语义已前置一裁（m-calllog-proj-1）非本批内裁。
