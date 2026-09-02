# idcore-solo：正身核哈希双哈希修复批

> task-packages 治理任务
> 承接：identity 委外调研验收（pk041-solo-materials 两件在档）与用户 2026-09-03 采纳裁（正身修复同意，四裁定随批准采纳）与命题 m-idcore/m-idcore2 两轮 boundary 通道错配改道记录（依据 sihankor-facet-measure 修订一：反对集中于起草权元问题非机制内容，落确定性核对通道不三跑）
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-03

## 一、问题陈述 {#problem}

正身 v3 身份串把身份核、会话绑定、新鲜度三层安全目标熔进同一盐哈希，五易变件（pid、ppid、parent_start、net_time、timestamp）加每次调用随机盐使同席同日哈希必漂移：2026-09-02 一日四哈希（58e22070、14cfa9c2、9634b1de、d9fd7386，SPEC-016 附录在档），tally R5 逐字节配对新报告必落漂移挂起。租约三键绑定（hostname、user、boottime）未坏，病灶是连认键选了必然易变的哈希。

## 二、关键设计 {#design}

双哈希方案（决策材料 sih-engine/sih/event/plan/pk041-solo-materials/identity-drift-proposal.md §4 与 §7，已采纳）：

- core_hash：身份核六件（mac、hostname、user、boottime、sandbox_id、归一化血统 token）无盐确定性 SHA-256，键序固定，缺席即空串；格式 core-v1 管道键值串；跨报告连认配对键
- full_hash：现役 v3 盐哈希与身份串一字不改，逐报告防篡改与防重放职能不变
- 归一化血统 token（已采纳候选规则）：ancestry 按 > 分段，剔除通用包装段集 {python3, uv, zsh, bash, sh, launchd, agent-tool-host}，zcode-host-local-N 归一为 zcode-host-local，余段 > 连接；空 ancestry 出空 token 不判败
- net_time 移出任何连认哈希；HTTP 探针可选化：显式 --no-net 加超时自动降级双轨，缺席即空串加异常标记不阻断；detect_anomalies 的 clock_skew 逻辑在场时不动
- tally R5：核哈希优先配对；材料或基线任一缺席 core_hash 即回退现行为，旧材料重放逐字节同判；R5 三态映射不动
- lease load_identity 增读 core_hash 入台账 identity 字段；identity_hash、file_sha256、会话号派生、三键硬闸全不动
- 声明式注入（SIH_SESSION_ID 与席位声明）已裁定与本批解耦，不在本批

金标：现役席位 core-v1 串 core-v1|mac=1cf64c65312e|hostname=MiniServer|user=moc|boottime=1787917459|sandbox_id=|lineage=zcode-cli>zcode-host-local>ZCode 的 SHA-256 等于 82f460c2ac2741fc4ee8d104d7ef14803f744da14a24b1d7356bd3bd3531d1dc，测试逐字节断言。

## 三、工作清单 {#work}

- [x] F-1 identity core.py：core_hash 函数与核键序常量与归一化血统 token；net_time 探针可选化双轨；v3 身份串、identity_hash、new_salt、COMPONENT_ORDER 逐字节不变
- [x] F-2 identity cli.py：报告增 identity.core_hash 与 identity.core_components；退出码三值与既有参数不动
- [x] F-3 lease core.py load_identity：增读 core_hash 入台账；其余不动
- [x] F-4 tally cli.py R5：核哈希优先配对带回退；旧材料重放同判
- [x] F-5 三工具 CONTRACT.md 各一条修订（identity 核哈希字段与可选探针、tally R5 语义、lease load_identity 增读）
- [x] F-6 测试守卫：三工具测试全绿；新增用例含确定性双跑、五易变件隔离、五向篡改各变核哈希、归一化四例、离线零网络、boottime 加一秒换纪元、存量 39 报告 full 复算逐字节回归、R5 旧材料回放同判、金标断言
- [x] F-7 治理收口：ask3 双门、intent 上链、管线、认证、双仓 settle、链 valid、结果档

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 | 实装 | core_hash 金标逐字节；五易变件与盐全改核哈希不变；五向篡改各变；v3 三件 git diff 为空 |
| F-2 | 实装 | 报告新字段在场；退出码 0/1/2 三场景实测不变 |
| F-3 | 实装 | 台账新会话 identity 字段含 core_hash；旧键全保留 |
| F-4 | 实装 | 四漂移报告材料对旧基线回放落回退路径同判挂起；同席新基线含 core_hash 即配对成立 |
| F-5 | 文档 | 三 CONTRACT 修订各一条，修订号递增 |
| F-6 | 测试 | 三工具 pytest 全绿计数入档；新增用例先红后绿或随实装绿 |
| F-7 | 收口 | 链 verify 0；reconcile 双零；管线 findings 亲读留证 |

## 五、请求写入节 {#scope}

- sih-tools/identity/src/identity/core.py
- sih-tools/identity/src/identity/cli.py
- sih-tools/identity/CONTRACT.md
- sih-tools/identity/tests/
- sih-tools/lease/src/lease/core.py
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/tests/
- sih-tools/tally/src/tally/cli.py
- sih-tools/tally/CONTRACT.md
- sih-tools/tally/tests/
- sih-tools/nomenclator/packs/core/
- sih-engine/sih/state/plan/idcore-solo.md
- sih-engine/sih/event/plan/idcore-solo-materials/
- sih-engine/sih/event/trail/2026-09-03.ndjson

禁区：历史 seat baselines（sih-tools/proposition/DES/**/seat-baseline-*.json）、台账历史行、存量 reports、identity/reports 新报告不入版控、pk-037/039/041/042 泊件、intanchor 与 pk-043 相关件零触碰。
