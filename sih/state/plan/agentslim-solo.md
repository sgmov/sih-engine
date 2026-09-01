# agentslim-solo：AGENTS.md 引导层瘦身批

> task-packages 治理任务
> 承接：用户 2026-09-01 令即 AGENTS.md 优化、主会话同日实测即全文件 43343 字节 315 行每会话固定吃约两万 token 量级
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-01

## 一、问题陈述 {#problem}

AGENTS.md 是引导层却长成史册：文件索引 10855 字节占四分之一挂满日期批名版本号，MCP 节 6679 字节大半为 TRAE 死环境正文即自身适用域声明已判 TRAE 专属，工具审计节混管线规则与沿革叙事。实锤烂点在案：parser 双条目即句读解析工具与自研解析工具同路径重复，lease 版本行停 1.8.2 而实况 1.9.1。每会话全量加载即固定成本稀释上下文。

## 二、关键设计 {#design}

四件。一即宪法零动：工程基线与禁止条款节逐字节不变，项目身份与 Agent 身份与哲学仓地位与数学仓地位的核心裁定句不变，允许压缩的是覆盖范围声明的展开叙述即压缩后判定句与检索指针在。二即三节瘦身：会话开始节保留两条命令 verbatim 删来源叙述；工具审计节留管线序与工具名与路径与退出码语义与强制规则清单，沿革叙事即 doclint 血统与切换时间线与历史实证段迁档；MCP 节留适用域声明三行加归档指针。三即文件索引表格化：列即名与路径与一句职能，删日期版本批名注记，双 parser 条目合一，版本号单源各 CONTRACT 即索引不载版本。四即移出内容全量落 sih-engine/doc/AGENTS-RETIRED-2026-09.md 不删史，件头注来源节与迁出日期。目标字节区间 15000 至 23000。

## 三、工作清单 {#work}

- [ ] 宪法节逐字节不变核验
- [ ] 三节瘦身与索引表格化
- [ ] 归档件落盘与字节对账
- [ ] 管线认证与收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 宪法零动 | 工程治理 | 工程基线与禁止条款节与改前逐字节一致，身份四节核心裁定句逐句在场 |
| **F-2** 目标区间 | 工程治理 | 瘦后全文件字节在 15000 至 23000，概览节同步改读数 |
| **F-3** 归档零损 | 工程治理 | 移出内容逐段在归档件可检索即每迁出段首句在档，归档件入引擎仓版控 |
| **F-4** 收口 | 链上治理 | AGENTS 在检词 core 域内 check 零违例、des-001 域外 exit-2 如实记、化格零改、认证入链、engine 仓收约、链 valid |

## 五、必读文件 {#read}

- 必读 1：AGENTS.md 全文即瘦身对象
- 必读 2：sih-tools/nomenclator/packs/core/ 即核查域含 AGENTS.md
- 必读 3：sih-engine/doc/decision/008-node-tree-clearance.md 即立名礼档防误删名分句

## 六、约束 {#constraints}

1. 工程基线五条与禁止条款一字不动（红线）
2. 产出前自检六项与 sih 触发语义保留（红线）
3. AGENTS.md 无仓版控即写经 lease --allow AGENTS.md、归档件与 trail 走 engine 仓收约
4. 瘦索引指向 sih-tools/BATCH-FACE.md 即 cmdface-solo 产物，路径先钉死
5. 上链前必须等绿、findings 亲读、禁管道掩退出码

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

瘦过头即宪法或启动义务丢失，防御即字节下限 15000 加宪法节逐字节对表加命令 verbatim 在场断言。指针烂即瘦后指针失养，防御即指针只留少数正典位即各仓 CONTRACT 与 llm-friendly-build 与 skills。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-01 AGENTS 优化令
- 链件：随批意图入当日链
- 关联：outslim-solo 与 cmdface-solo 前继批、scrutmerge-switch-solo 后继即修订落瘦文本

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[引导层]: 消解 即 AGENTS.md 既有自述词非新造
叩问处置[史册]: 消解 即大白话直述即历史记录载体，非登记术语
叩问处置[烂点]: 消解 即大白话直述即失修实例，非登记术语

## 十一、请求写入 {#requested-writes}

- AGENTS.md
- sih-engine/doc/AGENTS-RETIRED-2026-09.md
- sih-engine/sih/state/plan/agentslim-solo.md
- sih-engine/sih/event/plan/agentslim-solo-results.md
- sih-engine/sih/event/plan/agentslim-solo-materials/
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
