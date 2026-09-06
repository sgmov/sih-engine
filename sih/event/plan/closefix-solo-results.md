# closefix-solo 结果档：收约归并机械修复（台账盖版与 stash 吞件与回补网三病一役）

> 批：closefix-solo（T6 单线 solo，委外代理亲写零子代理）
> 会话：8f3455d8ba88f8e4（双仓租约，13 独占锁）
> 日期：2026-09-06
> 令源：用户 2026-09-06 令「修复」（承 ledgerloss5-solo 调查批根因呈报与 idenlane-guard-solo 批 stash 吞件发现，泊件 pk-071 出泊承载）+ 任务包 sih-engine/sih/state/plan/closefix-solo.md

## 一句话结论

批以三修一裁收口三病：台账面冻结豁免（LEASE_LEDGER_APPEND_FACES 四面永不整文件盖版只走并集追加通道 _union_append_face + 归并面三点式 merge-base 基准 + 预收提交最小化 add -A 收窄为面∩真脏位）根除盖版、stash 舞步结构性根除（未跟踪件照单登记不清场，全路径成功失败拒零离盘）、ledger_lock 同线程可重入根除回补网自死锁；TDD 十二件先红（现行码 11 红 1 绿）转全绿，ledgerloss5 沙盒四景全数转绿（甲 10 行零丢失、乙死锁解除、丙并集零丢失、H-3 台账面写点 4.026 秒阻塞至释放对照旧形 0.007 秒绕锁），并发活写窗压测 20 行零丢失，全测试族 161 绿即 149 基线零回归零适配；判定语义一裁 m-closefix-closechange-1 九发 stable_clear 机器终签链笔 39b33ff8（非 near_threshold 免呈报）；连带处置双仓历史 stash 二十五笔（closeguard 二十三笔复位 drop 加 WIP 二笔核实超越随批 drop）与数据卫生两清单呈报；本批收约即活体验收走新机械不冻结台账面完成。

## 二、前置机械链实录

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 三问双门 | scrutinator ask3 + ask3repeater | 0 / ok | 三锚 PRO-07 鉴×2 与 PRO-08 应，逐字节子串程序核验，验证件 2026-09-06-ask3-closefix-solo-validation.json |
| 叩问消化 | elicit check + digest | 1（7 轻信号）/ passed 7 | 七词全处置随批 register 通道登记，terms 217→224 |
| 正身 | identity verify | 0 | anomalies 0，identity 1c408342e9，core 82f460c2ac |
| 租约 open | lease open（任务包绝对路径） | 0 | 会话 8f3455d8ba88f8e4，scope_source package，双工地起 |
| 取锁 | lock 13 面独占 | 0 全绿 | lease 源码/测试/CONTRACT/CALL-LOG/pyproject、scribe CALL-LOG、facet 合同目录、DES 单元格、nomenclator terms、BATCH-FACE、任务包、结果档、materials |
| 书简意图 | scribe intent（裸调） | 0 | 325cc760 在链 grep 命中 1，链 verify valid |
| 例行读数 | gauge record | 0 | convergence 0.142857 / adoption 0.8 / mergeback 0.041667 三笔落链 |
| 泊界心跳 | selector route 双目录 | 0/0 | tools 15 件即 14 mainline 加 1 siding（pk-042 P102）零告警，engine 46 件即 40 mainline 加 6 scrap 零告警 |
| 开工净态 | watchcheck | 0 | 净态无主零处（收约前）；stash 复位处置段复跑 9 件无主呈人节点（见第八节） |

申报一：共享追加面建议锁（ledger 与 reports 与 trail 四路 append）被 allow 面范围验拒（任务包请求写入段为描述形字符串不覆盖目录路径），13 独占写面锁全持即载荷位全保，如实记档不代扩 allow。

## 三、TDD 先红后绿与修复实装（T-1/T-2/T-3/T-4/T-8）

### 3.1 红证（现行码即主树 1.26.0 形）

新增 tests/test_closefix.py 十二件对现行码跑出 11 红 1 绿（红档 materials/tdd-red-closefix-tests.log）：沙盒甲景台账面 7 行丢失重放（RED）、乙景回补网自死锁 12 秒探针（RED）、丙景并集通道缺位盖版（RED）、H-3 绕锁（RED）、冻结常量 ImportError（RED）、活体 close 台账丢行（RED）、拒路径吞件（bystander FileNotFoundError）、败路径吞件（同形）、预收提交固化面外脏件、锁重入缺失嵌套死锁、持锁窗内回补死锁；绿一件即跨线程互斥（两边皆守的安全回归项，如实注记）。沙盒四景另经 ledgerloss5 原脚本复跑确认红态在册（materials/sandbox/h4-repro-red.log 与 h1-h3-repro-red.log，甲 7 行丢失、乙死锁、H-3 0.007 秒绕锁对照 append_row 阻塞 2.009 秒）。

