# mcpwrite-solo 批结果档：HTTP 写面实装——标识牌域绑定与签发台与域布局规约

> 批名：mcpwrite-solo（solo 批独立立约独立收约，与主窗并行共用 2026-09-09 当日链）
> 会话号：62d01f86d6a40ba7（lease 1.39.0 签发，identity 报告 2026-09-09-mcpwrite-solo-identity.json，anomalies 空）
> 令源：DES-015 修订二（防呆形，m-mcpauth-2 九发 stable_clear 终签 cf826465）即唯一设计正典；任务包 sih-engine/sih/state/plan/mcpwrite-solo.md
> 批性质：T6 实装批——sih-tools/mcpline 承 DES-015 六件实装，mcpline 工具域外于 sih-engine 文档规范（mcpserv-solo 先例），本档归 sih/event/plan 域外 des-001 核阅域（exit-2 如实记入档）

## 一、实装六件终值

1. **tokens 明文登记册**：`mcpline/src/mcpline/tokens.py`——六字段固定（token_id、domain_root、scope、status、issued_at、issued_by）；scope 三值 readonly/domain_write/custom，status 两值 active/stopped，token_id 可读短串词形校验；写点 flock 串行加 O_APPEND 单次 write 原子整行加 fsync（`append_row`），文件缺席 O_CREAT 建；读数逐行解析每 token_id 取末行（`load_last_rows`/`resolve_token`），行形违例 TokensLedgerError 带行号显形；停行 `revoke_row` 承末行绑域与档位。落位两形：第一域 `sih-tools/mcpline/ledger/tokens.ndjson`（初册空文件随批入库），新城 `<域根>/sih/ledger/tokens.ndjson`（domains.py 投影）。测试：套件 92 绿含专用守卫（flock 加 O_APPEND 在位、整文件重写动词零命中）。
2. **签发台进视图**：`mcpline/src/mcpline/web.py`——`/tokens` 管理台三动作（列表 GET /tokens、签发 POST /tokens/issue→确认 /tokens/confirm-issue、撤销 POST /tokens/revoke→确认 /tokens/confirm-revoke），签发与撤销各带一步确认（确认步显示标识与所绑域与 scope，确认前零落笔实测读数 readings `console_issue_confirm_step.落笔前行数=3→确认后=4`）；动作即台账追加行（issued_by=console）；无管理钥；同栈同端口（与面板静态页同一 Starlette 应用）。重复 active 签发拒、非法 scope 拒（400 教学页零追加）。
3. **Bearer 头识别与域绑定**：`mcpline/src/mcpline/httpface.py`——BearerGateMiddleware（纯 ASGI，仅拦 /mcp POST 的写面十具）查中央登记册（`resolve_root()/sih-tools/mcpline/ledger/tokens.ndjson`，解析单点）；401 三因直说（missing_authorization 缺头／token_unregistered 不在册／token_stopped 已停行），403 分档拒（readonly 拒写、custom 零缺省放行）；active 牌逐调用新读登记册（零缓存执法）；停行牌降只读投影中央域且读出参附 identity_notice 教学语；一标识牌至多一活跃会话（SessionTable 按 token_id 一对一，close 成即解绑可再立，换绑域旧对象让位）；正身可选件 session_id 位注入项目标识（`mcpconn-<token_id>-<hex>`，DES-015 身份锚判词承载）；信封归因到域（会话行落所绑域会话册）。
4. **域布局规约两形**：`mcpline/src/mcpline/writeface/domains.py`——DomainLayout 路径投影（trail/ledger_paths/tokens_path/parking_dir/repos）；两形判别静态规则（域根==中央根即第一域映射形，其余新城正典形）；第一域映射登记表数据（FIRST_DOMAIN_* 常量：sih-engine/sih/event/trail、sih-tools/lease/ledger、sih-tools/mcpline/ledger/tokens.ndjson、双仓 repos），新城 `<域根>/sih/` 正典单根；承接 CLI 调用（scribe query/verify --trail、gauge read --trail/--sessions-ledger、lease 子命令 --root/--ledger/--locks/--claims）按域路径组装（passthrough.py 十二 argv 构造全承 DomainLayout），两形俱测（test_domain_layout 五绿加 HTTP 全链路两形各一）。
5. **HTTP 写面十具按域放行**：httpface 注册恰 α 五加 β 十（`assert_http_face` 启动复核 fail-closed；record_direct 与 lease_unclaim 零注册，takeover/bypass 照旧零注册）；域锚定——trail 与台账与 --root 按所绑域组装，repo 域指向参不透传（对表域仓集 `DomainLayout.repos()`，集外拒），路径参绑定验（静态词法前缀须属所绑域根，越界 path_outside_domain 403 教学）；scope readonly 与 custom 拒写透传、domain_write 十具透传；既有 CLI 执法透传零新增执法（scribe 三闸与租约五验与 closeguard 原位，拒定全部来自既有理由码）；写载荷加 retry_discipline 字段。
6. **stdio 零回归**：server.py 十五具注册面与分级裁剪零改动（五只读函数仅增内部 layout 可选 kwarg，None 即旧行为；注册 schema 零变）；test_stdio_smoke、test_write_gates、test_write_matrix、test_tools_unit 原文件零改动全绿；套件窗口双仓 git status 全等（test_zero_write 92 绿含零写证明）。

## 二、验收测试六组读数（工地全套件 92 passed，1:32，日志 materials/test-full-run-2026-09-09.log）

