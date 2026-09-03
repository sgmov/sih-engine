# resarch-solo：批余件入库归档批

> task-packages 治理任务
> 承接：debtswp-solo 验收披露的未提交余件，用户令委外即冲突模式并发启动
> 队形：单线形 solo——确定性归档与主线亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-resarch.json 双档零命中如实记
> 冲突测试：本批为 pk-045 多 agent 冲突测试样本库参与者，结果档须载冲突样本节

## 一、问题陈述 {#problem}

debtswp-solo 收尾后余未提交件三类。其一引擎 inputlog 2026-09-03 即 seq 6 与 7 两笔活写未入版控（seq 6 即批二 α 裁定原话）。其二工具仓五条 ledgrev 调用册行即让位抢救后为活写态。其三引擎 PARKING-v1 名册 pk-045 行为主树未提交态。余件入库即账实归一。

## 二、关键设计 {#design}

纯归档零行为变更：三件按现状入库，inputlog 逐笔对表即 seq 全序无缺号，调用册行逐条对表即五件全在，名册行与链上 pk-045 事件对表。入库不改任何一字。

## 三、工作清单 {#work}

- [ ] 三件现状对表读数
- [ ] 双仓入库
- [ ] 认证上链收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 逐字归档** | 治理 | 三件入库前后逐字节 identical |
| **F-2 对表零差** | 工程 | inputlog 序列连续、调用册五条在、名册行与链事件一致 |
| **F-3 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/BATCH-FACE.md 调用面与坑位
- 必读 2：sih-engine/sih/event/plan/debtswp-solo-results.md 披露节

## 六、约束 {#constraints}

1. 零改字即只入库不改写，发现的任何内容问题申报不改
2. 认证先落主树活链，链文件 settle 前一次性拷工地（pk-045 教训）
3. 撞锁有限重试如实计数，收约让位非 identical 即停批上报

## 七、请求写入 {#requested-writes}

- sih-engine/sih/event/inputlog/2026-09-03.ndjson
- sih-engine/doc/governance/PARKING-v1.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-engine/sih/state/plan/resarch-solo.md
- sih-engine/sih/event/plan/resarch-solo-results.md
- sih-engine/sih/event/plan/resarch-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 冲突样本节在结果档
- [ ] 认证入链，双仓结算收约，对表读数在档
