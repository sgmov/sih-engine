# domaware-solo 批任务包：M2 域感知四件——critsweep 新城形与 rootanchor 域界与收约回执闸跳显形与 gauge 域件

> 令源：用户 2026-09-10 令「同意M2实装」承司梦侧 M2 域感知问题报告（critsweep 新城形 degraded 与 phantom 缺日与泊界 exit 2；bare lease CLI 城内读中央台账跨域污染实证 db073c5c16e82a5b；闸跳静默；gauge 域件缺席）；工作台 P0/P0.5 用户同令划出本域
> 形：solo 批独立立约独立收约；M5（温故 MCP 投影与新城索引）候裁不在本批
> version: v1

## 一、使命

把 mcpauth-solo 既裁「链与锁册与台账与泊界住各项目自己的 sih 目录」从登记面落进三工具的路径解析执行位：critsweep 与 lease 与 gauge 遇新城正典域各认各的域界与域册，中央 first_domain 形零回归；收约回执把静默闸跳显形计数。判定语义零新增——布局判别即目录标记静态规则，台账解析即根拼接，闸跳计数即既有回执字段归集。

## 二、关键设计

**件一 critsweep 布局感知（sweep.py）**

- `detect_layout(root)`：root 下 sih-engine/Cargo.toml 与 sih-tools/pyproject.toml 俱在即 first_domain；root/sih/ledger 为目录即 canonical（新城正典形，与 mcpline 开域落地五步及测试夹具同形）；余即 unknown 如实降级
- canonical 路径形：trail=root/sih/event/trail；sessions 与 locks 台账=root/sih/ledger/*.ndjson；泊界单线（"domain"）=root/sih/state/parking/materials，路由仍走中央 selector（cwd 与 --pack 解析自 sweep.py 自身位置上溯的中央 sih-tools——两根分离承 stem 闸先例：码根中央、数据根本域）
- resolve_root 上溯搜加 canonical 标记（sih/ledger 目录）与 first_domain 标记并列先到先得
- 出参增 root_form 字段（first_domain 或 canonical）；first_domain 形输出逐字节零回归

**件二 lease rootanchor 域感知（core.py 与 cli.py）**

- `is_canonical_domain(path)`：path/sih/ledger 为目录即真
- `discover_workspace_root`：walk-up 每候选先检 canonical 标记即返域根，域界即停不越界（city 嵌套中央树下不再穿透读中央）
- CLI 台账缺省（main 内 session_ledger 与 locks_ledger 缺省位）：显式参优先不变；缺省改经根锚——root 为 canonical 域即 <root>/sih/ledger/，余即 tool_dir()/ledger/（现形零变）；域根基底取显式 root 缺省 cwd 上溯
- resolve_root 与五验与链闸与 hooks 零触碰；core 层 close_session 锁册缺省位不动（CLI 层单点修复）

**件三 收约回执 gates_skipped（core.py close_session）**

- 归集既有闸跳事实成 `gates_skipped` 字段：{"count": N, "orphan_bypass": 事由或 null, "calllog_bypass": 事由或 null, "ack_uncommitted": 路径数组}；count 即非空项计数
- 入 close 返回回执与 revoked 链行 detail 双面；零跳过形 count 0 全空如实呈现
- 闸本体判定语义零变更——只归集已发生事实，不新增旁路

**件四 gauge 域件与跨域卫（gauge/cli.py）**

- read 与 record 增 `--domain-root`（canonical 域根）：给参即数据面缺省展开——trail=<域>/sih/event/trail 全日、sessions-ledger=<域>/sih/ledger/sessions.ndjson、locks=<域>/sih/ledger/locks.ndjson、record 的 record-trail=<域>/sih/event/trail/<at>.ndjson；码面（src-root 与 tools-root 与 scribe）缺省落中央码根（gauge 包位置上溯两级的 sih-engine 与 sih-tools 与其 target/debug/scribe——两根分离同件一）；显式参恒优先
- 跨域卫：read 与 record 与 gqueue 的数据面路径（trail 与 sessions-ledger 与 locks 与 record-trail）逐件上溯域标记（first_domain 对或 canonical sih/ledger），非同域根即 exit 2 fail-closed 报文列分歧行——混域静默读数即 M2 病灶根治位
- 码面不参卫（组件清单与二进制天然属中央码根）

**件五 版本与文档与登记**

- lease 1.42.0 升 1.43.0（pyproject 与 __init__ 与 CONTRACT 同步）；CONTRACT 修订五十九（canonical 域台账缺省形与 discover_workspace_root 域界停止与 gates_skipped 回执字段）
- critsweep CONTRACT 修订（detect_layout 与 root_form 与中央码根路由）；critsweep 版本 1.0.0 升 1.1.0
- gauge 版本升位（__init__ 与 pyproject）；gauge 无 CONTRACT，用法入 --domain-root help 与 README 如缺席则读数入批材料
- 新词登记：domaware（code）与域感知（zh）同词条 established 入 core 包，manifest 升位；gates_skipped 工程字段名不登记
- CALL-LOG 三册（critsweep 与 lease 与 gauge）经 lease call-log append 追加

## 三、工作清单

- [ ] T-1 critsweep detect_layout 与 canonical 解析与 root_form
- [ ] T-2 lease 域感知两函数与 CLI 台账缺省改线
- [ ] T-3 close_session gates_skipped 归集
- [ ] T-4 gauge --domain-root 与跨域卫
- [ ] T-5 三族测试（critsweep 与 lease 与 gauge）
- [ ] T-6 两 CONTRACT 修订与版本三升与新词登记与 CALL-LOG 三册
- [ ] T-7 结果档与认证上链与双仓 settle 与收约对账与主树复跑

## 四、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F-1 | critsweep | canonical 夹具跑 root_form=canonical、判据读数零降级、泊界 domain 线 exit 0；first_domain 主树实跑输出零回归 |
| F-2 | rootanchor | canonical 域内 cwd 裸调台账缺省落 <域>/sih/ledger；discover_workspace_root 域界停止；中央 cwd 形缺省零变 |
| F-3 | gates_skipped | 双 bypass 形 count 2 双事由在列；ack 形路径在列；零跳过形 count 0；revoked 链行 detail 同字段 |
| F-4 | gauge | --domain-root 一旗读数与落域链成；混域数据面 exit 2 报文列分歧行；显式参优先回归绿 |
| F-5 | 回归 | 三族测试主树复跑全绿 |
| F-6 | 文档 | 两 CONTRACT 修订走管线三步；新词登记两笔在册；CALL-LOG 三册三腿齐 |

## 五、必读文件

- sih-engine/sih/event/plan/mcpauth-solo-results.md（每项目一域既裁与台账泊界域册布局令源，温故 recall 在档）
- sih-tools/mcpline/tests/fixture_root.py（canonical 夹具正典形参照）
- sih-tools/critsweep/sweep.py 与 CONTRACT.md（现形）
- sih-tools/lease/src/lease/core.py（resolve_root 与 discover_workspace_root 与 close_session 现场）
- sih-tools/gauge/src/gauge/cli.py（参数面现场）
- sih-engine/doc/decision/021-adjudication-dispatch.md（裁决分派界：本批工程实装非裁决类）

## 六、约束（红线）

1. mcpauth-solo 既裁边界：工具中央一份全域共用，本批只改路径解析不改工具部署形；不迁移司衡仓第一域历史布局
2. lease 五验与链闸与 closeguard 与 git hooks 与既有闸序零触碰；gates_skipped 只归集不新增旁路
3. DES-016 中性化：本批三工具对新城域只解析不代强（无哲学面投影）
4. 禁触：sih-visual、中央登记册实测数据、InferServer 与司梦工作区、引擎 Rust 零改动零重编
5. 主树零直写：一切待提交件先入工地；先红留痕禁清洗
6. 每条命令立即取退出码失败即停整链禁管道掩码
7. scribe 二进制一律主树 target/debug/scribe

## 七、验收标准

- 三族测试主树复跑全绿；F-1..F-4 实录齐
- 两 CONTRACT 修订走化格核阅检词管线（state/plan 与 event/plan 域外 exit-2 如实记）
- 新词登记两笔在册且 register 后 query established
- 认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid、收约回执 gates_skipped 在场

## 八、风险点

- critsweep 主树自跑即验收现场（会话启动挂点）：改坏即当日扫红——先工地绿再主树跑，红即停批
- resolve_root 上溯加 canonical 标记后 math 侧任务包目录（TASK_PACKAGE_DIRS 含 sih/state/plan）与 canonical 形交汇：resolve_package 不在批内改面，实查 sih-math 形零回归
- gauge record 落链走 scribe 锁位前查：--domain-root 形下 --locks 缺省展开须与 record-trail 同域，否则闸前读数错册——跨域卫覆盖
- gates_skipped 链行 detail 扩字段对冻结金向量（lease tests/frozen）漂移风险：若 sweepjson golden 载 revoked detail 即同步刷新并单字段漂移申报

## 九、范式偏离声明

无（用户令「同意M2实装」直承上批完工报告候令项；工作台划域与 M5 候裁两边界用户同日令定）。

## 十、关联文件

- mcpauth-solo-results.md（每项目一域既裁）；mcpnomgate-solo（stem 闸两根分离先例与 M2 报告源）；DES-015（域布局两形）；DEC-021（裁决分派界）；critsweep-solo（判据扫挂点正典）

## 十一、请求写入（逐路径分行）

sih-tools/critsweep/sweep.py
sih-tools/critsweep/CONTRACT.md
sih-tools/critsweep/tests/
（目录级理由：布局感知测试新件出生地）
sih-tools/lease/src/lease/core.py
sih-tools/lease/src/lease/cli.py
sih-tools/lease/src/lease/__init__.py
sih-tools/lease/pyproject.toml
sih-tools/lease/CONTRACT.md
sih-tools/lease/tests/
（目录级理由：域感知与 gates_skipped 测试新件出生地）
sih-tools/gauge/src/gauge/cli.py
sih-tools/gauge/src/gauge/__init__.py
sih-tools/gauge/pyproject.toml
sih-tools/gauge/tests/
（目录级理由：域件与跨域卫测试新件出生地）
sih-tools/nomenclator/packs/core/
（目录级理由：新词登记两笔走 register 写位与 manifest 升位）
sih-engine/sih/state/plan/domaware-solo.md
sih-engine/sih/event/plan/domaware-solo-results.md
sih-engine/sih/event/plan/domaware-solo-materials/
（目录级理由：批材料出生地，先红留痕与温故 recall 与管线读数入档）
sih-engine/sih/event/trail/2026-09-10.ndjson
sih-tools/scribe/reports/
（目录级理由：ask3 双门与叩问与认证报告出生地）
sih-tools/identity/reports/
（目录级理由：正身件出生地）
