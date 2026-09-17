# 结果档：integrity-ledger-parallel

实日 2026-09-18。任务包：sih/state/plan/integrity-ledger-parallel.md（T6 过，书简认证 event_hash 751c3b1287c5f62b80e583e5801a7b06adcae5faadeabc24b8cefbe09a47b1c1）。机械底稿：sih/state/plan/integrity-ledger-results-recall-20260918.md（事件轴检索 20 行）。

## 完成度表

| 件 | 负责 | 状态 |
|---|---|---|
| SDD 契约与任务包 | 主线 | 完成（T6 三步过：化格 0 改动、检词零违例、核阅 0） |
| 测试簇 checks/integrity.check.mjs | 子代理 A | 完成，八案全写，node --check 过 |
| 实现簇 src/store.mjs + src/tools.mjs | 子代理 B | 完成，含一轮修正（逐行台账升级） |
| 主线验收（红→绿→全门禁） | 主线 | 完成 |
| PR | 主线 | MiniMax-AI/MiniMax-Code-Plugins#48 已开（base main，Closes #46） |

## F 验证表

| F | 验证 | 结果 |
|---|---|---|
| F1 红相 | 测试先入，8/8 fail（TypeError: integrityHeads is not a function） | ✓ |
| F2 绿相 | integrity 套件 8/8（v1 实现 6/8，两红判负单末端锚设计，升级逐行台账后 8/8） | ✓ |
| F3 全套 | 80 测试（基线 72 + 新增 8）；一次 79+1 已知竞态偶发（#44 修复对象，非本批引入），一次全绿样本在案 | ✓ |
| F4 打包 | test:package 1/1；构建后 git diff 仅含本批 5 文件（dist 80 行纯增量，git show --stat 核对无 #44 夹带） | ✓ |
| F5 verify-claims | 不适用——claims 表在 #45 分支，本 PR 无该文件，如实记 | n/a |
| F6 仓库门禁 | 干净路径（node_modules 挪开）npm run validate exit=0，27 插件全过 | ✓ |
| F7 PR | #48 开出，正文含 SDD 要旨、Closes #46、sih-engine 出处 | ✓ |

## 队形验证

并联形实名实跑：双子代理各领一簇并行交付（测试簇 543s / 实现簇 913s+507s 含修正轮），主线亲写 SDD 契约与任务包并收敛验收，修正轮经 SendMessage 串行回实现簇一次，无形名不符。

## 插曲与经验（入档供后续批复用）

1. SDD 漏了 engine.start 外层事务包裹——实现簇自补可重入事务（txDepth），属任务包缺陷被实现侧正确补救。
2. 单末端锚在信息论上无法定位中间行篡改——测试簇按契约判负 v1 实现，升级 integrity_rows 逐行台账后精确命中。测试先行的直接实证。
3. workflow_status 列表形数组→对象的契约变更被既有消费者测试（workspace-router 第 55 行 .map）当场抓住，本批携带一行迁移；裸 JSON 数组挂不了字段是硬约束，设计期就应写入 SDD。
4. 堆叠 PR 不可行：base 分支在 fork 时上游仓看不到该 ref，GitHub 拒建——改 rebase 回 main 单提交 + 正文依赖注记。
5. cwd 漂移一次导致 build/amend 断链、dist 夹带 #44 改动的风险——git show --stat 纯度核对兜住；外部仓批的命令一律绝对路径。
6. 资源：双子代理合计约 184 万 subagent tokens；主线门禁复跑 5 轮。
