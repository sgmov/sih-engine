# mcpdual-parallel 批结果档：本地壳路径与 MCP HTTP 面三路一致性对表

> 批名：mcpdual-parallel（并联 parallel 批，双子代理并行簇＋主线簇，独立立约独立收约）
> 会话：a40d82e2618108ac（sess-zcode-260910-main-mcpdual，lease 1.40.0 签发，identity 报告 2026-09-10-mcpdual-parallel-identity.json，anomalies 空）
> 令源：用户 2026-09-10 令「司衡引擎的实装已经到位，你通过双子代理进行测试，本地开发环境的AGENTS和skills的配合与mcp的http是否一致」
> 对表正典：SPEC-023（alpha 读面契约）与 DES-014（写面安全模型）与 DES-015（HTTP 识别写面）与 sih-tools/mcpline/README.md 与 BATCH-FACE.md

## 一、使命与判词总览

三链路同参读矩阵七项对表：甲簇＝本地壳路径（AGENTS+skills 调用壳到 scribe 与 lease 与 critsweep 与 gauge CLI 本体）、乙簇＝MCP HTTP 路径（mcpline.web 127.0.0.1:8765/mcp streamable HTTP，curl JSON-RPC 直探）、主线簇＝stdio 面（ZCode 会话内 mcp__sih__ 十五具，工作区 .zcode/config.json sih 条目）。

判词：**两链路治理语义一致**——读矩阵语义级七处等价（R1 全日 169 笔事件（事件型，时戳）序列三路逐位一致；R2 过滤集 20 笔逐位一致；R3 verify 三路同判 valid 同首尾哈希 25989caf…8d5cd6e7；R4 critsweep 五判据三路同态俱 achieved；R5 秤星三维值等价 0.0/0.0/0.028571；R6 锁面读数同源等值；R7 缺日俱拒）。形态差异全数归因正典（投影形与错误信令形系 SPEC-023 载 MCP 面摘要投影与教学载荷契约所定）。发现跨路径不一致两件与 CLI 缺陷候选一件与契约实装差一件与文档陈旧两件，逐笔见第四节，俱候人节点裁。

## 二、读矩阵对表判词表

| 项 | 甲＝CLI | 乙＝HTTP | 主线＝stdio | 判词 |
|---|---|---|---|---|
| R1 全日链查询 | matches 169，退出码 0 | matches 169 returned 169，HTTP 200 | matches 169 returned 169 | 三路事件集（事件型，时戳）序列逐位一致（机械比对在档） |
| R2 事件型过滤 | `--event-type` 旗标，matches 20 | event_type 参，matches 169 returned 20 | 同 HTTP | 事件集逐位一致；matches 字段语义两形，见发现一 |
| R3 验链 | valid，169，25989caf…8d5cd6e7 | valid/true，同哈希 | valid/true，同哈希 | 三路同判同哈希；CLI 无 valid 布尔字段属投影形差异 |
| R4 判据扫 | 五判据俱 achieved，严格 JSON | 同态，events_scanned 1872 | 同态同值 | 语义等价；events_scanned 与 degraded 时点漂移归因批自身落链 |
| R5 秤星读数 | 显式单维历史形（裸 read 非法形退出码 2） | 三维摘要形 | 三维摘要形 | 值等价（0.0/0.0/0.028571，ga-2）；历史形对摘要形属投影形差异 |
| R6 锁面读数 | status 形 summary 计数形 | 列表投影形 | 列表投影形 | 同源等值（读数时点 7 持锁 1 会话一致；后时点漂移归因 G2 红证两行，见发现三） |
| R7 缺日 | 退出码 2，`{"error": "trail 不存在"}` 简式 | HTTP 200，四字段教学载荷 isError=false | 四字段教学载荷 | 教学载荷系 MCP 面契约（SPEC-023 报错即教学）；CLI 简式属底层原形；stdio 与 HTTP 的 isError 位差异观察在档（stdio 位会话内不可直观测，载荷同形） |

## 三、乙簇专项与闸拒探针判词

