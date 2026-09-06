# leftover-solo 结果档：遗留归位批

> 日期：2026-09-06　会话：3f4c69f5433bd358　租约 lease 1.30.0　范式：T6 单线 solo，委外代理亲写零子代理
> 令源：用户 2026-09-06 转 watchcheck 代理审计报告（逐件归属判定），本批承载归位与查实；姊妹批 declguard-solo（声明守卫批）随后接续
> 任务包：sih-engine/sih/state/plan/leftover-solo.md（绝对路径在册）

## 一、执行概要

八件未进版控面逐件归位三仓版控（工具仓一件、引擎仓六件、数学仓三件，逐件 commit 号见第三节）；billwire 双层嵌套错径测试件先归正（主树错径位移至正位 scribe/reports/）再收编。rootanchor 双工地半拆查实定论落档（第四节），残片七件留档后拆除双目录，worktree list 复核净态。lease-check 全族清点三十七件清单落档，处置权移交 declguard-solo，本批零迁移零清理（第五节）。零代码、零判定语义变更、零一裁，195 基线零回归面天然成立。三仓租约 settle 段1 段2、close、reconcile、verify 全跑，读数见第十节。

## 二、F 锚定逐条判定

| F 锚定 | 判定 | 依据 |
|---|---|---|
| F-1 归位 | 过 | 八件逐件入三仓工地 settle 提交（第三节对表），收约归并后主树对应路径零 untracked（收约后读数第十节与补笔行） |
| F-2 工地查实 | 过 | 定论落档 materials/rootanchor-teardown-evidence.json（证据链六类在案）；残片七件整体留档 materials/rootanchor-worktree-remnants/ 零静默丢弃；双目录拆除；worktree list rootanchor 零注册零目录 |
| F-3 清点移交 | 过 | 全族三十七件清单落档 materials/lease-check-family-inventory.json（仓库归属逐件在列），移交指针明确指向 declguard-solo；本批零迁移 |
| F-4 复扫 | 过（如实） | watchcheck 批前跑读数唯一无主件即本批第八件，归正后复跑净态退出码零；untracked 剩量如实读数见第六节，剩余面构成与任务包预期形偏差已逐类申报 |
| F-5 零代码 | 过 | 本批零源码改动、零 CONTRACT 修订、零判定语义变更；三仓 settle 提交逐件可 diff 复核 |

## 三、八件逐件归位对表（commit 号在档）

| 件 | 所属仓 | 归位后路径 | settle commit |
|---|---|---|---|
| billwire 双层嵌套错径测试件（先归正再收编） | sih-tools | scribe/reports/2026-09-06-ask3-billwire-live-test-record.json | fcb4887ab7dcaf5e6b2c58628a17af5bddaf96f4 |
| kernelmerge-solo-results.md | sih-engine | sih/event/plan/kernelmerge-solo-results.md | d32bcd9669ba83d9f21bc2f09ae8bae64ca83030 |
| kernelmerge-solo-results.json | sih-engine | sih/event/plan/kernelmerge-solo-results.json | 同上 |
| kernelmerge-solo-materials/（九件） | sih-engine | sih/event/plan/kernelmerge-solo-materials/ | 同上 |
| idenlane-solo-results.md | sih-engine | sih/event/plan/idenlane-solo-results.md | 同上 |
| idenlane-envelope-solo-results.md | sih-engine | sih/event/plan/idenlane-envelope-solo-results.md | 同上 |
| docmath 任务包副本 | sih-engine | sih/event/plan/docmath-task-package-2026-09-04.md | 同上 |
| PROB-018 条目 | sih-math | probability/entries/PROB-018-data-processing-inequality-and-lossy-chain.md | 0786e27bc287e4090c7cae3a87477e03d199bf26 |
| PROB-019 条目 | sih-math | probability/entries/PROB-019-gap-growth-dynamics.md | 同上 |
| gvecmath 任务包副本 | sih-math | sih/event/plan/gvecmath-solo.md | 同上 |

归正证据：错径位文件与已认证 billwire ask3 记录（链 record_hash 62218e03）cmp 逐字节 IDENTICAL，归正移动零内容损失；链上零笔引用错径路径，无断链副作用。收编件名保留 live-test 名以留痕 cwd 相对路径拼接错病灶。

