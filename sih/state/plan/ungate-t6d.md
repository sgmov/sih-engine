# ungate-t6d：撤gate与权源补法批

> T6D-XX task-packages 治理任务
> 承接：sess-zcode-260828-ungate 三问意图即 2026-08-28 链事件 42cdbf57、用户裁定即撤 gate 与补法甲加乙
> 范式：T6-D 范式—— **偏离：修订类单线，主线串行不派双子代理**
> 日期：2026-08-28

## 一、问题陈述 {#problem}

gate 后缀公式无令源铸入 DEC-017 属未签规范即越权账在案，用户裁定撤 gate 即宿主名改裸英文对；权源核验缺口经问政确认即工具族五面零承载，补法甲加乙即前向防生成与事后机械对表，甲本批落地，乙另开批待令。

## 二、关键设计 {#design}

### 2.1 撤 gate

宿主名改裸英文对即 retriever 与 ask3repeater，模块与宿主同名同源自组件一家。DEC-017 修订一即撤公式条款，宿主家族节降格重写为撤gate裁定记录与越权账。scribegate 撞退役工具仓 scribe 之名，另议待人签即候选名随物走承 scribe。旧名 retrievergate 与 ask3repeatergate 入死档。

### 2.2 甲落地

三问注入约束固定条即决策档规范级条款须令源引用，落 intent-refine skill 修订节。

### 2.3 乙登记

路择登记冲突谓词扩域即对决策档规范词与链上令源机械对表，SDD 另开批，本批只登记待令。

## 三、工作清单 {#work}

- [ ] DEC-017 修订一与 DEC-006 修订二与 SPEC-008 修订落档
- [ ] 代码面两改名即 git mv retrievergate 与 ask3repeatergate，测试同步全测绿
- [ ] 死档两件登记
- [ ] skill 修订落位即注入约束固定条
- [ ] 任务包与结果档落位、管线三件零违例、双仓段结算收约、免参对表退出码零

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 撤gate成立 | 链上治理 | DEC-017 修订一在档即公式条款撤除与越权账与撤制裁定三件齐 |
| **F-2** 名实相符 | 工程治理 | 两改名落地全测绿，仓内活文档 retrievergate 与 ask3repeatergate 清零即历史行除外 |
| **F-3** 甲落地 | 跨族治理 | skill 修订在档即注入约束固定条可引 |
| **F-4** 不越界 | 跨族治理 | scribegate 新名不定，乙不实现，历史与链零改 |

## 五、必读文件 {#read}

- 必读 1：DEC-017 宿主家族节即被撤条款、用户撤gate令与会话问政对谈
- 必读 2：intent-refine skill 约束注入节、selector CONTRACT 即谓词面
- 必读 3：DEC-013 名随物走先例即 scribegate 候选名的令源

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜即新词过检词
3. 人裁令源在版本行与决策正文显式引用
4. 决策档规范级条款须令源引用即本批新约束先行自证

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 结果档落 sih/event/plan/ungate-t6d-results.md，管线三件认证入链，双仓段结算收约

## 八、风险点 {#risks}

修订叠加修订易版本行失序，以逐档版本行递进锚定；撤后缀后裸名与模块同名易被读作撞名，以名随物走句钉义。

## 九、范式偏离声明 {#deviation}

修订类单线，保留 T6-D 命名与 F 锚定与管线认证。

## 十、关联文件 {#related}

- 任务包源：用户撤gate令与补法令及问政对谈
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 42cdbf57
- 后续：scribegate 新名签核与乙的路择扩域批均待令

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/decision/017-wengu-naming.md
- sih-engine/doc/decision/006-ask3-formalization.md
- sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md
- sih-engine/src/bin/
- sih-engine/src/retriever/
- sih-engine/tests/
- sih-engine/sih/state/skills/sihankor-intent-refine/SKILL.md
- sih-engine/sih/state/plan/ungate-t6d.md
- sih-engine/sih/event/plan/ungate-t6d-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/scribe/reports/
- sih-tools/nomenclator/packs/core/
- sih-tools/lease/
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
