# constclear2-solo：魔法数字清账批一——34 件两态路由表（账面盘点）

> 治理任务包（立文类＋清账类合一，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：用户 2026-09-08 令与口径令（原文照录）——「不能留任何拍脑袋的，要么是数学模型（优先），要么是工程实践（司衡引擎已经验证过，但是也要回归数学模型）」；令源与账面基线即前批 constclear-solo（2026-09-04 三分账，推导档 sih-math/docs/constclear-derivation-2026-09-04.md）与 pk-053 出泊笔 41fb339c（冻结态撤销，23 件全数重新路由）
> 日期：2026-09-08，会话 sess-zcode-20260908-fork2-constclear2

## 一、问题陈述 {#problem}

- pk-053 出泊撤销冻结态后，23 件冻结件处于"必须重路由而无路由表"的悬置态；两态制口径（数学模型优先，工程实践须已验证且负回归义务）较 09-04 三态制收紧，冻结不再合法
- 09-04 推导 8 件的载体引文与源码常数位未经批后复验（源码四天演进存在漂移可能）；改判 3 件的 env-params overlay 已在账但 RECLASS_INHERIT 未收编债未呈裁
- C4 判据（数学归因长尾收口）沉底 gap 5 天，其收口距离无机械读数

## 二、关键设计 {#design}

- **两态制路由口径**（承 pk-053 出泊 ruling 原文）：数学模型态＝载体条目实文承载该常数取值的判定语义，锚形限于族惯例锚或约束实例化锚，过 M-4（引文逐字节落条目文件与行号），待裁/挂起状态作注记不阻归因（facet ALPHA 先例同形）；工程实践三件套态＝引擎验证证据（在役批/测试/金向量/链上终签可指认）＋回归数学模型路径（逐件登记数学化义务与归位载体族）＋登记行（本批路由表行即登记）。零第三态，零冻结残留
- **批一边界**：零改在役代码值零改泊件零改已结算档；路由表落 sih-math/docs/ 双形态（md＋json）为登记面本体；critsweep registry C4 令牌自登记（constclear2 族）使判据扫可见本批活动
- **产出三件**：路由表（34 件逐件两态归属携可复算证据）、后继批拆分计划（清账批二登记面落位与批三代码注记候选与 pk-049 联动件）、C4 判据差距读数

## 三、工作清单 {#work}

- [ ] 核验：推导 8 件载体引文逐字节复核＋34 件源码常数位现值脚本核验（verify_constants.py 落材料，输出即证据件）
- [ ] 施工：路由表 md＋json 落 sih-math 工地；critsweep registry C4 令牌更新落 tools 工地
- [ ] 后继批拆分计划与 C4 差距读数入结果档
- [ ] 全链机械序走 BATCH-FACE（三问双门→叩问→正身→租约三仓→意图→施工→管线三步→checkcite→认证→三仓 settle→放锁 close→对账对表→verify→回填 bypass→CALL-LOG）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 推导八件核验归档 | 数据治理 | 8 件载体引文行逐字节命中（PROB-010 L12/L16/L24、PROB-005 L32、PROB-015 L61、PROB-013 L24、PROB-003 L7），34 件源码常数位现值与 09-04 档载逐件一致，核验脚本输出落材料，漂移逐件如实申报 |
| **F-2** 改判三件呈裁表 | 治理 | 改判 3 件呈裁表在档（env-params overlay 三行现状核验＋RECLASS_INHERIT 未收编债照录），零代裁候人节点裁 |
| **F-3** 冻结廿三件两态路由全覆盖 | 数据治理 | 23 件逐件恰一态零残留：数学模型态逐件载条目＋行号＋锚形，工程实践态逐件三件套齐（证据可指认＋回归路径载归位载体族＋登记行）；路由表 md＋json 双形态在档，每行携核验脚本证据可复算 |
| **F-4** 写入仅 allow 与零改值 | 治理 | 写入仅请求写入节所列路径；在役源码与在泊件与已结算档零触碰（核验脚本只读证） |
| **F-5** 链面全绿 | 治理 | 三仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增；C4 判据扫读数入结果档 |

## 五、必读文件 {#read}

- 账面基线：`sih-math/docs/constclear-derivation-2026-09-04.md`（§1 口径、§2 推导八件、§3 改判三件、§4 冻结廿三件、§5 env-params）
- 出泊令源：当日链 parking_exited 41fb339c（pk-053 ruling 原文）
- 命令面正典：`sih-tools/BATCH-FACE.md`（全文与全部坑位勘误）
- 载体条目：`sih-math/probability/entries/PROB-010 与 PROB-005 与 PROB-015 与 PROB-013 与 PROB-003`
- 先例：`sih-engine/sih/event/plan/critsweep-solo-results.md`（内容清单认证形与三态路由先形）

## 六、约束 {#constraints}

1. 零子代理；anchor.py 与 .zcode/config.json 与 .session-anchor.md 零触碰（多窗竞态在案，任务标识以 ask3 session_id 为准）
2. 在役源码与在泊件 JSON 与已结算档（constclear-derivation-2026-09-04.md 本体）零改值零改写；sih-math 管线脚本零触碰
3. 主树零直写；禁 plain commit；禁管道掩退出码；先红留痕；md 认证走内容哈希清单形
4. 出泊唯人节点：改判三件与后继批拆分皆呈裁不代裁；路由判定只列事实与锚不列建议（PRO-07）
5. watch 无主件（CALL-LOG 投影腿族七件）不豁免不代清；confpreempt 会话面零触碰
6. CALL-LOG 走 lease call-log append；critsweep registry 改动须双跑自证不漂他判据

## 七、验收标准 {#acceptance}

F-1 至 F-5 全过；三仓收约后零本批活跃锁零本批活跃会话；结果档 constclear2-solo-results.md 落 event/plan；路由表双形态落 sih-math/docs；完工报告回显当日锚行。

## 八、风险点 {#risks}

- 源码四天演进或致常数位行号漂移：核验脚本按常数名＋值匹配不按行号硬断，漂移逐件记录不阻断
- 三仓 settle 首用（math 工地）：open 后核对 repos 回显与请求写入节逐仓对表（confrevise 勘误）
- registry C4 令牌扩面若动他判据读数即双跑对照发现并回退

## 九、范畴排除 {#exclusions}

- 值级推导落档与代码注记与任何数值变更属后继批（批一零改值）；改判三件的裁与不裁候人节点；pk-049 联动件不并批；env-params RECLASS 收编属后继批；置信度线载体立项（pk-078）不属本批

## 十、关联文件 {#related}

- 出泊令源：41fb339c（pk-053）；改道件：pk-078（残余测量数学载体，置信度线 gate）
- 判据：GOV2-C4-math-attribution（critsweep registry）

## 十一、请求写入 {#requested-writes}

- sih-math/docs/constclear2-routing-2026-09-08.md
- sih-math/docs/constclear2-routing-2026-09-08.json
- sih-engine/sih/state/plan/constclear2-solo.md
- sih-engine/sih/event/plan/constclear2-solo-results.md
- sih-engine/sih/event/plan/constclear2-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/critsweep/registry.json
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/constclear2-solo
- worktrees/sih-engine/constclear2-solo
- worktrees/sih-math/constclear2-solo
