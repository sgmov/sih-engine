# webui-security-parallel 任务包

- 批名：webui-security-parallel（PR #55 复审安全加固批）
- 形：并联四簇（net / upload / db / ws），各自隔离工位（git worktree），主线串行集成验收
- 仓：/Users/moc/workspaces/MiniMax-Code-Plugins（主克隆，feat/v2-refactor @ 7b4aae80 = PR #55 头，树净）
- 工位：/Users/moc/workspaces/webui-sec-{net,upload,db,ws}，各自分支 webui/sec-{net,upload,db,ws} @ 7b4aae80
- 插件：plugins/Wzdhehe/mcode-webui（零运行时依赖，测试=node:test 全零依赖）
- 日期：2026-09-21
- 评审来源：hetaoBackend 对 #55 的 CHANGES_REQUESTED（2026-09-21T01:18Z，头 7b4aae80）

## 簇划分与文件领地（互斥，越界即停手申报）

| 簇 | 评审点 | 文件领地 |
|---|---|---|
| net | 1 通配 CORS×本地免权组合洞 + 2 默认绑 0.0.0.0 | server/router.js、server/lib/auth.js、server/lib/lan.js、server/routes/settings.js、server/lib/settings.js、server/lib/config.js |
| upload | 3 上传无界 | server/lib/upload.js、server/routes/upload.js |
| db | 4 会话删除假成功 | server/lib/db.js |
| ws | 5 工作区与持久化边界 | server/lib/workspace.js、server/lib/sessions.js |

## 共同不变量

1. 各簇只动自己领地（新测试文件除外）；不改共享面（如 server.js 装配序）时如确需，停手申报。
2. CLI/curl/MCP 等无 Origin 客户端零回归——安全边界针对浏览器面，不误伤本机工具链。
3. 假成功禁令延续：任何失败路径不得报成功（本批 db 簇的评审点正是此病家族）。
4. 单 commit 落各自分支，正文引评审原文要点；禁 git add -A；不 push。
5. 验证裸命令真实退出码：npm test（全套件）exit=0、git status 净。

## F-锚

- H1 各簇评审点红转绿并有回归测试钉住（形态见各簇施工规格）。
- H2 既有全套件零回归（套件数只增不减）。
- H3 主线集成后：全套件 + siinfer 远程门 + fork 预览（windows+codeql）全绿，更新 PR #55 并回复评审。
- H4 结果档 + T6 + sih-engine 提交。

## 检索申报

本批为上游安全加固施工，正典约束零命中预期；施工规格以评审原文为准。
