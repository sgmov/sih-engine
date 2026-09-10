# mcpmanual-solo 批任务包：MCP 调用 AI 使用说明书——AI-MANUAL 正典件与 /manual 面与双实例 instructions 指针与漂移守卫

> 令源：用户 2026-09-10 令「司梦彩排阶段，对于司衡mcp的使用没有概念，我们缺失了一份针对mcp调用的ai使用说明书。」
> 形：solo 批独立立约独立收约；跨工具生命周期教学（体检三具与五步写入链）与跨工具错误表首次成册
> version: v1

## 一、使命

把司衡 MCP 面的使用知识从「逐工具描述自足」补齐到「跨工具生命周期成册」：零上下文外部冷代理（司梦彩排侧）经一份说明书即知面内有什么、先调什么、写入怎么走、被拒怎么改、哪里有缺口。判定语义零新增——说明书是教学面不是契约面，每个事实与实装源码对表并用漂移守卫测试机械钉住。

## 二、关键设计

**件一 AI-MANUAL.md 正典件（sih-tools/mcpline/AI-MANUAL.md）**

- 读者设定：经 MCP 连接司衡的零上下文 AI 代理（外部冷代理，DES-016 中性化，零哲学代强制，操作语不引命题）
- 章节序：三十秒上手（体检三具加五步写入链）→ 接入形与身份（stdio 与 HTTP 与标识牌与档位与降只读形）→ 铁律六条 → 工具目录（α 七加 β 十逐具表，stdio 本地两具申报）→ 标准剧本五则（体检、命名前查词、写入全链、撞锁排队、停泊）→ 错误语义（载荷字段形加退出码三值加高频理由码表对 REASON_TABLE）→ 命名纪律（六态加 stem 闸两形）→ 最小工作例（照 test_http_write_loop 六步真实字段）→ 边界如实申报 → 正典指针
- 事实源逐一对表：理由码表对 writeface/errors.py REASON_TABLE；剧本对 test_http_write_loop；critsweep 边界对 sweep.py root_form 实装（domaware-solo 已修新城形，申报 root_form 字段非降级旧闻）

**件二 /manual 只读路由（web.py）**

- GET /manual：读 AI-MANUAL.md 原文 text/markdown 返回；文件缺席出教学语（读指针与仓内位）；单一 canonical 路径（manual_file() 函数，包仓根解析）
- _page 页脚与 /tokens 台面加说明书链接；路由只读零写通道（零写入守卫兼容）

**件三 双实例 instructions 指针（server.py 与 httpface.py）**

- stdio 与 HTTP 两 FastMCP instructions 增：说明书两位（/manual URL 与仓内路径）加体检三具速览加五步写入链速览加「被拒是教学改做法不重试」一句
- 工具描述冻结文本零改动（instructions 是服务器级指针位，改动走 DES-015 修订五申报）

**件四 漂移守卫测试（tests/test_ai_manual.py）**

- 面工具名逐一对表：HTTP_ALPHA_TOOLS 加 HTTP_WRITE_TOOLS 十七名逐一出现在说明书文本（防面增删而说明书写旧账）
- stdio 本地两具（record_direct 与 lease_unclaim）在边界节在列
- 理由码抽样对表（locked_elsewhere 与 open_precheck_conflict 与 chain_gate_missing 与 PackageAlreadyClaimed 与 IntentRecordUsedRejected 与 path_outside_domain 与三因 401 码）在说明书错误表在列
- instructions 指针断言（两实例 instructions 载 /manual 或 AI-MANUAL 字样）；/manual 路由 200 加 markdown 形加关键节标题；/tokens 与 _page 页脚链接在位

**件五 词形登记与版本与文档**

- nomenclator core 包登记 mcpmanual（code）加 AI 使用说明书（zh）一词条 established；登记后跑化格归一（packhyg 先例）；登记后 query established 双读数在档
- mcpline 0.8.0 升 0.9.0（__init__ 与 pyproject 与 README 同步）；README 增说明书节（读者、两位获取形、漂移守卫）
- DES-015 修订五：登记说明书位（/manual 路由加双实例 instructions 指针加漂移守卫测试位加语汇位）；走管线三步

## 三、工作清单

- [ ] T-1 AI-MANUAL.md 成文
- [ ] T-2 /manual 路由与页脚链接
- [ ] T-3 双实例 instructions 指针
- [ ] T-4 test_ai_manual.py 漂移守卫（先红后绿）
- [ ] T-5 词形登记在册与化格归一
- [ ] T-6 版本三处升 0.9.0 与 README 说明书节
- [ ] T-7 DES-015 修订五走管线三步
- [ ] T-8 结果档与认证上链与双仓 settle 与收约对账与主树复跑

