# recallfix-solo 结果档

> 批：recallfix-solo 检索器引用抽取存量红清偿批（f2_refs_machine_verifiable 存量红，md 摘录取原始行区间窗口词面）
> 会话：47832042590d8e6d（租约自生成）｜会话标识 sess-zcode-260909-recallfix（ask3 双标识空间，各认各的）
> 日期：2026-09-09 ｜ 队形：单线形 solo，零子代理 ｜ 双仓工地 tools@integral-stage-build 与 engine@main
> 承接：用户 0.9.0 发行裁定带债放行配套第五项（清单已知边界），红源 basefix-solo 结果档存量红申报在档；任务包 sih/state/plan/recallfix-solo.md
> 意图哈希：5e4a5502（intent 事件，meter 包裹；ask3 双门过：核阅 exit 0 零违规、重放 status ok 三锚，digest passed covered 4）

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| 考古先行（复现红与召回行实值与定位抽取位） | 完成 | materials/f2-first-run-red.log 与 materials/archaeology-2026-09-09.json 与 materials/recall-prefix-run-2026-09-09.ndjson |
| 机械抽取规则修复（落点由证据定） | 完成 | src/retriever/mod.rs md_raw_window 辅助位与两轴 md 分支，2 单测增 |
| cargo test 全目标零败 | 完成 | materials/cargo-test-full-2026-09-09.log，退出码 0 |
| 机械链全序 | 完成 | 本档第五节 |
| 完工回报形 | 完成 | 本档第五与第六节 |

## 二、考古判词与规则落点

1. 任务包假设机制被证据否证：任务包预期抽取位在「retriever 或 locator_bridge 的 md 引用扫描」且摘录取自 basefix 第 63 行申报行自身词面。实值考古：retriever 与 locator_bridge 均无引用形字串扫描位，md 行 reference 全部由 locator 索引条目 path 加 line_start 加 line_end 构造；basefix 第 63 行自身条目 reference 为 basefix-solo-results.md@63-63 形且 f2 核验通过，非红源。
2. 红行实值：reference 为 sih-engine/sih/event/plan/pk050sw-solo-results.md@3-9，axis topic，matched recall，excerpt 259 字符即 256 加省略号，词面是 pk050sw-solo-results.md 自身第 3 至 9 行 blockquote 头部的 markdown-it 语义视图词面（去行首标记，550 字符截 256）。
3. 红因机制：locator markdown 载体把多行 blockquote 切为单条目，条目 text 为语义视图词面；f2 重开原档按原始行含行首标记 join 后做子串比对，excerpt 在第 57 字符首个换行处失配（excerpt 形「回退位）换行会话」，window 形「回退位）换行大于号空格会话」），子串必假。
4. 行漂移诊断不成立：pk050sw-solo-results.md 自创建提交 9cf62f8 起第 3 至 9 行即 blockquote 形，头部区段零行漂移；批前全量 md 行 153 条中 f2 失配恰 2 条（pk050sw@3-9 与 scriwire2@3-10，俱 blockquote）；全索引带行区间 md 条目 20077 条中非逐字承载 335 条（blockquote 与多行列表语义视图），俱是结构性词面失配非个别文件漂移。
5. 规则落点（由证据定）：sih-engine/src/retriever/mod.rs。md 活引用出处可机械回验由构造保证，md 行摘录改取原始行区间窗口词面（逐字含行首标记），窗口不可开档或行区间越界即不录活引用；locator 底座切分语义零触碰（sih-tools 不在本批写入面）。

## 三、修复形与抽取输出 diff 概要

- 修复面：src/retriever/mod.rs 增 md_raw_window 辅助位（按路径缓存整档行，越界或缺档回 None）与主题轴文轴两处 md 分支改窗口词面；增 md_window_tests 两单测（窗口逐字含行首标记与缓存同读数、越界逆区间缺档回 None）；tests/mem_recall_f_suite.rs 零改动。
- diff 概要（同参对照批前快照 materials/recall-prefix-run-2026-09-09.ndjson）：行集零损即 1945 行前后恒等；82 行 diff 全落 excerpt 位；两存量红行转绿即 f2 失配零行；matched 词可见性位移零；事件载体与 json 载体行零触碰即哈希位零牵动。
- 金向量牵动申报：retriever 零冻结金向量（引擎金向量面为 attractor 与 event-stream，不涉检索抽取输出），零重冻项。

## 四、测试对表（批前批后零回归）

| 套件 | 批前（主树 HEAD 39e6b1d） | 批后（工地） | 判定 |
|---|---|---|---|
| mem_recall_f_suite | 8 passed 1 failed（f2 存量红，首跑红证在档，复跑退出码 101） | 9 passed 0 failed（104.71 秒） | 存量红清偿零回归 |
| retriever 单测 | 11 passed | 13 passed（增 md_window_tests 两测） | 零回归 |
| cargo test 全目标 | 含 f2 一红 | 239 passed 0 failed，退出码 0（明细 materials/cargo-test-full-2026-09-09.log） | 零回归 |

