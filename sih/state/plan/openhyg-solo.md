# openhyg-solo:开约面卫生批（解析联动 + 检验文件生命周期 + PID 探针）

> 令源：用户 2026-09-05「先修租约」令 + 2026-09-05 四条修复计划 userselect 点头 + 2026-09-05 设计对话三点裁定（任务包目录登记表联动 / 先验票后落盘且正身与时间戳出生即全 / PID 探针补「同正身误跑误删活窗」洞）
> 事故证据：docmath-namefit-solo 代理 2026-09-05T15:08Z 开约受阻——裸 stem 解析只搜 sih-engine 计划面，包在 sih-math/sih/event/plan/ 解析失败，检验文件已落盘（session_id null 残件）堵重试；constmodel-solo 首跑同类缺陷先例（结果档越线申报节）
> 范式：T6 单线 solo，委外代理亲写零子代理
> 编排定位：用户令本批先行；watchcheck-solo 与 idenlane-guard-solo（批 C）同候本批收约（lease CONTRACT 单写面）；idenlane-human-solo（批 B）已于 2026-09-05T16:06Z 收约完毕不在途

## 一、问题陈述 {#problem}

- **缺陷一（根因·解析面不对称）**：resolve_package 裸 stem 只搜 sih-engine/sih/state/plan/ 单目录；本工具自持 --repo 多仓而任务包合法落位跨仓（math 侧 sih-math/sih/event/plan/ 在账先例 constmodel-solo）。相对路径另有一病：不拼 root 按 cwd 解析（constmodel 首跑被拒实录）
- **缺陷二（次序错）**：检验钥匙闸（副作用：落盘检验文件）先于包存在性校验（core.py open_session 首行 must_exist 才拒），任何后续失败必然留残件
- **缺陷三（失败不清理）**：open 失败无人删自家检验文件；残件 session_id null 即「从未签发」的机械证据，工具不读
- **缺陷四（升级不对称）**：残件先以心跳新鲜堵重试三百秒，再升级「人节点显式接管」，且接管在心跳新鲜期同拒——死文件时间戳被当活进程存活信号；工具自伤噪声烧人类注意力，与工程基线第三条相抵
- **设计裁定补洞**：session_id null ≠ 死亡——签发前的活窗同为 null 形，时间戳单判会误删活窗（用户 2026-09-05 问三）；生死判据须换进程探针

## 二、关键设计 {#design}

### 2.1 任务包目录登记表 + 按表解析（缺陷一修复）

- 新冻结架构常量 `TASK_PACKAGE_DIRS`（core.py，学 SCOPE_SHARED_SURFACE 冻结登记先例），起步只收三目录、宁窄勿宽：
  1. `sih-engine/sih/state/plan/`（引擎侧批现行位）
  2. `sih-engine/task-packages/`（遗留兼容位，历史包路径在账）
  3. `sih-math/sih/event/plan/`（math 侧批现行位）
- 解析语义：裸 stem 按表逐目录搜——唯一命中即用；零命中报错列全搜索面；多命中报歧义列全命中径，拒开不猜
- 相对路径（含 `/` 但非绝对）：一律按 root 拼接后校验，不按 cwd（constmodel 缺陷收编）
- 登记表扩面走 CONTRACT 修订，不裸奔

### 2.2 先验票后落盘、出生即全须全尾（缺陷二修复 + 用户裁定序）

open 闸序重排为：

1. 包校验（按表解析 + must_exist + parse_requested_writes）
2. 正身读取（load_identity，只读）
3. 意图校验（validate_intent，只读）
4. 同包活跃会话查（PackageSessionActive 前移，只读台账扫描）
5. 开工预检闸（precheck，只读探测）
6. **检验钥匙闸（首个副作用位）**：检验文件一次性落全形——`{package, opened_at, heartbeat_at, identity_core, pid, session_id: null}`，正身与时间戳出生即在，无匿名窗；文件创建原子性（O_EXCL 竞败即拒）与写后回读校验照旧
7. 签发会话入台账，回填 session_id（唯一保留的回填字段——会话号只能签发时刻生成）

要求：第六位之后不得再有任何「本可在前五位验掉」的失败源。

### 2.3 失败自收桌（缺陷三修复）

- 检验文件落盘后、会话签发成功前，任一失败路径（WorktreeError、git 错、台账写败等）退出前删自家检验文件
- 删除前所有权核验：opened_at 与 pid 与己方一致才删，防误删他窗（并发竞败场景文件已被他窗覆盖）
- 清理失败不掩原错：两错并报
- 会话签发成功后的失败不删文件（那是真窗口，归心跳与接管管）

### 2.4 PID 探针判生死（缺陷四修复 + 用户裁定的误跑洞）

- 写侧：检验文件加 `pid` 字段（os.getpid()），辅 `pid_started`（进程启动时刻，可取则取；平台取不到记 null 如实申报）
- 读侧（open 闸与 takeover 同判据升级）：
  - PID 探活（POSIX kill(pid,0)，PermissionError 视为活）且启动时刻对表一致 → **活窗**：照旧拒开（window_active / 接管拒 window_active）——同正身误跑的双开保护红证位
  - PID 不存在（或启动时刻对表不符，跨启动复用对冲）→ **尸体**：open 闸机械自清（删残件、lock_event 记 `stale_cleared` 事件只 INSERT、留原文件快照于 detail）后正常续开；takeover 同判据可清
  - 无 pid 字段的旧形检验文件：回落心跳停滞判据（旧行为），detail 如实申报，零迁移（checks 面本就随收约清空）
