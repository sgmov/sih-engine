# 租约契约书：引擎正典 lease-engine-CONTRACT v1

按任务包生命周期治理写入的租约工具引擎件，主件 sih-engine/src/bin/lease.rs 加子模块 closegate 与 commitlaw 与 guardlaw 与 sddgate 与 attachments 与 calllogface 与 sweepcore 七件，二进制位 sih-engine/target/debug/lease，工件 tool.version 1.46.0 承契约面字面，SPEC-015 先例即工件工具名版本字段是契约面标识非二进制身份。钉实即借用 git 机制，立约开工建副本签发会话档，租内取锁放锁，期满收约归并删支拆本，台账 append-only 只增不改承工程基线四。融回落差规格正典 SPEC-024（sih-engine/doc/spec/SPEC-024-lease-mergeback-gap-v1.md），行为对表围堰现行文即同参双跑判词一致。

## 概览 {#overview}

承继声明：

- 本书为引擎正典。承 2026-09-14 载体切换即围堰退役宣告，即 sih-tools 生产工具转冻结只读兼容态、生产调用面全数切引擎 bin 位；与 2026-09-17 pk-066 出泊裁定，出泊事件 6baac9ed 在链，裁定语照录「引擎立新书即引擎侧新立租约契约书以引擎十二子命令面为准，旧书 sih-tools/lease/CONTRACT.md 冻结仓内退役归档零改动」，承用户 2026-09-17 令「裁，多子代理并行」按普查推荐裁定执行
- 旧书 sih-tools/lease/CONTRACT.md 自本书立位起在冻结仓退役归档零改动，围堰源码保留作对表基准与历史参照；旧书承载的裁定史与测量史继续在档有效，语义权威自此以本书与引擎实装为准
- pk-066 原病灶即书与实漂移，旧书用法节载十三对而围堰实装十七，自此从构造上消除：本书子命令面即引擎 main 分派块实载十二对，书与实装同源于 sih-engine/src/bin/lease.rs，后续子命令增删先改本书再改码，禁漂移
- 承继面：旧书机器形态节与验收判据节与载体引用节所载语义中已由引擎实装对等承接者随本书继承；未承接面在落差申报节如实登记，不默认继承不静默丢弃