### 3.2 实装（工地 sih-tools/closefix-solo）

- core.py：LEASE_LEDGER_APPEND_FACES 冻结常量四面（sessions.ndjson/locks.ndjson/bypass.ndjson/checks 目录）加 _is_ledger_append_face 判定；_merge_files 改三点式（merge-base 基准取 branch 侧改动，解算失败回落保守）；_union_append_face 并集追加通道（单锁窗内重读活面、活面原序原行保持、分支独有行按序经 append_row 补尾）；detect_merge_conflicts 三点面加未跟踪目录前缀命中（目录 vs 树旁证收编）、面内未跟踪同内容形不再跳过（改入 conflicts 由 same+untracked 让位处置）、面外未跟踪尾环废止；allow_and_merge 台账面路由并集通道（其余仍 checkout 让位）；close_session 预收提交段重写即 add 收窄为三点面∩真脏位、stash push -u 与 .git 标记备份还原舞步整体废止、stash pop 位废止；_close_precondition 归并面同改三点式。
- ledgerwrite.py：ledger_lock 同线程同路径可重入（_LOCK_HELD 计数式持有归零解锁，键即线程标识加锁文件绝对路径），跨线程跨进程照旧真互斥（test_ledgerwrite F2 跨线程阻塞回归在役）。
- cli.py 与 lockdb.py：零改动（修复载荷全落 core 与 ledgerwrite）。

### 3.3 转绿读数

- 全测试族 161 绿（149 基线零回归零适配 + test_closefix 12 件），基线对照档主树工地双跑一致。
- 沙盒四景转绿（materials/sandbox/green/，PROD_LEASE_SRC 指工地修复码）：甲 10→10 行零丢失预收提交 10 行 merge 终局 10 行 GREEN=true、乙 self_deadlock_reproduced=false 恢复非空窗完成、丙 rows_lost=0、H-3 checkout_waited_for_lock=true（4.026 秒阻塞至释放）活行 S2 存活对照组 2.01 秒。绿档脚本对原脚本两处机械适配如实注记：甲景备份位三点面下为空即读数容零（无归并冲突即无备份）、GREEN 判据并列原 RED 判据（原判据保持供对表）。
- 并发活写窗压测（materials/stress/concurrent-append-during-close.py）：close 进行期间他进程 append_row 追加 20 行（无前置 sleep 全程交叠），收约毕 stress_rows_present=20 revoked=1 issued=1 final=12/12 行 stash 零遗留 PASS=true。
- 并集窗探针（materials/stress/union-window-probe.log）：分支改台账面（settle 形 2 行）加主树活写 2 行形，并集通道保全全部行（base/live/branch 俱在）后闸四 merge-tree 内容冲突 fail-visible 整批拒零丢失零 stash——旧行为为静默盖版固化丢失，新行为零损失可见拒，如实呈报为边界语义。

## 四、判定语义一裁（T-10，F-5）

- 席位基线：本批 identity（1c408342e9）当日新标定即 temp_probe agent 模式四命题×5 发（safe-1/safe-2 comply、violate 全票判违、knife 边界旗如实），体温 0.0 准确性真零漂移，ledger 尾行提取 seat-baseline.json（R5 身份哈希一致亲核）。
- 出合同：measure.py --emit-contract（席位 ZCode:GLM-5.3-Flash 九发，ng medium 正典 d4f84701 同源）。
- 作答：席位亲写零子代理，九发同判（温度 0）comply 依据 baseline_1 一类。
- 计分：measure.py --score 响应 9 发入 trail 实写 9 空转 0，闸门 stable_clear（9/9 comply 变卦 0 旗 0）。
- 装配与执契：tally assemble 零判断装配，attractor check pass 12 项零失败（R5 席位当日基线判定可用且身份哈希一致），verify identical，裁决通过方向 comply。
- 终签：attractor sign 经引擎 scribe crosscheck 落链，链笔 39b33ff8，非 near_threshold 免呈报转主会条款不触发。
- 谱系披露：起草与作答同席且本席为修复实装者与避坑令退役的直接受益者，对己不利声明在 topic.md 在档（判 comply 即本席失去人肉冻结兜底的介入机会，判 violate 即继续依赖人类注意力补机械缺陷，利益指向 comply 如实披露）。

## 五、数据卫生（T-9，F-6）

材料件 materials/hygiene/data-hygiene-report.json：

