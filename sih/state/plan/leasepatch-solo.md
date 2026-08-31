# leasepatch-solo：lease 双缺陷修补批

> task-packages 治理任务
> 承接：用户 2026-08-31 修补令、wenguobs-solo 批踩坑实录即两真缺陷在案
> 队形：单线形 solo——执行代理亲写零子代理、主会验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

wenguobs-solo 批实录两真缺陷：其一 lockcore.py active_sessions 同 session_id 重开场景重复 pop 即重开同号致 session_not_active 误判，该批被迫换五次正身报告换会话号绕开；其二 commitcore.py 第 194 行 staged 清单取自 git diff --cached --name-only，git 对非 ASCII 件名默认八进制转义加引号包裹即中文取样件名触发 staged_out_of_scope 误拦，该批被迫取样件全改 ASCII 名绕开。两处皆是流程税非功能病。

## 二、关键设计 {#design}

三件。一重复 pop 修复即 active_sessions 配对逻辑对同会话号重复 issued 或 revoked 事件幂等，重开同号不再误判，先红即复现用例构造同号双事件跑红后修绿。二件名解析修复即 staged 清单改走 git -c core.quotepath=off diff --cached --name-only 或 -z 空分隔解析，中文件名与 ASCII 同径，先红即临时仓中文名 staged 断言旧法拦新法过。三lease 升 1.9.1 契约修订十八留痕即两缺陷现象、根因、修法、复现用例名。自举声明即本批跑批仍用主树带病件，两绕开法在案即新会话号与 ASCII 件名，修好件随归并回主树自下批起生效。

## 三、工作清单 {#work}

- [x] 缺陷一复现用例先红后修绿
- [x] 缺陷二复现用例先红后修绿
- [x] 版本升 1.9.1 与契约修订与管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 重复 pop | 工程治理 | 同 session_id 重开场景不再误判 session_not_active、既有全部测试零回归 |
| **F-2** 件名解析 | 工程治理 | 临时仓中文件名 staged 不再误拦 staged_out_of_scope、ASCII 件名行为零变 |
| **F-3** 收口 | 链上治理 | 版本 1.9.1 与契约修订十八在档、先红后绿迹存档、管线认证入链、双仓提交 routed、对表净增零 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/src/lease/lockcore.py 即 active_sessions 实形
- 必读 2：sih-tools/lease/src/lease/commitcore.py 第 185 至 210 行即 staged 清单取法与范围验
- 必读 3：wenguobs-solo 结果档偏离节即两坑实录

## 六、约束 {#constraints}

1. 两修皆向后兼容即不改台账 schema 不改既有事件语义
2. 既有测试全绿零回归
3. 上链前必须等绿、findings 亲读、禁管道掩退出码
4. 范围闸若拦即零提交收约改包重开；一切待提交件先进工地从工地提交，禁主树直写与收约后手工归并
5. 与并行批撞锁即报不绕行

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过即主会复验

## 八、风险点 {#risks}

active_sessions 修复须与 pk031gap 批的事件序配对语义兼容即先读该批结果档；quotepath 修复取 -z 空分隔时注意尾空元素剔除。

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 修补令与 wenguobs 偏离实录
- 链件：sih/event/trail/<当日>.ndjson
- 关联：lease CONTRACT 修订十六十七、pk031gap 批

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/lockcore.py
- sih-tools/lease/src/lease/commitcore.py
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/pyproject.toml
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/tests/
- sih-engine/sih/state/plan/leasepatch-solo.md
- sih-engine/sih/event/plan/leasepatch-solo-results.md
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[重复 pop]: 消解 即工作名直述即台账事件配对缺陷、不做登记
叩问处置[件名解析]: 消解 即工作名直述即 git 引号路径解析缺陷、不做登记
