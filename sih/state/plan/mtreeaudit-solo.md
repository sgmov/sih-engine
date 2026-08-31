# mtreeaudit-solo：主树直写七笔补证追认批

> task-packages 治理任务
> 承接：用户 2026-08-31 主树直写修正令即对 scrutmerge-sdd-solo 与 viewimpl-solo 两批七笔主树直写提交做补证与追认
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-31

## 一、问题陈述 {#problem}

scrutmerge-sdd-solo 四笔与 viewimpl-solo 三笔共七笔提交直接落于 sih-engine 主树，未经租约副本，message 不带 session 号，对表挂 unrouted 类。2026-08-31 封窗批即 lease 1.9.0 已将对表界线前移至 22550a6 封入七笔，封窗是对表起算窗前移不是身份追认。本批承修正令做补证与追认即历史不可改前提下补记七笔出处与审计结论上链。

## 二、关键设计 {#design}

五件。一机械审计七笔逐笔内容清单与任务包状态与链上事件笔数。二内容完好性核查即引擎测试与核阅化格检词四工具对 SPEC-013 与 GOV-003 双文档，内容不因流程违规判废。三补证批走正门即租约双仓立约收约、七笔追认证认逐笔 scribe append 上链、meter 包裹、提交指副本、收约归并。四对表前后分类对比以实际为准即七笔 message 无 session 行，分类按 commitcore 规则机械产出。五根因记录即裸 git commit 绕过 lease commit 拒主检出直提守卫，候选机械位两件入结果档后续节，裁定归用户。

## 三、工作清单 {#work}

- [ ] 七笔 git show 审计与台账对表与链上事件清点
- [ ] 审计表落 sih/event/plan/mtreeaudit-results.md
- [ ] 内容完好性核查即 cargo test 与四工具双文档
- [ ] 温故三 recall 与意图双腿验收与正身
- [ ] lease open 双仓
- [ ] 副本内落任务包与结果档与报告件
- [ ] 七笔追认证认 scribe append 上链并 meter 包裹
- [ ] 管线认证即任务包与结果档核阅化格检词
- [ ] 双仓 settle 指副本
- [ ] close 归并
- [ ] 根因与候选入结果档后续节

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 审计在档 | 工程治理 | 审计表七笔逐笔含内容清单与任务包状态与链上笔数，落 sih/event/plan/mtreeaudit-results.md 并入仓 |
| **F-2** 完好判定 | 工程治理 | cargo test 全绿、核阅化格检词对 SPEC-013 与 GOV-003 退出码零、viewer 冒烟在位，任一件非绿即内容缺陷如实记 |
| **F-3** 补证在链 | 链上治理 | 七笔追认证认事件逐笔在 08-31 链，content_hashes 载对应 SHA，事件哈希清单入结果档 |
| **F-4** 正门完整 | 工程治理 | 租约双仓立约与收约台账行在、settle 提交带 session 号与 cert 命中、提交落副本分支经归并、meter 包裹七件上链调用 |
| **F-5** 根因在档 | 工程治理 | 结果档后续节含裸提根因与候选机械位两件，裁定归用户字样在档 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/CONTRACT.md 修订十七即封窗裁定与 1.8.2 修订十七即 commit 拒主检出直提
- 必读 2：sih-engine/sih/event/plan/scrutmerge-sdd-solo-results.md 偏离表即病灶自述位
- 必读 3：sih-engine/sih/event/plan/viewimpl-solo-results.md 即同批偏离披露位

## 六、约束 {#constraints}

1. 历史零改写即禁 rebase 与 revert 重做与任何重写
2. 不入 SEAL_EXEMPTS 追认表
3. 追认不洗白即对表分类以实际为准，失败如实留
4. 只写请求净路径，不扫他批残件
5. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过

## 八、风险点 {#risks}

主树与副本两侧链文件与计数件内容须逐字节一致方免归并冲突，承 pkgclose 计数件并入先例。对表分类机制决定七笔无 session 行即 unrouted，追认证认上链不改该分类，如实留档。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 主树直写修正令
- 关联：sealwin2-solo 封窗批结果档、两批偏离披露结果档、GOV-004 历史不可改起点

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/mtreeaudit-solo.md
- sih-engine/sih/event/plan/mtreeaudit-results.md
- sih-engine/sih/event/trail/2026-08-31.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[补证]: 消解 即本批工作名 mtreeaudit 随任务包落档、先例词承封窗批补证追认语义即封窗非追认本批为追认
叩问处置[追认]: 消解 即 1.9.0 修订十七已立封窗非身份追认语义、本批补账即该语义的落施位、零新词
