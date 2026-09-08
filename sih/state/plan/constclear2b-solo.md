# constclear2b-solo：constclear2 批二——登记面落位＋收编＋新账扫描

> 治理任务包（立文类＋清账类＋收编类合一，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：用户 2026-09-08 令「执行 constclear2 批二（登记面落位＋收编＋新账扫描）」；三席裁定终签在当日链——A1 4073c5fa（pk-063 双通道出泊）、A2 15f9a0ad（改判三件收编且类目不豁免取值依据）、A3 1f03de72（批二三件与批三界线）；P1/P2 双签 67353ed9/f4b8f93f；pk-053 出泊 41fb339c 两态口径续承
> 日期：2026-09-08，会话 sess-zcode-20260908-fork2-constclear2b

## 一、问题陈述 {#problem}

- A1 席裁 pk-063 出泊 promoted 已落笔（472ec2b4）但 *-exit.json 与材料件 state 未随落——出泊腿缺账面尾巴；A1 双通道判定标准（能数学化挂数学载体、纯约定走登记面不伪装）须随本批登记面落位承接地
- 改判三件（r1/r2/r3）账面类目待收编：env-params 三行系 overlay 形，RECLASS_INHERIT 未收编（09-04 档 §5 债）；A2 裁类目收编不豁免取值依据——三件缺省值逐件补两态依据
- rev3 账本（09-04 基线）未覆盖 09-04 后新批常数族（GC_K_SIGMA/GC_CUSUM/GD_UNION/GD_SUSTAINED/GD_MONOTONICITY/RHO_BOUND 等）：新账未扫描未路由，C4 收口距离缺更新读数

## 二、关键设计 {#design}

- **pk-063 出泊腿**：落 pk-063-exit.json（ruling 照录 A1 席签 4073c5fa 与 P1/P2 双签与出泊笔 472ec2b4 与双通道标准）＋ pk-063.json state→exited（pk-044 先例同形）
- **收编**：rev3_script.py RECLASS_INHERIT 扩表三件（依据源 09-04 档 §3，族名沿用"采样上限族"语义位），再生 env-params-rev3.json 含三行 native；再生双跑 cmp 逐字节一致（exscan 判据），与在档版 drift 逐项照录
- **新账扫描**：rev3 机械扫描重跑（输出落本批材料，sih-math 零直写），新账判定性常数逐件两态路由入批二路由增补件（constclear2b-routing）；源码内联载体声明逐件对条目与推导档实文核验（GC_K_SIGMA/GC_CUSUM_B/H→PROB-010 公理一＋PROB-013 定理一＋gchart 与 contribmath 推导档 L51 实文；RHO_BOUND→PROB-015 定理二临界值＋queueing 推导档；GD_UNION/GD_SUSTAINED→contribmath 推导档合同参数节）
- **登记面落位（A1 承接地）**：constclear-registry v1 双形态落 sih-math/docs/——纯约定与环境参数类逐件行（类目、现值、两态依据、回归义务、裁定源），数学态件指路由表不重复登记；双通道判定标准文随登记面头部落档
- **界线**：零改在役代码值（rev3_script 收编扩表为 A3 明授的账面管线变更，非在役引擎代码）；注记与改值属批三；pk-049 联动件只呈报不动

## 三、工作清单 {#work}

- [ ] pk-063 出泊腿（engine 工地）
- [ ] RECLASS 扩表＋env-params 再生＋新账扫描双跑（math 工地＋材料输出）
- [ ] 登记面 v1＋批二路由增补件（math 工地）
- [ ] C4 差距读数更新（收约后 sweep --criterion）
- [ ] 全链机械序（三问双门→叩问→正身→租约三仓→意图→施工→管线三步→checkcite→认证→三仓 settle→放锁 close→reconcile→verify→回填→CALL-LOG）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** pk-063 出泊腿 | 治理 | pk-063-exit.json 落位且 ruling 载三哈希（A1 席签、P1/P2、出泊笔）；pk-063.json state→exited；与 472ec2b4 链笔一致 |
| **F-2** 改判三件收编 | 数据治理 | RECLASS_INHERIT 扩表三件在档；env-params 再生三行 native（batch 字段 native 形）；逐件两态依据入登记面（A2 类目不豁免取值） |
| **F-3** 新账扫描入两态路由 | 数据治理 | rev3 扫描双跑 cmp IDENTICAL；新账判定性常数逐件两态路由（内联载体声明对条目与推导档实文核验，命中即数学态、无实文即工程三件套不硬凑）；路由增补件双形态在档 |
| **F-4** 登记面落位与写入边界 | 治理 | constclear-registry v1 双形态在档且双通道标准文在头部；在役引擎源码与在泊件（除 pk-063 出泊腿）零触碰；写入仅请求写入节 |
| **F-5** 链面全绿 | 治理 | 三仓 settle 提交号在档，close 零失败，verify valid，reconcile unrouted 较批前零新增；C4 差距读数更新入结果档 |