## 四、rootanchor 双工地半拆查实定论

证据链六类（全量在 materials/rootanchor-teardown-evidence.json）：

1. 会话台账：两会话俱 revoked 在册（f8c7e4c3536ef8cc 首会话 27 秒；ba7aa2b83fbb90f8 正式批会话 11:02:36 至 11:06:40Z），revoked 行 removed 数组记 worktree_result=removed 两仓、branch_result=deleted。
2. 锁台账：拆除前复核 rootanchor 零 active 锁。
3. worktree 注册：git worktree list 双仓 rootanchor 零注册（本批拆除前后各复核一次一致）。
4. 分支态：msh/rootanchor-solo 双仓 branch -a 零命中，已删。
5. 管理目录：两仓 .git/worktrees 零 rootanchor 残留。
6. 盘面残片：双目录各余部分文件，mtime 2026-09-06T11:07Z 晚于吊销时刻 11:06:40Z；内容仅批产出件（ask3 记录与验证件与结果档与三件材料与 BATCH-FACE 工地副本），非完整检出。

定论：git 侧拆除完整（注册、分支、管理目录三清），台账 revoked 行如实；盘面残片系拆除后写入再创造的孤儿件，非 close 半失败在 git 侧。写入者带工地自举形路径错根特征：rootanchor 批四件材料 recall-topic.md 落主树、三件落工地形路径的分裂实证在案；残片 BATCH-FACE.md 勘误节双份重复为写入期混乱痕迹。深层成因与该批自述病史同类（results 档 74 与 146 行：worktree prune 与 close 收约同步竞态致副本蒸发），叠加同窗 kernelmerge 与 billwire 并发批作业。主树 BATCH-FACE.md 勘误节单份无污染，主树结果档与材料均较残片为新版，残片零独有内容。

处置：残片七件拆前整体留档本批 materials，双目录拆除，worktree list 复核净态。改善建议归后续批不越本批范畴（鉴只列事实）。

## 五、lease-check 全族清点（移交 declguard-solo）

全工作区 rglob 扫描（排除 sihankor 旧仓与 worktrees 工地副本）：三十七件，其中 sih-engine/sih/state/plan/ 三十六件、sih-math/sih/event/plan/ 一件。批前审计盘点三十六件，扫描时点 redkeep-solo 批刚收约新增 redkeep-solo.lease-check.json 一件，即时在册即全族如实计数。逐件路径与仓库归属与字节数在 materials/lease-check-family-inventory.json。处置权按协同条款移交 declguard-solo（收据归家：写位迁移 lease/ledger/receipts/ 单一目录加白名单登记加存量迁入），本批零迁移零清理。

## 六、untracked 面复扫读数（如实）

批前（会话启动时点）主树 untracked 计数：引擎仓 69 件（含 lease-check 族三十五件与未跟踪任务包二十三件与结果档材料等）、工具仓大宗俱豁免面活写件（scribe/reports 与 identity/reports 与 proposition 与 tally/reports 等；批首观测用截断视图误记 61 件，收约后实测同量级现势 957 件，特此更正申报）、数学仓 5 件（本批归位三件加 lease-check 一件加 __pycache__ 一件）。

本批归位并收约后消账：引擎仓 6 路径、数学仓 3 路径、工具仓错径位 1 路径（归正后新位收编）。剩余面构成如实申报：lease-check 族三十七件（候 declguard 承接）、在途批面（redkeep 已收约、sddpacks-solo 在途其面零触碰）、豁免面活写件（identity/reports 与 scribe/reports 与 facet/contracts 与 elicit/signals 等，constants.py 十六面在册）、未跟踪任务包二十一件（非本批八件清单内，审计未指派，候归档批次裁）与引擎 trail 2026-09-05 未跟踪一件（事件流只追加面，候后继批随批收编）与数学仓 __pycache__ 一件（构建残渣候清理令）。以上俱如实呈报不代清，收约后终读数见 scribe CALL-LOG 补笔行。

## 七、例行读数与泊界心跳

