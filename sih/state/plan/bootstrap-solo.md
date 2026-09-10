# bootstrap-solo 批任务包：M4 客户端配置会话在役化

## 判词

M4 修复批，域自举线会话收口。病灶一行摘要经父会话回查仅存 client config not in session；机械面按四层缺口重建，三件修复加一件送达机制，TDD 先红后绿，DES-015 修订五走 T6 管线，主树真跑验收。批名 bootstrap-solo 承在册词 bootstrap／域自举（2026-09-10 立名会裁决），段查册过闸不铸新词。

## 病灶（重建申报）

原报原文已压缩不可回，病灶名 client config not in session。机械面重建四层，完工后随结果档呈用户复核：

1. 出参教学缺口：bootstrap 出参 next 单行零重启生效语义零验证路径；client_config.note 零生效条件。客户端配置于新会话读取、既有会话不重载这一决定性事实零申报。
2. 写后零复核：客户端配置合并写毕零重读断言，畸形结果零检出。
3. 域树零会话可发现落位：开域链只落 sih/ledger/tokens.ndjson 镜像，域面接法即面 URL 与标识牌与配置路径与验证法零落位，域内冷会话不可自发现自己面。
4. 送达死递：已开域（手工 onboarding 域即报病灶域本身）重跑 bootstrap 在段一被 _precheck_bootstrap_face 以域已开域拒（bootstrap.py 幂等守卫），修复永达不了报病灶的域。

## 修复件

### 件一 出参教学定形加写后复核（bootstrap.py）

- client_config.note 增生效条件：客户端配置于新会话读取，既有会话不重载，须重开方见 sih 工具面；并载配置路径。
- next 改教学定形：重启语义加在役验证两法（其一新会话中 MCP 面 locks_read 心跳可调即配置在役；其二 HTTP 面同形 POST tools/list）加面卡指针（sih/README.md）。
- 写后复核位：客户端配置写毕重读该文件，断言 mcp.servers.sih 固定 payload 即 type http 与 url http://127.0.0.1:8765/mcp 与 Bearer 头与 enabled 真值与 timeoutMs 六万在位；缺即 exit 1 教学 JSON 零静默。

### 件二 域面说明落位（init.py 与 bootstrap.py，单一正典生成函数）

修订三载明 init 开域已落 sih/README.md 域自述（树图四行用法三条正典指针三条）。本件不是新文件是扩形：域自述卡生成收敛单一正典函数并增面说明节，init 开域与 bootstrap 全链与 bootstrap --complete 三路同源。写 <域根>/sih/README.md，幂等恒等重写。面说明节内容定形：

- 面 URL 与标识牌词形（语汇遵 DEC-010：标识牌是明文短标识防呆锚点非安全凭据，全篇禁「凭据」指称标识牌）与 scope。
- 中央登记册镜像指针（sih/ledger/tokens.ndjson 末行 active 为准）。
- 客户端注册路径与固定 payload 形（承 DES-015 修订四）与旗标选入语义（--client-config 显式给参才写）。
- 生效条件（重启语义）与在役验证行（两法，与件一同源）。
- 重开域禁令与 --complete 补全形指针。
- 典源行：DES-015 修订五；DEC-010。

### 件三 验证行实证（tests）

测试内经 tests/http_harness.py LiveServer 以与教学行同形的请求真调 tools/list，断言 200 且工具面在列。教学只列已验事实（鉴之虚静）：验证行写进面卡前先经活服实证，实证读数入批材料。

### 件四 --complete 补全形（bootstrap.py main 增旗标）

- 已开域无 --complete：拒照旧，教学更新指向 --complete 补全形。
- 已开域有 --complete：段一余检照跑（存在与目录与 .git 与非中央根加登记册读数），签发位复用照旧（active 复用 done:false），开域位改验域形（读 sih/domain.json 加验域链 valid），镜像核对位（中央 active 行与域镜像末行恒等对表，缺行补写恒等行），面卡落位，客户端注册位照跑。全程零域状态写零 reinit 语义，重开域候裁维持不动。
- 未开域给 --complete：正常全链，旗标无害。

