# facet-migration-batch-t6d：facet 跨族迁移 4 串行总任务包

> 总任务包：T6D-10 → T6D-11 → T6D-12 → T6D-13 串行
> 范式：T6-D 嵌套（外层 4 串行 + 内层双子代理并行 + 主线串行验证）
> 模式选择：长程任务模拟（**用户显式指定** 1 session 串行 + 减少人类介入）
> 日期：2026-08-18
> 偏离 T6-PARADIGMS.md T6-D 集群 ≤ 3 文件边界（4 串行）： 偏离理由：长程任务模拟 + 减少介入

## 一、问题陈述 {#problem}

T6D-09 完成后 v1-family 严格 calls 12 → 4。剩 4 个候选都是「跨族治理 + 治理架构」类工程债：

| 候选 | 范围 | 厂商依赖 |
|---|---|---|
| **T6D-10** | 4 R3a/R3b/R5 函数 v1→v3 跨族迁移 | 无 |
| **T6D-11** | 24 reads-only 函数评估 | 无 |
| **T6D-12** | v2↔v3 循环重构（替代 `sys._getframe(1)` hack）| 无 |
| **T6D-13** | baseline_checker 实际代码实现 | 实施无 / 验证需 A-2 caller 集成 |

用户决定：**4 串行，1 主 session 调用，尽量减少人类介入**。

## 二、关键设计 {#design}

### 2.1 T6-D 嵌套结构 {#2-1-t6-d-嵌套结构}

```
facet-migration-batch-t6d（总任务包, 1 主 session）
├── Cluster 1: T6D-10
│   ├── X1（worker, run_in_background=true）改 R3a/R3b 2 函数
│   ├── X2（worker, run_in_background=true）改 R5 2 函数
│   └── 主线 Cluster 1 验收（pytest + v1-scan + commit + 跨仓同步）
├── Cluster 2: T6D-11
│   ├── X1 评估 reads-only 函数 12 个
│   ├── X2 评估 reads-only 函数 12 个
│   └── 主线 Cluster 2 验收
├── Cluster 3: T6D-12
│   ├── X1 抽 `assess_maturation_v3_base` 函数
│   ├── X2 重构 `assess_maturation_v2` 移除 `sys._getframe(1)`
│   └── 主线 Cluster 3 验收
├── Cluster 4: T6D-13
│   ├── X1 写 baseline_checker 消费 facet 报告
│   ├── X2 写 baseline_checker 写 crosscheck_completed
│   └── 主线 Cluster 4 验收（含厂商依赖标记）
└── Cluster 5: 总收尾
    ├── 跨仓 commit（sih-engine + sih-tools/facet）
    ├── 总 results 文档
    └── 写 trail（4 串行 + 1 session 长程任务）
```

### 2.2 主线持续 active 机制 {#2-2-主线持续-active-机制}

**主 agent session 持续 active**，机制：

1. 立总任务包 + 派 Cluster 1 双子代理（run_in_background=true）
2. 等 background 完成通知（owner conversation 自动恢复）
3. Cluster 1 主线验收：跑 pytest + 跑 v1-scan + 跑 audit_pipeline + 写 commit
4. 派 Cluster 2 双子代理
5. ... 重复到 Cluster 5
6. 任何 Cluster 失败 → 立即 trail + 跳过后续 Cluster + 等用户决定
7. Cluster 全部完成 → 跨仓 commit + 总 results + trail

### 2.3 失败处理（不重试原则） {#2-3-失败处理-不重试原则}

- **任何 1 个 Cluster F 锚定触发** = 立即回滚该 Cluster commit + 写 fail 文档 + trail
- **跳过后续 Cluster**（避免叠加风险）
- **等用户决定**（不擅自重试 / 不擅自继续）
- trail 行格式：`{T6D-XX} FAILED at {commit hash}，跳过 T6D-YY+，待用户决定`

### 2.4 厂商依赖标记 {#2-4-厂商依赖标记}

- T6D-10/11/12 = 0 厂商
- T6D-13 实施 = 0 厂商（确定性程序）
- T6D-13 验证流量 = 需 A-2 caller 实际集成（间接需厂商 GLM + MiniMax + DeepSeek）

**总任务包范围** = T6D-10/11/12/13 实施（含 T6D-13 实施 0 厂商部分）。
**T6D-13 验证流量** = 后续 A-2 caller 集成任务包（不属本总任务包范围）。

## 三、工作清单 {#work}

### Cluster 1：T6D-10 实施（4 R3a/R3b/R5 函数 v1→v3） {#cluster-1-t6d-10-实施-4-r3a-r3b-r5-函数-v1-v3}

按 inventory §3. 低优先级 4 函数：

1. `main@r3a_gate_v2.py:82-150`
2. `gate_row@r3b_new_props.py:98-109`
3. `_baseline_row@r5_emphasis_feedback.py:83-89`
4. `run_subject@r5_emphasis_feedback.py:92-121`

**双子代理分工（按文件独立）**：

- X1: r3a_gate_v2.py（1 函数）+ r3b_new_props.py（1 函数）= 2 commit
- X2: r5_emphasis_feedback.py（2 函数 = 1 文件）= 1 commit

