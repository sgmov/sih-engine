# SPEC-017 贡献度测度规范：contrib 读路径承重事件计数与检验判据

本规范承接 gid m-contrib-load-1 stable_clear 终签即 2026-09-04 当日链 crosscheck-m-contrib-load-1 终签 f9520776 成文，把融回贡献度判据数学化落为引擎向界可机械校验的读路径规格。令源即用户 2026-09-04 裁定原话「融合贡献度应该走数学公理，不走人节点」，达标判定由用户裁量改检验判据，DEC-013 修订记录 v1.4 随本规范同批承接。体例承 SPEC-014 与 SPEC-015 与 SPEC-016。范围收敛声明：本规范 v1 只承载贡献度读路径的七节条款，alpha 具体取值属候选值随 pk-049 一并裁，实装载体 gauge contrib 子命令随本批 TDD 落地。

## 概览 {#overview}

- 事件底座即 meter 调用计数册，贡献度是窗内承重事件计数测度承 PROB-016::[计数底座](#measure)
- 承重集由三判据并集承载，三判据全机械复算零 LLM 零网络::[承重三判据](#criteria)
- 可加性恒等式即分日承重计数之和恒等于窗内总计数，破坏即对账拒::[可加性恒等式](#additivity)
- 达标即检验判据两腿，二项检验对空转基线加控制图连续窗判读::[检验判据族](#test)
- 窗口与基线与 alpha 与 p0 全部显式给参，体量与时间两重门槛降级为观察窗参数::[窗口参数](#params)
- 输出面单行 JSON 只报不判，退出码三值::[输出 schema](#schema)
- 可证伪条款与金向量场景逐条钉死::[可证伪与金向量](#falsifiable)

## 计数底座 {#measure}

- 事件底座是 meter 调用计数册逐调用一记录：贡献度测度定义为窗内承重调用的个数，承 PROB-016 计数测度与可加性，条目位 sih-math/probability/entries/PROB-016-counting-measure-and-additivity.md，映射面 sih-math/llm-friendly-build/mapping.md:211，推导档 sih-math/docs/contribmath-derivation-2026-09-04.md。
- 单点赋值承公理一：一条调用恰对应计数册一行一个点，不重不漏；按日分册承公理二：日期切片取计数册 UTC ts 前 10 位，与 gqueue 窗口切片同约定，分日区间两两不交跨册不双计。
- v1 权重均匀：每承重调用权重 1，权重升级走公式版本号不暗改。
- 判据外调用计空转不进计数：空转的调用量不计入承 2026-08-22 用户裁定，判据命中与否全由三判据机械判定，不设人工白名单。

## 承重三判据 {#criteria}

承重集是三判据的并集，逐调用判定，命中任一即承重；三判据均为哈希对表或路径检索，零 LLM 零网络，同输入逐字节可复现。

- 判据一认证引用：调用命令行中路径形 token 即含斜杠且文件实存者，其所指文件的内容 SHA-256 摘要现于链文本 64 位十六进制摘要集，或该路径串原文现于链文本；链文本指 --trail 各文件逐行全文。语义即调用报告哈希现于当日链认证事件引用面。
- 判据二闸门改流：调用解析工具名属于闸族清单且退出码被闸语义消费。闸族清单 v1 钉死九件：formatter 与 scrutinator 与 nomenclator 即管线三步闸承 AGENTS.md 工具层静态审计节、scribe 即写入闸三、selector 即路由拦截、attractor 即执契与路由判定、lease 即锁闸与 settle 通道、elicit 即叩问 digest blocked 拦收口、identity 即正身异常即停。退出码消费语义：退出码取 0 与 1 与 2 三值之一即视为被该闸文档语义消费，各值按各闸自身文档承放行或拦截或异常处置，如 formatter 一即已修改、管线闸零即放行一即拦截，不另设统一映射。显式排除：gauge 只报不判退出码仅指自身、meter 是包裹器非闸、facet 与 tally 是围堰兼容只读位判定正典已迁引擎件、parser 与 locator 无闸语义。工具解析：计数册 tool 字段非空即取之，空则扫命令行 token 匹配闸族名或首 token 基名。
- 判据三材料引用：调用命令行路径形 token 所指路径的任一字符串形态即 token 原文或基名，现于语料文本；语料指 --corpus 各结果档与任务包与基线文件逐行全文并集。v1 宽径检索即基名命中即算，窄径升版走版本号不暗改。

三判据计数分列申报：criteria 块载 auth_ref 与 gate_alter 与 material_ref 三并列计数与 union 并集计数，一调用命中多判据在并集只计一次。

## 可加性恒等式 {#additivity}

- 恒等式：窗按日划分为两两不交的分日集，窗内总承重计数恒等于分日承重计数之和，承 PROB-016 定理二对账恒等式即划分求和。
- 输出恒等式对账：additivity 块载 total 与 partition_sum 与 per_day 分日计数与 calls 分日调用数，verdict 取 identity_holds 或 violated，不裸报布尔。
- 破坏即对账拒承定理三漏重检出：total 与 partition_sum 严格不等即 violated，属账面异常视图告警处置归人节点，同窗两次对账不同结论即公理三伪。
- 公理三对账同窗：总量与分部取自同一闭窗同一切片约定，窗口错位即恒等式不适用不出读数。

## 检验判据族 {#test}

达标判定是两腿合取，替代原用户裁量条款，DEC-013 v1.4 修订承接；判据只出达标与否的读数，融回评估启动仍走批令留痕与融回门三查，判据外处置不因达标而跳步。

- 检验腿承 PROB-010 假设检验与显著性，映射面 mapping.md:200，facetmath 检验族同构：H0 即窗内承重率不超过 p0 空转基线，H1 即高于 p0；n 为窗内调用数，k 为承重数，尾概率为二项分布上尾逐项精确计算零近似；尾概率不超过 alpha 即 reject_H0，否则 fail_to_reject；n 为零即 insufficient 零虚构率与尾概率。拒绝语义承 PROB-010 公理二：只断言空转解释下观测稀有，不断言因果。
- 控制图腿承 PROB-013 平稳性与变点检测，映射面 mapping.md:203，与 gchart gc-1 先例同构：窗内逐日承重计数序列，基线期取前 baseline_n 日，样本标准差 ddof=1 定控制限，控制限为基线均值加减 k_sigma 倍标准差；监测期越限告警与 CUSUM 变点信号两谓词照 gc-1；status 四值 ok 与 insufficient 即窗日数不足 baseline_n 加一或 baseline_n 不足二与 degenerate 即基线零散布控制限收缩为均值，语义照 gc-1；chart verdict 取 sustained 即 status ok 且零下侧告警，或 collapsed 即有下侧告警即连续窗塌陷，或 insufficient 或 degenerate。
- 合取：两腿 verdict 分别 reject_H0 与 sustained 即 attains；任一腿 insufficient 即 overall insufficient 零虚构；其余即 not_attains。attainment 块载 verdict 与两腿子块，各子块载统计依据行 basis 不裸报布尔。
- 依据行载判定性常数语义：alpha=0.05 是候选值随 pk-049 一并裁即待裁登记非已裁，p0 缺省 0.5 是保守空转基线无数学载体属环境参数登记面显式给参，两常数改值走参数面不改公式。

## 窗口参数 {#params}

- 窗口 W 即 --window-days 缺省 7，观察窗参数：体量与时间两重门槛降级为贡献度的载体与观察窗，承 2026-08-22 用户裁定的本批数学化落位，窗长入环境参数登记面，资源性不入数学域。
- p0 即 --p0 缺省 0.5，alpha 即 --alpha 缺省 0.05，baseline_n 即 --baseline-n 缺省 3，k_sigma 缺省 3.0 与 cusum_b 缺省 0.5 与 cusum_h 缺省 5.0 照 gc-1 先例同参；全参数输出 params 块显式回显。
- --at 是参照时间 YYYY-MM-DD 显式给参禁 now，承 SPEC-011 先例。

## 输出 schema {#schema}

- 调用形：gauge contrib --at YYYY-MM-DD --counts meter 计数册可重复 --trail 链文件可重复 --corpus 语料文件可重复，可选 --window-days 与 --p0 与 --alpha 与 --baseline-n 与 --k-sigma 与 --cusum-b 与 --cusum-h；只读不落链，计数册与链与语料零写。
- 输出 stdout 单行 JSON 即 contrib 单键包裹：formula_version gd-1，status 两值 ok 与 insufficient 即窗内零调用，块序 contribution 与 additivity 与 attainment 与 params 与 inputs_digest 与 inputs。contribution 块载 total 与 calls 与 rate 即 n 为零出 null 与 uniform_weight 与 criteria 三并列加并集计数。
- inputs_digest 承 ga-2 口径：计数册内容摘要与链文件内容摘要与语料文件内容摘要与窗口与全参数排序拼接 SHA-256，可机械解析回证据源。
- 退出码三值：零成即 ok 与 insufficient 均成只报不判、一拦即参数违例如 window_days 小于一或 alpha 出开区间或 p0 出闭区间或 baseline_n 小于二、二异常即底座件缺席如 counts 或 trail 或 corpus 任一缺席并在 stderr 报缺席件名。

## 可证伪与金向量 {#falsifiable}

- 证伪形态一：构造判据外调用被计入承重或判据内调用被漏计的实录，即三判据并集承载伪；构造同输入两次运行输出不逐字节一致，即零 LLM 可复算伪。
- 证伪形态二：构造分日之和与总数不等而对账不报 violated 的实录，即可加性恒等式承载伪；构造 n 为零而出 rate 与尾概率的实录，即零虚构条款伪。
- 证伪形态三：构造承重率显著高于 p0 而判 not_attains 或全空转而判 attains 的实录，即检验判据族伪；金向量场景漂移即实现漂移。
- 金向量场景两态冻结于 gauge tests/fixtures/contrib/：attains 态即三判据各有命中且检验腿拒绝且控制图 sustained，not_attains 态即全空转零承重；双跑同参逐字节一致为冻结判据，夹具与金向量以仓根为 cwd 的相对路径寻址，禁绝对路径冻结，重放寻径约定随夹具在档。
- ga-2 三维读数与 gchart 与 gqueue 判据零触碰零回归是本规范的硬边界，新增只增不改。
