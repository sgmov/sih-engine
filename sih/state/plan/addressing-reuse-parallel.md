# 任务包：addressing-reuse-parallel

批名：addressing-reuse-parallel。实日：2026-09-18。

## 队形声明

并联形（parallel）。双子代理各领一簇：测试簇（checks/cross-reuse.check.mjs）与实现簇（src/engine.mjs、src/store.mjs、README.md），两簇同以本包 SDD 为唯一契约独立产出；主线亲写 SDD 与任务包并作收敛验收（先红后绿）与全门禁复跑。归宿：外部仓 MiniMax-AI/MiniMax-Code-Plugins，PR 对 main，正文引用 sih-tools/locator 为设计出处（派生稳定标识、键即身份）。

## 背景与目标

dynamic-workflow 插件（#42 已合入 main）现有复用仅限修复血统（repair 草稿经 reuseStepIds 指回单一源运行）。本批实现**内容寻址跨运行复用**：上下文（workspace/executor/input/fingerprints）与调用规格（spec，含 prompt/schema/input）逐项等值时，任何后继运行可复用任何先前运行的成功节点结果，出处保留、按需选入、预算不计。设计出处：sih-tools/locator 确定性寻址（派生稳定标识）。诚实边界：复用只证上下文同一与存储忠实，不证首跑语义正确。

## 温故检索记录

主题轴（寻址/复用，conclusion+experience）命中 3 条经验档，出自 legacy-sihankor 的 setsp-reuse-decision-2026-07-22.md（复用状态：直接复用）。无冲突先例。输出件：sih/state/plan/addressing-reuse-recall-20260918.md。

## SDD 契约（两簇唯一依据；实现基于 origin/main 的现行代码，不含未合入的 #44/#45/#48）

### 运行级字段

- `reuseAcrossRuns: boolean`，默认 false。start() 入 definition（校验：缺省 false、提供则必须为布尔）；update() 可改（request.reuseAcrossRuns===undefined 时保持原值，提供则必须布尔）。字段随 definition 进 requestHash——改它即参数变更，requestId 幂等语义自动成立。

### 复用键与查找

- 上下文四要素与 repair 复用的 contextHash 同构：`ctxHash = hash({workspace,input,executor,fingerprints})`；每个 run 的 ctxHash 惰性计算一次缓存在执行上下文（ctx）上。
- 查找：`store.findCrossRunReuse({contextHash,requestHash,excludeRunId,limit=20})` → 按 steps.rowid 降序返回至多 limit 条 `{runId,stepId,step}`，条件为 `steps.kind='agent' AND steps.status='succeeded' AND steps.requestHash=? AND runs.id<>?`，运行体解析后逐条校验 `hash({workspace:r.workspace,input:r.input,executor:r.executor,fingerprints:r.fingerprints})===contextHash`，不等的跳过。
- 引擎消费（agent() 内，位于既有 repair 候选块之后、预算闸之前，且仅当 `ctx.run.reuseAcrossRuns && !previous`）：遍历候选，`validateOutput` 存在时逐个重验 schema，首个通过者采用；全部无效则落新调用。采用时写入 step：内容承自源 step、`attempt:0`、`createdAt:Date.now()`、`startedAt:null`、`endedAt:源step.endedAt`、`usage:null`、`usageHistory:[]`、`sessionId/turnId:undefined`；`reusedFrom = 源step.reusedFrom ?? {runId:源runId,stepId:源stepId,endedAt:源step.endedAt,crossRun:true}`（保持最初生产者出处，运行内血统链不被跨运行标记覆盖）；emitEvent `step.reused` 附 `{sourceRunId,crossRun:true}`。预算闸之前返回——复用不计 attempts。
- 失败/运行中节点不复用；本运行自身被 excludeRunId 排除；previous/cached 既有短路优先级不变。

### 文件面

- src/engine.mjs：definition 增字段与校验、update 透传、agent() 插入跨运行候选消费、ctx 惰性 ctxHash。
- src/store.mjs：新增 findCrossRunReuse({contextHash,requestHash,excludeRunId,limit})。
- README.md：增"Cross-run reuse"小节（选入式、键等值条件、出处字段、诚实边界一段）。
- checks/cross-reuse.check.mjs：新测试套件。
- dist/main.mjs：实现后重建（可复现构建）。

### 不做的事

- 不改 repair 复用路径一行语义；不加新 MCP 工具或端点；不做跨 workspace 复用（store 本身按 workspace 分库，天然隔离）；不接完整性台账（#48 未合入，正文注明接缝：后续小 PR 把被接受的跨运行复用写入台账面）。

## 簇任务

- 测试簇（子代理 A）：checks/cross-reuse.check.mjs，≥10 案：默认关闭（两次同参运行第二次仍新调用）；选入后命中（attempt 0、reusedFrom.runId 指向源、crossRun true、调用数不增）；input 变更不复用；executor 变更不复用；prompt 变更不复用；schema 重验拒绝不合输出的旧结果（改 schema 后新运行不复用）；仅 succeeded 复用（源失败节点不复用）；同键多候选取最新；纯复用运行 maxCalls=1 仍成功（复用不计预算）；fingerprints 变更不复用（文件内容变化）。风格对齐 repair.check.mjs（fixture + start/finish 内联辅助），Engine fixture 的 execute 记录调用。
- 实现簇（子代理 B）：按 SDD 产全量改后 src/engine.mjs、src/store.mjs 与 README.md 增节，风格对齐现行（密集、中文文案），node --check 过。基于 origin/main 现行代码（不含 #44/#45/#48 未合入改动）。

## TDD 协议

主线先入测试簇 → `node --test checks/cross-reuse.check.mjs` 必红 → 入实现簇 → 同命令绿 → 全套 npm test 绿 → 打包冒烟、构建可复现、仓库校验器（干净路径）→ 提交推分支开 PR。

## F 验收清单

1. 红相在案；2. 绿相 cross-reuse 全过；3. 全套绿（main 基线 72 + 新增 ≥10）；4. 打包冒烟过 + dist diff 仅本批文件；5. 校验器干净路径 exit 0；6. PR 开出，正文含键推导、选入语义、出处字段、locator 出处与诚实边界。

## 结果档

完成后另文 addressing-reuse-parallel-results.md（完成度表、F 表、队形验证一行、插曲经验），起草前跑事件+时间轴检索。