- 注册位面：HTTP tools/list 恰十五具（五读十写），takeover 与 bypass 与 record_direct 与 lease_unclaim 四具结构性缺席（名单级缺席非调用才报错形），与 README 矩阵及外部缺省分级一致；stdio 面同十五具。F3 过。
- 识别三态：I2 无牌写调用 HTTP 401 reason_code missing_authorization 教学体齐备，屏障在路由层先于参数校验（lease_open 缺参校验错误从未出现）即屏障前置实证；I4 siinfer 活跃牌域绑定生效，读正确指向 InferServer 域 trail 且零 identity_notice，与 CLI 同参交叉核对完全一致（域链 1 笔 ef37552f 判 valid）。F4 过（含发现四残项）。
- 闸拒探针：G1 会话外 scribe intent＝SessionNotActive 拒退出码 1 零留痕（trail 字节前后相等 6241）；主线补针 stdio 会话外 lease_lock＝session_not_bound 拒教学载荷退出码 1 零写入；G2 会话外 lease lock＝不被拒（发现三）。F5 部分过（G2 即发现）。
- 传输形态：stateless（无 Mcp-Session-Id），全程 application/json 未遇 SSE，serverInfo mcpline-http 1.30.0。

## 四、发现清单（俱候人节点裁）

- 发现一（跨路径不一致）：chain_query 的 matches 字段在 CLI 形下载过滤后命中数（20），在 MCP 双面载全日笔数（169）不随过滤变；MCP 双面彼此一致。语义零损（returned 与事件集三路一致），字段语义两形候裁：统一或文档注明。
- 发现二（文档陈旧）：AGENTS.md「MCP Tool 调用义务」节「当前 ZCode 环境不接治理 MCP server」与实况不符（stdio 十五具在会话内在役）；同档 gauge read「三维全出」调用形描述与实态不符（三维缺省全出仅 record 成立，read 须显式单维加五参）。
- 发现三（CLI 缺陷候选，红证留痕）：无会话号 lease lock 撞 identity 同源已持锁路径不被拒，退出码 0 duplicate:true，且每调用向 locks.ndjson 追加一行冗余 acquired（本批致两行：8174 与 8175 行，先红留痕在册不清洗）；现势核算随放锁自愈（一次 unlock 成对释放该路径三行，held 9 还 6），永久效应即账内两行历史噪声。CLI 内部闸覆盖亦不齐：intent 有会话闸（G1 拒）而 lock 无。候裁：补闸或承认 duplicate 幂等形并去重追加。
- 发现四（契约实装差）：无牌（缺 Authorization 头）读连接静默降只读（出参零 identity_notice），携牌不在册读才带教学语；README 载「无标识或停行牌连接读工具出参附 identity_notice」。二取一候裁：实现补 notice 或文档收窄。伪牌读 HTTP 200 降只读与 initialize instructions 自述相容（401 只拦写调用），本批简报预期 401 属预期误校，实现合规，首跑读数留痕。
- 发现五（注册面陈旧项）：工作区 .zcode/config.json 并存 sih（stdio 在役）与 sihankor（http://localhost:9741/mcp，enabled=false 旧注册）两条目；旧条目退役态如实呈报，是否清除候裁。
- 事实缺口（候裁）：第一域（本工作区）无活跃标识牌，HTTP 写面正写探针无牌可用；管理台签发属人节点操作位，本批禁自签，只如实申报。写面正写一致性的活体验证在牌签发后可继（fixture 闭环已由 mcpline 套件 test_http_write_loop 承载）。

## 五、F 锚定验收表

