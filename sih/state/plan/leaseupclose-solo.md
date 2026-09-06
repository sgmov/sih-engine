# leaseupclose-solo：租约修复升级线收口批

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-06 令「一次性清掉」即租约线收口三件：pk-072 出泊事件补落、GOV-002 v2.4 第五条补落与达成追记、线程序包迁档结算
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 租约修复升级线五批实质工作已结算（closefix、leaseup、billwire、ledgerhyg、rootanchor），但三件收口文书未落：
- 其一 pk-072 出泊链事件缺失：leaseup-solo CONTRACT 修订四十一文述「pk-072 出泊 promoted 承载修复」而出泊材料 pk-072-exit.json 在档、链上 parking_exited 事件未落，账实不符
- 其二 GOV-002 v2.3 正文未随落：版本史行已落 v2.3，但换版所述正文即概览计数四改五、退出标准节增第五条、主线节承载段落俱未落，退条节仍四条
- 其三线程序包 leaseup-line-v1.md 仍在 state/plan 即线在系统眼中仍在飞，须迁 event/plan 归档结算

## 二、关键设计 {#design}

### 2.1 出泊材料承先例

pk-072-exit.json 由 leaseup-solo 备好在档，本批补一句迟到申报（链事件由本批补落、rootanchor 1.30.0 自举自卫与链证守门为后续加固面）后经引擎 scribe park 落 parking_exited 事件。

### 2.2 v2.4 三件套

补落 v2.3 换版所述正文（概览计数、第五条目、主线承载段落）、第五条达成追记与证据指针（CONTRACT 修订四十一至四十三、test_leaseup 四族与 test_rootanchor 红绿档、残留三项泊 pk-074 不属判据面）、差异申报两笔（v2.3 正文未落、pk-072 链事件后至）如实入版本史。冻结清单与范畴排除零字节改动。

### 2.3 迁档

leaseup-line-v1.md 自 state/plan 迁 event/plan，文件名零改动即路径迁移归档。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] GOV-002 v2.4 三处正文补落加版本史行
- [ ] leaseup-line-v1.md 迁 event/plan
- [ ] pk-072-exit.json 迟到申报补句后 scribe park 出泊上链

### Cluster 2：主线串行验证

- [ ] 管线三步跑 GOV-002（des-001 域内必须 exit 0）与任务包结果档线程序包（域外如实记档）
- [ ] 双仓 settle、放锁收约、reconcile 与链 verify、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 出泊链事件落 | 数据治理 | 当日链 parking_exited pk-072 一笔，disposition promoted，重入拒零触发 |
| **F-2** 第五条在档可判 | 数据治理 | GOV-002 退条节第五条目在档且携达成追记与证据指针，概览计数五条一致，核阅 des-001 exit 0 |
| **F-3** 线包归档 | 数据治理 | leaseup-line-v1.md 在 event/plan 在版控，state/plan 原位清空 |
| **F-4** 差异申报在档 | 治理 | 两笔账实不符（v2.3 正文未落、pk-072 链事件后至）在版本史与结果档俱如实申报，不修饰 |

## 五、必读文件 {#read}

- 线程序包：`sih-engine/sih/state/plan/leaseup-line-v1.md`
- 追记正典形：`sih-engine/doc/governance/GOV-002-mainline-lock-v1.md` 退条节第一条达成追记先例
- 出泊正形：`sih-engine/sih/state/parking/materials/pk-071-exit.json`

## 六、约束 {#constraints}

1. 零代码改动、零在役判据与退出码语义触碰、零 LLM 调用
2. 在泊件零触碰（pk-072 出泊材料除外即本批对象），出泊唯人节点即本批令源即用户「一次性清掉」令
3. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
4. 守卫在位禁 plain git commit，close 通道提交
5. 冻结清单与范畴排除零字节改动

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 leaseupclose-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- GOV-002 属 des-001 域内，v2.4 文字若不合规则包即 exit 1——以在役版本史行文风格为准逐句对形
- 收约归并遇主树未跟踪任务包即按备份让位归并对表法，真分叉即停批上报

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（park 与 intent 与 append 写位）
- 承载证据：lease CONTRACT 修订四十一至四十三、leaseup-solo 与 rootanchor-solo 结果档

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/leaseupclose-solo.md`
- `sih-engine/doc/governance/GOV-002-mainline-lock-v1.md`
- `sih-engine/sih/state/plan/leaseup-line-v1.md`（迁出）
- `sih-engine/sih/event/plan/leaseup-line-v1.md`（迁入）
- `sih-engine/sih/event/plan/leaseupclose-solo-results.md`
- `sih-engine/sih/event/plan/leaseupclose-solo-materials/`
- `sih-engine/sih/state/parking/materials/`
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 leaseupclose-solo 工地