## 四、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F-1 | 说明书完整性 | 十七具名逐一在说明书文本（漂移守卫绿）；理由码抽样在错误表；剧本六步与 test_http_write_loop 对表 |
| F-2 | 路由 | GET /manual 200 text/markdown 含关键节标题；文件缺席教学语形；/tokens 与页脚链接在位 |
| F-3 | instructions | 双实例 instructions 载说明书指针与体检三具与五步链速览字样 |
| F-4 | 登记与归一 | mcpmanual query established 双读数；登记后 core 包 pytest 全绿（canonical 形） |
| F-5 | 回归 | mcpline 族主树复跑全绿；零写入守卫与套件窗口零写证明不破 |
| F-6 | 文档 | DES-015 修订五走化格核阅检词（state/plan 与 event/plan 域外 exit-2 如实记）；DES-015 修订记录节升位 |

## 五、必读文件

- sih-tools/mcpline/src/mcpline/writeface/errors.py（REASON_TABLE 对表源）
- sih-tools/mcpline/tests/test_http_write_loop.py（剧本对表源）
- sih-tools/mcpline/src/mcpline/server.py 与 httpface.py 与 web.py（实装现场）
- sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md（修订五宿主）
- sih-engine/sih/state/plan/domaware-solo.md（任务包形参照与 critsweep 新城形令源）
- sih-tools/BATCH-FACE.md（批机械链正典）

## 六、约束（红线）

1. 说明书是教学面：零新增判定语义零新增工具零契约改动；SPEC-023 与 DES-014 与矩阵零触碰
2. DES-016 中性化：外部冷代理读者，操作语零哲学引文零哲学代强制
3. 工具描述冻结文本零改动；instructions 改动走 DES-015 修订五申报
4. 禁触：sih-visual、中央登记册实测数据、引擎 Rust 零改动零重编、record_direct 与 lease_unclaim 照旧零注册 HTTP 面
5. 主树零直写：一切待提交件先入工地；先红留痕禁清洗
6. 每条命令立即取退出码失败即停整链禁管道掩码
7. scribe 二进制一律主树 target/debug/scribe

## 七、验收标准

- test_ai_manual.py 先红后绿实录齐；mcpline 族主树复跑全绿
- GET /manual 200 markdown 实录；十七具名对表读数在档
- mcpmanual 登记后 query established 双读数在档；登记后 core 包 pytest 全绿
- DES-015 修订五管线三步读数在档
- 认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid、收约回执在场

## 八、风险点

- 说明书文本与实装漂移是长期病：漂移守卫测试只钉工具名与理由码与指针三类硬事实，语义漂移靠「冲突时以出参与正典为准」条款兜底，如实申报此界
- critsweep root_form 表述依赖 domaware-solo 落地态（已核 detect_layout 在主树实装）；若表述与实装有出入以 sweep.py 为准修文
- instructions 增字对冷 agent 上下文占用：控制在十行内，速览不成文
- nomenclator packs/core 是 domaware-solo 刚登记过的活面：登记前先读现形避免词条键冲突

## 九、范式偏离声明

无（用户令直承缺口；批名词形 mcpmanual 走 --new-stem 认领形即闸既裁通道，非偏离）。

## 十、关联文件

- mcpnomgate-solo（stem 闸与检词两具先例）；domaware-solo（critsweep 新城形与任务包形参照）；mcpboot-solo（域自举与 README 域自举节先例）；DES-015/DES-014/SPEC-023（正典指针）

## 十一、请求写入（逐路径分行）

sih-tools/mcpline/AI-MANUAL.md
sih-tools/mcpline/src/mcpline/web.py
sih-tools/mcpline/src/mcpline/server.py
sih-tools/mcpline/src/mcpline/httpface.py
sih-tools/mcpline/src/mcpline/__init__.py
sih-tools/mcpline/pyproject.toml
sih-tools/mcpline/README.md
sih-tools/mcpline/tests/
（目录级理由：test_ai_manual.py 新件出生地）
sih-tools/nomenclator/packs/core/
（目录级理由：mcpmanual 词条登记走 register 写位与化格归一）
sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md
sih-engine/sih/state/plan/mcpmanual-solo.md
sih-engine/sih/event/plan/mcpmanual-solo/
（目录级理由：结果档与批材料保留位出生地，先红留痕与管线读数入档；facefit 命名空间面子路径保留位形）
sih-engine/sih/event/trail/2026-09-11.ndjson
sih-tools/scribe/reports/
（目录级理由：ask3 双门与叩问与认证报告出生地）
sih-tools/identity/reports/
（目录级理由：正身件出生地）
