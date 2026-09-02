# debtswp-solo：三委外批余债清理收尾批

> task-packages 治理任务
> 承接：主会话 2026-09-03 三委外批验收报告的余债登记，用户令直接收尾
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-09-03

## 一、问题陈述 {#problem}

三委外批验收坐实余债四件。其一 entryunique-solo 段2 即工地提交 cd2f28f 因链分叉未归并，其 tdd_tests 断言同步缺席致主树 tdd_tests.rs:135 红测。其二链分叉在案即工地分支自 49 笔后自行追加四件认证而主链同期走他批，两链各自 valid。其三七份批件为主树未跟踪件即 entryunique 与 a2 与 ledgrev 三批的结果档与检索件。其四 entryunique 悬会话 3c7c814e 未撤销。

## 二、关键设计 {#design}

1. 认证重挂：分叉分支的四件认证即 results-fmt 与 scr 与 nom 与 tests，原件已在工具仓主线随 a00da6bd 入版控，按原报告原退出码重挂主链即新事件新哈希同语义，分叉分支作废不归并。
2. 红测修复：从 cd2f28f 对象取 tdd_tests.rs 同步入工地，cargo test 恢复全绿（先在三金向量漂移败除外如实记）。
3. 批件归位：七份未跟踪件随本批入版控。
4. 悬会话强拆：lease close --force --reason 承载即分叉作废与文件已抢救的处置记录。

## 三、工作清单 {#work}

- [ ] 四件认证重挂主链（meter 包裹，原退出码 0 加 2 加 0 加 0）
- [ ] tdd_tests.rs 同步与 cargo test 全绿读数
- [ ] 七份未跟踪批件入工地随批提交
- [ ] entryunique 悬会话 force 收口留 reason
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 认证重挂** | 工程 | 主链新增四笔 certification_completed，报告哈希与工具仓原件一致，链 verify valid |
| **F-2 红测归绿** | 工程 | tdd_tests.rs:135 断言过，cargo test 除先在三金向量漂移外全绿 |
| **F-3 批件归位** | 治理 | 七份未跟踪件随批入版控，主树 plan 目录本批相关零残留 |
| **F-4 悬会话收口** | 治理 | 3c7c814e 台账 revoked 即 reason 在案，工地与分支清除 |
| **F-5 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/BATCH-FACE.md 调用面与坑位
- 必读 2：sih-engine/sih/event/plan/entryunique-solo-results.md 即段2 结果档
- 必读 3：sih-philosophy/emanation/proodos/07-on-assay.md 与 06-on-canon.md 与 08-on-settle.md 引文原文只读切片

## 六、约束 {#constraints}

1. 分叉分支零归并零重写即作废处置，链只追加不回溯
2. 他会话未跟踪件与本批无关者零收编
3. 上链遇锁即等待，词债不过夜，findings 亲读

## 七、请求写入 {#requested-writes}

- sih-engine/src/event_stream/tdd_tests.rs
- sih-engine/sih/event/trail/2026-09-03.ndjson
- sih-engine/sih/event/plan/entryunique-solo-results.md
- sih-engine/sih/event/plan/entryunique-solo-materials/recall-results.json
- sih-engine/sih/event/plan/mathpipe-a2-solo-results.md
- sih-engine/sih/event/plan/mathpipe-a2-solo-materials/recall-results.json
- sih-engine/sih/event/plan/ledgrev-solo-results.md
- sih-engine/sih/event/plan/ledgrev-solo-materials/recall-ledgrev.json
- sih-engine/sih/event/plan/ledgrev-solo-materials/recall-results.json
- sih-engine/sih/state/plan/debtswp-solo.md
- sih-engine/sih/event/plan/debtswp-solo-results.md
- sih-engine/sih/event/plan/debtswp-solo-materials/
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 链 verify valid，reconcile 双仓零新增
- [ ] 悬会话收口读数在档
