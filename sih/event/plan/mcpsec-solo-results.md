# mcpsec-solo 批结果档

## 概览 {#overview}

- 本批承 mcpline 线批二使命，DES-014 MCP beta 相写面会话与租约映射安全模型落档 sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md，八问逐问有判词与依据，编号经地面核对自 DES-013 顺延
- 得一测量 m-mcpsec-1 九发九合变卦零谨慎零，basis baseline_1 单类，闸门裁决清晰稳定 stable_clear，执契 attractor check 十二项全过 verify identical sign 终签上链 1763a223 经 confirm 单笔确认
- 会话 02a0fd176dd15fbc 九路径锁全取零冲突，意图笔 bba1b257，管线三步绿，checkcite pass，双仓 settle，收约读数见收约节
- 零实装代码红线达成：本批产物只有设计档与测量材料，sih-tools/mcpline 与任何既有工具代码零触碰

## DES-014 八问判词摘要 {#verdicts}

- 问一连接与会话映射：一对一形，连接建立即 lease open 立会话，断开走收约，孤儿连接与僵尸会话归既有 PID 探针三态与心跳停滞与 takeover 通道处置；多对一禁（责任面混淆与审计去粒度），连接池不适用且连带禁（MCP stdio 无池形态）
- 问二身份：进程正身形，握手不自报身份，server 进程自采正身 v3 十二件签发，lease 悲观五验逐写复核，事件信封落 session_id 与 identity_hash；签发即采集，复核在五验身份验与链面审计双位；正身防偶发撞名不防产权级仿冒边界如实申明
- 问三授权：路由层矩阵形，本地可信 agent 全矩阵除 takeover 与 bypass，外部冷 agent 最小写集（open intent append park lock unlock wait-turn commit close claim），矩阵只裁剪透传不新增执法零 LLM 裁决位
- 问四写路径复用：零新增判定，scribe 三护栏（工地链禁追加、闸二重意图拒、闸三会话在册验）与 lease open 六位闸序与 lock 五验与 closeguard 与 close 链闸与 git hooks 俱原位生效，禁另立第二套判定禁缓存执法结果禁绕 CLI 直写
- 问五审计与重放：承既有哈希链，intent 以 record 路径为幂等键闸二拒重放，append 无幂等闸如实申报（信封审计加 meter 交叉核对加锁面持有防护，处置归人节点），锁冲突理由码原样透出，wait-turn 零 LLM 轮询透出，链铸单调时戳防乱序
- 问六失败语义：分级裁剪形，教协议与调用形与退出码三值与理由码与修复动作，不泄露绝对路径面与会话身份与他会话存在性（预检持有者清单按来源分级裁剪），裁剪规则静态映射表随批评审可调
- 问七威胁模型：进程劫持明示不防实时阻止、缓解在事后可审计（信封身份与链哈希与 verify 与 watchcheck 与 critsweep）；工具描述注入缓解在描述冻结与错误文本固定映射、明示不防客户端被注入后主动调写（防线归矩阵与既有执法）；锁面耗尽缓解在粒度持锁加保序队列加心跳清账加 takeover；台账膨胀明示不防（append-only 即容量换审计）
- 问八边界申明：注入面与自有运行时归 pk-079 不在本设计，GOV-002 与 v3 换版不触碰，判定语义正典留确定性程序零 LLM 裁决位，alpha 相读数面归 mcpline 线 SPEC 023 号契约件不重复不修改

## 测量与执契读数 {#measurement}