- 例行读数 gauge record 三维落链：convergence 0.375、adoption 0.777778、mergeback 0.038462（ga-2，event_hash 3dec5725 与 e8d6cc16 与 855278d2）。
- 泊界心跳双线零告警退出码零：工具线二十二件（21 mainline + 1 siding）、引擎线五十二件（46 mainline + 6 scrap_track）。

## 八、机械链实录

ask3 双门过（scrutinator ask3 包退出码零；ask3repeater status ok 三锚，引文 08-on-settle.md L13 与 L110 与 07-on-assay.md L55 程序切片逐字节）；叩问五词信号五条轻级未注册，digest passed covered 5，五词俱普通词面描述性使用不立名不登记；正身零异常 status attest identity_hash 5a983dff；三仓租约 open 双门预检零撞，十六笔锁一次取齐（append 三笔与 redkeep-solo 共存形、exclusive 十三笔）；书简意图一笔 event_hash 68cfb8f8；认证四笔裸调逐笔 grep 验证（6fc738db 与 cb682c5c 与 10f08319 与 e11956e4，session_id 绑定五行在链）。

## 九、偏差与越线申报

1. 双仓变三仓：任务包验收标准写双仓 settle，但 F-1 判据为所属仓 commit 号在档，八件中数学仓占三件，故租约按三仓先例（docmath 各批与 mathprobe 与 mathreg 三仓形）开立，三仓 settle 加 close 加 reconcile 加 verify 全跑。
2. scribe/CALL-LOG.md 一笔未随批：redkeep-solo 批（先于本批开工）持该路径 exclusive 锁，按 viewline 与 idenlane-envelope 与 ledgerloss5 先例形移出本批 allow 面，收约后走 bypass 通道补笔。
3. redkeep-solo.lease-check.json 即时新增：清点时点在途批收约产件，全族计数如实采三十七。
4. 第六节工具仓批前计数误记更正：批首以截断视图（head 40 行）误记 61 件，收约补笔时实测工具仓 untracked 同量级现势 957 件俱豁免面活写件，本补笔行更正并如实申报（先红留痕纪律同精神：误记不清洗，更正在档）。

## 十、收口读数（收约补笔回填）

- 收约：close 退出码零，revoked true，三仓归并零失败，worktree 三仓自拆、分支三仓自删；close 收据落 sih-engine/sih/state/plan/leftover-solo.lease-check.json（全族第 38 件，随 declguard 承接迁移）。
- 归并提交：sih-tools a1d1d800d3821a806de3695da188c3429486f942、sih-engine 1564de150da30b99079aa54550fc1441034ed205、sih-math 1fe621f39871b0f7afccf96f541ecf02e5c5de24。
- 八件终态：三仓主树对应路径已全部 tracked、零 untracked（tools 1 路径、engine 6 路径、math 3 路径逐路径 git ls-files 核实在案）。
- reconcile：三仓 unrouted 全零；cert_missing tools 1 与 math 2 俱批前既有（docmath-b4 时代同值在案），本批零新增；本批零 bypass 笔。
- 链 verify：主树正典二进制读数 status valid，158 事件。
- watchcheck 终读数：净态退出码零，无主修改零处（批前唯一无主件即本批第八件，归正即消）。
- untracked 终计数（如实）：工具仓 957、引擎仓 69、数学仓 2，俱豁免面活写件与 lease-check 族与未跟踪任务包（第六节逐类构成在案）；本批八件路径零残留。

本档段2 settle 时点收口前读数：段1 三仓提交 fcb4887a 与 d32bcd9 与 0786e27，段2 引擎 48fa368 与工具 e06e1fac，工作树洁净。

## 十一、关联文件

- 任务包：sih-engine/sih/state/plan/leftover-solo.md
- 姊妹批：declguard-solo（差集闸与收据归家，承接本批 lease-check 全族清点）
- 材料：sih-engine/sih/event/plan/leftover-solo-materials/（管线报告、rootanchor 拆除证据档、残片留档七件、lease-check 全族清单）
- CALL-LOG 双笔：sih-tools/lease/CALL-LOG.md（随批段2）、sih-tools/scribe/CALL-LOG.md（收约补笔）
