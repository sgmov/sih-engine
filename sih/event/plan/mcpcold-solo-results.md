# mcpcold-solo 结果档：mcpline α 相冷 agent 验收与结算批

> 承接：任务包 mcpcold-solo.md（第三节程序形五件）与线程序包 mcpline-line-v1.md（验收判据源头）与对表正典 SPEC-023；派单令源用户 2026-09-09「你拉起多子代理，进行司衡引擎的mcp开发任务」线内批三。
> 队形单线 solo 委外执行，批日 2026-09-09。会话号 sess-zcode-260909-mcpcold（lease 会话 fd958af0c311a10a，双仓 worktrees msh/mcpcold-solo）。冷 agent 会话 sess_b8dccf1f-1130-46db-bac4-b1f8288cc5e0。

## 意图锚定

- 意图事件：intent_refined，event_hash 前 8 `8d1e2340`（event_id cced38db-3193-4626-99d6-6874a6e2a5e4，闸三带 --session 与 --sessions 首跑即过）
- 双门：核阅 ask3 包 exit 0 零违规 findings 0（content_hash 6fd30e2a）；ask3repeater exit 0 status ok anchor_count 3；三锚引文程序切片自 sih-philosophy 原文（07-on-assay.md:55 与 01-ontology-of-names.md:18 与 08-on-settle.md:110），逐字节子串内嵌，生成器 make_ask3_mcpcold-solo.py 随报告目录提交
- 叩问：八轻信号 8/8（MCP 面、冷 agent、回滚法、对表单、工作区配置、注册形、结算件、零辅助）digest passed covered 8，处置八行俱「不立名不登记」描述性使用
- 正身：identity verify anomalies 0，identity.hash 前 8 `be1cb828`（core_hash e1c9a7fa）
- ask3 记录 sha256：`6fd30e2a890630a9e9bd8402f5d5e9e13b678a8fd88254321cfdfaa5a816304f`（核阅 content_hashes 与租约 issued 事件两处一致）

## 前置与开工读数

- 直提守卫：双仓 core.hooksPath 俱指 sih-tools/lease/hooks 在位
- 批前链基线：verify valid 73 笔（首 25989caf 末 bf5dce59），与派单基线一致
- 锁面开工：开工前零锁零在途；open 后十路径锁全取零失败（.zcode/config.json 与 plan 两件与 event/plan 三件与 trail 2026-09-09 与报告两目录与 DES m-mcpcold-1）
- 会话启动节律：回锚与判据扫与双线泊界心跳由本批执行代理跑（俱只读零写入：degraded false 五判据俱达成、双线心跳零告警、hooksPath 在位）；gauge record 落链读数归主窗节律位本批零代跑如实申报（mcpserv-solo 先例同形）

## 施工读数（链步六）

### 注册腿

- 工作区 `.zcode/config.json` mcp.servers 净增 sih 条目：stdio 形，command /opt/homebrew/bin/uv 绝对路径，args 指 sih-tools/mcpline 入口 python -m mcpline，env 显式 SIH_ROOT，timeoutMs 60000
- 只增不改程序实证：before/after 双影像比对 sihankor 条目 identical True、hooks 节 identical True、added {sih}、removed 空；前后 sha256 15c2f425 与 6f2b7e5a 在档
- 装载实证：冷 agent 进程启动日志 mcpServerCount 2（sih enabled 在册）；生效面暖冒烟与冷跑俱可调五工具

### 冷 agent 腿（verdict：pass）

- 起跑形：zcode CLI 0.16.5 无头 print 形新会话，HOME 暂存重定承载模型配置（真实用户配置与治理面零触碰，apiKey 零入档），工作区根为 cwd，--disallowed-tools 二十具禁用清单承载零写授权（冷会话 toolNames 实证：写与文件读写类工具全缺席，只余五 sih MCP 工具加会话编排类工具）
- prompt：只有任务句「用 sih 工具查一下今天的治理链并验证它完整，告诉我结论」加可用事实一句；零用法示例零参数表零路径指引零批名（红线一）
- 断言：单轮自主完成（起 19:21:00 讫 19:21:22 UTC，两笔模型请求），工具调用恰两笔——mcp__sih__chain_query{date:2026-09-09} 与 mcp__sih__chain_verify{date:2026-09-09}，入参全凭工具自描述与 schema 自选；判词 valid true，首 25989caf 末 8d1e2340 与主窗独立 scribe verify 复证逐字节同读数；零求助（零追问零错误重试），零越界写（双仓 git status 前后逐字节全等，链 74 笔不变，配置哈希不变）
- 链路径字样出现在冷 agent 终答系 chain_query 出参 trail 字段教学（工具自描述面），非 prompt 教学，如实申明
- 工作区 AGENTS.md 系 ZCode 环境自动投影非 prompt 输入；冷 agent 实际零消费该面（工具调用实录为证），完全零语境形态归 pk-079

