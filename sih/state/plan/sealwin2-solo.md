# sealwin2-solo：引擎仓封窗批

> task-packages 治理任务
> 承接：用户 2026-08-31 封窗令即七笔已披露 unrouted 存量封窗处置、三问意图即同日链事件 294848f5
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-31，修订一即实施中发现 epiacc 两笔封点后新残迹在窗外不默杀、留作真新增量信号如实记

## 一、问题陈述 {#problem}

viewimpl-solo 三笔与 scrutmerge-sdd-solo 四笔主树直写 unrouted 存量在对表常红，两批偏离均已各自结果档如实披露且功能分别经主会验收与实质通过，内容有效历史不改写。承 2026-08-27 修订十二封窗先例即对表起算窗前移恢复零为干净信号义。

## 二、关键设计 {#design}

一件。SEAL_BASES 引擎界线自 d2b4a24 前移至 22550a6 即 epiacc 归并点现头，窗内含七笔 unrouted 与十一笔 cert_missing 即链事故残迹存量一并封入，sih-tools 仓界线不动，追认表 SEAL_EXEMPTS 不动即七笔不进表即封窗非身份追认。lease 升 1.9.0 契约修订留痕，终态引擎免参对表退出码零。

## 三、工作清单 {#work}

- [ ] SEAL_BASES 改值与版本升与契约修订
- [ ] lease 测试全绿与引擎对表复跑退出码零
- [ ] 管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 界线前移 | 工程治理 | SEAL_BASES 引擎值改 22550a6、tools 值零动、追认表零动 |
| **F-2** 信号恢复 | 工程治理 | 七笔 unrouted 与十一笔 cert_missing 归零封入即不复现在窗口、对表剩余信号恰为 epiacc 两笔封点后新残迹即零为干净非零为真新增量的信号义恢复、tools 对表与封前同值 |
| **F-3** 留痕 | 链上治理 | 契约修订含裁定语义与七笔清单、任务包与结果档、认证入链、双仓结算收约 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/CONTRACT.md 修订十二即封窗先例原文
- 必读 2：sih-tools/lease/src/lease/commitcore.py 第 29 至 32 行即 SEAL_BASES
- 必读 3：sih-engine/sih/event/plan/scrutmerge-sdd-solo-results.md 偏离表即病灶披露位

## 六、约束 {#constraints}

1. 历史提交零改写
2. 不进 SEAL_EXEMPTS 追认表
3. tools 仓界线与对表现状零动
4. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过

## 八、风险点 {#risks}

封点后新批提交即入新窗正常计数，零为干净信号义自本批恢复。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 封窗令
- 链件：sih/event/trail/2026-08-31.ndjson 即意图 294848f5
- 关联：修订十二先例、viewimpl 与 scrutmerge-sdd 两批结果档

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/commitcore.py
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/pyproject.toml
- sih-tools/lease/CONTRACT.md
- sih-engine/sih/state/plan/sealwin2-solo.md
- sih-engine/sih/event/plan/sealwin2-solo-results.md
- sih-engine/sih/event/trail/2026-08-31.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[封窗]: 消解 即先例词承修订十二即对表起算窗前移、不做新登记
叩问处置[界线]: 消解 即 SEAL_BASES 起算界承 lease 契约既有词、召回面零命中如实记
