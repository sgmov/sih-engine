# redkeep-solo：五个既有测试红修复

> 治理任务包（工程类，单线形 solo，DEC-018；**委外执行**：与 toolincub-solo 同一委外令、同一代理**串行**执行，本批在后即 toolincub-solo 收约后方可开工；与在途 regulamath-solo 数学批并行，路径零交叠）
> 承接：用户 2026-09-06 令「同意」；红源即 kernelmerge-solo 批验出并建议另开批修复（结果档 2026-09-06 在案，建议批名 redkeep 即本批）
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 引擎 cargo test 基线存五个既有红（kernelmerge-solo 批材料 cargo-baseline.txt 在案：165 passed 加 5 failed 加 6 ignored），全部位于 scrutinator::tests
- 两道门实装批需要绿基线：TDD 门以 cargo test 族退出码为判据，红基线不修则门无法立
- 五红是历史问题非当日批引入（kernelmerge-solo 零漂移判据已证归并前后红集合一致）

## 二、关键设计 {#design}

### 2.1 五红实名（修复对象清单）

1. scrutinator::tests::cli_positional_and_flag_forms_byte_identical
2. scrutinator::tests::cli_positional_form_matches_golden
3. scrutinator::tests::exit_compliant_zero
4. scrutinator::tests::golden_des001_gov002
5. scrutinator::tests::multipack_attribution_pack_names

开工先全量 cargo test 复现，红名单与本清单对表：一致即按清单修；出现清单外新红即停批上报（基线漂移）；清单内红已自愈即如实记档只修余红。

### 2.2 修复纪律

- 逐红先复现留痕（修复前输出入批材料），根因归三类：测试过时（断言落后于已裁行为）、实现缺陷（行为偏离已立契约）、环境依赖（路径、二进制版本类）
- 最小修复：修一跑全，零顺手重构零无关改动
- **判据不改**：若根因是测试断言本身错误，如实申报候人节点裁处置，不私改断言不删测试；改断言即改判据，属越权
- 修复后读数目标：0 failed，passed 数不小于 170（165 加 5），6 ignored 不变，零新增测试除非修复必需并逐件申报

### 2.3 行为面红线

五红全在核阅测试域。若修复须触碰核阅在役判定行为（退出码语义、规则判定结果），即停批上报——本批修测试基线不是改判定语义；行为变更属另批另裁。

### 2.4 工具版本注记

lease 用主树 1.30.0（1.29.0 argparse bug 即 --trail 重复定义已由后续批修复，kernelmerge 勘误在案）；引擎件合并主树后先重编再调用（旧二进制按旧码作答，BATCH-FACE 勘误）。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门、叩问、正身、watch 对表（在盘遗留如实转述不代清）、泊界心跳两线

### Cluster 2：修复施工

- [ ] 租约开工：--package redkeep-solo，双仓工地（sih-engine base main、sih-tools base integral-stage-build），--allow 按请求写入节；零 sih-math 工地
- [ ] 全量 cargo test 基线复现，红名单对表 §2.1
- [ ] 逐红修复（先复现留痕、根因归类、最小修复、修一跑全）
- [ ] 修复后全量读数：0 failed、passed 不减、ignored 不变

### Cluster 3：管线与结算

- [ ] 改动文件过化格 general-v1 与检词 core（Rust 源码对核阅 des-001 域外 exit-2 如实记档，kernelmerge 先例同形）
- [ ] 温故 recall 底稿；结果档 redkeep-solo-results.md 落 event/plan（含逐红根因归类与修复摘要与前后读数）
- [ ] 双仓 settle → 放锁收约（撞主树同名未跟踪件按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔 → 完工报告

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 五红清零 | 工程 | cargo test 0 failed，五红名逐一转绿在档 |
| **F-2** 零回归 | 工程 | 原 165 passed 全仍绿，6 ignored 不变，零清单外新红 |
| **F-3** 先红留痕 | 治理 | 修复前五红复现输出入批材料，红名单与 kernelmerge 在案清单对表一致 |
| **F-4** 判据零私改 | 治理 | 测试断言零删除零弱化；断言必改即停批申报候裁 |
| **F-5** 写入仅 allow 且不越权 | 治理 | 写入仅请求写入节所列；零 sih-math 写入；在泊件与在盘遗留零触碰；串行序即 toolincub-solo 收约后才开工 |

## 五、必读文件 {#read}

- 红源在案：`sih-engine/sih/event/plan/kernelmerge-solo-results.md` 与 `kernelmerge-solo-materials/cargo-baseline.txt`
- 命令面：`sih-tools/BATCH-FACE.md`（勘误含重编与退出码直读）
- 引擎测试位：`sih-engine/src/scrutinator/`（五红所在域）

## 六、约束 {#constraints}

1. §2.3 行为面红线：触碰在役判定行为即停批上报
2. §2.2 判据不改：断言必改即停批申报
3. 最小修复零顺手重构
4. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
5. 零 sih-math 写入；在泊件与在盘遗留零触碰
6. 串行序：toolincub-solo 未收约即不开工

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 redkeep-solo-results.md 落 event/plan 含逐红根因账

## 八、风险点 {#risks}

- 五红若同源于一个共同根因（如 golden 件过期一批），修复面会集中，逐红留痕防一并带过
- 修复若需重生成 golden 件，重生成程序与旧件差异逐字节对表入批材料
- 与在途批共享当日链：close 归并遇链活面按纯追加并集超集通道，真分叉停批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-tools/lease`、cargo 即引擎仓构建

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/redkeep-solo.md`（批输入件经工地落位）
- `sih-engine/src/`（五红修复所必需的最小面）与 `sih-engine/src/scrutinator/` 测试域
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-engine/sih/event/plan/redkeep-solo-results.md` 与 `redkeep-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 redkeep-solo 工地
