# mathscan-solo 结果档

> 批：数学仓全盘扫描 rev3 与管线机械管理四核对器（任务包件一至件五）
> 会话：dc38bb4cc47e343b（sess-zcode-2026-09-04-mathscan）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理；单飞（sih-math 扫描面独占）
> 承接：主会 2026-09-04 盘点——rev2 后十批落地账面再度漂移，rev2 两已知缺陷（calculus 目录约定例外误报与别名口径误报）未修入扫描器；用户令源「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」
> 意图哈希：830e3bff（intent 事件哈希前八位，ask3 记录内容哈希 c02b4283，双门 status ok 三锚）
> 领取登记：claim 在先（claimed_at 2026-09-04T05:41:26Z，TTL 480 分钟），lease open 闸一联动警示读数对表一致

## 一、问题还原

rev2 快照后账面三处漂移：条目文件 165 至 168 之间漂移（实扫 167）、白名单 167 滞后（实扫 172）、rev2 后十批接线未反映（carrwire 四件、pk037impl、leasewire、elicitwire、locatorwire、latexwire、recallloop、queueing、gchart、contribmath、facetmath、basefix）。rev2 扫描器两缺陷未修：其一按统一 entries/ 目录约定找条目致 calculus 特例（条目实居 llm-friendly-build/entries/）误报伪缺口；其二 CALC 系双概念号别名（CALC-008=LIM-007 一文件两号）无记账规范化输出。本批修两缺陷出 rev3 全量重算，立四表核对器常设闸，书架层缺可证伪节只盘点不补节，复用对表逐对核对，例扫挂点提案过得一泊界呈报。

温故检索：materials/recall-mathscan.json 如实记（本批为机械扫描批，温故面由 rev2 三件与推导档目录全读承载）。

## 二、完成度表

| 件 | 判据 | 结果 | 证据 |
|---|---|---|---|
| 件一 rev3 全盘扫描 | 两缺陷修入扫描器，三态表全量重算，白名单联动落本体，双跑逐字节一致 | 完成 | rev3_script.py 三件（ledger-rev3.json 与 env-params-rev3.json 与 summary-rev3.md），双跑 cmp 全 IDENTICAL 含 stdout，findings 零笔 |
| 件二 四表一致性核对器 | checkmath.py 随批入版控可复算，零漂移或漂移逐件清单 | 完成 | checkmath.py 与 checkmath-report.json，双跑 IDENTICAL，零红灰 7 笔逐件在案 |
| 件三 书架层 M-4 盘点 | 缺可证伪节条目全量清单，只盘点不补节 | 完成 | shelfscan.py 与 shelf-falsifiable.json，167 = 6 有节 + 161 缺节对账，零补写 |
| 件四 跨条目定义复用对表 | 多消费概念定义指针与复用关系逐对核对，不一致清单出档 | 完成 | shelfscan.py 与 reuse-crossref.json，1051 对逐对核对，多消费概念 149，不一致 8 笔清单在案 |
| 件五 例扫挂点提案 | 过得一三态分流在案，泊界呈报不执行 | 完成 | facet 测量九发 stable_clear，attractor check R1-R7 pass，泊位 pk-052 停泊事件 42d232ce |

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 rev3 双跑一致** | 工程 | 双跑逐字节一致，三态表与源码扫描对表零漏项，白名单联动落本体 | 过 | run1/run2 cmp 三件加 stdout 全 IDENTICAL，退出码 0/0，漏项核对 findings 零笔，白名单实扫 172 联动落 rev3 本体（任务包预估 171 与实扫差一，以磁盘实态为准如实申报） |
| **F-2 核对器可复算** | 工程 | 随批入版控，四表零漂移或漂移清单完整 | 过 | checkmath.py 在 coverage 目录随批提交，双跑 IDENTICAL，红项零、灰项 7 笔（声明滞后）逐件在案 |
| **F-3 书架层清单完整** | 工程 | 总数对账 166+ 文件 | 过 | 167 = 6 + 161 对账成立，宽口径对照 126 与任务包批时预估约 127 对表，清单全量在档 |
| **F-4 复用对表逐对在档** | 工程 | 多消费概念与逐对核对与不一致清单 | 过 | 1051 对全在 reuse-crossref.json，多消费概念 149 个，任务包点名面（良基 ORD-011 x2、ORD-016 x6、ORD-023 x3、闭包 ORD-006 x4）全数在多消费面 |
| **F-5 例扫提案过得一** | 治理 | facet 测量三态分流在案 | 过 | 标定 20 发体温 0.0 判定可用，正式九发全 comply，闸裁决 stable_clear，attractor check verdict pass |
| **F-6 写入仅 allow** | 治理 | 写入仅请求写入节所列，工程仓源码零触碰 | 过（附申报） | 四处管线活写位（tally/reports 与 facet/contracts 与 facet/probes/calibration 与 proposition）不在 open allow 冻结面，锁取 scope_violation 拒，单飞零争用，文件随 settle 拷工地入版控，详见越线申报节；sih-engine src/ 与 doc/spec 零触碰（git status 空） |

