# 任务包 013 阶段 7 divergence metric 偏离率度量


> 关闭裁定（批 pkgclose-solo，2026-08-31）
> 裁定：部分承接闭项
> 承接位：异质性度量即 Jaccard、coverage、severity 分布由 facet compiler 承接即确定性聚合、配对 Jaccard 实证 0.92 与 0.30 在档；收敛态读数由秤星承接即三维快照、convergence 维组件治理批覆盖比、ga-1，量的是治理态收敛非原包 LLM 输出偏离，如实区分；收敛裁决机械闸由得一即二层裁决结构承接；K/N、波动系数、双指标阈值公式未作为 src 模块承接
> 证据：sih-tools/facet/FACET.md:23 compiler 算 coverage、Jaccard、severity 分布；sih-tools/facet/FACET.md:33 bps0 Jaccard=0.92 与 fps5 Jaccard=0.30 实证；sih-tools/gauge/CONTRACT.md:19 convergence 维定义；sih-engine/doc/decision/020-deyi-component-naming.md:3
> 原文主体零改动，本块为批 pkgclose-solo 追加头部件


## 概览 {#overview}

本任务包实现 8 阶段路径阶段 7,在 `sih-engine` 仓加 Rust 偏离率度量模块,确定性计算 K/N 偏离率、Jaccard 相似度、波动系数、收敛双指标判定。承接 DES-005 v2 偏离率度量机制与 SPEC-001 v2 数据契约与 DES-010 多专家范式与 DEC-005 子项一(263 文档集复用边界)。

