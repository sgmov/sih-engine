# DES-014 MCP beta 相写面会话与租约映射安全模型

## 概览 {#overview}

- 本设计承 mcpline 线程序包批二定位，为 record 与 lease 写操作经 MCP 的安全模型正典，是 beta 相实装批的硬前置，未过本设计不一裁实装批不开工
- 连接与会话映射判词取一对一形：一条 MCP stdio 连接绑定恰好一个 lease 会话，连接建立即 lease open 立会话，禁多对一禁池化::[连接与会话映射](#session-mapping)
- 身份判词取进程正身形：握手不自报身份，MCP server 进程自采正身件十二件签发，lease 五验逐写操作复核，事件信封落身份哈希::[身份](#identity)
- 授权判词取路由层矩阵形：本地可信 agent 全矩阵，外部冷 agent 最小写集，矩阵只裁剪透传不新增执法零 LLM 裁决位::[授权](#authorization)
- 写路径判词取零新增判定形：beta 写面是既有 CLI 的薄投影，scribe 闸二闸三、lease 五验、closeguard、git hooks 既有执法原位生效，禁另立第二套判定::[写路径复用](#enforcement-reuse)
- 审计判词承既有哈希链形：每笔写已上链是既有事实，本设计只定重放与滥调防护与排队透出形，零新审计面::[审计与重放](#audit-replay)
- 失败语义判词取分级裁剪形：报错即教学教协议与调用形与理由码，不泄露绝对路径面、他会话身份与他会话存在性，裁剪规则是静态映射表::[失败语义](#failure-semantics)
- 威胁模型三向各给缓解或明示不防：进程劫持不防实时阻止只保事后可审计，工具描述注入冻结描述文本不假设客户端可信，锁面耗尽承保序队列与心跳清账与 takeover::[威胁模型](#threat-model)
- 边界申明：注入面通用化与自有运行时归 pk-079 不在本设计，GOV-002 与 v3 换版不触碰，alpha 相读数面归 SPEC 023 号契约件 不重复::[边界申明](#boundary)

## 连接与会话映射 {#session-mapping}

判词：一对一。一条 MCP stdio 连接对应恰好一个 lease 会话，连接建立即立会话，断开走收约，禁多对一禁连接池。

绑定形：MCP initialize 握手完成后，server 以本进程正身件调用 lease open 立会话，会话号经工具结果透出给客户端记录；此后该连接上的一切写操作携带该会话号，租内五验逐操作核验。多对一判词是否定：会话是授权与审计的主体粒度，多连接共享一会话即责任面混淆与审计去粒度，事件信封无法区分笔迹来源；连接池判词是不适用且连带禁止：MCP stdio 形态下一客户端进程一连接无池形态，未来多路复用传输出现时池化仍禁，理由同多对一。一对一的依据是基线四可验证性：一会话一连接一 agent 即操作来源可追溯、责任面单一、链笔信封逐笔可归因。

隐式占位态(pk-093 其一归一，penface 批)：连接级自动立会话未成留既成事实(进程内 auto_open_error 在案)而零显式绑定即隐式占位态，会话在租约层可能存在；open 与 close 俱经同一绑定查找位(bound、implicit、unbound 三态)，close 于隐式占位态放行透传既有 CLI 收约(close 能收隐式会话，链闸与 closeguard 原位，零新增判定)，收约成即隐式占位事实清零可显式重开；结构卫两处错误载荷(session_already_bound 与 session_not_bound)教学互指隐式占位事实。

断开收约形：stdio 管道正常关闭即触发收约前置检查；零写连接轻收约即 lease close 拆会话；有在途写的连接走完整 close，链闸与 closeguard 三检照走，收约失败即会话转挂起候人节点 takeover 裁决，server 不强拆不静默弃置。

孤儿连接与僵尸会话处置：server 进程崩溃未走 close 时，既有机械兜底原位承接，即检验文件生死 PID 探针三态判(kill 探针加出生时刻对表)、心跳停滞判据(HEARTBEAT_STALE_SECONDS 冻结常量三百秒)、takeover 人节点显式接管以 takeover_release 清该会话全部现势锁与队列行。零新增清理机制，依据是 lease 契约修订三十三与 openhyg 勘误既有语义。

## 身份 {#identity}

判词：进程正身形。握手不自报身份，身份唯一承载是 MCP server 进程自采的正身件，签发即采集，复核在写路径双位。

握手验什么：MCP initialize 的 clientInfo 与一切客户端自报字段不作身份依据(自报不可信)；server 启动即跑正身 verify 生产路径采集组件产盐哈希，lease open 以该正身件立会话并落会话台账，正身件哈希随会话在册。

正身 v3 十二件绑定：pid、ppid、hostname、mac、user、sandbox_id、session_id、boottime、parent_start、ancestry、net_time、timestamp 键序固定；沙箱与会话两可选件经 SIH_SANDBOX_ID 与 SIH_SESSION_ID 环境注入，外部冷 agent 场景由 server 以连接派生值注入使不同连接的可选件可区分。事件信封正身绑定既有位生效：会话在册即每笔链事件落 session_id 与 identity_hash，笔迹逐笔归因。

谁签发谁复核：签发即 server 进程自采自签(盐随机逐次不同是设计内行为，复演对表走显式盐路径)；复核双位，其一 lease lock 悲观五验的身份验逐写操作重验(identity_anomaly 与 hostname_drift 与 user_drift 与 boottime_drift 理由码原位)，其二链面审计按 identity_hash 信封对会话台账回查。边界如实申明：正身防偶发撞名不防产权级仿冒(mac 属主可配置)，承 identity 契约既有边界，本设计不扩不缩。外部 agent 不自带身份件，其可区分性即会话号粒度。

## 授权 {#authorization}

判词：路由层矩阵形。授权矩阵只决定工具是否向调用方透传，透传后的判定全归既有程序；矩阵是静态映射表随批评审，零 LLM 裁决位。

record 写操作授权：intent、append、park 三操作本地可信 agent 与外部冷 agent 俱可调，逐操作过 scribe 闸二闸三；direct 直改笔仅本地可信 agent 可调(直改车道免全套租约仪式但免不了治理，只留本地可信位，冷 agent 一律走全仪式通道)。

lease 写操作授权：open、lock、unlock、wait-turn、claim、unclaim 俱可调(冷 agent 只能操作自己会话，五验与绑定验天然拦他会话操作)；commit、close 俱可调且受 close 链闸约束即本会话链上有意图笔与认证笔方可收约；takeover 不可调(人节点显式接管位，server 对任何来源拒透传)；bypass 不可调(绕行登记属人节点裁决位，拒透传)。

分级判词：本地可信 agent(与 server 同主机、走治理批任务包立约的 agent)全矩阵除 takeover 与 bypass 外可调；外部冷 agent(零上下文经 MCP 接入)取最小写集即 open、intent、append、park、lock、unlock、wait-turn、commit、close、claim。分级是路由层裁剪不是新增执法：拒透传即不发起调用，不存在矩阵放行而执法拦截的第二套判定；矩阵放行后的拒定全部来自既有程序理由码，可对表。

## 写路径复用 {#enforcement-reuse}

判词：零新增判定。beta 写面是既有 CLI 的薄投影，参数透传、结果透传、退出码透传，全部既有执法原位生效。

scribe 闸复用：工地链副本禁追加护栏(trail 物理路径含 worktrees 段即拒退出码二，认证先落主链)、闸二重意图拒(同 record 路径已有意图笔在链即拒，IntentRecordUsedRejected)、闸三会话在册验(会话须在会话台账活跃，SessionNotActive)三护栏对 beta 写面逐笔生效，server 禁以任何方式代填或绕过。

lease 执法复用：open 六位闸序(包校验、正身、意图、同包活跃、施工面交集预检、检验钥匙闸)、lock 悲观五验(在册验、会话择定、范围验、身份验、绑定验)、unlock 加持锁验、closeguard 收约三检(脏位枚举、diff 判别、让位补笔拒三态)、close 链闸、claims 领取账本(PackageAlreadyClaimed 拒与 claims_warning 警示)俱原位。

commit trail 与 root 透传(pk-091 其三，penface 批)：MCP lease_commit 补 trail(链路径列表，逐条 --trail)与 root(工作区根覆写)两参对齐 CLI；缺席即 CLI 缺省全链按日序发现原位，缺省发现语义零变；HTTP 面 root 属调用方域指向面零透传(DES-015 主防线，server 锚定所绑域)，trail 逐条绑定验(前缀属所绑域根)后透传。

git hooks 复用：commit-msg 位直提守卫原位，无登记 plain commit 即拒，绕行必经 lease bypass 落台账，reconcile 查 unbypassed。

禁令三条：server 禁内嵌任何 allow 或 deny 判定逻辑替代执法(授权矩阵是路由层非执法层)；禁缓存执法结果跨调用复用(每笔写独立过闸)；禁绕过 CLI 直写台账、链文件或 git 对象(写面封闭即源码写调用仅既有通道)。

## 审计与重放 {#audit-replay}

判词：承既有链。每笔写已上链是既有事实(scribe 哈希链 append-only，写即回执 event_hash 为法定凭据，confirm 单笔确认)，本设计零新审计面，只定重放与滥调防护与排队透出形。

幂等键：intent 以 record 路径为幂等键，闸二拒重放(同 record 路径唯一意图笔，--allow-reintent 通道缺省关)；append 无幂等闸如实申报，重放同 report 即重复认证笔，滥调防护靠三既有位即事件信封审计(每笔 append 载 session_id 与 identity_hash)、meter 计数交叉核对、锁面持有约束，重复笔可审计检出，处置归人节点，不新增自动拒重。

锁冲突语义透出形：locked_elsewhere 拒、queued 入队返位次、queued_not_your_turn 保序公平拒，理由码原样透出给调用方，不吞不译不重试代劳；wait-turn 以独立工具透出(阻塞机械轮询、timeout 到即如实返退出码一并载出队事实，零 LLM 轮询零 token 消耗语义保持)。

重排防护：链铸时戳由 append 写位 flock 临界区内机械自铸单调值(取调用方提示与链尾加一毫秒的较大者)，乱序重放不可插入，并行批上链零拒收；拒收睡等重试类消耗由构造根除。

## 失败语义 {#failure-semantics}

判词：分级裁剪形。报错即教学的边界按调用方来源分级，教学面教协议不泄露侦察面，裁剪规则是静态映射表非 LLM 判定。

教什么：参数名与必填集(调用形错误全参名透出)、退出码三值语义(零成、一拦、二工具异常)、闸位名称与理由码(SessionNotActive、IntentRecordUsedRejected、locked_elsewhere、scope_violation、open_precheck_conflict、PackageAlreadyClaimed)、修复动作指引(wait-turn 排队、close 后重开、显式带会话号、按理由码改调用形)。

不泄露什么：绝对路径面(错误转述一律工作区相对形，禁止绝对路径进错误文本)；他会话身份(会话号与持有者标识不进任何错误文本)；他会话存在性(错误只告本操作不可成与建议动作，不告谁在持有什么；施工面交集预检的持有者清单对本地可信位是协作信息、对外部位是侦察面，MCP 错误转述层按会话来源分级裁剪，外部位只载冲突计数与建议动作)。

张力如实申报：裁剪降低外部 agent 自助排障能力即教学面收窄，权衡依据是基线五不生成不等于不引导与基线二人类注意力只投异常信号；裁剪映射表随批评审可调，调整走修订不改判定语义。零动态拼接不可信数据进错误文本(拼接面即注入面，见威胁模型节)。

## 威胁模型 {#threat-model}

判词：三向各给缓解或明示不防，防域与不防域逐条申报不含糊。

服务器进程被劫持：明示不防实时阻止。server 进程持正身件与会话凭据，劫持即可以该身份写链，此为本地信任模型既有边界(identity 不防产权级仿冒同界)，本设计不声称改变；缓解在事后可审计，劫持者每笔写落信封身份与链哈希，链 verify 全文验、watchcheck 无主活写对表、critsweep 判据扫俱可检出异常，翻案经 inconsistency 通道留痕。

经工具描述的 prompt injection：缓解两件，其一工具描述冻结为契约文本(静态、随批评审、只载操作语义不含可执行指令面)，其二结果与错误文本固定映射零动态拼接不可信数据；明示不防客户端 LLM 被注入后主动调用写工具，防线归授权矩阵路由裁剪与既有执法(五验、闸位、锁面)，MCP 面不假设客户端可信也不新增防线，防域申明与 SPEC 023 号契约件 alpha 相一致。

恶意 agent 耗尽锁面与台账膨胀：缓解三件，即 lock 粒度持锁使占用面有界、wait-turn 保序队列使后来者不饿死、心跳停滞机械清账加 takeover 人节点显式回收使僵尸持有可清；台账膨胀明示不防(append-only 即容量换审计，清理归人节点后裁，零自动清理零自动降采样)；合法身份内的写频不设速率限制，明示不防归人节点监管位。

## 边界申明 {#boundary}

注入面通用化与自有对话循环归 pk-079 分相承载，本设计只涉 MCP server 进程边界内的工具投影层，不涉客户端运行时与注入面通用机制。GOV-002 与 v3 换版不触碰：写面投影以现行各工具版本为界(lease 1.39.0、scribe 引擎件现行、identity 0.4.0、meter 0.2.0)，版本升级走各工具既立换版程序，本设计不预设不预占。判定语义正典留确定性程序：本设计零 LLM 裁决位，授权矩阵与错误裁剪映射俱是静态数据，任何判定语义变更走既立程序(得一裁加执契终签)。alpha 相读数面归 SPEC 023 号契约件 已立契约，本设计不重复不修改不冲突；beta 实装批未过本设计不一裁不开工，本设计亦不预写实装。本设计属 des-001 域，产出经化格核阅检词认证序。

## 修订记录 {#revisions}

2026-09-09 修订一：随 mcpsec-solo 批起草，承 mcpline 线程序包批二定位与任务包 mcpsec-solo.md 八问，令源用户 2026-09-09 令「继续。多子代理协作」；地面实况核对即 design 目录编号最高 DES-013 顺延 014，执法语义对表 lease 契约修订三十三与修订四十与 identity 契约修订三与引擎 scribe 闸位注记。
2026-09-10 修订二：随 des016impl-solo 批即意图闸结构校验中性化落笔。设计承 DES-016-mcp-pure-tool-intent-form-v1.md；测量 m-des016-2 三发 stable_clear 经 attractor 裁决通过机器终签 756effb1 在链；令源用户 2026-09-10 常识裁定原话「世界上没有任何一个工具承载了哲学仪式」与同日开工令。意图闸六位闸序第三位描述更新：意图件验收改结构校验形，plain 与 ask3 双形同一必填集即 session_id 与 raw_input 与 round 与 intent_contract 之 goal 必填与 domain_contract；哲学引文逐字节核验摘出机械闸归司衡本域批纪律，BATCH-FACE 三问双门必跑步原位。scribe intent plain 通道即 --validation 对 plain 形豁免，拒因面收窄即 plain 形不触发 ValidationNotOk 与 LineageMismatch 与 FindingsPresent 三拒因。授权矩阵与身份与会话映射与写路径复用与审计与威胁模型与失败语义各节零触碰，本修订只及意图闸校验内容与 scribe 写位参数面；DES-015 标识牌层零涉。
2026-09-11 修订三：随 penface 批(gatefix-parallel 簇M)两部位落笔，令源用户 2026-09-11 令「多子代理全量修复」承 pk-091 与 pk-092 与 pk-093 出泊条件。其一连接与会话映射节增隐式占位态段：绑定查找归一单点位(bound、implicit、unbound 三态)，close 能收隐式会话，两处结构卫错误载荷教学互指隐式占位事实。其二写路径复用节增 commit trail 与 root 透传段：MCP lease_commit 参数面对齐 CLI，缺省发现语义零变，HTTP 面 root 零透传承 DES-015 主防线。授权矩阵与身份与审计与威胁模型与失败语义各节零触碰；DES-016 外部只教不拒零变；first_domain 形零变。锚定测试 tests/test_penface_fixes.py 八红先红后绿，红证认证 171b815c 在链，mcpline 全套 160 绿。
