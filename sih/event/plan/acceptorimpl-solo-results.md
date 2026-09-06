# acceptorimpl-solo 结果档：TDD 验收工具实装与判定包首例批

> 批：acceptorimpl-solo（文规向界工程批，委外单线 solo，零子代理）
> 会话：f373819d5719bf66（lease 1.30.0，scope_source package）｜日期：2026-09-07
> 承接：用户 2026-09-07 令转发委外提示词；契约权威即孵化登记件验收工具节（带日期不可变件不改文）

## 意图锚定

- 意图事件：intent_refined（event_hash `d5f5db74`）
- record／validation：sih-tools/scribe/reports/2026-09-07-ask3-acceptorimpl-solo-record.json 与同名 validation（ask3repeater status ok 三锚）
- 三锚引文程序切片（03-on-second-tao.md L15、07-on-assay.md L69、08-on-settle.md L52）

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 零违规；ask3repeater status ok 三锚；叩问 9 信号全轻 digest passed covered 9/9
- 正身：identity attest anomalies 0（identity `06fe3cf7`／core `82f460c2`）
- watch 净态无主零处；心跳两线 exit 0 零告警；gauge 三维落链
- 零撞锁一次开工（前批排队纪律与协调判定吸收）

## 件读数：双工作面

### 工作面一：TDD 判定包数据形（F-2 词汇表封闭）

- 首例判定包 `sih-tools/acceptor/packs/tdd-v0-sddchecker.json`：四查各声明操作与参数（命令串 token 数组、{ROOT}/{PACK_DIR} 占位、cwd 参数、冻结期望件、红证哈希加红证件、场景清单加判据语法参数加 judge_map）
- 操作全在封闭词汇表 v0（CONTRACT.md 全集四操作：double_run、baseline_freeze、red_then_green、scenario_coverage），词汇表外引擎拒载 exit 2

### 工作面二：引擎实装（F-1、F-4）

- 落位 `sih-tools/acceptor/`（uv src 布局照 checker 先例）：src/acceptor/engine.py 加 cli.py、tests 七测、fixtures 两坏样例加漂移期望件、packs 首例判定包、frozen 全报告冻结件、CONTRACT.md、pyproject
- **空腹构成性**：grep 测试（test_empty_stomach）机械证明引擎 src 零项目特异字面零命令字面零规则编号字面——绿
- **三值退出码三态实测**（F-1）：首例判定包对 sdd-v1 与 checker 线四查全过 exit 0；构造漂移样例 exit 1 且 findings 命中 TC-002 violation 带原始事实（差异偏移与字节数摘要，不归因）；坏红证哈希样例 exit 1 且命中 TC-003 missing 缺件态；缺判定包 exit 2
- **双跑决定论**：acceptor 同参两跑 stdout 逐字节一致
- **甲乙红线入报告**：declaration 声明范围字段 claim=unchanged-only（基线类查只主张没变不主张对），漂移查只报原始事实

### 首例判定包自反狗粮闭环（F-2）

- 双跑与冻结吃检查器线 golden 素材（sdd-v1 场景清单经检查器判定，冻结全报告本批自建 frozen/ 分位）
- 红证吃 checkerimpl 批材料红档（first-red-pytest.txt，哈希 `29b9bac3` 冻结进判定包，跨批**只读引用零触碰**原档）
- 覆盖吃 sdd-v1 场景清单五场景判据行，judge_map 承载判据程序寻径改指（validate.py 退役由检查器接管，manifest 0.2.0 指向声明），寻径核对在案
- 验收工具验判定机，判定机的素材喂验收工具——自反闭环成立

### 先红后绿（F-3）

- 测试先行七测先于实装成文；空实现 pytest 3 failed 红证归档批材料 red-evidence/first-red-pytest.txt（exit 1）
- 实装至全绿：pytest 7/7 passed exit 0；化格归一后复验仍绿

## v0 范围申报节（诚实边界，双申报之一）

