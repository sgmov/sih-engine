# 提案：正身（identity）模块哈希漂移修复 —— core-v1 双哈希方案

**状态声明**：本文是决策材料，不是决策。方案采纳与实装须另行走司衡治理流程（得一裁 + 人节点）。本文只把问题、选项、证据与代价摆清楚，不预设采纳。

**任务书**：`sih-engine/sih/event/plan/pk041-solo-materials/identity-research-brief.md`（Q1–Q5 调研）。
**调研与原型执行**：只读调研 + 只读实验采样（`identity verify` 与真实采集器），零源码修改、零 git 状态改动。
**日期**：2026-09-03。

---

## 0. 摘要

正身工具 v3 身份串把**三个不同安全层**（身份核 / 会话绑定 / 防重放新鲜度）熔进同一个盐哈希。其中 pid、ppid、parent_start、net_time、timestamp 五个易变件加每次调用的随机盐，使同一操作者、同一机器、同一 harness、同一天内每次调用哈希必不同。实测 2026-09-02 一日四哈希（58e22070 → 14cfa9c2 → 9634b1de → d9fd7386），tally R5"席位当日基线"的哈希优先配对（`sih-tools/tally/src/tally/cli.py:148-160`，完整盐哈希逐字节等值）因此对新报告永远配不上，每次都落漂移挂起、靠重标定兜底。

本提案把身份拆成两层哈希：

- **core_hash（新增）**：只对稳定身份核（mac、hostname、user、boottime、sandbox_id、归一化血统 token）做确定性哈希（无每次随机盐），**同席同日稳定可连认**，供 R5 配对与审计连认。
- **full_hash（保留不动）**：现有 v3 完整盐哈希一字不改，继续承担逐报告取证 / 防篡改 / 防重放。存量 39 份报告复算逐字节一致（§5.2 T6），**对链上已存身份引用零破坏**。

原型已验证：16 项静态测试全过；8 组活体采样中 full 哈希 8/8 互异（复现漂移），core_hash 同席同值 `82f460c2ac2741fc…`，且与当日四份历史漂移报告的 core_hash 完全一致——历史失败场景在新方案下被机械认定为同一席位。

---

## 1. 背景与问题

### 1.1 现象

正身（`sih-tools/identity`）每次治理批开工产一份身份报告：十二个环境组件拼成 `v3|k=v|…` 身份串，哈希 = SHA-256(盐 + "|" + 身份串)，盐缺省每次随机 32 字节。租约系统用该哈希绑定"本次写入是谁在哪个会话里做的"。

2026-09-02 一日四批，四个不同哈希（SPEC-016 附录 § 身份哈希漂移观察 已立观察）：

| 批次 | 时间(UTC) | 哈希前缀 | 报告件 |
|---|---|---|---|
| predsplitAB | 03:35:17 | 58e22070 | `sih-tools/identity/reports/2026-09-02-predab-identity.json` |
| autoflow2 | 04:40:09 | 14cfa9c2 | `sih-tools/identity/reports/2026-09-02-autoflow2-identity.json` |
| predspec2 | 06:33:15 | 9634b1de | `sih-tools/identity/reports/2026-09-02-predsp2-identity.json` |
| sweepclea6 | 07:34:56 | d9fd7386 | `sih-tools/identity/reports/2026-09-02-sweep6-identity.json` |

每次漂移触发"重标定"（20 发探针重建当日基线）。流程扛住了，但同一逻辑身份一天被识别成四个"人"，"今天的批和上午的批是否同一执行者"需人工对照。

### 1.2 消费面（谁在用这个哈希）

| 消费方 | 位置 | 用法 | 状态 |
|---|---|---|---|
| tally R5 席位当日基线 | `sih-tools/tally/src/tally/cli.py:134-169` | 材料 `identity_hash` 与基线文件 `identity_hash` **逐字节等值**配对；不一致即 `r5_state="suspend"`（漂移挂起） | **坏**：新报告永远配不上 |
| 席位基线文件 | 例 `sih-tools/proposition/DES/m-mathrelease/seat-baseline-zcode-2026-09-02.json` | 载当日 `identity_hash` 与稳定席位标签 `"seat": "ZCode:GLM-5.3-Flash:self-reported"` | 稳定标签已有，配对却不用它 |
| 租约悲观锁五验 | `sih-tools/lease/src/lease/lockcore.py:13,146-168` | `BINDING_KEYS=("hostname","user","boottime")` 三稳定键与 `.bindings/{session}.json` 侧档逐键等值，失配报 `{key}_drift` 拦截 | **没坏**：三键同席同日稳定 |
| 租约会话号 | `sih-tools/lease/src/lease/core.py:257-260,379-381` | `session_id = SHA-256(包名\|签发时刻\|报告文件字节哈希\|仓序列)[:16]` | 每会话唯一，本非连认键，无妨 |
| 会话台账 | `sih-tools/lease/ledger/sessions.ndjson`（198 条 issued） | 各载 `identity.{identity_hash, file_sha256}` 作取证引用 | 历史固化，须保持可解读 |

