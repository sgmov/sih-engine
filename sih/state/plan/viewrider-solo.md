# viewrider-solo：视图整改补批

> task-packages 治理任务
> 承接：用户 2026-08-31 补批令、主会 viewfix-solo 复验报告即三件尾巴在案
> 队形：单线形 solo——执行代理亲写零子代理、主会验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

viewfix-solo 整改本体经主会验收通过，留三件尾巴：其一工具侧十五件材料（viewfix 批 scribe 报告与调用册行与计数）悬在 sih-tools 主树未提交即认证在链文件未入版控；其二集成测试 tests/cli_multitrail.rs 随工地清理丢失即跨链回归守卫缺位，根因含任务包漏列 tests 顶层目录；其三收口合并 88c6b4a 主题词缺模板前缀归 unrouted，处置归人节点不在本批。

## 二、关键设计 {#design}

两件。一工具侧补提交即主树悬空的 viewfix 批材料经正规租约从工地提交入库，主树悬空清零。二恢复集成测试即重建 tests/cli_multitrail.rs 覆盖三断言簇：多链装载即 enter 在甲链加 exit 在乙链时双链跑出已出泊带裁决而单跑乙链时缺席、单链回归即单链输出与独立预期一致即整改前行为不变、可消费聚合即 alarms 跨链全量。测试先红证即新建文件断言跑于裁剪版装载逻辑不可行故红证以主会复验实录为准引结果档，绿证即 cargo test 全绿。

## 三、工作清单 {#work}

- [ ] 主树悬空十五件清点与工地复制与租约提交
- [ ] tests/cli_multitrail.rs 重建与 cargo test 全绿
- [ ] 管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 工具侧入库 | 工程治理 | 悬空件清零即 git status 无 viewfix 残留、入库提交带 session 与 cert |
| **F-2** 集成测试 | 工程治理 | tests/cli_multitrail.rs 在库、三断言簇齐、cargo test 全绿含既有八十六测 |
| **F-3** 收口 | 链上治理 | 全程租约零直写、双仓 routed、链 verify valid、unrouted 净增零、88c6b4a 零触碰 |

## 五、必读文件 {#read}

- 必读 1：sih/event/plan/viewfix-solo-results.md 即整改实录与主会复验基准
- 必读 2：src/bin/viewer.rs 即已修复的装载层
- 必读 3：sih/event/trail/2026-08-25.ndjson 与 2026-08-30.ndjson 即跨链测试可采的真实事件样本

## 六、约束 {#constraints}

1. viewer.rs 与 src/view/ 零改动即测试恢复不是功能返工
2. 88c6b4a 零触碰即 unrouted 处置归人节点
3. 上链前必须等绿、findings 亲读、禁管道掩退出码
4. 范围闸若拦即零提交收约改包重开
5. 与 scrutmerge-tdd 并行即锁面不相交、撞锁即报不绕行

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过即主会复验

## 八、风险点 {#risks}

集成测试用临时链文件构造即不依赖真实链路径；主树悬空件中或有他批并行新写入，只取 viewfix 名下件不误收。

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 补批令
- 链件：sih/event/trail/<当日>.ndjson
- 关联：viewfix-solo 批与主会复验报告

## 十一、请求写入 {#requested-writes}

- sih-engine/tests/
- sih-engine/sih/state/plan/viewrider-solo.md
- sih-engine/sih/event/plan/viewrider-solo-results.md
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[补批]: 消解 即工作名直述即尾巴清账小批、不做登记
叩问处置[viewrider-solo]: 消解 即本批名 viewrider-solo 视图整改补批
叩问处置[工具侧入库]: 消解 即工作名直述即悬空件经租约从工地提交入库
叩问处置[集成测试]: 消解 即工作名直述即 tests/cli_multitrail.rs 重建三断言簇
叩问处置[cli_multitrail]: 消解 即工作名直述即 cargo 集成测试文件 cli_multitrail
叩问处置[tests目录]: 消解 即工作名直述即 cargo tests 顶层目录
叩问处置[viewrider]: 消解 即本批主题简称 viewrider 视图整改补批
叩问处置[尾巴清账]: 消解 即工作名直述即 viewfix-solo 收口后三件尾巴清账
叩问处置[工地提交铁律]: 消解 即工作名直述即禁主树直写禁裸 commit 的批纪律
叩问处置[模板前缀]: 消解 即工作名直述即 commit subject 缺模板前缀的归 unrouted 处置归人节点
叩问处置[范围闸]: 消解 即工作名直述即 lease 范围四验拦即收约改包重开
叩问处置[补批]: 消解 即工作名直述即尾巴清账小批
叩问处置[集成测试恢复]: 消解 即工作名直述即 F-2 集成测试恢复