- **三态归因全量机制是后继不是本批**：漂移查 v0 只报原始事实（漂了、首差异偏移、字节数），纯期望过期／真回归／工装抖动的三态归因属基线向量管理工具（登记件工作名）后继。
- **申报影响集包含判定**（漂移集与申报影响集的包含关系，SPEC-021 第三层）属同上后继；v0 判定包 schema 不含 impact_set 字段。
- CONTRACT.md 与本结果档双申报，不伪装已全。

## 登记件验收判据对表节

登记件验收工具节验收判据逐条对表：

| 登记件判据面 | 实测 | 结果 |
|---|---|---|
| 四查即数据、逐查退出码 | 首例判定包四查逐查跑逐查记，聚合退出码三值 | 过 |
| 甲乙红线（基线类只主张没变） | declaration.claim=unchanged-only 入报告，漂移不归因 | 过 |
| 空腹（规则来自包） | grep 测试零项目字面零命令字面，绿 | 过 |
| 三值退出码 | 0／1／2 逐件实测 | 过 |
| 双跑一致 | acceptor 同参两跑 cmp IDENTICAL | 过 |
| 报告双版本戳 | engine 0.1.0 加 pack 0.1.0 在报告 | 过 |

## 管线读数

- 化格：json-canonical-v1 对判定包三件归一（exit 1 属治理窄域正常）；general-v1 对 CONTRACT 归一；**frozen 全报告冻结件不归一**——逐字节冻结物非规范 JSON 文档，与 checker 线精简形 golden 的重放比对本质不同，选择性申报
- 核阅：des-001 全域外 exit-2 如实记档
- 检词：core 首跑 exit 0 零违例（受检文档用 TDD 验收工具指称规避懒波词面）
- 温故 recall 加 checkcite：exit 0 pass（零数学引用形如实申报）

## 一裁读数

- 命题 m-acceptor-impl-1：facet 九发 9/9 comply、变卦 0%、谨慎信号 0；同席采样谱系双声明载 topic authored 行
- 席位基线：当日正身制（identity `06fe3cf7`／core `82f460c2` 配对）
- tally assemble stable_clear；attractor check 裁决通过 failed 0；verify identical；**sign ac7feffa**（doc_id crosscheck-m-acceptor-impl-1）

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 三态实测 | 工程 | 全过零、漂移一、坏红证缺件、缺包二 | 通过（逐件实测） |
| F-2 首例跑绿 | 工程 | 首例判定包四查全过零，报告含声明范围字段 | 通过 |
| F-3 先红后绿 | 治理 | 红证在档后全绿 | 通过（3 failed 归档→7/7 绿） |
| F-4 空腹证明 | 工程 | grep 测试零项目字面零命令字面 | 通过（test_empty_stomach） |
| F-5 终签在链 | 治理 | stable_clear 终签入链 verify valid | 通过（ac7feffa） |
| F-6 写入仅 allow | 治理 | 零租约零引擎零 sih-math 源码；红证只读零触碰；对表入档 | 通过 |

## 越线与误差申报

1. 开发期中途红五笔（__init__ docstring 拼接未闭合 SyntaxError、TC-004 校验器对无命令操作误一刀切、{PACK_DIR} 层位错两笔、被检命令 cwd 缺失与 -m 形缺失）均在实装至绿窗口修复；先红后绿首跑红证以空实现归档件为准
2. checker 命令形实勘：脚本直跑 sys.path[0] 不含 cwd，判定包命令以 `-m checker.cli` 加 cwd 参数承载（包数据修正非引擎改动）
3. 差集闸拦截一笔（declared_uncommitted：结果档未提交即 close 被拒）——租约新闸首次实战拦截本批，按闸指引先提交结果档再 close，零绕行；此即先前申报的租约硬化缺口已修的实证
4. TC-003 的 uv run 会在主树 checker 下生成 .venv 运行时缓存（cargo target 缓存先例同款，非治理写入），如实申报
5. reconcile 与终验读数随补笔回填

## 收口读数

- 双仓 settle：tools（归并在档）、engine（归并在档）；cert 取 `0f3405c8`；结果档随 seq 2 补提交
- 放锁收约：十一锁 unlock 全放零失败；close 经差集闸拦截后按指引补提交重跑
- 本结果档经收约补笔 bypass 通道入版控（先例同形）
