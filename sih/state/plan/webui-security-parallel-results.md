# webui-security-parallel 批结果档

- 批链：webui-security-parallel（任务包 webui-security-parallel.md），五簇并联施工 + 主线串行集成
- 仓：MiniMax-Code-Plugins（主克隆 /Users/moc/workspaces/MiniMax-Code-Plugins），分支 feat/v2-refactor
- 基线：7b4aae8（PR #55 复审头）；终头：5f375b3；PR：https://github.com/MiniMax-AI/MiniMax-Code-Plugins/pull/55
- 日期：2026-09-21
- 评审来源：hetaoBackend 对 #55 的 CHANGES_REQUESTED（五点独立安全阻塞）

## 工位与提交清单

| 簇 | 工位 | 提交（工位） | 集成后 |
|---|---|---|---|
| net（评审 1+2） | webui-sec-net | f0862e1 + 2a13c06（测试 Linux 健壮性补） | 4667144、5f375b3 |
| upload（评审 3） | webui-sec-upload | d923be8 | 6f00fde |
| db（评审 4） | webui-sec-db | 9b70cad | 2192c5a |
| ws（评审 5） | webui-sec-ws | d32348f + 14409b7（containment 测试迁 checks） | 85160d2、32cdc60 |
| docs（配套） | webui-sec-docs | 598f0e7 + 72785d8（plugin.json/README 窄补） | 4f978df、2e01819 |

五簇文件领地互斥，cherry-pick 集成零冲突。

## 修复对照（评审五点 → 实现）

1. 通配 CORS × 本地免权组合洞：可信源逐字反射（自身源 + 显式 trustedOrigins 白名单），不可信/无 Origin 零 CORS 头，预检一致，Vary: Origin；新增 Gate 1b（非 GET/HEAD/OPTIONS 且 Origin 在而不可信即 403，先于一切豁免，明确不豁免回环 socket；Origin:null 在而不可信）；lanUrlWithToken 改首次引导一次性面（确认后整字段省略、轮换后重发一次）。
2. 默认绑 0.0.0.0：resolveBindHost（env HOST > 持久 lanBind > 回环 127.0.0.1）；LAN 显式 opt-in，快照披露五字段（lanBind/bindHost/lanExposed/bindRestartPending/lanExposureNotice 双语）；既有显式配置零惊扰。
3. 上传无界：有界流式状态机（内存 O(块)），三限额流中途强制（50MiB 请求/25MiB 单文件/200MiB 配额，MCODE_WEBUI_UPLOAD_* env 覆盖），413 带 code+指引+Connection:close、400 畸形，失败清理无半写；顺带治愈既有伪边界截断缺陷（RFC 2046 §5.1.1 后继字节验证）。
4. 会话删除假成功：错误分类（缺表须同连接 schema 目录确证；no such column: session_id 经 table_info 确证为 unsupported_schema；其余重抛），显式事务回滚，结局枚举 deleted/already_absent/unsupported_schema/db_error + 审计 outcome；dryRun 同分类（预览不再假成功）。
5. 工作区与持久化：allowed roots containment（resolve+realpath 落根内，默认 home+默认工作区+tmp，MCODE_WEBUI_WORKSPACE_ROOTS 设置即全量替换；browse 同界，根视图只列允许根）；sessions 原子写（tmp+rename）+ 单进程同步串行 + 损坏隔离（SESSIONS_DB.corrupted-<ts>，原文件不动）斩断数据消失链。
- 配套：SECURITY-NOTES（单源真值）八项事实对齐；API.md、HTTPS-REVERSE-PROXY.md（新增反代 trustedOrigins 指引）；plugin.json manifest（HOST 缺省、securityNotes、capability、四个新 env 键）与 README 对齐。

## 测试与闸门

- 用例：插件套件 1034 → 1144（净增 110；net +50、upload +29、ws +25+15 迁移计数中性、db +6）；macOS 双面绿（module-mock 面 + 无旗标面 630/630）。
- siinfer（Linux，净树）：根门 737/737（终头 5f375b3）。该门三次抓真问题：mock 依赖误放 test/（迁 checks）、固定端口 bind 竞态（独立端口+等退出）、settings 套件 HOME 隔离债（events/settings 路径 per-suite 隔离，顺带修掉第二处真实 HOME 写入泄漏）。
- fork 预览（windows-latest + CodeQL）：run 35556777288 双绿。
- 上游 #55 检查：validate 绿、其余作业自动收敛中，mergeable 保持。

## 主线验收记录

五簇 diff 全部逐簇过目（错误分类双重确证、Gate 1b 先于豁免、流式限额中途强制、containment 残余窗口诚实申报等要点核过）；集成后权威套件亲跑三轮（85160d2/32cdc60/5f375b3 各一轮）；siinfer 与预览亲跑；PR 推送与回评亲发。

## 偏离与勘正记录

- setReadOnly/rotateToken 审计写入归属勘正：源自 rigor 批 54af814，先于本批头；Linux 门暴露的测试隔离债按测试面最小修，产品码零改动。
- ws 簇损坏处置裁量：损坏仍返回 []（30+ 处领地外调用方无 catch），以隔离副本+原文件不动+可行动错误承载"显式 corrupted"；数据消失链（损坏→空库→覆盖保存）已斩断。
- docs 簇申报既有漂移四条未动（MCODE_WEBUI_TOKEN vs TOKEN、Traefik replacePath 疑误、SECURITY-NOTES §2.2 与 §9 矛盾、API workspace 节未全核），前两条已写入 PR 回评尾部供上游知悉。

## 检索申报

批内检索零命中预期（上游安全加固施工），规格以评审原文为准，未加载治理命题。
