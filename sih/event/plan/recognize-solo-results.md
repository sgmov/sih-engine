# recognize-solo 批结果档：HTTP 读面连接识别域隔离与签发闸

> 会话：3cd5e72d225251f7（r2；前会话 5bb82da95e17141a 零写入轻收约，勘误重开在案）
> 令源：用户 2026-09-14 开批裁定；范式：T6 单线 solo，主代理亲写零子代理

## 一、问题与交付 {#summary}

实验侧（SiMuseor）2026-09-14 越权读汇报经开发侧核定：HTTP alpha 读面对一切连接一律投影中央第一域（绑域只约束写面），降级形未附 identity_notice，控制台签发无域自举闸，locks_read HTTP 面退出码 127。本批六件交付：

- iso-01 绑域投影：providers 闭包经 `conn_root(&c)` 取连接会话域根传入 alpha 六具（chain_query、chain_verify、critsweep、heartbeat、locks_read、retriever_recall），nomenclator_check 目标锚同步根参化。HTTP active 牌读数落所绑域，stdio 面连接根本为中央根零变化。
- iso-02 降级教学面：降级四因（缺头、不在册、已停行、登记册不可读）读域数据六具回受控教学面（identity_notice 加因码加公共字段），零域数据投影；nomenclator 三具为共享命名登记面不在闸。
- iso-03 签发域自举闸：webface issue 与 confirm-issue 两步俱验 `domain_opened` 谓词（绑根等于中央根或新城正典形 sih 树在位），未域自举根教学拒——sim-dev 病灶（牌先于域）源头收口。
- iso-04 locks_read 修复：uv 裸名子进程（launchd 极简 PATH 下 spawn 失败 127）改引擎 lease bin 直调，台账路径经域布局投影显式传参。
- iso-05 回归：mcpserver 套件 47 绿（新增闸测试四件）、端到端闸面验收 `tests/recognize_http_gate.rs` 绿（真 axum 路由真 TCP 六用例）、引擎租约甲表回归四件绿；全仓 234 过 2 失败（金向量两件为主树基线既有失败，见第五节）。
- iso-06 AI-MANUAL 第 2 节与第 8.4 节语义修订：降级形由「降只读投影中央域」改「受控教学面零投影」，未域自举绑根读写俱硬拒教学（手册同批修订在案）。

## 二、关键设计判定 {#design}

- 读面闸挂 bearer 闸（传输层，tools/call 结果形回教学面），写面域自举闸挂 write_gate_verdict（403），签发闸挂 webface 两步——三闸共用 `domain_opened` 单源谓词（httpface），零第二实现。
- 连接识别四元化（HttpIdent 增 degraded 因码与 domain_opened），请求标识单次解析复用，零二次查册。
- 根管道弃 REQUEST_IDENTITY task_local 直取形：task_local 不跨 rmcp spawn 任务（活体验收 r1 实证：工作台牌 critsweep 仍回中央根），改由连接会话承载（工厂即持 ident.root），providers 闭包传入——这是 r1 到 r2 的唯一结构性修正。

## 三、F 锚定逐条判定 {#falsifiable}

| F 锚定 | 判据 | 判定 | 证据 |
|---|---|---|---|
| F-1 绑域投影 | active 绑域牌读数落所绑域 | 过 | 活体验收：sim-aesthetic-workbench 牌 critsweep 回 root=/Users/moc/workspaces/SiMuseor/sim-aesthetic-workbench（canonical）；e2e 用例六闸放行 |
| F-2 硬拒 | 未域自举绑根读硬拒零回退 | 过 | e2e 用例四（isError true domain_not_bootstrapped）；活体验收同形 |
| F-3 教学面 | 降级形教学面零路径泄漏 | 过 | 活体验收：无牌 missing_authorization、停行牌 token_stopped，俱附 identity_notice 零投影；e2e 用例一二三 |
| F-4 写面不回退 | 写闸语义原样 | 过 | 活体验收：停行牌写 401 token_stopped；e2e 用例五未域自举写 403 |
| F-5 签发闸 | 未域自举根签发教学拒 | 过 | issue 与 confirm-issue 双步闸在码，webface 单测往返夹具改走已域自举域过闸 |
| F-6 locks_read | HTTP 面退出码 0 | 过 | 活体验收：200 出参 held_count 与 active_sessions 实数 |
| F-7 套件 | mcpserver 绿加新增回归绿 | 过 | 47 加 1（e2e）加 4（甲表）全绿 |
| F-8 甲表对表 | 引擎认领径对围堰判词一致 | 过 | 非对称判定修复加四件回归（new 段认领可过、双向错配拒） |

## 四、管线与验收回执 {#pipeline}

- 化格（formatter general-v1）：包档与结果档零改动退出码 0。
- 核阅（scrutinator des-001）：state/plan 与 event/plan 目标域外退出码 2，如实记档（BATCH-FACE 勘误位）。
- 检词（nomenclator core）：零违例退出码 0（mcpread-iso 草名期一处懒波词五处已统一改「域自举」后净）。
- 活体验收：materials/acceptance-live-2026-09-14.json（launchd kickstart 换新二进制后六用例）。
- 端到端：tests/recognize_http_gate.rs（进程内临时中央根，真 HTTP 六用例）。

## 五、异常与偏离申报 {#deviation}

- CLI identity verify 全形退出码 137（子进程采集链被杀），正身改走围堰 Python identity 件（v3 格式同源），候修另批。
- 引擎租约 --new-stem 甲表派生对表等值判定死锁（围堰 core.py 非对称判定），双跑不一致实锤——本批 iso-07 修复加回归；修复前立约曾撞闸，故批名改用既立词 recognize。
- 引擎开约 allow 面整行散文收纳致锁范围验必拒（围堰为路径抽取），双跑偏差第二处——包档九节改裸路径重开，解析位修复候租约线批。
- 主树 domaware-solo.md 批名改名时被 mv 覆盖，git restore 即时恢复原档，事故与恢复全程在案。
- 全仓两件金向量失败（golden_des001_gov002 与 cli_positional_form_matches_golden）：主树基线同跑同败，GOV-002 主线锁文档改动后金向量未更新之既有陈旧，不属本批改动面，候金向量维护批。
- P2 读面访问审计候另裁；/ 视图面板静态资产缺席教学降级维持；critsweep 出参瘦身与 tools/call 双份回显裁减候另批（越权汇报衍生优化项）。

## 六、联动改动面 {#touched}

- sih-engine/src/mcpserver/httpface.rs（识别四元化与三闸与教学载荷）
- sih-engine/src/mcpserver/alpha.rs（六具根参化与 locks_read 引擎 bin 直调）
- sih-engine/src/mcpserver/providers.rs（conn_root 助手与闭包传根）
- sih-engine/src/mcpserver/webface.rs（签发两步域自举闸）
- sih-engine/src/bin/lease/attachments.rs（iso-07 甲表非对称判定）
- sih-engine/src/tools_registry.rs（插件槽位心跳根参适配）
- sih-engine/tests/recognize_http_gate.rs（新增端到端验收）
- sih-tools/mcpline/AI-MANUAL.md（第 2 节与第 8.4 节语义，仓外直改申报）

## 七、候人节点 {#pending}

- P2 读面访问审计立批与否
- sim-dev 重发与否（SiMuseor 根域自举后走签发闸正常通道；或径用 workbench 既有牌）
- 金向量维护批与 identity 137 修复批候令
