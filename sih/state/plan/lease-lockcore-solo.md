# lease-lockcore-solo：租约融回 TDD 腿一开工批——金向量冻结与红态落位

> T6D-XX task-packages 治理任务
> 承接：用户 2026-09-13 令「做租约 tdd 开工」；SPEC-024 v1.1（台账家位 sih/ledger/ 已定谳，TDD 入口齐备）测试计划 T1 至 T4；DEC-013 第二步先红后绿
> 范式：T6-D 范式 —— 本任务包偏离：实施类单线 solo，委外零子代理
> 日期：2026-09-13

## 一、问题陈述 {#problem}

- SPEC-024 测试计划要求 T1 金向量冻结先于 T2 逐字节一致：围堰 lease 对 fixture 域根净目标实测输出冻结入 src/lease/fixtures/golden/。
- 红态即测试先行而入口未建：src/lease/ 零存在，tests/lease_mergeback/ 红测对缺席入口断言编译期红。
- 本批为腿一开工批：T1 净目标基形冻结加红态三件落位；脏目标五形与 T5 专项红测归腿一后继批，范围显式收窄。

## 二、关键设计 {#design}

### 2.1 fixture 域根

workspace 根 work/lease-fixtures/domain1/ 下 git init 雀域（新城正典形即 stem 闸零代强制），围堰 lease 以 --root 与三台账覆写旗标全指 fixture，生产台账零触碰；净目标基形即 open 至 lock 至 unlock 至 close 单会话全链，--at 定值保逐字节稳定。

### 2.2 红测三件

tests/lease_mergeback/ 三件：mod.rs（入口缺席编译红承载）、golden.rs（T2 金向量逐字节断言先红）、exit_codes.rs（T3 退出码全表先红），断言目标 src/lease 与 src/bin/lease.rs。

## 三、工作清单 {#work}

- [ ] fixture 域根 git init
- [ ] 围堰仪式实测 open/lock/unlock/close 出参冻结
- [ ] 金向量落 src/lease/fixtures/golden/
- [ ] 红测三件落 tests/lease_mergeback/
- [ ] cargo test 红态实录
- [ ] 结果档与结算

## 四、可证伪条件 {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 金向量冻结 | 数据治理 | golden/ 下至少 open 与 lock 与 unlock 与 close 四回执 json 且围堰复跑逐字节一致 |
| **F-2** 红态在场 | 代码修复 | cargo test 对 tests/lease_mergeback 编译红或断言红，红因即入口缺席 |
| **F-3** 生产面零污染 | 数据治理 | 生产 ledger 五册行数与批前一致 |

## 五、约束 {#constraints}

1. 生产台账与围堰源码零改动。
2. 本批零绿：绿转归腿一后继批，本批只红。
3. fixture 域根不入引擎 git（work/ 在仓外）。

## 六、请求写入 {#requested-writes}

- sih-engine/src/lease/fixtures/golden/
- sih-engine/tests/lease_mergeback/
- sih-engine/sih/state/plan/lease-lockcore-solo.md
- sih-engine/sih/event/plan/lease-lockcore-solo-results.md
- sih-engine/sih/event/trail/（append）
