# exscanwire-solo 结果档

> 批：例扫挂点执行与 pk-052 出泊（任务包件一至件四）
> 会话：8040c9fae7c90e0e（sess-zcode-2026-09-04-exscanwire）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理；与 newcarr2-solo 并联（施工面不相交，共享追加面 append 短持，pk-045 冲突模式成员）
> 承接：m-exscanhook-2 九发 stable_clear 已执契机器终签落据（重放锚 sih-tools/proposition/DES/m-exscanhook-2/m-exscanhook-2-signcheck.json，链 sign 笔 crosscheck_completed 5fb04cb6 verdict pass 在 2026-09-04 当日链）；用户 2026-09-04 令「得一裁一过一执行一」
> 意图哈希：1e6bbfcd（intent 事件哈希前八位，ask3 记录内容哈希 0e5ad5f8 至补全版 d6b38922，双门 status ok 三锚）

## 一、问题还原

mathscan-solo 批件五唯一裁决点（数学仓全盘扫描例行化挂点两案）过得一裁 stable_clear 后呈泊 pk-052（入泊事件 42d232ce），用户 2026-09-04 裁定「得一裁一过一执行一」——A 案（挂 gauge 例行读数旁：每日三维快照落链后同跑 rev3 双跑与 checkmath 四表核对）经 m-exscanhook-2 终签落据即执行，泊位出泊。本批承执行位四件：件一 pk-052 出泊（出泊事件上链与名册行出泊态）、件二泊界投影照链补齐（mathscan 批 allow 闸未落主树的 pk-052.json 副本照链补齐）、件三调用形登记（BATCH-FACE 增「例扫日扫调用形（m-exscanhook-2 终签执行）」节，AGENTS.md 零触碰——例行读数条目入宪法另走治理）、件四首跑实录（按件三调用形实跑，读数落 first-run-2026-09-04/）。挂点位置为机制形之确定性后承：调用形登记落既有例行读数程序位，close 关键路径零新增闸。

温故检索：materials/recall-exscanwire.json 如实记（温故面由 m-exscanhook-2 终签材料三件、pk-052 泊材料与链上 park 事件 42d232ce、出泊先例 b68b178a、BATCH-FACE 2026-09-04 勘误节与补遗节全读承载）。

## 二、完成度表

| 件 | 判据 | 结果 | 证据 |
|---|---|---|---|
| 件一 pk-052 出泊 | 出泊材料与出泊事件上链与名册行出泊态附事件哈希 | 完成 | pk-052-exit.json（disposition promoted，ruling 记 A 案经 m-exscanhook-2 终签执行），出泊事件 198f936b 经引擎 scribe park 上链（调用形同 pk-037 出泊 b68b178a 先例），PARKING-v1.md 名册行改历史住户出泊态附哈希（在泊九改八，历史住户十三改十四） |
| 件二 泊界投影照链补齐 | pk-052.json 照链零改写补齐 state/parking/materials/ | 完成 | 工地 sih/state/parking/materials/pk-052.json 自 mathscan-solo-materials/pk-052.json cp 后 cmp identical（零改写），内容以链上 park 事件 42d232ce 为正典（mathscan 结果档申报的照链补齐先例） |
| 件三 调用形登记 | BATCH-FACE 增节命令 verbatim 经首跑实测 | 完成 | BATCH-FACE.md 增「例扫日扫调用形（m-exscanhook-2 终签执行）」节（#exscan-daily），命令即件四实跑形，附判据与坑位注记；AGENTS.md 零触碰 |
| 件四 首跑实录 | rev3 双跑 cmp 与 checkmath 读数落档 | 完成 | first-run-2026-09-04/：run1 与 run2 三件加 stdout 四路 cmp 全 IDENTICAL，checkmath zero_drift 红零灰 7，全部 stdout 转写在档；sih-math 全仓零写入 |

## 三、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| **F-1 出泊三面一致** | 出泊事件在链 verify valid，名册行与 materials 副本与链三面一致 | 过 | 出泊事件 198f936b（parking_exited，entry_id pk-052，disposition promoted）在 2026-09-04 当日链，当日链 verify valid；名册行出泊态附哈希 198f936b；materials 副本（件二）与链上 park 事件 42d232ce 内容一致；三面对表见认证清单节 |
| **F-2 调用形登记可复现** | 登记在 BATCH-FACE 且命令经首跑实测可复现 | 过 | #exscan-daily 节命令与件四实跑逐字同形（$FR 变量形即实跑形），首跑四路 cmp 全 IDENTICAL 即可复现实证 |
| **F-3 首跑读数在档** | 双跑 IDENTICAL 或漂移如实记，checkmath 红灰如实记 | 过 | 双跑四路 cmp 退出码全 0 全 IDENTICAL（零漂移），checkmath verdict zero_drift：reds 0、greys 7（声明滞后灰项与 mathscan-solo 批同款：algebra 与 order 与 probability 概览计数声明、mapping 概览三笔），零红灰如实转述，全部读数落 first-run-2026-09-04/ |
| **F-4 写入仅 allow** | 写入仅 allow 清单，工程源码与 sih-math 零触碰 | 过（附申报） | 写入面逐件对照 allow：engine 五处加 trail 经引擎 scribe，tools 四处（BATCH-FACE 走工地，scribe/reports 与 identity/reports 与 meter/counts 为管线固有面）；worktrees 两处锁取 scope_violation 拒如实记（见越线申报一）；工程仓 src/ 零触碰，sih-math 零写入（rev3 输出走 argv[1] 批 materials 目录，checkmath 走 stdout，git status 两处 M 为批前遗留见越线申报二） |

