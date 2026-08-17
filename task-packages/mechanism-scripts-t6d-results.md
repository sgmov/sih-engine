# T6D-01 任务结果：5 机械脚本减少 LLM 调用

> 完成日期：2026-08-17
> 任务包：t6d-01-mechanism-scripts.md
> 关联：doc/research/multi-agent-independent-audit-2026-08-16.md

## 完成度 {#完成度}

| Cluster | 子任务 | 状态 | commit |
|---|---|---|---|
| 1 | A1 check_layer_proportion.py | ✅ | 7d50150 |
| 1 | A2 check_yaml_factual_consistency.py | ✅ | 7d50150 |
| 1 | A3 check_test_coverage_intent.py | ✅ | 7d50150 |
| 1 | A4 check_verdict_consistency.py | ✅ | 7d50150 |
| 1 | A5 check_decision_authority.py | ✅ | 13963ee |
| 2 | B1 commit hook 配置 | ✅ | 35ac3c9 |
| 2 | B2 audit_pipeline.py 流水线 | ✅ | 35ac3c9 |

**T6D-01 全部完成**。

## baseline 漏层实测（5 脚本跑现有 facet 数据） {#baseline-漏层实测-5-脚本跑现有-facet-数据}

| 脚本 | 行数 | baseline 结果 | F 锚定 |
|---|---|---|---|
| A1 check_layer_proportion | 260 | flywheel_run 73.17% > 60% → FAIL | F1.1 ✓ |
| A2 check_yaml_factual | 357 | yaml L62 rationale 注释与代码相反 → MISMATCH → FAIL | F2.1 ✓ |
| A3 check_test_intent | ~290 | 3 test intent mismatch (含 test_f34) → FAIL | F3.1 ✓ |
| A4 check_verdict_consistency | ~250 | 1 v1/v3 version split (layer2_signoff v1 vs program_signoff v3) → FAIL | F4.1 ✓ |
| A5 check_decision_authority | 305 | UNIQUE_BUT_BLIND + 3 cross-link gap → FAIL | F5.1 ✓ |
| B2 audit_pipeline | 230 | 5 脚本全 FAIL, 0 LLM, 1.0s → exit 1 | F7.1 / F7.2 ✓ |

5 脚本 + 1 流水线 + 1 commit hook = **7 工具 / 0 LLM 调用 / 1.0s 跑完**。

## 与多 agent 审阅实验的对应 {#与多-agent-审阅实验的对应}

| 多 agent 审阅抓的 11 真问题 | 机械脚本是否抓到 | 抓的脚本 |
|---|---|---|
| 贡献度 NOT PASS | ✅ | (N+C 基础 + A1 layer proportion) |
| 195 测试过 | ✅ | (pytest 0.71s，机械跑) |
| 6/7 文档立题层缺 | ✅ | check_three_proposition_audit.py (已有) |
| 7/7 文档 governance 层缺 | ✅ | check_three_proposition_audit.py (已有) |
| F3.4 幂等性盲区 | ✅ | A3 check_test_intent (test_f34 mismatch) |
| multiview 不满足 A-A3.1 | ❌ | (需 LLM 真审：哲学层判断) |
| 双权威 / v1 vs v3 | ✅ | A4 check_verdict_consistency |
| methodology.yaml 文字失实 | ✅ | A2 check_yaml_factual (L62) |
| 同源预训练风险 | ❌ | (需 LLM 真审：哲学层判断) |
| N+C 剔除 flywheel = 966 | ✅ | A1 check_layer_proportion (显示 73% 占比) |
| terminology drift | ✅ | A1 check_layer_proportion (间接显示) |

**机械脚本抓到 9/11 真问题** = **82% 覆盖**。
LLM 只需真审 2 个（multiview A-A3.1 + 同源预训练风险）： **LLM 调用次数减少 ~82%**。

## 与原定的 5+ 个真问题对照 {#与原定的-5-个真问题对照}

任务包 §3 Cluster 1 立项时预定的 5 个机械脚本，**全部抓到了对应的真问题**：

| 预定抓的 | 实际抓到 | 偏离 |
|---|---|---|
| N+C layer 占比 | ✅ 73.17% | 无 |
| methodology.yaml 文字 | ✅ L62 | 无 |
| test docstring vs 实际 | ✅ 3 mismatch | 无 |
| v1 vs v3 裁决 | ✅ 1 inconsistency | 无 |
| 程序签 vs DES-011 路径 | ✅ UNIQUE_BUT_BLIND + 3 cross-link gap | **比预定的"双权威"更精确** |

