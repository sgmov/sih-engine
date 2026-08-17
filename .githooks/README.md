# .githooks — 治理工程 git hooks

承接任务包 T6D-01 子任务 B1：commit 前跑治理工程审阅流水线（零 LLM 调用）。

## 安装

```bash
# 在仓根目录
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit-audit
```

设置后，**每个 commit** 会自动跑 `audit_pipeline.py`（5 个机械脚本串行），输出审计状态。

## 模式

| 模式 | 行为 | 设置方法 |
|---|---|---|
| `warn`（默认） | 机械层失败 → 警告，**不阻断 commit**，状态写入 git notes | `AUDIT_HOOK_MODE=warn` |
| `strict` | 机械层失败 → **阻断 commit**，必须 fix 或 `--no-verify` | `AUDIT_HOOK_MODE=strict` |

默认 warn 是因为 baseline 已知多脚本 FAIL（A1 flywheel_run 73.17% > 60% / A2 yaml L62 错位 / A3 test_f34 名字 vs 实际 / A4 v1/v3 矛盾 / A5 cross-link 缺失）—— 全部 FAIL 是 task package 已立的"已知 baseline"。

未来当 FAIL 项 fix 后，**可切到 strict 模式**：B2 跑全过才放行。

## trail root 覆盖

```bash
# 自定义 trail root
AUDIT_TRAIL_ROOT=/path/to/your/trail git commit
```

默认 `sih-tools/proposition/DES`。

## 跳过 audit

```bash
# 单次跳过
git commit --no-verify
```

## 当前已知 baseline（2026-08-17）

5 脚本全 FAIL，符合 task package F1.x-F5.x 锚定：

| 脚本 | 状态 | 根因 | task package 处置 |
|---|---|---|---|
| A1 check_layer_proportion | FAIL | flywheel_run 占比 73.17% > 60% | F1.1 锚定通过，task B 路径修 |
| A2 check_yaml_factual | FAIL | yaml L62 rationale LLM 注释与代码事实相反 | F2.1 锚定通过，方法学 v1.1 修订 |
| A3 check_test_intent | FAIL | test_f34_reproducibility 名字 vs 实际（3 mismatch） | F3.1 锚定通过，task B 路径修 |
| A4 check_verdict_consistency | FAIL | layer2_signoff v1 vs program_signoff v3 矛盾 | F4.1 锚定通过，task B 路径修 |
| A5 check_decision_authority | FAIL | UNIQUE_BUT_BLIND + 3 cross-link 缺失 | F5.1 锚定通过，待 DEC |

## 不做的

- 不调 LLM（0 LLM 调用）
- 不改任何已知缺陷（脚本只检查 + 报告）
- 不在 commit message 写 audit 状态（用 git notes 不污染主 commit 历史）