- 十二子命令即 open 与 lock 与 unlock 与 close 与 status 与 commit 与 bypass 与 reconcile 与 sweep 与 call-log 与 install-hooks 与 uninstall-hooks，逐件一句契约语义::[调用](#usage)
- 台账双册与账单与检查件 append-only 追加即唯一写点，退出码三值即零成一拦二工具异常::[机器形态](#machine)
- 正身留痕不签，租约位执行即拦，验证与执行分工承 DEC-010 与 DEC-011::[协议](#protocol)
- 落差申报即引擎件与围堰现行为的已知差异逐条在档，含 sqlite 索引腿未实装与锁面现势推导形差异::[落差申报](#gaps)
- 锁判定语义数学承载 ORD-020 全序资源分配与死锁自由::[载体引用](#carrier)
- 本书版本线起 lease-engine-CONTRACT v1，与旧书版本线即终版 1.46.0 分轨::[修订记录](#revisions)

## 调用 {#usage}

工作区根起跑，二进制位 sih-engine/target/debug/lease，十二子命令分派即 main match 块实载，未列子命令拒退出码二并报覆盖面。

- open：立约开工即域感知定根，显式 --root 优先缺省自 cwd 上溯，canonical 域界先检即停；后按四面登记表即 sih-engine/sih/state/plan 与 sih-engine/task-packages 与 sih-math/sih/event/plan 与 sih/state/plan 搜包档；解析请求写入节为 allow 面第一源，遇全角破折号截断取路径部理由散文不入面；过 stem 查册闸与甲表认领机械兜底，兜底旗标即 --new-stem 与 --claim-zh 与 --claim-code 与 --claim-derivation，承 DEC-017 修订五修订六；验意图结构面，必填集即 session_id 与 raw_input 与 round 与 intent_contract 与 domain_contract，载 anchors 者为 ask3 形缺省 plain 形承 DES-016，与正身件；起 msh/<包名> 副本分支记基线分支，以确定性派生会话号 sha256(包名|签发时刻|身份件哈希|仓序列)[:16] 落台账 issued 行。
- lock：租内取锁即活跃会话在册验，缺 --session 且活跃会话唯一即自取，不唯一拒 session_not_active，加路径互斥验，他会持同路径拒 locked_elsewhere，同会话重入回执 duplicate:true 幂等零追加，新锁 acquired 行落锁册，--mode exclusive|append 缺省 exclusive。
- unlock：放锁即同会话持锁位核验，他会所持拒 locked_elsewhere，无锁拒 not_held，released 行落锁册。
- close：收约执法闸序即锁清零、工地卫生、冲突三态、预收提交、链证守门、SDDG 四判据即判据定义单源 DEC-024、GATE_SINCE 2026-09-12 不溯往、CALL-LOG 跑步机与随批检查、无主闸、声明差集闸、归并删支拆本、账单与收约凭据，拒即整批零动作零半程，--bypass-sddgate 显式绕行落 bypass 台账留痕。
- status：查册即读会话台账与锁册出 header 与 locks 与 sessions 三面投影。
- commit：提交正身路径即 --stage 二值 wip 或 settle 经四验即会话在册验与范围验即 staged 全落 allow、条目精确或目录前缀、归一共形剥前导点斜杠与尾斜杠，与认证在链验即 settle 必挂 --cert 且哈希前缀命中链上认证笔、wip 免认证门，与信息机械生成即模板单源三形态 wip 与 settle 与收约归并、base 由工具自算、零自由 message、wip 形纳 note。
- bypass：绕行留痕即 --repo 与 --sha 与 --reason 三必填落 bypass.ndjson 一笔，台账只增不改，sha 全号短号皆收即对表前缀互含。
- reconcile：三方对表即 git log 对会话台账对 trail 对 bypass 台账分类即 routed 与 routed_merge 与 routed_direct 与 bypass 与 unbypassed 与 session_orphan 与 cert_missing 与 cert_backfilled 与 cert_writtenoff 与 sealed 十类，内置封线 SEAL_BASES 与追认表 SEAL_EXEMPTS 冻结，表变更须升版本，只报不拦即对表是可见性不是闸门，告警类非零即退出码一。
- sweep：扫残留即幻影活跃会话与僵尸锁与停滞检验文件与散位收据与无主工地五类协调态残留全机械普查零 LLM，三态输出即自清件与候裁件与净外信息，--fix 自清件径走既有通道零新执法位零 git commit 零 bypass，退出码零净一有残二工具异常。
- call-log：调用留痕册导入即 19 册 CALL-LOG.md markdown 存量 verbatim 整行迁入权威腿 calls.ndjson 即 P3 迁移忠实形，尽力解析失败即空值申报不猜不编，行数对表与 verbatim 零丢与失败清单在档，投影腿 verbatim 重组形再生。
- install-hooks：守卫安装即向仓写 git config core.hooksPath 指守卫目录，--repo 可重复，config 属环境态不入版控即换机须重装。
- uninstall-hooks：守卫拆卸即 unset 可逆，键缺席计 already_absent 照常收敛不判败。

## 机器形态 {#machine}

- 台账族即 sessions.ndjson 与 locks.ndjson 与 bypass.ndjson 与 lockface-bills 账单与 checks 检查件与 receipts 收约凭据，追加即唯一写点，历史行原文保留不随改名重写；会话现势即台账事件序单遍配对：issued 置位 revoked 弹在册同号，同号重开不被旧 revoked 误抹。
- allow 面组合即包档请求写入节抽取面加显式 --allow 键抽取面去重并入，requested 面序在前原样保留；范围两闸即 lock 与 commit 共用归一式比对，allow 条目点号语义为仓根递归全容。
- 收约归并机械即台账追加面冻结豁免不走整文件盖版、归并面三点式以 merge-base 为基准取 branch 侧改动、未跟踪件照单登记不清场零 stash 舞步、幂等删支即 ref 缺席计 already_gone、close_failed 随败随记。
- SDDG 门四判据全判定命令形零 LLM 零网络即正则匹配与时序比较与 git 差分文件清单三件机械事，判定命令单源照录 DEC-024 禁改写。
- 直提守卫纯函数承 guardlaw 即提交信息形态判定与 staged 面三面覆盖判定即活跃租约锁面加共享追加面加轻车道白名单，只拦形态不判内容，白名单与共享面扩面走本书修订禁裸奔增删。
- 退出码三值：零即成；一即拦含会话不在册与锁态冲突与收约闸拒与对表告警；二即工具异常含文件非法与 git 不在与用法错。

## 协议 {#protocol}

正身留痕不签，租约位执行即拦，验证与执行分工承 DEC-010 与 DEC-011。写面封闭即源码写调用仅台账追加与检查件与收据与 git，零 LLM 零网络。哲学仪式不入机械层承 DES-016 即意图闸验结构不验引文，ask3 双门归司衡本域批纪律位。

## 落差申报 {#gaps}

- call-log 索引腿未实装：sqlite calls.db 腿不随引擎件，Cargo 零 rusqlite，A9 依赖零新增，权威腿 calls.ndjson 与投影腿在役，索引腿候后继批裁。
- 锁面现势推导形差异：围堰现势锁面从 sqlite lockdb 投影读，引擎从 ndjson 锁册 acquired/released 事件序推导，语义同构；sweep 僵尸锁 fix 围堰走 lockdb.takeover_release INSERT，引擎落锁册 released 行。
- 围堰在役而引擎未承接的子命令面即 claim 与 unclaim 与 heartbeat 与 takeover 与 wait-turn 与 ledger-repair 等不在引擎分派块，其职能承载候各自裁定面，本书不默认继承不判退役。

## 载体引用 {#carrier}

锁判定语义承载 ORD-020 全序资源分配与死锁自由，条目位 sih-math/order/entries/ORD-020-total-order-resource-allocation-deadlock-freedom.md，mapping 位 mapping.md:206，三类语义形式化即互斥与死锁不自由与等待终止性，推导档 sih-math/docs/ordwire-lease-derivation-2026-09-03.md 与 sih-math/docs/leasewire-derivation-2026-09-04.md 在档，引擎判定位以本书调用节所载各子命令实装位为准。

## 修订记录 {#revisions}

2026-09-17 v1 初版：随 pkexits3 批起草，令源 pk-066 出泊裁定，事件 6baac9ed，承 2026-09-14 载体切换，子命令面照引擎 main 分派块十二对实载，书与实装同源即 sih-engine/src/bin/lease.rs。本件为暂存草案，落正典位与管线三步即化格核阅检词候新书批。
