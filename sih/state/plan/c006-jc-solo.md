# c006-jc-solo：C006 即称类手改分卷（撤回收口）

> task-packages 治理任务
> 承接：用户 2026-08-30 开批令即 c006 手改分卷
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30
> 状态：**撤回**（2026-08-30 撤回收口，承 rulecal-solo 撤回先例）

## 一、问题陈述 {#problem}

数学仓 calculus 子仓全角括号 C006 违例 1242 处，分三类：即称类（谓词含"即/称/简/全/亦/又名"）95 处、短注类 ~680 处、真内容类 ~808 处（合计 1583 与实测 1242 口径差未核）。本批为手改分卷系列首批，**取即称类 95 处**。前批 rulecal-solo 撤回教训即规则校准靠引擎级例外，规则不动；前批 fmtc2b-solo 成功模式即内容修复通道 + 核阅恰降对表。

## 零、口径精化与撤回原因 {#precision-and-reason}

任务包落盘后实测精化口径：

| 口径 | 数 | 性质 |
|---|---|---|
| 宽口径（行内含"即/称/简/全/亦/又名"任一） | 95 | 任务包原估 |
| 谓词内口径（括号内含即/称任一） | 28 | 第一次精化 |
| **真即称类**（括号内是即称语义） | **6** | **实测可改** |
| 描述/同位型（括号内是描述） | 3 | 应保留 |

撤回原因：6 处真即称类改法必然是"删括号改语义"或"留括号不降 C006"二选一——F-1（核阅 C006 恰降 95）与 F-2（零语义变化）不能兼过。fmtc2b 模式只对"X（公式引用）"有效（机械引用删括号语义零变），即称类是同位语删括号必丢"即/称"语义。

撤回后下批走法：c006-sb-solo 即短注类分卷，680 处走"X（Y 描述）→ X，Y 描述，"删括号改逗号语义零变；6 处真即称类作为留人工特例入档待下下批或规则校准。

## 二、关键设计 {#design}

段一谓词识别：grep -E "即|称|简|全|亦|又名" 扫全仓生成 95 实例清单，存 materials 目录附 5 字段（文件、行列、原文、改法、备注）。

段二改法决议：95 实例按句式选模板逐条决议——
- "X-ID 名字（Y-ID 是 Z 的应用）" 类：留人工
- "X（Y 简称 Z）" 类：模板 "X（Z）"
- "X（又称 Y）" 类：模板 "X（Y）"
- "X（即 Y）" 类：模板 "X，Y，"
- "X（Y，又称 Z）" 类：拆分或保留
- "X（Y 全称 Z）" 类：模板 "X（Y）"
- 其他：留人工带句式标签

段三改法执行：写 Python 脚本按决议机械替换 + 留人工清单不动。

段四核阅对表：复跑核阅 des-001-mathe C006 计数恰降 95，差一即回滚。其余类 S005 / M008 / C002 / N002 计数逐一不变。

段五化格检词复跑：化格退出码零，检词零新增违例。

段六书简上链：append 段结算事件，verify 链 valid，双仓收约。

## 三、工作清单 {#work}

- [ ] recall 跑过切面件落 /tmp/recall-c006-jc.ndjson
- [ ] 谓词识别脚本生成 95 实例清单落 materials/c006-jc-instances.ndjson
- [ ] 改法决议表落 materials/c006-jc-resolutions.tsv
- [ ] 改法脚本执行恰 95 实例
- [ ] 核阅 des-001-mathe 复跑 C006 恰降 95
- [ ] S005 / M008 / C002 / N002 计数逐一不变
- [ ] 化格复跑退出码零
- [ ] 检词复跑零新增违例
- [ ] 书简 append 段结算，verify 链 valid
- [ ] 结果档落 sih/event/plan/c006-jc-solo-results.md
- [ ] 双仓段结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 恰 95 清偿 | 工程 | 替换实例数 = 核阅 C006 降数 = 95 |
| **F-2** 零语义变化 | 治理 | 除括号改法外 diff 无其他变更 |
| **F-3** 其余类不变 | 治理 | S005 / M008 / C002 / N002 计数逐一不变 |
| **F-4** 检词化格零新增 | 治理 | 化格退出码零，检词零新增违例 |
| **F-5** 留痕不可篡改 | 治理 | 链 valid，事件哈希八前缀可回验 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/event/plan/fmtc2b-solo-results.md 即内容修复通道先例
- 必读 2：sih-engine/sih/event/plan/rulecal-solo-results.md 即规则校准撤回教训
- 必读 3：sih-engine/sih/event/plan/fmtcontent1-solo-results.md §遗留 即 C006 余量分卷建议

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 只动 sih-math/calculus/llm-friendly-build/entries/ 与本批 materials 与 trail
5. 改法脚本与决议清单留档可回验
6. 留人工处不静默，标句式标签

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/ 即称类命中文件
- sih-engine/sih/state/plan/c006-jc-solo.md
- sih-engine/sih/state/plan/c006-jc-solo-results.md
- sih-engine/sih/event/plan/c006-jc-solo-results.md
- sih-engine/sih/event/plan/c006-jc-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 认证入链
- [ ] 结果档落位
- [ ] 双仓段结算收约

## 九、风险点 {#risks}

- 即称类句式散布 6+ 种，模板覆盖率可能 < 60%；不覆盖处带句式标签留人工，标差数
- 改法脚本误伤短注类或真内容类；防御即改法前 grep 二次确认仅含谓词
- 1242 vs 1571 口径差未核；本批 95 实例以本批识别结果为准

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 开批令
- 链件：sih-engine/sih/event/trail/2026-08-30.ndjson（本批 append）
- 关联：fmtc2b-solo 内容修复通道、rulecal-solo 撤回教训、fmtcontent1-solo 余量建议

## 十一、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十二、后续 {#next}

- 短注类 ~680 处手改分卷（待令）
- 真内容类 ~808 处手改分卷（待令）
- 全仓 C006 归零进度跟踪