**F 锚定**：

- F1 改完 4 函数
- F2 pytest 217+ 全过
- F3 v1-scan 严格 calls = 0（4 → 0）
- F4 历史实验探针测试（如果存在） + smoke test
- F5 CLI 不破
- F9 失败 F 锚定触发 = 回滚 + 跳过后续

### Cluster 2：T6D-11 实施（24 reads-only 函数评估） {#cluster-2-t6d-11-实施-24-reads-only-函数评估}

按 v1-scan 输出的 reads-only 函数清单（24 个）：

- 评估每个函数的「读取用途」（数据依赖 / 决策依赖）
- 数据依赖（仅用于日志/审计）= 不迁
- 决策依赖（用于决策）= 标"待 T6D-XX 迁移"

**双子代理分工（按文件分）**：

- X1: 12 个 reads-only 函数评估
- X2: 12 个 reads-only 函数评估

**F 锚定**：

- F1 评估完 24 函数
- F2 评估清单可读
- F3 数据依赖 vs 决策依赖分类
- F9 失败 = 回滚 + 跳过

### Cluster 3：T6D-12 实施（v2↔v3 循环重构） {#cluster-3-t6d-12-实施-v2-v3-循环重构}

按 T6D-08 X2 报告的 `sys._getframe(1)` hack 脆弱性：

- 抽 `assess_maturation_v3_base` 函数（v3 闸基础，不调 v2）
- 重构 `assess_maturation_v2` 调用 `assess_maturation_v3_base`（移除 `sys._getframe(1)`）
- 重构 `assess_maturation_v3` 调 `assess_maturation_v3_base` + v2 嵌套逻辑

**双子代理分工**：

- X1: 抽 `assess_maturation_v3_base` 函数
- X2: 重构 v2/v3 调用图（移除 `sys._getframe(1)`）

**F 锚定**：

- F1 `sys._getframe(1)` 0 命中
- F2 pytest 217+ 全过（v1+v2+v3 全覆盖）
- F3 v2 闸行为不变（向后兼容）
- F9 失败 = 回滚 + 跳过

### Cluster 4：T6D-13 实施（baseline_checker 实际代码实现） {#cluster-4-t6d-13-实施-baseline_checker-实际代码实现}

按 OQ-22 桥接件落位 + DES-011 DEC：

- 消费 facet 报告（read sih-tools/proposition/DES/<guidance_id>）
- 验 PRO-07 / PRO-10-c / 不可逆 / PRO-08 四守卫
- 写 `sih-engine/trail/YYYY-MM-DD.ndjson`（crosscheck_completed event）
- 回写 facet `cross_link_verified` 字段

**双子代理分工**：

- X1: 写 baseline_checker 消费 facet 报告
- X2: 写 baseline_checker 写 crosscheck_completed + 回写 cross_link_verified

**F 锚定**：

- F1 baseline_checker 函数实现
- F2 消费 facet 报告（机械读 ndjson / json）
- F3 4 守卫验证（PRO-07 / PRO-10-c / 不可逆 / PRO-08）
- F4 写 ndjson 事件（crosscheck_completed）
- F5 回写 facet cross_link_verified 字段
- F9 失败 = 回滚 + 跳过

**T6D-13 验证流量（需厂商）= 后续 A-2 caller 集成任务包**：

- 改 proposition-defense + redteam 2 skill 走 facet pipeline
- 真实流量触发 cross-link 事件
- baseline_checker 消费事件
- 验证 A5 decision_authority verdict 翻 UNIQUE

### Cluster 5：总收尾 {#cluster-5-总收尾}

- 4 任务全部 commit 进 facet 仓 + sih-engine 仓
- 总 results 文档：`task-packages/facet-migration-batch-t6d-results.md`
- 跨仓 ROADMAP §P5 同步
- 写 trail（4 串行 + 1 session 长程任务）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F1** Cluster 1-4 F 锚定全部 NOT TRIGGERED | 实施 | 每个 Cluster 的 F 锚定状态分项 |
| **F2** pytest 217+ 全过（每 Cluster 后）| 实施 | `pytest` 退出码 0 |
| **F3** v1-family 严格 calls 4 → 0 | 跨族治理 | T6D-10 后 v1-scan 严格 calls = 0 |
| **F4** v1-family reads 口径 = 0 | 跨族治理 | T6D-11 后 check_verdict_consistency v1-family 0 个 |
| **F5** v2↔v3 循环 hack 0 命中 | 元层工具 | T6D-12 后 grep `sys._getframe(1)` 0 命中 |
| **F6** baseline_checker 函数实现 + 4 守卫验证 | 治理架构 | T6D-13 后 baseline_checker.py 存在 + 4 守卫单测全过 |
| **F7** cross-link 协议落地 | 治理架构 | T6D-13 后 crosscheck_completed ndjson 事件可写 |
| **F8** 总 results 文档 + 跨仓 ROADMAP 同步 | 元层工具 | Cluster 5 后 `task-packages/facet-migration-batch-t6d-results.md` commit + facet ROADMAP §P5 同步 commit |
| **F9** 任何 Cluster F 锚定触发 = 失败回滚 + 跳过 | 范畴边界 | F1-F8 任一 Cluster 失败，立即 trail + 跳过后续 Cluster |
| **F10** 长程任务 1 session 不中断 | 元层工具 | 主 agent session 持续 active，每 Cluster 完成自动恢复 + 派下一个 |

