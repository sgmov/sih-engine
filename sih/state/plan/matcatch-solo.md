# matcatch-solo：三批产物版控补提批

> task-packages 治理任务
> 承接：用户 2026-09-01 出提示词令、主会同日验收裁定即 outslim 与 agentslim 过 cmdface 打回、三批共性欠账即任务包与当日链与批报告件未入版控违 trail 随批入版控常设规则
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-01

## 一、问题陈述 {#problem}

2026-09-01 三批即 outslim 与 cmdface 与 agentslim 收约时均未把任务包与当日链与 scribe/reports 批件提交入版控，engine 侧三包 untracked、链文件 untracked、tools 侧三批报告件与计数件 untracked。承 ordfound-mat-solo 迷你批先例即纯机械补提转正。

## 二、关键设计 {#design}

三件。一即补提范围钉死：engine 侧三任务包加当日链 2026-09-01.ndjson 加 inputlog 2026-09-01 新建件，tools 侧 scribe/reports 下 2026-09-01 三批全部批件加 meter/counts/2026-09-01.ndjson；identity/reports 不入即承历批存量形态。二即补提前三任务包走管线笔在核前即化格核阅检词，域外 exit-2 如实记。三即存量旧账不碰即 batch-materials 与 c006-sb3 与 viewimpl 系与 scrutmerge-tdfix 包与 08-30 inputlog 均非本批范围，封窗裁归用户。

## 三、工作清单 {#work}

- [ ] 三任务包管线三步
- [ ] 双仓补提 stage 与 settle
- [ ] inputlog 2026-09-01 新建落四笔逐字
- [ ] 收约对表验链

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 补提零漏 | 工程治理 | 双仓 status 中 2026-09-01 三批产物零 untracked 残留即三包与链与批件与计数件全入册，identity 除外 |
| **F-2** 管线 | 链上治理 | 三包化格核阅检词零违规或域外如实记，认证入链 |
| **F-3** 存量不动 | 工程治理 | 既有 untracked 存量清单与本批前一致即零误收编 |
| **F-4** 收口 | 链上治理 | 双仓 settle、reconcile 零增、链 valid、尾随本批件入版控 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/event/plan/ordfound-mat-solo-results.md 即迷你补提批先例
- 必读 2：AGENTS.md 即工具层静态审计节管线序

## 六、约束 {#constraints}

1. 只补提不改动即补提件内容零编辑，管线化格若改动即格式归一属例外须如实记
2. 存量旧账零收编（红线）
3. identity/reports 不入册（承存量形态）
4. 上链前必须等绿、findings 亲读、禁管道掩退出码

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

误收编存量即范围扩散，防御即 stage 逐文件点名禁目录通配。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-01 出提示词令
- 链件：随批意图入当日链
- 关联：outslim-solo 与 cmdface-solo 与 agentslim-solo 三包、ordfound-mat-solo 先例

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[补提]: 消解 即大白话直述即补充提交，非登记术语
叩问处置[转正]: 消解 即大白话直述即 untracked 转入册，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/outslim-solo.md
- sih-engine/sih/state/plan/cmdface-solo.md
- sih-engine/sih/state/plan/agentslim-solo.md
- sih-engine/sih/state/plan/matcatch-solo.md
- sih-engine/sih/event/plan/matcatch-solo-results.md
- sih-engine/sih/event/plan/matcatch-solo-materials/
- sih-engine/sih/event/trail/2026-09-01.ndjson
- sih-engine/sih/event/inputlog/2026-09-01.ndjson
- sih-tools/scribe/reports/
- sih-tools/meter/counts/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
