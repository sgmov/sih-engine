# ledgerloss5-solo 收口档:会话台账丢失第五笔机制调查

> 批:ledgerloss5-solo(只读取证,零生产写,零代码修复)
> 会话:e04b0a380af622cf
> 日期:2026-09-06
> 令源:用户 2026-09-06 令「丢失 #5 调查」+ 任务包 sih-engine/sih/state/plan/ledgerloss5-solo.md

## 一句话结论

第五笔不是并发丢行,是 idenlane-human-solo 收约(先后两次尝试)的收约让位归并路径(close 前置机械对表三步即收约硬化面)用 `git checkout <branch> -- lease/ledger/sessions.ndjson` 把工位分支的旧版台账整文件盖进主树,再由预收提交(`git add -A` + commit)把盖版结果固化为正史:第一击(00:00:13Z 本地 08:00 外显为 2026-09-06T00:00:13+08)杀 7 行(含 watchcheck 两会话三行加在途 revoked),第二击(00:06:02+08)再杀 3 行;该路径无锁无回补,pk057fix 三件套(flock、合并窗互斥、回补网)全部覆盖不到它,而回补网本体在恢复非空时还有同进程 flock 自死锁缺陷。

## 一、T-1 时间轴矩阵与 revoked 行去向定论

### 1.1 主链逐提交台账读数(sih-tools 仓,ledger=lease/ledger/sessions.ndjson,时间为提交时刻 +08)

| 提交 | 时刻(+08) | 行数 | watchcheck 行 | 事件 |
|---|---|---|---|---|
| c8861e9f | 09-05 22:55:56 | 734 | 0 | constmodel 补录直改笔;**idenlane-human 开工基线(其 base_branch=integral-stage-build 当时指此)** |
| 682d243e | 23:17:41 | 736 | 0 | docmath-namefit 预收提交 |
| e00edccd | 23:21:06 | 736 | 0 | gaugecontrib 提交 |
| e9dc3cfb | 23:26:52 | 738 | 1 | watchcheck 第一会话(1d463181f4bf7570,issued 15:25:45Z)close 预收提交 |
| eaa95689 | 23:57:05 | 738 | 1 | watchcheck 段1 工地分支提交(分支台账与 merge-base 逐字相同,从未改台账) |
| e8d632da | 23:57:45 | 740 | 3 | watchcheck 第二会话(445c12fbb3e4283c)close 预收提交;三行=issued/revoked-1d46 加 issued-445c |
| 17ec7a34 | 23:57:46 | 740 | 3 | watchcheck 副本归并(台账字节与 e8d632da 全同,本批收约干净) |
| 15:57:47Z(未提交) | — | 741(工作面) | 4 | revoked-445c 落主树工作面(receipt closed_at=15:57:47Z) |
| f165e0aa | 23:59:59 | 740 | 3 | watchcheck 收约补笔(索引态,台账未再暂存) |
| **2b7b1ce2** | **09-06 00:00:13** | **734** | **0** | **idenlane-human 第一次 close 预收提交:盖版固化,7 行俱失(6 已提交+1 在途)** |
| 3a7ad35d | 00:05:03 | 735 | 0 | regula-rename 预收提交(issued-regula 补回后入史) |
| 82ba663e | 00:05:20 | 735 | 0 | regula 补笔 |
| **cc2f36c4** | **00:06:02** | **734** | **0** | **idenlane-human 第二次 close 预收提交:再杀 3 行(issued/revoked-regula 加重补的 issued-idenlane-human)** |
| f943fbfd | 00:06:03 | 734 | 0 | idenlane-human 副本归并 |
| 5cba19e5 | 09-06 09:04:04 | 746 | 2 | 主会补录三笔(watchcheck issued 逐字+revoked 凭据重建+idenlane-envelope issued 逐字),repair 标记在账 |

### 1.2 丢失行明细(f165e0aa→2b7b1ce2 删除的 735-740 六行加工作面第七行)

issued docmath-namefit ab0aa6b23047a03d(15:13:52Z)、issued idenlane-human be316fe3596bf16e(15:14:57Z)、revoked docmath-namefit ab0aa6b23047a03d(15:17:43Z)、issued watchcheck 1d463181f4bf7570(15:25:45Z)、revoked watchcheck 1d463181f4bf7570(15:26:53Z)、issued watchcheck 445c12fbb3e4283c(15:27:05Z)、revoked watchcheck 445c12fbb3e4283c(15:57:47Z,仅工作面)。

### 1.3 revoked 行去向定论(F-1 三选一)

