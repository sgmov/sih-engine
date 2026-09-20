# webui-rigor-fix-parallel 结果档

- 批名：webui-rigor-fix（mcode-webui v2 严谨性修复批）
- 范式：并联形。队形验证一行：**并联形实跑成立**——八施工簇（G1/S1/H1/W2/H2/M1/U1/S2）后台并行/接序各领独立簇，主线零代码施工、专职验收与裁决；用户指令覆盖"主线写最复杂件"位如实记于任务包。
- 日期：2026-09-20。对象：MiniMax-Code-Plugins@feat/v2-refactor（PR #55）之 plugins/Wzdhehe/mcode-webui。
- 切面检索：主题轴 recall（mcode-webui）3 命中均为微软 agent-governance-toolkit 审计日志先例（webui-rigor-fix-recall-20260920.md）；事件轴 recall（2026-09-20 当日）零命中如实记（webui-rigor-fix-slice-20260920.md，envelope count 0）。

## 完成度表

| 簇 | 面 | 状态 | 要点 |
|---|---|---|---|
| G1 治理核 | 审计 fail-closed + 移除测试态自动放行 + 写前审计 + 闸门阻断测试 | ✓ 验收 | append 失败即抛；九位点 intent/outcome；authorize 无 execArgv 分支；顺带修旧签名 bug（真实链 4 行脏行止血） |
| S1 安全雷 | 三 CodeQL（正则/innerHTML/cmd.exe） | ✓ 验收 | 消回溯正则；DOM 构造；mcode-exec 直 spawn node 入口全链 fail-closed |
| H1 工程收尾 | 52 临时文件、断脚脚本、CI.md 诚实化 | ✓ 验收 | 撤"8 CI runs"虚声明；lint/format 死门带全历史查证删除 |
| W2 slash 接线 | chat.js 直连旁路修复 | ✓ 验收 | 接回带闸壳；变异检验证明测试抓旁路 |
| H2 文档收口 | 六处漂移 + SBOM + 更正注记 | ✓ 验收 | plugin.json/SECURITY-NOTES 与上游 v2 规划历史档头部更正注记，同幅如实化 |
| M1 mock 迁移 | 23 件迁 checks/ 解锁根门禁 | ✓ 验收 | 根发现目录规则实证；test:unit 无旗标化；devDeps 裁至 c8；SBOM 115→52 components |
| U1 上传隔离 | server-spawn 测试 env 重定向 | ✓ 验收 | 五件补齐 UPLOAD_DIR/SESSIONS_DB；杂散目录零回生 |
| S2 新告警 | render.js 确认条 + router.js 两处 | ✓ 验收 | DOM 化；regex-injection 判名字面伪报取最小加固带全注释 |

主线补笔（验收位，非簇施工）：COVERAGE-REPORT 两行、PR_DESCRIPTION 两语三处（checks/ 与 test:mocked 纪实）。

## F 验证表（主线亲跑，真实退出码）

| 锚 | 结果 | 判定 |
|---|---|---|
| F1 带旗标全量 | 929/923/4（四例全为 better-sqlite3 ABI 环境性，与基线逐例一致） | ✓ |
| F2 无旗标全量 | 537/535/2，t.mock.module 报文 0 | ✓（合并硬堵点已除） |
| F3 对表 | exit 0 | ✓ |
| F4 机械审计 | cmd.exe=0、innerHTML=0、execArgv=0、吞错返 null=0、脚本缺件=NONE、gitignore 命中、杂散目录零回生 | ✓ |
| F5 根仓全量 | validate 全 OK（含 Wzdhehe/mcode-webui），根套件 639/637/2（同族 ABI 环境性） | ✓ |
| F6 siinfer 终试 | 第六发 FULL GATE GREEN（99434b3，647/647 零失败，真实退出码 0）。六轮迭代全记录：①remote-ci.sh 旧血统硬编码（条件化）②杂散 .webui-uploads（U1 根因）③Linux 悬挂（U4 定案 acp.mjs 生产排水缺陷并修复）④sqlite3 环境缺件（apt download+dpkg -x 免 root 抽取）⑤better_sqlite3_not_loaded（U5 门控）⑥终绿 | ✓ |
| F7 T6 链 | 本档与任务包过化格→核阅→检词 | 收口时执行 |
| 附加：fork 预览门 | 四轮迭代：CodeQL 每轮绿、Windows 从 3 红面收敛至双 job 全 success（run 35496393762）；对照实验揭出三颗安全页隐藏雷（S2 簇立项依据） | ✓ |
| 附加：推送与评论 | 99434b3 已推（053c594..99434b3），PR #55 validate 38 秒绿（旧 head 同套件爬五小时），全量证据评论在案 | ✓ |

## 追加簇（U 系列，八簇之后按证据链续派）

| 簇 | 面 | 状态 | 要点 |
|---|---|---|---|
| U1 上传隔离 | server-spawn 测试 env 重定向 | ✓ | 五件补齐 UPLOAD_DIR/SESSIONS_DB；杂散目录零回生 |
| S2 新告警 | render.js 确认条 + router.js 两处 | ✓ | CodeQL 六雷清零的另一半 |
| U2 超时持活 | lib-authorize unref'd 定时器 | ✓ | watchdog 持活形 |
| U3 轮询持活 | withDecisions unref'd interval | ✓ | ref'd + 5s 兜底自清 |
| U4 Linux 悬挂 | acp.mjs 生产排水缺陷 | ✓ | 上机证伪静态结论；error/exit/close 三信号幂等排水（生产修复经用户裁定） |
| U5 平台诚实化 | 六件测试环境/平台门控 | ✓ | 26 例逐项有主；15 例改写优先于 skip；发现 _probeCandidate 真值陷阱（申报债） |

## 事件与处置纪实

1. #55 validate 慢爬诊断：现推 head 带测试态自动放行 + 根门禁无旗标，每过闸测试真等 5 分钟超时——G1/U1/M1 三修后此形态根除。
2. F6 两揭：remote-ci.sh 对 #42 血统插件硬编码（条件化修复）；.webui-uploads 测试隔离泄漏（U1 根因修复 + tar 排除双保险）。杂散 .webui-sessions.json 查实为空会话壳，归位 ~/.mcode-webui/ 正名。
3. fork 预览通道（用户刷 workflow scope 后建成）：对照实验在未修复 head 抓出 6 告警（上游行内仅示 3），机械性验证通过；S2 簇由此立项修毕三新雷。
4. M1 施工事故：探针误执行脚本顶层 rmSync 删 fixture，按 .dump 全等再生并恢复 HEAD 字节，过程全披露。
5. 已申报权衡三处（用户质询后复核确认）：checks/ 迁移的根门禁覆盖面取舍、authorize 决策在审计落笔失败时仍 resolve（响亮告警形）、prepush.sh 本地 ABI 签名白名单（siinfer 权威门无白名单）。

## 产出清单

- 代码/测试/文档改动 125 件（41 M + 77 D + 7 新，全部困在插件域）；checks/ 23 件、新测试 6 件、.gitignore、SBOM 再生（52 components）。
- 工具件（不入上游仓）：mcp-local-ci/prepush.sh（五层机械闸）、mcp-local-ci/preview.yml（fork 侧 CodeQL+Windows 预览）、remote-ci.sh 条件化与排除面修订。
- 推送决策：候用户。推送前序=本档 F6 补行 + 修复镜像分支真预览 run（CodeQL 零告警 + Windows 双模绿为预期）。
