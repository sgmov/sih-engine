# gvec-v2-serial：金向量双种方法论重裁与两件出泊

> 治理任务包（立文类，**队形串行 serial**：主会起草与验收、单子代理持全量上下文连续执行链，承 sihankor-marshalling 硬性工作流与 DEC-018）
> 承接：用户 2026-09-06 令「开工，任务包完成后拉起子代理跑」，即同日会话金向量方向与数学选型讨论、六处修正（R1 至 R6）与三残留清账的定案执行
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 上回金向量方法论命题 gvec-method-guard-1 经九发重采因依据族三值分散（baseline_1×1／baseline_4×6／baseline_5×2）闸裁 boundary，落泊界 pk-046 待重订（2026-09-03）
- 本会话完成重设计与逐轮修正清账，定案形态即修正稿（一词两物、四层模型、六冻结对象、T1 至 T11），停在对话与批输入件，无重裁无成文
- pk-046（方法论载体与条款组重订）与 pk-068（规约测试与基线金向量双种拆分）两件出泊条件已由开工令承裁，待机械门即单锚重裁 stable_clear 终签

## 二、关键设计 {#design}

### 2.1 单锚归约重裁

命题 gid gvec-v2-guard-1，单基线锚即可验证性：每条条款只问能不能机械验，不问哲学该不该。防依据族分散复发。重裁闸不过即泊位保持如实呈报，不硬闯。

### 2.2 出泊双依据

出泊唯人节点已由开工令承裁（令源在档）；机器门即 attractor sign stable_clear 终签落据。两依据齐才出泊两笔。

### 2.3 号实取下一空位

SPEC-021 按仓例顺延（现役用至 020），承 contribmath-solo 先例，不经任何批的契约外增设。

### 2.4 分工边界

主会（主线）：起草修正稿与命题与任务包、开约锁链、验收 F 锚、收约补笔。子代理（串行）：施工成文、管线、facet 三步（出题／回填九发／计分）、得一核对与终签、出泊写入、结果档、双仓 settle。子代理不得代人行签核类节点。

## 三、工作清单 {#work}

### Cluster 1：子代理执行链（串行单代理）

- [ ] 工地落位：topic.md 即 gvec-v2-guard-1 命题（依批输入件 2026-09-06-gvec-v2-design.md 与命题模板先例 gvec-method-guard-1/topic.md 成文）入 sih-tools 工地 facet 目录；SPEC-021 成文入 sih-engine 工地 doc/spec/（正文底稿即修正稿，按 spec 文档形）
- [ ] 管线三步：SPEC-021 与任务包与结果档过化格（general-v1）与核阅（des-001，SPEC-021 属域内真核阅，退出码 0 才过；state/plan 与 event/plan 域外 exit-2 如实记档）与检词（core）
- [ ] facet 三步：emit-contract（零 LLM 零网络）→ 回填半（子代理即席作答九发，谱系披露同席位承先例）→ score 携正身件；计分材料复制入 proposition/DES/gvec-v2-guard-1/ 单元格
- [ ] 得一三步：attractor check → verify（重放 identical）→ sign（stable_clear 即落据，refused 或写链失败如实呈报）
- [ ] 出泊两笔（仅 stable_clear 后）：pk-046-exit.json 与 pk-068-exit.json 落工地，scribe park exit 两笔入链，exit 载裁决指向即开工令与终签哈希
- [ ] 结果档：起草前按事件与时间轴跑温故检索取切面为机械底稿（retriever recall --event --since 2026-09-03 --until 2026-09-06 --at 2026-09-06 --out 批材料），落 event/plan/gvec-v2-serial-results.md
- [ ] 双仓 settle（cert 取 ask3 记录认证哈希前八位）；工地 CALL-LOG 落笔

### Cluster 2：主线验收（主会）

