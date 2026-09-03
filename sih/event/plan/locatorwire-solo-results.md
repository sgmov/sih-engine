# locatorwire-solo 结果档

> 批：寻址 locator 接 ORD-019 载体（m3clear 实例化候选第三件，标准四件套接线批，零行为变更）
> 会话：92cec27f94d03d4a（sess-zcode-260904-locatorwire）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理；冲突模式成员（pk-045 样本库，与 queueing-solo 与 latexwire-solo 同日并行）
> 承接：m3clear-solo 处置清单实例化候选行「locator：ORD-019 承寻址与陈旧检测」；四件套形制承 ordwire 与 elicitwire 先例；政策行（用户 2026-09-04 裁定）「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」
> 意图哈希：d866212fda71447a567a49a3fd66d7dd233598ecaed2737f81dd03fd32c0e876（会话台账 intent.record_sha256 对表一致，ask3 验证 status ok 三锚，意图事件 c7d130f0）

## 一、问题还原

locator 的判定面即三件：稳定标识派生（identity.stable_id，路径加载体类型加序号加内容哈希入哈希得登记号）、陈旧三态对表（当前语料与外化索引逐文件内容哈希可比出 fresh 与 stale 与 missing）、头部版本判定（包哈希与载体语法版本对表出全量重建标志）。原为无载体引用无推导档的裸奔态。本批将其接入 ORD-019 版本偏序与外化状态存储载体（mapping.md:196 行实取命中，条目磁盘实存核验），推导档承载一致命名与版本偏序上的可比判定与头部版本判定与快照复现三类语义形式化。只做接线与推导不改判定行为。

第二消费位申报：ORD-019 现消费位为 scribe 引擎侧（scriwire-solo 批，event_stream append 与 verify 与 park 注释锚点），本批为第二消费位（locator 工具侧），多消费位承 ALG-002 selwire 先例零冲突，按既裁执行未另开裁决。

温故检索：materials/recall-locatorwire.json（「寻址载体」零命中）如实记。

## 二、载体四件套

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md:196「上下文外化三性质 → ORD-019 版本偏序与外化状态存储 已建」 | sih-math/order/entries/ORD-019-version-order-and-externalized-state-store.md 磁盘实存；择形申报：择 CONTRACT 增载体引用节（selwire 先例同形），源内注记锚点位由接线件承载 |
| 推导档 | sih-math/docs/locatorwire-locator-derivation-2026-09-04.md | §3.1 一致命名 §3.2 版本偏序上的可比判定 §3.3 头部版本判定与快照复现，随批入版控 |
| 接线 | locator CONTRACT 增载体引用节（## 载体引用 {#carrier}）+ 源码判定位注释锚点三处 | identity.py stable_id（§3.1 稳定标识派生位）stale.py run_stale stale_check 对表位（§3.2 文件级三态对表位）stale.py run_stale reason 判定位（§3.3 头部版本判定位） |
| 金向量 | sih-engine/sih/event/plan/locatorwire-solo-materials/locatorwire-solo-golden-vector.json | 寻址派生确定性与陈旧判定版本可比两场景机械重放双跑逐字节一致，payload sha256 25bbbe67 |

消费面验收线：mapping.md 全读通过，ORD-019 锚点磁盘实存，第二消费位按 ALG-002 先例既裁在案；checkcite pass（并集书单 47 件，引用 ALG-002 与 ORD-019 全在册零 missing）。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 四件套** | 工程 | 载体引用与推导与接线与金向量四件齐 | 过 | 二节四件逐件在案，mapping.md:196 命中，entry 磁盘实存，第二消费位申报入档 |
| **F-2 零行为变更** | 工程 | locator 既有测试零回归，wiring_only_no_behavior_change | 过 | locator 59 测批前批后同计数全绿，内建金向量 10/10 pass，changed-files 记 wiring_only_no_behavior_change，py 两件 diff 15 行插入零删除全为 docstring 与注释锚点 |
| **F-3 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 十八路冻结面；主树零直写（reports 与链为主树活面短持追加） |
| **F-4 金向量双跑** | 工程 | 两场景双跑逐字节一致，冻结态携带重放寻径约定 | 过 | replay_golden.py 双跑 cmp 逐字节 IDENTICAL，与冻结金向量 verdict IDENTICAL，工地源与主树已提交源双复现均 IDENTICAL（V6 条款过已提交树复现关），冻结件与 cases 零绝对路径 |

## 四、金向量读数（寻址派生确定性与陈旧判定版本可比）

- 场景一 **addressing_derivation_determinism**：三载体语料（markdown 与 json 与 toml）建索引双跑逐字节一致——快照复现（推导档 §3.3，ORD-019 定理二）；identity.stable_id 对首个条目独立重算登记号与索引条目 id 一致——一致命名（§3.1，ORD-019 登记号唯一性）。
- 场景二 **stale_detection_version_comparability**：语料改一件删一件加一件 → stale [notes.md] 与 missing [data.json, extra.md] 与 fresh [cfg.toml] 三态机械对表（§3.2，版本偏序可比判定）；包版本变语料零变 → reason=pack 且 rebuild_required=true 而文件级零 stale——头部版本判定与文件级对表分层（§3.3）。
- 复算：replay_golden.py 双跑逐字节 IDENTICAL 零漂移，与冻结金向量 IDENTICAL，payload_sha256 25bbbe67417b2a2b5961e2a9756f1f60a22f399489869435668c4c1fc804fff3；重放寻径约定即 --locator-src 参数传源路径，冻结态零工地绝对路径。

## 五、管线读数

