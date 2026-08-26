# ask3-formalization：三问正式化文档链落地

> task-packages 治理任务
> 承接：PRO-003 提案、DEC-035 旧仓命名决策沉默参考、名实复审会话结论、COURSE-v1 第 3 段
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— 本任务包无偏离，标准 T6-D
> 日期：2026-08-22

## 一、问题陈述 {#problem}

- 三问名已立（旧仓 DEC-035 五件套）、提案在册（PRO-003）、手工降级在跑（intent-refine skill），正式化三步即 DEC 立名、DES 设计、SPEC 接口未走。
- 层级本体论五层分级停在提案（PRO-005），主线锁已按其用词但 DEC 欠账未清。
- 措辞欠账：活文档仅 intent-refine skill 一处「三问单元」，随 skill 升格一并归位。

## 二、关键设计与钉死决策 {#design}

### 2.1 已裁事项（本包不重议，直接承接）

一，PRO-003 候选三通过即从命题推导组件形态，旧仓设计沉默参考，skill 为已验证输入。二，责权拆分即责在三问（每个推演节点必须产出锚点并移交、禁止熔断为刚性约束）、权在书简与事件流（唯一写入与哈希链）。三，旁路机制不保留。四，三问与参验不合并（时序分离加哲学归属不同）。五，LLM 边界即三问不增加 LLM 调用总量只约束已有调用，调用次数度量入 DES。六，命名五件套适配即中文正式名三问、代码标识符小写 ask3repeater、内部代号 ASK3、展示名 Ask 上标三 Repeater 保留不用、历史曾用名追问引擎为反向锚点；层级为组件，层级词不入名。七，三层诘察维持已验证形态即一问析出显层、二问挖掘隐含、三问递归细化，向内诘察严禁外问。

### 2.2 文档分工

DEC-006 三问正式化决策（主线亲写）：子决策含命名五件套入册、候选三通过、责权拆分、旁路不保留、不合并、LLM 边界与度量、说话口接线形态即 intent-refine skill 升格为手动期正式载体。格式按 DES-001 决策文档特异化规范即决策集加决策后果加备选方案加关系图谱加认识论立场，子决策含决策内容决策理由关联规则，备选案用定义列表拒绝理由非空。

DEC-007 工程层级本体论决策（X2 写）：PRO-005 五层分级转决策即引擎、基础设施、组件、模块、单元五子决策加领域非层与全态两条边界，备选案承 PRO-005 三候选的已拒两案，格式同上。

DES-013 三问组件设计（主线亲写）：三层诘察协议承 intent-refine 已验证形态、意图契约产出格式指针 SPEC-005、推理锚点要点指针 SPEC-005、认知域契约即目标域支撑域深度限制、与参验时序分离与书简承接关系、说话口接线即 skill 载体与 sih 协同、facet 会话模式信号经书简回流方向登记不实装、调用总量度量方案、降级复现关系。

SPEC-005 三问接口与锚点格式规格（X1 写）：输入即自然语言意图经说话口、输出即意图契约加推理锚点数组的记录格式、锚点七字段模式承旧仓 SPEC-036 重推导不直接迁移须含哲学引文理据证据行号三件套血统、持久化即经书简 append 写入不独立持有格式、事件类型建议 intent_refined 归 record_only 指明 SPEC-004 修订随实现批、认知域深度限制字段、禁熔断即低置信节点必留、调用计数字段即 calls_in 与 calls_out。格式参照 SPEC-004。

### 2.3 验收工具链

四件新文档为引擎治理文档，走 T6 全链即核阅 des-001 包、检词 core 包、doclint 旧二进制、书简上链。

## 三、工作清单 {#work}

### Cluster 1（双子代理并行）

- [ ] X1：SPEC-005 全文按 2.2 钉死项撰写
- [ ] X2：DEC-007 全文按 2.2 钉死项撰写；三问术语包词条 JSON 草稿即 zh 三问、en Ask3Repeater、code ask3repeater、含代号与展示名说明的词条一句话、source 指向 DEC-006 承 DEC-035

