# fail-loud-serial 批结果档

- 批链：fail-loud-serial（任务包 fail-loud-serial.md）+ schema-robust-serial（任务包 schema-robust-serial.md）+ win 清理两补（14a7fd5、e1c976b，随批收口）
- 仓：MiniMax-Code-Plugins（fork modacker），分支 community/fail-loud，基线 e3d0166（PR #52 头）
- 终头：e1c976b；PR：https://github.com/MiniMax-AI/MiniMax-Code-Plugins/pull/57（栈于 #52）
- 日期：2026-09-20
- 形：串形三簇（fail-loud 簇、schema-robust 簇、win 清理补簇），主线全程验收

## 提交清单

| commit | 内容 | 簇 |
|---|---|---|
| 6a3cc6c | 引擎层失败响亮化：approve/resume 闸前 mcode --version 探测（零模型调用）、零展开护栏 NO_AGENTS_EXECUTED、全灭传播 EXECUTOR_FAILURE_CODES、run.preflight 透出；checks/fail-loud.check.mjs 五用例 | fail-loud |
| 1af672c | 结构化输出无歧义单候选提取：恰一围栏（json_fence_embedded）或恰一洁净边界裸候选（json_embedded），schema 仍为唯一数据闸，拒绝面守恒；检查件 4→7 测 | schema-robust |
| 14a7fd5 | win32 清理竞态第一修：rmWithRetry 瞬态码指数退避（后被证明不充分，保留为兜底） | win 清理 |
| e1c976b | win32 清理根因修：清理前显式 --stop-service 杀 detached 守护进程（对齐 fail-loud/lifecycle 惯例） | win 清理 |

## 现场证据摘要

### 缺陷一：引擎层全灭被译为成功

2026-09-20 坏 CLI 窗口（better-sqlite3 ABI 断裂 + mcode v0.2.4），DB 落册三笔假成功：12058707、272b0cb6、3b9fcbff，全部 status=succeeded、executor=mcode、attempts=0、零步骤、errorDetails=null，事件面 run.started→run.finished 42ms 内。CLI 修复后三通道（表单/MCP/桌面）同脚本全绿，坐实吞错为引擎独立缺陷。

### 缺陷二：散文包裹的合法 JSON 被整段校验处决

同日两份用户反馈同形态：worker 最终输出 = 完整合法 JSON + 前导或尾随叙述散文，副作用产物（文档）完好，节点判 OUTPUT_SCHEMA_INVALID，render_result 置 null，run completed_with_gaps。英文报错串经 grep 三面（src/dist/web）零命中，判定拒判动作在用户脚本自己的校验层，但病根形态与插件 structured-output.mjs 脆性同构，插件层修为无歧义提取。

## 主线验收记录（验收子代理报告不可信，全部亲跑）

- fail-loud 簇：F1 diff 逐行过目；F2 npm test 123/123、test:package、build、dist 重建幂等、树净；F3 亲手独立复现（PATH 注入 NODE_MODULE_VERSION 141/147 坏 CLI → MCP 面起服务 → approve 484ms 拒，run failed/MCODE_PREFLIGHT_FAILED/attempts 0/steps 0，stderr 证据透出）。
- schema-robust 簇：diff 过目（字符串感知扫描转义处理、围栏优先、破围栏不落裸层、尾随结构符截断征兆判死）；npm test 126/126 亲跑；独立 G1 复现用两份截图逐字原文跑通（前导散文形与 JSON 前置形均 json_embedded 提取且字段完好，负面对照双候选/尾逗号/类型错照旧拒绝）。
- win 清理两补：diff 过目（close 错误延后重抛不吞、瞬态码穷尽后仍失败可见；停服务步身份匹配论证成立）；npm test 126/126 亲跑两轮。

## 闸门矩阵（终头 e1c976b）

| 闸门 | 结果 |
|---|---|
| darwin npm test | 126/126 exit=0 |
| darwin test:package / build / dist 幂等 / 树净 | 全绿 |
| siinfer（Linux，Node 24，净树）仓库根门 | 345/345 exit=0（14a7fd5 与 e1c976b 两跑皆绿） |
| siinfer 插件套件 | 126/126 exit=0 |
| fork 预览 windows-latest 全套件 + 构建幂等 | 绿（run 35506787027） |
| fork 预览 CodeQL | 绿 |

Windows 门三跑弧线：35506090983 暴露 cross-reuse-mcp 清理 EBUSY → 35506404071 烧尽 6 秒重试预算坐实 detached 守护进程根因（--stdio spawn detached、cwd=workspace、句柄锁树）→ e1c976b 停服务修复后 35506787027 全绿。

## 偏离与裁量记录

- engine.check.mjs 一处既有断言翻转（missing CLI 拒后 run 留 pending_review → 改判 failed）：与不变量 1 直接冲突，按包裁定改判，failed 可 resume 不损可恢复性。
- schema-robust 施工裁量：恰一围栏但内容 parse 失败时不落裸候选层（围栏是更强意图信号，破围栏+他处裸 JSON 视为歧义拒），收敛于无歧义不变量。
- 截断征兆判定只查闭括号后首个非空白字符（不查前导侧）：前导冒号与散文冒号不可区分，前导侧截断由 schema 闸兜底，方向为过度拒绝（安全侧）。

## 已知残留（如实）

- 探测无 TTL 缓存，每次 approve/resume 冷启一次 --version。
- /api/config.mcodeAvailable 仍为解析级未接探测（不变量 4 最小落点在 run.preflight）。
- 新错误码未加 web i18n 专键，走既有降级。
- 用户脚本自建严格 JSON 校验的形态（英文报错出处）不受本批插件修复覆盖，正解为脚本改用 ctx.agent schema 参数，已写入 PR 指引。

## 检索申报

fail-loud 簇 3 命中（存档 fail-loud-recall-20260920.md）；schema-robust 簇 3 命中均旁系零约束（存档 recall/schema-robust-recall-20260920.md）。
