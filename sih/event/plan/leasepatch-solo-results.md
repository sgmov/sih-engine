# leasepatch-solo 结算结果档

> task-packages 治理结果
> 队形：单线形 solo
> 日期：2026-08-31

## 一、完成度表 {#completion}

| 项                         | 状态          |
| ------------------------- | ----------- |
| 缺陷一复现用例先红后绿               | 完成          |
| 缺陷二复现用例先红后绿               | 完成          |
| 版本升 1.9.1 与契约修订与管线认证与结算收约 | 完成（认证与收约定后） |

## 二、施工 {#build}

缺陷一即 lockcore.active\_sessions 实形落后 core 单遍事件序配对，两遍分离置位弹出导致同 session\_id 重开即 issued-revoked-issued 序被旧 revoked 误抹、重开后真实在册会话被判 session\_not\_active，朴素 process 被迫换会话号绕开。修法即 lockcore.active\_sessions 与 core 对齐改单遍事件序配对即 issued 置位、revoked 仅弹在册同号，重复事件幂等。修后文件即 lease/src/lease/lockcore.py。

缺陷二即 commitcore 取 staged 清单走 git diff --cached --name-only 未关 core.quotepath，git 对非 ASCII 件名默认八进制转义加引号包裹即中文取样件名被误判 staged\_out\_of\_scope，朴素 process 被迫取样件全改 ASCII 名绕开。修法即加 -c core.quotepath=off 使中文档与 ASCII 同径输出原文路径。修后文件即 lease/src/lease/commitcore.py。

两修皆向后兼容即台账 schema 与事件语义零改。版本升 1.9.1 即 lease/src/lease/__init__.py 与 lease/pyproject.toml。契约修订十八在档即 lease/CONTRACT.md。

自举声明即本批跑批仍用主树带病件，新会话号与 ASCII 件名两绕开法在案，修好件随归并回主树自下批起生效。

## 三、红绿迹 {#redgreen}

缺陷一红即 test\_lockcore\_requires\_session\_same\_id\_reopen 旧实形跑红，绿即单遍配对后转绿。
缺陷二红即 test\_commit\_accepts\_staged\_chinese\_filename 旧实形误拦 staged\_out\_of\_scope 跑红，绿即 -c core.quotepath=off 后转绿。
测试计数即二新测并入既有，合计四十四测绿零回归。

## 四、F 验证表 {#f}

| F          | 类别   | 判据                                                      | 结果     |
| ---------- | ---- | ------------------------------------------------------- | ------ |
| F-1 重复 pop | 工程治理 | 同 session\_id 重开场景不再误判 session\_not\_active、既有全部测试零回归   | 过      |
| F-2 件名解析   | 工程治理 | 临时仓中文件名 staged 不再误拦 staged\_out\_of\_scope、ASCII 件名行为零变 | 过      |
| F-3 收口     | 链上治理 | 版本 1.9.1 与契约修订十八在档、先红后绿迹存档、管线认证入链、双仓提交 routed、对表净增零     | 收约定后补全 |

## 五、队形验证 {#formation}

单线形即执行代理亲写零子代理。

## 六、偏离 {#deviation}

核阅对 lease/CONTRACT.md 返回退出码 2 即目标在 des-001 声明治理域之外，sih-tools 契约不受 sih-engine des-001 治理域约束系 AGENTS.md 治理边界内预期，不计为文档违规。本批跑批用主树带病件系任务包批准的自举绕开法，非偏离。

## 七、关联 {#related}

* 任务包：sih/state/plan/leasepatch-solo.md

* 温故检索切面：/Users/moc/workspaces/SiHankor/worktrees/sih-engine/leasepatch-solo/sih/event/plan/leasepatch-solo-recall.jsonl