## 四、rev3 三态表（24 机制）

| 状态 | 数量 | 机制 |
|---|---|---|
| 已实例化 | 19 | cascade、elicit、facet、formatter、gauge、identity、latex-helper、lease、locator、locks、meter、nomenclator、parser、scrutinator(sih-tools)、selector、tally、scribe(sih-engine)、scrutinator(sih-engine)、wikirecall |
| 可指认未实例化 | 5 | scribe(sih-tools)、ask3repeater、attractor、retriever、viewer |
| 无可指认载体 | 0 | （空） |

rev2 读数 10 / 5 / 8（23 件）；rev3 机制名册 24 件（增补 wikirecall 第 24 件：任务包所称 retriever 语义层以磁盘实态勘误为 sih-tools/wikirecall，引擎 retriever src 实扫零数学 ID 词面，pk037impl 推导档消费侧即 wikirecall/semantic.py）。rev2 至 rev3 三态变动九件转已实例化（elicit、formatter、latex-helper、locator、locks、meter、nomenclator、parser、wikirecall 新位）。

rev3 白名单与口径三分列：白名单正身号 172（calculus 114 + order 21 + probability 17 + topology 8 + algebra 12）、磁盘条目文件 167 件、别名记账 4 笔（CALC-001→HIS-015、CALC-003→HIS-016、CALC-008→LIM-007、CALC-031→DIFF-032）。INDEX 在册无磁盘 5 笔全为 algebra 待建（ALG-003/004/005/006/007，正常缺省态非漂移），磁盘孤儿零。

双实存核验 36 行全实存（19 已实例化机制源码推导载体逐件 INDEX 与磁盘双在），承继位缺口零——rev2 伪缺口（gauge LIM-007）由目录实态枚举根除。

## 五、四表核对读数（checkmath）

| 面 | 读数 |
|---|---|
| 表 1 mapping | 概念 ID 69 个（mapping.md 238 行） |
| 表 2 INDEX | ID 172 个（calculus 114 / order 21 / probability 17 / topology 8 / algebra 12） |
| 表 3 entry | 磁盘 167 件（calculus 居 llm-friendly-build/entries/，余四仓居 entries/，实态枚举） |
| 表 4 白名单 | = 表 2 恒等（联动断言过） |
| R1 mapping 悬空 | 0 |
| R2 已建缺盘 | 0 |
| R3 待建有盘 | 0 |
| R4 磁盘孤儿 | 0 |
| R5 白名单联动 | 恒等成立 |
| R6 别名正身 | 4 笔全实存 |
| R9 INDEX 重复行 | 0 |
| 灰项（声明滞后，只记不代修） | 7 笔：algebra「当前已建 3 条待建 6 条」vs 实建 7 待 5；order「已建 11」vs 实建 20；probability「已建 7」vs 实建 17；mapping 概览 CALC 113 vs 114、ORD 18 vs 21、PROB 15 vs 17、ALG 5 vs 12 |

判定：四表零红漂移（verdict zero_drift），退出码 0。灰项为 mapping 概览与三子仓 INDEX 概览计数声明滞后于登记面，属投影陈旧非数据漂移，本批零改写（mapping/INDEX 只读红线），修订归词表或索引批。

## 六、书架层清单节（件三）