### Cluster 2（主线串行验证）

- [ ] DEC-006 与 DES-013 主线亲写
- [ ] intent-refine skill 升格修订即三问单元归位为三问组件、承接 DEC-006/DES-013/SPEC-005、修订记录入档，投影同步
- [ ] 检词 register 三问词条过四查
- [ ] 四件新文档过核阅检词 doclint 三查
- [ ] 书简上链认证
- [ ] COURSE-v1 第 3 段实现状态补记
- [ ] F1 至 F8 验证与结果文件
- [ ] 双仓 commit

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 命名入册 | 数据治理 | nomenclator register 三问词条过四查零拒绝，query 三问返回 established |
| F2 措辞归位 | 数据治理 | 活文档三问单元残留为零，推导记录 PRO-003 与 PRO-005 豁免在案 |
| F3 过核阅 | 元层工具 | scrutinator des-001 对四件新文档退出码 0 |
| F4 过检词 | 元层工具 | nomenclator core 对四件新文档零违例 |
| F5 过 doclint | 元层工具 | 旧二进制对四件新文档退出码 0 |
| F6 上链 | 跨族治理 | 书简 append 认证事件入链链校验全绿 |
| F7 层级 DEC 落盘 | 数据治理 | DEC-007 含五层子决策与领域全态两条边界 |
| F8 回归 | 跨族治理 | 兄弟工具全测全绿即核阅六化格十书简二十二检词二十三路择一百二十四 |

## 五、必读文件 {#read}

- sih-engine/doc/proposal/PRO-003-ask3-component-redesign.md 全文即审议与裁决源
- sih-engine/doc/proposal/PRO-005-engineering-layer-ontology.md 即 DEC-007 转写源（X2）
- sihankor/doc/decision/DEC-035-ask3-naming.md 即命名五件套源（沉默参考）
- sihankor/doc/spec/SPEC-035-ask3-tool-spec.md 与 SPEC-036-anchor-format.md 即锚点格式沉默参考（X1）
- sih-engine/doc/design/DES-001-document-format/decision-spec.md 即决策文档格式（X2）
- sih-engine/doc/spec/SPEC-004-event-stream.md 即规格文档格式参照（X1）

## 六、约束 {#constraints}

1. 本批零代码零 src 改动零 skill 语义变更，三层诘察措辞与 intent-refine 已验证形态对齐不发明新机制。
2. 旧仓四文档为沉默参考，引用其内容须重推导表述，不挂旧仓原子 id 与路径。
3. 四件新文档遵守字符集规范即无破折号无装饰符号、全角括号仅论证引用、标题锚点齐全。
4. X1 与 X2 各只写自己那一件文档加词条草稿，不碰对方文件不碰主线文件。
5. 检词红线：新文档不得引入未登记新造词，临时概念平实语句。

## 七、验收标准 {#acceptance}

- [ ] F1 至 F8 逐条过并列表于结果文件
- [ ] 四件新文档三查全过零违例
- [ ] skill 修订同步投影
- [ ] 双仓 commit

## 八、风险点 {#risks}

- 子代理写治理文档格式失准：DES-001 规范与样例钉进必读，核阅 doclint 检词三查兜底。
- 锚点七字段重推导走样：钉死三件套血统即哲学引文理据证据行号，主线验收比对旧 SPEC-036 字段清点。

## 九、范式偏离声明 {#deviation}

无偏离。标准 T6-D：X1 与 X2 并行，主线亲写两件治理文档与 skill 修订并串行验收。

## 十、关联文件 {#related}

- 任务包源：PRO-003、DEC-035、名实复审会话、COURSE-v1 第 3 段
- 连带：PRO-005、intent-refine skill、nomenclator 术语包
- 谱系：task-packages/README.md、skills/sihankor-t6d/SKILL.md
