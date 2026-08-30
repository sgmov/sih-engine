# crossgraph-solo：跨方核毕专属事件与旧投影清残批

> task-packages 治理任务
> 承接：sess-zcode-260830-crossgraph 三问意图即 2026-08-30 链事件 0e88aeeb、用户同日开令
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30，修订一即 AGENTS tally 行因并行会话 rulesem-solo 持 AGENTS.md 锁延期候补

## 一、问题陈述 {#problem}

两件。一即执契跨方核毕专属事件类型：crosscheck_completed 自事件流核心批即在词表与 requires_details 在册而无写入者，执契终签现借认证位事件承载即名实分裂在案。二即 graph 旧投影清残：sih/state/graph/CASCADE.json 停 0.1.0 旧位，正式投影已按章程落 doc/CASCADE.json 且本日以 0.4.0 重建，两处边册名实分裂。

## 二、关键设计 {#design}

三件。一引擎照读数先例补 crosscheck.rs 即守卫件与事件构建件：守卫冻十五字段即执契核对报告十二字段加从所指材料拈三即 topic_sha256 与 dc_fingerprint 与 n_shots 承 DES-011 载荷六扩展字段，报告本体不动免破 watch 重放，材料缺席即拒；event_class 按处置机械分即裁决通过取仅记录余取可消费承 DES-011 输出契约；doc_id 即 crosscheck-加 gid。二书简增 crosscheck 子命令即读写数先例带锁位守卫，执契 sign 子进程从 append 换 crosscheck 即专属事件落链。三 graph 旧投影删除留痕即删档不迁即正式位已在。

## 三、工作清单 {#work}

- [ ] crosscheck.rs 守卫与构建与单测，先红后绿
- [ ] scribe crosscheck 子命令与用法串
- [ ] 执契 sign 换子命令与测试断言与版本升 1.1.0
- [ ] graph 旧投影删除
- [ ] 契约修订、管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 守卫件 | 工程治理 | 十五字段冻结面即多字段少字段拒、三拈字段形态校、材料缺席拒、同输入同判 |
| **F-2** 专属落链 | 工程治理 | crosscheck 子命令产 crosscheck_completed 事件即链 verify 过、event_class 按处置机械分、doc_id 唯一形 |
| **F-3** 执契切换 | 工程治理 | sign 调 crosscheck 不调 append、拒签路径零动、verify 重放逐字节同 |
| **F-4** 清残 | 工程治理 | graph 旧投影删除、链上留痕、正式位 doc/CASCADE.json 不动 |
| **F-5** 收口 | 链上治理 | 契约修订、认证入链、双仓结算收约、unrouted 零、AGENTS tally 行因并行会话持锁延期候补如实记 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/src/event_stream/reading.rs 即守卫与构建形制
- 必读 2：sih-engine/doc/design/DES-011-adjudication-baseline-check.md 即载荷与分类语义
- 必读 3：sih-tools/tally/src/tally/cli.py 即 sign 现行落据路径

## 六、约束 {#constraints}

1. 报告十二字段冻结面零改即 watch 既有重放零破
2. 词表与 append 与 verify 与 query 零改
3. 不重签既有旧档即边界变更由 watch 如实浮出归人
4. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过

## 八、风险点 {#risks}

材料相对路径解析以报告同目录为准，绝对路径即拒；旧档 m-namefit 缺三拈字段即本批后首次重放浮异常视图，如实归人不静默。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 开令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 0e88aeeb
- 关联：tally CONTRACT 期票句、DES-011、读数先例批、tokencap 批即正式投影 0.4.0

## 十一、请求写入 {#requested-writes}

- sih-engine/src/event_stream/
- sih-engine/src/bin/scribe.rs
- sih-engine/sih/state/graph/CASCADE.json
- sih-engine/sih/state/plan/crossgraph-solo.md
- sih-engine/sih/event/plan/crossgraph-solo-results.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/tally/src/tally/cli.py
- sih-tools/tally/tests/
- sih-tools/tally/CONTRACT.md
- sih-tools/tally/pyproject.toml
- sih-tools/tally/src/tally/__init__.py
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[跨方核毕]: 消解 即召回面按登记主题轴零命中如实记、语义已从 DES-011 原文核验
叩问处置[crosscheck]: 消解 即代码标识符非自然语言词、召回面零命中如实记
叩问处置[旧投影清残]: 消解 即工作名直述即删旧位投影留痕、不做登记
叩问处置[跨方核毕事件]: 消解 即工作名直述即 crosscheck_completed 专属事件、不做登记