### 对表腿

- SPEC-023 逐项对表（工具名、入参、出参、错误四字段、红线四条）：零实质差异；两条申报即 chain_query 出参信封超集与 sweep-latest.json 开发期遗留跟踪件（时点 2026-09-08 21:32 与 git 跟踪态实证，本批前后零变化）。全档 spec-023-contract-compare.md

## 管线读数（链步七）

- 结果档与结算件与 prompt 件与对表单：化格与核阅与检词三步照跑，读数见认证清单表下管线行
- 书单对表：本批产出引用零推导档引用 ID（纯工程验收批，零数学书单引用），10.5 节 checkcite 无对表对象，如实申报

## 认证实录（链步八，逐件经引擎 scribe append，meter 包裹，闸三带 --session 与 --sessions）

| 件 | 认证哈希前 8 |
|---|---|
| ask3 记录 | 2c5c2ff9 |
| 验证件 | 30b5631b |
| 正身件 | 163a7a5f |
| manifest 内容哈希清单件 rev1 | 91ac9b2d（settle 段一 cert 同取此值；被 rev2 取代如实申报） |
| 投影件 mcpcold-solo.json | 7b9cdd65 |
| manifest rev2 | d1b200ba（settle 段二 cert 同取此值） |

md 件直证承先例走内容哈希清单件（manifest 覆盖任务包定稿件与 prompt 件与结果档与结算件与对表单五件，定格 settle 前实值；收约补笔后哈希漂移如实申报）。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 注册只增不改 | 工程 | sih 条目增补一处，退役 sihankor 与 hooks 原样 | 通过（双影像程序比对四断言全过，sha256 前后在档） |
| F-2 冷 agent 零辅助 | 验收 | prompt 零教学零语境，工具自描述唯一教学面 | 通过（prompt 正典照录与 transcript 双证，任务句加可用事实外零内容） |
| F-3 MCP 面被治理交互 | 验收 | 自主调 chain_query 与 chain_verify 成功且判词 valid | 通过（两笔调用入参自选，valid 判词，主窗独立 verify 同读数复证） |
| F-4 零越界写 | 治理 | 冷 agent 会话零写授权零写事实 | 通过（进程级禁用清单加双仓 status 前后全等加链笔数不变三重证） |
| F-5 契约对表 | 工程 | SPEC-023 五工具与错误四字段与红线逐项 | 通过（零实质差异，超集与遗留两条如实申明） |
| F-6 结算件落档 | 治理 | mcpline-alpha-settlement-v1.md 落 event/plan | 通过（本批随档提交，pk-070 gate 触发条件达成候裁） |

## 越线与误差申报

- 暂存 HOME 机制：裸 CLI 无头路径要求用户级配置带 model 键，真实用户配置不在本批写入面，故经 HOME 环境变量重定 /tmp 暂存存储承载模型选择；真实用户配置与工作区治理面零触碰，apiKey 零入档。此系起跑基建非冷 agent 变量，冷跑两笔请求的模型引用与装载实证在 cold-launch-record.json
- close 首跑红证：真分叉冲突整批拒（主树主窗预落派单件 mcpcold-solo-prompt.md 与工地域内本代理所写冷 prompt 记录件同路径相叉）；备份让位归并对表法处置——本代理版本字节备份 cold-prompt-record.md，工地域内件让位换入派单正典字节，冷 prompt 证明职能由 cold-launch-record.json 承载；首跑退出码经管道 tail 误捕一次（禁管道掩码违例）如实留痕 close-firstrun-red.json；manifest 因本件与 prompt 件哈希变动重定格并补认证笔，前笔 manifest 认证被后笔取代如实申报
- CLI 帮助面漂移：--allowed-tools 与 --max-turns 与 --settings 三旗标帮助面宣示而解析器拒识（bundle 零命中实测）；零写授权改由 --disallowed-tools 单旗标承载，禁用清单与 toolNames 缺席实证在档。漂移记档候工具线处置，不在本批修
- 叩问 check 先于 ask3 记录定稿一跑（预登记 8/8，闸门位序不变，mcpserv 先例同形）
- m-mcpcold-1 DES 路径 allow 预占零写入：本批无得一测量腿，照录不增删，预占位如实申报零写入
- 会话启动四件里 gauge record 落链读数未由本批代跑（归主窗节律位），回锚判据扫泊界心跳只读三件已跑如实申报
- sweep-latest.json 遗留件与本批零涉，如实申明候人节点处置
- 其余误差零申报（闸三首跑即过；十锁零冲突；五读数与冷跑俱首跑绿）

## 大白话节

