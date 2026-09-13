# SPEC-024 租约融回落差规格

本规格承接用户 2026-09-13 令「租约启动融回」与 DEC-013 复用条款「下一件融回即租约」，钉死租约从围堰 sih-tools/lease 融回引擎侧的全部待建面。本规格是 SDD 产物即先于实现，TDD 批按本规格逐判据先红后绿，切换批与退役批另开。融回基准权威即围堰现行文：sih-tools/lease/CONTRACT.md 1.46.0 修订六十二现行文、sih-tools/lease/src/lease/ 十二件源码约 7834 行、sih-tools/lease/tests/ 三十四件 381 测、sih-tools/lease/ledger/ 五册台账实物。观测面缺陷先行如实申报：融回评估判据面即 gd-2 承重计量的 meter 供给 2026-09-11 后断流且历史只包书简一件，窗内租约承重计数为零，本批以用户批令留痕启动评估，承 DEC-013 门开条款即融回门开关归治理程序与用户批令留痕，评估启动不以判据全绿为前提，判据面缺陷如实登记不粉饰，计量供给修复归候裁项不归本批。

## 概览 {#overview}

- 家位与模块形即 Rust 原生实装承 DEC-013 第一步「引擎侧是开发非移植」，库模块 src/lease/ 加二进制 src/bin/lease.rs 加台账家位落 sih/ledger/ 承用户 2026-09-13 裁定::[家位与模块形](#shape)
- 接口契约对表即十九顶层子命令加 call-log 五子命令逐件对表围堰 cli.py，退出码与台账 ndjson 行形与回执 json 形零语义漂移::[接口契约对表](#interface)
- 腿切分清单即体量分腿承 SPEC-014 先例，锁核腿加 CLI 腿加附件腿三件分批实施::[腿切分清单](#legs)
- 双模并存条款即围堰原位保留为生产写位与融回基准，mcpserver passthrough 照旧，切换批换旗不预写::[双模并存条款](#dual-mode)
- 验收判据即金向量逐字节与退出码对齐与相对根不变量与迁链路标与悬置收口，六处散档待清债逐条归入::[验收判据](#acceptance)
- 金向量脏目标条款与同参形条款显式在场，净目标用真实批仪式实测冻结::[金向量脏目标与同参形条款](#golden)
- 回迁债即依赖面与 DEC-001 归位映射行与观测缺陷申报与 hooks 接锁::[回迁债](#debt)
- 测试计划 T1 至 T7 先红后绿，红转绿记录入 TDD 批结果档::[测试计划](#tdd)

## 家位与模块形 {#shape}

### 库模块 src/lease/ {#lib}

- 租约家位即 src/lease/，承 DEC-013「名随物走」纪律即命名不另起，租约之名随承诺回家，检词词条租约／lease 自 2026-08-26 已立零新增命名
- 腿切分承围堰模块边界即 core.py 台账与锁核与 CLI 解析为腿一，commitcore.py 与 sddgate.py 与 guardcore.py 收约执法面为腿二，sweepcore.py 与 calllog_import.py 附件面为腿三； Rust 侧模块划分按腿落子模块，接口宁窄勿宽
- 台账家位即裁定落位 sih/ledger/ 下五册即 sessions、locks、claims、bypass、lockface-bills：承用户 2026-09-13 裁定「账本应该放到 sih/ 里」，与 DES-015 新城域正典形即域根下 sih/ledger/ 形制统一，第一域历史册零迁移见边界节
- 域根形承 DES-015 正典：新城域台账住域根下 sih/ledger/，第一域历史映射不迁移，两形并存是登记面既定形非本规格新增

### 二进制 src/bin/lease.rs {#bin}

- 二十四个子命令面即 open、claim、unclaim、lock、unlock、status、check、close、commit、reconcile、bypass、install-hooks、uninstall-hooks、heartbeat、takeover、preempt-release、wait-turn、ledger-repair、sweep 加 call-log 五子命令，参数面逐旗标对表围堰 cli.py 777 至 1008 行定义
- 退出码三值对齐围堰：0 成、1 拦即既有执法拒且理由码原样透出、2 工具异常

## 接口契约对表 {#interface}

围堰 cli.py 与 core.py 逐项对表，零语义漂移：

- 台账五册行形逐字段对表：sessions 与 locks 与 claims 与 bypass 与 lockface-bills ndjson 行键序与字段集与空值形逐字段恒等，append-only 语义构成性，历史行不改写
- 回执 json 形即 open 与 lock 与 commit 与 close 出参逐键对表，sort_keys 加 indent 2 加 ensure_ascii 禁用加尾换行，同输入双跑逐字节一致无时间戳无随机
- 五验悲观锁语义逐条对表：在册、择定、范围、身份、绑定五验原位，撞锁即 locked_elsewhere 拒或 wait 入队受理返位次
- close 链闸与 SDDG 四判据两道门与无主闸与差集闸与归并三检语义逐条对表：锁清零、分支归并删支两态、拆本吊销
- stem 查册闸与甲表认领三件即 --claim-zh、--claim-code、--claim-derivation 机械核对语义逐条对表，nomenclator query 消费形不变
- 错误信封即 json 对象单键 error，退出码一或二按执法拒与工具异常分派，理由码字面零漂移

## 腿切分清单 {#legs}

- 腿一锁核腿即 core.py 加 lockcore.py 加 lockdb.py 加 ledgerwrite.py 四件承接，产出 src/lease/ 台账与锁核子模块加台账家位落位，验收判据 A1 至 A3
- 腿二收约腿即 cli.py 加 commitcore.py 加 sddgate.py 加 guardcore.py 四件承接，产出子命令全谱与收约执法面，验收判据 A4 至 A8
- 腿三附件腿即 sweepcore.py 加 calllog_import.py 加 hooks 三件承接，产出 sweep 与 call-log 与 git 钩子面，验收判据 A9；腿三可按需并入腿二批
- 每腿独立开 SDD 补钉批加 TDD 批，逐腿先红后绿，红转绿留痕于实现批结果档

## 双模并存条款 {#dual-mode}

- 围堰 sih-tools/lease 原位保留为生产写位：第一域全部批仪式与 mcpserver passthrough 照旧即 src/mcpserver/passthrough.rs lease_base 经 uv run 出调，围堰源码零改动即本规格红线
- 引擎 lease 是融回新增面非替换：切换批未执行前两实现并存，围堰为融回基准与金向量源，引擎件经金向量逐字节证明等价后由切换批换旗
- 切换批换旗位即 passthrough lease_base 改指引擎二进制或原生进程内调用，两形候切换批裁，本规格不预写；「行为对等基准 sih-tools/mcpline writeface」换基随切换批申报
- 并存期对表判据即同参形双跑台账行与回执逐字节一致，见金向量条款；双模不是常态，切换完成即进退役批

## 验收判据 {#acceptance}

### A1 金向量逐字节一致 {#a1}

- 引擎 lease 对金向量全组逐字节一致，含回执 json 全文与台账行全文与错误信封文本，键序缩进空值形尾换行漂移即判负返工
- 金向量构成须满足金向量脏目标条款，冻结后任何字段漂移即判负返工

### A2 退出码对齐 {#a2}

- 三值即 0 成、1 拦、2 工具异常，同输入同退出码全表对齐金向量与围堰实测

### A3 相对根不变量 {#a3}

- 归 mem-impl-t6d 三处缺陷债：commit 别名解析先 resolve 后比对相对串恒不等、close 以相对串 exists() 判工地恒假跳过拆本、绑定侧档居所随相对根漂移；引擎侧判据即 open 相对根拒收或入账即绝对化，专项红测三形冻结
- 围堰同三缺陷修不修归围堰契约修订流程非本规格范畴，显式范畴排除

### A4 版本三源一致 {#a4}

- 归 redtest-t6d 版本串债并记核销注记即原报 1.2.0 对 1.6.0 而勘明围堰现行 1.46.0 三源已一致且该债已被后续批吸收：引擎侧判据即 Cargo 版本与回执 tool.version 与 CONTRACT 三源同值，金向量含版本位

### A5 迁链路标机械位 {#a5}

- 归 wenguwire 两处加 lockguard 一处加 closeidem 一处四笔债：副件链迁移留 pk023spec 式路标副件，机械执行位随本融回实装落定；判据即 reconcile 四方对表对链零 unrouted 增量、迁链动作逐笔留路标

### A6 围堰相对根存量病测试 {#a6}

- 归 closeidem 与 lockguard 两处债：围堰布局相对根存量病四测即主线绿围堰红在引擎侧归零，相对根 fixture 形冻结入金向量脏目标

### A7 悬置会话收口 {#a7}

- 归 scribeback-t6d 债：会话 2e6489d28a719eff 悬置行收口，判据即引擎侧台账零悬置行或悬置有显式事由入 bypass 册

### A8 免参不含副件链 {#a8}

- 归 closeidem 余四证据之一：close 免参形不触副件链，链面零写入，红测冻结

### A9 机械腿不变量 {#a9}

- 引擎 lease 全程零网络零 LLM 零 key 读取；台账面唯一写点即五册只经 lease 写路径追加，零直改正本；源码扫描断言加 Cargo 依赖断言加离线可跑承 attractor T5 先例

## 金向量脏目标与同参形条款 {#golden}

承 SPEC-013 修订四教训与 SPEC-014 与 SPEC-015 两条款先例，两条款显式在场即本规格验收的组成部分。

金向量须含脏目标条款
: 净目标禁单腿。净目标即真实批仪式围堰实测冻结：fixture 域根走 open 至 lock 至 commit 至 unlock 至 close 全链回执与台账行冻结至少两会话形。脏目标至少五形即撞锁形即二会话争一路径后至者 locked_elsewhere 拒、无主闸拦形即 close 遇非本会话写件拒、差集闸拦形即 --ack-uncommitted 缺席拒、SDDG 拦形即链证缺席 close 拒、相对根形即相对根开约三缺陷复现形；另含 stem 闸拒形即未立名新词裸开拒错误信封冻结

双跑同参形条款
: A1 逐字节判据的执行条件即双侧同参形：域根一致即 fixture 逐字节同源、正身一致即 identity 报告逐字节同源即盐同值、时戳一致即 --at 显式给参同值、台账一致即双侧净册起点同形；cmp 零差与退出码一致。台账物理路径不同不算调用形差异，行内容逐字段恒等是等价性构成条件非调用形豁免

## 回迁债 {#debt}

### 依赖面 {#dep}

- 引擎侧零新增外部依赖即 serde_json 加 serde 加 sha2 均在引擎既有 Cargo 依赖面，锁库 SQLite 面承 lockdb.py 对表 Rust 侧 rusqlite 候选即依赖新增申报位，TDD 批裁决
- 围堰 Python 依赖 calllog 与 watchcheck 不随迁即附件腿自足，显式范畴排除

### DEC-001 围堰归位映射行 {#cofferdam}

- 本批归位评估行：租约锁核与台账与收约执法面贡献度评估以用户批令启动且观测缺陷见首段申报，归位至 src/lease/ 加 src/bin/lease.rs 加台账家位提案位；围堰 CLI 原位保留双模并存
- 归位三原则对表：事件只追加即链历史行不改写；旧路径不失效即围堰原路径继续有效批仪式照旧；迁移动作走批留痕即本规格批即租约融回三步曲第一步

### 观测缺陷申报 {#observation}

- gd-2 承重计量 meter 供给断流即 2026-09-11 后无计数册且历史只包书简如实登记，计量供给修复两径即重启 meter 包裹纪律或改链上认证面取数候用户裁，本规格不裁不预写

### hooks 接锁 {#hooks}

- 归 DEC-011 修订四预告「引擎融回后 commit 闸门接锁」收口位：install-hooks 与 pre-commit 与 commit-msg 三件随腿三实装，接锁语义对表围堰现行文

## 测试计划 {#tdd}

逐判据先红后绿，红态即测试先行而入口未建，绿态即实现批完成。七组如下。

T1 金向量冻结
: 围堰 lease 对净目标两会话形与脏目标五形与 stem 拒形实测输出落 src/lease/fixtures/golden/，红即 fixtures 目录不存在；金向量构成须满足金向量脏目标条款

T2 金向量逐字节一致
: 引擎 lease 对金向量全组 cmp 零差加退出码一致，红即 src/lease/ 不存在；同参形条款为执行条件

T3 退出码全表
: 三值全表各形同输入同退出码，红即 src/bin/lease.rs 未建

T4 五验与闸族语义
: 悲观五验逐条正反例加 close 链闸加 SDDG 四判据加无主闸加差集闸加归并三检加 stem 闸单测，红即锁核未建

T5 相对根与悬置与免参专项
: A3 三形加 A6 四测加 A7 悬置加 A8 免参红测，红即相对根不变量未立

T6 机械腿不变量
: 源码扫描断言加 Cargo 依赖断言加离线可跑加台账唯一写点扫描，红即网络 IO 或 LLM 调用或直写残留

T7 hooks 与附件腿
: 三钩子接锁测试加 sweep 与 call-log 对表测试，红即腿三未建

红转绿记录入 TDD 批结果档，全绿为切换批入口条件。

## 边界 {#boundary}

- 本规格不含围堰 sih-tools/lease 任何代码改动即源码零改动
- 本规格不含实现即零实现，锁核与子命令与台账家位待 TDD 批落码；台账家位已经用户裁定落 sih/ledger/，TDD 批入口条件齐备
- 围堰 CONTRACT 修订流程照旧即围堰在并存期继续演进，融回基准以切换批开工日现行文为准
- 第一域台账存量行零迁移即历史行不改写，新册启用形归切换批
- 上链前必须等绿；读 findings 不只看退出码
- 锁被他在持即报不绕行

## 规格修订记录 {#revisions}

2026-09-13 v1 随 lease-Reintegration-solo 批起草即 SDD 产物，令源用户 2026-09-13「租约启动融回」，观测缺陷 meter 断流如实申报在案。

2026-09-13 v1.1 修订一，台账家位落位定谳：提案位 sih/state/ledger/ 作废，承用户 2026-09-13 裁定「账本应该放到 sih/ 里」改落 sih/ledger/，与 DES-015 新城域正典形统一，第一域历史册零迁移纪律不动。

2026-09-13 v1.2 修订二，腿二收约执法面执行记录：批即 lease-commitlaw-parallel、队形并联形。commitcore 与 sddgate 与 guardcore 与 close_session 闸序 Rust 对表实装转绿，金向量三十二场景围堰实测冻结于 src/lease/fixtures/golden2/，集成测试五件全绿即 t5_commit 五测、t6_close_gates 五测、t6_sddgate 四测、t7_guard 五测、t2 与 t3 与 t4 回归。落差发现四处如实登记：其一围堰 date-only --at 使 issued_at 入账为 naive 形致 close 在 calllog 过渡条款比较处 TypeError 崩，findings finding-dateonly-at-treadmill-crash 在档，引擎侧 parse 归一 UTC 兼容两形，围堰修复归其契约修订流程；其二 close 期三闸 bypass 留痕行硬锚 tool_dir 真工具台账不受 --root 锚定，引擎侧改 root 锚定加 --bypass-ledger 透传；其三会话号派生式 make_session_id 为确定性 SHA-256 前十六位，引擎腿一随机 uuid 形按本批对表改派生形；其四 macOS /var 与 /private/var 别名归一入金向量同参形条款别名域。未实装面显式申报：同内容与纯追加让位归并机械即 backup 与 allow_and_merge 与 re_certify、bills SQL 投影腿、gauge_summary 全读形，归腿二后继批或切换批裁量。

2026-09-14 v1.3 修订三，腿三附件腿完工记录：批即 lease-attachments-solo、队形单线。sweep 与 call-log import 与 install-hooks 与 uninstall-hooks 四子命令 Rust 对表实装转绿，pk-103 出泊：stem 闸 root 缺省自 cwd 上溯即 core.detect_domain_context 对表且 canonical 标记先检域界即停再双仓标记，加全查路实装即 spawn uv nomenclator query 只读子进程 fail-closed 零静默，加 --new-stem 甲表三件机械核对移植承 pk-090 件五对表，T7 七测全绿即 sweep 净态与幻影修复与停滞候裁守卫与 call-log verbatim 零有损与 hooks 回环与 stem 全查拒与包缺席 skip 回归，t2 至 t6 全族回归不破。三落差如实申报：其一 call-log sqlite 索引腿未实装，Cargo 零 rusqlite 承 A9 依赖零新增，循腿二 bills SQL 同款申报，ndjson 权威腿与投影腿 verbatim 重组再生已实装；其二 sweep 僵尸锁现势锁面从 ndjson 锁册 acquired 与 released 事件序推导，围堰从 sqlite lockdb 投影读语义同构，僵尸锁 fix 落锁册 released 行而围堰走 lockdb.takeover_release INSERT；其三 import 逐行界对表 Python splitlines 即尾换行不产尾空行，Rust split 会，实装期修正在案。A9 机械腿不变量申报面：零网络零 LLM 零 key 读取成立，nomenclator query 为本地子进程；takeover 与 ledger-repair 与 heartbeat 与 wait-turn 与 preempt-release 独立子命令未实装，sweep fix 通道内联覆盖 takeover 与 ledger-repair 两形，余三候后批按需。

## 内容充分性 {#sufficiency}

- 本节循 SPEC-TEMPLATE-sufficiency-v1 形，只加节不改本文实质。
- 判据红证对表：本文金向量脏目标条款五形即判据红证载体；待清债九处散档引文即回迁债红证载体：closeidem-solo-results.md:33、lockguard-solo-results.md:23 与 33、mem-impl-t6d-results.md:56、redtest-t6d-results.md:43、scribeback-t6d-results.md:35、wenguwire-solo-results.md:15 与 35；其余判据零信息部分如实申报，清账路径为后继修订批逐件补红证。
- 判定性常数挂锚对表：本规格无声明的判定性常数；GATE_SINCE 与合同参数面承围堰 CONTRACT 修订六十二现行文不重申。des-001 C007 与 C008 与 C009 行级与邻近级闸在役核验。
- 约束算子对表：本文无约束算子面，如实申报。