结论：**真正的连认路径（租约三键绑定）没坏；坏的是 R5 拿必然易变的盐哈希做逐字节等值配对。**

---

## 2. 根因分析

### 2.1 逐组件分类（源码 + 四份漂移报告逐组件 diff 实测）

| 组件 | 采集源（core.py） | 稳定类 | 同席同日实测 | 安全目标 | 归属层 |
|---|---|---|---|---|---|
| `pid` | `os.getpid()` | 易变 | 5361→30831→74569→93260 | 实例标记/防重放 | 新鲜度层（搭车） |
| `ppid` | `os.getppid()` | 易变 | 5360→30830→74568→93259 | 实例标记 | 新鲜度层（搭车） |
| `parent_start` | `ps -p <ppid> -o lstart=` | 易变 | 03:35→04:40→06:33→07:34 | 防 pid 复用冒充 | 新鲜度层（搭车） |
| `net_time` | HTTP `cloudflare/cdn-cgi/trace` | 易变 | 1788320118→…→1788334498 | 外部权威钟对照本地时钟 | 新鲜度/异常信号（搭车，且网络依赖） |
| `timestamp` | `datetime.now(utc)` | 易变 | 每次调用变 | 本条观察的新鲜度 | 新鲜度层（搭车） |
| `hostname` | `socket.gethostname()` | 稳定 | MiniServer 恒定 | 防冒充：哪台机器 | **身份核** |
| `mac` | `uuid.getnode()`（组播位判缺席） | 稳定（云 VM 随机化时降格） | 1cf64c65312e 恒定 | 防冒充：哪块物理网卡 | **身份核** |
| `user` | `getpass.getuser()` | 稳定 | moc 恒定 | 防冒充：哪个操作者 | **身份核** |
| `boottime` | `sysctl kern.boottime` | 稳定（重启变） | 1787917459 恒定 | 哪个开机纪元 | **身份核**（重启=新席位纪元，语义与租约 `boottime_drift` 拦截一致） |
| `sandbox_id` | 环境变量（当前恒空） | 稳定（声明式） | 空 | 防冒充：哪个沙箱 | **身份核**（在场时） |
| `session_id` | 环境变量（当前恒空） | 稳定（声明式） | 空 | 会话绑定 | **会话面**（本应是连认主键） |
| `ancestry` | 上溯父链进程名（深度封顶 32） | 半稳定 | ZCode 链内恒定；跨 harness 变；host-local-1/-2 亦变 | 跨 harness 血统区分（原始意图） | 全链进报告（取证）；**归一化 token 进身份核** |

血统脆性实证（全量 40 份报告去重）：`zcode-host-local-1`（29 份）、`MiniMax Code`（5）、`TRAE SOLO CN`（4）、`zcode-host-local-2`（1）、空（2）——**同一 ZCode harness 内 host-local-1 与 -2 已分叉**。

### 2.2 根因判断

十二组件集合是"防冒充"与"可连认"两个目标的**过度折衷**——准确说是把三层安全目标熔进一个哈希：

1. **身份核（who/what）**：mac、hostname、user、boottime、sandbox_id、归一化血统。慢变、难冒充、同席同日可复现。
2. **会话绑定（which seat/day/instance）**：session_id、ancestry 全链、ppid/parent_start。
3. **新鲜度/防重放（this observation now）**：pid、timestamp、net_time、parent_start。

业界签名语句（JWT、TCG quote、in-toto）里时间戳/nonce 是**进签名域做防重放**的，不是**进身份做连认**的。当前设计把新鲜度分量当成了身份分量，是类别错误。随机盐再叠加一层"生产路径必不同"。

### 2.3 失败复现（原型实测）

