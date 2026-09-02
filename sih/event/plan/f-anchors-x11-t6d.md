# f-anchors-x11-t6d：5 F 锚定当前状态（X11 子代理）

> 范式：T6-D 范式（X11 子代理，单子任务，A 立即可做 5）
> 主题：5 F 锚定（哲学桥接 / 集成学习 / 阈值参数 / 多视角独立性 / 真收敛区）当前状态记录
> 日期：2026-08-25
> 触发：用户 2026-08-25 决策「之前 LLM 口算 F 锚定 = 违反工程基线第 4 条可验证性」
> 承接：sih-tools/facet/task-packages/kimi-k2-thinking-probe-1000-t6d.md 阶段 5 形式化方向修正表

## 一、问题陈述 {#problem}

- 之前 F 锚定状态由 LLM 推理口算（kimi-k2-thinking-probe-1000-t6d-results.md §四 F 锚定评估），违反工程基线第 4 条「可验证性」（可机械校验）。
- F 锚定判定须以证据文件 + 机械检查为输入，由路择（selector）空腹谓词引擎逐件机械判定，判在别处。
- 本任务包为 X11 子代理：把 5 F 锚定当前状态以记录形态入档，提供给路择作为可机械路由的输入材料。

## 二、5 F 锚定当前状态（立文） {#state}

锚定 1：F-M1 哲学桥接（PRO-07 形式化）

- 判据：PRO-07 哲学命题已形式化为数学命题（voter 独立性 + p>1/2 + 多数决）
- 当前阶段：阶段 5 形式化方向确定，未证
- 证据：sih-philosophy/llm-friendly-build/entries/PRO-07.md + sih-tools/facet/task-packages/kimi-k2-thinking-probe-1000-t6d.md 86-100 行 F-M1/M3/M4 修正表
- 状态：draft（方向已定，公理验证未跑）

锚定 2：F-M2 集成学习（ensemble vs baseline）

- 判据：5 厂数据 ensemble vs baseline 对比，跨厂决策一致性提升
- 当前阶段：5 厂数据未采，仅 Kimi K2-Thinking 21 shots（k2t-c1-01）
- 证据：sih-tools/proposition/DES/k2t-c1-01/flywheel-trail.jsonl 21 行
- 状态：draft（5 厂未启动）

锚定 3：F-M3 阈值参数（facet 闸阈值）

- 判据：facet 闸阈值 + 参数选择证据（v3 冻结表 + 动态基线）
- 当前阶段：阈值在 sih-tools/facet/probes/maturation_gate.py 实现，未与 PRO-07 公理对齐
- 证据：sih-tools/facet/probes/maturation_gate.py 体温表 + baseline flip rate 协议
- 状态：draft（v3 冻结，PRO-07 公理验证未跑）

锚定 4：F-M4 多视角独立性（voter 距离）

- 判据：voter 距离 + 独立性证据（5 厂视角独立 + Banach 收敛）
- 当前阶段：voter 距离未测，单厂 21 shot 不足以验跨厂独立性
- 证据：sih-tools/facet/probes/multiview 探针历史 + PRO-07 多 voter 理论
- 状态：draft（5 厂未跑，距离未测）

锚定 5：F-M5 真收敛区（单 + 跨命题）

- 判据：单命题 18/20 shot 决策一致 = 90%（已有）+ 跨命题 50 命题 × 60 shot 真收敛
- 当前阶段：单命题真收敛 18/20 = 90% 一致（k2t-c1-01 实测），跨命题 50 命题 × 60 shot 未跑
- 证据：sih-tools/proposition/DES/k2t-c1-01/flywheel-trail.jsonl 21 shot + sih-tools/facet/task-packages/kimi-k2-thinking-probe-1000-t6d-results.md §三单命题真收敛区
- 状态：process（单命题 PASS，跨命题 PENDING）

## 三、可证伪条件（立文） {#falsifiable}

| F 锚定 | 判据 | 实际状态 | 可证伪条件 |
|---|---|---|---|
| F-M1 | PRO-07 形式化为数学命题 | draft | 若 Knaster-Tarski 公理验证 fail，F-M1 退化为启发式工具 |
| F-M2 | 5 厂 ensemble vs baseline | draft | 若 5 厂跨厂一致率 < 50%，F-M2 退化为单厂 |
| F-M3 | 闸阈值公理化 | draft | 若 v3 冻结表与不动点公理不一致，F-M3 退化为经验阈值 |
| F-M4 | voter 距离有界 | draft | 若 5 厂 voter 距离无界，F-M4 退化为单视角 |
| F-M5 | 单 + 跨命题真收敛 | process | 若跨命题一致率 < 50%，F-M5 仅单命题成立 |

## 四、约束 {#constraints}

1. 0 LLM 调用（路择空腹零 LLM 谓词引擎）
2. 不写「建议」：按 5 F 锚定机械判定
3. verdict 三选 1：PASS / FAIL / PENDING
4. 每 F 锚定必有 falsifiable_condition

## 五、验收 {#acceptance}

- 5 F 锚定 selector 判定完成
- selector 报告路径入档
- 报告列 5 F 锚定 PASS/FAIL/PENDING + 理由
