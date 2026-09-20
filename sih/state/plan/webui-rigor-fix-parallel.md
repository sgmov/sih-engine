# webui-rigor-fix-parallel（mcode-webui v2 严谨性修复批）

- 范式行：并联形 parallel。三子代理并行各领独立簇（G1 治理核 / S1 安全雷 / H1 工程收尾与 CI 诚实性）。
- 形覆盖声明：并联形正典含"主线写最复杂件"；本批用户指令明令主编排不写代码，主线亲写位转派子代理（G1 承最复杂簇），主线专职验收。此为用户指令对队形的显式覆盖，如实记。
- 日期：2026-09-20。工作对象：/Users/moc/workspaces/MiniMax-Code-Plugins（分支 feat/v2-refactor，PR #55）之 plugins/Wzdhehe/mcode-webui。
- 温故检索：sih/state/plan/webui-rigor-fix-recall-20260920.md，3 命中均为微软 agent-governance-toolkit 审计日志先例，无本插件直接先例；批内先例以 v2 重构批档（.tmp/mcode-webui-refactor/、docs/VERIFICATION-REPORT.md）为准。
- 背景：DSH/Harness 严谨性对表审计（2026-09-20 本会话）结论"形态到位、执法未到位"，六类缺口即本批修复面。

## F 锚定（验收线，逐条实跑）

- F1 插件全套件（带 --experimental-test-module-mocks）绿
- F2 插件全套件（不带旗标，双模）绿
- F3 node scripts/check-docs-alignment.mjs 绿
- F4 机械 grep 审计全过：mcode-exec.js 无 cmd.exe；main.js 初始化失败路径无 innerHTML 拼接；authorize.js 无 execArgv 测试态自动放行；events.js append 不吞写失败；git 跟踪面零 coverage 临时文件；package.json 全脚本可解析
- F5 根仓 npm ci + npm run check 绿（本地）
- F6 siinfer（Ubuntu 24.04 / Node 24）远端全量门绿：/Users/moc/workspaces/SiHankor/mcp-local-ci/remote-ci.sh feat/v2-refactor
- F7 本包与结果档过 T6 链（化格→核阅→检词）

## 簇 G1：治理核（审计 fail-closed + 移除测试态自动放行）

文件所有权：server/lib/events.js、server/lib/authorize.js、server/lib/db.js、server/lib/slash.js、server/lib/settings.js、server/routes/sessions.js、server/routes/settings.js、server/routes/export.js、server/cleanup.js、test/_setup.js、test/lib-events*.test.js、test/lib-authorize.test.js、test/integration/event-chain.test.js、以及需接入决策注入助手的其他测试件。

1. events.js：append() 失败即抛（不再吞错返 null）；_writeAtomic 读旧内容失败时抛出（禁止带截断风险继续写）；修 _resetForTests 在 ESM 中用 require 的静默无效 bug（改模块顶 import unlinkSync）。
2. 写前审计（write-ahead）落位：破坏性/状态变更流在持久变更前落 intent 事件、变更后落 outcome 事件，两处失败均向上传播（HTTP 5xx + alerts 通道告警）。覆盖位：session.delete、cleanup-orphans、cleanup-all、settings.update、token.reset、slash.clear/new、db 级会话删除、export、upload。
3. authorize.js：整体移除 execArgv 探测的测试态自动放行分支（约 134-164 行）及 opts.testMode 残留；_tryWriteEvent 改静态 import（events.js 已在）；决策结局事件（approve/reject/timeout/cancelled）落笔失败不得静默——pushAlert + console.error，用户决策照常 resolve（点击不可撤销，如实在注释里记理由）。
4. 测试侧决策注入：test/_setup.js 增助手（经 getPendingRequestIds / _decideForTests 驱动真实决策路径），不触生产代码、不依赖模块 mock 旗标；所有间接触发 authorize() 的既有套件改用此助手。
5. 新增闸门阻断集成测试：拒绝即不删、超时即拒、批准即删且审计链 verify() 仍 ok。

## 簇 S1：安全雷（三 CodeQL）

文件所有权：server/lib/auth.js、public/app/main.js、server/lib/mcode-exec.js、test/lib-auth.test.js、新 test/lib-mcode-exec.test.js。

1. auth.js extractToken：`/^Bearer\s+(.+)$/i` 改 `/^Bearer[ \t]+(\S.*)$/i`（不相交字符类消回溯）；补边界测试（多空格/制表符、Bearer 后无空格、Bearer 后空捕、大小写）。
2. main.js 初始化失败兜底：innerHTML 字符串拼接改 DOM 构造 + textContent；补静态断言测试（该 catch 块零 innerHTML 赋值）。
3. mcode-exec.js：根除 cmd.exe。零依赖本地解析 MCODE_CMD：PATH 定位；.cmd/.bat 垫片时定位同级 node_modules/@minimax-ai/code/cli.js 直 spawn process.execPath [entry,...]；解析失败即抛清晰错误（fail-closed），不回退 cmd.exe。模式参照（只读不 import）：plugins/hetaoBackend/mcode-dynamic-workflows/src/mcode-location.mjs。Windows 路径同理。
4. lib-mcode-exec.test.js：解析器单测（伪 PATH 垫片目录夹具：断言解析为 process.execPath + entry、绝无 cmd.exe；不可解析即抛）。

## 簇 H1：工程收尾与 CI 诚实性

文件所有权：package.json、docs/CI.md、.github/workflows/ci.yml（删）、.gitignore（建/扩）、COVERAGE-REPORT.md（补注）。

1. 卫生：删除 coverage/ 与 coverage_tmp/ 两目录（52 个临时 JSON）；.gitignore 补 coverage/、*.lcov、*.log。
2. package.json：处置 setup:plugin / package:plugin 两个断脚（scripts/ 下无对应文件）——查 git 历史有无 v1 实现，有则复原、无则删条目；test 脚本 glob 扩至 test/integration/*.test.js 与 test/matrix/*.test.js，加 test:unit / test:integration 别名；逐脚本核验可解析。
3. CI 诚实性：删插件子目录 .github/workflows/ci.yml（子目录 workflow 永不触发）；CI.md 重写为真实门禁（市仓根 validate + 本地 test 全序 + lint + sbom/audit），撤"每次 push 8 个 CI 运行"的虚声明，矩阵表改为"本地复现命令"。
4. COVERAGE-REPORT.md 补注：c8 排除面（mcode-acp/mcode-exec/mcode-rpc/acp.mjs）现状与 mcode-exec 新增单测的说明。
5. 双模自检：本簇改动后跑带旗标与不带旗标两遍套件，失败如实报（G1 域失败转交主线转达，不越界修）。

## 验收协议（主线）

1. 不信子代理报告：F1-F4 由主线亲跑（套件双模、对表脚本、grep 审计、脚本解析）。
2. 代码抽查：G1/S1/H1 全部改动 diff 逐件过目（git diff），形名不符或越界改即打回。
3. F5 根仓本地全量（npm ci + npm run check）。
4. 全绿后 F6 siinfer 远端门。
5. 结果档 webui-rigor-fix-parallel-results.md 含完成度表、F 验证表、队形验证一行；起草前按事件/时间轴跑温故检索取切面。