- 固定同一 salt 隔 1.2s 连跑两次 `identity verify`：`35da9b8e…` vs `8d22a487…` → **去掉盐变量，易变件仍独立致漂**。
- 同一稳定核只改五个易变件 → 哈希变（`aae0dc1c…`）→ **五易变件（不含盐）足以打破连认**。
- 四份历史漂移报告按 full 哈希两两配对：0/3 成功，全落漂移挂起 → **与 SPEC-016 附录"复用对表不过即按先例重标定"记录一致**。

---

## 3. 业界对照与分层原则（调研摘要）

各系统对"什么进身份哈希、什么进会话元数据"的处理：

- **(a) SSH 证书与短期凭据**：身份 = CA 背书的稳定公钥；有效期窗/serial/nonce 是证书字段与短期证明。身份稳定可连认，凭据短命防重放，二者语义分离。
- **(b) TEE/attestation 测量寄存器**：SGX 的 MRENCLAVE/MRSIGNER（代码+配置+签名者度量）是稳定身份；REPORT_DATA（调用方 nonce，会话绑定）是报告里的并列字段，不在度量里。度量=身份，nonce=会话/新鲜度，绝不相熔。
- **(c) Web 设备指纹 + session token**：指纹是**有意的粗粒度概率性**信号，业界用打分/阈值容忍漂移，从不用精确等值连认；精确等值只留给稳定核心。SiHankor 现状恰是反模式：拿不稳定信号做精确等值。
- **(d) SLSA provenance**：in-toto 语句签名覆盖全句，但句内 `builder.id`（稳定身份 URI）、`subject`（产物摘要）、时间戳/来源（元数据）是**并列具名字段**——身份是字段，不是"把所有东西哈希成一个值"。
- **(e) 多 agent 系统**（LangGraph、AutoGen、CrewAI 一类）：agent 是谁 = 逻辑名/角色/配置（稳定）；哪次运行 = thread_id/run_id；逐动作新鲜度 = 每动作 nonce/时间戳。**没有任何主流实现把 PID/进程树/时间戳放进身份哈希。**

（以上按既有知识陈述，未能联网二次核对规范细节处已在调研中如实标注；分层原则本身不依赖个别细节。）

**可借鉴分层原则**：身份（稳定核，连认只认它）/ 会话绑定（中速变，进报告与次级哈希）/ 新鲜度防重放（高速变，只进签名域，不作连认键）——三层各归其位。

---

## 4. 方案设计（core-v1 双哈希 + 会话面 + 归一化血统）

### 4.1 组件分层定稿（提案）

| 层 | 组件 | 去处 |
|---|---|---|
| 身份核（进 core_hash） | `mac, hostname, user, boottime, sandbox_id, lineage`（归一化血统 token） | 主连认键，同席同日稳定 |
| 会话面（进报告，不进 core_hash） | `pid, ppid, parent_start, ancestry 全链, session_id` | 取证、会话绑定；session_id 待 harness 注入后升为声明式连认佐证 |
| 新鲜度/异常信号（不进任何连认哈希） | `timestamp, net_time` | timestamp 留在 v3 身份串与 full_hash（防重放语义不变）；net_time 移出哈希，HTTP 探针降级为**可选**异常检测源（缺席=空串+异常标记，不阻断） |

### 4.2 core-v1 定义（已原型验证）

```
core_string = "core-v1|" + mac=…|hostname=…|user=…|boottime=…|sandbox_id=…|lineage=…
core_hash   = SHA-256(core_string)          # 确定性，无每次随机盐，仅域分隔前缀
```

- 键序固定；缺席即空串（承现有"缺席不判败"语义）。
- 确定性派生是连认成立的前提：同席同日同输入必同值，且 `--inject` 全核键 + 同参数双跑逐字节一致（已实测）。
- 现役席位实测值：`core-v1|mac=1cf64c65312e|hostname=MiniServer|user=moc|boottime=1787917459|sandbox_id=|lineage=zcode-cli>zcode-host-local>ZCode` → `82f460c2ac2741fc4ee8d104d7ef14803f744da14a24b1d7356bd3bd3531d1dc`。

**lineage 归一化 token（候选规则，待裁定）**：ancestry 按 `>` 分段，剔除通用包装段 {python3, uv, zsh, bash, sh, launchd, agent-tool-host}，`zcode-host-local-N` 归一为 `zcode-host-local`，余段以 `>` 连接。实测：ZCode → `zcode-cli>zcode-host-local>ZCode`；MiniMax → `MiniMax Code Helper>MiniMax Code`；TRAE → `TRAE SOLO CN Helper>Electron`。保留跨 harness 血统区分的原始意图，剔除嵌套深度与 host 实例号脆性。

