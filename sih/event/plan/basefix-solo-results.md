# basefix-solo 结果档

> 批：basefix-solo 地基修复四件（locks 互斥绕穿修复、scribe 追加原子化、lease close 并集复查、泊界投影同步），行为变更批 SDD 先行 TDD 先红后绿
> 会话：1a70a4e901564b09（租约自生成）｜会话标识 sess-zcode-260904-basefix（ask3 双标识空间，各认各的）
> 日期：2026-09-04 ｜ 队形：单线形 solo，零子代理 ｜ 三仓工地 tools@integral-stage-build 与 engine@main 与 math@main
> 承接：pk-051（locks 判定面偏差两笔，leasewire-solo 实查申报 as-is 冻结）与 pk-045 样本库 trail 竞态类两起实案（facepark-solo 丢两笔、leasewire-solo close 丢 pk050sw 认证一笔已由主会补笔复原 7e1aa2）与 DEC-013 融回序位注记
> 政策行：用户 2026-09-04 令源裁定「pk-051 修复插到载体接线前，并行批安静窗口开工」——本批预期零人节点裁决点，机械链全绿自行收口，结果档不设「等你令」节
> 意图哈希：ae01bdea（ask3 双门过：核阅 exit 0 零违规、重放 status ok 三锚；digest passed covered 6；intent 事件 ae01bdea）

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| F-1 locks 互斥绕穿修复 | 完成 | active_locks 单遍事件序配对加 normalize_path 资源同一化（root 参），SDD 钉死互斥不变式于 SPEC-018，TDD 两测先红后绿 locks 10 绿，l4 金向量判变重冻双跑 IDENTICAL（as-is 旧向量留档披露不删），推导档 basefix-derivation-2026-09-04.md 在档 |
| F-2 scribe 追加原子化 | 完成 | SDD 裁形进程间文件锁（flock）择于单写原语三依据申报于 SPEC-018，append 位锁覆盖读算写全程锁内重读现行链，CLI append 位有界重试三笔，t7 双进程竞测先红后绿，引擎全套 208 绿零回归（存量红一笔批前同态），事件 schema 与哈希公式零触碰 |
| F-3 close 并集复查 | 完成 | 归并前重读现行链重算并集超集：纯追加 ndjson 活面自备份让位归并并透出 re_certify_hashes 补笔清单，真分叉维持整批拒，TDD 先红后绿 lease 88 绿，BATCH-FACE 勘误续一行（close 复跑前必复查并集，代码闸已加），lease 1.17.0 三源对齐 |
| F-4 泊界投影同步 | 完成 | 名册照链补齐 pk-048 与 pk-049 与 pk-050（补入泊事件号）与 pk-051 四行，在泊计数六改八，历史十二改十三，pk-051 出泊行 promoted（出泊事件 f6b3293e，本批 F-1 即出泊条件路径），pk-046 行 SPEC-017 号位撞注记照录 pk-043 与 pk-047 先例，materials 补 pk-051.json 逐字段抄链上停泊事件，名册过化格核阅检词三门 |
| F-5 判变申报 | 完成 | 见本档判变申报节 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 | 互斥不变式任一时刻任一路径至多一持锁会话；三态路径同一化；SDD 钉死；TDD 红绿；l4 重冻申报 | 过 | SPEC-018 互斥不变式与资源同一化两节；红证即 s2 再取 granted 与绝对形 scope_violation，绿证即 locked_elsewhere；金向量 b2 改判 b3 三态 |
| F-2 | 并发安全形；双进程竞测红绿；schema 与哈希零触碰；全套零回归 | 过 | flock 单写临界区；t7 红两形态（parse 损坏与时间戳拒）绿 12 事件 valid；diff 只在写入并发形与 CLI 重试位；208 绿 |
| F-3 | 归并前重读现行链重算并集超集；红绿；勘误一行 | 过 | _chain_append_grown 分类加备份让位加补笔清单；红证即事故形整批拒逼出人工让位复跑窗口，绿证即自让位出清单；勘误续节 |
| F-4 | 名册照链逐字段；出泊记账随批；三门全过 | 过 | 入泊事件 1b563a66 与 752aa64a 与 0b3da7ec 与 97ee4c20、出泊事件 f6b3293e 全数照链；化格 0、检词 0、核阅域外如实记 |
| F-5 | 判变只向前生效历史账目零改写 | 过 | 判变申报节五笔 |

## 三、SDD 裁形申报（SPEC-018 摘要）

