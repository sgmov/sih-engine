# pk033casc-solo：级联 0.3.0 边抽取接解析层批

> task-packages 治理任务
> 承接：sess-zcode-260830-pk033casc 三问意图即 2026-08-30 链事件 39c634a2、用户同日双出泊令与开工令即 pk-033 出泊
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

级联 v1 边抽取只扫 markdown 正文，代码载体即 rs 文件不在边域。句读 v1 成后解析层可用即泊档解锁条件满足，v2 表达式层已交付。

## 二、关键设计 {#design}

四件。一 build 增 rs 扫描即逐件调句读 entries 子进程产条目投影入册新 code 节即件路径与条目 kind 与 name 与行位与内容哈希。二代码到文档边即 rs 注释文本即行注释与块注释由级联自扫五族引用词建边，同 v1 语义即自引排除与多义无主入注记与去重排序。三 check 与 orphans 对 code 边同表即上游三态与孤儿兜底一致。四升 0.3.0 契约修订四即注释侧边界如实声明承句读按设计丢注释记号。

## 三、工作清单 {#work}

- [ ] rs 扫描与条目投影先红后绿
- [ ] 注释侧引用词建边与同表测试
- [ ] 契约修订四与升版
- [ ] pk-033 出泊与管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 条目投影 | 工程治理 | rs 经句读子进程产条目、code 节在册、子进程缺席退出码二报件名 |
| **F-2** 代码边同表 | 工程治理 | 注释引用词建边、自引排除、check 三态与 orphans 对 code 边一致、双跑逐字节一致 |
| **F-3** 收口 | 链上治理 | 升 0.3.0 契约修订、pk-033 exit promoted、认证入链、unrouted 零 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/cascade/src/cascade/core.py 即现行抽取
- 必读 2：sih-tools/parser/CONTRACT.md 即解析层边界

## 六、约束 {#constraints}

1. 句读零改动
2. v1 边语义零动
3. 双跑逐字节一致
4. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过

## 八、风险点 {#risks}

注释剥离粗糙即字符串内 // 假阳性，接受即注释文本按行首与块包裹近似、误边方向拦多不拦漏承语义边界节。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 双出泊令与开工令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 39c634a2
- 关联：pk-033 泊档、pk027expr 前批、句读 CONTRACT

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[条目投影]: 消解 即句读条目的册内镜像直述工作名不作登记
叩问处置[注释侧]: 消解 即级联自扫边界直述不作登记

## 十一、请求写入 {#requested-writes}

- sih-tools/cascade/src/cascade/
- sih-tools/cascade/tests/
- sih-tools/cascade/CONTRACT.md
- sih-tools/cascade/pyproject.toml
- sih-engine/sih/state/plan/pk033casc-solo.md
- sih-engine/sih/event/plan/pk033casc-solo-results.md
- sih-tools/parking/materials/pk-033-exit.json
- sih-tools/PARKING-v1.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/
