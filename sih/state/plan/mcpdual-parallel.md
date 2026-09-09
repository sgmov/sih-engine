# mcpdual-parallel 任务包

## 元信息（版本位必填）

- 批名：mcpdual-parallel
- version: v1
- 日期：2026-09-10
- 范式：并联 parallel——并行簇双子代理（甲＝本地 CLI 壳路径簇，乙＝MCP HTTP 路径簇）＋主线簇（stdio 面读矩阵亲跑＋收敛验收），编排者是主线
- 令源：用户 2026-09-10 令「司衡引擎的实装已经到位，你通过双子代理进行测试，本地开发环境的AGENTS和skills的配合与mcp的http是否一致」

## 一、问题陈述

- mcpline 线 alpha 相五读数与 beta 相写面俱已实装：stdio 面十五具挂 ZCode 会话（工作区 .zcode/config.json sih 条目），HTTP 面统一 web 服务 mcpline.web 8765 端口 /mcp 识别写面在役
- 两链路声称同一执法层（写面零新增执法、读面承接 CLI 只读子命令），但「本地 AGENTS+skills 壳配合」与「MCP HTTP 面」的行为一致性零同批三路对表实证
- 工作区 .zcode/config.json 另有 sihankor 条目（http://localhost:9741/mcp，enabled=false 旧注册）与在役 sih 条目并存，注册面实况未对表
- AGENTS.md「MCP Tool 调用义务」节声明「当前 ZCode 环境不接治理 MCP server」与实况（stdio 十五具在会话内）已陈旧，陈旧面如实呈报候裁

## 二、关键设计

- 三路对表：同参读矩阵七项在甲（CLI）、乙（HTTP JSON-RPC）、主线（stdio mcp__sih__）三簇实跑，判词两级——逐字节 cmp 与语义字段对表，结构性面差异（如 replay 字段、identity_notice、降只读投影）逐笔归因不混入不一致
- 乙簇加测三面：注册位面（tools/list 恰十五具、takeover/bypass/record_direct/lease_unclaim 结构性缺席）、识别三态（缺头 401 教学三因、无牌读降只读带 identity_notice、siinfer 活跃牌读所绑 InferServer 域对 CLI 同参）、缺日错误教学载荷
- 甲簇加测闸拒教学两件：会话外 scribe intent 与会话外 lease lock 的拒定载荷与退出码对表正典
- 零写入红线：三簇俱纯读加拒定探针，生产链零探针写；写面正写所需第一域活跃标识牌缺位属事实缺口，如实呈报候人节点裁，禁自签禁代行管理台

## 三、工作清单

- [ ] T-1 ask3 双门＋叩问＋正身＋立约（主线，已序）
- [ ] T-2 任务包落位＋锚首行改写（主线）
- [ ] T-3 甲簇读矩阵＋闸拒探针出簇档（子代理甲）
- [ ] T-4 乙簇读矩阵＋注册位＋识别三态出簇档（子代理乙）
- [ ] T-5 主线簇 stdio 读矩阵（主线）
- [ ] T-6 收敛对表判词＋结果档＋管线三步（主线）
- [ ] T-7 认证＋双仓 settle＋收约对账（主线）

## 四、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 | 测量 | 读矩阵七项三簇俱实跑俱档，读类出参对表：逐字节一致或语义字段一致，结构性面差异逐笔归因在档 |
| F2 | 测量 | chain_verify 三路同判 valid；critsweep 五判据三态三路一致 |
| F3 | 验收 | HTTP tools/list 恰十五具；takeover/bypass/record_direct/lease_unclaim 结构性缺席 |
| F4 | 验收 | 缺头写调用 401 教学载荷；无牌读降只读带 identity_notice；siinfer 牌域内读命中所绑域且零 identity_notice |
| F5 | 验收 | 甲簇闸拒两件退出码与载荷符合 BATCH-FACE 勘误正典形（缺 --sessions 闸拒教学） |
| F6 | 治理 | 探针期零写入：会话册与登记册零探针新行，双仓 git status 探针前后全等（批件写入面除外） |
| F7 | 治理 | settle 归并 close，reconcile 零新增，链 verify 全文 valid |

## 五、必读文件

- AGENTS.md（本地壳路径对表底本）
- sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md（alpha 面契约正典）
- sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md（写面安全模型正典）
- sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md（HTTP 识别写面正典）
- sih-tools/mcpline/README.md（面契约与工具清单）
- sih-tools/BATCH-FACE.md（批机械链命令面）

## 六、约束

- 引擎与工具代码零改动；中央登记册与 ZCode 配置零改动
- siinfer 牌所绑 InferServer 域只读：零写调用零会话建立
- 每条命令立即取退出码，失败即停整链，禁管道掩码
- 子代理产出属符号材料，入库前过主线验收；子代理不得代行签核类节点
- 锁冲突走 wait-turn 禁绕行禁 preempt

## 七、验收标准

- F1 至 F7 全过，三路对表判词表在结果档，不一致项（如有）逐笔如实呈报候裁
- 完工回报附判词表与五行锚回显

## 八、风险点

- HTTP 面读工具对第一域与有牌域的出参组装差异（identity_notice、降只读）须归因为面差异非不一致
- critsweep 的 replay 字段与 root_basis 等运行环境字段属结构性差异候选，归因须引正典条目
- 子代理环境缺 mcp 工具面：乙簇一律 curl JSON-RPC 直探，不依赖会话工具面

## 九、范式偏离声明

- 无偏离：并联形双子代理实跑并联簇，主线亲写最复杂件（收敛对表与结果档）

## 十、关联文件

- 温故检索档：sih-engine/sih/event/plan/mcpdual-parallel-materials/recall-topic-mcpline.md
- 线程序包：sih-engine/sih/state/plan/mcpline-line-v1.md
- 前验收先例：sih-engine/sih/event/plan/mcpcold-solo-results.md（冷 agent 零辅助验收）

## 十一、请求写入（收窄条款示范位）

sih-engine/sih/state/plan/mcpdual-parallel.md
sih-engine/sih/event/plan/mcpdual-parallel-results.md
sih-engine/sih/event/plan/mcpdual-parallel-materials/
（目录级声明理由：簇档与原始出参与检索档等新文件出生地，批量面）
.session-anchor.md
（根文件任务锚首行改写，无仓版控原地）
sih-tools/scribe/reports/
sih-tools/identity/reports/
sih-engine/sih/event/trail/2026-09-10.ndjson
（链文件只经引擎 scribe 写位）
