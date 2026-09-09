# mcpauth-solo 批结果档

## 概览 {#overview}

- 本批承 mcpline 线 HTTP 写面设计批使命，DES-015 MCP HTTP 面多项目识别与分项目写授权安全模型落档 sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md，八问逐问有判词与依据，编号经地面核对自 DES-014 顺延；批中承用户同日基底改令「每项目一域，工具中央账本随项目」按新基底全稿重推导，旧基底稿存批材料 des015-oldbasis-draft-v1.md 作对照
- 得一测量 m-mcpauth-1 九发九合变卦零谨慎零，规约引用 baseline_1 与 baseline_4 与 baseline_5 三类，闸门裁决清晰稳定 stable_clear，执契 attractor check 裁决通过 comply verify identical sign 终签上链 a4f11822 经 confirm 单笔确认
- 会话 9895d6695cf45290 九路径锁全取零冲突，意图笔 0cd9e7cb，管线三步绿，checkcite pass，双仓 settle，收约读数见收约节
- 零实装代码红线达成：本批产物只有设计档与测量材料，sih-tools/mcpline 与任何既有工具代码零触碰

## 基底改令申报 {#pivot}

- 令源与转向：批开工承令「http要分项目的如何做，也要做写入权限」，设计按共享单一治理域加令牌命名空间裁剪基底起草；批中用户裁定转向「每项目一域，工具中央账本随项目」，即链与锁册与台账与泊界住各项目自己的 sih 目录、令牌首要职责改域绑定即 token 映射 SIH_ROOT 写只落所绑域、工具本体中央一份全域共用、司衡仓即第一域历史布局不迁移、SiInfer 即首个新城
- 承接形：DES-015 八问框架不变判词按新基底重推导全稿重写；旧基底稿（已过化格与核阅首跑红证修复态）存 sih/event/plan/mcpauth-solo-materials/des015-oldbasis-draft-v1.md 作对照不删；facet 命题 m-mcpauth-1 按新基底定稿文本起草，谱系披露载转向申报
- 判词位移如实申报：跨项目越权防线从命名空间逐调用裁剪改为域绑定结构性隔离（越权靠结构性不可寻址不靠裁剪拦截），scope 三档退为域内细粒度薄层；域内治理完整性（链验与判据扫与秤星域内自足闭合域间零依赖）升为设计目标；新增中央工具版本对全域的约束一节（域内零工具副本防执法分叉、读兼容硬约束、升级程序唯一、中央单点可用性边界如实申报）

## DES-015 八问判词摘要 {#verdicts}

