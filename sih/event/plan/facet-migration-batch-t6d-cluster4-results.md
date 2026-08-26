# facet-migration-batch-t6d Cluster 4 results（T6D-13 baseline_checker 实现）

> Cluster 4 T6D-13 合并 results
> X1 commit `c03fbf3` + X2 commit `e444add`
> 日期：2026-08-18

## 摘要 {#summary}

T6D-13 实现 baseline_checker 治理层确定性程序（消费 facet 报告 + 4 守卫 + 写 ndjson + 回写 facet cross_link_verified）。

双子代理分工：X1（baseline_checker 消费 + 4 守卫）+ X2（crosslink_writer 写 ndjson + 回写 facet）。

**关键成果**：

- baseline_checker.py：163 行（4 守卫 + check_baseline 主入口 + CLI）
- crosslink_writer.py：195 行（ndjson 写 + facet 回写 + process_baseline 端到端入口）
- 70/70 pytest 全过（X1 44 + X2 26）
- cross-link 协议落地（单向引用 + 双向确认）
- 0 LLM 调用

## 关键设计决策 {#decisions}

### 决策 1：4 守卫边界明确 {#决策-1-4-守卫边界明确}

按 X1 报告：

- **PRO-07** (路由)：boundary + promote → 拒；boundary + hold/reject/refine → 过
- **PRO-10-c** (层)：promote + 核心公理 → 拒；hold/reject + 核心公理 → 过
- **不可逆** (rationale)：promote + rationale < 10 字 → 拒
- **PRO-08** (签核留痕)：无 signed_by → 拒

总体 verdict 规则：

- 0 失败 → PASS
- 任意失败含 "boundary" 关键词（PRO-07 触发）→ BOUNDARY
- 其他失败 → FAIL

### 决策 2：cross-link 协议（DES-011 §协议不变量） {#决策-2-cross-link-协议-des-011-协议不变量}

按 X2 报告：

- 单向引用：facet.sih_engine_event_id → sih-engine.event_id
- 双向确认：sih-engine 写完回写 facet cross_link_verified=True
- 事件含反向引用：facet_event_id
- 失败态保留：cross_link_verified=False 保持为"待核对"标记

### 决策 3：跨子代理解耦（X1 X2 协调） {#决策-3-跨子代理解耦-x1-x2-协调}

按 X2 报告：

- X2 通过 `from baseline_checker import check_baseline` 消费 X1 出口
- X1 完全不动（X2 commit 时 X1 终态是 c03fbf3）
- 模块顶部注入 `probes/` 自身到 sys.path，跨 workspace root / module 两种跑法都支持

### 决策 4：0 LLM 调用（强制约束） {#决策-4-0-llm-调用-强制约束}

按 X2 报告：

- 测试 `test_no_llm_invocation` 显式断言模块源码不含 `import openai/anthropic`
- 0 LLM 依赖，符合「T6D-13 实施 0 厂商」约束
- 验证流量（需 A-2 caller 集成）= 后续任务包，不在本范围

## 关键修改 {#changes}

| 文件 | 范围 | 改法 |
|---|---|---|
| `sih-engine/probes/baseline_checker.py` (新) | X1 | 4 守卫 + check_baseline 主入口 + CLI |
| `sih-engine/probes/tests/test_baseline_checker.py` (新) | X1 | 44 测试 (PRO-07: 6 / PRO-10-c: 5 / 不可逆: 8 / PRO-08: 4 + 集成) |
| `sih-engine/probes/crosslink_writer.py` (新) | X2 | ndjson 写 + facet 回写 + process_baseline 端到端入口 |
| `sih-engine/probes/tests/test_crosslink_writer.py` (新) | X2 | 26 测试 (5 类: compute_event_id / write_crosscheck_completed / write_back_cross_link_verified / process_baseline / 集成) |

总 4 文件 / 1334 行（含 70 测试）。

## F 锚定状态 {#falsifiable}