新增两单测与修复同批落地未单独先红；首跑红证即 f2 存量红批前实跑，归档 materials 零清洗。

## 五、机械链实录

- ask3 双门：核阅 exit 0 零违规（记录件 content bbb03885），重放 status ok 三锚，digest passed covered 4。
- 叩问：活引用与行区间与摘录与词面四词俱未注册轻信号，处置四条入记录，digest 核销。
- 正身：anomalies 空，identity 0.5.0。
- 租约：open 会话 47832042590d8e6d 双仓（tools base integral-stage-build，engine base main），allow 十径逐路径锁 rc 全 0。
- 书简意图：5e4a5502 在链（meter 包裹，闸三 session 加 sessions 全带）。
- 管线三步：本档经化格（exit 0 零改动）与核阅（exit 2 域外如实记）与检词（exit 0 findings 0）。
- 书单对表：recall exit 0 加 checkcite verdict pass（allowed 44，cited 0，missing 0，本批零数学引用形）。
- 认证清单（逐笔 meter 包裹引擎 scribe append，闸三 session 加 sessions 全带）：

| 件 | 事件哈希前八位 | 退出码 |
|---|---|---|
| intent（ask3 记录加验证件） | 5e4a5502 | 0 |
| archaeology 考古记录 | 2a9a1153 | 0 |
| f2 红证报告（首跑加复跑退出码 101） | fb7b1c57 | 101 |
| 管线三步与 checkcite 读数 | caa9fd83 | 0 |
| cargo test 全目标读数（239 passed 0 failed） | 39b2228e | 0 |

- 认证四笔报告 JSON 外原始证据全数在 materials（首跑红证 log 与复跑 log 与批前批后召回快照与修复 diff 与全测 log）。
- 双仓 settle 与链 verify 与 reconcile：见收口附记。

## 六、判变申报

1. 抽取行为变更只向前生效：md 行 excerpt 词面源从 locator 语义视图词面改原始行区间窗口词面，行首标记入词面；行集与 reference 与排序零变。历史链笔与已收批读数与索引产物零改写零重算，批前历史判定位以工具旧版实查为准。
2. 断言零弱化：mem_recall_f_suite 九测与断言逐字零改动，f2 因抽取规则语义收紧自然转绿；新增仅 retriever 单测两笔。

## 七、越线与误差申报

1. 任务包第二节判则预期「确认摘录取自申报行自身词面」被实值否证，判词改从证据（第二节），如实申报；修复规则形为任务包「或等价规则」授权面。
2. mem_recall_f_suite 单套件实跑 104.71 秒，超任务包约 99 秒预估，如实转述。
3. identity 工具 0.5.0 与 BATCH-FACE 记载 0.4.0 漂移，如实记。
4. integration_root dead-code 警告批前既有（主树 HEAD 同参编译同出），本批零新增警告。
5. CALL-LOG：本批零 sih-tools 工具改动，scribe 册未记（任务包 allow 清单未列 calllog 面，从严不越线）。

## 八、收口附记（回填，2026-09-09 收约补笔）

- 双仓 settle 提交号：engine 3b4fd31 与 tools e0bdd55f（cert 39b2228e 即 cargo test 认证笔；三检 session_active 与 staged_in_scope 与 cert_on_chain 全绿）。
- 归并提交号：engine 2e8d032 与 tools 25c0641a（close 机械归并，双工地拆除、分支 msh/recallfix-solo 双删、会话 47832042590d8e6d 吊销）。
- closeguard 收约对表件：tools a39460f（主树批件六件 pre-close 提交）。
- 链 verify：valid 142 笔（尾笔 39b2228e 即本批全测认证）。
- reconcile：双仓 unrouted 0 加 cert_missing 0（比批前零新增）；unbypassed tools 125 加 engine 89 如实转述（含本批 closeguard 对表件 a39460f 与主窗 release 提交，属既有告警类非本批绕行）。
- 主树复验：主树重编后 mem_recall_f_suite 9 passed 0 failed 退出码 0（101.88 秒）。
- 越线申报两笔：其一 Cargo.lock 工地内 staged_out_of_scope（cargo build 触碰 0.1.0 改 0.9.0，主窗发布版本面）还原因零入提交；其二 close 无主闸拦主树 Cargo.lock（主窗在飞件非本批写入）走 --bypass-orphan 显式绕行落 bypass 台账留痕，件零触碰候主窗发布流结算。
- 直改链笔：本节回填经 scribe direct agent 笔落 direct_edit_completed 声明，正身件挂 identity 0.5.0 报告；补笔后化格与检词复跑俱绿。
