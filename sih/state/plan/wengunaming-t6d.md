# wengunaming-t6d：温故立名批

> T6D-XX task-packages 治理任务
> 承接：sess-zcode-260828-wengunaming 三问意图即 2026-08-28 链首事件 772f7c10、用户签令即温故与 retriever 立名通过、补三问立名与 ask3gate 修令、术语裁定即英文对归位弃工程代码标识符说法、批中追加令即 ask3gate 改名 ask3repeatergate 承家族公式
> 范式：T6-D 范式—— **偏离：立名契约类单线，主线串行不派双子代理**
> 日期：2026-08-28

## 一、问题陈述 {#problem}

温故与英文对 retriever 已由用户签署，须落档连带改写与词条登记闭环。连带两件治理欠账随批清偿：三问命名集的英文对槽位在新仓术语下未归位（承旧仓 DEC-035 签收推导），ask3gate 为未登记之名即既成事实不豁免登记。

## 二、关键设计 {#design}

### 2.1 DEC-017 立名决策

温故命名集五件即中文正式名温故、英文对 retriever、内部代号不设、展示名不设、历史曾用名项目记忆反向锚点。术语裁定入档即英文对为仓内正名（承立名本体论英文对取名理据节用词），工程代码标识符说法弃用。gate 宿主家族惯例登记即 ask3gate 与 scribegate 与 retrievergate 三件。

### 2.2 三问补全与 ask3gate 登记

DEC-006 增修订行：英文对槽位归位 Ask3Repeater 承旧仓 DEC-035，宿主二进制名随批由 ask3gate 改 ask3repeatergate 即家族公式统一无特例，旧名入死档。不改旧仓档不改历史链。

### 2.3 代码面名实相符

src/memory 改 src/retriever、宿主 memgate 改 retrievergate、测试与调用面同步，全测绿后落笔。memgate 入死档。

## 三、工作清单 {#work}

- [ ] DEC-017 落 doc/decision
- [ ] DEC-006 修订行即三问英文对归位与 ask3gate 登记
- [ ] 连带改写六件即 DEC-014、DEC-015、GOV-002 v1.5、GOV-003 v1.3、SPEC-007、SPEC-008，DEC-016 修订行即崩溃语料路径迁移注记
- [ ] 代码面三改即模块、宿主、测试，cargo 全测绿
- [ ] 检词词条登记温故与 retriever，死档登记司忆与英文四件与 memgate
- [ ] 任务包与结果档落位、管线三件零违例、双仓段结算收约、免参对表退出码零

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 立名成立 | 链上治理 | DEC-017 落档含五件套与术语裁定与宿主家族登记与血统档死因 |
| **F-2** 补全成立 | 链上治理 | DEC-006 修订行在档，三问英文对归位 Ask3Repeater、ask3gate 登记 |
| **F-3** 名实相符 | 工程治理 | 代码面三改落地且 cargo 全测绿，仓内活文档旧名归零（推导档豁免除外） |
| **F-4** 不越界 | 跨族治理 | 不改 PRO-008 与旧仓档与历史链，pk-025 不动，SPEC 文件名不改 |

## 五、必读文件 {#read}

- 必读 1：DEC-006 五件套与 DEC-035 承接关系、DEC-014 立项即工作名悬置槽位
- 必读 2：立名本体论英文对取名理据节、DEC-016 崩溃语料路径即改名影响面
- 必读 3：sih-tools/nomenclator packs/core 登记形态即词条与死档字段

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜即新词过检词
3. 人裁令源在版本行与决策正文显式引用
4. 文件零改名即只改内容加修订行

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 结果档落 sih/event/plan/wengunaming-t6d-results.md，管线三件认证入链，双仓段结算收约

## 八、风险点 {#risks}

改写面广易漏旧名残株，以 grep 清点收口；代码改名易破引用，以全测红绿守界。

## 九、范式偏离声明 {#deviation}

立名契约类单线，保留 T6-D 命名与 F 锚定与管线认证。

## 十、关联文件 {#related}

- 任务包源：用户签令与会话对谈
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 772f7c10
- 后续：句读签核与消费侧接线批均待令

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/decision/017-wengu-naming.md
- sih-engine/doc/decision/006-ask3-formalization.md
- sih-engine/doc/decision/014-project-memory-initiation.md
- sih-engine/doc/decision/015-quality-baseline.md
- sih-engine/doc/decision/016-parser-initiation.md
- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md
- sih-engine/doc/governance/GOV-003-fullstate-course-v1.md
- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/doc/spec/SPEC-007-project-memory-component.md
- sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md
- sih-engine/src/memory/
- sih-engine/src/retriever/
- sih-engine/src/bin/
- sih-engine/src/lib.rs
- sih-engine/tests/
- sih-engine/sih/state/plan/wengunaming-t6d.md
- sih-engine/sih/event/plan/wengunaming-t6d-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/scribe/reports/
- sih-tools/nomenclator/packs/core/
- sih-tools/lease/
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