| 项 | 读数 |
|---|---|
| gid 与席位 | m-mcpsec-1，ZCode:GLM-5.3:self-reported，九发 |
| 采样 | 9 发 9 合，变卦 0%，谨慎 0/9，规约引用 baseline_1 单类 |
| 闸门 | 清晰稳定 stable_clear（判据 v3） |
| 标定 | temp_probe agent 模式 retired 件 /tmp 副本实跑，四命题五发，体温 0.0，violate 5/5 判违，knife bdy 1.0，判定可用，漂移无 |
| R5 配对 | 基线 identity fb15ea92826a 与计分材料一致（哈希优先配对） |
| 执契 | assemble 后 attractor check 十二项全过 disposition 裁决通过 direction comply，verify identical，sign crosscheck-m-mcpsec-1 落链 1763a223 index 92，confirm 在档 |
| 材料 | 合同与响应与基线与计分材料落 facet/contracts/m-mcpsec-1/，topic 与飞轮 trail 与计分材料与执契材料与 check 报告与 signcheck 落 proposition/DES/m-mcpsec-1/ |

- 谱系披露：命题文本出自本批委外代理起草承任务包八问，采样作答席位与命题起草同席（委外单席位批无跨席隔离，如实申报），改写链第 0 次；对己不利声明载 topic authored 行
- 前置分道：事实面（DES 档在盘八问节齐、任务包八问在档、线程序包候前置条款在档）归确定性核对通道不入采样，采样只裁规约相容性

## 管线三步与书单对表读数 {#pipeline}

| 闸 | 首跑 | 修复 | 终态 |
|---|---|---|---|
| 化格 formatter general-v1 | exit 0 零改动 | 两轮材料修复后各复跑 exit 0 零改动 | 绿 |
| 核阅 scrutinator des-001 | exit 1 红证 41 笔全 C006 全角括号内容非法 | 全角换半角材料修复判定文本零改动 | exit 0 零违规绿 |
| 检词 nomenclator core | exit 0 零违例 | 复跑两轮 exit 0 | 绿 |
| checkcite | exit 1 红证 cited SPEC-023 missing（红证留 2026-09-09-mcpsec-solo-checkcite-firstrun-specid-red.json） | SPEC-023 改书 SPEC 023 号契约件，引用语义零变 | exit 0 verdict pass 绿 |

- 材料修复申报：两处首跑红俱材料形缺陷（括号全半角、治理编号书写形撞引用扫描正则），按 facepatch F-1 材料修复即修复重核通道推进，判定文本零改动，非材料与源实质冲突不停批；修复先例即 mcpspec 同款 selfname 红证同形解法

## 误差与红证申报 {#deviations}

- attractor verify 首跑 exit 2 报缺报告文件：check 判定输出走 stdout 未自动落盘，本批首跑即 verify 触错；红证如实申报，复跑 check 重定向落盘 check-report.json 后 verify identical 一次过；先例 m-mcpopen-1 同形（其 check-report 亦为 stdout 落盘件）
- 标定账本落 /tmp 副本（adjudicate2 与 sweepjson 与 mcpopen 先例同形）：temp_probe score 账本落点随脚本目录解析，本批在 /tmp 副本实跑使主树与工地标定账本零污染零直写，drift 对真实账本副本历史配对（同族 0905 与 0906 两行在册）读数为无
- temp_probe 已退役归档（baseinject-solo 批）申报：本批标定走 probes/retired/temp_probe.py 在 /tmp 副本实跑，零改退役件本体，零主树写入；当日基线以账本尾行 verbatim 出件，build-baseline 读档注入通道（baseline_inject.py）未用因其取末条可用行为 GLM-5.3-Flash 席位与本席不符，如实申报择路理由
- 起草与作答同席申报：委外单席位批无跨席隔离，命题起草与九发作答同席，topic authored 行与谱系披露节双载
- 其余误差零申报：ask3 双门首跑绿、叩问预登记十二词与实测十二信号一致、租约九锁一次全取、intent 与 park 未复跑、sign 一次过未复踩闸三坑位

## 链笔与收约 {#settle}

- 意图笔 intent_refined bba1b257（session 02a0fd176dd15fbc）；终签笔 crosscheck_completed 1763a223 index 92；认证 append 笔见当日链 confirm 复算
- 双仓 settle 提交号与 reconcile 增量与 verify 全文见完工回报，收约后回锚重跑