1. **F-2 文件锁择于单写原语**：零新增常驻进程与传输层即新增失效面最少（flock 经 extern 声明直调，Cargo 依赖零增，进程死亡锁自动释放）；读算写三段同一临界区即链尾 prev_hash 计算与落写之间无窗口；语义零变。单写原语引入新常驻组件与进程间协议违工程基线第五条。
2. **F-3 扩集受限形择于纯拒**：纯追加活面由 close 自备份让位归并并透出补笔清单（扩集含两方事件经书简补笔通道达成），链写单一通道红线不破；真分叉非纯追加维持整批拒零动作。

## 四、金向量读数（判变申报位）

- 三件落 sih-engine/sih/event/plan/basefix-solo-materials/（golden_cases.json 输入内联、replay_golden.py 重放器经 --locks-src 传源路径、basefix-solo-golden-vector.json 冻结向量，formula_version bf-1）。
- 四场景：b1 放后重取持位、b2 l4 重冻（as-is granted 改判 locked_elsewhere）、b3 路径三态同一、b4 常规序配对回归。
- 双跑读数：工地修复源两遍 cmp IDENTICAL；归并前主树旧源重放 differs（修复未在主树，预期）；主树已提交源复现关随 close 后补跑记入收口附记。
- as-is 旧向量（l4 两遍配对 W2 granted 形）留档披露于 leasewire-solo-materials 不删，payload_sha256 1dae0857 前缀可对。

## 五、管线读数（化格→核阅→检词，笔在核前，findings 亲读）

| 目标 | 化格 | 核阅 | 检词 |
|---|---|---|---|
| SPEC-018 加名册 PARKING-v1 | exit 0 无需改 | exit 2 域外如实记（工地路径在 des-001 域外） | exit 0 零违例 |
| 推导档（sih-math/docs） | exit 1 已修改即归一落笔 | exit 2 域外如实记 | exit 0 零违例 |
| BATCH-FACE 加两 CONTRACT 加两 CALL-LOG | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| pk-051.json 加 materials 三件加任务包加 dispatch | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| replay_golden.py | exit 2 py 域外如实记 | exit 2 域外如实记 | exit 0 零违例 |
| 域内对照件主树 GOV-003（PARKING-v1.md doc 位） | — | exit 0 findings 0（采集面在位佐证） | — |

零管线违规零返工；归并后主树复验（SPEC-018 与名册核阅与金向量复现关）随收口附记补跑。

## 六、测试对表（批前批后零回归）

| 工具 | 批前（主树） | 批后（工地） | 判定 |
|---|---|---|---|
| locks | 8 passed | 10 passed（+2 新测先红后绿） | 零回归 |
| lease | 86 passed | 88 passed（+2 新测一红一界） | 零回归 |
| sih-engine | 207 passed 1 failed | 208 passed 1 failed（+1 即 t7） | 零回归 |

存量红申报：f2_refs_machine_verifiable（mem_recall_f_suite）批前即红，摘录截断前段不在行区间 pk050sw-solo-results.md@3-9，pk050sw 批结果档行漂移所致，非本批回归，归后批词表批同款归后继修复通道。红绿实录：F-1 两测红证（s2 再取 granted 即互斥绕穿实演；绝对形 scope_violation）与 F-3 红证（事故形整批拒）与 t7 红证两形态（parse line failed 互缴损坏；TimestampNotMonotonic）全数在案。

## 七、判变申报（F-5）

1. **locks 行为变更只向前生效**：active_locks 单遍配对与 normalize_path 资源同一化自本批合并时点对新操作生效，历史链文件与已收批账目与历史锁操作记录零改写零重算；批前历史判定位以工具旧版实查为准（双标识如实并存）。
2. **l4 金向量判变重冻**：as-is 偏差形旧向量留档披露于 leasewire-solo-materials 不删（零触碰），修复后形态重冻于 basefix-solo-materials，过门判据即 l4 场景 locked_elsewhere 与锁面读出全——达成。
3. **scribe 追加原子化语义零变不涉判变**：事件 schema 与哈希公式与退出码三值零触碰，diff 只在写入并发形与 CLI 重试位；重试位申报为新增机制非语义变更（拒绝路径仍为后手保底）。
4. **close 并集复查行为变更**：纯追加 ndjson 活面从「前置态真分叉整批拒」改「自备份让位归并加补笔清单」，写面封闭测试增豁免位一处（dest.write_text 唯一且落点限 .close-backups 子树），只向前生效。
5. **locks 零行为窗口终止**：leasewire 推导档 4.1 与 4.4 两处「修法归后续行为变更批」申报由本批闭环，locks 判定面与 lease 侧两先例三源对齐。

## 八、认证清单（逐笔 meter 包裹引擎 scribe append，闸三 --session 加 --sessions 全带）