## 验收判据（可证伪）

- T-1 先红留档：教学与面卡断言先红日志入 bootstrap-solo-materials/，禁清洗。
- T-2 全链 fresh：--client-config 加新域，镜像行在位加 sih/README.md 在位含面 URL 与标识牌词形与验证行与生效条件行，全篇无「凭据」指称标识牌。
- T-3 幂等重跑：面卡重写逐字节恒等；客户端配置幂等其余键保留。
- T-4 台面开域：/tokens/confirm-open 后面卡在位（同一 run_chain 路径零二次实现）。
- T-5 验证行实证：LiveServer 同形 POST tools/list 200 且工具面含 locks_read。
- T-6 写后复核：正常链复核通过；构造缺 sih 键的合法 JSON 旧形文件走注册位被拒 exit 1。
- T-7 已开域两态：无 --complete 拒且教学含 --complete 指针；有 --complete 补全全链 active 复用 done:false 加镜像核对加面卡在位加域链 verify 事件数恒等（零新笔）。
- T-8 出参形：严格 JSON 单对象，client_config.note 与 next 含重启语义与验证路径。
- T-9 全套回归：工地 mcpline pytest 全绿（现 117 基线加新增）。
- T-10 主树真跑：合并后主树 mcpline pytest 全绿（F-1 硬性项）。

## 治理包裹

- 三问双门绿（ask3repeater status ok anchor_count 3，scrutinator ask3 零 finding；首跑红 A11 A12 已归档 materials）；叩问 digest passed（标识牌 unregistered 轻信号，候 mcpmanual-solo 登记批消解）；正身零异常。
- 协调红线（锁台账 2026-09-11 00:57 读数）：wengumcp-parallel 会话 cbdff9259c1aa6b0 在飞持 mcpline/tests 与 README.md 与 pyproject.toml 与 __init__.py 与 server.py 与 httpface.py 与 writeface/matrix.py 与 SPEC-023 与 retriever 全源——本批零触碰其面；mcpline/tests 候其放锁后取锁开工；trail 2026-09-11 为 append 共享面（judouwire 与 wengumcp 双持并存先例）可并写。mcpmanual-solo 术语登记面 packs/core 零写，标识牌登记候其消解。
- 版本号位零触碰：mcpline 0.8.0 维持，版本升位随 wengumcp 后批（如实申报）。
- DES-015 修订五（域面说明落位形加出参教学定形加写后复核位加 --complete 补全形四点）走 T6 管线化格核阅检词；任务包与结果档为 des-001 域外 exit-2 如实记。
- CALL-LOG 走 lease call-log append mcpline 一笔。
- 结算：双仓 settle 加 close 加 reconcile 双零加 scribe verify valid 加回锚重跑五行回显。

## 明确不做

- web.py 零改（面卡经 run_chain 单一正典路径自然覆盖台面开域）。
- reinit/重开域维持候裁（--complete 零域状态写非 reinit）。
- 立名供给三件已入泊 pk-090（2026-09-11 用户令）候令另批。
- M2 域感知面与 M5 温故面零触碰（用户既裁另 agent 在飞）。
- mcpline 包 README.md 零改（wengumcp 在持；面卡正典落 DES-015 修订五，包 README 更新候后批）。
- 版本 bump 候后批。

## 请求写入（逐路径）

sih-tools/mcpline/src/mcpline/bootstrap.py
sih-tools/mcpline/src/mcpline/init.py
sih-tools/mcpline/tests/
sih-tools/mcpline/CALL-LOG.md
sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md
sih-engine/sih/state/plan/bootstrap-solo.md
sih-engine/sih/event/plan/bootstrap-solo-results.md
sih-engine/sih/event/plan/bootstrap-solo-materials/
sih-engine/sih/event/trail/2026-09-11.ndjson
sih-engine/sih/state/parking/materials/pk-090.json
sih-tools/scribe/reports/
sih-tools/identity/reports/