判据：正文含「## 可证伪」节标题（严格口径）。总数对账：167 = 6 有节 + 161 缺节。宽口径对照（含「可证伪」字样）：41 有 / 126 无，与任务包批时预估约 127 对表一致——批时预估为宽口径，本批清单以严格节标题口径为准全量在档。

有节六件（全为 newcarr 与 pk037impl 两批新立载体，M-4 双答结构承接成立）：ALG-011、ALG-012、ORD-023、ORD-024、PROB-016、PROB-017。

缺节 161 件全量清单（只盘点不补节，补节归载体管线批过得一）：

**algebra（5）**：ALG-001、ALG-002、ALG-008、ALG-009、ALG-010

**calculus（114）**：APP-001、APP-002、APP-003、APP-004、APP-005、APP-006、APP-007、APP-008、APP-009、APP-010、APP-011、DIFF-001、DIFF-002、DIFF-003、DIFF-004、DIFF-005、DIFF-006、DIFF-007、DIFF-008、DIFF-009、DIFF-010、DIFF-011、DIFF-012、DIFF-013、DIFF-014、DIFF-015、DIFF-016、DIFF-017、DIFF-018、DIFF-019、DIFF-020、DIFF-021、DIFF-022、DIFF-023、DIFF-024、DIFF-025、DIFF-026、DIFF-027、DIFF-028、DIFF-029、DIFF-030、DIFF-031、DIFF-032、HIS-001、HIS-002、HIS-003、HIS-004、HIS-005、HIS-006、HIS-007、HIS-008、HIS-009、HIS-010、HIS-011、HIS-012、HIS-013、HIS-014、HIS-015、HIS-016、INT-001、INT-002、INT-003、INT-004、INT-005、INT-006、INT-007、INT-008、INT-009、INT-010、INT-011、INT-012、INT-013、INT-014、INT-015、INT-016、INT-017、INT-018、INT-019、INT-020、INT-021、INT-022、LIM-001、LIM-002、LIM-003、LIM-004、LIM-005、LIM-006、LIM-007、LIM-008、MUL-001、MUL-002、MUL-003、MUL-004、MUL-005、MUL-006、MUL-007、MUL-008、MUL-009、MUL-010、NS-001、NS-002、NS-003、SER-001、SER-002、SER-003、SER-004、SER-005、SPEC-001、SPEC-002、SPEC-003、SPEC-004、SPEC-005、SPEC-006、SPEC-007

**order（19）**：ORD-001、ORD-002、ORD-003、ORD-004、ORD-005、ORD-006、ORD-007、ORD-008、ORD-009、ORD-010、ORD-011、ORD-015、ORD-016、ORD-017、ORD-018、ORD-019、ORD-020、ORD-021、ORD-022

**probability（15）**：PROB-001、PROB-002、PROB-003、PROB-004、PROB-005、PROB-006、PROB-007、PROB-008、PROB-009、PROB-010、PROB-011、PROB-012、PROB-013、PROB-014、PROB-015

**topology（8）**：TOP-001、TOP-002、TOP-003、TOP-004、TOP-005、TOP-006、TOP-007、TOP-008

## 七、复用对表节（件四）

消费对总数 1051（消费方条目到定义位概念，文件内去重后逐对），多消费概念（被引 ≥2）149 个。任务包点名面核验：良基 ORD-011 x2（ORD-021、ORD-023）、良基 ORD-016 x6（ORD-015、ORD-017、ORD-019、ORD-020、ORD-022、ORD-023）、抽象重写系统 ORD-023 x3（ALG-012、ORD-024、PROB-016）、闭包算子 ORD-006 x4，全数在多消费面。

不一致 8 笔（全为 defines_at_missing_disk，即已建条目引用待建概念定义位）：ALG-001→ALG-003/004/005/006/007 五笔、ALG-008→ALG-003、ALG-009→ALG-003、ALG-010→ALG-003 三笔。八笔引用方上下文明标「待建」，属登记面已声明的前向引用，复用链补全归载体管线批（ALG-003 线性映射为最大共享定义位缺件）。别名词面引用（CALC 系）零笔。定义节（## 定义）缺失零笔。

## 八、例扫提案三态节（件五）

