# idenlane-guard-solo 结果档

> 批:idenlane-guard-solo(守卫直改车道收尾与判定,idenlane 拆三批之批 C)
> 会话:337307effbb94e41(双仓租约,16 路 allow,15 锁)
> 日期:2026-09-06
> 队形:单线形 solo,委外代理亲写零子代理
> 令源:用户 2026-09-05 16:42 拆分裁定 + 2026-09-05 17:53 勘误令(主树已合 2 文件起步不重做) + 2026-09-06 扩面令(T-8/T-9/T-10) + 主会冻结活写面避坑令(收约前冻结台账 git 面)

## 一句话结论

批 C 以三件一裁收口:守卫直改车道夹具收尾(T-1 至 T-4 四证加纯函数位双证)、reconcile 路由认账(T-8 新类 routed_direct,令源即主会四笔 bypass 噪声)、轻车道白名单冻结常量(T-9 十条目入 CONTRACT 修订三十九,声明位零执法如实注记);判定语义三裁经 facet 合同模式九发 stable_clear(gid m-idenlane-okform-1,机器终签链笔 8e97cadc,非 near_threshold 免呈报);T-10 锁门两案材料呈用户候裁零实装,含一处任务包表述勘误(364f2919 系 doc_id 非事件哈希且该笔不在独占窗内,独占窗内实证为 d3db0787 一笔);全测试族 149 绿(基线 139 零回归),双仓 settle close reconcile verify 全链收约。

## 二、前置机械链实录

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 三问双门 | scrutinator ask3 + ask3repeater | 0 / ok | 三锚 PRO-07 鉴×2 与 PRO-08 应,逐字节子串程序核验,验证件 ask3-idenlane-guard-solo-validation.json |
| 叩问消化 | elicit check + digest | 1(8 轻信号)/0(passed 8) | 八词全处置随批登记,terms 209→217 |
| 正身 | identity verify | 0 | mode attest anomalies 0,ih 5068160a9807,core 82f460c2ac27 |
| 候位 | gvec-v2-serial(会话 c41ce2af)01:24:39 取 11 独占锁,01:54:49Z 收约让位 | — | 预会话三件先行零冲突,让位后即 open,未撞 basisunion(其已收约 69430329) |
| 租约 open | lease open(任务包绝对路径) | 0 | 会话 337307effbb94e41,scope_source package,双工地起 |
| 取锁 | lock 15 面 | 0 全绿 | 独占 11(lease 源码/测试/CONTRACT/CALL-LOG、scribe CALL-LOG、facet、DES、nomenclator core、任务包、结果档、materials)+ 追加 4(ledger、scribe/identity reports、trail) |
| 书简意图 | scribe intent(裸调) | 0 | 851abc72 在链 grep 命中 1,链 verify valid |
| 例行读数 | gauge record | 0 | 见第七节收约读数(本批挂会话跑) |
| 泊界心跳 | attractor route 双目录 | 0/0 | tools 与 engine materials 零告警(在途批读数随会话) |

## 三、TDD 先红后绿双证(T-1 至 T-4 与 T-8/T-9)

- 起点基线:139 passed(openhyg-solo 收约数一致,工作树起点 69430329)
- 红证一(现行码):新增夹具对现行码 4 failed 20 passed——routed_direct 三态三件(T-8 未实装落 unbypassed/KeyError)与白名单冻结对表一件(ImportError,T-9 常量缺席)红
- 红证二(前代守卫):对 d5cd596f^(GUARD_VERSION 1.13.0,guardcore 与 hooks/commit-msg 与 commitcore 三件前代版)7 failed 3 passed——T-2/T-3 钩子证与 ok_direct 纯函数证与白名单与 T-8 三件红;绿三件即 T-1(无挂接拒)与 T-4(形不符拒)与形不符落兜底,系两边皆拒的安全回归项如实注记
- 绿证:实装后全测族 149 passed(139 基线零回归 + 直改车道族 10 件)
- 夹具设计注记:钩子端到端走自洽假根(依 hooks 与 src 复制三深位构造,当日 trail 落假根链位)零真仓耦合

## 四、T-10 直改车道锁门语义两案裁定材料(呈用户,零引擎实装)

**机制事实**:scribe direct 分支调用 lockgate_guard,但 --locks 缺席即零查返回(lockguard-solo 旧行为)——锁门是参数门控非路径门控,直改笔调用形(watch 协议)不带 --locks 即免锁门。

**实证**:d3db0787(01:27:15Z,PARKING 投影与 pk-070.json 文书类)落在 gvec-v2-serial 独占 trail 窗 01:24:39Z–01:53:08Z 内,落链畅通坐实。**勘误**:任务包称「364f2919 与 d3db0787 两笔在独占持有期入链」——364f2919 系 01:03:59Z 台账补录直改笔(事件哈希 3c8d20d0)的 doc_id 非事件哈希,该时点 trail 锁面为空,不在任何独占窗内;独占窗内实证为 d3db0787 一笔。证据件 materials/t10/lockgate-two-cases-evidence.json。

