# outslim-solo：围堰工具输出瘦身批

> task-packages 治理任务
> 承接：用户 2026-09-01 令即 B 输出瘦身、主会话 2026-08-31 viewreg-solo 批实测即 nomenclator 每查打印 domain 头约四十行六次即约二百四十行无关内容灌入会话上下文
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-01

## 一、问题陈述 {#problem}

围堰工具的 stdout 把完整 JSON 报告含 domain 头与包元数据全量打印，在环消费即 LLM 会话上下文被无关重复内容稀释。病不在报告本身即报告该全落文件，病在缺省全量上屏。承 convergence 层容量感知结论即治理须主动缓解上下文稀释非等压缩救场。

## 二、关键设计 {#design}

三件。一即统一紧凑旗标 `--quiet` 加六工具全子命令：nomenclator 加 formatter 加 lease 加 elicit 加 identity 加 meter。语义即 stdout 只打一行摘要 JSON 含状态与计数与关键标识，完整报告照旧落 --out 文件或保持可重定向，无 --out 的工具完整形仍可经缺省形取得。二即缺省形态逐字节不变：回归测试钉死即同输入下缺省输出与改前逐字节一致，防 golden 与双跑对表被扰动。三即各工具升 minor 版与 CONTRACT 修订记录。

## 三、工作清单 {#work}

- [ ] 六工具 `--quiet` 旗标与一行摘要形
- [ ] 缺省形态逐字节回归测试
- [ ] CONTRACT 修订与升版
- [ ] 管线认证与双仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 紧凑形 | 工程治理 | 六工具 `--quiet` 下 stdout 行数不超三行且状态与计数与关键 id 在场，测试覆盖每工具至少两子命令 |
| **F-2** 缺省不变 | 工程治理 | 同输入缺省输出与改前逐字节一致，六工具回归测试全绿 |
| **F-3** 收口 | 链上治理 | 六 CONTRACT 修订与升版、管线认证入链、双仓收约、unrouted 零增、链 valid |

## 五、必读文件 {#read}

- 必读 1：sih-tools/nomenclator/src/nomenclator/cli.py 即 domain 头打印位
- 必读 2：sih-tools/lease/src/lease/cli.py 即 gauge 块与 allow 列表打印位
- 必读 3：sih-engine/sih/event/plan/viewreg-solo-results.md 即实测病样参照

## 六、约束 {#constraints}

1. scrutinator 零改动（红线）即融回切换窗口输出形态冻结，引擎件字节一致判据在身
2. 引擎五件即 scribe 与 viewer 与 retriever 与 ask3repeater 与 scrutinator 零改动（红线）
3. 缺省输出任何字节漂移即判负返工
4. 上链前必须等绿、findings 亲读、禁管道掩退出码

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过

## 八、风险点 {#risks}

quiet 形摘要缺关键信息即在环判断失据，防御即摘要必含状态与退出相关计数与 id 即事件哈希与会话号类。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-01 B 令
- 链件：随批意图入当日链
- 关联：cmdface-solo 后继批、viewreg-solo 实测参照

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[紧凑旗标]: 消解 即大白话直述工作名，非登记术语
叩问处置[在环消费]: 消解 即大白话直述即会话内即时消费，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-tools/nomenclator/{src,tests,pyproject.toml,CONTRACT.md}
- sih-tools/formatter/{src,tests,pyproject.toml,CONTRACT.md}
- sih-tools/lease/{src,tests,pyproject.toml,CONTRACT.md}
- sih-tools/elicit/{src,tests,pyproject.toml,CONTRACT.md}
- sih-tools/identity/{src,tests,pyproject.toml,CONTRACT.md}
- sih-tools/meter/{src,tests,pyproject.toml,CONTRACT.md}
- sih-tools/{nomenclator,formatter,lease,elicit,identity,meter}/CALL-LOG.md
- sih-engine/sih/state/plan/outslim-solo.md
- sih-engine/sih/event/plan/outslim-solo-results.md
- sih-engine/sih/event/plan/outslim-solo-materials/
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