| 步 | 读数 |
|---|---|
| 标定 | temp_probe agent 模式 20 发（safe-1/safe-2 comply 全票 bdy 0、violate 判违全票、knife 判违 bdy 1.0），体温 0.0、漂移零、判定可用；基线件 tally/reports/seat-baseline-zcode-2026-09-04-mathscan-solo.json，身份哈希 33994506 与正身件配对一致 |
| 出题 | m-mathscan-exscan-1（ng medium，n 9），topic 落 mathscan-solo-materials/topics/，合同与响应与计分材料落 facet/contracts/zc-glm53flash-20260904-mathscan/ |
| 采样 | 九发围堰席逐发回填（pk037impl 先例同形，零 LLM 堆叠），九发全 comply、变卦 0、谨慎信号 0/9、规约引用 baseline_1 一类 |
| 闸裁决 | gate_verdict **stable_clear**（三态分流：stable_clear / near_threshold / boundary） |
| 得一裁 | 围堰 tally assemble 零判断装配后，引擎 attractor check R1-R7 全过：R2 三哈希复算一致、R3 指纹复算一致、R5 席位基线可用且身份一致、R6 九发未超预算、R7 trail 判定流一致；verdict pass，direction comply |
| 泊界呈报 | **泊位 pk-052**（state parking，TTL 30 天）：停泊事件 42d232ce 经 scribe park 上链，泊材料 pk-052.json 在 mathscan-solo-materials/；择案与执行节奏归用户裁，本批零挂接执行（stable_clear 亦呈泊界不执行，承用户 2026-09-04 裁定） |

## 九、管线读数

- 化格：结果档与泊材料 general-v1 跑数见认证清单前后实录（py/json 件域外如实记）。
- 核阅：结果档 event/plan 域外（des-001 域只盖 sih-engine/doc）exit-2 如实记不属违规；ask3 双门第一门 exit 0 零违规。
- 检词：结果档与泊材料 core 包零违例 exit 0。
- 词债：例扫挂点、四表核对、书架层、复用对表、双概念号、白名单联动六轻信号未登记，处置不登记让位后批词表批，词债记本节与偏差申报节。

## 十、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 830e3bff | 意图笔（scribe intent） | ask3 记录 c02b4283 |
| 3aaa0344 | 管线报告（2026-09-04-mathscan-solo-pipeline.json） | 2026-09-04-mathscan-solo-pipeline 内容即报告件 |
| 0f015eca | rev3 读数报告（2026-09-04-mathscan-solo-rev3.json） | rev3 三件与脚本 sha256 载于报告件 artifacts 节 |
| 9fed7cfe | 四表与书架读数报告（2026-09-04-mathscan-solo-shelf.json） | 核对器与盘点清单 sha256 载于报告件 artifacts 节 |
| a72880a3 | 得一裁与泊界报告（2026-09-04-mathscan-solo-adjudication.json） | 裁决材料与泊材料路径载于报告件 |
| （本笔随结果档定稿上链） | 结果档报告（2026-09-04-mathscan-solo-results.json） | 结果档内容 sha256 载于报告件 |

认证一律先落主树活链（meter 包裹引擎 scribe append，闸三 --sessions 必带），链 verify 以 close 后读数为收。

## 十一、越线与误差申报

