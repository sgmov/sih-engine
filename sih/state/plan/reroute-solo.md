# reroute-solo：C3 measure-poly 改道批——GOV-002 v2.6 换版与双出泊与判据扫登记

> 治理任务包（立文类＋测量类合一，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：用户 2026-09-08 三笔裁定照录——其一「同意改道」（C3 measure-poly 判据改道，主会建议案经裁）；其二「第二席：pk-053——34 个"魔法数字"的账全部清掉」；其三口径「不能留任何拍脑袋的，要么是数学模型（优先），要么是工程实践（司衡引擎已经验证过，但是也要回归数学模型）」
> 日期：2026-09-08

## 一、问题陈述 {#problem}

- GOV-002 v2.4 第三条退出标准「measure-poly 程序级验收」所指标的四批程序自 2026-09-04 立项后零批沉底；其灵魂前提已被 pk-044 出泊裁定关闭（独立性来源改道，A/B 切换硬前置闭合）、其躯体已散装落地（温度探针经 baseinject 退役、上下文注入经 attnanchor 落地、确定性回算经 critsweep 落地），用户裁改道
- pk-054 出泊条件挂在永不开工的批四上，悬空互指；pk-053 承载的 23 件冻结态与新口径冲突（冻结=拍脑腐藏身处），用户裁全清出泊
- critsweep registry C3 条目仍指旧判据文本，判据扫持续报沉底

## 二、关键设计 {#design}

- **GOV-002 v2.6 换版**：第三条退出标准改写为「测量面确定性化验收达成」，证据四指针即 (i) pk-044 出泊裁定与温度探针退役（链事件与 baseinject 结果档）；(ii) 上下文注入与确定性回算基础设施在役（attnanchor 与 critsweep 结果档）；(iii) 残余测量数学载体（可识别性、方差/异质性分解、校准与偏差校正）改道置信度线承载即泊位 pk-078；(iv) measure-poly-rev1 程序文档状态改注「已改道吸收」不删除。版本史 v2.6 行照录改道令源与两出泊与口径原文。换版须经得一签署即 facet 测量 m-gov002v26-sign-1 stable_clear 加执契机器终签（GOV-002 v2 先例同形，BATCH-FACE 判定与采样调用面全坑位适用：facet 从工地跑、checkcite --cited 单值、attractor verify/sign 同 cwd 相对形）
- **pk-054 出泊**：disposition closed（吸收闭项），ruling 载明探针退役已由 pk-044 裁定与 baseinject 实装承载、A/B 硬前置已闭合、批四永不开工、闭项不留悬空互指
- **pk-053 出泊**：disposition promoted，ruling 照录用户口径原文；冻结态撤销；后继 constclear2 系列批执行清账；工程实践态须逐件登记验证证据与回归数学模型路径
- **pk-078 入泊**：残余测量数学载体归置信度线，gate 即置信度线阶段立项或结算件落 event/plan（conf 令牌可机械查），related pk-054 与 pk-073，正形携 id/path/state 路由三字段
- **critsweep registry**：C3 条目更新（title 改测量面确定性化、新证据指针、令牌改 attnanchor/critsweep/baseinject/conf 族、registered_at 2026-09-04 起算史保留），改后 sweep --at 2026-09-08 复算 C3 应出 achieved 且双跑逐字节一致

## 三、工作清单 {#work}

### Cluster 1：换版与出泊（engine 工地）

- [ ] GOV-002-mainline-lock-v1.md v2.6 换版（正文第三条＋版本史行）
- [ ] measure-poly-rev1-progdoc.md 头部状态注记（event/plan 在档件就地注记）
- [ ] pk-054 出泊 closed、pk-053 出泊 promoted、pk-078 进泊（scribe park 写位）
- [ ] 得一签署：facet 测量 m-gov002v26-sign-1（stable_clear）＋执契终签落链

### Cluster 2：判据扫登记（tools 工地）