- **甲案 追加面豁免合法化**:trail 与台账属追加共享面(SCOPE_SHARED_SURFACE 六路冻结),直改笔对追加面免锁门即既成事实正典化。利:与追加共存语义一致、零实装成本、人节点补笔零等待。弊:exclusive 名不副实(被直改笔穿透)、写点并发防线少一道、锁面可读性受损
- **乙案 直改笔尊重全锁面**:scribe direct 强制 --locks 或自根解析缺省启用,他会话 exclusive 持 trail 即拒写出排队指引(append 持位不拦,既有 G8 语义)。利:锁面语义单一与锁库互斥谓词同义、pk-057 同族防线闭合、直改笔与租约笔锁门前平权。弊:人节点补笔在他批独占期须排队、他批误取 exclusive 会连带憋住直改笔、需引擎实装归后继批
- 附注:gvec 对 trail 取 exclusive 而非约定 append 形是本次实证成立前提;若追加面锁形纪律(trail 取 append)被守住,两案差异只在 exclusive 误取时显形
- 本批不代裁(PRO-07 鉴不投射判断),裁定权归用户;甲案后继为文书正典化,乙案后继为引擎 direct 分支 TDD 实装

## 五、T-7 判定语义三裁(批 C 裁合法提交形)

- 席位基线先行:当日基线 ZCode:GLM-5.3-Flash:self-reported @2026-09-06 判定可用,体温 0.0,violate 5/5 全票判违,零漂移,parse_fail 0(4 命题×5 发亲答,R5 核哈希配对一致)
- 出合同:attractor emit-contract(引擎件),ng medium 正典 d4f84701,九发
- 作答:席位亲写零子代理,九发同判(温度 0)comply 依据 baseline_4 一类
- 计分:围堰 measure.py --score(上游闸,runs 实写 9 空转 0)+ 引擎 attractor score(runs_written 0 如实载即围堰腿先写幂等复入)
- 闸门:stable_clear(9/9 comply 变卦 0 旗 0,依据族集内单类承 basisunion 并集语义)
- 装配与执契:tally assemble 零判断装配,attractor check pass 12 项零失败,verify identical
- 终签:attractor sign 经引擎 scribe crosscheck 落链,链笔 8e97cadc,方向 comply,处置裁决通过(非 near_threshold,免呈报转主会条款不触发)
- 谱系披露:起草与作答同席且本席为直改车道既得便利者,对己不利声明在 topic.md 在档(判 comply 即本席后续文书小改动走轻车道而 bypass 例外不再为本席背书,判 violate 即本席继续享灰色便利,利益指向 violate 如实披露)

## 六、F 表

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-3 守卫直改车道 | 携带链笔引用直改提交过闸,无引用裸提交仍拒,引用未在链拒 | 过 | T-1/T-2/T-3/T-4 四夹具绿证,主树 d5cd596f 实装零改只对表,前代红证在档 |
| F-5c 判定语义三裁 | 合法提交形分类清楚,facet 合同模式九发 stable_clear 过执契 | 过 | m-idenlane-okform-1,check pass 12,verify identical,sign 8e97cadc |
| T-8 路由认账 | ok_direct 视 routed 零 bypass 强制 | 过 | routed_direct 类三态测试(认账/不在链告警/已登记归 bypass),CONTRACT 修订三十九登记 |
| T-9 白名单冻结 | 架构常量入 CONTRACT | 过 | DIRECT_LANE_FILE_WHITELIST 十条目逐项对表测试,声明位零执法如实注记 |
| T-10 两案材料 | 呈用户零实装 | 过(候裁) | 本档第四节 + materials/t10/ 证据件,含 364f2919 勘误 |
| 回归 | 既有测试全绿 | 过 | 139 基线零回归,合计 149 绿 |

## 七、管线与收约实录

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 管线三步 | 化格 → 核阅 → 检词 | 0 系列 | 见 materials/pipeline-report.json(化格在核前,检词在书简前,des-001 域外 exit-2 两件如实记,零域内件) |
| 认证 | scribe append 裸调逐笔 grep | 0 | 四笔 94015fcc 与 00a0bfbb 与 37b46a46 与 2558eb81 全 grep 命中在链 |
| 双仓 settle | lease commit --stage settle | 0 | tools 7ed03c86(段1)与 5490b5f9(段2 CALL-LOG 并集),engine dacc664(段1),cert 2558eb81 三验全过 |
| 台账冻结 | 主会避坑令执行 | 0 | 冻结笔 b69631d7(ledger 三台账 + pyproject 版本对齐) |
| 放锁收约 | unlock 15 + close | 0 | 首跑被闸四拦(scribe/CALL-LOG 三方冲突)零合并,并集对齐后二跑 revoked true 双工地双分支拆,merge edc3226c 与 13b41d6 |
| 台账复位 | 直改链笔 7762ffdd + 直改车道提交 | 0 | ab66aaac 以「直改链笔: 7762ffdd」挂接过合并版守卫 GUARD 1.15.0 即 ok_direct 活体放行,reconcile 归 routed_direct 计入 routed(F-3 与 T-8 双活体证) |
| reconcile | 双仓各一 | 读数 | tools unrouted 0(批前 0 零新增)cert_missing 9(全部 2026-09-05 存量即 stash 事故致 09-05 链临时离盘窗口读数,复位后归位),engine unrouted 0 cert_missing 12(存量),routed_direct 1 即 ab66aaac |
| 链 verify | scribe verify | valid | 2026-09-05 与 2026-09-06 双日 valid 零断链 |

