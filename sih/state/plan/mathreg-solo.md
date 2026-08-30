# mathreg-solo：数学仓修表与登记批（修复计划批三）

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图即 2026-08-30 链事件 1ab0a4c1，用户同日批准四批修复计划并定序批三先行
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

摸底批（mathprobe-solo）证实数学仓检索路径失真（mapping 七行已建标待建）、algebra 子仓在顶层 INDEX 与 README 无户籍、核阅包 des-001-mathe 覆盖域双重排除 calculus 全部 113 条且不兼容工作树路径、ai-ex 存在失实记载（13 条目 0 违规）、摸底报告自身有一处误报（ALG-007 未登记）。表与登记面的伤不修，批一至批四的条目修复没有可靠的核验地基。

## 二、关键设计 {#design}

只动表与登记面共七件，条目实体零触碰。mapping 七行待建改已建按 INDEX 实况回填；顶层 INDEX 与 README 子仓清单四改五补 algebra；核阅包升 0.2.0 即 include 改通配段兼容工作树加 calculus 改真实路径加 exclude 删 llm-friendly-build 整目录排除（mapping 与 INDEX 与 README 已单列排除不依赖整目录排除）；验收即包修后对全量 126 条直跑不再借镜像目录。ai-ex 勘正不改原文独立留痕。摸底报告只增附记节不动正文。章程修订只起草交签。

## 三、工作清单 {#work}

- [ ] mapping 七行状态回填
- [ ] 顶层 INDEX 与 README 补 algebra 户籍
- [ ] des-001-mathe manifest 升 0.2.0
- [ ] 包修后全量 126 条直跑核验并记录违规总账
- [ ] ai-ex 勘正记一篇加索引行
- [ ] 摸底报告附记（历史重构加误报撤回）
- [ ] 章程修订草案起草
- [ ] 三管、认证上链、三仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 表实相符 | 工程 | mapping 七行与 INDEX 与实体三处一致，顶层两件含五子仓，algebra INDEX 零改动 |
| **F-2** 包域修复 | 工程治理 | 新包对数学工作树条目直跑域匹配成功，calculus 113 条入域，全量违规总账在档 |
| **F-3** 勘正如实 | 治理 | ai-ex 勘正记在且原文零改，附记含 ALG-007 误报撤回，章程草案成文未落 |

## 五、必读文件 {#read}

- 必读 1：sih-math/docs/survey-mathprobe-2026-08-30.md 即摸底报告
- 必读 2：sih-tools/scrutinator/packs/des-001-mathe/manifest.toml 即现行包域
- 必读 3：sih-engine/src/scrutinator 域匹配语义即 src/scrutinator/domain.py

## 六、约束 {#constraints}

1. 条目实体零触碰
2. 已认证报告正文零改动，只增附记
3. 章程草案不落，落须用户签
4. 链在他会话锁下即等待不绕行（本批已排队十一分钟实证）
5. 零 LLM 调用于工具执行层，词债不过夜

## 七、请求写入 {#requested-writes}

- sih-math/llm-friendly-build/mapping.md
- sih-math/INDEX.md
- sih-math/README.md
- sih-math/docs/survey-mathprobe-2026-08-30.md
- sih-tools/scrutinator/packs/des-001-mathe/manifest.toml
- sih-engine/sih/state/plan/mathreg-solo.md
- sih-engine/sih/state/plan/mathreg-solo-results.md
- sih-engine/sih/event/plan/mathreg-solo-results.md
- sih-engine/sih/event/plan/mathreg-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- ai-ex/MATH-AUDIT-CORRECTION-2026-08-30.md
- ai-ex/INDEX.md

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 全量违规总账入档，认证入链，三仓段结算收约