| F | 判据 | 实态 |
|---|---|---|
| F1 | 读矩阵七项三簇俱实跑俱档，出参对表 | 过（三簇 summary 与原始出参全在 materials；机械比对判词见第二节） |
| F2 | verify 三路同判；critsweep 三路同态 | 过 |
| F3 | HTTP 恰十五具，四具结构性缺席 | 过 |
| F4 | 401 教学载荷；降只读；有牌域内读 | 过（发现四残项候裁不拦本判据：401 与域绑定与 CLI 交叉一致俱实证） |
| F5 | 闸拒退出码与载荷合正典形 | 部分过（G1 与 stdio 补针合正典；G2 即发现三红证） |
| F6 | 探针期零写入 | 部分过（G2 两行冗余即违约面本身，红证留痕如实申报；其余探针零写入，InferServer 域只读零触碰，登记册与会话册零新行） |
| F7 | settle 归并 close，reconcile 零新增，链 verify valid | 过（engine settle 22670d7 与 closeguard 546f2fd 与归并 4ee07b4，tools 零改动免 settle；reconcile 双仓 unrouted 与 cert_missing 双零，unbypassed 92 与 129 为既有存量类如实转述非本批新增；当日链 verify valid 10 笔尾哈希 3de607cb 即本批认证笔；收约后锁面会话双清零净态） |

## 六、队形验证

并联形双子代理实跑并联簇：甲乙两子代理后台各领一链路簇（甲 35 次工具调用 525 秒，乙 33 次 613 秒），主线亲写最复杂件（stdio 簇读数与三路机械对表与本档），收敛验收归主线。形名相符零偏离。

## 七、必读与材料位

- 簇档：materials/cluster-local-a/（summary 与 r1 至 r7 与 g1 与 g2 全件）与 materials/cluster-http-b/（summary 与 21 件原始响应与 99-commands.txt）与 materials/cluster-stdio-main/summary.json
- 温故检索：materials/recall-topic-mcpline.md（61 行先例，零命中项无）
- 红证：locks.ndjson 8174 与 8175 两行（不清洗，候裁后处置）；甲簇 g2.txt 全出参

## 八、结算读数（收约补笔回填）

- 双仓 settle：engine 工地提交 22670d7（段1 settle，cert 3de607cb），closeguard 预收 546f2fd，归并 4ee07b4；tools 工地零改动（零工具改动批）免 settle 如实申报。
- 收约：close 三闸全过（calllog_gate unreleased 0、chain_gate checked trails 20、declaration_gate clean），双仓 worktree 拆除与分支删除，收据在 sih-tools/lease/ledger/receipts/mcpdual-parallel.json。
- 对账：reconcile engine 与 tools 退出码一，成因即 unbypassed 既有存量类（92 与 129），本批两判据 unrouted 与 cert_missing 俱零新增。
- 链面：2026-09-10 链 verify valid 10 笔，first b6c7e1c7 尾 3de607cb（本批认证笔）。
- 红证处置候裁：locks.ndjson 8174 与 8175 两行冗余 acquired 留痕在册（发现三），本批不清洗。
- 补笔后管线复跑见直改链笔申报。

## 九、候裁处置后记（2026-09-10 用户令「待裁过得一，裁一过一执行一，未过入泊」）

- HTTP 写能力确认（用户指位隔壁域）：InferServer 域内落盘证据坐实——会话 d04e81e7e4c265dd 全生命周期（2026-09-09T23:20 issued 至 2026-09-10T00:06 revoked），域链 2026-09-10 四笔（意图一加认证三）与 2026-09-09 一笔俱 verify valid，包材料五件在 sih/state/plan/；HTTP 面携 siinfer 活跃牌读回今日域链逐笔同哈希且零 identity_notice。判词：识别写面写路径与域锚定实证成立，第一域不再另添探针笔。
- 过一执行一：发现二之 AGENTS.md MCP 节陈旧已修（实况改写含三正典指针与一致性判词指针）；发现五之 .zcode/config.json 旧 sihankor 条目（http://localhost:9741，enabled=false）已清除，mcp.servers 现役仅 sih。
- 未过入泊：发现一与发现三与发现四并 heartbeat 描述命令形漂移入泊 pk-083（sih-tools/parking/materials/pk-083.json，停泊事件 1093cbe7，ttl 30 天，名录行在 sih-tools/PARKING-v1.md），出泊条件即用户裁修复形态。
- 缺口消项：第一域活跃标识牌按需经管理台（:8765/tokens）人节点自签即开，HTTP 写确认已由第二域承载，不再候裁。
- 本节与 pk-083 件与泊界名录行走直改链笔声明（no-session 主会处置位）。