| 件 | 事件哈希前八位 |
|---|---|
| intent（ask3 记录 + 验证件） | ae01bdea |
| pk-051 出泊 | f6b3293e |
| pipeline 管线读数 | 9612ddba |
| derivation 推导档 | 13132034 |
| golden 金向量读数件 | f8c88f73 |
| golden 冻结向量哈希件 | 659036ea |
| changed-files 变更件 | b9b0b8c7 |
| checkcite 书单对表 | 88765ae7 |

温故检索：三词召回（全序资源分配与死锁自由加版本偏序与外化状态存储加锁面一致性），词通道 ORD-020 与 ORD-019，并集书单 allowed_size 60；checkcite pass（数学引用域内 ORD-019 与 ORD-020 零 missing，recall-basefix.json 落 materials 如实记；首次全量跑 SPEC-014 至 018 五笔被抽取器捕入即引擎规范体例引用非数学引用，如实申报后按数学引用域内重跑）。

## 九、越线与误差申报

1. 机械链序前置位写面四笔即 ask3 生成器与 ask3 记录与叩问信号（scribe/reports 面）与正身件（identity/reports 面）写于会话签发前，面锁不可得即事实直写，如实申报（leasewire 先例同形）；零共享冲突（批期活锁零）。
2. t7 首版两笔施工误差如实申报：其一测试进程参数 sh -c 位错位（$0 占位缺位致夹具路径串位），补占位修复；其二测试用例首版断言 held==1 误计行数（同会话幂等重取两 acquired 行均在锁为契约语义 L5），改断言唯一持位方——两笔均在工地内修正，零工地外影响。
3. checkcite 首跑 verdict fail 五笔（SPEC-014 至 018 被抽取器捕入），属引擎规范体例引用被误作数学引用，按数学引用域内重跑 pass，首跑件如实留存于本节申报。
4. 核阅域外 exit-2 十四路（工地路径全在 des-001 域外）如实记入档，域内对照 GOV-003 exit 0 佐证采集面在位，不属违规；归并后主树复验随收口附记补跑。
5. 例行读数三笔（gauge 三维快照 reading_recorded）与本批 intent 前入链，属会话启动义务非施工写入，如实申报。
6. 租约 ledger 面零直写：sessions 与 locks 与 claims 三册全部经 lease 工具追加即唯一写点承载。

## 十、冲突样本节（pk-045 样本库）

本批批期活锁台面：开工时活锁零（主会已核，窗口成立）；批期九把 exclusive 锁一射取获零重试零撞锁；共享追加面（trail）append 短持即取即放八笔零等待（intent 一笔加出泊一笔加认证六笔；另一次 derivation 首试报告件未建即拒 rc=2 零留痕）；零并行施工批交锋，pk 号零新占用（pk-051 出泊消费）。t7 双进程竞测为本批刻意制造的冲突样本即并发无锁写互缴链损坏与同尾双算 prev_hash 分叉两形态，机械响应即 flock 临界区加锁内重读加有界重试，解决路径已固化于 SPEC-018 与 tdd_tests。

## 十一、收口读数（认证时点）

- 链：sih-engine/sih/event/trail/2026-09-04.ndjson 认证时点 verify valid（批前 149，例行读数三笔在批前计入；本批 intent 一笔加出泊一笔加认证六笔（golden 两笔即读数件与冻结向量哈希件）；首哈希与批前一致零分叉）。
- 锁：九把 exclusive 在持（locks 全域加 lease 全域加引擎 event_stream 加 scribe.rs 加 BATCH-FACE 加名册加 doc/spec 加 sih-math/docs 加任务包），settle 后放锁收约。
- 工地：三仓批件落齐（tools 九件修改；math 推导档；engine 源码三件加 SPEC-018 加名册加 pk-051.json 加 materials 拷贝与结果档与任务包与 dispatch，trail 快照 settle 前一次性拷）。
- close 与 reconcile 与终态 verify 读数与三仓提交号见文末收口附记（close 后补记）。

## 十二、队形验证

单线形 solo 零子代理全程成立：本批全部写入由会话 1a70a4e901564b09（sess-zcode-260904-basefix）亲写，零 Agent/Task 子代理调用；互斥修复与原子化与并集复查与投影同步全由确定性程序承载（pytest 红绿、cargo test 双进程竞测、cmp 双跑、checkcite 书单对表），链写入经引擎 scribe 闸三（--session 加 --sessions）零直写链文件。

## 收口附记（close 后补记）

（认证哈希回填与 close 归并与 reconcile 与 verify 与三仓提交号与主树复验读数随 close 后补记。）
