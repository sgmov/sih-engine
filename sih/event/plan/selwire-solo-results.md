# selwire-solo 结果档

> 批：selector 谓词划分载体接线批（ALG-002 承接，mathpipe-full 程序批四起收官件二）
> 会话：c28da6a8cb7bbc53（sess-zcode-260904-selwire）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理；冲突模式成员（pk-045 样本库，与 tallywire2-solo 同跑）
> 承接：mathpipe-full-program-v1.md 批四节即 selector 对挂谓词划分；用户裁定 2026-09-03 政策行「等我裁的东西首先过得一，得一有问题的异常才让我看」
> 意图哈希：a9eb2bbde2bd2680703a102412f90996c0a4ff5bcbd0e45c97b60dff0799d8f4（会话台账 intent.record_sha256 对表一致，ask3 验证件 status ok 三锚，意图事件 9c3601c9）

## 一、问题还原

selector 逐件机械判定主线、停放、丢弃三路，判定面即谓词族对材料集的划分（等价类分块与代表选取），原为无载体引用无推导档的裸奔态。本批将其接入 ALG-002 等价关系与商集隔离载体（mapping.md:197 命中），推导档承载谓词族等价关系定义、划分完备性与互斥性、代表选取与规范形。只做接线与推导不改判定行为。

零命中判归申报：词面「谓词划分」在 sih-math/llm-friendly-build/mapping.md 检索零命中，已显式申报并裁归商集第二消费位（idwire 双载体先例），本批即 ALG-002 第二消费面，按包内既裁执行未另开裁决。

温故检索：materials/recall-selwire.json 双档零命中如实记。

## 二、载体四件套

| 件 | 落位 | 证据 |
|---|---|---|
| 载体引用 | mapping.md:197「选择性隔离与防污 → ALG-002 等价关系与商集隔离 已建」 | sih-math/algebra/entries/ALG-002-equivalence-relation-and-quotient-isolation.md 磁盘实存；零命中判归申报入 CONTRACT 载体引用节与本档第一节 |
| 推导档 | sih-math/docs/selwire-selector-derivation-2026-09-04.md | 承载谓词划分语义形式化 §3.1 谓词族诱导等价关系 §3.2 划分完备性与互斥性 §3.3 代表选取与规范形，随批入版控 |
| 接线 | selector CONTRACT 增载体引用节（## 载体引用 {#carrier}）+ 源码判定位注释锚点 | pack.py load_pack(§3.1 谓词装载位) route.py route_material(§3.2 逐件判定位) route.py 首败定路与 pass_route(§3.3 三路归位位) |
| 金向量 | sih-engine/sih/event/plan/selwire-solo-materials/selwire-solo-golden-vector.json | 同特征归同路与边界材料谓词分界定位两场景机械重放逐字节一致（F-3），payload sha256 c4fcbb51 |

消费面验收线：mapping.md 全读通过，ALG-002 锚点磁盘实存，零命中判归按既裁在案。

## 三、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 四件套** | 工程 | 载体引用（含零命中判归申报）与推导与接线与金向量四件齐 | 过 | 二节四件逐件在案，mapping.md:197 命中，entry 磁盘实存，零命中判归入档 |
| **F-2 零行为变更** | 工程 | selector 既有测试全绿，wiring_only_no_behavior_change | 过 | selector 140 测全绿（批前批后同计数同 failures），changed-files 报告 change_type=wiring_only_no_behavior_change，diff 仅 docstring 注释锚点与契约节与 terms.json 词债一行，判定谓词与路由逻辑零改动 |
| **F-3 金向量** | 工程 | 两场景双跑逐字节一致 | 过 | replay_golden.py 双跑 cmp 逐字节 IDENTICAL，与冻结金向量 verdict IDENTICAL，两场景 scenarios_ok true |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 三仓工地 git status 对表未越 allow 冻结面（allow 十一路全数在包内） |

## 四、金向量读数（同特征归同路与边界分界定位）

- 场景一 **same_feature_same_route**：材料 mat-a 与 mat-b 判定特征全同（四谓词全过）→ 双双 mainline，same_route true 即同类——等价关系机械重放（推导档 §3.1）。
- 场景二 **boundary_predicate_localization**：材料 mat-c 锚点含出白名单项 → 首败谓词 g_anchor 机械定位分界，route siding、failed_predicate g_anchor、边界 checks pass false——类边界谓词定位机械重放（推导档 §3.3）。
- 复算：replay_golden.py golden_cases.json 双跑 cmp 逐字节 IDENTICAL 零漂移，与冻结金向量 IDENTICAL，payload_sha256 c4fcbb51a1340f325de7e06e33a51f660b9d5e6f46abd2c9d2bcc98b1bec3f8e。

## 五、管线读数