- 问一项目识别形：一项目一治理域形，域目录布局规约定稿即域根下 sih 目录树承载链与台账册族与治理状态与泊界，第一域即司衡工作区双仓历史布局登记映射不迁移；token 签发通道归主窗签发位承人节点令源（签发不对 HTTP 客户端开放一律拒透传），台账形即域内 tokens.ndjson 哈希落册明文一次性，传输形三形取舍取 Authorization Bearer 头形拒路径段形拒每项目端口形
- 问二识别与会话映射：令牌会话一对一形，一令牌同时刻至多一个活跃 lease 会话且会话住所绑域，身份锚从进程正身形变体为 server 进程正身件加令牌哈希注入复合形（正身件答谁在写、令牌册答写到哪域），五验逐项 HTTP 变体逐条载（在册验查域册末行 active、择定验落本连接会话、范围验 scope 薄层、身份验进程侧漂移加令牌哈希对表、绑定验凭据哈希加路径前缀属所绑域根）
- 问三授权矩阵：域隔离为主加域内薄层形，scope 三档 readonly 与域写档与 custom，分项目写授权主防线是域绑定结构性隔离；与既有 scope_violation 与 locked_elsewhere 与 wait_turn 理由码原位衔接零新理由码零新执法位
- 问四写路径复用：零新增执法，令牌只裁剪透传与域锚定，scribe 三闸与租约五验与 closeguard 与 git hooks 在所绑域原位生效；record append 无幂等闸的 HTTP 重试风险如实申报，缓解两件即锁纪律强制形加重放明示不防（审计检出处置归人节点）
- 问五审计与撤销：承既有哈希链加域内自足形，信封 identity_hash 经令牌哈希注入直接归因零回查闭合；撤销即所绑域册追加行末行为准拒定位在路由层拒透传；轮换同批落链双活窗口如实申报；链验判据扫秤星每域自足闭合域间零依赖
- 问六威胁模型：令牌泄露域限爆炸半径缓解三件、跨项目越权结构性消失（残余面 server 进程劫持承 DES-014 同界影响面放大申报）、重放缓解链铸单调时戳明示不防两件、枚举缓解高熵加 401 统一零区分、台账膨胀明示不防域限、中央工具本体篡改明示不防实时阻止缓解事后可审计三件；缺省绑定 127.0.0.1，远程与 HTTPS 暴露明示不防入边界
- 问七失败语义：双层映射形，传输层 401 统一五因零区分（含跨域令牌与域册不可读 fail-closed）不泄露跨域存在性、403 薄层拒、429 映射语义在档限流器零实装；载荷层执法理由码原样透出加教学四字段加 retry_discipline；不泄露绝对路径面他会话身份他会话存在性他域存在性
- 问八边界申明：司衡仓即第一域不迁移、SiInfer 彩排即首个新城第一实例但不预占、自有运行时归 pk-079 与 GOV-002 v3 不触碰、判定语义零 LLM 裁决位、alpha 相读数面归 SPEC 023 号契约件不重复、DES-014 stdio 面不废两面对表同一执法层重叠面从其判词

## 测量与执契读数 {#measurement}

| 项 | 读数 |
|---|---|
| gid 与席位 | m-mcpauth-1，ZCode:GLM-5.3:self-reported，九发 |
| 采样 | 9 发 9 合，变卦 0%，谨慎 0/9，规约引用 baseline_1 与 baseline_4 与 baseline_5 三类 |
| 闸门 | 清晰稳定 stable_clear（判据 v3） |
| 标定 | temp_probe agent 模式 retired 件 /tmp 副本实跑，四命题五发，体温 0.0，violate 5/5 判违，knife bdy 1.0，判定可用，漂移无 |
| R5 配对 | 基线 identity 2e50ddedfcc5f394 与计分材料一致（哈希优先配对，本批当日正身） |
| 执契 | assemble 后 attractor check 裁决通过 direction comply，verify identical，sign crosscheck-m-mcpauth-1 落链 a4f11822 index 146，confirm 在档 |
| 材料 | 合同与响应与基线与计分材料落 facet/contracts/m-mcpauth-1/ 工地位并以快照形复制入 proposition/DES/m-mcpauth-1/contract-snapshot/，topic 与飞轮 trail 与计分材料与执契材料与 check 报告与 signcheck 落 proposition/DES/m-mcpauth-1/ |

- 谱系披露：命题文本出自本批委外代理起草承任务包八问与批中基底改令，采样作答席位与命题起草同席（委外单席位批无跨席隔离，如实申报），改写链第 0 次；对己不利声明载 topic authored 行
- 前置分道：事实面（DES 档在盘八问节齐、任务包八问在档、中央工具现位在档、第一域双仓布局在档）归确定性核对通道不入采样，采样只裁规约相容性

## 管线三步与书单对表读数 {#pipeline}

| 闸 | 首跑 | 修复 | 终态 |
|---|---|---|---|
| 化格 formatter general-v1 | 旧稿与新稿首跑各 exit 0 零改动 | SPEC 023 引用形修复后复跑 exit 0 | 绿 |
| 核阅 scrutinator des-001 | 旧稿首跑 exit 1 红证 1 笔 N002 概览导航项 write:ns 半角冒号破字符类；新稿首跑 exit 1 红证 2 笔 C001 加 C002 破折号 | 两轮均材料形修复判定语义零改动 | exit 0 零违规绿 |
| 检词 nomenclator core | exit 0 零违例 | 修复后复跑 exit 0 | 绿 |
| checkcite | exit 1 红证 cited SPEC-023 missing（红证留批材料 checkcite-firstrun-spec-red.json） | SPEC-023 连字符形改书 SPEC 023 号契约件，引用语义零变 | exit 0 verdict pass 绿 |