- 心跳字段保留为审计与回落判据；正身只作归属记录，不作清除授权（用户裁定：活没活由进程说了算）
- 判定语义变更点：停滞残件处置由「拒开索人接管」改为「PID 死机械自清留痕」——过得一裁（facet 合同模式，near_threshold 呈用户）

## 三、工作清单 {#work}

- [x] T-1 TASK_PACKAGE_DIRS 注册与 resolve_package 按表解析（含相对路径 root 拼接修复、歧义与零命中报文）
- [x] T-2 open 闸序重排：前五位只读化，检验文件末位落盘出生全形，签发后仅回填 session_id
- [x] T-3 失败自收桌：签发前失败路径删自家文件，所有权核验，两错并报
- [x] T-4 PID 探针：写侧 pid/pid_started、读侧三态判、自清留痕（lockdb 加 stale_cleared 只 INSERT 形）、takeover 判据升级
- [x] T-5 旧形回落与 detail 如实申报
- [x] T-6 TDD 四族先红后绿（解析族 / 生命周期族 / PID 族 / 回归族）
- [x] T-7 判定语义一裁（facet 合同模式九发，near_threshold 呈用户转主会）
- [x] T-8 BATCH-FACE 坑位注记（stem 按表解析新语义与 math 侧包位）+ CONTRACT 修订 + CALL-LOG 双笔 + 双仓收约链

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 登记表解析 | math 侧 stem 命中即开；歧义报双径拒；零命中报文列全搜索面；相对路径按 root 拼接（constmodel 缺陷红证收编） |
| **F-2** | 检验文件生命周期 | 出生全形（identity_core 与 opened_at 与 pid 落盘即有）；前五位任一失败零残留；签发前失败自收桌；所有权核验夹具在档 |
| **F-3** | PID 判生死 | 误跑红证：活 PID 二开拒；死 PID 自清留 stale_cleared 事件并放行；活窗慢签发夹具（PID 活心跳停滞超阈）证不误删；无 pid 旧形回落心跳 |
| **F-4** | 判定语义一裁 | 自清语义 facet 合同模式 stable_clear 过执契，near_threshold 呈用户 |
| **F-5** | 零回归 | 全测试族全绿（现基线 111 件 + 批 B 收约后实数）；sessions 台账行格式零变更 |

## 五、必读文件 {#read}

- 现行源位：sih-tools/lease/src/lease/cli.py（_check_key_gate 与 _cmd_open 与 _cmd_takeover）、core.py（resolve_package 与 open_session）、lockdb.py（事件写面）
- 契约：sih-tools/lease/CONTRACT.md 修订三十五至三十七（补录通道、写点唯一化、--source 旗标）
- 事故实录：sih-tools/lease/CALL-LOG.md 2026-09-05 各行（constmodel 补录与遗留申报）
- 机械链：sih-tools/BATCH-FACE.md（含全部勘误节）
- 一裁工具链：sih-tools/facet/（合同模式）与 sih-tools/proposition/DES/ 先例

## 六、约束 {#constraints}

1. 判定语义变更（自清）过得一裁；near_threshold 呈用户转主会，不自行终签
2. sessions 台账 issued 与 revoked 行格式零变更零迁移；ledgerwrite 写点唯一化机制零动
3. TASK_PACKAGE_DIRS 为冻结架构常量，起步三目录宁窄勿宽，扩面走 CONTRACT 修订
4. v3 正身体系与 identity_hash 与 core_hash 零动
5. TDD 先红后绿；全测试族零回归
6. 编排串行：watchcheck-solo 与批 C 候本批收约后开工（lease CONTRACT 单写面）；预检拦即排队候位（gateswitch 先例），不抢不绕
7. scribe 写入裸调逐笔 grep 验证，禁 meter 包裹掩败；des-001 域只盖 sih-engine/doc，域外 exit-2 如实记
8. 引擎件与工具件合并后主树重编再验收

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 活体验收：docmath-namefit-solo 以裸 stem 按新语义重开成功（本批事故正主）
- [ ] 双仓 settle + close + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/openhyg-solo-results.md 与 materials/
- [ ] CONTRACT 修订与 CALL-LOG 双笔随批

## 八、风险点 {#risks}

- PID 探针平台语义：macOS 与 Linux kill(pid,0) 返回差异、PermissionError 判活、PID 跨启动复用——pid_started 启动时刻对表对冲，取不到回落心跳并如实申报
- 自清与活窗误判边界：活窗慢签发夹具（PID 活、心跳停滞超阈）必须先红后绿，证不误删
- watchcheck 若已开工在途：预检拦即候位，CONTRACT 冲突零容忍
- 登记表多目录同名包歧义：现状三目录天然不相交，测试构造歧义夹具验证报文即可，不引入猜测语义

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。本批为开约面卫生独立修复批，不属 leaseopt 线（该线已结算）。

## 十、关联文件 {#related}

- 上游事故：docmath-namefit-solo 开约受阻（2026-09-05）与 constmodel-solo 首跑越线申报
- 平行批：watchcheck-solo（候本批收约）、idenlane-human-solo（已收约 2026-09-05T16:06Z）
- 下游批：idenlane-guard-solo（批 C，候本批收约）
- 泊件关联：pk-057（写点唯一化，本批零动其机制）

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/（core.py、cli.py、lockdb.py）
- sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md（修订）与 CALL-LOG.md、sih-tools/scribe/CALL-LOG.md
- sih-tools/BATCH-FACE.md（坑位注记节）
- sih-engine/sih/state/plan/openhyg-solo-results.md 与 materials/
- sih-engine/sih/event/trail/2026-09-06.ndjson 及后续日链