**主树曾有**:revoked-445c 由 append_row 于 15:57:47Z 落主树工作面(收约凭据 closed_at 与 17ec7a34 之后才追加的时序俱证),从未入任何提交,于 00:00:13+08 被 idenlane-human 第一次 close 的 checkout 盖版从工作面抹除;原笔字节已不可恢复(无 VCS blob),现账行为 09:04:04 凭据重建笔(repair 标记 verbatim=false 同族语义)。第一会话两行与 issued-445c 则主树曾有且已入史(e8d632da/17ec7a34/f165e0aa 可逐字恢复)。

### 1.4 工地与备份考古

- worktrees/sih-tools/watchcheck-solo 与 idenlane-human-solo 工地已拆,无残件可考。
- worktrees/.close-backups/idenlane-human-solo/be316fe3596bf16e/sih-tools/lease/ledger/sessions.ndjson 为 **00:06 第二次 close 的备份**(737 行,零 watchcheck 行,文件 mtime 00:06)——00:00 第一次 close 的备份(盖版前 741 行)已被第二次 close 同路径覆写,**首击的完整备份已不存在**。
- 同窗 locks.ndjson 同机制被盖:f165e0aa(5781 行)→2b7b1ce2(5734 行),47 行已提交锁行被删;00:06 备份 5790 行。
- 双仓各留五笔未弹的 `closeguard-solo pre-close stash`(stash 卷走未跟踪活写面缺陷的遗留,watchcheck-solo-results.md 第九节已记录其恢复实录)。

### 1.5 首次 close 为何失败、二次为何成

00:00:13 第一次尝试:收约前置机械对表处置与预收提交完成后,merge-tree 预检在 `lease/CALL-LOG.md` 与 `scribe/CALL-LOG.md` 检出双侧同改内容冲突(merge-base=c8861e9f,旧式 merge-tree 出 2 处冲突标记),StateError 整批拒——但盖版与固化已先发生。00:02:55 regula-rename 开工、00:05:04 收约,主链 CALL-LOG 尾行前进使冲突 hunk 错开,00:06:02 第二次尝试 merge-tree 自解通过,收约成功(receipt closed_at=16:06:04Z)。

## 二、T-2 close 路径源码走读对时间轴(核心文件 sih-tools/lease/src/lease/)

事故链条逐行(行号对应当晚在役代码,当前主树同形):

1. `close_session`(core.py:702 起)锁清零与工地卫生检查通过后,进入收约前置机械对表步骤一与二(core.py:743-757,代码注释内名即收约硬化面):`detect_merge_conflicts`(core.py:1047)用 **`git diff --name-only base branch` 两端点差集**(core.py:1056)取归并面——idenlane-human 的 base 引用 integral-stage-build 在批期内前进了 6 行,分支台账与 merge-base 逐字未动,两端点差集仍把 sessions.ndjson 列入归并面(分支侧「看似改了」实为主干侧前进),工地分支并无任何台账新内容可归并。
2. `is_pure_append_conflict`(core.py:1148)判型:工作面 741 行以 base 版 740 行为严格前缀加 1 尾行,判 `append`(core.py:1190-1200),live_only=revoked-445c 一行。
3. 纯追加处置(core.py:777-808):`backup_conflict_files`(core.py:1274,备份正确落 .close-backups)→ **`allow_and_merge`(core.py:1306)在 core.py:1341 执行 `git checkout <branch> -- <f>` 整文件换版**——工作面被替换为分支旧版 734 行,7 行俱失;此步**不持 ledger_lock、不调 restore_missing**,与台账写点纪律(BATCH-FACE 台账写点纪律节)直接冲突;`generate_re_certify_hashes` 只产补笔清单不复原。
4. 预收提交块(core.py:833-878):`git stash push -u` 卷未跟踪件后 `git add -A` 加 commit(core.py:872-873)把盖版态提交为 2b7b1ce2——**丢失入史**;若后继失败(本次 merge-tree 拒),盖版不回滚。
5. 合并窗回补网(basefix F-3,core.py:913-956):`_chain_append_grown`(core.py:529)要求工作面以 base 版为前缀——**此时工作面已被第 3 步盖成 734 行(≤base 740),判 None,union_files 为空,`_chain_union_yield` 整段跳过**:pk057 的 ledger_lock 互斥与 restore_missing 回补在该路径上结构性不可达(次序倒置:防护段在破坏段之后)。
6. `git merge --no-ff`(core.py:938)在已固化态上合并,丢失终局;分支删陔回录 revoked(core.py:1016)。

另:预收提交块 stash 弹回 `_git(repo,"stash","pop")` 返回码被忽略(core.py:970),与 watchcheck 事故记录(当日链静默缺席十分钟)同源,已在彼档候裁,本档不重复裁。

## 三、T-3 沙盒复现(红证,materials/sandbox/,生产代码只读导入)

