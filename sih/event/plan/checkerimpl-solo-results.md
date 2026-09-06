# checkerimpl-solo 结果档：检查器实装与包判定面机器化批

> 批：checkerimpl-solo（文规向界工程批，委外单线 solo，零子代理）
> 会话：f51895f3648c806c（lease 1.30.0，scope_source package）｜日期：2026-09-07
> 承接：用户 2026-09-07 令转发委外提示词；契约权威即孵化登记件检查器节（带日期不可变件不改文）

## 意图锚定

- 意图事件：intent_refined（event_hash `043c6977`）
- record／validation：sih-tools/scribe/reports/2026-09-07-ask3-checkerimpl-solo-record.json 与同名 validation（ask3repeater status ok 三锚）
- 三锚引文程序切片（03-on-second-tao.md L15、07-on-assay.md L69、08-on-settle.md L52）

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 零违规；ask3repeater status ok 三锚；叩问 15 信号全轻 digest passed covered 15/15
- 正身：identity attest anomalies 0（identity `8a2a473b`／core `82f460c2`）
- watch：净态无主零处；心跳两线 exit 0 零告警；gauge 三维落链
- **排队候位**：open 首跑被 preflight 拦（在途 confmath-solo 会话 82eac6da 持 facet／CALL-LOG／DES 交界面锁），候位两轮约 17 分钟零写入，对方收约后重开过（协调纪律按锁台账与时间戳判定，不按会话存在推断）

## 件读数：双工作面

### 工作面一：包判定面机器化（F-2）

- sdd-v1 五包三十条规则全带 `check` 机器字段：封闭操作词汇表 v0 全集（CONTRACT.md：十六原子操作加 any_of 组合子），操作全在词汇表内，词汇表外引擎拒载 exit 2
- 义械双载体：原中文 predicate 句保留为义面、check 为械面；参数显式（层级、正则、标记、引用源）
- manifest schema 版本进位 0.2.0，change 声明机器化；**登记件不改文**，契约补全申报见下专节

### 工作面二：检查器引擎实装（F-1、F-4）

- 落位 `sih-tools/checker/`（uv src 布局：src/checker/engine.py 加 cli.py、tests、fixtures、CONTRACT.md、pyproject）
- 空腹构成性：引擎只实现词汇表通用求值零文档形状知识；grep 测试（test_empty_stomach）机械证明 src 零文档特异字面（SHALL、Scenario、WHEN、THEN、判据、归因、回链、证据、范畴排除、规则 ID 形）——绿
- 三值退出码实测：绿五件 exit 0 findings 空；红样例 exit 1 且 findings 与预期一致（SL-003 violation 重号＋SL-007 missing 缺判据行）；缺席目标 exit 2；非法包 exit 2
- 双版本戳报告（engine 0.1.0 加 pack 版本）落 stdout，同参双跑 cmp 逐字节 IDENTICAL；无网络无隐式状态

### 先红后绿（F-3，本线自家纪律首次实战）

- 测试先行七测（机器化证、绿五件、红样例、缺席目标、双跑决定论、空腹 grep、golden 重放）先于实写成文
- 空实现上 pytest 5 failed——红证归档批材料 red-evidence/first-red-pytest.txt（exit 1）
- 实装至绿：pytest 7/7 passed exit 0；化格归一后复验仍绿

### D-4 围堰向量与 validate.py 退役（F-6）

- 冻结期望报告六件（绿五件加红样例）落包目录 `golden/`，重放测试全等绿；manifest 申报迁移点（基线向量管理工具建成后迁移）
- validate.py 按预设退役声明退役：移批材料 validate-retired-2026-09-07.py 留档；manifest self_validator.retired=true 指向 sih-tools/checker/

## 契约补全申报节（登记件不改文通道）

1. **判定面机器化**：登记件契约草案「判定面逐条规则带失败定位声明」的承载形升格为 check 机器字段（封闭词汇表），manifest schema 版本 0.1.0 进位 0.2.0。此为登记件契约的补全非推翻：三态失败定位（missing、violation、broken_ref）与三值退出码与双版本戳逐项与登记件一致。
2. **CLI as-built 披露**：登记件 CLI 最小形之外的实际形为 `--pack <单包.json> <目标> [--reference <被引用文档>]`——包族目录形 v0 不支持（按包逐目标由调用方编排）；--reference 为跨文档引用闭包的最小扩参。两笔 as-built 出入入 CONTRACT.md，属最小形的收敛实现不属语义变更。
3. **围堰向量家位**：D-4 必携位的向量以围堰形态落包目录 golden，基线管建成后迁移申报（已知债，manifest 在案）。