### 4.3 full_hash 保留不动

v3 身份串与盐哈希**一字不改**：继续承担逐报告防篡改（盐使每条报告哈希唯一，报告被改动即哈希不配）、防重放（timestamp/net_time 在签名域内）、存量兼容。报告新增 `identity.core_hash` 与 `identity.core_components`（列出参与核的键值，供审计复核）两个字段，纯增量。

### 4.4 net_time 处置

**移出任何连认哈希**；其安全价值（外部权威钟对照本地时钟、防时钟回拨冒充）保留为**异常信号**（现有 `detect_anomalies` 的 `clock_skew` 逻辑不动）；HTTP 探针改为**可选**——离线/沙箱环境缺席即空串加异常标记，不阻断 core_hash 产出。这满足零网络硬约束：core_hash 计算纯本地。

### 4.5 声明式字段（方案 C 补强，排期待定）

harness 侧注入 `SIH_SESSION_ID`（会话面连认主键）与席位声明（如 `ZCode:GLM-5.3-Flash`，席位基线文件已有此标签可对齐）。注入前行为不变（空串不判败）；注入后连认从"观测推导"升为"声明优先 + 观测佐证"。原型已验证：设 `SIH_SESSION_ID` 不扰动 core_hash（会话面声明），设 `SIH_SANDBOX_ID` 改变 core_hash（核声明，设计内）。

---

## 5. 原型测试结果（证据）

**方法**：原型以 heredoc 内存执行（零文件落盘、零源码修改），只读导入真实采集器 `identity.core`，只读使用既有 40 份报告；活体采样走真实 `collect_components`（含真实外部钟探针）。

### 5.1 静态测试（16 项，16/16 PASS）

| 项 | 内容 | 结果 |
|---|---|---|
| T6 | 向后兼容：39 份存量报告按 `盐\|v3身份串` 用工具自带函数复算 | 逐字节一致，0 不符（2 份无哈希空件除外） |
| T7a | 旧 R5 复现：四份漂移报告 full 哈希两两配对 | 0/3 成功，全落漂移挂起（复现历史失败） |
| T7b | 新 R5：同四份报告算 core_hash | 4 个完全一致 `82f460c2ac27…` |
| T1 | 同输入双跑 | 逐字节一致（复演确定性） |
| T3 | 五易变件全改 | core_hash 不变（易变件彻底隔离） |
| T4a–e | mac / hostname / user / sandbox_id / 血统 token 单点篡改 | core_hash 各变（防冒充保留，含跨 harness 冒充） |
| T8 | boottime+1s（模拟重启） | core_hash 变（重启=换纪元，与租约拦截语义一致） |
| T9 | net_time 置空（离线） | core_hash 不变（零网络依赖） |
| T5a–d | 血统归一化 | host-local-1/-2 同 token 同 core；三 harness 两两不同；空 ancestry 出空 token 不判败 |

### 5.2 活体采样（8 组，真实采集）

| 变体 | pid | sess面 | 沙箱面 | full(随机盐) | full(固定盐) | core |
|---|---|---|---|---|---|---|
| A1–A5 直连（间隔 1s×5） | 58874 同 | — | — | 5 值互异 | 5 值互异 | 82f460c2 ×5 |
| B 多一层 bash 嵌套 | 58997 | — | — | 异 | 异 | 82f460c2 |
| C +SIH_SESSION_ID=live-test | 59012 | live-test | — | 异 | 异 | 82f460c2 |
| D +SIH_SANDBOX_ID=sbx-x | 59030 | — | sbx-x | 异 | 异 | 8d4ec258（按设计变） |

关键读数：

1. A1–A5 的 pid/ppid 相同而 full 固定盐仍 5 值互异 → 时间戳+外部钟两件即足以独立致漂，根因坐实。
2. B 的父链实测 `python3>python3>zsh>zcode-cli>…`（嵌套深度 +1），A 为 `python3>zsh>zcode-cli>…` → **嵌套层数变了，core_hash 不变**。
3. **现役席位活体 core_hash（82f460c2ac2741fc…）与四份历史漂移报告的 core_hash 完全一致** → "同一逻辑身份一天四个哈希"的失败场景，在新方案下被机械认定为同一席位，且跨当日全时段（03:35–07:34 历史件与 09-03 活体件）连认成立。
4. D 的变化是设计内行为：sandbox_id 属身份核，沙箱声明改变即身份改变。