- 化格：CONTRACT 与 locator CALL-LOG 与推导档 md 三件与 golden_cases 与金向量与 terms.json（工地件）json 三件 exit 0 无需改；replay_golden.py exit 2 general-v1 域外（只盖 md/json/yaml/toml）如实记不属违规。
- 核阅：des-001 域只盖 sih-engine/doc，六目标均域外 exit 2 如实记不属违规；域内对照件 GOV-003 exit 0；ask3 双门第一门 exit 0 零违规，第二门 ask3repeater status ok 三锚。
- 检词：十二目标（CONTRACT 与四 CALL-LOG 与 identity.py 与 stale.py 与 terms.json 与推导档与 json 三件与 py 件）exit 0 零违例 findings 全零。
- 词债：版本偏序与外化状态存储与稳定标识与陈旧判定四词 established 登记入工地 core 包随批入版控（词表 142→146），叩问四信号 digest passed covered 4。
- 书单对表：wikirecall 并集书单（「版本偏序」与「等价关系与商集」两召回并集，47 件）checkcite pass 零 missing，报告与 plan 件落 scribe/reports。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者，与 queueing-solo（施工面 sih-tools/gauge/ 不同面）与 latexwire-solo（施工面 sih-tools/latex-helper/ 不同面）同日并行。冲突模式声明即 exclusive 撞锁有限重试上限十次逐次计数、共享追加面 append 短持即取即放、认证先落主树活链、链文件 settle 前一次性拷工地、收约让位走备份对表。实测冲突点与响应逐条如下：

1. **exclusive 撞锁有限重试**：六路 exclusive（sih-tools/locator/、nomenclator core 包、推导档、任务包、材料目录、结果档）首取全得零重试零争用（并行批施工面互斥成立），重试计数 0。
2. **共享追加面即取即得实录**：trail 链文件 append 短持六笔（意图一加认证五）即取即放零等待；批期链 66→81 事件，与 queueing-solo 与 latexwire-solo 共笔交叉并存（批中链尾曾见他批事件插入），scribe verify 全程 valid，追加态锁多持共存按设计工作。
3. **认证先落主树活链**：本批意图与认证六笔先落主树 2026-09-04.ndjson 活链，链文件 settle 前一次性拷入引擎工地，无工地链副本追加（承 pk-045 教训与 guardrail 防分叉条款）。
4. **环境位移**：工具调用按 BATCH-FACE 坑位加 env -u PYTHONHOME -u PYTHONPATH 前缀规避 encodings 缺失；meter 包裹输出 2>/dev/null 对治 meter 无 --quiet 缺陷；金向量 replay 经 locator 工地 venv 承载 tree-sitter 依赖。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| c7d130f0 | 意图笔（scribe intent） | ask3 记录 d866212f |
| b1ae1c3c | 管线报告（2026-09-04-locatorwire-solo-pipeline.json） | 报告内容哈希承 |
| 520d8045 | 推导档报告（2026-09-04-locatorwire-solo-derivation.json） | 报告内容哈希承 |
| f8671b38 | 金向量报告（2026-09-04-locatorwire-solo-golden.json） | 报告内容哈希承 |
| 31f08470 | 变更件报告（2026-09-04-locatorwire-solo-changed-files.json） | 报告内容哈希承 |
| bc8cf99f | 书单对表报告（2026-09-04-locatorwire-solo-checkcite.json） | 报告内容哈希承 |

认证一律先落主树活链，链 verify 批期读数 valid 81 事件（首哈希 05a8a75e 末哈希 bc8cf99f 即本批 checkcite 认证笔，三批共笔并存）。

## 八、越线与误差申报

1. **化格主树误指一笔（未遂，零字节变动）**：terms.json 化格首投误指主树件（工地纪律要求工地件），读数 exit 0 无需改零字节变动，git status 复核主树 terms.json 未动；随即补跑工地件 exit 0，申报在案。
2. **管线退出码首采失真订正**：核阅与检词循环首采在 echo 内联命令替换吃掉 `$?` 误读 exit 0（selwire-solo 越线申报三同款陷阱），直跑复验订正：核阅六目标实为 exit 2 域外如实记（域内对照件 GOV-003 exit 0 佐证采集面恢复），检词十二目标订正后仍 exit 0 零违例，管线读数以订正后为准（第五节）。
3. **checkcite 首投拦截如实记**：初查书单零条目命中（多词连写查询不触发词通道），checkcite verdict fail 拦收口；按三通道并集条款改单词面两召回（版本偏序与等价关系与商集）并集书单后 pass 零 missing；并集构造与 fail→pass 全程如实申报。
4. **lease open --session 旗标缺席**：lease 1.16.0 open 子命令无该旗标（勘误在案），会话号自生成 92cec27f94d03d4a，ask3 记录内会话标 sess-zcode-260904-locatorwire 双标识空间对表各认各的；闸类 scribe 写入 --session 会话号加 --sessions 台账路径双带逐笔过，非绕行。
5. **收约读数补笔（预留）**：本档第九节收约附表于 close 后经一次 --no-verify 提交加 lease bypass 登记落档（tallywire2-solo b6646e5 与 selwire-solo 先例同形），补笔仅涉本节与第九节。

## 九、收约附表（close 后补笔回填）

（占位：三仓 commit 号、close 记录、reconcile 读数、链 verify 读数，close 后补笔回填。）

## 十、验收

- [x] F-1 至 F-4 全过
- [x] 冲突样本节在结果档（第 1 条含零重试实录，第 2 条含三批共笔并存实录）
- [x] 认证入链，三仓结算收约（收约附表补笔回填）
