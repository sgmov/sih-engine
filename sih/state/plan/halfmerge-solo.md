# halfmerge-solo：谓词机器半归并方向一裁与委外件起草

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-06 令「过得一裁，过了出任务包和提示词，委外执行」即半归并方向（内核归一＋命令面保持＋文规骑统一内核）经机器裁决，裁过即出内核统一批任务包与委外提示词，由用户转发委外代理执行
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 半归并方向停在会话对话，无机器裁决无链上落据，违背裁决路由三态之灰区从严过得一（DEC-021）
- 谓词求值内核统一悬案 pk-060 在泊，其出泊条件即用户裁统一与否与形态；本批机器门与用户开工令构成双依据
- 内核统一批的执行件（任务包与委外提示词）未起草

## 二、关键设计 {#design}

### 2.1 单锚归约命题

命题 gid m-halfmerge-1，单基线锚 baseline_4 可验证性：内核归一的成立性由金向量双跑逐字节零漂移与 CLI 契约重放零漂移机械证成。防依据族三值分散复发（gvec-method 前鉴）。

### 2.2 席位亲笔温度零

九发由本席（ZCode:GLM-5.3:self-reported）即席亲笔作答，温度零，谱系披露双向（提出者立场偏向 comply 如实披露）。当日席位基线以温度探针批内自跑标定（gvec-v2 前一日旧基线致 sign refused 前鉴）。

### 2.3 双态可证伪

stable_clear 即出委外件两件与终签落据；boundary 或 refused 即泊位保持零委外件，如实呈报不硬闯。

### 2.4 委外件两件

kernelmerge-solo.md（内核统一批任务包，含 pk-060 出泊写入条款）与 kernelmerge-solo-prompt.md（委外提示词），仅 stable_clear 后落位。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 命题 topic.md 落工地 facet 合同目录，attractor emit-contract 九发
- [ ] 九发亲笔作答与温度探针当日标定，attractor score 出计分材料
- [ ] attractor check 与 verify 与 sign（stable_clear 即 crosscheck_completed 落链）
- [ ] stable_clear 后委外件两件落位 state/plan
- [ ] 关键报告件认证入链（ask3 记录、验证件、正身件、委外件两件）

### Cluster 2：主线串行验证

- [ ] 管线三步跑任务包与结果档与委外件（state/plan 属 des-001 域外，如实记档）
- [ ] 双仓 settle、放锁收约、reconcile 与链 verify（退出码直读禁管道掩码）
- [ ] 泊界心跳复算零告警

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 一裁终签 | 治理 | m-halfmerge-1 九发经引擎 attractor，stable_clear 即 crosscheck_completed 在链；闸不过即零委外件零终签如实呈报（双态可证伪） |
| **F-2** 委外件仅裁后 | 治理 | kernelmerge-solo.md 与提示词仅在 stable_clear 后落位，闸不过即不落位 |
| **F-3** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-4** 谱系不遮蔽 | 治理 | 命题内谱系披露双向如实，九发亲笔温度零，依据族单锚不分散 |

## 五、必读文件 {#read}

- 方向源：本会话半归并讨论（域外即对话）
- 先例：`sih-engine/sih/event/plan/gvec-v2-serial-results.md`（单锚重裁全流程与当日基线坑）
- 命令面：`sih-tools/BATCH-FACE.md` 第五节判定调用面

## 六、约束 {#constraints}

1. 零代码改动：引擎与工具源码零触碰，仅命题、合同、计分、委外件与链
2. 闸不过即停：boundary 或 refused 即不出委外件不硬闯，pk-060 泊位保持
3. 在泊件除零触碰外不并批；pk-060 出泊写入归 kernelmerge 委外批不归本批
4. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
5. 守卫在位禁 plain git commit，close 通道提交
6. 在盘遗留无主件不豁免不代清，本批不新增无主写

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过（F-1 双态即 stable_clear 全过形或闸停形，均如实）
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 对照批前零新增（退出码直读）
- [ ] 结果档 halfmerge-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 席位当日基线缺失或旧账本尾行即 sign refused（gvec-v2 前鉴）——批内自跑温度探针标定后复签
- 依据族分散复发即 boundary——命题单锚 baseline_4，九发作答依据族单值
- close 归并遇主树未跟踪任务包按备份让位归并对表法，真分叉停批上报

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/attractor`（emit-contract/score/check/verify/sign）、`sih-engine/target/debug/scribe`（intent/append/park 写位）、`sih-tools/facet/probes/temp_probe.py`（席位标定）
- 跨仓引用：`sih-tools/facet/contracts/halfmerge-260906/m-halfmerge-1/`、`sih-tools/proposition/DES/m-halfmerge-1/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/halfmerge-solo.md`
- `sih-engine/sih/state/plan/kernelmerge-solo.md`
- `sih-engine/sih/state/plan/kernelmerge-solo-prompt.md`
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-engine/sih/event/plan/halfmerge-solo-results.md`
- `sih-engine/sih/event/plan/halfmerge-solo-materials/`
- `sih-tools/facet/contracts/halfmerge-260906/`
- `sih-tools/proposition/DES/m-halfmerge-1/`
- `sih-tools/facet/probes/calibration/ledger.jsonl`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 halfmerge-solo 工地
