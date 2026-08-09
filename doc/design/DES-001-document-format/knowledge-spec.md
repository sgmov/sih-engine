# 知识库文档特异化规范

本规范承载 `doc/knowledge/` 目录下 KNOW-NNN-slug.md 知识库文档的类型特异格式规则。按司衡哲学结构收敛对照的双视角论证，结构性元规则归决策文档，类型特异规范归本规范。本规范承接仓库结构决策与通用格式规范设计，定义知识库文档的必选章节、归位判据、与其他类型的关系与 wiki 预备仓目标。

本规范的元依据是仓库结构决策。本规范本身是设计文档，遵守通用格式规范设计与本规范自身的约束。

知识库文档承接知识工程定位：根据治理痕迹、经验、项目细节组织为资深开发专家级建议，供所有能访问的开发者使用。此定位于 2026-07-17 命名审查会话明确。

## 概览 {#overview}

- 知识库文档必选章节覆盖外部引用、经验沉淀、术语映射三类之一::[结构](#structure)
- WRITING-KNOW-001 至 WRITING-KNOW-003 校验必选章节存在性::[必选章节](#required-sections)
- 知识库归位判据承接知识聚合非内部架构或治理规则的内容核心::[归位判据](#classification-criteria)
- 知识库与 ARC、GOV、DES 的分工是源素材、流程约束、工程实现对照知识加工::[与其他类型的关系](#vs-others)
- 知识库的未来目标是 OKF 类型的 LLM wiki 预备仓，消费模式为渐进式披露::[OKF wiki 预备仓](#okf-wiki-prep)
- WRITING-KNOW-001 至 WRITING-KNOW-004 是 CI 校验规则全集::[CI 校验规则汇总](#ci-rules)
- 本规范承接仓库结构决策与通用格式规范设计::[关联](#relation)
- 本规范为 external-anchor 与 design-corollary 混合，附可证伪条件::[认识论立场](#epistemic-stance)

## 结构 {#structure}

知识库文档承载知识聚合产物，承接知识工程的工程定位。知识库文档必选章节如下，承载外部引用、经验沉淀、术语映射三类之一。

- 外部引用：竞品分析、技术文章、行业实践，引用样式链接汇总至文末
- 经验沉淀：项目过程中积累的经验和教训
- 术语映射：司衡术语定义与外部概念映射，承载术语表或 glossary

## 必选章节 {#required-sections}

本节承载知识库文档必选章节的存在性校验，WRITING-KNOW-001 至 WRITING-KNOW-003 是 CI 校验规则。

- WRITING-KNOW-001 校验必须包含「外部引用」或「经验沉淀」或「术语映射」三类之一
- WRITING-KNOW-002 校验外部引用必须使用引用样式链接，汇总至文末
- WRITING-KNOW-003 校验术语映射必须使用定义列表格式

## 归位判据 {#classification-criteria}

知识库文档归位判据如下。

归位判据
: 一份内容核心是知识聚合，即外部引用、经验沉淀、术语映射，非内部架构或治理规则，则为知识库文档。

## 与其他类型的关系 {#vs-others}

知识库是司衡的重大交付成果之一。根据治理痕迹、经验、项目细节所有的文本材料和资产，组织并且给到所有能够访问到 KNOW 的开发者一个资深开发专家级的建议。

- 与 ARC 的关系：ARC 是系统本体结构即源素材，KNOW 是知识加工产物
- 与 GOV 的关系：GOV 是治理规则即流程约束，KNOW 是经验沉淀，不约束引擎行为
- 与 DES 的关系：DES 是工程实现思路即怎么建，KNOW 是外部资产聚合即参考

## OKF wiki 预备仓 {#okf-wiki-prep}

知识库的未来目标是 OKF（Open Knowledge Foundation）类型的大语言模型（Large Language Model，以下简写为：LLM）wiki 预备仓。消费模式为渐进式披露，按 LLM 或开发者需要的粒度提供知识单元，不是一次性打包消费。

本节承接哲学仓的 `sih-philosophy/knowledge-absorption-process.md`，如该文档存在。

## CI 校验规则汇总 {#ci-rules}

CI 校验规则覆盖知识库文档的类型特异校验项，不重复通用格式规范设计已承载的字符集、标题、锚点校验。WRITING-KNOW-001 至 WRITING-KNOW-004 是 CI 校验规则全集。

- WRITING-KNOW-001 校验必须包含「外部引用」或「经验沉淀」或「术语映射」三类之一
- WRITING-KNOW-002 校验外部引用必须使用引用样式链接，汇总至文末
- WRITING-KNOW-003 校验术语映射必须使用定义列表格式
- WRITING-KNOW-004 校验术语映射的术语表必须包含「术语 + 定义 + 来源」三要素

## 关联 {#relation}

- 元规则：DEC-001
- 通用规范：DES-001
- 体系定义：DES-006，若涉及系统本体的知识应归 ARC 而非 KNOW
- 哲学对应：道家术语到工程概念的映射，承接 `sih-philosophy/convergence/00-terminology-mapping.md`
- 工程参考：`SETSP/engineering-bridge/meta/07-terminology-lineage-术语血统工程化.md`

## 认识论立场 {#epistemic-stance}

本规范为 external-anchor 与 design-corollary 混合。

- 外部引用部分为 external-anchor：引用外部已存在的资料
- 经验沉淀部分为 design-corollary：组织加工是工程设计选择
- 术语映射部分为 external-anchor：术语来源有经典文献

可证伪条件：若知识库文档失去引用来源即外部资产失效，或组织加工失去专家级建议的价值，则知识库作为「重大交付成果」失效，需重新评估其角色。
