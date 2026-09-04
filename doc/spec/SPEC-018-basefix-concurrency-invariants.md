# SPEC-018 地基并发不变式规范：locks 互斥与链追加原子化与收约并集复查

本规范承接 basefix-solo 批，令源即用户 2026-09-04 裁定「pk-051 修复插到载体接线前，并行批安静窗口开工」。把三类并发缺陷的修复形态与不变式钉死为可机械校验条款。承接面：pk-051 即 locks 判定面双偏差，leasewire-solo 实查申报；pk-045 样本库 trail 竞态类两起实案，即 facepark-solo 丢两笔与 leasewire-solo close 丢 pk050sw 认证一笔；DEC-013 融回序位注记，租约将来融回地基先行。体例承 SPEC-014 与 SPEC-015 与 SPEC-016 与 SPEC-017。载体引用：ORD-020 全序资源分配与死锁自由，配对洞即全序性破坏点，修复即回归载体，注记级不立新条目；ORD-019 版本偏序与外化状态存储，链追加即覆盖关系只增不改写。推导档 sih-math/docs/basefix-derivation-2026-09-04.md。行为变更批判变申报：三件修复只向前生效，历史链文件与已收批账目零改写。

## 概览 {#overview}

- locks 互斥不变式即任一时刻任一路径至多一持锁会话，单遍事件序配对承载，放后重取形不绕穿::[互斥不变式](#mutex)
- 路径同一化即绝对形与 ./ 前缀形与裸相对形三态同一标识，异形态同路径撞锁::[资源同一化](#identity)
- 链追加原子化即进程间排他锁覆盖读算写全程，并发追加全数在链且 verify valid::[追加原子化](#append)
- 收约并集复查即归并前重读现行链重算并集超集，纯追加活面自备份让位并透出补笔清单::[并集复查](#union)
- 判变只向前生效，l4 金向量重冻 as-is 旧向量留档披露不删::[判变申报](#judge)

## 互斥不变式 {#mutex}

SDD 修复形态：locks core.active_locks 由两遍事件序改单遍事件序配对，即按台账追加序逐行扫描，acquired 行置位 path 至 session 的持位，released 行仅弹同号同路径位。不变式一即互斥：任一时刻任一路径至多一持锁会话，acquire 判定位对在锁映射的他会话持位即 locked_elsewhere 拒绝。反例钉死即修复前红证：W1 取放重取同路径后，两遍配对使重取行被旧 released 误抹，锁面读出缺在锁位，W2 再取被放行即互斥绕穿；单遍配对下重取行在后即持位在册，W2 被 locked_elsewhere 拦。机械证：locks 测试 test_release_reacquire_pairing_keeps_lock 先红后绿，金向量 l4 场景重放改判 locked_elsewhere。修法承 lease 侧单遍配对先例，即 leasepatch-solo 缺陷一同类。

## 资源同一化 {#identity}

SDD 修复形态：locks core.normalize_path 增 root 参，绝对路径剥工作区根前缀，去 ./ 前缀，尾斜杠统一；acquire 与 release 与 lock_status 与 check_baseline 全判定位传 root。不变式二即标识唯一：绝对形与 ./ 前缀形与裸相对形三态同一资源同一判定，异形态同路径撞锁即 locked_elsewhere。机械证：locks 测试 test_normalize_path_three_forms_same_identity 先红后绿，修复前红证即绝对形 scope_violation 或异标识放行；金向量重放载同形场景。修法承 lease 侧 locksplit-solo 资源同一化先例。

## 追加原子化 {#append}

SDD 裁形申报：进程间文件锁即 flock 择于单写原语，依据有三。其一零新增常驻进程与零新增传输层即新增失效面最少，flock 二元经 extern 声明直调，Cargo 依赖零增，进程死亡锁自动释放无陈锁残留；单写原语即专用写守护或消息队列引入新常驻组件与进程间协议即新增失效面，违工程基线第五条治理延伸是减少机制。其二读算写三段同一临界区，即链尾 prev_hash 计算与校验与落写之间无窗口，锁内重读现行链不信调用方早前快照。其三语义零变，事件 schema 与哈希公式与退出码三值零触碰，只收并发写面；非 Unix 平台降级直写不拦，本工程目标平台 macOS 与 Linux 全在位。

修复形态：event_stream append 位 path 在场即以 create 加 read 加 append 三旗开链文件、flock 排他、锁内 load_events 重读、append_in_memory 校验四项与事件构建、落写一行、解锁回填调用方视图；path 缺席即纯内存追加语义逐字节不变。scribe append 位增有界重试：并发下他进程先行落链使本进程预取时间戳不越新链尾即拒 TimestampNotMonotonic，取新鲜时间戳重试三次，事件内容仍由报告件确定性派生零伪造，拒绝路径仍为后手保底退出码一。

不变式三即追加原子：双进程对同一链文件并发 append 各 N 笔，终态两进程事件全数在链且 scribe verify valid。机械证：tdd_tests t7_concurrent_append_race_two_processes 先红后绿，红证两形态即并发无锁写互缴致 parse line failed 链损坏与同尾双算 prev_hash 分叉，绿证即 2N 事件全链 valid。

## 并集复查 {#union}

SDD 裁形申报：close 拒或扩集两选项中择扩集含两方事件的受限形，即纯追加活面由 close 自备份让位归并并透出补笔清单，链语义修复归书简补笔通道，close 零直写链文件；真分叉非纯追加维持整批拒零动作。依据：链文件唯一合法写通道是 scribe，承工程基线第一条确定性程序与链写单一通道红线，close 直写链即第二写通道；纯追加形让位只丢尾行且备份全保全，补笔清单使丢失面可机械对账经书简复认证，承 pk050sw 补笔复原 7e1aa2 先例。

修复形态：close_session 归并前重读现行链，_chain_append_grown 分类即 ndjson 归并件落盘行序以 base 全行集为严格前缀且多出尾行，此类不再入前置态真分叉拦位；归并前备份落盘内容至 worktrees/.close-backups 确定性路径再 checkout 让位，归并后重读合并件算 re_certify_hashes 即备份独有行 sha256 前十六位清单，随 removed 记录入 revoked 或 close_failed 台账事件可对账。不变式四即收约不丢笔：settle 快照落地后活链新增的每一行，要么在合并结果中，要么在补笔清单中且全文保全于备份。机械证：lease 测试 test_close_chain_append_growth_auto_yields_union_report 先红后绿，红证即修复前整批拒逼出人工让位复跑窗口即事故根因；test_close_diverged_non_append_still_refused 边界绿；写面封闭测试增豁免位一处即 dest.write_text 唯一且落点限 .close-backups 子树。

## 判变申报 {#judge}

三件修复只向前生效：locks 互斥判定与路径同一化与链追加并发形与收约并集复查均自本批合并时点对新操作生效，历史链文件与已收批账目与历史锁操作记录零改写零重算。l4 金向量判变重冻：as-is 偏差形旧向量留档披露于 leasewire-solo-materials 不删，修复后形态由 basefix-solo-materials 重冻，l4 场景改判 locked_elsewhere 与锁面读出全，重放寻径约定承两工具源路径参数传入，cases 零绝对路径。scribe 追加原子化语义零变不涉判变。

## 版本 {#version}

v1 于 2026-09-04 basefix-solo 批随批首建，号位 SPEC-018 实取下一空位，SPEC-017 号位已被贡献度测度规范占用。本规范在引擎治理域内，核阅与检词从引擎文档流程。验收对表即互斥不变式与标识唯一与追加原子与收约不丢笔四不变式各载机械证，测试读数 locks 10 绿与 lease 88 绿与引擎全套零回归，存量红 f2_refs_machine_verifiable 批前同态非本批回归。

## 内容充分性 {#sufficiency}

- 本节为 docmath-b4-solo 收尾批按新旧都管裁定补齐，模板见 SPEC-TEMPLATE-sufficiency-v1，只加节不改本文实质。
- 判据红证对表：本文判据条目见 互斥不变式、资源同一化、追加原子化、并集复查等节。按证伪覆盖度载体如实申报：历史判据清单未逐件附可构造红证即零信息部分，清账路径为后继修订批逐件补红证，承 sih-math/docs/docmath-carriers-derivation-2026-09-04.md。
- 判定性常数挂锚对表：本文档无声明的判定性常数。des-001 C007 与 C008 与 C009 行级与邻近级闸在役核验。
- 约束算子对表：本文无约束算子面，如实申报。