### 5.3 如实声明的残余

- A1–A5 同进程连跑（pid 天然相同）；pid 易变性由 B/C/D 子进程 + 静态 T3 + 四份历史报告（pid 5361→93260）覆盖，未单独做"纯 pid 变化"活体组。
- 归一化规则与 boottime 进核与否为**候选**，本次按候选规则验证可行性，不构成裁定（§9）。
- 原型只验证哈希层性质；R5/租约侧代码改动、退出码与零写路径约束须等实装批在围堰内验证。

---

## 6. 与司衡约束逐条自查（首选方案）

| 硬约束 | 怎么过 |
|---|---|
| 零 LLM 零网络依赖 | core_hash 纯本地计算；net_time 移出哈希、HTTP 探针降级为可选异常源，离线环境零网络可出 core_hash |
| 复演确定性（--salt + --inject 同参双跑逐字节一致） | 现有语义不动（已实测 `cb92c7b9…` 双跑一致）；core_hash 确定性派生，`--inject` 覆盖全部核键即可复现（T1/T3） |
| 退出码三值钉死 | 全为报告字段增量，不改判定语义：0=合规 / 1=违规或异常 / 2=工具异常；R5 三态映射（DES-011 优先级）不动 |
| 零写路径 | identity 工具仍只读（仅 stdout）；core_hash 随现有报告由批流程落盘，工具侧零新增写点 |
| 跨 harness 血统区分（ancestry 原始意图）不丢 | 全链留报告（取证）；归一化 token 进核（承意图、去脆性）；声明式 harness 字段为后续正名位 |

**不确定处明说**：(1) 归一化规则的精确形态（通用包装段集合、host-local-N 是否归一）需一次明确裁定；(2) boottime 进核 = 重启即换席位纪元，若治理上认为重启后仍应连认，则需把 boottime 降到会话面（二选一，需裁定）；(3) session_id/harness 声明式注入属跨组件契约，落地面与排期待定。

---

## 7. 变更清单（实装批输入）

| 组件 | 变更 | 性质 |
|---|---|---|
| `sih-tools/identity/src/identity/core.py` | 新增 `core_hash()` 与核键序常量；`collect_components` 不动；v3 身份串与 `identity_hash` 不动；net_time 探针改可选（可加 `--no-net` 或按环境自动降级，具体形态实装裁） | 增量 |
| `sih-tools/identity/src/identity/cli.py` | 报告新增 `identity.core_hash` 与 `identity.core_components`；退出码与现有参数不动 | 增量 |
| `sih-tools/lease/src/lease/core.py`（`load_identity`） | 新增读 `core_hash` 入台账 `identity` 字段（`file_sha256`、`identity_hash` 不动，`session_id` 派生不动） | 增量 |
| `sih-tools/lease/src/lease/lockcore.py` | 三键 `verify_binding` 硬闸保留（机器/操作者/纪元变了仍必拦）；可选增强：core_hash 匹配作正向连认注记 | 增量（三键闸不动） |
| `sih-tools/tally/src/tally/cli.py`（R5） | 配对键改为 **core_hash 优先等值**；材料无 core_hash（旧件）时回退现有 full 等值/纯基线判定（复用现有回退结构 `cli.py:161-169`） | 增量（回退保旧判） |
| 席位基线文件 | 新增 `core_hash` 字段（`identity_hash`、`seat` 标签不动） | 增量 |

围堰面声明：以上均为 sih-tools 工具侧（identity/lease/tally），不动 sih-engine 治理引擎；工具线批按 sih-tools/COURSE-v2 结算追加制走。

---

## 8. 迁移路径（不破坏存量）

**存量面**：台账 198 条 issued（各载 `identity.{identity_hash, file_sha256}`）；tally/facet 材料散见 `sih-tools/proposition/DES/**`（`identity_hash` + `seat_baseline_path`）；当日链 301 事件仅 2 行提及 identity（身份引用主要在台账/材料/基线，不在链逐事件）；`.bindings` 侧档只载三稳定键，与哈希无关，天然兼容。

**四步（全增量，旧值不失效）**：

