# mcpnomgate-solo 批结果档：检词上 MCP 写面——stem 查册闸与两具投影与开闸三毛边

> 批名 mcpnomgate-solo · 会话 3c8442699d11725e · 2026-09-10 · 单线形 solo
> 令源：用户 2026-09-10 令「同意，这个是既有功能没上mcp」承 DEC-017 修订四常设纪律兑现定性（司梦域 M1/M3 源码级问题报告对表在案）

## 一、落地面

- **lease open stem 查册闸**（cli.py CLI 层横切闸位，承 _precheck_gate 先例；core.py 承闸体）：判定语义全循 nomenclator query 既有六态零新造。本域（root 下 sih-engine 与 sih-tools 俱在，静态判别）established 过、dead 与 lazy 与 candidate 拒教学、unknown 拒教学加 `--new-stem` 显式认领（认领即新词认领声明入回执可追）；词典包缺席＝回执显形跳过（checked:false 加 pack_absent_skip，零静默）；nomenclator 工具缺席或 query 异常＝StemGateToolError fail-closed 退出码二拒开（与闸拒 exit 1 分形）；新城正典域零查询零代强制纯教学（DES-016 同形）。回执增 stem_check 与 allow_empty_teaching 两字段（additive，台账行形零变更）。
- **两具投影上 MCP 面**：nomenclator_query（六态查词）与 nomenclator_check（文档两规则核查）入 stdio 面与 HTTP 面（α 只读面五具扩七具，stdio 19、HTTP 17）；register 零投影（登记＝立名入口，红线）；词典中央位域无涉；assert_http_face 与矩阵与面计数断言同步。
- **开闸毛边**：intent 件先验正标签（intent record unreadable / not valid JSON 替代旧形 identity file unreadable 错指，验序与必填集零变更）；scope_violation 与空 allow 教学语指向真病灶。
- **项目位两根分离**：闸的 nomenclator 项目位即 lease 仓兄弟目录（代码根解析，worktree 与主树俱真项目），词典包位即数据根（fixture 词料可控）——承 mcpline SIH_ROOT 与 SIH_MCPLINE_CODE_ROOT 两根既立模式。
- **版本**：lease 1.41.0 升 1.42.0；mcpline 0.7.0 升 0.8.0。

## 二、测试读数（工地窗）

- **lease 全套 330 passed**（含 stem 闸十二件 S-1..S-11 加真词典集成钉、openhyg、sweepjson 金向量重冻后）。
- **mcpline 工地窗 100 passed 加 13 failed**：13 笔全数 write_gates 族，实因本批在飞 23 锁中 SPEC-023 与 trail 等面与测试 allow 面（sih-engine/doc/ 等）交集互撞——环境干扰非码病（本批 intent 面前次 mcpboot 批同形判例在档），绿读数以收约后主树复跑为准。
- mcpline 新增 test_nomenclator_tools 七件绿；面计数断言（stdio 19、HTTP 17、external 17）与 stdio_smoke 期望面俱更新绿。

## 三、文档与管线读数

- SPEC-023 修订一（五具扩七具，register 零投影红线）：化格 0、核阅 0、检词 0（双跑逐字节一致）。
- DEC-017 修订五（执行位落闸三件）：化格 0、核阅 0、检词 0（首跑 1 笔 dead_ban「铸名」红证在档——死因「造词反讽即工具恰拒未裁铸词」，本批教学语全数涤净改循「新词认领」）。
- lease CONTRACT 修订五十八：化格 0、核阅域外 exit 2 如实记（des-001 域只盖 sih-engine/doc）、检词 3 笔存量 lazy 与 dead（资产包、开域、落账——前置修订四十九与五十六时代存量，正典文本不改写，处置候立名程序；本批新增文本零违例）。
- 检词双跑一致性：三档双跑逐字节一致（判据五在档核）。
- AGENTS.md MCP 节原地改（十五具升十七具与 stem 闸申报），改前形归档 mcpnomgate-solo-materials/AGENTS-mcp-section-archive.md。

## 四、先红留痕（禁清洗申报）

1. **stem 闸首跑孵不出**：fixture 数据根的 nomenclator 目录无 uv 工程位，子进程 Failed to spawn——修法即项目位代码根分离（兄弟目录）加包位数据根，红证在 tdd-red 记录与本档申报。
2. **测试隔离缺陷致真台账游离会话**：stem 闸测试初版未显式传 --ledger/--locks，根锚回落写真工作区台账一线（db073c5c16e82a5b），即 M2 串账病活体复刻；修法即测试显式双账本参，真台账复核零残留（grep 零行），红证与本申报在档——第二批 M2 rootanchor 域感知的实证加一。
3. **sweepjson 金向量漂移**：版本位 1.41.0→1.42.0 正当漂移，重冻一字节在档。
4. **核阅首跑红**：无（三档核阅俱零；CONTRACT 域外 exit 2 如实记非违规）。
5. **检词死档红**：DEC-017 铸名一笔（上节）。
6. **面计数漂移三处**：write_matrix 两断言与 stdio_smoke 期望面常量俱红后修（常量补两具）。

## 五、偏差申报

1. 任务包件三原拟「intent 先验前移到身份闸之前」，按红线三（闸序位次只顺延申报不重排）收敛为函数内先验加正标签，闸序零重排。
2. 任务包件二原拟「授权矩阵两行」，实装定性为 α 只读具不入矩阵（承 alpha 五先例，矩阵只载写面），矩阵行数零变更；面计数申报改 stdio 19、HTTP 17。
3. 任务包件三「空 allow 即教学或拒」落为回执教学字段（不拒），read-only 会话合法空形保留。

## 六、链笔与提交

- intent 笔：3445bea2（2026-09-10 引擎链）。
- 认证笔：管线与结果档两笔（哈希见当日链）。
- 双仓 settle：commit 哈希见完工报告。

## 七、关联

- DEC-017 修订四（2026-08-28 常设纪律令源）与 DEC-017 修订三（ask3gate 错名事故——本批纪律的出生事故，与司梦 M1 同病）；DES-016（外部域中性化边界）；SPEC-023 修订一；司梦 M1/M3 报告；mcpboot-solo（bootstrap 先例与检词正登 bootstrap／域自举——本批闸的 established 过面首例）。
- **第二批候令**：M2 域感知四件（判据扫认新城形、rootanchor 域感知台账落点、close 回执 gates_skipped 计数、gauge 域件）——本批测试隔离事故即其必要性之活证。