| F 锚定 | 类别 | 状态 | 事实 |
|---|---|---|---|
| F1 Cluster 1-4 F 锚定全部 NOT TRIGGERED | 实施 | NOT TRIGGERED ✓ | Cluster 1-4 全部完成 |
| F2 pytest 247+ 全过 | 实施 | NOT TRIGGERED ✓ | facet 247/247 + sih-engine 70/70 |
| F4 4 守卫单测全过 | 实施 | NOT TRIGGERED ✓ | PRO-07: 6 / PRO-10-c: 5 / 不可逆: 8 / PRO-08: 4 |
| F6 baseline_checker 函数实现 + 4 守卫验证 | 治理架构 | NOT TRIGGERED ✓ | 4 守卫 + check_baseline 完整 |
| F7 cross-link 协议落地 | 治理架构 | NOT TRIGGERED ✓ | 单向引用 + 双向确认 + 失败态保留 |
| F9 失败回滚机制不触发 | 范畴边界 | 不触发 ✓ | 全部 F 锚定通过 |
| F10 1 session 持续 active | 元层工具 | NOT TRIGGERED ✓ | 主线持续 active，4 Cluster 派发 + 验收无中断 |

## 关键边界声明 {#boundaries}

### 实施 vs 验证分离（关键！） {#实施-vs-验证分离-关键}

- **实施**（本总任务包 Cluster 4）：0 厂商，70/70 pytest 全过
- **验证**（A-2 caller 集成，后续任务包）：
  - proposition-defense skill 走 facet pipeline
  - redteam skill 走 facet pipeline
  - 真实流量触发 cross-link 事件
  - baseline_checker 消费事件
  - 验证 A5 decision_authority verdict 翻 UNIQUE

### audit_pipeline A4 仍 FAIL 真实根因 {#audit_pipeline-a4-仍-fail-真实根因}

- A4 (check_verdict_consistency) 看 reads 口径
- T6D-10 后 calls = 0（v1-family 严格清零）
- reads 口径仍 25（探针函数 reads v1 verdict 字段）
- **reads 口径下降 = cross-link 协议落地后真实流量触发** = T6D-13 验证流量 = A-2 caller 集成后

预期内，T6D-13 验证流量阶段（后续 A-2 caller 任务包）A4 会进一步改善。

## 后续路径 {#next}

### Cluster 5 总收尾 {#cluster-5-总收尾}

- 跨仓 commit（sih-engine + sih-tools/facet）
- 总 results 文档
- trail 记录 4 串行 + 1 session 长程任务

### T6D-14 候选（bing_xseat / yi_depth / xseat_fill v1→v3 迁移） {#t6d-14-候选-bing_xseat-yi_depth-xseat_fill-v1-v3-迁移}

- T6D-11 评估发现的 2 个决策依赖真待迁
- 复用 T6D-09/10 模式
- 工作量：1-2 天

### A-2 caller 实际集成（后续任务包） {#a-2-caller-实际集成-后续任务包}

- proposition-defense + redteam 2 skill 走 facet pipeline
- 真实流量触发 cross-link
- 验证 A5 verdict 翻 UNIQUE
- 工作量：1-2 天起步 + 5-12 月慢变量

### T6D-15 候选（v1-scan 优化） {#t6d-15-候选-v1-scan-优化}

- 加 decision_source 字段 + 完善 FP 排除规则
- 工作量：1 天

## 关联文件 {#related}

- 任务包：`sih-engine/task-packages/facet-migration-batch-t6d.md` §三 Cluster 4
- X1 实施：`sih-engine/probes/baseline_checker.py` (commit c03fbf3)
- X2 实施：`sih-engine/probes/crosslink_writer.py` (commit e444add)
- 治理 DEC：`sih-engine/doc/design/DES-011-baseline-checker-DEC.md`
- OQ-22：OPEN-QUESTIONS.md §裁决材料与基线核对二层结构
- 流水线状态：Cluster 1 ✓ / Cluster 2 ✓ / Cluster 3 ✓ / Cluster 4 ✓ / Cluster 5 待派
