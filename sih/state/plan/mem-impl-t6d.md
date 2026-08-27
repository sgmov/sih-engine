# mem-impl-t6d：项目记忆组件实装批即 SDD 先行 TDD 先红后绿

> T6D-XX task-packages 治理任务
> 承接：sess-zcode-260827-memimpl 三问意图即 2026-08-27 trail 意图事件 26f6f00d、SPEC-007 冻结契约与 DEC-014 立项决策、2026-08-27 用户实装批令即契约已就位可直接开工
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：实装类单线，SDD 立文与 TDD 红绿与编码在副本主线串行，不派双子代理**
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- SPEC-007 已冻结 recall 接口与验收判据八条，实装零落地即组件名册第六席有席位无本体。
- 引擎现面缺四件即五档映射、三轴语义编排、切面记录产出、宿主命令行面；底座两件已具备即寻址 locator 四子命令与引擎 query 位。
- locator core 包全仓构建段错误即缺陷候选，须窄域包绕开并登记，不阻塞本批。

## 二、关键设计 {#design}

### 2.1 SDD 落差规格先行

SPEC-008 承 DEC-013 第一步纪律落 doc/spec，钉现面盘点、五档映射路径集、三轴语义细则、ref 形态对无行位 json 载体的补钉、排序与确定性落点、测试计划。落差钉死后才开始实现。

### 2.2 TDD 先红后绿

F-1 至 F-8 逐判据失败测试先行落 tests，红态留痕后实装转绿，红转绿记录入结果档。

### 2.3 引擎 src 实装

memory 模块落 src 即五档分类与三轴编排与切面产出，宿主命令 memgate 落 src/bin 承载子命令 recall，事件轴底座即库内 query 位直调，主题轴底座即寻址只读子进程调用，五档路径集以窄域包 memory 预筛落 sih-tools/locator/packs。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] SPEC-008 落差规格落盘
- [ ] F-1 至 F-8 失败测试落盘跑红留痕
- [ ] src/memory 模块与 src/bin/memgate.rs 实装
- [ ] 测试转绿
- [ ] locator 窄域包落盘
- [ ] 任务包与结果档落位

### Cluster 2：主线串行验证

- [ ] cargo test 全绿零警告
- [ ] 检词核查零违例
- [ ] 管线三件零发现
- [ ] 双仓免参对表退出码零

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 五档覆盖 | 功能 | 五档各至少一条真实材料命中且出处可回验 |
| **F-2** 出处可机械回验 | 功能 | 每条切面的 ref 可机械解析回源即文件行区间或事件哈希在链 |
| **F-3** 主题轴跨档 | 功能 | 同一主题词跨至少三档返回切面 |
| **F-4** 事件轴正确 | 功能 | 按事件类型过滤事实档与意图档，返回集与 query 位直查一致 |
| **F-5** 时间轴边界 | 功能 | since 与 until 对链上事件时间戳过滤正确，边界日含即含 |
| **F-6** 只报不判 | 边界 | 输出无 score 无 suggestion 无 ranking 字段 |
| **F-7** 确定性 | 功能 | 同参双跑逐字节一致 |
| **F-8** 退化不崩 | 边界 | 底座件缺席时退出码二并报缺席件名，五档载体原样无损 |
| **F-9** SDD 先行 | 流程纪律 | SPEC-008 在测试落盘之前已在档即落差先钉后实现 |
| **F-10** 红转绿留痕 | 流程纪律 | 红态输出与绿态输出入结果档即绿非宣称是测试在档 |

## 五、必读文件 {#read}

- 必读 1：SPEC-007 即接口签名与数据契约与底座与确定性与验收判据八条
- 必读 2：DEC-013 即第一步引擎开发 SDD 与 TDD 纪律，DEC-014 即立项与契约先行
- 必读 3：DEC-007 即层级归属判据即组件落 src，SPEC-006 即落差规格形态先例
- 必读 4：locator 契约与 SPEC 即条目十字段与查询语义与退出码，event_stream 即 query 位与 Event 结构

## 六、约束 {#constraints}

1. **零 LLM 调用**于工具执行层即检索与过滤与产出全确定性
2. **接口不破冻**即七参数与切面七字段与退出码语义随 SPEC-007 不动，落差补钉限 SPEC-008 且显式登记
3. **底座只读**即五档载体与 locator 与 scribegate 代码零修改，工具侧新增限窄域包数据件
4. **不接线**即消费侧三处只声明，接线批另开

## 七、验收标准 {#acceptance}

本任务包验收 = 3 项：

- [ ] F-1 至 F-10 全过
- [ ] 结果档落 sih-engine/sih/event/plan/mem-impl-t6d-results.md，管线三件认证入链，双仓段结算收约
- [ ] memgate recall 可在真实工作区跑通即五档真实材料可检索

## 八、风险点 {#risks}

- json 载体无行位与 SPEC-007 行区间条款的落差即 ref 形态补钉走 SPEC-008 显式登记，若人节点判破冻即走 SPEC-007 修订记录不静默。
- locator 子进程调用经 uv 启动即首跑建环境有延迟，索引建临时件不落工作区，确定性不受影响。
- 全仓构建段错误缺陷候选复现命令在结果档登记，修复归另批不扩本批范围。

## 九、范式偏离声明 {#deviation}

本任务包**偏离 T6-D 范式**的「双子代理实施」流程：

- 理由：实装类即 SDD 与红绿与编码同链耦合，编辑位在副本须主线串行
- 偏离：双子代理改为主线单线
- 保留：T6-D 命名约定 + F 锚定 + 跨仓同步 + 管线认证

## 十、关联文件 {#related}

- 任务包源：SPEC-007 与 DEC-014 与本会话实装批令
- 链件：sih/event/trail/2026-08-27.ndjson 即意图 26f6f00d 落位
- 后续：消费侧接线批另开待令

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md
- sih-engine/src/memory/
- sih-engine/src/bin/memgate.rs
- sih-engine/src/lib.rs
- sih-engine/tests/
- sih-engine/sih/state/plan/mem-impl-t6d.md
- sih-engine/sih/event/plan/mem-impl-t6d-results.md
- sih-tools/locator/packs/memory/pack.json
- sih-tools/scribe/reports/
- sih-tools/lease/
- sih-tools/nomenclator/CALL-LOG.md
