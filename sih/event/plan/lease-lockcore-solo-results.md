# lease-lockcore-solo 结果档

> 承接：任务包 sih/state/plan/lease-lockcore-solo.md 与用户 2026-09-13 令「做租约 tdd 开工」；SPEC-024 v1.1 测试计划 T1 至 T4 腿一开工形。
> 日期：2026-09-13　会话：edaff98714715d6a　范式：单线 solo 委外零子代理

## 执行实录 {#execution}

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| fixture 域根 | work/lease-fixtures/domain1 git init 雀域 | 0 | 新城正典形 stem 闸零代强制 |
| 围堰全链 | open 至 lock 至 unlock 至 close 五Receipt --at 定值冻结 | 0/0/0/0 | src/lease/fixtures/golden/ 六件 |
| 双跑实证 | domain2 全链复跑 cmp | 0 归一后 | 跨根差量仅域根路径与 session_id 两域，NORMALIZATION.md 在档 |
| 红测三件 | tests/lease_mergeback_*.rs 平铺落位 | 0 | t2_golden 加 t3_exit_codes 加 t4_five_verifications |
| 红态实录 | cargo test 三目标 | 101 红 | 三红一绿（金向量在档检），红证 work/lease-fixtures/red-state-tdd.log |
| 意图笔 | scribe intent | 0 | 234c0f37 |

## 交付摘要 {#deliverable}

T1 净目标基形金向量六件（open 与 lock 与 unlock 与 close 四Receipt加空收约对照形加归一注记）冻结入 src/lease/fixtures/golden/；T2 与 T3 与 T4 红测三件落位，红态经 cargo test 实录定格。生产台账五册零触碰，围堰源码零改动。

## F 锚定对表 {#f-table}

| 锚定 | 判据 | 结果 |
|---|---|---|
| F-1 金向量冻结 | 四Receipt在档加围堰复跑逐字节一致 | 过（归一后）：双跑差量仅域根路径与开约签发 session_id 两域，零时戳泄漏，NORMALIZATION.md 承载归一语义 |
| F-2 红态在场 | cargo test 红，红因入口缺席 | 过：三测红，红因即 target/debug/lease 未建，红证日志在案 |
| F-3 生产面零污染 | 生产 ledger 行数批前一致 | 过：全程 --root 与三台账覆写指 fixture |

## 偏差申报 {#declarations}

- 偏差一：SPEC-024 同参形条款落差发现即 session_id 归一项缺位，跨跑金向量比对须归一开约签发 session_id 与域根路径两域，修订候选归 SPEC-024 后继修订批，NORMALIZATION.md 先行机械承载。
- 偏差二：红测文件布局由 tests/lease_mergeback/ 子目录改平铺三件，cargo 集成测试目标只认 tests/ 直属文件，锁面换宽至 sih-engine/tests/ 目录对表。
- 偏差三：fixture 首开约时 close 误先于 lock 执行即顺序失误，空收约回执留存为 close-receipt-idle.json 合法对照形，全链重跑金向量以第二次为准。
- 偏差四：本批只红不绿即开工批范围，绿转与脏目标五形与 T5 专项红测归腿一后继批。
- 零偏差显式申报位：除上列四条外本批零其他偏差。

## 后续待令 {#next}

- 腿一绿批：src/lease/ 锁核实装先红后绿，T2 归一 cmp 转绿为入口条件。
- SPEC-024 v1.2 修订候选：同参形条款增 session_id 归一项。

## 结算节 {#settle}

- wip 提交 da8596d，红测与红证经 git 9ddff61 绕行落盘 bypass 在案，认证报告经 scribe append 7815b2a6 落链后以 git d5f5ac5 绕行落盘 bypass 在案。
- 本段结算尾注为 settle staged 承载件，先例同形。
