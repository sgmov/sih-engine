# scrutmerge-tdfix-solo：核阅 TDD 整改批

> task-packages 治理任务
> 承接：用户 2026-08-31 整改令、主会 TDD 验收报告即三硬伤两小瑕在案
> 队形：单线形 solo——执行代理亲写零子代理、主会验收位
> 日期：2026-08-31

## 一、问题陈述 {#problem}

scrutmerge-tdd-solo 实质大半过硬即主会亲验逐字节一致成立，但三硬伤打回：一即模块名 src/scrutiny/ 撞检词死词即 Scrutiny 死因核阅位，规格明文选 src/scrutinator/ 而实装未随；二即全模块零自动化测试即金向量六件躺成 fixtures 无守卫，SPEC 测试计划 T1 至 T6 不可机械执行；三即 CLI 形漂移即工具件位置参数目标而引擎件要 --target 旗标，规格自身写漂、切换批一开全调用点断。另有小瑕二即结果档路径写作 src/核阅/ 与实况不符、主树待清段描述散件实际不存在。

## 二、关键设计 {#design}

四件。一模块名实归位即 src/scrutiny/ 整体迁 src/scrutinator/ 即 git mv 保历史、lib.rs 导出行改、内嵌包路径与二进制引用随改、全树零 scrutiny 残留即死词不再作新标识符。二测试守卫落地即金向量六件接入 cargo 测试即引擎件同包同目标输出对金件逐字节断言、退出码五场景用例即合规零空载零单违规一域外二缺包二、多包归因用例即双包同载发现逐条携包名、cargo test 全绿与计数入结果档；红证即位置参数用例改前红改后绿。三 CLI 对齐即引擎件增位置参数目标形态与工具件同形、--target 旗标保留为别名、SPEC-013 修订记录声明双形即位置参数为正典工具兼容形加旗标为引擎别名加切换映射一句。四结果档两处失实更正经管线复绿即认证后改文使认证作废须重走管线重认证。

## 三、工作清单 {#work}

- [ ] 模块 git mv 与引用随改与零残留验证
- [ ] 金向量测试接入与退出码与多包用例，先红后绿
- [ ] 位置参数形态与 SPEC-013 双形修订与双跑复验
- [ ] 结果档更正与管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 名实归位 | 工程治理 | src/scrutinator/ 在位、src/scrutiny 零残留、cargo build 与 test 全绿 |
| **F-2** 测试守卫 | 工程治理 | 金向量六件逐字节断言在册、退出码五场景与多包归因用例在册、cargo test 计数入档、位置参数用例红转绿迹存档 |
| **F-3** CLI 对齐 | 工程治理 | 位置参数形态与工具件同包同目标同路径串双跑逐字节一致、--target 别名并存同输出、SPEC-013 修订声明双形与切换映射 |
| **F-4** 收口 | 链上治理 | 结果档两处失实更正并重走管线重认证、双仓提交 routed、链 verify valid、对表净增零、工具件零改动保持 |

## 五、必读文件 {#read}

- 必读 1：主会验收报告结论即本包第一节
- 必读 2：doc/spec/SPEC-013-scrutiny-mergeback-gap.md 即家位与命令行节及修订记录
- 必读 3：src/bin/scrutinator.rs 与 src/scrutiny/mod.rs 即现 CLI 与装载实形

## 六、约束 {#constraints}

1. 引擎件输出语义零变即任何形下输出与工具件逐字节口径不变
2. 工具件 sih-tools/scrutinator 零改动保持
3. 规格文件名不改即仅内容修订，文件名含 scrutiny 属历史 slug 由切换批或用户另裁
4. 上链前必须等绿、findings 亲读、禁管道掩退出码
5. 范围闸若拦即零提交收约改包重开；一切待提交件先进工地从工地提交，禁主树直写与收约后手工归并
6. 与并行批撞锁即报不绕行

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过即主会复验

## 八、风险点 {#risks}

git mv 后内嵌 include_str 路径与测试引用漏改即编译红即拦；金向量逐字节断言对目标路径敏感即测试用金件内原路径形或以内容哈希断言，取其一并在测试注明。

## 九、队形声明 {#formation}

单线形即执行代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-31 整改令与主会验收报告
- 链件：sih/event/trail/<当日>.ndjson
- 关联：scrutmerge-sdd 与 scrutmerge-tdd 两前批、检词死档 Scrutiny 条、切换批待放行

## 十一、请求写入 {#requested-writes}

- sih-engine/src/scrutiny/
- sih-engine/src/scrutinator/

更名前家位即 sih-engine/src/scrutiny/ 为 git mv 保历史之删除侧，与更名后家位 sih-engine/src/scrutinator/ 成对入面，删除侧非独立写入意图即同一件 git mv 之两面。
- sih-engine/src/bin/scrutinator.rs
- sih-engine/src/lib.rs
- sih-engine/Cargo.toml
- sih-engine/Cargo.lock
- sih-engine/doc/spec/SPEC-013-scrutiny-mergeback-gap.md
- sih-engine/sih/event/plan/scrutmerge-tdd-solo-results.md
- sih-engine/sih/state/plan/scrutmerge-tdfix-solo.md
- sih-engine/sih/event/plan/scrutmerge-tdfix-solo-results.md
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[名实归位]: 消解 即工作名直述即模块名对齐规格与死档、不做登记
叩问处置[测试守卫]: 消解 即工作名直述即金向量入测试面、不做登记
叩问处置[融回]: 消解 即 recall 主题真零命中如实记、融回先例档以英文 slug 索引即 mergeback-eval-t6d 与 mergeback-sdd-t6d 与 mergeback-tdd-t6d 三前批在档、工作名直述、不做登记
叩问处置[金向量]: 消解 即先例已现于 elicitimpl-solo 结果档 golden-check 双跑逐字节一致行与 TDD 批结果档、工作名直述、不做登记
叩问处置[退出码]: 消解 即工作名直述即核阅引擎三值 0 合规 1 违规 2 工具自身异常、不做登记
叩问处置[位置参数]: 消解 即工作名直述即 CLI 正典工具兼容形见 SPEC-013 修订记录追加条、不做登记
叩问处置[空载]: 消解 即工作名直述即退出码五场景之一即零规则包装载运行、不做登记
叩问处置[单违规]: 消解 即工作名直述即退出码五场景之一即含违规单件运行、不做登记
叩问处置[域外]: 消解 即工作名直述即退出码五场景之一即目标不在任何已加载包治理域内、不做登记
叩问处置[缺包]: 消解 即工作名直述即退出码五场景之一即装载不存在的包名、不做登记
叩问处置[合规]: 消解 即工作名直述即退出码五场景之一即零违规运行、不做登记
叩问处置[归因]: 消解 即工作名直述即多包运行时发现逐条携带触发它的那个包名、不做登记
叩问处置[红转绿]: 消解 即工作名直述即位置参数用例改前红改后绿且红迹存档、不做登记
叩问处置[双跑]: 消解 即先例已现于 TDD 批结果档双跑狗粮对照节与 elicitimpl-solo 结果档、工作名直述、不做登记
叩问处置[重认证]: 消解 即工作名直述即结果档改后重走化格核阅检词三件管线并重新书简 append 认证、不做登记
叩问处置[整改]: 消解 即工作名直述即对主会打回的定向修复批、不做登记
