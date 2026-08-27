# scribeback-t6d：书简宿主错名回滚批

> T6D-XX task-packages 治理任务
> 承接：sess-zcode-260828-scribeback 三问意图即 2026-08-28 链事件 a9822eba、用户裁定即 scribegate 是错名本名就是 scribe、回滚实例定性
> 范式：T6-D 范式—— **偏离：回滚类单线，主线串行不派双子代理**
> 日期：2026-08-28

## 一、问题陈述 {#problem}

书简融回切换批另立错名 scribegate 即决策错误，本名 scribe 承 DEC-004 名随物走即写入位迁引擎时名应随行。本批回滚错名为本名，为 DEC-015 质量判据二即回滚点账本的首个活实例。

## 二、关键设计 {#design}

### 2.1 回滚裁定

DEC-017 修订二落档：错名出生证供保全即 SPEC-006 承先例行与出生批提交不改写，回滚点可指，回滚非立新名。

### 2.2 代码面回滚

二进制 git mv 改本名 scribe，actor 归属随批改即未来事件署名 scribe，存量链事件 actor 保持 scribegate 即历史不改写，金向量冻结件随 actor 归属重冻即新基线自本批起算。

### 2.3 连带面全查

引擎测试与 SPEC 两件与 DEC-015 与 scribe 契约与 locator 契约与 AGENTS 根域行逐件归位，scribegate 入死档 name_only。

## 三、工作清单 {#work}

- [ ] DEC-017 修订二落档
- [ ] 代码面 git mv 与 actor 改与测试同步与金向量重冻，全测绿
- [ ] 连带改写五件与 AGENTS 根域直改
- [ ] 死档一件登记
- [ ] 任务包与结果档落位、管线三件零违例、双仓段结算收约、免参对表退出码零

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 回滚成立 | 链上治理 | DEC-017 修订二在档即错名认领与回滚点与实例定性三件齐 |
| **F-2** 名实相符 | 工程治理 | 二进制本名 scribe 落地全测绿，actor 归属改 scribe，金向量重冻后对拍绿 |
| **F-3** 连带归位 | 工程治理 | 活文档 scribegate 清零即历史行与存量链除外，AGENTS 根域行归位 |
| **F-4** 不越界 | 跨族治理 | 历史链与 SPEC-006 历史行与工具侧物理目录零改，本批零泊界动作 |

## 五、必读文件 {#read}

- 必读 1：DEC-004 名随物走先例、DEC-015 判据二回滚点账本
- 必读 2：sih-tools/scribe/CONTRACT.md 修订五与六即错名承接的出生语境
- 必读 3：fixtures/golden 冻结件与 t1 重算纪律

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜即新词过检词
3. 人裁令源在版本行与决策正文显式引用
4. 决策档规范级条款须令源引用

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 结果档落 sih/event/plan/scribeback-t6d-results.md，管线三件认证入链，双仓段结算收约

## 八、风险点 {#risks}

回滚易顺手改写历史即以两不改写清单钉死；金向量重冻易被误读为破冻即以新基线起算句钉义。

## 九、范式偏离声明 {#deviation}

回滚类单线，保留 T6-D 命名与 F 锚定与管线认证。

## 十、关联文件 {#related}

- 任务包源：用户回滚裁定令
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 a9822eba
- 后续：句读待签与消费侧接线批待令

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/decision/017-wengu-naming.md
- sih-engine/doc/decision/015-quality-baseline.md
- sih-engine/doc/spec/SPEC-007-project-memory-component.md
- sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md
- sih-engine/src/bin/
- sih-engine/src/event_stream/
- sih-engine/fixtures/golden/
- sih-engine/tests/
- sih-engine/sih/state/plan/scribeback-t6d.md
- sih-engine/sih/event/plan/scribeback-t6d-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CONTRACT.md
- sih-tools/locator/CONTRACT.md
- sih-tools/nomenclator/packs/core/
- sih-tools/lease/
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