- **h4-repro.py 甲景**:生产收约让位归并子序列直调(detect→classify→backup→allow_and_merge→预收提交→merge),主链前进 6 行加工面活写 1 行的镜像初态:10 行→盖版后 3 行,**7 行丢失**,预收提交固化 3 行,`_chain_append_grown` 回 None,merge 终局 3 行——与生产 741→734→734 逐量对表可重放。RED=true。
- **h4-repro.py 乙景**:`_chain_union_yield` 子进程直调(恢复非空)——12 秒超时探针机械固定**同进程自死锁**:`_chain_union_yield` 持 ledger_lock(core.py:580)→`restore_missing`(ledgerwrite.py:68)→`append_row`(ledgerwrite.py:49)→`ledger_lock` 同路径二次 flock,flock 按 open file description 判定,同进程同线程亦阻塞。生产从未死锁只因该路径从未带着非空恢复被执行过。RED=true。
- **h4-repro.py 丙景**:修复方向证明——单锁窗加窗内差集 O_APPEND(与 restore_missing 同语义,锁不重入):10 行→10 行,零丢失。RED=true。
- **h2 单元**:`restore_missing` 无外持锁时对缺 6 行回补 6 行,比对键与备份路径本身正确。RED=true。
- **h1-h3-repro.py H-1**:假工地内执行工具副本,`tool_dir()` 指向工地副本、缺省台账位随之漂移(前提为真);`resolve_root` 双仓祖先搜上仍回真根。
- **h1-h3-repro.py H-3**:他进程持 ledger_lock 时 `allow_and_merge` 的 checkout 0.008 秒即时完成并盖掉活写行(绕锁),对照组 `append_row` 同刻阻塞 2.01 秒至锁释放——让位归并写点绕开 pk057 互斥网的机械证明。

## 四、T-4 根因钉死与 H 表裁断

**唯一根因(代码行级)**:`close_session` 的收约让位归并纯追加处置链——`detect_merge_conflicts` 两端点差集把主干侧前进误入归并面(core.py:1056)→ `allow_and_merge` 用分支版整文件 checkout 盖掉主树活面且无锁无回补(core.py:1336-1347)→ 预收提交 `git add -A`+commit 把盖版固化为正史(core.py:872-873)→ basefix 回补网因前缀前置已破而结构性跳过(core.py:551)。触发条件:批期内主链台账前进(多批并行收约的常态)加工地上分支台账与 merge-base 相同(凡工地不改台账的批皆如此)——**每次收约都在掷骰子成立**。

| 假设 | 裁断 | 依据 |
|---|---|---|
| H-1 台账位错(tool_dir 随工地定位) | **证伪(对本笔)**,前提本身为真 | 事故序列中 revoked/锁行/盖版全部落在主树台账与主树工作面(00:06:04 revoked 在主账在档),close 以主树 tool_dir 执行;沙盒证前提存在(openhyg 申报一同族)但不是本笔机制 |
| H-2 回补网缺陷(比对键或备份路径错位) | **字面证伪,实质成立(更深层)** | restore_missing 单元正确(h2 单元);缺陷实为两层:回补网对让位归并盖版不可达(次序倒置)+可达且恢复非空时 ledger_lock 同路径重入自死锁(乙景红证) |
| H-3 并发覆盖(他会话活写交错、锁序未覆盖) | **部分成立** | 不需要他会话并发即丢(甲景单线复现);但「close 链内绕开 ledgerwrite 直写」成立:allow_and_merge 的 checkout 绕开 flock 互斥(H-3 沙盒 0.008 秒机械证明) |
| H-4 git 层整文件替换(纯追加并集超集判定未触发或漏行) | **成立(主因)** | checkout 整文件换版+预收提交固化;超集判定(修订二十八/回补网)未触发的机制是次序倒置致 `_chain_append_grown` 回 None,非超集计算漏行 |

**前四笔误判修正(候裁)**:当晚同签名倒灌至少还有两笔——b679086f(idenlane-envelope 第二次 close,18:21:33+08,杀 6 行含 gateswitch 与 viewline 两对)与 cc2f36c4(00:06:02,杀 3 行);constmodel 两行按 CALL-LOG 116 亦被 b679086f 倒灌抹除。前四笔「运行时追加无锁与合并窗口交错」的归因至少未覆盖此机制,重归因候选呈报候裁。

## 五、T-5 修复方向(呈用户裁,零实装)

按影响面从小到大,可组合:

1. **让位归并禁改台账面**:allow_and_merge 与预收提交对 `lease/ledger/*.ndjson`(锁册会话册)整文件 checkout 与 add -A 设冻结豁免——台账只许差量追加或回补,主树活写面永不入 stash 与整文件换版面(对 watchcheck stash 事故同治)。
2. **归并面差集改三点式**:`detect_merge_conflicts` 的归并面改用 merge-base 三点差集(`git diff --name-only $(git merge-base base branch) branch`),主干侧前进不再误入归并面,「分支无新内容可归并」的文件零触碰。
3. **回补网前移加去重入**:纯追加处置改为先持 ledger_lock,窗内 checkout 后以窗内差集 O_APPEND 回补(丙景语义),并消除 `_chain_union_yield`→`restore_missing`→`append_row` 的同路径二次 flock(锁对象可重入化或回补走窗内直写)。
4. **预收提交最小化**:`git add -A` 收窄为归并面文件清单,杜绝把台账与全部未跟踪活写面一次性卷入提交与 stash。

修复批另立过一裁;本批零修复实装。

## 六、连带伤亡盘点(截至本批,活账仍未恢复的行,候人节点令)

- watchcheck 第一会话 issued/revoked 1d463181f4bf7570 两行(VCS 有原笔,e8d632da 可逐字恢复);
- issued idenlane-human be316fe3596bf16e 一行(revoked 已在账;VCS 有原笔);
- issued/revoked docmath-namefit ab0aa6b23047a03d 两行(现账 docmath 行系 openhyg 活体验收重开会话 1c924498519b0dc8,非原会话);
- issued/revoked regula-rename dc8a180223aa590f 两行(issued 在 3a7ad35d 有原笔,revoked 原笔仅存在于 00:06 备份件 737 行版);
- locks.ndjson 已提交 47 行(2b7b1ce2 删)与后续窗各行(锁库 SQLite 为判定正典,ndjson 为镜像,影响面待评估)。

## 七、F 表自检

- **F-1 时间轴**:1.1 节矩阵在档;revoked-445c 去向定论=主树曾有(工作面),三选一落定。
- **F-2 复现**:甲景与生产 741→734 逐量对表可重放,四红证(materials/sandbox/*.log)。
- **F-3 根因**:代码行级落 core.py:1056/1336-1347/872-873/551;H 表四项逐判。
- **F-4 零生产写**:生产台账与锁与 locks.db 与双仓主树源码零改动;沙盒件与产物全部限 materials 与工地;本批自身链笔与认证除外;git 考古全只读命令。
- **F-5 修复呈报**:第五节方向呈报,零实装。

## 八、关联

- 任务包:sih-engine/sih/state/plan/ledgerloss5-solo.md
- 加固批:pk057fix-solo(lease 1.24.0,修订三十六)
- 同族先例:openhyg-solo 申报一(tool_dir 工地定位)、basefix-solo(F-3 回补网)、watchcheck-solo-results.md 第九节(stash 卷走活写面)
- 补录:主会 5cba19e5(台账补录三笔)
- 后继:修复批(本档第五节呈报后另立过一裁)

## 九、当场复验:本批自身收约再触发同款盖版(损伤已复位)

本批收约(2026-09-06T02:07Z 前后)按机械链执行时,根因机制在调查批自身重演一次,构成第五笔机制的活体复验:

- 收约前冻结:主树会话台账(sessions.ndjson)与当日链经 bypass 冻结直改笔(tools 662ac8bb 与 engine d6400c1)先行入 VCS,两文件收约时净态——**零损伤**(sessions 749→750 仅追加本批 revoked,链 41 行零损)。
- 漏网:本批三把锁的 unlock 笔在冻结之后追加 locks.ndjson,使其在收约探测时再度为脏;收约让位归并对 locks.ndjson 触发同款 checkout 盖版,预收提交 0aca5f68 固化,26 行已提交锁行(gvec 全部锁活动与本批三行 acquired)加 3 行 working-tree released 笔俱失(5859→5830)。
- 复位:26 行自 662ac8bb 逐字恢复,3 行 released 按 lockdb 正典(SQLite lock_event 表 released_at=02:06:59Z)重建,经 ledgerwrite.append_row 逐行追加复位至 5859 行,恢复直改笔 5a7cf7ff bypass 登记在册。
- 判读:冻结者存、未冻结者亡,同一收约同一机制两种命运——根因结论(让位归并整文件盖版+预收提交固化+回补网不可达)获生产环境当场复验,修复方向第一条(台账面冻结豁免)的必要性同时获证。

另:收约首跑被真分叉拦一次——主树未跟踪目录 ledgerloss5-solo-materials(recall-topic.md,与分支内容逐字相同)与分支新增树在两点位差集以「目录 vs 树」形态相撞,被误判 diverged;按备份让位归并对表法(BATCH-FACE § 10)四步处置(diff 逐字节 IDENTICAL 在档)后收约一次过。此为 detect_merge_conflicts 目录形态误判的旁证,随修复方向第二条(三点式差集)一并呈报。
