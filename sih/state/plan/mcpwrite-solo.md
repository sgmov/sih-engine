# mcpwrite-solo 批任务包：HTTP 写面实装——标识牌域绑定与签发台与域布局规约

> 令源：DES-015 修订二（防呆形，m-mcpauth-2 九发 stable_clear 终签 cf826465，用户裁定原文在档）；设计承裁定即验收，实装开工
> 形：solo 批独立立约独立收约，与主窗并行共用今日链走租约排队

## 一、使命

按 DES-015 修订二（唯一设计正典）实装 HTTP 写面六件，终态：外部项目以标识牌经 HTTP 面绑定自己的域，在域内走完整租约流程写自己的链，跨域结构性不可达。

## 二、实装六件

1. **tokens 明文登记册**：六字段（token_id 与 domain_root 与 scope 与 status 与 issued_at 与 issued_by）；落位第一域 sih-tools/mcpline/ledger/tokens.ndjson，新城正典位 <域根>/sih/ledger/tokens.ndjson；flock 加 O_APPEND 原子整行；停行即追加行末行为准
2. **签发台进视图**：mcpline.web 令牌管理台三动作（签发/列表/撤销），各带一步确认，动作即台账追加行，与面板静态页同栈同端口
3. **连接识别与域绑定**：Authorization Bearer 头带标识牌，查册得 domain_root 绑定本连接；写操作只落所绑域；无标识或停行牌降只读并出教学语
4. **域布局规约**：新城正典布局 <域根>/sih/event/trail/<date>.ndjson 加 <域根>/sih/ledger/*.ndjson 加 <域根>/sih/state/parking/；第一域经映射用司衡仓现存双仓布局不迁移；全部承接 CLI 调用（scribe query/verify、critsweep、gauge read、lease 子命令）按域路径组装，第一域映射与新城规约两形俱测
5. **HTTP 写面十具放行**：识别加绑定加 scope（readonly 拒写、域写透传）后经既有 CLI 执法透传，零新增执法；信封主体记项目标识
6. **stdio 零回归**：原十五具与五只读探针与 fixture 写径照旧

## 三、验收

- 测试六组：签发台端点（签发/列表/撤销与确认位）；识别绑定（牌对域一对一）；跨域拒（A 域牌触 B 域路径即拒）；HTTP 写径闭环（fixture 域根内 open 到 lock 到 intent 到 append 到 verify 到 close 全程真 CLI）；scope readonly 拒写；stdio 回归加零写证明（fixture 域之外双仓 git status 全等）
- 认证上链、双仓 settle、reconcile 双零

## 四、红线

1. DES-015 修订二唯一正典，缺口停批候裁禁自由发挥
2. 零新增执法零 LLM；既有工具代码零改动（scribe、lease、gauge、critsweep、nomenclator、basemgr）
3. 禁触 sih-visual/**、SPEC-023、.zcode 配置
4. 与主窗并行：锁面含 sih-engine/sih/event/trail/2026-09-09.ndjson，冲突走 lease wait-turn 禁绕行禁 preempt
5. 每条命令立即取退出码，失败即停，禁管道掩码

## 五、写入面（allow 清单）

- sih-tools/mcpline/**（含 tests 扩展与 ledger 目录）
- sih-engine/sih/state/plan/mcpwrite-solo.md 与 mcpwrite-solo-prompt.md
- sih-engine/sih/event/plan/mcpwrite-solo-results.md 与 mcpwrite-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-mcpwrite-1/

## 六、完工回报形

批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、六件各自终值、六组测试读数、跨域拒实录、第一域映射与新城规约两形读数、签发台动作实录。
