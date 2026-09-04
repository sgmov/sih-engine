# SPEC-015 谓词融回落差规格

本规格承接 GOV-002 退出标准判据二路择谓词件经可插拔机制融回判定器模块与判据三按轮判定的截流谓词族融回三问模块与判据五引擎 task-packages 内 facet 过程件归零，钉死路择谓词机从围堰 sih-tools/selector 融回引擎侧的全部待建面。本规格是 SDD 产物即先于实现，TDD 批按本规格逐判据先红后绿。融回基准权威即围堰现行文：sih-tools/selector/CONTRACT.md 修订三现行文、sih-tools/selector/src/selector/{predicates,route,pack,cli}.py 四件源码、sih-tools/selector/packs/{core,parking}/ 两包四件纯数据。凭据即 predsplit-a1 与 predsplit-a2 与 predsplit-a3 三枚 stable_clear 终签在链，2026-09-02 当日链 161 至 164 位含 predmerge-refine-b1 共四枚，件一过裁直执行。

## 概览 {#overview}

- 家位与模块形即 src/attractor/route.rs 谓词机库模块加 src/attractor/packs/ 纯数据随迁加 src/bin/attractor.rs 增 route 子命令，可插拔机制即规则包形态承 scrutinator packs 先例::[家位与模块形](#shape)
- 接口契约对表即围堰 route 子命令参数面与报告 json 形态与退出码三值逐项对表，报告头 tool 字段承契约面字面，零语义漂移::[接口契约对表](#interface)
- 谓词机对表即十谓词 kinds 与 fail-closed 语义与 route_on_fail 缺省与批级告警四件逐条对表::[谓词机对表](#predicates)
- 截流谓词装配位即 src/ask3repeater/intercept.rs 两公共函数，形式承 selector predicates.py 谓词形态与 GOV-002 判据三文本，接口宁窄勿宽::[截流装配位](#intercept)
- 双模并存条款即围堰 selector CLI 原位保留，心跳调用形照旧，围堰源码零改动::[双模并存条款](#dual-mode)
- 验收判据即金向量逐字节与退出码对齐与谓词机语义零漂移与机械腿不变量与多包归因::[验收判据](#acceptance)
- 金向量脏目标条款与同参形条款显式在场，净目标用真实材料跑围堰实测冻结::[金向量脏目标与同参形条款](#golden)
- 回迁债即依赖面与 DEC-001 归位映射行与域声明装载边界::[回迁债](#debt)
- 测试计划 T1 至 T6 先红后绿，红转绿记录入 TDD 批结果档::[测试计划](#tdd)
- 过程件归零即判据五执行位，归档迁移形机械判定::[过程件归零](#zero-proc)

## 家位与模块形 {#shape}

### 库模块 src/attractor/route.rs {#lib}

- 路由谓词机家位即 src/attractor/route.rs，承 SPEC-014 attractor 家位纪律即库与二进制与检词登记同位，融回件依赖闭包全落在引擎既有面
- 三段职能：pack 装载即 manifest.toml 加 routes.toml 全量校验、谓词判定即十 kinds 逐件求值、三路输出即 mainline 加 siding 加 scrap_track 路由与批级告警装配
- 纯数据随迁即 src/attractor/packs/{core,parking}/ 两包四件自围堰逐字节复制零改动，四件即 manifest.toml 加 routes.toml 各两件，包是数据非代码即随迁是复制不是重写
- 可插拔机制即规则包形态：判定知识全在包不在代码，工具本体零内置谓词零内置路由，承 scrutinator packs 与围堰 selector 空腹纪律先例
- 判定器模块归属即 route 落 src/attractor/ 是判定器 attractor 的谓词装载与判定入口，GOV-002 判据二字面「经可插拔机制融回判定器模块」的落位形

### 二进制 src/bin/attractor.rs 增 route 子命令 {#bin}

- attractor 七子命令即既有六子命令加 route，参数面逐旗标对表围堰 cli.py
- 退出码三值对齐围堰：0 路由毕无告警、1 路由毕有告警、2 包非法或材料缺失或解析失败

### 报告工件 {#report}

- route 报告 json 即唯一工件，落盘形承围堰即 sort_keys 加 indent 2 加 ensure_ascii 禁用加尾换行，同输入双跑逐字节一致无时间戳无随机
- 报告头 tool 字段承契约面字面即 name selector 加 version 0.4.0：工件工具名字段是契约面标识非二进制身份，承引擎 tally 核对报告 tool 字段即 tally 先例，二进制身份在调用形与调用册承载
- 报告头 pack 字段承包声明即 name 加 version，domain 字段承 manifest 域声明原样

## 接口契约对表 {#interface}

围堰 cli.py 逐项对表，零语义漂移：

- 子命令名即 route，参数即 --pack <包目录> 必填加 --reference-time <ISO 日期> 可选加 MATERIAL 位置参数可多个
- MATERIAL 解析即逐个路径为文件则收为材料、为目录则展开其下全部 json 子件按文件名排序、缺席即退出码二；零材料即空批为合法绿态零路由零告警退出码零
- --reference-time 给参时校验 ISO 日期形，非法即退出码二；给参时报告头增 reference_time 键，未给参不增即旧输出形态不变
- 报告顶级四键即 header、routed、summary、status 字面 routed；header 三键即 tool、pack、domain
- 逐材料条目五键即 id、path、route、failed_predicate、checks，checks 逐谓词 id 加 pass 布尔按包序；轮记录即 round 键为对象时条目增 round 真，停泊材料即 parking 键为对象时条目增 parking 真，普通材料不携带
- summary 五键即 total、mainline、siding、scrap_track、alarms；登记冲突告警在场时增 conflicts 键载件数，在泊超期告警在场时增 agings 键载件数，无产出不增键即旧输出形态不变
- 材料装载即顶层须为 json 对象，非法或非对象即退出码二整批拒收不静默
- 错误信封即 json 对象单键 error，sort_keys 紧凑形，退出码二

## 谓词机对表 {#predicates}

十谓词 kinds 与围堰 predicates.py 逐条对表，fail-closed 语义零漂移。单件材料判定族四件查材料顶层五字段：schema_required 查 fields 列字段在且为 str；state_annotation 查 state 在 allowed 枚举；anchor_whitelist 查 anchors 逐项 fnmatch 白名单模式空数组过；write_boundary 查 requested_writes 逐项 fnmatch allowed_roots 空数组过。按轮判定族四件查 round 键内结构，round 键缺省或非对象或轮内数组非法一律判败即 fail-closed：anchor_density 查断言有效锚占比达 density_threshold 零断言判过，有效锚即 evidence 条目路径前缀拼 resolve_root 真实存在，条目形「路径」或「路径:行」冒号尾段数字取前缀；standing_constraints 查回执 tool 集覆盖 required_receipts 全部；assertion_lists 查每断言 evidence 数组在场非空只查结构；registry_conflict 恒过非路由位，产出批级告警。时间维度族两件查 parking 键内时序，parking 键缺省或非对象或字段非法一律判败即 fail-closed：time_deadline 参照时间越过 entered_at 加 ttl_days 判败到期日当日即败，参照时间显式给参不读系统钟；parking_aging 恒过非路由位，产出批级告警。

- route_on_fail 缺省四件即 anchor_density 与 standing_constraints 与 assertion_lists 与 time_deadline 缺省 siding，其余单件族必填；registry_conflict 与 parking_aging 禁配 route_on_fail 配即包非法
- params 解析按 kind 专属：density_threshold 数值缺省 1.0 布尔拒、resolve_root 必填非空 str、required_receipts 非空 str 数组、aging_threshold_days 正数、assertion_lists 与 registry_conflict 与 time_deadline 无参数、四记录族 KIND_PARAM 单键非空 str 数组
- 三路枚举即 mainline 加 siding 加 scrap_track，pass_route 必在枚举内；告警阈值两件非负整数
- 包校验错误全量即包目录缺席、两 toml 缺席或非法、manifest 字段非法、kind 不在十 kinds、route_on_fail 非法、params 非法、defaults 或 alarms 缺席或非法，全部拒包报错
- 判定序即包内声明顺序逐条评估首败定路，全过走 pass_route；逐材料输出全部谓词过败记录
- 批级告警四件即 siding 件数达 siding_surplus_threshold 有余、mainline 件数不高于 mainline_starvation_threshold 饥饿、registry_conflict 逐结论查 establishes_term 撞回执 flagged_terms、parking_aging 参照时间与 entered_at 差值达 aging_threshold_days 逐件；后两件以包内声明对应谓词为门，无声明零产出；空批零告警防空批饥饿误报；告警入报告并置退出码一，告警是事实不是动作不改路由不拦批

## 截流装配位 {#intercept}

- 家位即 src/ask3repeater/intercept.rs，三问模块新增子模块，GOV-002 判据三字面「按轮判定的截流谓词族融回三问模块」的落位形
- 接口宁窄勿宽只两公共函数：装配位即 load_intercept_pack 读谓词包目录返回已校验包，按轮判定即 round_interception 对轮记录逐谓词求值返回过败记录序列
- 形式承 selector predicates.py 谓词形态即七谓词 kinds 在场于两包四件纯数据，core 四谓词与 parking 三谓词，另承 GOV-002 判据三文本，判定语义不复实现即截流装配位调用 src/attractor/route.rs 谓词机，单一实现双接位
- 三问既有行为零改即 validate 与 gate 与 record 全不动，装配位是接口与测试承载，按轮判定接入三问会话起点与逐轮闸位的调用归后续批承载，本规格只钉装配位本体
- 截流谓词族的围堰侧对应物是按轮判定四 kinds 加时间两 kinds，装配位不设谓词白名单即包内声明什么评什么，空腹纪律同源

## 双模并存条款 {#dual-mode}

- 围堰 selector CLI 原位保留，泊界心跳与例行调用形照旧，围堰 selector 与 facet 与 tally 源码零改动即本批红线
- 引擎 route 是融回新增面非替换：切换批未执行前两实现并存，围堰为融回基准与金向量源，引擎件经金向量逐字节证明等价后由切换批换旗
- 并存期对表判据即同参形双跑逐字节一致，见金向量条款；双模不是常态，切换归切换批不预写

## 验收判据 {#acceptance}

### A1 金向量逐字节一致 {#a1}

- 引擎 route 对金向量全组逐字节一致，含报告 json 全文与错误信封文本，键序缩进空值形尾换行漂移即判负返工
- 金向量构成须满足金向量脏目标条款，冻结后任何字段漂移即判负返工

### A2 退出码对齐 {#a2}

- route 三值即 0 无告警、1 有告警、2 包非法或材料缺失或解析失败，同输入同退出码全表对齐金向量与围堰实测

### A3 谓词机语义零漂移 {#a3}

- 十 kinds 求值语义与 route_on_fail 缺省与禁配与批级告警四件与判定序首败定路逐条单测对表围堰行为
- 包校验错误全量逐类拒收，错误信封形态对表

### A4 机械腿不变量 {#a4}

- src/attractor/route.rs 与 src/ask3repeater/intercept.rs 与 src/bin/attractor.rs route 位全程零网络零 LLM 零 key 读取零目标仓写入
- 源码扫描断言加 Cargo 依赖断言加离线可跑承 attractor T5 先例

### A5 多包归因 {#a5}

- 同一真实材料集分别经 core 与 parking 两包路由，两报告各自冻结为金向量，归因逐字节即同件异包各归各路
- 随迁两包数据与围堰原包逐字节一致即归因等价的构成性条件

### A6 截流装配位与三问零伤 {#a6}

- intercept 两公共函数走 route 谓词机有测试承载
- 三问既有测试零改全绿即 validate 与 gate 行为零改的机械证明

## 金向量脏目标与同参形条款 {#golden}

承 SPEC-013 修订四教训与 SPEC-014 两条款先例，两条款显式在场即本规格验收的组成部分。

金向量须含脏目标条款
: 净目标禁单腿。净目标即真实材料跑围堰 selector 实测输出冻结：task-packages 侧用 sih-tools/selector/baseline/process-files 真实任务包评价材料至少六件经 core 包路由，parking 侧用 sih-tools/parking/materials 真实在泊材料全量经 parking 包路由。脏目标至少三形即 schema 缺字段形即缺 state 判 R001 败走 scrap_track、域外路径形即 anchors 出白名单判 R003 败走 siding、时间到期形即 parking 材料参照时间越 ttl 判 time_deadline 败走 siding；另含包非法形即未知谓词 kind 拒包错误信封冻结。多包归因基线即同材料集双包双报告冻结见 A5

双跑同参形条款
: A1 逐字节判据的执行条件即双侧同参形：包数据一致即随迁包与围堰包逐字节一致、材料一致即输入 json 逐字节同源、参照时间一致即 --reference-time 同值同形、绝对路径形态即两侧均以绝对路径运行；cmp 零差与退出码一致。包目录物理路径不同不算调用形差异即报告不载包路径，随迁数据逐字节一致是等价性构成条件非调用形豁免

## 回迁债 {#debt}

### 依赖面 {#dep}

- 零新增依赖即 toml 加 serde_json 加 serde 均在引擎既有 Cargo 依赖面，route 依赖闭包不触网络族不触 LLM 族
- 域声明装载边界即 manifest domain include 与 exclude 装载入报告头原样，不做逐材料域判定，承围堰行为字面即域是治理域声明入报告头非路由判定位，此为显式范畴排除非默认沉默

### DEC-001 围堰归位映射行 {#cofferdam}

- 本批归位评估行：selector 谓词机与包装载与路由输出贡献度评估合格，predsplit-a1/a2/a3 三枚 stable_clear 在链即贡献度实证，归位至 src/attractor/route.rs 加 src/attractor/packs/ 加 route 子命令加截流装配位；围堰 CLI 原位保留双模并存
- 归位三原则对表：事件只追加即链历史行不改写；旧路径不失效即围堰原路径继续有效心跳照旧；迁移动作走批留痕即本规格批即谓词融回三步曲第一步
- 切换与退役归切换批即 T6 管线与心跳调用面换旗另批承载，本规格不预写退役细节

### 过程件归零 {#zero-proc}

- 判据五执行位即 sih-engine/task-packages/f-anchors-x11-t6d.md 归档迁移至 sih/event/plan/ 过程件档，机械判定形即归零后 task-packages 目录零 facet 过程件
- 迁移动作即复制入档删原位，归档件内容零改，落位路径 sih/event/plan/f-anchors-x11-t6d.md，TDD 批执行并留实证

## 测试计划 {#tdd}

逐判据先红后绿，红态即测试先行而入口未建，绿态即实现批完成。六组如下。

T1 金向量冻结
: 围堰 selector 对净目标与脏目标三形与包非法形与多包归因材料实测输出落 src/attractor/fixtures/route/golden/，红即 fixtures/route 目录不存在；金向量构成须满足金向量脏目标条款

T2 金向量逐字节一致
: 引擎 route 对金向量全组 cmp 零差加退出码一致，红即 src/attractor/route.rs 不存在；同参形条款为执行条件

T3 退出码全表
: route 三值全表即 0 加 1 加 2 各形同输入同退出码，红即 src/bin/attractor.rs route 子命令未建

T4 谓词机语义与多包归因
: 十 kinds 逐条正反例与 fail-closed 与缺省败向与告警四件与判定序与包校验全量单测，红即谓词机未建

T5 机械腿不变量
: 源码扫描断言加 Cargo 依赖断言加离线可跑，红即网络 IO 或 LLM 调用残留

T6 截流装配位与三问零伤
: intercept 两函数测试加三问既有测试零改全绿，红即装配位未建

红转绿记录入 TDD 批结果档，全绿为切换批入口条件。

## 边界 {#boundary}

- 本规格不含围堰 selector 任何代码改动即源码零改动，包纯数据随迁是复制不是改动
- 本规格不含实现即零实现，route 模块与装配位与子命令待 TDD 批落码
- 域逐材料判定不做即域声明入报告头，范畴排除显式声明见回迁债节
- 谓词族扩容即新 kinds 增加归围堰契约修订流程，本规格只随迁现行十 kinds 不扩
- 上链前必须等绿；读 findings 不只看退出码
- 锁被他在持即报不绕行

## 规格修订记录 {#revisions}

2026-09-02 v1 随 autoflow2-solo 批起草即 SDD 产物，凭据 predsplit-a1/a2/a3 三枚 stable_clear 在链。

## 内容充分性 {#sufficiency}

- 本节为 docmath-b4-solo 收尾批按新旧都管裁定补齐，模板见 SPEC-TEMPLATE-sufficiency-v1，只加节不改本文实质。
- 判据红证对表：本文对拍与冲突样本节即判据红证载体；其余判据零信息部分如实申报，清账路径为后继修订批逐件补红证，承 sih-math/docs/docmath-carriers-derivation-2026-09-04.md。
- 判定性常数挂锚对表：无声明的判定性常数。des-001 C007 与 C008 与 C009 行级与邻近级闸在役核验。
- 约束算子对表：本文无约束算子面，如实申报。
