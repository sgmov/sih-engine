# SPEC-014 得一融回落差规格

本规格承接 sih-tools/facet/docs/CONTRACT-MODE-SPEC.md 采样腿契约与 sih-tools/tally/CONTRACT.md 机械腿契约与 DEC-013 融回门机制与 DEC-020 得一组件立名，钉死判定器席实例得一从围堰融回引擎侧的全部待建面。本规格是 SDD 产物即先于实现，TDD 批按本规格逐判据先红后绿。工具侧现状权威即两 CONTRACT 现行文：采样腿契约承 CONTRACT-MODE-SPEC v1 现行文，机械腿契约承 tally CONTRACT v1.3.0 现行文加 listzero-solo 五修即 2026-09-02 签署印守卫修后现文。融回门判据背景承 GOV-002 退出标准判据一判定器席，判据二至五全挂其后。

## 概览 {#overview}

- 家位与模块形即 src/attractor/ 库模块加 src/bin/attractor.rs 二进制同名，代码标识符 attractor 承 DEC-020，治理名得一不变，采样观察腿与机械核对腿分层::[家位与模块形](#shape)
- 接口契约对表即采样腿 emit-contract 与 answer 与 score 三段契约件加机械腿 assemble 与 check 与 verify 与 sign 与 watch 与 crosscheck 六子命令逐条对表，crosscheck 事件十五字段负载逐字段列，零语义漂移::[接口契约对表](#interface)
- 双模并存条款即 facet CLI 围堰原位保留承 GOV-002 判据四，采样腿留围堰即组件层零 LLM 纪律，引擎机械腿经进程边界调围堰采样腿，模块边界显式::[双模并存条款](#dual-mode)
- 腿切分清单即逐文件「融回 / 留堰」判定加一句理由，本规格心脏，TDD 批按此逐条验::[腿切分清单](#leg-split)
- 验收判据即金向量逐字节含合同哈希与响应哈希与终签锚哈希、退出码对齐、des-011 判定规约随迁、机械腿零网络零 LLM 不变量、跨腿契约逐字段兼容::[验收判据](#acceptance)
- 金向量脏目标条款与双跑同参形条款显式在场，承 SPEC-013 修订四教训::[金向量脏目标与同参形条款](#golden)
- 回迁债即 req 与 llm_client 依赖切分清单、跨仓子进程调用形态、DEC-001 围堰归位映射行::[回迁债](#debt)
- 测试计划 T1 至 T6 先红后绿，红转绿记录入 TDD 批结果档::[测试计划](#tdd)

## 家位与模块形 {#shape}

### 库模块 src/attractor/ 与二进制 {#lib}

- 库模块家位即 src/attractor/，命令行面即 src/bin/attractor.rs，库与二进制与检词登记同位同名，承 SPEC-013 核阅融回先例的家位纪律
- 二元代码标识符小写 attractor 不另起，承 DEC-020 命名集英文对即 attractor 动力系统吸引子行业根；治理名得一不变，中文正式名承道德经三十九章文本根
- 立名段零动作：attractor 与得一已于 2026-08-30 经 DEC-020 签署登记，检词核心包词条在册即 terms.json attractor 条目义为二层裁决结构组件，本规格只引用登记条目不另立名
- 席位：组件第七席，席序承第六席温故之后，承 DEC-020 席位与连带节
- 子模块拟五件即 contract 合同装配与对表、score 计分内核、stats 统计量族、check 基线核对即 R1 至 R7 与三态映射、sign 终签编排；cli 归 src/bin/attractor.rs
- 机械腿零规则内嵌纪律即判定规约外置：R1 至 R7 规则码与三态映射是代码承载的核对逻辑，rules_version 字段随材料声明版本，规则增改走版本管理即 rules_version 形变更为唯一入口
- 引擎侧不受句读零第三方约束即按需引依赖，区别于工具侧；引擎侧哈希复算复用既有 sha2 面即与 compute_event_hash 同源工具箱

### 腿分层 {#legs}

- 采样观察腿即 LLM 席位采样与直跑编排，留围堰 sih-tools/facet，即组件层零 LLM 纪律的物理承载
- 机械核对腿即合同装配对表、响应严格对表、原文解析、计分材料、R1 至 R7 核对、三态映射、终签编排、重放监视，落 src/attractor/，全确定性零 LLM
- 两腿唯一接口是文件合同与工件：采样合同 json、响应 jsonl、计分材料 json、核对报告 json、signcheck json，与 CONTRACT-MODE-SPEC 范畴排除二一致即文件是最低公共分母
- 引擎侧 crosscheck 守卫与 crosscheck_completed 事件已在位即 src/event_stream/crosscheck.rs，本融回不迁不改即执契 sign 落据写权仍走书简管线，承 tally CONTRACT 边界节

### 治理名与代码标识符分离 {#naming}

- 治理名得一不变，承 DEC-020 命名集中文正式名
- 代码标识符 attractor 承 DEC-020 命名集英文对，展示名与内部代号不设
- 席位参验承 DEC-007 即组件是已立名实例，DEC-013 融回只搬实现不改接口

## 接口契约对表 {#interface}

两工具契约逐条对表如下，零语义漂移。采样腿对表基准即 CONTRACT-MODE-SPEC v1 现行文，机械腿对表基准即 tally CONTRACT v1.3.0 现行文加 listzero-solo 签署印守卫修后现文。

### 采样腿合同件对表 {#contract-workpiece}

采样合同（contract）json 载七顶级键，工具侧 contract_mode.emit_contract 落盘形即 json dumps ensure_ascii false 加 sort_keys true 加 indent 2 加尾换行，同输入重 emit 逐字节一致无时间戳无随机：

- kind 即 facet-sampling-contract
- contract_version 即 1
- seat 即席位自报 {framework, model_id, thinking, version}，自报对表不验证
- pack 即 {paradigm_id, atom, direction, ng_label, ng_text_sha256}
- proposition 即 {gid, title, topic_sha256}
- shots 即逐发数组 {key, shot, system_prompt, user_prompt}，key 形如 gid#r k 序号
- responses_format 即 {file: jsonl, fields: [key, shot, raw], discipline: 逐发模型完整原文回填，不改写、不摘要、不挑选}，另 meta 载来源溯源

装配确定性对表：提示词构造与 src/runner.py parallel 路径同源，同一 atom.yaml integrator 模板加同一替换序列，ng 文本取 scheme yaml 解析 clip 形即非空恰多一尾换行；逐字节一致性由单测对真 runner 路径 mock 对表机械证明，融回后此判据随迁。

### 采样腿两段式对表 {#two-stage}

- emit-contract 出题半：measure.py 命令形即 measure.py topic.md --emit-contract out.json --seat framework:model 及可选 --shots N，零 LLM 零网络零 key，落采样合同
- answer 响应回填半：响应工件（responses）jsonl 每行恰 {key, shot, raw} 三键，raw 为模型完整原文，纪律即逐发原文回填不改写不摘要不挑选；此半由框架自带模型执行即采样动作本体，留围堰
- score 计分半：measure.py 命令形即 measure.py topic.md --score responses.jsonl --contract contract.json 加 --identity-report 或 --identity-hash 强制携带承 pk-035；响应严格对表后逐发入飞轮 trail 过判据 v3 闸落计分材料
- 计分材料（score material）json 载：kind 即 facet-contract-score、score_version 即 2、gid、identity_hash 即六十四位十六进制强制、seat_string 即规范席位串三段式、contract_sha256、responses_sha256、n_shots、voids、contract_path、responses_path、trail_path、runs_written、gate_verdict
- 计分严格对表即 key 全在合同内、无重复、shot 对齐、零缺失、raw 非空，任何违例拒收整批不静默；解析失败不静默丢即落 no_answer 加 boundary_flag true 空转标记逐发计入
- 哈希绑定链即合同哈希绑定响应哈希绑定裁决材料，verify 据此全量重算，承工程基线四

### 机械腿 tally 六子命令对表 {#tally-subs}

tally CONTRACT 契约源即 DES-011 裁决基线核对，七规则 R1 执行者归属、R2 来源可追溯、R3 结果可机械校验、R4 历史不可篡改、R5 席位有效性、R6 预算上限即同 gid 累计不超九发、R7 谱系完整；闸三态映射处置四值即裁决通过、材料退回、挂起、打回重作，方向沿用席位众数不重算语义方向：

- assemble 确定性装配：--gid 加 --des-root 加 --topics-dir 可重复加 --baseline 加 --date 加 --out；零判断装配 tally-check-input，择件规则即计分材料 responses_sha256 与当前响应文件哈希一致者取 n_shots 最大，无一致者报错不猜；topic 按合同 topic_sha256 哈希匹配，无匹配即报错
- check 全量核对：--material；R1 至 R7 逐条加三态映射落处置，报告十二字段，输出通过项与失败项清单，失败项带规则标识与位置；退出码 0 核对全过、1 有失败项、2 用法或环境错误
- verify 重放：--material 加 --report；同输入重跑核对与既报报告逐字节比对，identical 出 0、divergent 出 1 即复算与既报不符整包作废
- sign 机器终签：--material 加 --out 加 --trail 加 --scribe-binary 加 --session 加 --locks；仅裁决通过时经引擎 scribe crosscheck 落据，写权归书简管线；三态输出对表：处置非裁决通过出 refused 出 1；scribe 非零退出出 sign failed 带 scribe_exit 即落据未发生链上无事件唯退出码与此输出为准，退出码如实透传不出 signed，承 listzero-solo 五修现文；成功出 signed 出 0 并出人话视图行即 gid 处置方向重放锚位置，人类知晓零动作要求无需复签
- watch 边界变更监视：--reports；对目录内全部 signcheck 批量重放复算浮异常视图，有异常出 1
- crosscheck 落链：引擎 scribe 专属子命令，事件类型 crosscheck_completed，载荷十五字段见下节；材料经 --material 显式传参不猜路径，报告十二字段冻结面零动即 watch 既有重放零破

### crosscheck 事件十五字段负载对表 {#fifteen}

引擎侧既有守卫 guard_crosscheck 冻结面，事件负载十五字段即报告十二字段加所指材料拈三，逐字段列示零漂移：

- gid：小写字母数字短横线形，首字符非短横线
- disposition：非空字符串，按处置机械分事件类
- direction：非空字符串，席位众数沿用
- verdict：pass 或 fail 二值
- gate_verdict：非空字符串即闸三态之一
- rules_version：des 三位号 r 数字形即 des-011-r1
- material：相对 json 路径形，绝对路径拒
- tool：字面 tally
- version：三点数字形
- passed：数组
- failed：数组
- alarms：数组
- topic_sha256：六十四位小写十六进制，从所指材料拈取
- dc_fingerprint：非空字符串，从所指材料拈取
- n_shots：不小于 1 的整数，从所指材料拈取

事件侧对表：doc_id 即 crosscheck-gid；event_class 按处置机械分即裁决通过取 record_only 余取 consumable；缺字段与多字段与形态违例逐字段定位拒绝不静默；材料缺席拒。本融回对十五字段面只搬运不改一字，引擎侧守卫已在位即零改动。

### 核对报告十二字段冻结面对表 {#report-twelve}

check 与 signcheck 报告十二字段即 crosscheck 冻结面承前十二项：tool 即 tally、version、material、gid、gate_verdict、direction、disposition、rules_version、passed、failed、alarms、verdict。报告落盘形即 json dumps ensure_ascii false 加 sort_keys true 加 indent 2 加尾换行。watch 重放即对此冻结面逐字节比对，融回后任何字段增删即判负返工。

### 跨腿契约字段对表 {#cross-leg}

计分材料字段与 tally check 输入的兼容链逐字段对表：assemble 从计分材料取 gate_verdict 与 contract_sha256 与 responses_path 与 responses_sha256 与 n_shots 与 voids 与 identity_hash，从合同取 proposition.topic_sha256，从 trail 复算 dc_fingerprint，拼 tally-check-input 即 kind、gid、topic_path、topic_sha256、trail_path、dc_fingerprint、gate_verdict、criteria_version 即 v3、contract_path、contract_sha256、responses_path、responses_sha256、n_shots、voids、rules_version 即 des-011-r1，另 identity_hash 与 seat_baseline_path 与 date 条件在场。跨腿零语义漂移判据即引擎机械腿读工具件产计分材料与读引擎件产计分材料同输入同处置。

## 双模并存条款 {#dual-mode}

- facet CLI 围堰原位保留即 GOV-002 退出标准判据四字面：facet 转模块接口调用，单独发布能力保留即双模并存；measure 与 singleseat 与 dose_driver 三腿 CLI 全原位不动
- 引擎侧模块接口可被组件层调用即判据四的引擎半：src/attractor/ 出 lib 面即五子模块公共函数，组件层不经 CLI 直调
- 采样腿在围堰经进程边界调用，模块边界显式：引擎机械腿需要新鲜采样时以子进程调围堰 CLI 两段式，工件经文件合同交接，命令形见接口对表两段式节；不做进程内嵌入不做 FFI 不做跨仓 import
- 组件层零 LLM 纪律：src/attractor/ 与 src/bin/attractor.rs 零 LLM 调用零网络零 key 读取；采样只发生在围堰进程内即 llm_client 与 runner 所在侧，两腿边界即零 LLM 纪律的实现形态
- 存量感知幂等：合同模式计分半的存量感知幂等续跑语义随迁，即 run_id 已在 trail 即跳过该发，重复调用不重写历史行，承 R4 历史不可篡改
- 双模不是常态：并存期以生产 trail 全量复验双跑一致为切换判据承 DEC-013 第二步，切换完成即进工具侧退役第三步，本规格不预写退役细节

## 腿切分清单 {#leg-split}

逐文件判定「融回 / 留堰」，每行一句理由。融回即功能落 src/attractor/ 重写为 Rust，围堰原件双模并存期不动不删；留堰即留在 sih-tools 围堰。TDD 批按此清单逐条验，清单外文件不迁。

融回十一行：

contract_mode.py
: 融回。合同 emit 与 load 与响应严格对表与原文解析与 dc 装配与计分材料落盘与规范席位串即采样腿契约的机械内核，import 面仅 hashlib json time pathlib 加懒加载 paradigm_loader，零 LLM 零网络零 key

facet_stats.py
: 融回。统计量门面 re-export 十一函数为唯一公共入口，纯标准库零 LLM 零网络零写路径，其门面自述即此

facet_stats_metrics.py
: 融回。距离度量族四函数即 hellinger 与 tv 与 js 与 generalized jaccard，纯数值标准库实现

facet_stats_inf.py
: 融回。推断校正族五函数即二项检验与 boundary 检验与 Bonferroni 与 BH FDR 与 power，纯数值标准库实现

facet_stats_conv.py
: 融回。收敛推断族二函数即共享收敛置换检验与混合效应 ANOVA，纯数值标准库实现

compiler.py
: 融回。聚合与指标计算与发散发谱与判定收敛装配全为文本解析与数值聚合，import 面仅 anchors 与 model_utils，零 LLM；其发散发谱格式串已被合同出题经空标记快照引用即跨腿契约一部分

validators.py
: 融回。校验器族即白名单与过滤与 markdown 形与 json schema 纯解析，无 llm_client 依赖

anchors.py
: 融回。锚定白名单装载与锚引用校验纯机械，compiler 依赖件

model_utils.py
: 融回。actor_id 拆解纯字符串函数族，compiler 依赖件

paradigm_loader.py
: 融回。atom 与 chain yaml 装载与范式校验纯机械，contract_mode 出题依赖件

tally/src/tally/cli.py 全件与 tests/test_tally.py
: 融回。check 与 verify 与 assemble 与 watch 与 sign 五子命令全确定性核对零 LLM 零采样对材料只读，sign 的链上写经子进程交书简即机械腿本体；测试面随迁作 TDD 基线

留堰十二行：

llm_client.py
: 留堰。AsyncOpenAI 客户端构建与 call_llm 采样调用本体，组件层零 LLM 纪律的直接禁融回对象

engine.py (facet/src)
: 留堰。execute_atom 建 AsyncOpenAI 直呼 llm_client 即 LLM 调用腿本体，import openai 实证；其对 validators 的依赖倒挂见回迁债节处置

runner.py
: 留堰。直跑判定流编排串 execute_atom 采样与 stage 推进，LLM 编排腿

env_loader.py
: 留堰。env 文件装载与组配置读取即密钥面，密钥不入引擎仓

req.py
: 留堰。key 存在性探针即密钥面

config.py
: 留堰。模型注册表与组运行时装载即采样腿配置面，与 env 同源

thinking_resolver.py
: 留堰。thinking 模式解析供采样请求构造，采样参数面

concurrency.py
: 留堰。采样并发信号量与默认并发数，直跑编排依赖

persistence.py
: 留堰。直跑采样输出持久化写位，机械腿只读不变量不携此写路径

audit.py
: 留堰。直跑审计写入位，同上写路径留堰

measure.py 与 singleseat.py 与 dose_driver.py 及 probes 全件
: 留堰。CLI 围堰原位保留即双模并存条款承载面；measure 的判据 v3 闸接线与飞轮 trail 写入属 facet 单独发布能力，其合同模式两段即引擎机械腿经进程边界调用的入口

timestamp.py
: 留堰。直跑时间戳面供 runner 报告与审计，融回件自身用标准时钟不经此件

清单边界声明：facet_stats 数值函数不在两工具契约面上，其金向量判据取统计结论等值与退出码一致，不取字节级即浮点文本形差不算漂移，此为显式范畴排除非默认沉默；两契约面即合同件与计分材料与核对报告与 signcheck 四类 json 工件取逐字节判据。

## 验收判据 {#acceptance}

### A1 金向量逐字节一致 {#a1}

- 四类工件金向量即采样合同 json、计分材料 json、核对报告 json、signcheck json，同包同目标下引擎件与工具件输出逐字节一致，键序、缩进、空值形、尾换行漂移即判负返工
- 金向量内含合同哈希与响应哈希与终签锚哈希即 contract_sha256 与 responses_sha256 与 signcheck 报告字节随件冻结，任何字段漂移即判负返工
- 数值函数面例外见腿切分清单边界声明

### A2 退出码对齐 {#a2}

- check 三值即 0 全过、1 有失败项、2 用法或环境错误，同输入同退出码
- sign 三态即 refused 出 1、scribe 非零退出 failed 透传 scribe_exit、signed 出 0
- verify 即 identical 0 与 divergent 1；watch 即无异常 0 有异常 1
- 引擎二进制与工具件同输入同退出码全表对齐

### A3 des-011 判定规约随迁 {#a3}

- 判定规约即 R1 至 R7 七规则与闸三态映射四值处置与优先级序随迁，rules_version 字段承载 des-011-r1 形，形校验与引擎既有 is_rules_version 一致
- 规则语义零漂移即同一份 tally-check-input 引擎件与工具件 check 处置四值与方向与通过失败清单逐字段一致
- 多 gid 材料顺序加载处置互不串扰即谱系与预算按 gid 独立累计

### A4 机械腿不变量 {#a4}

- src/attractor/ 与 src/bin/attractor.rs 运行全程零网络调用、零 LLM 调用、零 key 读取、零目标仓写入
- 以进程监视机械验证即 dtrace 或 lsof 或等价监测网络与文件写
- 采样动作只存在于围堰子进程即进程边界可审计

### A5 跨腿契约逐字段兼容 {#a5}

- score 报告即计分材料字段与 tally check 输入经 assemble 逐字段兼容，见跨腿契约字段对表节
- 引擎件 sign 产 signcheck 报告经既有 guard_crosscheck 零改通过即十五字段守卫即插即用
- watch 对引擎件产 signcheck 与工具件产 signcheck 混合目录重放行为一致

## 金向量脏目标与同参形条款 {#golden}

承 SPEC-013 修订四教训，两条款显式在场即本规格验收的组成部分。

金向量须含脏目标条款
: SPEC-013 修订四原文引用即「金向量六件全零发现净目标对结构与语义天然免疫等价性从未被字节级钉住」；attractor 金向量同样禁净目标单腿：核对报告类金向量须含裁决通过净目标至少一件加脏目标至少三件即挂起形 (near_threshold 或 R5 漂移挂起)、材料退回形 (R2 至 R7 任一失败项在案)、告警形 (R7 同命题改写超三次)；合同类金向量须含响应违例拒收形即缺发与键不符与 shot 错位与 raw 空至少各一件的拒收基线；冻结后任何字段漂移即判负返工

双跑同参形条款
: A1 逐字节判据的执行条件即双侧同参形：判定规约一致即 rules_version 字符串字节级一致，目标路径一致即两侧均用绝对路径形态，输出形态逐字节一致；即两侧均以 uv run tally check --material 绝对路径与引擎二进制同参运行，cmp 零差与退出码一致；相对路径形态在双跑中视为调用形差异非等价性差异，不作为切换判据

## 回迁债 {#debt}

### req 与 llm_client 依赖切分清单 {#dep-split}

- 融回件依赖闭包全落在融回件互依：compiler 依赖 anchors 与 model_utils，contract_mode 依赖 paradigm_loader，facet_stats 门面依赖三子模块；融回闭包零依赖 llm_client、req、env_loader、config，即融回与留堰的依赖面天然可切
- 依赖倒挂一处即 facet/src/engine.py import validators 的 VALIDATORS：处置即双模并存期围堰原件不动，工具侧 validators.py 在围堰续用，引擎侧 src/attractor/ 的 validators 重写件独立存在，两份不跨仓 import；切换批退役时随工具件整体转兼容只读
- 切分清单即腿切分清单节全量，此处不重复；TDD 批红态测试须含清单逐行核对用例即防清单与实现漂移

### 跨仓子进程调用形态 {#subprocess}

- 引擎机械腿调围堰采样腿以子进程承载：命令即 python3 加围堰脚本绝对路径加 CONTRACT-MODE-SPEC 两段式旗标，工作目录与路径全部绝对形
- 工件交接只经文件：合同 json 与响应 jsonl 与计分材料 json，引擎侧不解析围堰进程内存态
- 子进程退出码与 stderr 如实透传上层不静默吞；超时与缺仓即围堰不在位出明确错误态出退出码二即环境错误类
- 路径解析承 DEC-001 旧路径不失效原则：围堰仓位经归位映射行解析，硬编码相对猜测禁用

### DEC-001 围堰归位映射行 {#cofferdam-line}

承 DEC-001 围堰物理对应与归位映射节 tools 独立 CLI 工具条目即物理载体外仓 sih-tools 各工具契约持 CONTRACT.md 归位动作融回按贡献度逐件评估：

- 本批归位评估行：facet 机械层与 tally 全件贡献度评估合格，归位至 sih-engine/src/attractor/ 与 src/bin/attractor.rs；采样观察层留围堰不归位即组件层零 LLM 纪律的构成性条件，非评估不合格
- 归位三原则对表：事件只追加即 trail 历史行不改写；旧路径不失效即双模并存期 sih-tools 原路径继续有效；迁移动作走批留痕即本规格批即融回三步曲第一步
- 贡献度证据：adisp-guard-1 与 m-p3xcarr 两笔非自证 stable_clear 在链即判定器席贡献度的既有实证

## 测试计划 {#tdd}

逐判据先红后绿，红态即测试先行而入口未建，绿态即实现批完成。六组如下。

T1 金向量冻结
: 工具件对四类工件在真实材料上生成期望输出落 src/attractor/fixtures/golden/，红即 fixtures 目录不存在；金向量构成须满足金向量脏目标条款即净一加脏三以上

T2 四类工件逐字节一致
: 引擎件对金向量 cmp 零差加退出码一致，红即 src/attractor/ 不存在；同参形条款为执行条件

T3 退出码全表
: check 三值与 sign 三态与 verify 两值与 watch 两值全表同输入同退出码，红即 src/bin/attractor.rs main 入口未建

T4 跨腿契约兼容
: 计分材料经 assemble 至 tally-check-input 逐字段兼容加引擎 signcheck 经 guard_crosscheck 零改通过，红即跨腿对表逻辑未建；含腿切分清单逐行核对用例

T5 机械腿不变量
: 全程零网络零 LLM 零 key 读取零目标仓写入，进程监视机械验证，红即网络 IO 或 LLM 调用残留

T6 des-011 规约与存活性
: rules_version 形校验与三态映射四值处置零漂移加存量感知幂等续跑不重写历史行，红即规约随迁未建

红转绿记录入 TDD 批结果档，全绿为切换批入口条件。

## 边界 {#boundary}

- 本规格不含采样腿围堰内任何代码改动即 facet 与 tally 源码零改动，工具侧双模并存期不改不删
- 本规格不含实现即零实现，src/attractor/ 与 src/bin/attractor.rs 待 TDD 批落码
- sign 链上写权归书简管线即引擎 scribe crosscheck 子命令不动，本融回不新增事件类型不扩十五字段面
- 判据 v3 闸即 probes/maturation_gate.py 属 facet Path C 不改，留堰
- 切换批执行前 TDD 批必须全绿；切换批执行前金向量必须冻结
- 上链前必须等绿；读 findings 不只看退出码
- 锁被他在持即报不绕行

## 规格修订记录 {#revisions}

2026-09-02 修订一，deyimerge-switch-solo 批切换执行记录：承 deyimerge-sdd-solo 规格批与 deyimerge-tdd-solo 实装批两步主会验收，本批按用户 2026-09-02 切换放行令执行三步曲第三步。双跑判据执行实录：活体五场景 check 同参形双跑 cmp 全 IDENTICAL 且退出码对齐即 0/0/0/1/0、score 活体双跑 cmp IDENTICAL，证据入 sih-engine/sih/event/plan/deyimerge-tdd-solo-materials/live-double-run-cmp.log 与 live-double-run-score.log；pk-036 金向量重录按现行 GOV-003 真实内容以围堰件为基准刷期望输出 content_hash 与 findings 区，断言逻辑零改，重录后 cargo test golden_des001_gov003 转绿且双跑 cmp 围堰件与引擎件 IDENTICAL 留证。判定位换旗完成：BATCH-FACE.md 执契与 facet 测量命令段改指引擎件 target/debug/attractor、DEC-020 无调用位表述零碰、宪法节与启动命令一字不碰字节级 diff 自证仅限换旗行。工具件退役完成：facet CONTRACT 首立与 tally CONTRACT 退役标注即判定职能强制位改指引擎件、转兼容只读、facet CLI 采样双模并存保留即本规格双模并存条款与 GOV-002 判据四字面，两 CALL-LOG 落尾行。本批本规格不再修订仅以修订一作切换执行记录归档。

## 内容充分性 {#sufficiency}

- 本节为 docmath-b4-solo 收尾批按新旧都管裁定补齐，模板见 SPEC-TEMPLATE-sufficiency-v1，只加节不改本文实质。
- 判据红证对表：本文冲突样本与对表读数节即判据红证载体；其余判据零信息部分如实申报，清账路径为后继修订批逐件补红证，承 sih-math/docs/docmath-carriers-derivation-2026-09-04.md。
- 判定性常数挂锚对表：无声明的判定性常数。des-001 C007 与 C008 与 C009 行级与邻近级闸在役核验。
- 约束算子对表：本文无约束算子面，如实申报。
