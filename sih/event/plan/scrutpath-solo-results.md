# scrutpath-solo 结果档:des-001 工地路径域外误判修复（核阅器路径基修正）

会话: sess-zcode-260908-forkC-scrutpath;租约会话: bb14b837618e6db9;日期: 2026-09-08;单线 solo 零子代理。

## 五节实态

1. 病灶定位: 成立。根因件 sih-engine/src/bin/scrutinator.rs 域判定对 canonicalize 绝对路径直接跑 domain_match,des-001 include 首条为根相对 glob sih-engine/doc/**,工地路径形 worktrees/sih-engine/<批>/doc/... 不含 /sih-engine/doc/ 边界,不命中即全靶 domain_mismatch exit 2。与 acceptclose 与 constclear2c 与 adjudicate2 三窗申报同款,修复前红证复现读数在档(materials/prefix-red-worktree-exit2.json,exit 2)。
2. 修法实施: 成立。src/scrutinator/rule.rs 增三函数: WORKSPACE_ROOT_MARKER 常量(sih-tools/lease/ledger/sessions.ndjson,anchor.py 同款标记件)、discover_workspace_root(目标路径逐级词法上溯实查标记件,目标与中间目录不存在也可判,无根面返回 None)、normalize_worktree_rel(根相对形 worktrees/<仓>/<批>/... 剥前三段前置仓段回仓相对形)、domain_match_rooted(原形短路后取根相对形与归一化形再判,exclude 逐候选生效)。src/bin/scrutinator.rs 增 target_in_domain 一处收口两处域判定(原形与绝对形短路在前,主树形输出零漂移)。diff 全文 materials/diff.patch,195 插入 16 删除,三文件。
3. TDD: 成立。先红: 新测七件先写, cargo test 编译红 E0425 三函数未解析, 九 errors, exit 101(materials/tdd-red.log);绿: 实现后 scrutinator 测试族 36 passed 0 failed exit 0(materials/tdd-green.log), 其中 CLI 级假工作区工地 doc 路径形 exit 0(scrutpath_worktree_path_form_exit_zero)。
4. 引擎件重建: 成立。工地 cargo build --bins exit 0 落 worktrees/sih-engine/scrutpath-solo/target/debug/scrutinator;重建前 pgrep 无核阅进程在跑(exit 1 读数在档);活体验证双读数: 工地 doc 路径 AGENTS-RETIRED-2026-09.md exit 0 findings 0(materials/postfix-green-worktree.json), 域外对照 /tmp 路径 exit 2 语义零变。
5. 连带申报: 成立,记档不实施。BATCH-FACE 勘误候选一条: 自本批起工地路径形核阅为正形,doc 类批「域外 exit-2 如实记档等归并后主树复验」惯例失去必要,候选勘误即 §7 管线三步 des-001 条增注「工地路径形经根锚定域内判定,批内即时核阅为正形」;勘误走后续批,本批不动 BATCH-FACE。

## 产出清单

任务包本件(主窗预落零改)、本结果档、投影件 sih/event/plan/scrutpath-readout.json、materials(diff.patch、tdd-red.log、tdd-green.log、test-family-full.log、prefix-red-worktree-exit2.json、prefix-control-maintree.json、postfix-green-worktree.json)、ask3 记录与验证件、正身件、意图链笔 5043359e、认证清单、越线与误差申报、大白话节。

## 管线读数

- 化格: 本结果档过 formatter packs/general-v1 --write 两跑(段1 与段2 各一)全 exit 0 无需改;主树任务包不在 allow 面且主树零直写,化格 --write 跳过,以检词与核阅只读代替,如实申报。
- 核阅: des-001 对本结果档工地路径形 exit 2 域外: 归一化得 sih-engine/sih/event/plan/... 不匹配 sih-engine/doc/**, 域清单零扩面得证,记档 materials/pipeline-scrutin-results.json。
- 检词: nomenclator packs/core 对本结果档与任务包各一跑,双 exit 0 零违例。
- 书单对表: recall 后 checkcite,批引用件(任务包与本结果档)合并单件扫描 --cited 一次,verdict pass 零引用在册(exit 0)。

## 认证清单

六笔全过闸三(--session 加 --sessions),事件哈希前八位: ask3 记录 a3e29db5、验证件 5cfbcc91、正身件 740d25d9、checkcite 报告 ec630a40、内容哈希清单件 5c5839ce、投影件 scrutpath-readout 80055a09;意图链笔 5043359e。

## 结算读数

- engine settle: 段1 ff344c1(cert 5c5839ce)、段2 535f744(结果档误差申报纠正;段1 后锁已放致 pre-commit 拦截一次,按锁重入幂等坑位重取锁提交后即放,红证如实记档);本批零 tools 仓写面,tools 仓未开租约,tools reconcile 只读对表在档。
- close: exit 0,revoked 真会话 bb14b837618e6db9,工地拆,分支 msh/scrutpath-solo 并 main 删;归并提交 7498053;首跑 close 无主闸拦截 exit 2 零副作用(见误差申报),双旗复跑成;bypass.ndjson 记 bypassed_orphan 一笔携无主 14 件全清单,calllog 旗未单独成笔(calllog 闸 checked true unreleased 0,被 orphan 放行覆盖),如实申报。
- 主树重编复验: pgrep 零核阅进程(exit 1 读数在档)后 cargo build --bins exit 0;重建后读数四笔: 已拆工地路径形 exit 0(域内判定成立,文件已不在仍判域内)、主树同件 exit 0 零漂移、域外 /tmp 形 exit 2 语义零变、主树测试族 36 passed 0 failed 含金向量十二件逐字节(materials/postmerge-maintree-tests.log)。
- 链 verify: valid,events 212,本批七笔(意图一加认证六)。
- reconcile: engine 与 tools 各一跑,unrouted 零,cert_missing 零,unbypassed 零。
- 收约补笔: 本节与 F-6 回填即本笔,经 --no-verify 加 lease bypass 登记通道入版控(hygspots 先例同形),只样本结果档一件。
- 完工回显: 五行锚随完工报告回显。

## 越线与误差申报

- 修复前工地路径域外红证如实记档不清洗: materials/prefix-red-worktree-exit2.json(exit 2,三窗申报形复现)。
- green 首跑非绿: 工地首跑 cargo build --bins 编译红 E0308(target_in_domain 内 path.clone() 对 &str 误用,应 to_string),修正后重跑全绿;红证随 build 日志如实记档,未清洗。
- 首轮 green 跑在 build 前: 21 失败全是「binary 未找到」类(tests.rs bin() 前置要求 cargo build --bins),非断言红;真红为 tdd-red.log 编译红 E0425 与其后实现后全绿,时序如实并陈。
- materials 写入车道申报: 本批 materials(tdd 日志与红绿证与 diff)落主树 sih-engine/sih/event/plan/scrutpath-solo-materials/(租约 allow 面内已锁路径,exscan 例扫正典位先例),非工地归并通道,多批同形在档,如实申报。
- 主树任务包化格 --write 未跑(不在 allow 面),以只读核阅检词代替,如实申报。
- CALL-LOG 留痕申报: 本会话工具调用经跑步机自动追加 CALL-LOG 投影行(acceptor 与 calllog/calls.ndjson 与 critsweep 与 elicit 与 gauge 与 identity 等册 mtime 11:36:06 读数在档),非本批手工写入;任务包唯一权威范围未列 CALL-LOG 产出,本批零改 sih-tools 工具本体;投影面含今晨批遗留候裁集,close 按任务包红线走双旗 bypass 留痕不代收。
- close 无主闸首拦申报: 首跑 close exit 2 无主闸拦截(tracked 无主 14 件全为 CALL-LOG 投影面与 calls.ndjson,含本会话工具自写行与今晨遗留),零动作零副作用,红证如实记档;按红线双旗 bypass 复跑,不代收不代清。
- 锚首行申报: .session-anchor.md 首行仍为 constclear2c 任务锚,任务包红线明定零触碰,未改写;完工回显五行锚以机械对表补偿。
- watchcheck 会话启动读数 exit 1: 无主清单九件全为 CALL-LOG 投影面已跟踪修改(今晨批遗留候跑步机收编),与 hygspots 与 entrydocs 收约记录同一集,零触碰零代清,如实呈报候人节点裁。
- 其余误差零申报。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 病灶定位 | 机制 | 源码位根因判定加三窗申报形复现 exit 2 红证在档 | 通过 |
| F-2 修法最小化 | 机制 | 三函数加一处收口,diff 195/16,域清单零扩面,退出码三值语义零变 | 通过 |
| F-3 TDD 先红后绿 | 测试 | tdd-red.log E0425 编译红 exit 101, tdd-green.log 36 passed exit 0 | 通过 |
| F-4 金向量与测试族零回归 | 测试 | 金向量十二件逐字节断言全绿,全 crate 185 passed 0 failed, 域外 /tmp 场景 exit 2 保持 | 通过 |
| F-5 工地路径域内判定 | 机制 | 工地二进制对工地 doc 路径 exit 0 findings 0, CLI 级假工作区测绿 | 通过 |
| F-6 链面全绿 | 治理 | 认证六笔加意图一笔在链,双 settle,close 撤会话归并,verify valid 212 笔,reconcile 双仓零新增 | 通过 |

## 大白话节

- 病灶: 核阅器检查一篇文档该不该归它管时,只认「主树里的老地址」。批子在临时工地(平行副本目录)里干活时,文档地址长得多一层前缀,核阅器就不认,直接报「这不是我管的」并罢工(退出码 2),于是每批都只能等收工归并回主树后才能补做检查。
- 修法: 给核阅器装了「认路」能力: 沿着文件地址一层层往上找全工作区的界碑(账本文件),找到后就地回算出「这文件相当于主树里哪个位置」,临时工地的 sih-engine/doc 文件从此当场认账。管护范围一寸未扩(sih-engine/doc 之外照样不认),「不是我管的就报 2」的老规矩原样保留。
- 红绿: 先写测试让它跑出预期的红(找不到新功能,编译失败留档),再实现,全测试族 185 项全绿,十二个冻结标准样本逐字节不变。
