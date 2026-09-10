# scoperoot 批结果档

> gatefix-parallel 簇L执行代理落档，2026-09-11。令源 gatefix-parallel.md 簇L行与 pk-091 其一其二、pk-093 其二其三出泊条件，用户 2026-09-11 令「多子代理全量修复」。批机械链全序承 sih-tools/BATCH-FACE.md。

## 一、批身份与链证 {#identity}

- 批名：scoperoot，查册 unknown（nomenclator query 与 map --concept 乙前注入双证：零近邻零 code 形），--new-stem 甲表认领；概念锚 zh 范围归一根形，既裁 code 形显式申报无承，派生机械形整词单段 scoperoot:new（闸按分隔符分段 scoperoot 不分节；语义派生 scope 范围既立语素加 root 根锚定载于 zh 锚与意图件，查册 scope 与 root 俱 unknown 故不入机械对表），回执 stem_check.new_coinage_claim 在案
- 会话号：a6d6cfc9b6e35012（已 revoked，工地拆除，支 msh/scoperoot 删除归并）
- 意图笔：a8591258f0426d39 在链，plain 形 validation 豁免（DES-016）
- 直改声明笔：d6412e31ba80bcb9 在链（本档收约后交付件 direct_edit_completed，--no-session-reason 显式事由形，会话已 revoked 非会话期写入）
- 认证四笔在链（2026-09-11 trail）：a8591258（intent）与 ade5a474（tdd 红证封装八测俱败）与 febe43a2（suites 352 绿报告，即 settle cert 引用笔）与 aac2f475（文档管线读数）；trail 全链 verify valid（52 events）
- settle 提交：sih-tools 仓 22ba3646（段1 in-scope）；归并 merge ecfac044（前有簇K trailhome 归并 12540dff）
- reconcile：unrouted 0 加 cert_missing 0 双零；主树改件三笔 bypass-orphan 留痕在 bypass.ndjson

## 二、F1–F7 逐条实跑 {#f-results}

- F1 根锚定夹具仓（repo 恰为 root）细粒度 allow 提交过且先红：过。test_f1_root_anchored_repo_fine_allow_commit_passes 红转绿，伴钉 test_f1b 越界明细取仓相对裸形（outside == ["outside.md"] 非 ./ 前缀形）
- F2 allow 点号形子路径取锁与提交两闸俱过且先红：过。test_f2_dot_allow_subpath_lock_and_commit_both_pass 红转绿（先红半在锁闸：点号条目对归一裸形路径原拒 scope_violation）
- F3 legacy 点号斜杠前缀 allow 等价过：过。test_f3_legacy_dot_slash_prefix_allow_equivalent 红转绿（锁闸先红，commit 侧归一前后俱过即等价不破）
- F4 wip note 入 message 先红后绿：过。test_f4_build_message_wip_carries_note 红转绿，钉三态即 wip 有 note 精确字节、wip 无 note 字节零变、settle 双态零变；本批 settle 提交信息即 settle 形带 note 自证
- F5 解析夹具「——」截断散文不入 allow：过。test_f5_parse_requested_writes_truncates_at_emdash 红转绿，夹具即司梦原例行 api/migrations/ —— 目录级声明
- F6 lease 全套绿：过。工地铁 352 passed（基线 344 加新增 8 零回归），主树真跑 351 passed 1 failed（红即 test_path_inclusion_conflict，环境性渗读见第六节同参对照四重实证非码坏）
- F7 CONTRACT 修订管线过：过。修订六十一落 CONTRACT；化格 exit 0 零改；核阅 exit 2 域外如实记（des-001 只盖 sih-engine/doc）；检词 exit 1 四笔俱存量（主树同参对照逐字节同位，本批新增文段单独核查零 findings）

## 三、四件落形 {#four-items}

- 件一 归一共形：lockcore 增 normalize_scope_path 归一式单一函数两闸共用（scope_allows 单一比对点），受检路径与 allow 条目同剥前导 ./ 与尾斜杠；commitcore.commit_staged repo_rel 点号形取仓相对裸形（越界明细同裸形）；allow 条目点号语义定仓根递归全容（归一后空串或点号即全容）
- 件二 存量兼容：legacy 显式 ./ 前缀条目归一后剥前缀等价不破，司梦实证此类现可过 commit 闸之形两闸俱过
- 件三 build_message wip 形纳 note：有 note 即空行分隔缀尾，无 note 字节零变，settle 形零变
- 件四 请求写入解析净化：parse_requested_writes 列表项遇「——」截断取路径部，理由散文不入 allow；冻结启发族语义修订走 CONTRACT 修订六十一不裸奔，DECLARATION_* 常数本体零动
- 版本位 1.44.0 升 1.45.0 三源对齐（pyproject 与 __init__ 与 CONTRACT）；sweepjson 金向量版本位重冻（正当漂移在档）；DES-016 外部只教不拒零变；first_domain 形零变；五验与链闸与 closeguard 闸序零动

## 四、套件数字 {#suites}

