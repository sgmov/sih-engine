# viewfix-solo：视图心跳跨链腿整改批

> task-packages 治理任务
> 承接：用户 2026-08-31 整改批准令、主会话验收报告即 viewimpl-solo F-2 打回与三笔 unrouted 残迹在案
> 队形：单线形 solo——执行代理亲写零子代理、主会验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

viewimpl-solo 批功能验收 F-2 打回：viewer 命令行对可重复 --trail 只装载最后一条链，跨日泊史全丢即真实多链心跳在泊恒空、已出泊件只剩当日单件。配对纯函数正确，病在 CLI 装载层。另该批结果档两处失实即 F-2 证据句自称三链合并十件全配对经 CLI 复现不出、unrouted 残迹写二笔实测三笔。

## 二、关键设计 {#design}

三件。一 CLI 装载改全量按参序接续即 --trail 每条都读、按参数顺序串联事件流供三子命令消费，承书简与级联多链先例即参序即序。二跨链集成测试先红后绿即红证用真实链复现整改前行为（多链心跳空或单件、在泊件缺席）、绿证即七链 08-25 至 08-31 心跳含 pk-013 与 pk-016 在泊未到期（08-25 入泊未出）与已出泊件配对带 disposition。三结果档更正两处失实句并经管线复绿，unrouted 残迹计数更正为三笔、处置归属人节点不在本批。

## 三、工作清单 {#work}

- [ ] 红证采集即整改前 CLI 多链行为输出存档
- [ ] CLI 全量按参序装载与三子命令复验
- [ ] 跨链集成测试先红后绿
- [ ] 结果档两处更正与管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 多链接续 | 工程治理 | --trail 可重复全量装载按参序、七链 08-25 至 08-31 心跳 pk-013 与 pk-016 在泊在列、已出泊件配对带 disposition、单链行为与整改前逐字节不变 |
| **F-2** 红绿迹 | 工程治理 | 红证即整改前缺陷输出存档在批材料、绿证即同口径复跑通过、cargo test 全绿含新跨链用例 |
| **F-3** 更正 | 工程治理 | 结果档 F-2 证据句改如实、unrouted 计数改三笔、经化格核阅检词全绿 |
| **F-4** 收口 | 链上治理 | 全程租约零直写零裸 commit、双仓提交 routed、链 verify valid、本批 unrouted 净增零、三笔历史提交零触碰 |

## 五、必读文件 {#read}

- 必读 1：src/bin/viewer.rs 即病在装载层一行
- 必读 2：sih/event/plan/viewimpl-solo-results.md 即两处失实句所在
- 必读 3：sih-tools/scrutinator/CONTRACT.md 多 trail 参序先例即 check --trail 可重复按参序接续

## 六、约束 {#constraints}

1. 三笔历史提交零触碰即不改写不补挂，unrouted 处置归人节点
2. 配对纯函数零改动即病只在装载层
3. 上链前必须等绿、findings 亲读、禁管道掩退出码
4. 范围闸若拦即零提交收约改包重开，禁工地裸 commit 与主树直写
5. GOV-003 零改即 bug 修复不升向界版本

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过即主会验收位复跑

## 八、风险点 {#risks}

多链串联后事件时序跨日交错，配对重放按装载序即参序非时间排序，与级联最近认证胜先例同构；同链重复传参属垃圾入即垃圾出不防。

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 整改批准令
- 链件：sih/event/trail/<当日>.ndjson
- 关联：viewimpl-solo 批与主会验收报告、scrutinator 多链先例

## 十一、请求写入 {#requested-writes}

- sih-engine/src/bin/viewer.rs
- sih-engine/src/view/
- sih-engine/src/lib.rs
- sih-engine/sih/event/plan/viewimpl-solo-results.md
- sih-engine/sih/state/plan/viewfix-solo.md
- sih-engine/sih/event/plan/viewfix-solo-results.md
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[viewer 跨链腿]: 消解 即工作名直述即多链装载接续能力、不做登记