- 任务目标::[目标](#task-objective)
- 项目背景::[背景](#project-background)
- 任务构成规则::[规则](#task-rules)
- 参考文件::[参考](#references)
- 交付要求::[交付](#deliverables)
- 边界::[边界](#boundary)

## 任务目标 {#task-objective}

实现 `sih-engine/src/divergence/` 子模块,提供偏离率度量入口函数,确定性计算 K/N、Jaccard、波动系数、双指标判定结果。承接 DES-005 v2 偏离率段 L58-66 与 SPEC-001 v2 数据契约段 L34-72。

子目标分五块。

第一块,实现 `DivergenceRate` 结构,沿用 SPEC-001 v2 「偏离率数值结构」段 L46-50,含 k、n、rate、attempt_id、expert_count 五字段,rate 保留四位小数。

第二块,实现 `JaccardSet` 结构,沿用 SPEC-001 v2 「Jaccard 集合结构」段 L54-58,含 set_a、set_b、jaccard_index、pair_label 四字段,jaccard_index 保留四位小数。

第三块,实现 `ConvergenceIndicators` 结构,沿用 SPEC-001 v2 「收敛双指标结构」段 L62-66,含 volatility_coefficient、jaccard_mean、volatility_pass、jaccard_pass、converged、threshold_volatility、threshold_jaccard 七字段。

第四块,实现 `DivergenceExperimentOutput` 结构,沿用 SPEC-001 v2 「偏离率实验输出结构」段 L70-72,含 document_id、check_items、divergence_rate、jaccard_sets、convergence_indicators、timestamp、model_version 七字段。

第五块,实现度量编排函数 `measure(collection: &AuditUnitCollection, targets: &[SemanticNode], config: &MeasureConfig) -> DivergenceExperimentOutput`,按 `MeasureConfig` 配置计算 K/N 与 Jaccard 与波动系数与双指标。

## 项目背景 {#project-background}

DES-005 v2 偏离率段 L58-66 明确,偏离率是路径二的核心可复现度量,承接 SPEC-003 L190 原则 LLM 是材料生成器不是度量权威。计算公式 K 等于命中偏差的单元数,N 等于产出物总单元数,偏离率等于 K 由 N 除。计算由确定性程序执行,可复现。

DES-005 v2 收敛判定段 L68-76 明确,收敛双指标含 K/N 波动系数低于阈值与审阅单元集合 Jaccard 相似度高于阈值,两个指标都过才算收敛,任一不过不算。参数阈值留空,待实验数据填入。

DES-010 实验方法段 L57-92 明确多专家范式度量:K/N(单专家时 K 等于该专家命中偏差数,多专家时 K 等于并集命中偏差数,任一专家命中即计入 K)、Jaccard 相似度(两两配对聚合取平均)、边际收益(每增加 1 个专家的稳定性提升)。

DEC-005 子项一判定「部分复用」,旧 Phase 2 实验数据中可复用部分是 263 份文档集与八大类型分层与失败标记清单十份与可复现性五要素,不可复用部分是 N=4 隔离采样协议与判断分布方差数据与基础置信度公式与 7392 条判断记录。本任务包消费 `AuditUnitCollection` 是新结构化数据,符合 DEC-005 不可复用部分须重新收集判定,旧 7392 条记录不能直接喂入。

上游依赖。阶段 6 NPC orchestration,task-012-stage-6-npc-orchestration.md,产出 `AuditUnitCollection`。阶段 3 semantic tree,task-009-stage-3-semantic-tree.md,产出 `SemanticNode` 用于 N(总单元数)统计。

下游承接。阶段 8 calibration 实验,本任务包产出的 `DivergenceExperimentOutput` 是 SPEC-001 v2 偏离率实验的输入数据结构。

## 任务构成规则 {#task-rules}

### self-check 六项 {#self-check}

1. 主动判断产出性质,这是 divergence metric Rust 模块实现,不是 DES 设计
2. 服务原始意图,实现 K/N 与 Jaccard 与波动系数与双指标,不发明新度量
3. 范畴排除显式,只动 src/divergence/ 与 src/npc/(只读消费)与 src/semantic_tree/(只读消费),不破 src/judge/
4. 不逃避当下责任,波动系数与 Jaccard 阈值留空须显式提示,不能填默认值
5. 每行去掉会犯错吗,度量结果必须可复现(同输入同输出)
6. 与哲学仓相容,承接工程基线第一确定性程序承载

### 通用格式规范 {#general-format}

子代理写 Rust 代码不必遵守 DES-001,DES-001 只约束文档类产出。本任务包本身的 markdown 文档须遵守 DES-001。实现完跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0,同时 `cargo build` 与 `cargo test` 须通过。

### 类型特异化约束五条 {#type-specific}

1. 结构特异非字符集,五数据结构与 measure 编排是结构层
2. 特异化需支撑,五字段引用 SPEC-001 v2 「偏离率数值结构」L46-50 等四段,Jaccard 公式引用 DES-005 v2 L72,波动系数公式引用 DES-009 P1-1 L122
3. 不照抄通用,度量结构不复用 stage 4 `Violation`,度量与违规是不同范畴
4. 不重复定义,不重写 DES-005 v2 与 SPEC-001 v2 已定义公式与结构
5. 最小化原则,只实现度量计算,不做实参定参(阈值留空)

### 哲学检索 {#philosophy-search}

承接工程基线第一条确定性程序是治理操作唯一执行者(K/N 与 Jaccard 与波动系数由确定性程序计算)与工程基线第四条可验证性约束(度量结果可机械校验)与工程基线第五条 LLM 是材料生成器不是度量权威。本任务包无新哲学命题。

### Rust 实现约束 {#rust-constraints}

- 公共 API 暴露 `DivergenceRate`、`JaccardSet`、`ConvergenceIndicators`、`DivergenceExperimentOutput`、`MeasureConfig`、`measure` 六个公共项
- 五个数据结构字段顺序与类型对齐 SPEC-001 v2
  - `DivergenceRate` 结构字段,`k` u32、`n` u32、`rate` f64、`attempt_id` 字符串、`expert_count` 专家数枚举
  - `JaccardSet` 结构字段,`set_a` 字符串向量、`set_b` 字符串向量、`jaccard_index` f64、`pair_label` 字符串
  - `ConvergenceIndicators` 结构字段,`volatility_coefficient` f64、`jaccard_mean` f64、`volatility_pass` 布尔、`jaccard_pass` 布尔、`converged` 布尔、`threshold_volatility` Option f64、`threshold_jaccard` Option f64
  - `DivergenceExperimentOutput` 结构字段,`document_id` 字符串、`check_items` 字符串向量、`divergence_rate` 偏离率结构、`jaccard_sets` Jaccard 集合向量、`convergence_indicators` 收敛指标结构、`timestamp` 时间戳、`model_version` 模型版本字符串
- `MeasureConfig` 结构含 `volatility_threshold` Option f64、`jaccard_threshold` Option f64、`aggregation` 聚合方式枚举 三字段
  - `Aggregation` 枚举含 `Mean`、`Max`、`Min` 三值,默认 `Mean`,对多对 Jaccard 取聚合方式
- `measure` 函数实现要点
  - `k` 计算:遍历 `AuditUnitCollection` 命中偏差的单元(severity 等于 Error 或 Warning),按 DES-010 L63 规则,多专家时取并集
  - `n` 计算:遍历 `targets` 列表,语义单元总数
  - `rate = (k as f64) / (n as f64)`,`n == 0` 时 `rate = 0.0` 且 `n == 0` 视为协议违规返回 `MeasureError::EmptyTargets`
  - `jaccard_index = |set_a ∩ set_b| / |set_a ∪ set_b|`,两集合都为空时 `jaccard_index = 0.0` 且返回 `MeasureError::EmptySets`
  - `volatility_coefficient = std_dev(rates) / mean(rates)`,N 大于等于 2 才有定义,N 等于 1 返回 `MeasureError::InsufficientAttempts`
  - 双指标判定:`volatility_pass = volatility_coefficient < threshold_volatility`、`jaccard_pass = jaccard_mean > threshold_jaccard`,阈值为 None 时不判定,`pass` 字段取 false 且 `converged = false`
  - 跨专家 Jaccard:expert_count 等于 1 时 `jaccard_sets` 为空列表,expert_count 大于 1 时两两配对
- `rate` 与 `jaccard_index` 与 `volatility_coefficient` 全部保留四位小数,实现用 `f64::round() * 10000.0 / 10000.0` 或等价函数
- 单元测试覆盖
  - K/N 基础计算(单专家单次审阅)
  - Jaccard 基础计算(两两配对)
  - 波动系数计算(均值与标准差)
  - 双指标联合判定(双过、单过、双不过)
  - 边界条件(`n=0`、`attempt < 2`、阈值 None)
  - 263 文档集 fixture 测试(承接 DEC-005 子项一,fixtures 路径 `sih-engine/fixtures/old_warehouse/` 当前为占位,真实 fixtures 归阶段 8 calibration 承载)
- 集成测试:用 mock `AuditUnitCollection` 跑 `measure`,验证 `DivergenceExperimentOutput` 序列化
- 派生 `Serialize` 与 `Deserialize` 在五个数据结构上,字段名 snake_case 对齐 SPEC-001 v2
- 不引入外部统计库(如 `statrs` 等),统计函数自实现(均值、标准差)
- 不持久化 `DivergenceExperimentOutput`,序列化输出由调用方决定

## 参考文件 {#references}

- 上游依赖: `sih-engine/state/tasks/task-012-stage-6-npc-orchestration.md` 阶段 6 任务包
- 上游依赖: `sih-engine/state/tasks/task-009-stage-3-semantic-tree.md` 阶段 3 任务包
- 上游依赖: `sih-engine/doc/decision/005-phase-0-data-and-intent.md` DEC-005 子项一 263 文档集复用边界判定
- 上游设计: `sih-engine/doc/design/DES-005-semantic-verification-calculus.md` 「偏离率」段 L58-66 与「收敛判定」段 L68-76
- 上游设计: `sih-engine/doc/design/DES-009-crosscheck-semantic-layer.md` 「偏离率」段 L94-102 与 P1-1 段 L122
- 数据契约: `sih-engine/doc/spec/SPEC-001-divergence-experiment.md` 「数据契约」段 L34-72
- 多专家范式: `sih-engine/doc/design/DES-010-single-key-multi-expert-experiment.md」 「实验方法」段 L57-92
- 仓骨架: `sih-engine/Cargo.toml` 与 `sih-engine/src/lib.rs` 与阶段 6 产出的 `src/npc/` 与阶段 3 产出的 `src/semantic_tree/`
- 工程基线: `sih-engine/AGENTS.md` 工程基线第一条与第四条与第五条

## 交付要求 {#deliverables}

- `sih-engine/src/divergence/mod.rs` 模块入口与公共项 re-export
- `sih-engine/src/divergence/rate.rs` DivergenceRate
- `sih-engine/src/divergence/jaccard.rs` JaccardSet
- `sih-engine/src/divergence/indicators.rs` ConvergenceIndicators
- `sih-engine/src/divergence/output.rs` DivergenceExperimentOutput
- `sih-engine/src/divergence/config.rs` MeasureConfig 与 Aggregation
- `sih-engine/src/divergence/measure.rs` measure 编排函数
- `sih-engine/src/divergence/stats.rs` 统计工具(均值、标准差)
- `sih-engine/src/divergence/tests.rs` 单元测试与集成测试
- 编译验证:`cargo build` 与 `cargo test` 通过
- doclint 验证:跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0
- 关键决策点
  - 波动系数计算为何用自实现而非外部库(栈一致性,公式简单,标准差与均值无需复杂统计)
  - 多专家 Jaccard 聚合为何默认 Mean(对齐 DES-005 v2 L73 平均 Jaccard 反映重合度,Mean 是最常见聚合方式,Max/Min 留可配置)
  - 263 文档集 fixture 为何不实装(对齐 DEC-005 子项一,可复用部分由阶段 8 calibration 实装,本任务包不预生成 fixtures)
  - 阈值 None 时为何不判定(对齐 DES-005 v2 L76 参数留空,默认值违反本任务包规则)
- clarifications
  - 263 文档集真实 fixtures 归阶段 8 calibration 实验承载
  - 实参定参归 SPEC-001 v2 偏离率实验数据填入后由 DEC 承载
  - 跨类型汇总(八大类型分层)归阶段 8 calibration 承载

## 边界 {#boundary}

- 不读任务包外文件
- 不发明新机制,严格按 DES-005 v2 偏离率段与 SPEC-001 v2 数据契约实现
- 任务包 doclint 必跑,缺 exit code 等于失败
- 任务包不实装 263 文档集真实 fixtures(归阶段 8)
- 任务包不实参定参(阈值留空)
- 任务包不引入外部统计库
- 任务包不持久化度量结果
- 任务包不实装八大类型分层统计(归阶段 8)