- wenguobs 双写行：sessions.ndjson 762 行在档，session 3a1171b19da42244（wenguobs-solo）issued/revoked 成对四组（行 335/336、337/338、339/341、342/343），四笔 issued 字段全同唯 allow 面递增（14→16 项），四笔 revoked reason 各异如实载重开事由（范围闸拦改包重开、lease 1.9.0 重开同 session_id 漏 identity_hash、lockcore active_sessions 重复 pop bug、换新 identity 报告），净语义效果即 revoked 终态对判定与对表零歪曲；成因 2026-08-31 重开舞步遗留（先于修订十八与修订三十七两修）。append-only 台账不改写历史行，双写行留档披露，处置（count 视图去重或保持原样）随用户裁。
- locks 镜像对 lockdb 对表：镜像 5901 笔 acquired/released 为 SQLite 正典 6032 笔的严格子集（镜像独有零笔），现势表 lock_state 13 面对镜像事件序重放 13 面零漂移；db 独有 131 笔（acquired 69 加 released 62）即盖版窗被抹的镜像行，日期分布 2026-09-05 计 109 笔与 2026-09-06 计 22 笔与 ledgerloss5 §一伤亡窗吻合。判定正典在 lockdb 无损即零判定影响，镜像缺行属可见性损失，修不修随用户裁（补镜像可按正典重放追加走 append-only 通道）。

## 六、双仓历史 stash 复位处置（T-2 后半）

任务包立包时在册六笔，处置时点实数双仓二十五笔（后继批 close 继续累积所致，如实申报）：engine 十三笔（closeguard 十二加 WIP 一）与 tools 十二笔（closeguard 十一加 WIP 一）。

处置序（材料件 materials/stash-audit/）：审计先行（逐笔 sha/日期/文件清单/盘面状态对表 JSON 两件加双仓 git bundle 保引用审计件）→ LIFO 复位（stash@{0} 起 newest-first 逐笔对表，缺件自 stash 提取复位，在盘件跳过，跟踪修改面保盘面新者）→ 零缺失复验（双仓全 stash 面内件盘面零缺失）→ drop。

读数：tools 复位 1150 件（含 2026-09-06 13:59 收约窗活吞 1142 件即 scribe/reports 594、proposition/DES 268、identity/reports 117、proposition/topics 51、tally/reports 46、facet/contracts 40 等批产出件）、跳过在盘 11010 件、跟踪面保新 4 处；engine 复位 1 件、跳过 319 件、跟踪面保新 3 处；零错误。closeguard 二十三笔全数 drop；WIP 二笔内容核实已被盘面超越随批 drop（engine WIP 的 trail 三事件俱在现链 grep 命中、tools WIP 的 maturation_gate v3 迁移已由现版承载），bundle 保引用在档。处置毕双仓 stash list 俱空。

复位后 watchcheck 复跑：脏文件 1315 件中无主 9 件呈人节点二值裁决（mtime 俱为复位时刻，即被吞件复位落地件：m-wengui-fit-1 两散件、meter-quiet-probe.txt、nomenclator 临时件、wikirecall/verify_triggers.py、worktrees_placeholder 等，处置建议「我的」走通道或「不是我的」机械回滚，见 BATCH-FACE watch 处置协议节）。

## 七、版本与工件

- 三源对齐 1.27.0（pyproject 与 __init__ 与 CONTRACT 修订四十），机器形态节增收约归并机械段。
- 术语七条随批登记入 core pack（盖版/并集追加/三点式/冻结豁免/可重入/回补网/吞件），terms 217→224，化格归一零改动行，nomenclator 30 测绿。
- BATCH-FACE 增坑位勘误 2026-09-06（closefix-solo 批）节：主会冻结避坑令退役登记（T-11）。
- pk-071 出泊材料 pk-071-exit.json（outcome promoted，修复承载出泊）。

## 八、越线与误差申报

1. allow 面不含共享追加面目录即 append 建议锁四路被范围验拒，如实记档（第二节申报一）。
2. 沙盒绿档脚本两处机械适配（三点面下备份位空读数容零、GREEN 判据并列原 RED 判据）如实注记，原脚本与红档零改动保留对表。
3. 任务包「六笔历史 stash」立包后增至双仓二十五笔，按实数处置如实申报（第六节）。
4. 材料件初落主树后移入 engine 工地（工地铁律自纠，主树副本清除，工地副本随 settle 提交）。
5. 一裁 R5 首跑以 idenlane 基线（身份哈希异）落挂起，即以本批 identity 当日新标定重装配过（席位基线重跑四命题×5 发亲答），非挑选作答即标定与作答同席同日如实披露。
6. stash 复位写入主树未跟踪件即 mandate 载明的复位动作（idenlane-guard 先例同形），复位件不属本批 allow 面（各批产出件），以对表 JSON 留痕不混入本批 settle。
7. 主树 closefix-solo.md 任务包勾选后其工地副本同步（settle 提交），主树未跟踪原件保持——本批收约让位归并对该未跟踪同内容件走新机械 unlink 让位即活体验收的组成部。

## 九、收约读数（活体验收 F-1/F-2 生产实证）