- [ ] F 锚逐条实跑复核
- [ ] 放锁收约（unlock 全部、close，撞未跟踪件按备份让位归并对表法）
- [ ] 链 verify、reconcile 双仓、心跳复算（出泊后 pk-046/pk-068 出泊记录入废轨属常态）
- [ ] 结果档回填结算读数、收约补笔 bypass 通道

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 重裁终签 | 治理 | gvec-v2-guard-1 九发经引擎 attractor，stable_clear 即 crosscheck_completed 在链；闸不过即泊位保持如实呈报（双态可证伪） |
| **F-2** SPEC-021 成文 | 工程 | doc/spec/ 下 021 新件落位，化格核阅检词全过即 des-001 域内退出码 0，零绕行 |
| **F-3** 出泊两笔 | 数据治理 | stable_clear 后 pk-046 与 pk-068 出泊事件在链且 exit 件载裁决指向；闸不过则本条改判泊位保持且零出泊写入 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-5** 子代理不越权 | 编组治理 | 子代理零人节点代行、零绕行、零 plain commit、零预设重裁结论；主线验收 F 锚逐条实跑 |

## 五、必读文件 {#read}

- 修正稿（正文底稿）：`sih-tools/scribe/reports/2026-09-06-gvec-v2-design.md`
- 上回 boundary 原档：`sih-engine/sih/event/plan/gvec-method-solo-results.md`
- 命题形先例：`sih-tools/facet/facet_task_packages/gvec-method-guard-1/topic.md`
- SPEC 立项先例：`sih-engine/sih/event/plan/contribmath-solo-results.md`（号实取下一空位与判据命题终签流程）
- 出泊批先例：`sih-engine/sih/event/plan/pk013exit-solo-results.md`
- 命令面：`sih-tools/BATCH-FACE.md`（含 facet 三步与 attractor sign 调用形与坑位勘误）
- 温故检索输出件：`sih-tools/scribe/reports/2026-09-06-gvec-v2-serial-recall-pre.md`（落包前取，随批材料入档）

## 六、约束 {#constraints}

1. 重裁闸不过即停：boundary 或 violate 即不出泊不硬闯，泊位保持如实呈报
2. 子代理不得代人行签核类节点；出泊写入仅在 stable_clear 终签后执行
3. SPEC-021 过不了核阅即修到过，禁绕行禁裸奔
4. 在泊件除 pk-046 与 pk-068 外零触碰不并批
5. 数学仓扩条目为后继批不在本批，不越权成文
6. 零代码改动：引擎与工具源码零触碰，仅文档与泊材料与链
7. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
8. 守卫在位禁 plain git commit，close 通道提交
9. 在盘遗留无主件不豁免不代清，本批不新增无主写

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过（F-1 与 F-3 双态即 stable_clear 全过形或 boundary 停批形，均如实）
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 gvec-v2-serial-results.md 落 event/plan

## 八、风险点 {#risks}

- facet 回填由子代理即席作答即同席位采样，谱系披露必须在 topic 文内双声明（同席与成本加重），承 gvec-method-guard-1 先例
- attractor sign 相对路径按调用 cwd 解析即 --material 传相对路径且 cwd 置工地根（openhyg 勘误）
- SPEC-021 在 des-001 域内属真核阅，格式违规即修即重跑，不属越线
- close 归并遇主树未跟踪件（任务包与批输入件）按备份让位归并对表法，真分叉停批上报

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（intent/park/append 写位）、`sih-engine/target/debug/attractor`（check/verify/sign）、`sih-tools/facet/measure.py`（三步）
- 跨仓引用：`sih-tools/proposition/DES/gvec-v2-guard-1/`、`sih-tools/facet/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/gvec-v2-serial.md`
- `sih-engine/sih/state/parking/materials/`
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-engine/sih/event/plan/gvec-v2-serial-results.md`
- `sih-engine/sih/event/plan/gvec-v2-serial-materials/`
- `sih-engine/doc/spec/`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- `sih-tools/facet/`
- `sih-tools/proposition/DES/`
- worktrees 双仓 gvec-v2-serial 工地