## 五、必读文件 {#read}

- 三席裁定：`sih-tools/facet/contracts/adjudicate-260908/m-adjudicate-{a1,a2,a3}/topic.md` 与 `sih-tools/proposition/DES/m-adjudicate-{a1,a2,a3}/tally-material.json`（gate_verdict stable_clear）
- 出泊笔：当日链 472ec2b4（pk-063）与 41fb339c（pk-053 口径承继）
- 账面基线：`sih-math/docs/constclear-derivation-2026-09-04.md` §3/§5 与 `sih-math/docs/constclear2-routing-2026-09-08.{md,json}`（批一）
- 推导档：`sih-math/docs/gchart-derivation-2026-09-04.md`、`contribmath-derivation-2026-09-04.md`、`queueing-derivation-2026-09-04.md`
- 命令面正典：`sih-tools/BATCH-FACE.md`（全文，含例扫调用形与坑位勘误）

## 六、约束 {#constraints}

1. 零子代理；anchor.py 与 .zcode/config.json 与 .session-anchor.md 零触碰（多窗竞态在案）
2. 在役引擎源码零改值零注记（批三界线）；rev3_script.py 仅 RECLASS_INHERIT 扩表（A3 授权面）；在泊件除 pk-063 出泊腿零触碰；pk-049 联动件只呈报
3. 主树零直写；链文件只经引擎 scribe；禁 plain commit；禁管道掩退出码；先红留痕
4. 登记面纯约定面零伪装数学（A1 双通道）；数学态件不重复登记只指路由表
5. watch 无主件（CALL-LOG 族）不代清；confpreempt 会话面零触碰
6. CALL-LOG 走 lease call-log append；md 认证走内容哈希清单形

## 七、验收标准 {#acceptance}

F-1 至 F-5 全过；三仓收约后零本批活跃锁零本批活跃会话；结果档 constclear2b-solo-results.md 落 event/plan；登记面与路由增补件与再生 env-params 落 sih-math/docs；完工报告回显锚行。

## 八、风险点 {#risks}

- rev3 再生全量输出与在档版 drift 可能非零（源码四天演进）：逐项照录不静默，checkmath 红灰判读，红即停批
- 新账扫描的判定性正则可能误捕或漏捕：扫描归确定性程序（A3），正则本批不改正则只消费其输出，漏捕在新账节如实申报
- close 或撞 CALL-LOG 族无主面：双旗 bypass 通道（批一先例同形）

## 九、范畴排除 {#exclusions}

- 代码注记与任何数值变更属批三；pk-049 联动件只呈报；REGISTRY 不收数学态件（指路由表）；critsweep registry 本批零改动（constclear2 令牌已覆盖 constclear2b 子串）；GATE_TOOLS 排除族与正则本体零触碰

## 十、关联文件 {#related}

- 上游：constclear2-solo（批一路由表）、adjudicate-solo（三席裁定）、pk063split-solo（P1/P2 双签）
- 判据：GOV2-C4-math-attribution；远景：pk-078（残余测量载体）

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/parking/materials/pk-063-exit.json
- sih-engine/sih/state/parking/materials/pk-063.json
- sih-math/docs/constclear-registry-v1.md
- sih-math/docs/constclear-registry-v1.json
- sih-math/docs/constclear2b-routing-2026-09-08.md
- sih-math/docs/constclear2b-routing-2026-09-08.json
- sih-math/docs/mathpipe-coverage-2026-09-04/rev3_script.py
- sih-math/docs/mathpipe-coverage-2026-09-04/env-params-rev3.json
- sih-math/docs/mathpipe-coverage-2026-09-04/ledger-rev3.json
- sih-math/docs/mathpipe-coverage-2026-09-04/summary-rev3.md
- sih-engine/sih/state/plan/constclear2b-solo.md
- sih-engine/sih/event/plan/constclear2b-solo-results.md
- sih-engine/sih/event/plan/constclear2b-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/constclear2b-solo
- worktrees/sih-engine/constclear2b-solo
- worktrees/sih-math/constclear2b-solo
