# SPEC-013 核阅融回落差规格

本规格承接 sih-tools/scrutinator/CONTRACT.md 工具契约与 DEC-013 融回门机制，钉死核阅从围堰外切态融回引擎侧的全部待建面。本规格是 SDD 产物即先于实现，TDD 批按本规格逐判据先红后绿。工具侧现状权威即工具 CONTRACT.md 第 17-33 行六件机器形态与第 35-47 行五判据与第 49-51 行融回门三查。

家位令源：库模块路径选 src/scrutinator/ 与二进制同名，承检词死档 discipline 即相关英文同形词已被 PRO-007 死档登记，库与二进制同位精简路径承 DEC-001 基础设施层归位。

## 概览 {#overview}

- 引擎侧家位即 src/scrutinator/ 库模块加 src/bin/scrutinator.rs 命令行面，承 SPEC-006 书简融回先例::[家位与模块形](#shape)

- 接口契约对表即工具 CONTRACT.md 六件机器形态与五判据与融回门三查逐条对表，接口语义不因搬迁而变::[接口对表](#interface)
- 规则包家位即 manifest.toml 加 rules.toml 纯数据随件迁引擎侧，三包 des-001 0.1.0 与 des-001-mathe 0.3.0 与 ask3 0.1.0 全量迁，工具侧留档不删::[规则包迁移](#packs)
- 验收判据即同包同目标下引擎件与工具件输出 JSON 报告逐字节一致、退出码三值一致、报告 content_hashes 字段与 scribe append 认证位兼容::[验收判据](#acceptance)
- 回迁债即 TOML 解析依赖、双跑对表基线、DEC-001 围堰物理对应归位映射的核阅行::[回迁债](#debt)
- 引擎侧即基础设施层落点，承 DEC-007 基础设施层归属判据即不承接哲学命题与提供运行前提::[基础设施层归属](#infra)

## 规格修订记录 {#revisions}

2026-08-31 修订一，scrutmerge-tdd-solo 批三处规格修正走本批管线：
1. mathe 包规则清单重数。原批一规格写「共十四件」即承 des-001 十二件加 mathe 新增二件；实测 rules.toml 实有二十条 [[rules]]：承 des-001 十二件（C001 C002 C006 S002 S004 S005 S006 F000 F002 F003 F005 N002）+ mathe 新增八件即 M008 H1 prefix 合法集加 M010 与 M010a 至 M010f 共七件 facet 启发式禁词。本规格以二十件为基线，并把承继重声明关系写清。
2. 两「验收判据」节并节消歧义。原规格行 137 ## 验收判据 与行 218 ## 验收判据 同名双节；批二规格合并为单节 ## 验收判据 即 A1 至 A6 六条。
3. 规则包装载方式定案。详见 § 规则包装载方式节，取舍理由即工具件 Python 运行时路径读 TOML 与 Rust 编译期内嵌对比。

2026-08-31 修订二，scrutmerge-tdfix-solo 批 CLI 双形与空载形声明：
1. 命令行双形。位置参数目标为正典工具兼容形即 scrutinator --pack <包> <目标> [<目标>...]，与工具件 sih-tools/scrutinator 的位置参数目标形态同形；--target 旗标保留为引擎别名，与位置形并存；两形同包同目标同路径串输出逐字节一致，验证即双跑 cmp 对表加 cargo test 用例 cli_positional_and_flag_forms_byte_identical。切换映射一句：调用点由 --target 旗标形整体替换为位置参数形，即删 --target 字面留其值、值位不变；两形并存期间同传即按出现顺序取并集。
2. 空载形。--pack 取空值即空载形，零规则包装载，产 packs 空数组报告与发现零条与退出码零，实装位即 § 接口对表 第一件空腹对表 引擎侧行，验证即 cargo test 用例 exit_empty_load_zero。

## 家位与模块形 {#shape}

### 库模块 src/scrutinator/ {#lib}

- 子模块三类即 rule 规则加载与解析、report 报告生成、cli 命令行解析
- 子模块不允许内嵌规则知识即规则全在数据，承 CONTRACT.md 第 17 行第一件空腹纪律
- lib.rs 导出 public API 即 run 入口签名与三类型即 EngineReport / PackManifest / RuleEntry
- 公共函数四件即 load_packs 加 run_engine 加 render_report 加 build_manifest
- 引擎侧不受句读零第三方约束即允许引 toml crate，区别于工具侧

### 命令行 src/bin/scrutinator.rs {#cli}

- 二元承 ask3repeater 先例即代码标识符小写 scrutinator 不另起，承 DEC-006 本名回滚
- 子命令承 CONTRACT.md 第 33 行 CLI 契约最小形态即 scrutinator 命令加可重复的 --pack 加可重复的 --target，无网络调用无任何文件写入
- 退出码三值对齐工具侧即零合规、一违规、二工具自身异常
- 输出 JSON 必含引擎版本加全部已加载包版本加目标内容哈希加发现数组，承 CONTRACT.md 第 27 行第四件 json 报告

### 治理名与代码标识符分离 {#naming}

- 治理名核阅不变，承 CONTRACT.md 第 11 行词条
- 代码标识符承 DEC-006 本名回滚即代码标识符小写 scrutinator，承 DEC-007 子决策三核阅承接鉴的反映职能
- 席位参验承 DEC-007 即核阅是组件层已立名实例，DEC-013 融回只搬实现不改接口

## 接口对表 {#interface}

工具 CONTRACT.md 六件机器形态与五判据与融回门三查逐条对表如下，零语义漂移。

### 第一件空腹对表 {#hunger}

- 工具侧：引擎不内置任何规则，运行时从规则包加载，无规则包即无判据，空载运行拒绝产出任何裁决性输出
- 引擎侧：src/scrutinator/rule/mod.rs 不内嵌任何规则码，规则全在 manifest.toml 与 rules.toml 数据加载，空载运行产 json 报告 status=ok 退出码零但 findings 数组空，不拒绝产报告但报告头必含 packs=[] 域与发现零条

### 第一件附材料轴对表 {#material-axis}

- 工具侧：manifest 增可选键 material 取值 text 或 json 缺省 text
- 引擎侧：PackManifest.material 字段直接读 manifest.toml 的 [domain] 同级 material 键，缺省 text

### 第一件附词表对表 {#vocab}

- 工具侧：记录材料谓词五类即 json_field 加 json_array_schema 加 json_number_range 加 json_field_compare 加 json_parse；类型词表七种即 str 加 int 加 float 加 number 加 bool 加 object 加 array
- 引擎侧：Rust 枚举 RuleKind 与 ElementType 与 FieldType 同步五类与七种，json_parse 接受事件类型与 type 字段值一致

### 第二件规则即配置对表 {#rule-config}

- 工具侧：规则以数据文件存在，包形态含 manifest 与规则条目
- 引擎侧：规则文件的具体格式归本规格，承 manifest.toml 含 [domain] 与 [metadata] 两段，rules.toml 含 [[rules]] 数组即每条规则 id 加 kind 加 params 加 message 四字段

### 第三件多规则包对表 {#multi-pack}

- 工具侧：单次运行可加载多个规则包，每条发现携带包标识，跨包同名规则不合并
- 引擎侧：run_engine 接受 Vec<PathBuf> 包路径，发现数组每条 finding 携带 pack_name 与 pack_version 字段，与工具侧逐字节一致

### 第四件 json 报告对表 {#json-report}

- 工具侧：标准输出为人读机读同体的 json，必含引擎版本、每个已加载包的包名与版本与治理域、目标清单与各目标内容哈希、发现数组、汇总计数
- 引擎侧：EngineReport 结构体即 engine: EngineHeader + packs: Vec<PackHeader> + targets: Vec<TargetHeader> + content_hashes: HashMap + findings: Vec<Finding> + summary: Summary，与工具侧逐字段对表

### 第五件三值退出码对表 {#exit-codes}

- 工具侧：零合规、一违规、二工具自身异常
- 引擎侧：src/bin/scrutinator.rs main 函数 emit 函数与工具侧三值映射一致

### 第六件双版本戳加治理域对表 {#versions-domain}

- 工具侧：报告必含引擎版本与全部已加载规则包版本，规则包 manifest 必须显式声明其治理域，域外目标拒绝核验退出码二
- 引擎侧：EngineHeader 字段 name="scrutinator" version="0.1.0"，PackHeader 字段 name + version + domain 同步，域外目标产 DomainMismatch 发现并退出码二

### 五判据对表 {#criteria}

- 判据一回归对表：同包同目标下引擎件与工具件输出 JSON 报告逐字节一致，承 CONTRACT.md 第 39 行
- 判据二域判定：skill 文件在治理域之外，引擎件与工具件同行为产退出码二与域不匹配说明，承 CONTRACT.md 第 41 行
- 判据三版本戳：json 输出含引擎版本与规则包版本，缺任一即判据失败，承 CONTRACT.md 第 43 行
- 判据四多包：至少两个规则包同时加载运行，发现数组每条携带正确的包归因，承 CONTRACT.md 第 45 行
- 判据五不变量：运行全程无网络调用、无目标仓写入，承 CONTRACT.md 第 47 行

### 融回门三查对表 {#three-checks}

- 一查验收判据全过：五判据逐一对表
- 二查接口契约未变：本规格即比对基准，融回只搬实现不改接口
- 三查回迁债已评估：见下文回迁债节，承 CONTRACT.md 第 51 行

## 规则包迁移 {#packs}

### 迁引擎侧家位 {#pack-location}

- 三包家位：sih-engine/src/scrutinator/packs/{des-001,des-001-mathe,ask3}/
- 形态：manifest.toml + rules.toml 纯数据随件迁
- 版本：des-001 0.1.0、des-001-mathe 0.3.0、ask3 0.1.0
- 工具侧保留：sih-tools/scrutinator/packs/ 留档不删作兼容只读，承切换批

### 规则码全量迁移清单 {#rule-codes}

des-001 0.1.0 共十二件
: C001 charset_forbid 禁止半角破折号
: C002 charset_allow 字符集允许
: C006 forbid_pattern 全角括号内容非法
: S002 header_structure single_h1
: S004 header_structure anchor_required
: S005 header_structure first_h2_name="概览"
: S006 header_structure no_level_skip
: F000 line_flag fence_open
: F002 line_flag blockquote
: F003 line_flag bold
: F005 line_flag table
: N002 nav_format 导航项格式

des-001-mathe 0.3.0 共二十件
: 承 des-001 十二件即 C001 C002 C006 S002 S004 S005 S006 F000 F002 F003 F005 N002，全部重声明于本包 rules.toml，不依赖 des-001 包加载，承继逻辑由 manifest.toml 域差异实现
: C001 charset_forbid 半角破折号同 des-001
: C002 charset_allow 字符集扩展含 Greek 加 Math Operators 加 Letterlike 加 Arrows 加 Subscripts/Superscripts，与 des-001 同 kind 不同 ranges
: C006 forbid_pattern 全角括号白名单扩「以下简写为」「又称」「原名」「即」四模式
: S002 header_structure single_h1
: S004 header_structure anchor_required
: S005 header_structure first_h2_name="定义"，数学 6 段规约首段，不沿用 des-001 "概览"，kind 同 first_h2_name，name 数组接受「定义」或「概览」二选一
: S006 header_structure no_level_skip
: F000 line_flag fence_open
: F002 line_flag blockquote
: F003 line_flag bold
: F005 line_flag table
: N002 nav_format
: M008 forbid_pattern H1 标题 prefix 合法集（TOP PROB ORD ALG CALC LIM DIFF INT APP HIS MUL NS SER SPEC）
: M010 forbid_pattern 禁 facet 启发式「5 厂投票加权」
: M010a forbid_pattern 禁 facet 启发式「voter 投票加权」
: M010b forbid_pattern 禁 facet 启发式「5-factory voting」
: M010c forbid_pattern 禁 facet 启发式「voter weight」
: M010d forbid_pattern 禁 facet 启发式「5 厂投票策略」
: M010e forbid_pattern 禁 facet 启发式「5 厂配比」
: M010f forbid_pattern 禁 facet 启发式「投票权重分配」

承继重声明语义：des-001-mathe 包不依赖 des-001 包加载即可独立运行，十二件 des-001 规则在 mathe 包内重声明；承继是逻辑承继非物理依赖，承 DEC-001 围堰归位映射即工具侧不删不改语义，引擎侧独立家位。

ask3 0.1.0 共二十三件
: 锚点 schema + 八意图契约子字段 + 四认知域契约子字段 + 三跨字段比较 + 数值闭区间

### 规则包装载方式 {#pack-load}

两候选对比：
- 候选 A：运行时从文件路径读 TOML。工具件 Python 实现即此方式，灵活可热替换，代价是运行时 IO 失败面与文件路径管理负担。
- 候选 B：Rust 编译期内嵌 manifest.toml 与 rules.toml。引擎侧决定选此方式，理由四件：
  1. 引擎侧 zero-IO 在装载阶段：运行时不读不写规则包文件，承工程基线第一条确定性程序；
  2. 编译期单源权威：manifest.toml 与 rules.toml 在编译时嵌入二进制，金向量冻结后任何字段漂移即重 build；
  3. 切换批零迁移成本：规则包文件源在 sih-tools/scrutinator/packs/，引擎侧 home 位在 src/scrutinator/packs/ 由 include_str! 嵌入，工具侧退役只作兼容只读不删不修；
  4. 路径管理归零：运行时不传 --pack 路径即走编译期内嵌默认包，工具件要求传路径形态在引擎侧不必要。

实现接口：src/scrutinator/packs/{des-001,des-001-mathe,ask3}/ 目录 + manifest.toml + rules.toml + asset.rs 用 include_str!("packs/<name>/manifest.toml") 与 include_str!("packs/<name>/rules.toml") 编译期内嵌。验收：F-5 不变量即运行无文件写覆盖本约束，cargo build 单源生成 binary 含规则字节。

### 工具侧与引擎侧包内容同步 {#pack-sync}

- 工具侧与引擎侧规则内容逐字节一致
- 任何规则增删改必须双侧同步且走规则包版本管理
- 规则包版本号在 manifest.toml 增 minor 加 patch

## 验收判据 {#acceptance}

### 判据 A1 同包同目标逐字节一致 {#a1}

- 测试：同包同目标下引擎件与工具件输出 JSON 报告逐字节一致
- 工具：先跑工具件对三包在真实目标上生成期望输出作为金向量冻结于 src/scrutinator/fixtures/golden/
- 引擎件逐字节对表工具件输出，键序、缩进、空值形漂移即判负返工
- 例外：engine.name 字段引擎件固定 "scrutinator" 工具件固定 "scrutinator" 名字一致；engine.version 字段引擎件与工具件版本号独立即不要求逐字节一致

### 判据 A2 退出码三值一致 {#a2}

- 测试：合规场景零、空载场景零、单违规场景一、域外场景二、缺包场景二
- 引擎件与工具件同输入同退出码

### 判据 A3 报告 content_hashes 与 scribe append 认证位兼容 {#a3}

- 测试：报告 content_hashes 字段含目标文件 SHA-256
- scribe append 走 certification_completed 事件八项负载必载即报告路径、报告哈希、规约包版本清单、目标内容哈希清单、发现计数
- 引擎件与工具件报告 content_hashes 字段格式与键名逐字段对表

### 判据 A4 多包加载与归因 {#a4}

- 测试：des-001 + des-001-mathe 双包同载，目标命中双包规则时发现逐条携带正确 pack_name
- 引擎件 findings 数组每条 pack_name 字段与 loaded_packs 数组的 name 一致

### 判据 A5 不变量 {#a5}

- 测试：运行全程无网络调用、无目标仓写入
- 以进程监视机械验证：strace 或 dtrace 或 lsof 监测网络与文件写
- 引擎件与工具件不变量测试同行为

## 回迁债 {#debt}

### TOML 解析依赖 {#toml-dep}

- 引擎侧需引 toml crate 解析 manifest.toml 与 rules.toml
- Cargo.toml 必加 toml = "0.8" 依赖
- 引擎侧原本零第三方依赖：sha2 与 uuid 与 chrono 与 serde 与 serde_json 已是必引；toml 为融回必加

### 双跑对表基线 {#baseline}

- 在切换批执行前冻结三包对真实目标的双跑对表基线即金向量
- 金向量家位：sih-engine/src/scrutinator/fixtures/golden/
- 金向量冻结后任何字段漂移即判负返工

### DEC-001 围堰物理对应归位映射核阅行 {#cofferdam-line}

承 DEC-001 第 107-141 节围堰物理对应归位映射，本规格的归位映射核阅行
- tools 独立 CLI 工具：物理载体外仓 sih-tools 即各工具契约持 CONTRACT.md，归位动作融回按贡献度逐件评估，承 DEC-001 第 118 行
- 本批核阅归位评估：融回按贡献度评估合格，归位至 sih-engine/src/scrutinator/ 与 src/bin/scrutinator.rs

## 基础设施层归属 {#infra}

- 引擎侧核阅归属承 DEC-007 子决策二基础设施层归属判据即不承接哲学命题与提供运行前提
- 核阅提供组件运行前提即格式规约核验位是治理动作网关层的前置设施
- 不承接哲学命题即核阅本身无对应哲学命题，是工程的纯机械校验位

## 测试计划 {#tdd}

逐判据先红后绿，红态即测试先行而入口未建，绿态即实现批完成。六组如下。

T1 金向量冻结
: 工具件对三包在真实目标上生成期望输出，落 src/scrutinator/fixtures/golden/，红即 fixtures 目录不存在

T2 同包同目标逐字节一致
: 引擎件逐字节对表金向量，红即 src/scrutinator/ 不存在

T3 退出码三值
: 合规、空载、单违规、域外、缺包五场景同输入同退出码，红即 main 入口未建

T4 多包加载与归因
: des-001 + des-001-mathe 双包同载，发现逐条携带正确 pack_name，红即多包加载逻辑未建

T5 不变量
: 运行全程无网络调用、无目标仓写入，进程监视机械验证，红即 stdout 写或网络 IO 残留

T6 报告 content_hashes 兼容
: 报告 content_hashes 字段含目标 SHA-256 且与 scribe append 八项负载兼容，红即 content_hashes 字段未建

红转绿记录入 TDD 批结果档，全绿为切换批入口条件。

## 验收判据 {#acceptance-criteria}

- A1 三件入口签名与负载与工具生产面对表无漏项
- A2 多包加载逻辑可判定 pack_name 归因
- A3 金向量三包各二件全量对表计数
- A4 生产目标复验全零违规
- A5 退出码三值与工具侧对齐
- A6 库面可被 MCP 层直接调用即无 CLI 耦合入库层

## 验收判据合并节 {#acceptance-merged}

本节合并原规格两同名「验收判据」节即行 137 acceptance 即 A1 至 A5 五条与行 218 acceptance-criteria 即 A1 至 A6 六条，统称「验收判据」，承批二并节消歧义，A1 至 A6 六条不重复列示，行 218 原六条作 A1 至 A6 通行面即：

- A1 三件入口签名与负载与工具生产面对表无漏项，承 § 验收判据 #acceptance 第一节即 A1 同包同目标逐字节一致
- A2 多包加载逻辑可判定 pack_name 归因
- A3 金向量三包各二件全量对表计数
- A4 生产目标复验全零违规
- A5 退出码三值与工具侧对齐
- A6 库面可被 MCP 层直接调用即无 CLI 耦合入库层

行 137 acceptance 节 A1-A5 与本节 A1-A6 对表：行 137 A1 即本节 A1 即逐字节一致，行 137 A2 即本节 A5 即退出码三值，行 137 A3 即本节 A3 即 content_hashes 兼容由 § 验收判据 #acceptance-criteria A6 库面可被 MCP 层调用覆盖，行 137 A4 即本节 A2 即多包归因，行 137 A5 即本节 A4 即不变量。

## 边界 {#boundary}

- 本规格不含规则内容的裁决即规则增删改归规则包版本管理
- 工具件 Python 实现不删不改语义，工具侧退役只标注与转兼容只读由切换批执行
- 切换批执行前 TDD 批必须全绿
- 切换批执行前金向量必须冻结
- 上链前必须等绿；读 findings 不只看退出码
- 锁被他在持即报不绕行
