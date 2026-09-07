# anchorskill-solo：注入式回锚触发层钩子退役转 skill 壳

> 治理任务包（实装类＋立文类合一，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「你直接做掉」即本会话换壳裁定的执行令；钩子经用户于 ZCode 客户端卸载退役即 hooks.events 空置，触发层换行为承载，anchor.py 机器核零改动
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- UserPromptSubmit 钩子持续报错经用户卸载退役，回锚触发层缺位；hookfix-solo 修的是路径解析与包裹形，本轮报错在客户端集成面且根因未定位，脚本本体实跑健康即 0.5 秒退出码零与严格单键与五行正形
- 回锚义务不随触发层退役消失：启动节律无回锚步、批结算无回锚挂点、完工报告无回显，行为承载的静默失效无补偿位
- 钩子退役与换壳决策与补偿机制停于会话对话，未落契约与命令面档

## 二、关键设计 {#design}

- 触发层三件：会话启动自检后回锚一次（sihankor-attnanchor skill 壳，AGENTS.md 启动节三件套改四件套）；批结算收约后回锚一次（BATCH-FACE 增挂点节）；完工报告回显当日五行锚（静默失效的机械补偿，缺锚可事后对表即 grep 即查）
- 承载层换、机器核不换：anchor.py 与五行内容与降级可见条款零改动，读钟例外条款不变，推式每回合注入终局仍归 pk-077 自有运行时远景
- 换壳记档走 CONTRACT 修订二（hookfix-solo 修订一先例同形），钩子报错根因如实记未定位

## 三、工作清单 {#work}

### Cluster 1：换壳件

- [ ] .agents/skills/sihankor-attnanchor/SKILL.md 新增（调用壳：触发位、调用命令、读数规则）
- [ ] AGENTS.md 会话启动节加回锚步
- [ ] sih-tools/BATCH-FACE.md 增回锚挂点与完工回显节
- [ ] sih-tools/attnanchor/CONTRACT.md 修订二
- [ ] .session-anchor.md 任务锚切换一写

### Cluster 2：机械链

- [ ] 三问双门、叩问 digest、正身、租约开锁、书简意图入链
- [ ] 机器判据复算：anchor.py 同日双跑逐字节一致退出码恒零
- [ ] 管线三步、书单对表、认证、双仓 settle、放锁收约、对账对表、链 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 换壳四件落位 | 治理 | skill 壳件在位且 frontmatter 合形；AGENTS.md 启动节含回锚步；BATCH-FACE 含回锚挂点节；CONTRACT 修订二在档；钩子退役现态核验即 hooks.events 空置如实记档 |
| **F-2** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-3** 回锚可复算 | 跨族治理 | anchor.py 同日双跑退出码全零且输出 cmp 逐字节 IDENTICAL；skill 壳命令形实跑一次出五行正形 |
| **F-4** 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，链 verify valid，reconcile 较批前零新增 |

## 五、必读文件 {#read}

- 契约：`sih-tools/attnanchor/CONTRACT.md`（机器形态与修订一先例）
- 命令面：`sih-tools/BATCH-FACE.md`（watch 对表挂点节即本批挂点节先例同形）
- 壳形态先例：`.agents/skills/sihankor-meter/SKILL.md`

## 六、约束 {#constraints}

1. anchor.py 机器形态与五行内容零改动；钩子报错根因不在本批定位，如实记未定位
2. 他批活面零触碰：confpreempt-solo 与 callloghyg-solo 在途未持锁，其工作树与写入面零碰
3. 主树零直写即待提交件经工地 settle 通道（无仓版控件 AGENTS.md 与 .agents/ 与 .session-anchor.md 除外，原地改随批如实申报），链文件只经引擎 scribe 写位
4. 守卫在位禁 plain git commit，close 通道提交；收约补笔走 bypass 登记
5. 在盘无主件与 watch 呈报件（anchor.py 权限位变更与 calls.ndjson 活面）不豁免不代清
6. pk-077 远景零触碰

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-4 全过
- [ ] 双仓 settle 提交号在档，收约后零本批活跃锁零本批活跃会话
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 anchorskill-solo-results.md 落 event/plan 且完工报告回显五行锚

## 八、风险点 {#risks}

- close 归并撞主树批输入件（任务包与报告件）未跟踪活面即按备份让位归并对表法四步处置
- AGENTS.md 无仓版控零链锚，本批只做最小增量编辑并归档件落引擎工地，居所问题归 pk-070 不属本批

## 九、范畴排除 {#exclusions}

- 不修钩子报错根因；不动 anchor.py；不动 .zcode/config.json（用户已卸载，只读核验）；不注册新术语；不触碰他批活面与 pk-077

## 十、关联文件 {#related}

- 契约：`sih-tools/attnanchor/CONTRACT.md`
- 远景：pk-077（MCP 实装远景件：注入通用化终局与自有运行时）
- 工具：`sih-engine/target/debug/scribe`（intent 与 append 写位）

## 十一、请求写入 {#requested-writes}

- AGENTS.md
- .agents/skills/sihankor-attnanchor/
- .session-anchor.md
- sih-engine/sih/state/plan/anchorskill-solo.md
- sih-engine/sih/event/plan/anchorskill-solo-results.md
- sih-engine/sih/event/plan/anchorskill-solo-materials/
- sih-engine/sih/event/trail/2026-09-07.ndjson
- sih-tools/attnanchor/CONTRACT.md
- sih-tools/BATCH-FACE.md
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/anchorskill-solo
- worktrees/sih-engine/anchorskill-solo