1. **allow 冻结面四处管线活写位**：tally/reports（标定响应与席位基线）、facet/contracts（合同与响应与计分材料）、facet/probes/calibration/ledger.jsonl（标定账本一行主树活写，newcarr 先例同形）、proposition/DES/m-mathscan-exscan-1/（flywheel trail 与 tally-material 与 score-material，命题区即 counter 与 facet 共享输入输出位）。open 时 allow 未列此四处，锁取 scope_violation 拒（如实记不绕行）；单飞零并发窗口实际零争用，全部文件 settle 前一次性拷 tools 工地随批入版控。
2. **facetmath 挂起申报**：facetmath-derivation 声称消费侧 probes/maturation_gate.py boundary_criterion 门控位实测零 PROB-010 词面命中（门控挂起形），PROB-010 源码面仍由 facet_stats_inf.py 承载，rev3 不扩 facet 载体面不代判。
3. **任务包口径勘误两笔**：白名单「171」预估与实扫 172 差一，以磁盘实态为准；「retriever 语义层」实居 sih-tools/wikirecall（引擎 retriever 零词面），机制名册 24 件申报。
4. **temp_probe score --out 旗标误差**：首跑误以 --out 落基线件（该旗标仅属 export-pack，score 只打印读数入账本），attractor check 首跑 R5 fail 如实暴露后改 score 返回 dict 转写落盘，复跑装配与 check 全绿——首跑 fail 与复跑 pass 两读数并列在案不以绿饰红。
5. **书架层口径差**：任务包批时实数约 127 为含字样宽口径（本批复算 126），本批清单为节标题严格口径 161，两口径并列在档（6 + 161 = 167 对账）。
6. **词债**：六轻信号未登记（见管线读数节），词面由任务包与结果档行文承载。
7. **泊界投影缺口**：pk-052 名册行（doc/governance/PARKING-v1.md）与 state/parking/materials/pk-052.json 副本因 allow 闸未落主树，链上 park 事件与泊材料为正典，投影交后批照链补齐（泊界投影照链补齐先例）。

## 十二、冲突样本节（pk-045 样本库）

本批单飞，pk-045 样本库零并行成员（开工实查零活动会话零锁，租约窗口全程无撞锁）。冲突响应预案在案：共享追加面 append 短持、exclusive 撞锁重试上限十次不绕行、收约撞主树同名未跟踪件走备份让位归并对表法。实测冲突零笔。泊界心跳（开工例行）：tools 线 19 件 18 主线 1 停放（pk-042 P102 未过）、engine 线 19 件 18 主线 1 弃置（pk-013 P101 未过），两目录退出码零零 alarms，如实转述。

## 十三、验收

- [x] F-1 至 F-6 全过（F-6 附申报）
- [x] 件一至件五全完成，结果档各点名节在档
- [x] 认证入链（认证逐笔后回填哈希），三仓 settle，close 后收口读数见收口附记
- [x] 队形验证：单线形 solo 成立——本批全部写入由会话 dc38bb4cc47e343b（sess-zcode-2026-09-04-mathscan）亲写，零 Agent/Task 子代理调用；三态判与核对与盘点全由可复算脚本承载，facet 测量九发与标定 20 发为合同模式围堰席逐发回填，终签判据由 attractor R1-R7 确定性核对承载；链写入经引擎 scribe 闸三，零直写链文件

## 收口附记（close 后补记，本节经 close 通道外 bypass 提交回填，ordwire 与 pk037impl 先例同形）

- close 三跑：第一跑 math 归并成（dd97fa0 副本），tools 与 engine 撞主树未跟踪件与 tracked 本地改动（engine 主树任务包 mathscan-solo.md、tools 主树管线报告件与 facet 合同与标定基线与命题区裁决材料与 calibration 账本本地行）；第二跑 engine 归并成、tools 仍撞 identity 与 meter 与 scribe 报告件；让位前备份全部落 /tmp/mathscan-close-backup，第三跑全绿：三仓归并、分支全删、工地全拆、会话吊销。
- 让位对表：备份与归并结果 21 件逐件 cmp 全数 IDENTICAL，让位零丢失零漂移。
- 链读数：当日链 177 事件 verify valid（开工实取 157，本批 +20：例行读数三维、intent、六笔认证、park 停泊、settle 路由笔），链尾即结果档认证笔 ff27c6c4；close 归并前并集复查 live_only_count 0（零 clobber）。
- 三仓主树归并提交：sih-math e274cd8、sih-tools 2a9eb021、sih-engine f71bd0d；分支内段提交：math dd97fa0、tools ccb78bb0 与 a214bff3（bypass 补笔）、engine 1d85f88 与 72ff7e8（当日链快照段二）。
- reconcile：engine 全零；tools cert_missing 1 与 math cert_missing 2 加 unbypassed 1 均为批前历史旧账非本批新增（本批 settle 与归并全数 routed，本批 bypass 仅 tools a214bff3 一笔已登记）；unrouted 三仓全零。
- 泊界收口：pk-052 在泊（state parking），停泊事件 42d232ce 在链，名册投影行与 state/parking/materials 副本交后批照链补齐。