## 登记件验收判据对表节

| 登记件判据 | 实测 | 结果 |
|---|---|---|
| 判据一三刀判定 | 红样例（重复编号、缺判据行）落 violation／missing 对应态，exit 1；未独立成件由场景清单独立件设计承载 | 过 |
| 判据二 D-4 必携 | 挂锚规则 v0 为零（五包出处面 d4_anchor_slot.required=false 显式声明），违规路径以引擎拒载与空槽位声明承载；向量围堰位已建 | 过（含申报） |
| 判据三三值退出码 | 合规件零、红样例一、包非法件二，逐件实跑 | 过 |
| 判据四双跑一致 | 同参形两跑 cmp IDENTICAL 加退出码一致 | 过 |
| 判据五幂等只读 | 判定前后目标内容哈希不变（主树现态实测 b4af966b 前后等） | 过 |

## 管线读数

- 化格：json-canonical-v1 对六包加 golden 六件归一（exit 1 归一落笔属治理窄域正常）；general-v1 对 CONTRACT 归一
- 核阅：des-001 对批件全 exit 2 域外如实记档（域只盖 sih-engine/doc）
- 检词：core 首跑 exit 1（懒波词面"基线管"两处）改全称表述后 exit 0 零违例
- 温故 recall 加 checkcite：exit 0 pass（批产出零数学引用，零引用形如实申报）

## 一裁读数

- 命题 m-checker-impl-1：facet 九发 9/9 comply、变卦 0%、谨慎信号 0；同席采样谱系双声明载 topic authored 行
- 席位基线：ledger 尾行为 09-06 行，按 regulamath 先例手工制当日基线（identity 8a2a473b／core 82f460c2 配对）
- tally assemble stable_clear；attractor check 裁决通过 failed 0；verify identical；**sign b848e41e**（doc_id crosscheck-m-checker-impl-1）

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 三态实测 | 工程 | 绿五件零、红样例一 findings 合预期、缺席二、双跑 IDENTICAL | 通过（CLI 逐件实测） |
| F-2 包机器化 | 工程 | 三十条全带 check 且词汇表内，manifest 进位申报 | 通过（测试断言加 30 条清点） |
| F-3 先红后绿 | 治理 | 红证在档后全绿 | 通过（5 failed 红证归档→7/7 绿） |
| F-4 空腹证明 | 工程 | grep 测试绿 | 通过（test_empty_stomach） |
| F-5 终签在链 | 治理 | stable_clear 终签入链 verify valid | 通过（b848e41e） |
| F-6 写入仅 allow | 治理 | 零租约零引擎零 sih-math；validate 退役；对表入档；遗留零触碰 | 通过 |

## 越线与误差申报

1. settle 首跑 tools 侧 --repo 路径双拼 fatal（shell 变量拼差错位），修正重跑过；连带的 engine 工地 git add 被 && 链吞，settle nothing_staged 后补 add 重跑过——两笔首跑失败读数即红证在案
2. 开发期三次中途红（child op 参数缺项 KeyError、list op marker 拼接双空格、测试路径少一层）均在实装至绿窗口内修复，先红后绿纪律的首跑红证以空实现 pytest 归档件为准，中途红随批材料如实申报
3. 主树 DES 单元格与合同目录与 CALL-LOG 工地写均为 allow 面内；收约后幂等判据五实测在主树现态补跑（工地已拆）
4. reconcile：双仓 unrouted 0；engine cert_missing 0；tools cert_missing 1 与 unbypassed（80/49）为历史累计账面项，本批贡献即 closeguard 兜底一笔，相比批前零新增

## 收口读数

- 双仓 settle：tools `3a3dc342`（归并 `f1498de3`）、engine `2378b0b`（closeguard `4d8da61`＋归并 `a292382`）；cert 取 `1bb73adb`
- 放锁收约：十二锁 unlock 全放零失败；close exit 0 会话 f51895f3648c806c revoked
- 链 verify：2026-09-07 当日链 valid，last `1bb73adb`
- 本结果档经收约补笔 bypass 通道入版控（先例同形）