## 八、越线与误差申报

1. **冻结活写面避坑令(主会 2026-09-06 追加指令)随批执行**:收约前对 sih-tools/lease/ledger/ 下 sessions.ndjson、locks.ndjson、bypass.ndjson 先 git add + commit 冻结使 close 归并的分支版 checkout 盖不到未提交行,令源即 ledgerloss5-solo 调查批根因(allow_and_merge 分支版整文件 checkout 盖主树台账活面);本批收约首跑未触发 diverged,如实记档
2. **任务包 T-10 表述勘误**:364f2919 系 doc_id 非事件哈希且该笔(01:03:59Z)落在锁面空窗非独占窗;独占窗内畅通实证为 d3db0787(01:27:15Z)一笔,材料按实测事实呈报不改任务包原文
3. **T-9 白名单声明位零执法如实申报**:commit-msg 钩位不见文件面即白名单本批无机械执法,源代码提交携合法链笔今日仍可过 ok_direct;机械执法归后继裁定面(可挂 T-10 乙案同批或独立批),不假装已执法
4. **候位实录**:gvec-v2-serial 在途持 11 独占锁,本批预会话三件先行(写白名单共享面新件零冲突),等其 01:54:49Z 收约让位后 open,未用 wait-turn 阻塞形(让位先于本批 open 完成),basisunion-solo 已先收约未撞面
5. **scribe 写入纪律**:intent 与认证 append 与 attractor sign 全程裸调逐笔 grep 验证在链,禁 meter 包裹掩败(pk-057 附记)执行无例外
6. **pyproject 版本对齐越 allow 面**:三源对齐件 lease/pyproject.toml 漏入 allow 清单,settle 首试 staged_out_of_scope 如实拦;处置即 worktree 回退该件、版本对齐随主树冻结笔 b69631d7 显式承载(wip 形过守卫,subject 明示漏报处置),不偷走 pre-close 搭车通道不越白名单走直改车道(源代码类不入选自守)
7. **scribe/CALL-LOG 批期内基线前进三方冲突**:basisunion 收约补笔 926aab2b(10:45Z)与本批 CALL-LOG 笔同位追加,close 首跑闸四 merge-tree 拦即整批拒;处置即双侧归并同内容并集(工地 settle2 5490b5f9 与主树 wip b1d12210,blob 逐字节 identical 复核)后二跑过闸
8. **close 首跑让位盖版雷实录(冻结避坑令兑现)**:首跑 pre-close 阶段把主树 locks.ndjson 削 15 取锁行(ea91a476,放锁行未入 VCS 同失),冻结笔 b69631d7 保住底本即 15 行逐字恢复加 15 放锁行按 lockdb 正典重建,复位提交 ab66aaac 走直改车道零 bypass(ledgerloss5 5a7cf7ff 同形对照:彼批走 bypass 本批 T-8 落地后免例外登记)
9. **pre-close stash 吞未跟踪件系统性缺陷(新发现,呈报主会与用户)**:close 的 pre-close 舞步对主树未跟踪件 git stash push -u 后不复原,engine 主树五笔 closeguard stash 在册(archpark 至本批五次 close 累积),2026-09-05 整日链文件与四十五项未跟踪件(各批结果档与任务包与泊材料与 lease-check)一度全体离盘;本批以 stash apply 保引用方式全量复位(五笔 stash 引用保留作审计),复位后双日链 verify valid;根因属 close 收约机制即 stash 后无 pop 通道,建议立泊件承载修复

## 九、关联

- 上游:idenlane-solo-results.md(拆分裁定)与 idenlane-envelope-solo-results.md(批 A)与 idenlane-human-solo(批 B 已收约 be316fe3)
- 本批工件:CONTRACT 修订三十九(lease 1.26.0)、guardcore GUARD_VERSION 1.15.0、facet/contracts/idenlane-guard-260906/m-idenlane-okform-1/、proposition/DES/m-idenlane-okform-1/、materials/(tdd、adjudication、t10、pipeline)
- 待决项:T-10 两案候用户裁(甲案文书正典化或乙案引擎实装归后继批);T-9 白名单机械执法位候裁;追加面锁形纪律(trail 取 append 非 exclusive)建议随 T-10 裁定一并正典化