## 四、首跑读数节（件四）

| 路 | 读数 |
|---|---|
| rev3 双跑退出码 | 0 / 0 |
| cmp ledger-rev3.json | exit 0 IDENTICAL |
| cmp env-params-rev3.json | exit 0 IDENTICAL |
| cmp summary-rev3.md | exit 0 IDENTICAL |
| cmp stdout | exit 0 IDENTICAL |
| rev3 findings | 零笔 |
| 三态 | 已实例化 19 / 可指认未实例化 5 / 无可指认载体 0 |
| 白名单与条目与别名 | 白名单正身 172、磁盘条目 167、别名记账 4、机制 24、F3 双实存 36 |
| checkmath 退出码 | 0 |
| checkmath verdict | zero_drift：reds 0、greys 7（声明滞后：algebra「已建 3」vs 实建 7、order「已建 11」vs 实建 20、probability「已建 7」vs 实建 17、mapping 概览 CALC 113 vs 114、ORD 18 vs 21、PROB 15 vs 17、ALG 5 vs 12） |
| 口径 | mapping 69、INDEX 172、entry 磁盘 167、白名单=INDEX 恒等 |

判定：与 mathscan-solo 批读数逐位对表零漂移（同参同判），例扫挂点首跑绿，如实转述零红。

## 五、管线读数

- 化格：BATCH-FACE.md 与 PARKING-v1.md 与 results.md general-v1 跑数见认证清单前后实录（json 件域外如实记）。
- 核阅：PARKING-v1.md 在 des-001 域内（sih-engine/doc），首跑工地内容入域检 exit 1 findings 3（C006 全角括号论据三对），主会代收段修正三段即字句形后复跑 exit 0 零 findings；results.md 与 materials 件在 event/plan 域外 exit-2 如实记不属违规；BATCH-FACE.md 在 sih-tools 域外 exit-2 如实记；ask3 双门第一门 exit 0 零违规。
- 检词：BATCH-FACE.md 与 PARKING-v1.md 与 results.md core 包跑数见认证清单前后实录。
- 词债：例扫挂点、出泊、照链补齐、调用形、首跑五轻信号未登记，处置不登记让位后批词表批（叩问 digest passed covered 5），词债记本节与偏差申报节。
- 代收申报：原执行代理于件一至件四完成后、认证与收约前因模型请求故障中断（约 07:40 UTC），主会按 pk-045 冲突库「停在收约主会代收」类承接收口段：管线三步、C006 修正、认证三笔、settle 与 close 全部经原会话 8040c9fae7c90e0e 与其锁面完成，施工内容零改写（唯一新增为名册三段 C006 合规修正与结果档定稿），验收入主会亲验。

## 六、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 97e8afe5 / d1179c56 / c0990d7c | 例行读数三维（convergence 0.428571 / adoption 1.0 / mergeback 0.043478） | gauge record ga-2 公式三维快照 |
| 1e6bbfcd | 意图笔（scribe intent） | ask3 记录 d6b38922（补全版） |
| 198f936b | pk-052 出泊事件（parking_exited） | 出泊材料 pk-052-exit.json |
| 60ed688a | 例扫首跑报告（2026-09-04-exscanwire-solo-firstrun.json） | first-run 读数件 sha256 载于报告件 |
| bc15d97e | 管线报告（2026-09-04-exscanwire-solo-pipeline.json） | 化格核阅检词三步读数载于报告件 |
| （本笔随结果档定稿上链） | 结果档报告（2026-09-04-exscanwire-solo-results.json） | 结果档内容 sha256 载于报告件 |

认证一律先落主树活链（meter 包裹引擎 scribe append，闸三 --session 与 --sessions 必带），链 verify 以 close 后读数为收。

## 七、越线与误差申报