- **本批做了什么（说人话）**：上批把「只读查询窗口」（MCP 服务器）造好了，这批验证一个真外人能不能只用这个窗口办成一次正经事。做法是：把窗口挂到工作区配置上（只加一条，旧条目一根手指头没动），然后新起一个对司衡一无所知的 agent，只丢给它一句话「用 sih 工具查一下今天的治理链并验证它完整，告诉我结论」。它自己找到了两个工具，先查账本（74 笔），再验账本（判词：完整有效），给出结论，全程没喊一句救命，也没碰任何不该碰的东西。
- **怎么证明它没被偷偷帮忙（说人话）**：三道证。一，给它的话原文在档，除任务句和「有工具面」一句外什么都没有——没有工具名、没有参数、没有文件路径；二，它的对话全录音在档，从头到尾只有它自己调工具和说话；三，主窗自己拿验钞机（scribe verify）独立验了一遍，和它报的读数一个字不差。
- **怎么证明它没乱写（说人话）**：它上手之前就把写字的手绑住了（进程级禁掉所有写文件和执行命令的工具，工具清单里压根不存在）；跑完前后给两个代码仓各拍一张照，照片逐字节一样；账本笔数没变，配置文件指纹没变。
- **本批不做什么（说人话）**：不改服务器一行代码；不做写操作面（β 相要等安全模型设计批）；不裁工程基线搬家的事（pk-070，本结算件落档只是把它点着了，裁决在人）。

## 投影件路径

- 机器可读投影：sih-engine/sih/event/plan/mcpcold-solo-materials/mcpcold-solo.json
- manifest 内容哈希清单件：sih-engine/sih/event/plan/mcpcold-solo-materials/manifest-sha256.json
- 冷 agent 证据族：cold-transcript-extract.json 与 cold-rollout-model-io.jsonl 与 cold-run-cli-response.json 与 cold-launch-record.json
- 五工具读数：stdio-five-tool-readings.json（生成器 make_stdio_readings.py 同目录）
- 注册证据族：registration-and-rollback.md 与 zcode-config-before-image.json 与 zcode-config-after-image.json
- 零写入证明族：gitstatus-engine-before/after.txt 与 gitstatus-tools-before/after.txt 与 verify-prebatch.json 与 verify-postcold.json
- 对表单：spec-023-contract-compare.md
- ask3 生成器：sih-tools/scribe/reports/make_ask3_mcpcold-solo.py（随批提交）
- 结算件：sih-engine/sih/event/plan/mcpline-alpha-settlement-v1.md

## 结算读数（收约补笔）

- 双仓 settle：tools 段一 `cd9495f6`（base integral-stage-build，批机械链侧写六件）、engine 段一 `b865ef2`（base main@200a535，注册证据与冷跑证据与五读数与对表单与结算件）与段二 `bb6cbf5`（cert d1b200ba，close 首跑真分叉红证处置）；closeguard 预收 tools `5f5ca21c`（报告五件）与 engine `36ae44d`（主窗预落派单件）；归并 tools `002f7563`、engine `adcdf6c`。
- 放锁收约：close 三跑形即首跑真分叉拒（prompt 件相叉，红证 close-firstrun-red.json）、二跑无主闸拦 attnanchor 与 attractor 两册 CALL-LOG 跨批存量脏件（mtime 2026-09-08T17:32 先于本批）、三跑带双旗 bypass-orphan 与 bypass-calllog 显式留痕成（bypass 台账两笔）；十路径锁 unlock 俱过、中途复锁三路径补段后复放；双工地拆、双分支删，收据落 sih-tools/lease/ledger/receipts/mcpcold-solo.json。
- 链 verify：valid，events 80（first `25989caf`、last `d1b200ba` 即 manifest rev2 认证末笔）；本批链笔序即意图 `8d1e2340` 加认证六笔（ask3 `2c5c2ff9`、验证件 `30b5631b`、正身 `163a7a5f`、manifest rev1 `91ac9b2d`、投影件 `7b9cdd65`、manifest rev2 `d1b200ba`）。
- reconcile：双仓 unrouted 俱零、cert_missing 俱零即判据双零；unbypassed tools 120 与 engine 84（前批基线 119 与 83 各 +1，增量归本批 closeguard 预收与归并机械自生提交）；session_orphan tools 19 与 engine 24 与前批基线同值；跨批存量不代清。
- 主树零直写申明：工作区 `.zcode/config.json` 一处增补系任务包 allow 面内写入；本补笔前双仓主树零直写（一切待提交件经工地与链）。
- 补笔申报：本节与认证清单实值回填即本笔，经直改链笔申报（scribe direct）加 --no-verify 提交替 lease bypass 登记通道（pkexits2 与 settlement-v2 与 mcpopen-solo 与 mcpserv-solo 先例同形）；任务包 mcpcold-solo.md（主窗预落，closeguard 本批未扫）随本补笔提交版控；补笔后复跑化格 check 与检词俱绿。
