# cmdface-solo：命令面速查固化批

> task-packages 治理任务
> 承接：用户 2026-09-01 令即 C 命令面速查、主会话 2026-08-31 viewreg-solo 批实测即每会话重新摸命令面约十五次调用即 lease repo 要绝对路径、elicit words 要逐个重复传、formatter 不吃 --target、scribe 缺参只报缺参不列全旗标
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-01

## 一、问题陈述 {#problem}

命令面知识无固化载体，每会话以 --help 与源码探测重新发现，纯重复消耗。四实锤在案：lease open 的 repo 参数按调用 cwd 解析致相对路径必错、elicit check 的 words 旗标须逐词重复无示例、formatter 目标为位置参数无说明、引擎 scribe 缺参错误不列用法。

## 二、关键设计 {#design}

三件。一即用法补全：六围堰工具与引擎 scribe 的 usage 与缺参错误补至可复制即粘贴即含必填旗标与形态示例，机器可验即测试断言 help 输出含关键旗标词。二即 lease repo 根相对解析修即升 1.9.x：repo 非绝对且工作区根下存在即按根解析，绝对路径照旧，行为修带测试两形覆盖。三即正典调用面档落 sih-tools/BATCH-FACE.md：批机械链全序逐命令 verbatim 即三问双门、书简意图、任务包、叩问、正身、租约开锁、工地施工、管线三步、认证、双仓 settle、放锁收约对表，含坑位注记即引擎 scribe 才是写位、repo 绝对或根相对、elicit 重复旗标、formatter 位置参数、撞锁即报不绕行、收约碰撞的备份让位归并对表法、CALL-LOG 留痕位、AGENTS 特殊 --allow。以 viewreg-solo 为新鲜参照取命令原样。

## 三、工作清单 {#work}

- [ ] 用法补全与机器断言测试
- [ ] lease 根相对解析修与测试
- [ ] BATCH-FACE.md 落盘
- [ ] 管线认证与双仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 用法完备 | 工程治理 | 各工具 help 或缺参输出含全部必填旗标与至少一条形态示例，断言测试全绿 |
| **F-2** 根相对修 | 工程治理 | lease open 以根相对 repo 路径开成即 worktree 落根下 worktrees 位，绝对路径形回归不破，两测试在册 |
| **F-3** 调用面档 | 工程治理 | BATCH-FACE.md 含全链序逐命令 verbatim 与坑位注记，管线过即化格核阅检词零违规 |
| **F-4** 收口 | 链上治理 | 认证入链、双仓收约、unrouted 零增、链 valid |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/event/plan/viewreg-solo-results.md 即新鲜全链参照
- 必读 2：sih-tools/lease/src/lease/core.py 即 repo 解析位 open_session
- 必读 3：sih-tools/lease/CONTRACT.md 即修订体例

## 六、约束 {#constraints}

1. scrutinator 输出形态零改动即融回切换窗口在身（红线）
2. BATCH-FACE.md 只载命令与坑位不载历史沿革即沿革归各 CONTRACT
3. lease 行为修不带缺省形态变化即显式路径两形皆过
4. 上链前必须等绿、findings 亲读、禁管道掩退出码

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

调用面档与工具实态漂移即速查变误查，防御即断言测试锚关键旗标词加档内每命令注明验证批名，漂移由测试暴露。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-01 C 令
- 链件：随批意图入当日链
- 关联：outslim-solo 前继批、agentslim-solo 后继批即瘦索引指向本档

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[调用面档]: 消解 即大白话直述工作名即文件非术语，登记面为零
叩问处置[坑位注记]: 消解 即大白话直述即易错点注记，非登记术语
叩问处置[命令面]: 消解 即大白话直述命令接口面，非登记术语
叩问处置[可证伪]: 消解 即英文 falsifiable 的白话直述，非登记术语
叩问处置[法四]: 消解 即六要素法第四则即损补的简称，见 06-on-canon，非登记术语
叩问处置[singlish]: 消解 即单线形队形的白话语码名，非登记术语
叩问处置[verbatim]: 消解 即英文照原样直述，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/{src,tests,pyproject.toml,CONTRACT.md}
- sih-tools/{nomenclator,formatter,elicit,identity,meter}/CONTRACT.md
- sih-engine/src/bin/scribe.rs
- sih-engine/src/event_stream/
- sih-engine/Cargo.toml
- sih-engine/Cargo.lock
- sih-engine/tests/
- sih-tools/BATCH-FACE.md
- sih-tools/{nomenclator,formatter,lease,elicit,identity,meter}/CALL-LOG.md
- sih-engine/sih/state/plan/cmdface-solo.md
- sih-engine/sih/event/plan/cmdface-solo-results.md
- sih-engine/sih/event/plan/cmdface-solo-materials/
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