1. **worktrees 两处锁 scope_violation 拒**：任务包 F-4 allow 未列 worktrees/sih-tools/exscanwire-solo 与 worktrees/sih-engine/exscanwire-solo，锁取被 scope_violation 拒（如实记不绕行）；两工地由 lease open 自动创建归租约管辖，其余十一锁（exclusive 七加 append 四）全过零撞。
2. **sih-math 主树批前本地改动两件**：algebra/INDEX.md 与 llm-friendly-build/mapping.md 各 1 行改动，mtime 2026-09-04 06:23 UTC 早于本批开工 07:26 UTC，非本批造成（本批对 sih-math 只读实跑），本批零写入 sih-math 如实申报；改动归属归在途批或主会核查，本批不代处置。
3. **sih-math 零写入实现形**：rev3_script.py 三件输出走 argv[1] 指定目录（本批指 first-run-2026-09-04/run1 与 run2），checkmath.py 不带输出参走 stdout——两脚本无 sih-math 写路径，红线以调用形承载并已登记入 BATCH-FACE 坑位注记。
4. **词债五笔**：例扫挂点、出泊、照链补齐、调用形、首跑五轻信号未登记（本批词表零写入面），词面由任务包与结果档与 BATCH-FACE 新节行文承载，登记归后批词表批。
5. **ask3 记录两版**：elicitation 补全前内容哈希 0e5ad5f8（首门首验版）、补全后 d6b38922（intent 上链版），双门对两版均 status ok，两版并存如实申报不以一饰二。

## 八、冲突样本节（pk-045 样本库）

本批与 newcarr2-solo 并联（pk-045 冲突模式成员，任务包申报）。开工实查（2026-09-04 07:15 至 07:27 UTC）：sessions 台账零 newcarr2 记录（对方尚未 lease open），主树 engine 有 newcarr2-solo-materials/ 与 sih/state/plan/newcarr2-solo.md 未跟踪件（任务包预备面，非施工冲突面）；活动会话仅 mainqm-adjud-solo（8b6ca23f35207c8a，07:15:43 issued），其 allow 面（proposition 与 facet/contracts 与 tally/reports 与 scribe/reports 与 identity/reports 与 meter/counts 与 engine trail 与 state/plan）与本批 exclusive 施工面不相交，共享追加面（trail 与 scribe/reports 与 meter/counts）按 append 短持预案各行追加。租约窗口锁取十一笔全过零撞锁；出泊 park 与 intent 与认证笔经闸三会话在册验零冲突。冲突响应预案在案：exclusive 撞锁重试上限十次不绕行、收约撞主树同名未跟踪件走备份让位归并对表法（mathscan-solo 收口先例）。实测撞锁冲突零笔。

## 九、验收

- [x] F-1 至 F-4 全过（F-4 附申报）
- [x] 件一至件四全完成，结果档各点名节在档
- [ ] 认证入链（认证逐笔后回填哈希），双仓 settle（sih-math 零写入零提交属红线达成非缺漏），close 后收口读数见收口附记
- [ ] 队形验证：单线形 solo 成立——本批全部写入由会话 8040c9fae7c90e0e（sess-zcode-2026-09-04-exscanwire）亲写，零 Agent/Task 子代理调用；例扫读数与出泊事件与三面对表全由确定性程序承载（rev3_script.py 与 checkmath.py 与引擎 scribe 与 lease），链写入经引擎 scribe 闸三，零直写链文件

## 收口附记（close 后补记，本节经 close 通道外 bypass 提交回填，ordwire 与 pk037impl 与 mathscan 先例同形）

- 收口形态：原执行代理于件一至件四完成、认证收约前因模型请求故障中断，主会按 pk-045 冲突库「停在收约主会代收」类经原会话 8040c9fae7c90e0e 代收至 close，施工内容零改写，唯一内容新增为名册三段 C006 合规修正与结果档定稿与本附记。
- close 三跑：首跑撞十锁未放（代收段先 unlock 后 close 的次序教训，unlock 缺可读正身件时静默返零如实暴露后补正身件复跑十锁全释）；次跑 tools 腿撞工地未跟踪正身件（unlock 拷入件），补 settle 段三笔后三跑全绿：双仓归并、分支删、工地拆、会话吊销。
- 认证笔：意图 1e6bbfcd、出泊 198f936b、首跑 60ed688a、管线 bc15d97e、结果档 3835a017、例行读数三维 97e8afe5/d1179c56/c0990d7c。
- 链读数：close 后当日链 206 事件 verify valid。
- 三仓主树归并提交：sih-tools 92f5de3c 段三与 926c1caa/c3cae275 归并、sih-engine 98d51b2 段二与 c004dbe 归并；sih-math 零写入零提交（红线达成非缺漏）。
- reconcile：双仓 unrouted 零、session_orphan 零、unbypassed 零；cert_missing tools 2 与 engine 1 为批前存量非本批新增。
- 让位对表：备份二十一件与归并结果逐件 cmp 全数 IDENTICAL；投影副本 pk-052.json 于合入后发现工作树被删（删除者非本批非主会，或为并行在途批动作），自 index 还原后与备份 cmp IDENTICAL，如实申报。
- 本附记经 close 通道外 --no-verify 提交加 lease bypass 登记回填。
- [x] 认证入链，双仓 settle，close 后收口读数见本节