- 材料修复申报：三轮首跑红俱材料形缺陷（导航项冒号字符类、禁用破折号字符、引用书写形撞扫描正则），按 facepatch F-1 材料修复即修复重核通道推进，判定语义零改动不停批；先红留痕纪律红证全量归档批材料

## 误差与红证申报 {#deviations}

- 管道掩码自违两笔申报：其一书简意图笔落笔后退出码经 tail 管道取码被顶替（意图笔以写即回执 event_hash 0cd9e7cb 在场加 confirm index 145 补验零损）；其二 facet emit-contract 首跑 --seat 三段形报 ValueError 而退出码被 tail 顶替误报零，改重定向取真码复跑两段形即过；两笔俱即改重定向形续链，真码假码辨析以链上回执为法定凭据
- facet responses key 首跑错形申报：九发响应 key 自拟 gid-序号形与合同 key 形 gid#rN 不符，score 拒收（key 不在合同内），红证即报文如实申报，按合同 key 形对齐重写后 score 一次过，响应原文零改动只改 key 字段
- 标定账本落 /tmp 副本（mcpsec 与 mcpopen 先例同形）：temp_probe score 账本落点随脚本目录解析，本批在 /tmp/mcpauth-cal 副本实跑使主树与工地标定账本零污染零直写；temp_probe 依赖 probes 母目录模块与 atom.yaml，以 PYTHONPATH 注入加副本目录结构仿真承载，退役件本体零改
- 委外单席位申报：命题起草与九发作答同席 ZCode:GLM-5.3:self-reported 无跨席隔离，topic authored 行与谱系披露节双载
- contract-snapshot 快照申报：facet/contracts/m-mcpauth-1/ 五件不在任务包第五节 allow 面，承 mcpsec 先例预防性以快照形复制入 allow 覆盖的 proposition/DES/m-mcpauth-1/contract-snapshot/，facet/contracts 原位随工地拆除散失如实申报
- 其余误差零申报：ask3 双门首跑绿、叩问十二词与十二信号一致、租约九锁一次全取零 wait-turn 实发、管线认证未复跑、sign 一次过未复踩闸三坑位

## 链笔与收约 {#settle}

- 意图笔 intent_refined 0cd9e7cbb5c211d1（session 9895d6695cf45290）；终签笔 crosscheck_completed a4f11822dc65383c index 146 经 confirm 确认；认证 append 四笔即管线报告 8e9ca9bb 与计分材料 56eaf042 与 check 报告 78b92a8c 与 signcheck 71085170
- 双仓 settle：tools 40cb0be9（段 1，cert 71085170，基 integral-stage-build）与 engine 24c48b0（段 1，cert 71085170，基 main）与 engine 738fd3d（段 2，close 首跑卫生闸红证补笔），两仓 commit 均指向登记 worktree
- 吞笔申报：段 2 提交面只含当时 staged 的红证件，本档 settle 行回填未入 stage 即段 2 落档为占位旧版，非工具缺陷系操作次序失察，回填内容经段 3 补回，零信息损失申报在案
- 链 verify：status valid 151 笔（首哈希 25989caf 末哈希 71085170，即本批 signcheck 认证笔）
- reconcile 双仓：unrouted 0 与 cert_missing 0 即验收双零达成，reconcile 退出码 1 由跨批存量 alarm 族承载（tools unbypassed 125 加 session_orphan 19、engine unbypassed 90 加 session_orphan 24，俱先于本批在册候裁），本批增量零 bypass 零 unrouted 零 cert_missing
- 收约读数与回锚：close 收据落 lease/ledger/receipts/mcpauth-solo.json，收约后读数（revoked 与双工地拆与分支删）入完工回报转述；批后回锚重跑五行锚入完工回报