- [ ] critsweep/registry.json C3 条目更新与 tests 对照更新
- [ ] sweep --at 2026-09-08 复算 C3 achieved、双跑 IDENTICAL、pk-044 散文对照不复发

### Cluster 3：机械链

- [ ] 温故检索两步（落包前主会已跑判据扫主题零命中在档；结果档起草前按事件与时间轴再跑）
- [ ] 全链序走 BATCH-FACE：三问双门→叩问→正身→租约→意图→施工→管线三步→checkcite→认证→双仓 settle→放锁 close→对账对表→verify→回填 bypass→CALL-LOG

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** v2.6 与签署链 | 治理 | GOV-002 v2.6 在档且得一签署链笔在链（facet stable_clear＋执契终签） |
| **F-2** 三泊笔在链 | 数据治理 | pk-054 出泊 closed 与 pk-053 出泊 promoted 与 pk-078 进泊三笔在当日链，ruling 含口径原文照录 |
| **F-3** 判据扫复算 | 跨族治理 | sweep --at 2026-09-08 C3 报 achieved 新证据指针，双跑 IDENTICAL 退出码零，C1/C2/C4/C5 读数不漂 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-5** 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 较批前零新增 |

## 五、必读文件 {#read}

- `sih-engine/doc/governance/GOV-002-mainline-lock-v1.md`（v2.4 现文与版本史与 v2 签署先例）
- `sih-tools/BATCH-FACE.md`（判定与采样调用面节＋全部坑位勘误）
- `sih-tools/critsweep/CONTRACT.md` 与 `registry.json`（登记 schema）
- `sih-engine/sih/event/plan/measure-poly-rev1-progdoc.md` 与泊材料 pk-053/pk-054
- 先例：`sih-engine/sih/event/plan/critsweep-solo-results.md` 与 `anchorskill-solo-results.md`

## 六、约束 {#constraints}

1. 零子代理；anchor.py 与 .zcode/config.json 与 .session-anchor.md 与 confpreempt 面（置信度线活面）零触碰
2. 主树零直写；AGENTS.md 本批零触碰；守卫在位禁 plain commit；补笔走 bypass 登记
3. 出泊唯人节点已裁即本批 ruling 照录用户令源，agent 不添裁；pk-078 出泊条件不预设处置
4. 在泊其余件零触碰；watch 无主件不代清；scribe append 只认 JSON（md 走内容哈希清单形）
5. 禁管道掩退出码；先红留痕；工地期 SIHANKOR_CALLLOG_DOGFOOD 不开

## 七、验收标准 {#acceptance}

F-1 至 F-5 全过；收约后零本批活跃锁零活跃会话；结果档 reroute-solo-results.md 落 event/plan；sweep 首跑读数入结果档。

## 八、风险点 {#risks}

- 得一测量不收敛（boundary/near_threshold）自动转人诊断即停批呈报，不硬凑 stable_clear
- 改判据文本影响 critsweep tests 金向量：registry 与 tests 同批改、双跑自证

## 九、范畴排除 {#exclusions}

- 34 件清账本体不属本批（constclear2 系列承载）；置信度线实装不属本批；判据一二四五文本零触碰；冻结清单与范畴排除节零字节改动（v2.2 起先例）

## 十、请求写入 {#requested-writes}

- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md
- sih-engine/doc/governance/GOV-002-history-v1.md
- sih-engine/sih/event/plan/measure-poly-rev1-progdoc.md
- sih-engine/sih/state/parking/materials/
- sih-engine/sih/state/plan/reroute-solo.md
- sih-engine/sih/event/plan/reroute-solo-results.md
- sih-engine/sih/event/plan/reroute-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/critsweep/registry.json
- sih-tools/critsweep/tests/
- sih-tools/facet/contracts/
- sih-tools/proposition/DES/
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/reroute-solo
- worktrees/sih-engine/reroute-solo