- lease 工地铁 352 passed 0 failed（批前基线 344，净增八测）
- lease 主树真跑 351 passed 1 failed（环境性红，非本批码坏，证据见第六节）
- 红证：tests/red/scoperoot-tdd-red.log 八测俱败在档随批提交，封装认证 ade5a474 在链承先红留痕纪律

## 五、改动文件清单 {#changed-files}

- sih-tools/lease/src/lease/lockcore.py：normalize_scope_path 新增加 scope_allows 归一共形
- sih-tools/lease/src/lease/commitcore.py：repo_rel 点号裸形加 build_message wip 纳 note
- sih-tools/lease/src/lease/core.py：parse_requested_writes 「——」截断
- sih-tools/lease/src/lease/__init__.py 与 lease/pyproject.toml：1.45.0
- sih-tools/lease/CONTRACT.md：修订六十一
- sih-tools/lease/tests/test_scoperoot.py 新件八测；tests/red/scoperoot-tdd-red.log 与 scoperoot-tdd-green.log 红绿证在档
- sih-tools/lease/tests/frozen/sweepjson/golden-report.json：版本位重冻
- sih-tools/lease/CALL-LOG.md：lease call-log append 三腿齐落一笔（收约后直改车道，候主线 campaign commit）
- 本 results 档：收约后交付件落主树候主线 campaign commit

## 六、误差申报与未决项 {#deviations}

- 任务包承载形偏差：四登记包面（TASK_PACKAGE_DIRS）俱在主线写域，簇内不可写，scoperoot.md 以路径形落工作区外 /tmp/scoperoot-pack/（resolve_package 路径形开约与收约俱用之），内容为任务书簇L节逐字转录加请求写入节逐路径分行（chainstamp 勘误正形），全文转录在本档附录；open 与 close 均 --package 路径形传递
- 写入清单补列偏差：sih-tools/lease/pyproject.toml 不在任务书簇L清单而版本三源对齐义务必需，已入 allow 与声明面随批提交，承 nomsupply nomenclator 包根 pyproject 越界补笔先例，本批以显式纳入避免 bypass；如主会不认可回退该点位
- 意图闸形偏差：治理序字面 record_intent 先于 lease open，机械实态承 BATCH-FACE 全序——意图件先落盘作 open --intent 输入（open 闸三结构校验），意图链笔在 open 后经引擎 scribe 落链（笔需会话在册），nomsupply 同序在案
- settle 双仓偏差：本簇零 sih-engine 改件（DES-016 零变、DES-014 归簇M），会话只注册 sih-tools 单仓，零 engine 工地零归并零 reconcile 义务
- 主树环境红一笔：test_path_inclusion_conflict 主树真跑红，同参对照四重实证非码坏——其一归并内容与工地铁零差（git diff 22ba3646 HEAD -- lease/ 空）；其二同 commit 临时 worktree 隔离跑 1 passed；其三失败机制即夹具 allow sih-engine/doc 前缀交叠簇M penface 会话 b8aa24a2 在飞独占锁 sih-engine/doc/design/DES-014（locks_read 读数在案）致 open 预检 open_precheck_conflict；其四即 nomsupply 已申报隔离病族同款（夹具 allow 前缀交叠渗读、放锁复绿、根治候隔离批）。簇M 放锁后主树复跑应 352 全绿
- 无主闸 bypass 一笔：三件批前存量脏件（facet/probes 两件 mtime 2026-09-10T04:11 加 mcpline/ledger/tokens.ndjson mtime 2026-09-10T08:50，俱早于本会话签发）非本批所改零触碰原样保留，候人节点按 watchcheck 协议二值裁决，承 nomsupply 同三件先例
- 差集闸认领两笔：目录级声明无尾斜杠被按文件形判（declgate is_dir 判据即尾斜杠），src/lease 与 tests 内容实已随 settle 全量提交（ls-tree 分支树可证），形判偏差 --ack-uncommitted 逐路径带事由放行入 close 回执 gates_skipped；后继批任务包目录级声明宜带尾斜杠
- 检词存量四笔：CONTRACT.md 检词 findings 四笔俱存量修订文段（词面与行位在管线读数档与认证 aac2f475 报告体在档，主树同参对照逐字节同位），非本批引入，本批新增修订六十一文段单独核查零 findings，存量候清缴批；本档转述亦以间接指称避词面复染
- 未决项：主树复跑 352 全绿候簇M放锁后主线抽查；三 pk 账出泊笔归主线；批词汇 scoperoot 候立名程序人节点终裁登记（本批零写 terms.json）；/tmp 包件与探针件（map 报告与管线读数 JSON）候归档或失性自清；results 档与 lease CALL-LOG 投影腿候主线 campaign commit

## 七、附录：任务包承载件全文 {#appendix}

批任务包 scoperoot.md 落 /tmp/scoperoot-pack/scoperoot.md，内容为 gatefix-parallel.md 簇L节逐字转录（件一至件四加 F 锚定七条），请求写入节五行即 sih-tools/lease/src/lease、sih-tools/lease/tests、sih-tools/lease/CONTRACT.md、sih-tools/lease/CALL-LOG.md、sih-tools/lease/pyproject.toml（末行补列申报见第六节）。