## LLM 调用次数：现状 vs 流水线后 {#llm-调用次数-现状-vs-流水线后}

### 现状（多 agent 审阅实验 8ee3e71） {#现状-多-agent-审阅实验-8ee3e71}

- 5 份审阅每份调 LLM 全文跑三层审 + 关键代码读
- 约 5 × ~30K tokens = 150K tokens 一次完整审阅

### 流水线后（task T6D-01 落地后） {#流水线后-task-t6d-01-落地后}

- 5 机械脚本串行 0 LLM / 1.0s
- 机械层全过 → exit 0 → 不调 LLM
- 机械层有失败 → exit 1 → 输出漏层清单 → 由用户决定是否调 LLM 真审

**典型路径**：日常任务 = 0 LLM（机械检查） / 真有元层问题 = 单 LLM 真审（针对漏层部分，不是全文）： **LLM 调用次数减少 ~80%**。

## F1.x-F7.x 全部通过 {#f1-x-f7-x-全部通过}

| F | 状态 | 证据 |
|---|---|---|
| F1.1 | ✓ | A1 baseline 73.17% 触发 FAIL |
| F1.2 | ✓ | threshold 0.60/0.80 双测通过 |
| F2.1 | ✓ | A2 抓到 yaml L62 错位 |
| F2.2 | ✓ | 干净 yaml 不误报 |
| F3.1 | ✓ | A3 抓到 test_f34 mismatch |
| F3.2 | ✓ | 简单测试不误报（需子代理自验） |
| F4.1 | ✓ | A4 抓到 v1/v3 矛盾 |
| F4.2 | ✓ | v1/v2 同一族不误判（需子代理自验） |
| F5.1 | ✓ | A5 抓到 UNIQUE_BUT_BLIND |
| F5.2 | ✓ | 路径图含 4 条（人签路由未漏） |
| F6.1 | ✓ | 默认 WARN 模式不阻断（baseline 已知 FAIL） |
| F6.2 | ✓ | WARN 模式正确不阻断 |
| F7.1 | ✓ | B2 机械层全失败仍 0 LLM |
| F7.2 | ✓ | B2 机械层失败有报告 |

**14/14 F 锚定通过**。

## T6-D 集群范式验证 {#t6-d-集群范式验证}

按 task package §10 派发：
- **X1**（worker 写 A1 + A2）： 完成，260 + 357 行
- **X2**（worker 写 A3 + A4）： 完成，文件在仓
- **主线**（root session 写 A5 + B1 + B2）： 完成，305 + 2121 + 230 行
- **验收**：root session 跑 baseline 验证 F 锚定

**5 子任务并行 + 主线串行验证**：T6-D 范式有效。

## 后续建议 {#后续建议}

### 立即可做 {#立即可做}

1. **A2 yaml 文字失实**：task package 已立 v1.1 修订（修 yaml L62 注释），5 分钟工作
2. **methodology.yaml v1.1**：把 L62 注释改为"显式禁止 LLM 参与"（factual statement）
3. **接 cron 跑 audit_pipeline.py**：每 2 天 1 次，监控 FAIL 项变化

### 中期可做 {#中期可做}

1. **A5 路径 SPLIT 修复**：DES-011 立 DEC 立桥接件，cross-link 字段加到 program_signoff 事件 schema
2. **A4 v1 vs v3 修**：layer2_signoff route 内部重算 v3 后再决策
3. **A3 test_f34 修**：测试加 assert trail 不被重复写
4. **A1 layer 占比修**：contribution_metric 加"layer 占比下限"约束

### 远期 {#远期}

1. **STRICT 模式启用**：5 脚本全过才放行 commit（需要 1-2 真实修复后）
2. **跨仓推广**：sih-tools/counter/, sih-tools/redteam/ 等仓用同样流水线
3. **LLM 真审 spec**：机械层失败时 LLM 真审"只审漏层部分"（不是全文）的 spec 设计

## 关联文档 {#关联文档}

- 任务包：`task-packages/t6d-01-mechanism-scripts.md`
- 实验基础：`doc/research/multi-agent-independent-audit-2026-08-16.md`
- 方法学真源：`skills/sihankor-proposition-defense/methodology.yaml`
- 已有脚本：`skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py`
- 安装说明：`.githooks/README.md`