1. **identity**：按 §7 加 core_hash 与 core_components 字段；full 一字不改；net_time 探针可选化。
2. **lease**：`load_identity` 增读 core_hash 入账；连认键 = 三键硬闸（不变）∪ core_hash 正向连认。**租约侧读哪个哈希：连认/对表读 core_hash；取证与台账历史仍读 identity_hash（full）。**
3. **tally R5**：core_hash 优先配对；旧材料无 core_hash 自动回退现行为（旧件重放逐字节同判）。
4. **链上审计双认**：按版本双轨——材料/台账含 `core_hash` 字段即按 core 语义认；只有旧 `identity_hash` 即按 v3 全哈希语义认且**不做跨批等值连认**（其本就不稳），只作单条取证。字段在场即可机械判别，无需人工对照。

**并存期**：以完整"席位日"为最小单位，至少覆盖一个跨重启日（验证 boottime 纪元语义），粗估 **2–5 个席位日**。并存期 R5 与租约双轨全绿方算收敛；收敛后旧 full 等值配对标退役（保留只读兼容）。

---

## 9. 需裁定事项（人节点）

1. **归一化血统 token 规则**：通用包装段集合 {python3, uv, zsh, bash, sh, launchd, agent-tool-host} 与 `zcode-host-local-N → zcode-host-local` 归一，是否成立？（候选规则，原型已验可行性）
2. **boottime 进核的纪元语义**：重启 = 换席位（与租约 `boottime_drift` 拦截一致，提案倾向）vs 重启后仍连认（boottime 降会话面）。
3. **声明式注入排期**：`SIH_SESSION_ID` 与 harness 席位声明由谁注入、何时注入（跨组件契约，可与 1、2 分离先行）。
4. **net_time 探针可选化的具体形态**：显式参数（`--no-net`）vs 按环境自动降级。

---

## 10. 兼容性、回退与风险

- **兼容性**：full v3 一字不改，39 份存量报告复算逐字节一致（T6）；旧材料 R5 回退现行为（复用既有回退结构）；`.bindings` 侧档与 session_id 派生不动。
- **回退**：任一步异常可"停用 core_hash 连认、回退 full/三键"，不影响已落链历史。
- **残余风险**：net_time 探针可选化后，离线环境失去外部钟对照，`clock_skew` 检测能力下降（可用 boottime+timestamp 本地一致性部分补偿）；mac 在云 VM 随机化时降格为缺席（现有行为，core_hash 对空串稳定）；core_hash 输入均在报告 `observed` 中明文，其防冒充价值来自**组件组合与确定性连认**而非密码学隐匿——盐对公开输入的额外保密性本就有限（full 哈希同理），此为设计内取舍。
- **防冒充不降级声明**：机器（mac/hostname）、操作者（user）、纪元（boottime）、沙箱（sandbox_id）、harness（lineage token）五向冒充检测全部保留（T4a–e）；实例级防重放由 full 哈希原样承担。

---

## 11. 参考与证据索引

- 工具源码：`sih-tools/identity/src/identity/core.py`（COMPONENT_ORDER、identity_string、identity_hash、collect_components、detect_anomalies）、`sih-tools/identity/src/identity/cli.py`（verify 双模式、退出码三值）
- 租约消费面：`sih-tools/lease/src/lease/core.py:84-99,180-192,257-260,379-397`（load_identity、write_binding、make_session_id、open_session）、`sih-tools/lease/src/lease/lockcore.py:13,146-168`（BINDING_KEYS、verify_binding、verify_five）
- R5 配对：`sih-tools/tally/src/tally/cli.py:134-169`
- 漂移观察：`sih-engine/doc/spec/SPEC-016-attractor-proposition-drafting.md` § 附录（身份哈希漂移观察）
- 漂移证据件：`sih-tools/identity/reports/2026-09-02-{predab,autoflow2,predsp2,sweep6}-identity.json`
- 席位基线样例：`sih-tools/proposition/DES/m-mathrelease/seat-baseline-zcode-2026-09-02.json`
- 台账：`sih-tools/lease/ledger/sessions.ndjson`（198 条 issued）
- 当日链：`sih-engine/sih/event/trail/2026-09-02.ndjson`（301 事件）
- 批流程先例（正身→lease open→取锁→上链全序）：`sih-engine/sih/event/plan/autoflow2-solo-materials/dispatch.md`、`sweepclea6-solo-materials/dispatch.md`
