# fail-loud-serial（mcode-dynamic-workflows 引擎吞错修复批）

- 范式行：串行形 serial。单子代理持全量上下文连续产强耦合多文件（engine/executor/store 面）；主线零亲写专职验收——用户指令显式覆盖，如实记。
- 日期：2026-09-20。工作对象：/Users/moc/workspaces/SiHankor/MiniMax-Code-Plugins（分支 community/fail-loud，基于 e3d0166 = PR #52 head 叠栈）之 plugins/hetaoBackend/mcode-dynamic-workflows。
- 温故检索：sih/state/plan/fail-loud-recall-20260920.md（3 行命中，内容见档）。
- 背景铁证（2026-09-20 本机实测档案）：CLI 损坏期间（better-sqlite3 ABI 断裂 + v0.2.4 古董），三个 workflow run 的库记录为 `run.started → run.finished(succeeded)` 同秒（42ms）、`attempts:0`、`steps:0`、`errorDetails:null`、`executor:"mcode"`、topology 有效含 planned 节点——**执行层全灭被翻译成成功**。CLI 修复后全通道验证健壮（表单/MCP/桌面三通道 × 静态/map 脚本矩阵全绿），证明吞错是独立缺陷：任何未来底层故障（CLI 再坏、auth 过期、网络断）都会再次变假成功。

## F 锚定（验收线，主线逐条实跑）

- F1 复现检查红转绿：新增 checks 检查件构造坏 mcode 环境（PATH 注入失败垫片）→ run 必须终态 failed 且 errorDetails 含可行动信息；修复前该检查红、修复后绿。
- F2 既有面零回归：npm test 全检查面绿（基线 118/118 + 新增件）、npm run test:package 绿、npm run build 后 git diff --exit-code -- dist 干净。
- F3 主线亲验：主线亲手跑复现检查 + 亲手实测一次坏环境行为。
- F4 siinfer 远端门绿：MCP_REPO=<旧克隆> remote-ci.sh community/fail-loud（注意 remote-ci.sh 的 dynamic-workflows 条件分支自会 npm ci）。
- F5 fork 预览绿：镜像分支推 fork（preview/fail-loud），windows-latest 跑 process-tree + win-launcher + 新检查，全绿。
- F6 PR：推 community/fail-loud → PR（标叠栈于 #52）+ 证据评论；推送决策归主线闸门后执行。
- F7 结果档 fail-loud-serial-results.md + T6 链 + sih-engine 治理材 commit。

## 修复不变量（设计边界，越界须申报）

1. **起跑前 pre-flight**（executor 为 mcode 时）：解析 mcode（沿用 mcode-location 面）+ 廉价活性探测（禁止真实模型调用，形如版本/接口探测）；不可用即 run 立即 failed，errorDetails 可行动（含 auth/setup 提示语向）。
2. **零展开护栏**：脚本 driver 完成但 steps==0 且 topology 含 planned agent 节点 → run failed（"planned nodes never executed"类错误）；topology 无 agent 节点的合法零步脚本不受牵连。
3. **引擎级失败传播**：spawn/exec 层失败样本进 run errorDetails，失败阻断全部执行时终态必为 failed；脚本内自行处理单步失败（读 r.status 分支）的既有语义不变。
4. workflow_status 透出 pre-flight/executor 健康为可读字段（最小实现）。
5. 不新增 npm 依赖；不动 demo executor 语义；既有 19 检查件全绿；dist 同步不变式保持。

## 验收协议（主线）

不信子代理报告：diff 逐件过目 → F1/F2 亲跑 → F3 亲手坏环境实测 → F4 siinfer → F5 预览镜像 → 全绿推 PR（F6）→ 结果档与 T6（F7）。
