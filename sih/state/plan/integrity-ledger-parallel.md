# 任务包：integrity-ledger-parallel

批名：integrity-ledger-parallel。实日：2026-09-18。

## 队形声明

并联形（parallel）。双子代理各领一簇：测试簇（checks/integrity.check.mjs）与实现簇（src/store.mjs、src/tools.mjs），两簇同以本包 SDD 为唯一契约独立产出；主线亲写最复杂件即 SDD 契约与任务包本体，并作收敛验收（先红后绿）与全门禁复跑。产出归宿：外部仓 MiniMax-AI/MiniMax-Code-Plugins 的 dynamic-workflow 插件，PR 对 main，Closes 上游 issue #46。

## 背景与目标

上游插件 dynamic-workflow（#42 已合入 main）的 repair_cache 行是后续运行的复用输入，SQLite 文件无完整性保护（README 自述不防同机其他进程）。目标：对追加面（repair_cache 插入与 events）加哈希链，workflow_status 回显链头并可按需全量校验，使"复用所信即先前所产"可机械核验。威胁模型是 agent 手滑与误操作，非蓄意攻击者。零迁移、零新工具、零既有行为变更。

## 温故检索记录

主题轴（哈希链/完整性/留痕，conclusion+experience）命中 3 条经验档，均出自旧仓 SETSP 技术选型先例（.tmp/SihEngineeringTechnologySelectionPrecedent/.../repository-structure-draft.md）：哈希链 vs 区块链之辨、OAuth+哈希链+SQLite 组合、前置记录哈希（哈希链）。结论：防篡改留痕的既定选型即哈希链，与本设计一致，无冲突先例。输出件：sih/state/plan/integrity-ledger-recall-20260918.md。

## SDD 契约（两簇唯一依据）

### 链机制

- 每数据库两条独立链：events 链（按 seq 升序遍历）、repair 链（按 repair_cache 的 rowid 升序遍历）。
- 行摘要：`sha256 hex of "${prev}:${kind}:${key}:${body}"`；prev 为 64 位十六进制（创世 = 64 个 '0'）；kind 取 'event' 或 'repair'；key 取 String(seq)（events）或 `${runId}/${id}`（repair）；body 为库内存储的原始 JSON 字符串。
- 链头存 settings 表：键 `integrity_events` 与 `integrity_repair`，值为 JSON `{head, upto}`；upto = 已覆盖的最大 seq 或 rowid。
- 写路径：`event()` 与 `saveRepairCandidate()` 均以 `this.transaction(...)` 包裹"插入 + 链头推进"，崩溃不可能拆散两步。链头缺省时 prev 取创世、upto 取本行位置——首个锚定写隐式承诺当时既有行的内容，事后篡改可检出。
- `verifyIntegrity()`：纯读计算，从创世走链至 upto，末端哈希与记录链头比对；返回 `{events:{head,upto,verified,checked,unchained,firstDivergence},repair:{同构}}`。覆盖区内行被改/删 → verified:false 且 firstDivergence 报 `{surface,key,expectedHead,actualHead}`；upto 之外的行计 unchained（诚实窗口，不算篡改）；链头缺省 → `{head:null,upto:0,verified:null,checked:0,unchained:行数,firstDivergence:null}`。verify 不写库。
- `integrityHeads()`：轻读 settings 两键，返回 `{events:{head,upto}|null,repair:{head,upto}|null}`。

### tools 层

- `workflow_status`（无 runId 的列表形）响应恒增 `integrityHeads` 字段；入参增可选布尔 `verifyIntegrity`（默认 false），为 true 时响应再增 `integrity` 字段（全量重算结果）。带 runId 的单运行形不变。TOOLS 里 workflow_status 的 inputSchema 增 `verifyIntegrity:{type:'boolean'}` 可选属性。全为增量字段，既有消费者零影响。

### 文件面

- src/store.mjs：event()/saveRepairCandidate() 事务化挂链；新增 verifyIntegrity() 与 integrityHeads()；内部摘要助手。
- src/tools.mjs：workflow_status 增字段与 schema 可选参。
- checks/integrity.check.mjs：新测试套件。
- dist/main.mjs：实现合入后重建（可复现构建）。

## 簇任务

- 测试簇（子代理 A）：按 SDD 写 checks/integrity.check.mjs 全套，风格对齐 checks/repair.check.mjs（node:test、assert/strict、密集单行、无新依赖、Store 直接构造 + Engine fixture + createToolHandler 三层）。最少覆盖八案：双链锚定、repair 篡改检出与恢复后复验、events 删行检出、settings 链头篡改检出、upto 外行计 unchained 不误报、首个锚定写承诺既有行（预置直插行后被改必检出）、workflow_status 的 integrityHeads 恒在与 verifyIntegrity 按需、repair 全流程后链仍 verified。交付：/tmp/wf-integrity/checks/integrity.check.mjs，node --check 通过，报测试清单。
- 实现簇（子代理 B）：按 SDD 产全量改后 src/store.mjs 与 src/tools.mjs，风格对齐现行（密集、中文报错文案），不改动既有公开行为，node --check 通过。交付：/tmp/wf-integrity/src/ 两文件，报新增方法与行号。
- 主线件：本包（SDD）、先红后绿验收、全门禁、PR 正文与提交。

## TDD 协议

主线先只入测试簇产出 → 跑 `node --test checks/integrity.check.mjs` 必须红（Store 缺方法即失败）→ 再入实现簇产出 → 同命令绿 → 全套 `npm test` 绿 → 打包冒烟、构建可复现、verify-claims、仓库级门禁（node_modules 挪开）逐项复跑，全绿方算完成。

## F 验收清单

1. 红相存在（测试先于实现失败）并留输出。
2. 绿相：integrity 套件全过。
3. 全套 npm test 全过（73 + 新增）。
4. test:package 过；npm run build 后 git diff 仅含本次预期文件。
5. verify-claims 4/4（提交树上）。
6. 仓库级 npm run check 干净检出绿。
7. PR 提交并对 main，正文含 SDD 要旨、Closes #46、sih-engine 出处引用。

## 结果档

完成后另文 sih/state/plan/integrity-ledger-parallel-results.md，含完成度表、F 验证表、队形验证一行；起草前跑事件+时间轴检索取切面为机械底稿。
