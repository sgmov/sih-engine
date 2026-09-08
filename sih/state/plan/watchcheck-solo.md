# watchcheck-solo：watch 对表与异常呈报

> 令源：用户 2026-09-05 裁定（结算批结果档第八节追记原文在档）：零 token 二值协议、通道必经、单协议处置、置信度奖惩；兼承病灶二「无租约活写零实时告警」（pk-059 批六承载面）
> 范式：T6 单线 solo，委外代理亲写零子代理
> 前置：idenlane-solo 批在役（直改链笔面是其声明面来源；先 A 后 B，避免 lease CONTRACT 撞车）

## 一、问题陈述 {#problem}

- **问题 1**：无租约活写零实时告警（病灶二）——主树直写只在收约时被动撞见，沉默期无 alarm。
- **问题 2**：直改车道（idenlane 批交付）的声明面没有对表方——改了不记笔、记了笔没改，都无人照。
- **问题 3**：异常处置现无协议载体——零 token 二值裁决（呈报、回滚、通道）缺机械入口。

## 二、关键设计 {#design}

### 2.1 对表命令（确定性，零 LLM）

- 判定式：脏文件集 −（租约锁面 ∪ 直改链笔声明面 ∪ 豁免面）= 无主清单。
- 实现位候选：lease 子命令（复用 lockdb holds 与台账读取）或独立工具，批内择一给理由。
- 输出：人话一行结论 + 无主文件清单（路径加 mtime），退出码 0 净 / 1 有主外修改。**只呈报不代裁**——回滚与通道处置是人节点动作，工具零代行。

### 2.2 豁免面（冻结登记）

- 合法活写面显式枚举：sih-engine/sih/event/trail/、sih-tools/lease/ledger/、sih-tools/meter/counts/、sih-tools/scribe/reports/；批内核实补全后冻结登记，判定常数与豁免清单零裸奔。

### 2.3 挂点

- 会话启动例行读数旁：gauge record 落链后同跑（m-exscanhook-2 例扫挂点先例，BATCH-FACE 登记节随批）。
- close 前已有关键路径脏位对表（closeguard），本命令不重复挂收约位。

### 2.4 处置协议（文档承载，非代码）

- 呈报后人节点二值：「不是我的」→ 机械回滚（git restore，本批只出操作指引不代执行）；「我的」→ 走司衡治理通道（域内管线三步加直改链笔）。协议文入 BATCH-FACE 或工具 README 随批。

## 三、工作清单 {#work}

- [ ] watch-01：对表判定实装与豁免面冻结
- [ ] watch-02：TDD 夹具三族（声明内净、无主红、豁免面不误报）先红后绿
- [ ] watch-03：例行读数挂点接线与 BATCH-FACE 登记
- [ ] watch-04：处置协议文与判定语义得一裁

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 判定谓词 | 三族夹具先红后绿：声明内净态绿、无主修改红、豁免面不误报 |
| **F-2** | 双跑一致 | 同参双跑输出逐字节一致 |
| **F-3** | 挂点 | 例行读数后同跑实测在档，BATCH-FACE 登记节在档 |
| **F-4** | 零 LLM | 命令全程零模型调用零网络 |
| **F-5** | 判定语义 | 无主判定谓词与豁免面过得一裁，near_threshold 呈用户 |
| **F-6** | 呈报形 | 输出即人话清单，人节点可二值裁决，零代裁动作 |

## 五、必读文件 {#read}

- 协议裁定：sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 第八节
- 直改笔形：sih-engine/sih/state/plan/idenlane-solo.md 与其结果档（前置批）
- 挂点先例：sih-tools/BATCH-FACE.md 例扫日扫调用形节
- 锁面读取：sih-tools/lease/src/lease/lockdb.py（holds）

## 六、约束 {#constraints}

1. 零 LLM 零网络，确定性双跑一致
2. 只呈报不代裁：回滚与通道是人节点动作，工具零代行
3. 判定语义（无主谓词、豁免面）过得一裁；判定常数与清单零裸奔
4. 与 idenlane-solo 串行：先 A 后 B，CONTRACT 撞车预防

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] BATCH-FACE 挂点登记节与 CALL-LOG 双笔随批
- [ ] 得一裁材料在 facet/contracts/，裁决结论入结果档

## 八、风险点 {#risks}

- 豁免面漏项即每批自鸣（噪声致盲）：批内以两周台账实际活写面核实补全，宁宽勿漏并如实申报
- 声明面依赖直改链笔 grep 当日 trail：量大时的性能与当日文件边界须实测

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 T6-D 命名约定、F 锚定、得一裁红线。

## 十、关联文件 {#related}

- sih-engine/sih/event/plan/watchcheck-solo-results.md（随批产出）
- sih-engine/sih/state/parking/materials/pk-059.json（批六承载面，本批交付即其首个销账项）

## 十一、请求写入 {#requested-writes}

- 实现位（lease 子命令或独立工具，批内定）与其 CONTRACT
- sih-tools/BATCH-FACE.md（挂点登记与处置协议）
- sih-engine/sih/event/plan/watchcheck-solo-results.md 与 materials/
