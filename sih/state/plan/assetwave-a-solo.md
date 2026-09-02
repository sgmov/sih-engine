# assetwave-a-solo 任务包

- 队形：单线（主线直写）
- 日期：2026-09-02
- 触发：用户令「得一裁，裁一过一执行一，未过入泊界」，裁A m-assetanchor-rev stable_clear 终签 8f4065bb verify identical，执行 A
- 温故检索：回锚、登记面两题零命中，如实记（recall-01、recall-02 随批材料）

## 目标

盘点 SETSP 两册、旧仓、ai-ex 三源，逐机制回锚哲学仓现行原文，产出命题×机制×资产出处登记面，落 sih-math/docs/asset-anchor-registry-2026-09-02.md。

## 排除

- SETSP 与旧仓只作盘点源，登记出处不转录降格内容
- 外仓只登记候选不盘点，出泊唯人
- 不动 entries、INDEX、mapping（B 批面）

## 必读文件清单

- sih-tools/proposition/topics/2026-09-02-assetanchor-rev.md（已裁命题）
- sih-tools/proposition/DES/m-assetanchor-rev/（飞轮与终签材料）
- .tmp/SihEngineeringTechnologySelectionPrecedent/engineering-bridge/（盘点源，只读）
- ai-ex/SETSP-SILENT-REFERENCE-BOUNDARY.md（边界，只读）

## F 锚定清单

| F | 验证 | 通过条件 |
| --- | --- | --- |
| F1 | 登记面落 sih-math/docs/ | 文件在 math 工地，含盘点源名录与登记表与载体指向 |
| F2 | 化格门 | formatter 退出码 0 或 1（已改） |
| F3 | 检词门 | nomenclator 零违例退出码 0 |
| F4 | 核阅门 | des-001-mathe 域外如实记（docs/ 不在域内，exit 2 记录不属违规） |
| F5 | 认证上链 | scribe append 认证事件入当日链，doc_id 可查 |
| F6 | 双仓收约 | math 与 engine commit settle --cert 全哈希，close PACKAGE IDENTICAL |
| F7 | reconcile | 双仓 unrouted 0，cert_missing 0 新增 |
| F8 | 链 verify | 当日链 valid |

## 队形验证

单线形：主线直写登记面与任务包，无子代理派单。