**F 锚定触发（F9 范畴）** = 任务失败 + trail + 等用户决定

## 五、必读文件 {#read}

主线持续 active 期间，按需加载：

- T6D-10: `sih-engine/task-packages/critical-path-migration-t6d.md` + `mid-priority-migration-t6d.md`（T6D-08 + T6D-09 模式复用）
- T6D-11: `v1-scan` 输出（24 reads-only 函数清单）
- T6D-12: T6D-08 X2 报告 `sys._getframe(1)` hack 段
- T6D-13: `sih-engine/doc/design/DES-011-baseline-checker-DEC.md` + `OPEN-QUESTIONS.md OQ-22`

## 六、约束 {#constraints}

1. **1 主 session 持续 active**（用户显式指定）
2. **0 LLM 调用**（机械代码修改）
3. **每 Cluster 完成自动跑主线验收 + commit + 派下一个**（减少人类介入）
4. **任何 Cluster F 锚定触发 = 立即失败回滚 + 跳过后续 + 等用户决定**（不重试不擅自继续）
5. **T6D-13 验证流量（需厂商）= 不在本总任务包范围**（标后续 A-2 caller 集成任务包）
6. **T6-PARADIGMS.md T6-D 集群 ≤ 3 文件边界偏离** = 长程任务模拟 + 减少介入（trail 记录）

## 七、验收标准 {#acceptance}

本总任务包验收 = 4 Cluster F 锚定全部 NOT TRIGGERED + 总收尾：

### Cluster 1-4 {#cluster-1-4}

- [ ] T6D-10 实施 + pytest + v1-scan 4 → 0 + commit
- [ ] T6D-11 实施 + 24 函数评估 + commit
- [ ] T6D-12 实施 + `sys._getframe(1)` 0 命中 + pytest + commit
- [ ] T6D-13 实施 + baseline_checker 实现 + 4 守卫 + cross-link + commit

### Cluster 5 总收尾 {#cluster-5-总收尾}

- [ ] 总 results 文档 commit
- [ ] 跨仓 ROADMAP §P5 同步 commit
- [ ] trail 记录 4 串行 + 1 session 长程任务

### F 锚定 10/10 {#f-锚定-10-10}

- F1-F8 全部 NOT TRIGGERED
- F9 失败回滚机制不触发
- F10 1 session 持续 active

## 八、风险点 {#risks}

### 风险 1：主 session 长程 active（数天） {#风险-1-主-session-长程-active-数天}

**缓解**：每 Cluster 后立即 commit + 写 trail + 派下一个，session 不会因单个错误中断。

### 风险 2：4 Cluster 串行失败成本高 {#风险-2-4-cluster-串行失败成本高}

**缓解**：每 Cluster 独立 commit + 独立回滚（不回滚全部）。F9 触发 = 该 Cluster 回滚 + 跳过后续。

### 风险 3：双子代理 context 累积 {#风险-3-双子代理-context-累积}

**缓解**：每个 T6D 是独立任务包 + 独立 prompt，主线 context 只增 task package 引用 + commit hash，不重复加载。

### 风险 4：T6D-13 实施 vs 验证混淆 {#风险-4-t6d-13-实施-vs-验证混淆}

**明确**：本总任务包 = T6D-13 **实施**（0 厂商）。T6D-13 **验证**（需 A-2 caller 集成）= 后续任务包，不在本范围。

### 风险 5：长程任务模拟偏离 T6-PARADIGMS.md {#风险-5-长程任务模拟偏离-t6-paradigms-md}

**缓解**：trail 记录偏离理由 = 用户显式指定 + 长程任务模拟 + 减少介入。按 PRO-09 模式选择元层第四防御：subagent 不能单方面确定模式变更，但用户已显式指定。

## 九、关联文件 {#related}

- 任务包 1（Cluster 1 实施）：T6D-10 = `task-packages/critical-path-migration-t6d.md` + `mid-priority-migration-t6d.md` 模式复用
- 任务包 2（Cluster 2 实施）：T6D-11 = 待立
- 任务包 3（Cluster 3 实施）：T6D-12 = 待立
- 任务包 4（Cluster 4 实施）：T6D-13 = `doc/design/DES-011-baseline-checker-DEC.md` 实施
- 后续任务包（不在本总任务包）：A-2 caller 实际集成（proposition-defense + redteam 2 skill 走 facet pipeline）
- 上游：`sih-engine/task-packages/fix-governance-boundaries-t6d-v1-family-inventory.md`
- 范式文档：`ai-ex/T6-PARADIGMS.md`（T6-D 集群 ≤ 3 文件边界，本总任务包偏离）
- 跨仓：sih-tools/facet/ROADMAP.md §P5
