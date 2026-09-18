# 结果档：addressing-reuse-parallel

实日 2026-09-18。任务包：sih/state/plan/addressing-reuse-parallel.md（T6 过，书简认证 event_hash b3b352ffd3fd491ff3d1b4fbb3050007baf05cb7d08805f8ef39b3145b0b2ae5）。机械底稿：sih/state/plan/addressing-reuse-results-recall-20260918.md（事件轴检索 27 行）。

## 完成度表

| 件 | 负责 | 状态 |
|---|---|---|
| SDD 契约与任务包 | 主线 | 完成（T6 三步：化格 0、检词 0、核阅 2=des-001 域外如实记；认证 b3b352ff） |
| 测试簇 checks/cross-reuse.check.mjs | 子代理 A | 完成，十一案全写，node --check 过 |
| 实现簇 src/engine.mjs + src/store.mjs + README | 子代理 B | 完成，node --check 过，另附 /tmp 沙盒 8 场景冒烟自验 |
| 主线验收（红→绿→回归修复→全门禁） | 主线 | 完成 |
| PR | 主线 | MiniMax-AI/MiniMax-Code-Plugins#49 已开（base main，正文引用 locator 设计出处） |

## F 验证表

| F | 验证 | 结果 |
|---|---|---|
| F1 红相 | 11 测试 6 过 5 红——五个正向特性案全红（特性缺失被契约捕捉），负向案天然过 | ✓ |
| F2 绿相 | cross-reuse 套件 11/11 | ✓ |
| F3 全套 | 83/83（基线 72 + 新增 11）；期间抓到并修复一个真回归（见插曲 1） | ✓ |
| F4 打包 | test:package 1/1；构建后 diff 仅本批 5 文件（README 6 行、新套件 106 行、dist、engine 20 行、store 32 行） | ✓ |
| F6 仓库门禁 | 干净路径 npm run validate exit=0（27 插件） | ✓ |
| F7 PR | #49 开出，正文含键推导、选入语义、出处字段、locator 出处、诚实边界、与 #48 的接缝说明 | ✓ |
| F5 verify-claims | 不适用（claims 表在 #45 分支），如实记 | n/a |

## 队形验证

并联形实名实跑：双子代理各领一簇并行交付（测试簇 391s / 实现簇 970s），主线亲写 SDD 与任务包并收敛验收（含一处主线直修的回归修复），无形名不符。

## 插曲与经验

1. **全套装门禁抓到真回归**：实现簇把 `reuseAcrossRuns:false` 无条件写进 definition，改变了 requestHash，打破 legacy 幂等回放（"legacy limits…old idempotent requests"测试红）。修法与作者 DEFAULT_LIMITS 手法同构：**缺省即不在场**，仅显式 true 时键才进 definition——对变更前的一切重放零扰动。
2. **实现簇的两次正确擅补**：store 需引 common.mjs 的 stable-JSON 哈希（上下文哈希必须与引擎侧逐位一致）；steps 表无 kind/status/requestHash 列，按既有 json_extract 先例落地。均为 SDD 未写、实现侧正确的补位。
3. 测试簇澄清假设质量高：源运行无需带旗标（闸在消费运行）、findCrossRunReuse 黑盒不直测、案7 用 broken/repaired 形构造部分成功源。
4. 资源：双子代理合计约 150 万 subagent tokens；主线门禁复跑 4 轮。
