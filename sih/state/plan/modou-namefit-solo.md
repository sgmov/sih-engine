# modou-namefit-solo：共享求值芯立名墨斗与顶层注册批

> 治理任务包（立名类＋实装类合一，单线形 solo，DEC-018 承立名流程 skill 三段）
> 承接：用户 2026-09-06 裁「批准墨斗（英文对 snapline、代号 snapline）」；推导与反推全档在本会话立名会话（本质段七面、反候选三具死因、十二署名位反推会师、词场核验）
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 共享求值芯现以工作名 predkernel 在役（kernelmerge-solo 落地，#[path] 重映射寄居核阅命名空间），未立名未登记，违反命名次序裁定之收口义务
- 模块路径 crate::scrutinator::rule::predkernel 与目录 src/predkernel 名实不一，顶层注册缺位
- pk-060 出泊链上事件缺笔（kernelmerge 批只认证了 exit 件未写 parking_exited 事件，前查缺陷在案）

## 二、关键设计 {#design}

### 2.1 立名三件套

中文对墨斗、英文对 snapline、代号 snapline。英文对非翻译是平行表达（墨线对粉线，本体论体例）。两用名纪律随词条登记：重放句只写经消费方金向量间接证或单测直证；中文对不进文件名与代码标识符。

### 2.2 名实相副机器门

命题 m-modou-fit-1 单锚 baseline_4：名实对表面即本质段七面（共享、匹配原语、零语义、无胃、无命令面、确定性、家位首件），全部机械可证（碰撞零命中 grep 可复算、零语义面代码可审、注册后测试全绿零漂移）。

### 2.3 代码落地

src/predkernel 整目录改 src/snapline；lib.rs 顶层 pub mod snapline 注册；rule.rs 删 #[path] 重映射改 use crate::snapline；route.rs 引用同步；零漂移验收即 cargo test 全绿（五既有红集合不变）加金向量重放一致。

### 2.4 pk-060 补笔

既有 pk-060-exit.json（kernelmerge 落档、disposition promoted、裁决指向完备）经 scribe park 补写 parking_exited 链上事件，补齐泊界四字组停有痕。

### 2.5 家位边界

本批只立共享模块首件之顶层注册，不裁 pk-062 全局家位（单元级其余重复族仍归 pk-062 出泊裁），pk-062 零触碰。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 代码落地与零漂移验收（工地内）
- [ ] 命题 m-modou-fit-1 九发＋当日席位标定＋score＋check＋verify＋sign
- [ ] 词条登记 terms.json（立名即登记）＋反候选死因两条入 dead.json＋化格归一
- [ ] pk-060 park exit 补笔上链

### Cluster 2：主线串行验证

- [ ] cargo build 零新增警告、cargo test 零回归（五既有红集合不变）
- [ ] 管线三步、认证、双仓 settle、放锁收约、reconcile 与链 verify（退出码直读）
- [ ] 泊界心跳复算（pk-060 出泊后 exit 件入废轨属常态）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 立名终签 | 治理 | m-modou-fit-1 stable_clear 即 crosscheck_completed 在链；闸不过即名不立零登记零改写（双态可证伪） |
| **F-2** 零漂移 | 工程 | 改名注册前后 cargo test 结果集合一致（五既有红不变）、金向量重放一致、cargo build 零新增警告 |
| **F-3** 词条在册 | 数据治理 | terms.json 出现墨斗条目（zh/en/code/definition/source/signed 齐）且 nomenclator check 过；dead.json 增准绳绳墨两死因 |
| **F-4** pk-060 补笔 | 数据治理 | parking_exited pk-060 事件在链且与既有 exit 件裁决指向一致 |
| **F-5** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |

## 五、必读文件 {#read}

- 立名权威：`sih-philosophy/emanation/proodos/01-ontology-of-names.md`（体例与英文对理据）
- 流程：`.agents/skills/sihankor-naming/SKILL.md` 三段
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 零漂移红线：本批代码改动仅限模块路径与注册，任何行为面变化即失败
2. 闸不过即停：F-1 boundary 或 refused 即不登记不改写，如实呈报
3. pk-062 与其余在泊件零触碰
4. 主树零直写，链文件只经引擎 scribe 写位
5. 守卫在位禁 plain git commit
6. 在盘遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 对照批前零新增（退出码直读）
- [ ] 结果档 modou-namefit-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- lib.rs 注册触发既有测试对模块路径的引用失效——编译期即暴露，逐件修复属路径改写不算行为变化
- 席位当日基线须本批自跑（gvec-v2 前鉴）
- terms.json 收编后必须化格归一（packhyg 勘误）

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/{attractor,scribe}`、`sih-tools/facet/probes/temp_probe.py`、`sih-tools/nomenclator`
- 跨仓引用：`sih-tools/facet/contracts/modou-260906/m-modou-fit-1/`、`sih-tools/proposition/DES/m-modou-fit-1/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/modou-namefit-solo.md`
- `sih-engine/src/lib.rs`
- `sih-engine/src/snapline/`（自 src/predkernel 改名）
- `sih-engine/src/scrutinator/rule.rs`
- `sih-engine/src/attractor/route.rs`
- `sih-engine/sih/state/parking/materials/`
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-engine/sih/event/plan/modou-namefit-solo-results.md`
- `sih-engine/sih/event/plan/modou-namefit-solo-materials/`
- `sih-tools/facet/contracts/modou-260906/`
- `sih-tools/proposition/DES/m-modou-fit-1/`
- `sih-tools/facet/probes/calibration/ledger.jsonl`
- `sih-tools/nomenclator/packs/core/terms.json`
- `sih-tools/nomenclator/packs/core/dead.json`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 modou-namefit-solo 工地