1. **签发台端点**（test_token_console，7 绿）：确认步未落笔断言、确认追加行六字段、重复 active 签发拒、非法 scope 400 零追加、撤销确认步追加 stopped 末行、不在册撤销 400、同栈同端口。
2. **识别绑定**（test_http_identity，7 绿）：401 三因直说（缺头/不在册/停行，错误文本零拼接调用方可控数据）、降只读读形 identity_notice 投影中央域、active 读按域组装、一对一（再开 session_already_bound）、写只落所绑域（alpha 链两笔而中央链零写入）、close 后同牌可再立、正身 session_id 位载 token（`mcpconn-alpha-tok-`）。
3. **跨域拒**（test_cross_domain，6 绿）：相对越界（`../sih-tools/README.md`）拒、beta 绝对路径拒、record 越域拒、repo 集外拒，俱 path_outside_domain/403；拒后 beta 会话册零触碰；第一域映射形写径全链（open→lock→intent→append→close，repo 锚定 sih-engine）。
4. **HTTP 写径闭环**（test_http_write_loop，1 绿）：fixture 新城域根内 open→lock→intent→append→chain_verify(valid 恰 2 笔)→unlock→close 真 CLI 全程；会话册 issued 与 revoked 俱在所绑域；收约后 locks_read 域形式零持锁零活跃。
5. **scope readonly 拒写**（test_scope_readonly，5 绿）：只读五具可调（locks_read 域形式）、lease_open/record_append/lease_lock/lease_close 拒写俱 scope_violation/403、custom 零缺省放行拒、拒透传零调用（域内台账与锁册零触碰）。
6. **stdio 回归加零写证明**：test_stdio_smoke（3 绿）加 test_write_gates 加 test_write_matrix 加 test_tools_unit 原样全绿；test_zero_write 套件窗口双仓 git status 全等绿。

读数实录（活服真 JSON 出据）：materials/readings-2026-09-09.json——open_session（repos 锚定所绑域仓集）、跨域拒两例（path_outside_domain/403）、readonly_reject（scope_violation/403）、401 两因、闭环七步全零、chain_verify valid 2 笔、两形路径投影、critsweep 新城降级读（degraded true，degradations 6 条在档）、签发台四步（确认步两形、追加行六字段、停行末行）。

## 三、边界与误差如实申报（应而不藏，零隐藏）

1. **critsweep 新城降级读**：sweep.py 现行版本以 --root 推导第一域双仓路径形（sih-engine/sih/event/trail 与 sih-tools/lease/ledger），canonical 域根即降级（degraded true，degradations 在档可见）。按 DES-015 边界申明「写面投影以现行各工具版本为界」承载，critsweep 域内回算候其自身换版程序，本批零工具改动。第一域形全量回算照旧（判据扫五判据 achieved 绿）。
2. **canonical 域收约链闸与无主闸空转**：lease close 缺省链枚举只认第一域双居所（_default_trails），带 --trail 即触发无主闸 watchcheck dirty_set 对 root/sih-engine 与 root/sih-tools 的 git status，canonical 单仓域根即 ToolError（wprobe 手工复现在档）。故 HTTP 收约 argv 恒不传 --trail：第一域形行为零变（缺省枚举），canonical 形链闸空转（workspace_chainless 注记可见）。链闸域内执法候 lease 侧既有换版程序。
3. **custom 档零缺省放行**：三值在册校验在位，custom 行写工具一律拒透传（403 教学语明示白名单登记形待后继批评审）——DES-015「登记面即授权面，零缺省放行」的忠实承载，白名单行格式设计未定不停批（readonly 与 domain_write 为本批验收两档）。
4. **测试夹具勘误（先红留痕）**：fixture 登记册未入库提交触发 close 无主闸拦收约一例（无主清单指认 sih-tools/mcpline/ledger/tokens.ndjson ??），seed_tokens 补入库提交修复；tests/test_zero_write.py 守卫模式串转义形笔误一处（`['"]` 未转义致收集期 SyntaxError），改 `['\"]` 与原文件同形修复。两例首跑红证在本档与本日志在档。
5. **零写证明豁免申报**：套件窗口双仓全等判据天然豁免两仓既有脏件；本批测试全程 fixture 域根隔离，identity verify 只读采集，真仓零写入由 92 绿含零写证明承载。

## 四、管线与链证

- 管线三步（本档，event/plan 域外形）：化格 formatter --pack packs/general-v1 --write、核阅 scrutinator --pack des-001（域外 exit-2 如实记）、检词 nomenclator check --pack packs/core；读数见链面认证笔与批材料。
- 链面：当日 intent 笔 b51e1e74（event_id b91114d7-6e01-4cc9-84c6-16ccc3dd58ae），认证笔与 settle 与回填段哈希见完工回报（主窗独立复算）。
- 双仓 settle：tools 工地（mcpline 六件加 ledger 初册加测试九件加 README/pyproject）与 engine 工地（任务包两件加本档加 materials 两件），提交号见完工回报。

## 五、完工判定

使命达成：外部项目以标识牌经 HTTP 面绑定自己的域（Bearer alpha-tok 全程写 alpha/sih/ 树），在域内走完整租约流程写自己的链（open→lock→intent→append→verify→close 真 CLI），跨域结构性不可达（绑定验路由加 repo 锚定加既有五验三层）。六件终值俱在第一节，六组测试俱绿，红线零违（零新增执法零 LLM、既有工具代码零改动、sih-visual 与 SPEC-023 与 .zcode 零触碰、冲突走 wait-turn 队列零绕行、逐命令退出码即取即断）。