收约前主树台账活面未冻结（冻结避坑令已退役）：sessions.ndjson（762 行含未提交活行）与 locks.ndjson（5903 行）与 bypass.ndjson（158 行）三台账带脏直入收约，tools 未跟踪 915 项加 engine 未跟踪 47 项在盘，双仓 stash 俱空。

自举形：主树在合并前仍载 1.26.0 即收约代码须自工地执行（新机械即工地 1.27.0，--root 与 --ledger 与 --locks 显式指主树正身位，resolve_root 双仓祖先搜上回真根）。

首跑（工地执行）读数：双仓归并俱成（tools merge 1a99d017 与 engine merge c1c12a4）即预收提交最小化兑现（engine 仅固化任务包一件 778af31 即面内未跟踪同内容让位后 checkout 入 index 的机械结果，内容逐字节同零损失）、双工地拆、双分支删、stash 零新增；但 revoked 行未落即进程静默退出零 stdout——根因钉死为 append_event 函数体内延迟 import ledgerwrite 在工地自拆后 sys.modules 未载该模块即 import 机械访问已删除源码树抛 ModuleNotFoundError（未被 cli 例外族捕获即零输出退出）；该延迟 import 形系 pk057fix 既有形态非本批引入，本批自举调用形使其可达，登记 pk-072 承载修复（import 顶置加回归夹具）；影响面即会话暂不吊销加零读数，零数据损失，复跑幂等收敛。

二跑（主树执行即合并后 1.27.0 在役）读数：revoked=true 双仓 already_gone 收敛，revoked 行 15:41:17 落账（762→763 即 issued 1 加 revoked 1），收约凭据 closefix-solo.lease-check.json 转写；收约毕对表：sessions 763 行、locks 5916 行（含本批 13 取 13 放）、bypass 158 行、trail 2026-09-05 137 行 2026-09-06 69 行零损、tools 未跟踪 915 项 engine 未跟踪 46 项（任务包一件经 merge 转跟踪即净减一）零离盘、双仓 stash 仍空——F-1 台账活行零丢失与 F-2 未跟踪件零离盘俱获生产实证。

## 十、F 表自检

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 盖版根除 | 沙盒甲景转绿；并发活写窗 close 主树活行零丢失 | 过 | 甲 10 行零丢失 GREEN=true；压测 20 行全在；活体收约读数第九节 |
| F-2 stash 根除 | 未跟踪件 close 全路径零离盘（三态夹具）；历史 stash 复位 drop 审计件在档 | 过 | 拒/败/成三态夹具绿；二十三笔 closeguard 复位 drop 加 WIP 二笔核实 drop，bundle 与对表 JSON 在档 |
| F-3 回补网 | 乙景自死锁转绿；回补位次前移后丙景零丢失维持 | 过 | 乙 deadlocks=false；丙 rows_lost=0；台账面走并集通道即盖版不可达 |
| F-4 零回归 | 全族 149+ 绿；台账行格式零变更；openhyg 与批 C 语义零动 | 过 | 161 绿即 149 基线零适配；行格式断言测试在役；resolve_package/PID 探针/routed_direct/白名单零触碰 |
| F-5 一裁 | close 行为变更 facet 合同模式 stable_clear 过执契 | 过 | m-closefix-closechange-1，check 12 项 verify identical sign 39b33ff8，非 near_threshold |
| F-6 数据卫生 | 双写与镜像漂移清单呈报在档（处置随用户裁） | 过（候裁） | materials/hygiene/data-hygiene-report.json |

## 十一、待决项清单（呈用户候裁）

1. wenguobs 双写行处置：count 视图去重或保持原样（append-only 不改写，本批只披露）。
2. locks 镜像 131 笔缺行：按 lockdb 正典重放追加补镜像或保持（判定零影响，可见性损失）。
3. 复位后无主 9 件二值裁决（watchcheck 协议：「我的」走通道或「不是我的」机械回滚）。
4. ledgerloss5 §六残余伤亡行销账进度：主会已补录七行（5daac427），四活跃陈旧会话（leaseopt-fixguard 与 idenlane 三件）revoked 行仍缺即 active_sessions 误显，候人节点令。
5. 分支改台账面形收约 fail-visible 拒（并集窗探针）：如后继批遇此形，处置即冻结台账后重跑或人工并账，机制属安全边界非缺陷。

## 十二、关联

- 任务包：sih-engine/sih/state/plan/closefix-solo.md
- 上游调查：ledgerloss5-solo（根因与沙盒四红证）与 idenlane-guard-solo（stash 发现）
- 出泊承载：pk-071（本批交付即出泊 promoted）
- 修复工件：lease 1.27.0 修订四十、guardcore 零动、test_closefix 十二件
- 后继建议：无（三病俱机械根除，冻结避坑令已退役）