- 化格：CONTRACT 与推导档 md 两件与 terms.json 与金向量 json 两件 exit 0 无需改；py 三件（pack.py、route.py、replay_golden.py）general-v1 域外（只盖 md/json/yaml/toml）exit 2 如实记不属违规。
- 核阅：des-001 域只盖 sih-engine/doc，七目标均域外 exit 2 如实记不属违规；ask3 双门第一门 exit 0 零违规，第二门 ask3repeater status ok 三锚。
- 检词：CONTRACT 与推导档与 py 三件与 json 两件七目标 exit 0 零违例，基线与批后同为零 findings。
- 词债：谓词划分（Predicate Partition）一词 established 登记入工地 core 包随批入版控（词表 124→125），叩问信号一信号（谓词划分轻信号）digest passed covered 1。

## 六、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者，与 tallywire2-solo（会话 be02815afcd3149f，施工面 sih-tools/tally/ 不同面）同跑。冲突模式声明即 exclusive 撞锁有限重试上限十次逐次计数、共享追加面 append 短持即取即放、认证先落主树活链、串文件 settle 前一次性拷工地、收约让位走备份对表。实测冲突点与响应逐条如下：

1. **exclusive 撞锁有限重试（本样本第 1 条）**：本批 exclusive 五路首取，三路（sih-tools/selector/、批件 results/materials/plan 三路）零争用即得；sih-math/docs/ 第 1 次 locked_elsewhere（tallywire2 持）、第 2 次获让位释放，重试计数 2；sih-tools/nomenclator/packs/core/ 第 1、2、3 次 locked_elsewhere、第 4 次获让位释放，重试计数 4。全部在十次上限内，未绕行未加时，锁账本 acquired/released 时间戳对表在案。
2. **共享追加面即取即得实录**：trail 链文件、meter/counts、scribe/reports 三面 append 短持笔笔取放零等待；同日链双批并存实测——2026-09-04.ndjson 中本批意图 9c3601c9 与认证 97e5dd5a/2bf86016/cb60ef51/5f6dd712 五笔与 tallywire2-solo 意图 3d00ea6c 及认证四笔交叉并存，scribe verify valid，追加态锁多持共存按设计工作。
3. **认证先落主树活链**：本批认证四笔先落主树 2026-09-04.ndjson 活链，链文件 settle 前一次性拷入引擎工地，无工地链副本追加（承 pk-045 教训与 guardrail 防分叉条款）。
4. **环境位移**：工具调用按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀规避 encodings 缺失；meter 包裹输出 2>/dev/null 对治 meter 无 --quiet 缺陷。

## 七、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 9c3601c9 | 意图笔（scribe intent） | ask3 记录 a9eb2bbd |
| 97e5dd5a | 管线报告（2026-09-04-selwire-solo-pipeline.json） | 报告内容哈希承 |
| 2bf86016 | 推导档报告（2026-09-04-selwire-solo-derivation.json） | 报告内容哈希承 |
| cb60ef51 | 金向量报告（2026-09-04-selwire-solo-golden.json） | 报告内容哈希承 |
| 5f6dd712 | 变更件报告（2026-09-04-selwire-solo-changed-files.json） | 报告内容哈希承 |

认证一律先落主树活链，链 verify 批期读数 valid 13 事件（例行读数三笔加双批意图二笔加双批认证八笔，双批并存各五笔），链尾哈希 5f6dd712 即本批变更件认证笔。

## 八、越线与误差申报

1. **lease open --session 旗标缺席**：dispatch 台面注记「显式 --session」，lease 1.15.0 open 子命令实测无该旗标（unrecognized arguments），会话号自生成 c28da6a8cb7bbc53，正身件与 ask3 记录内会话标 sess-zcode-260904-selwire 对应同批；闸类 scribe 写入的 --session 均带会话号在册验过，非绕行，按工具实态执行如实申报。
2. **unlock 缺 --identity 首投退出码二**：短持锁首笔 unlock 漏 --identity 报 usage 退出码二，补参重跑即 released，零状态残留，工具异常先处置在案。
3. **化格退出码采集失真一次**：py 件化格循环内命令替换吃掉 `$?` 致首采误读 exit 0，直跑复验实为 exit 2 域外，全五件重采订正，管线读数以订正后为准（第五节）。
4. **收约读数补笔**：close 归并与 reconcile 与终 verify 读数于收约后经一次 --no-verify 提交加 lease bypass 登记补笔落本档（red line 明示通道），补笔仅涉本档收约附表节。

## 九、收约附表

三仓 settle 段 1 与放锁收约与 reconcile 与链 verify 终读数见本档末节收口读数（收约后补笔）与 lease CALL-LOG 行。

## 十、验收

- [x] F-1 至 F-4 全过
- [x] 冲突样本节在结果档（第 1 条含 retry 计数，第 2 条含追加态双批并存实录）
- [x] 认证入链，三仓结算收约，对表读数在档（收约附表）
