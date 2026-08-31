# 任务包 011 阶段 5 rules 规则定义层


> 关闭裁定（批 pkgclose-solo，2026-08-31）
> 裁定：部分承接闭项
> 承接位：规则承载形态由规则包承接即 des-001 规则包版本管理、规则语义增删改走规则包版本管理；术语规则由检词术语包承接即已立与懒波与死档三态登记；空腹谓词形态由路择谓词包承接即零 LLM 只读；规则可重放可核由执契 verify 承接即同输入逐字节同判；CounterExample 反例三字段结构无机械承接位
> 证据：AGENTS.md:158 规则覆盖以包承载且规则语义增删改走规则包版本管理；sih-tools/nomenclator/CONTRACT.md:27 三态登记；sih-tools/tally/CONTRACT.md:19 verify 同输入逐字节同判
> 原文主体零改动，本块为批 pkgclose-solo 追加头部件


## 概览 {#overview}

本任务包实现 8 阶段路径阶段 5,在 `sih-engine` 仓加 Rust 规则定义模块,用声明式语言承载规则,规则本身是治理对象,可证伪、可被反例推翻。承接 DES-009 第四层规则定义段与 P1-4 convergence P3.1 退化机制定位。

- 任务目标::[目标](#task-objective)
- 项目背景::[背景](#project-background)
- 任务构成规则::[规则](#task-rules)
- 参考文件::[参考](#references)
- 交付要求::[交付](#deliverables)
- 边界::[边界](#boundary)

## 任务目标 {#task-objective}

实现 `sih-engine/src/rules/` 子模块,提供规则声明式定义与反例验证能力,使规则可被登记、可被反例验证、可被反例推翻。承接 DES-009 第四层规则定义段 L62-68 与「规则可证伪性元约束」段 L104-110。

子目标分四块。

第一块,实现规则 schema 数据结构,含规则 ID、规则描述、规则表达式(对 `SemanticNode` 的谓词)、反例列表三部分。规则表达式用闭包 `Box<dyn Fn(&SemanticNode) -> bool>` 表达,避免引入 DSL 解析器,保持工程简单性。

第二块,实现反例数据结构,沿用 DES-009 「规则可证伪性元约束」段 L106 三字段结构,反例路径、反例上下文(语义单元树切片)、复现步骤(确定性程序跑出反例的最小指令集)。

第三块,实现规则注册表 `RuleRegistry`,支持规则的增删查改,所有变更进 trail(对齐工程基线第四条可验证性约束,规则本身是治理对象)。

第四块,实现反例验证器,跑反例的复现步骤,若规则仍命中反例则规则有效,若规则不再命中反例则反例失效标注 `superseded`。

## 项目背景 {#project-background}

DES-009 第四层规则定义段 L62-68 明确,第三层规则用声明式语言定义,不硬编码。规则本身是治理对象,承接 PRO-09 元层。元约束一规则可证伪,每条规则必须配至少一个反例;元约束二冲突检测,规则间可能矛盾,声明式规则格式天然支持冲突检测,靠类型系统与约束求解,规则数少于 10 不强制显式跑冲突检测,多于 10 进 CI。

DES-009 「规则可证伪性元约束」段 L104-110 明确反例三字段结构与反例失效处理(反例可标 `superseded by` 新反例)。反例本身进治理,反例变更进 trail,反例失效是规则退化的检测信号。

DES-009 P1-4 段 L144 明确,承接 convergence 层 P3.1 退化机制,规则集本身会退化,反例是规则退化检测器,与 P3.1 上下文退化同源。承接工程基线第一确定性程序承载,反例验证由确定性程序跑。

上游依赖。阶段 4 judge 判定器,task-010-stage-4-judge.md,产出 `Rule` trait,本任务包在 `Rule` trait 之上加声明式层。

下游承接。阶段 6 NPC orchestration,task-012-stage-6-npc-orchestration.md,消费本任务包产出的 `RuleRegistry`,按规则调度 NPC 专家团。

## 任务构成规则 {#task-rules}

### self-check 六项 {#self-check}

1. 主动判断产出性质,这是 rules Rust 模块实现,不是 DES 设计
2. 服务原始意图,实现声明式规则定义,不发明新规则语言
3. 范畴排除显式,只动 src/rules/ 与 src/judge/(只读消费 Rule trait),不破 src/semantic_tree/
4. 不逃避当下责任,反例验证失败须显式标记 superseded,不静默接受
5. 每行去掉会犯错吗,规则注册变更必须进 trail
6. 与哲学仓相容,承接 PRO-09 元层治理

### 通用格式规范 {#general-format}

子代理写 Rust 代码不必遵守 DES-001,DES-001 只约束文档类产出。本任务包本身的 markdown 文档须遵守 DES-001。实现完跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0,同时 `cargo build` 与 `cargo test` 须通过。

### 类型特异化约束五条 {#type-specific}

1. 结构特异非字符集,Rule schema 与反例结构是结构层
2. 特异化需支撑,反例三字段引用 DES-009 「规则可证伪性元约束」段 L106,冲突检测阈值引用 L66
3. 不照抄通用,Rule trait 不重定义,本任务包加规则表达层而非替换 trait
4. 不重复定义,不重写阶段 4 已有 `Rule` trait
5. 最小化原则,只实现声明式层与反例验证,不做冲突检测求解器(规则数少于 10 不强制)

### 哲学检索 {#philosophy-search}

承接 PRO-09 元层治理(规则本身是治理对象)与 convergence 层 P3.1 退化机制(规则集本身会退化,反例是退化检测器)与工程基线第一条确定性程序承载。本任务包无新哲学命题,但需在 src/rules/mod.rs 注释中显式引用 convergence/witness-framework.md 路径(对齐 DES-009 P1-4 L144)。

### Rust 实现约束 {#rust-constraints}

- 公共 API 暴露 `RuleDef`、`CounterExample`、`RuleRegistry`、`RuleExpression` 四个公共项
- `RuleDef` 结构含 `id` 字符串、`description` 字符串、`expression` 表达式枚举、`counter_examples` 反例向量 四字段
- `RuleExpression` 枚举含 `Predicate` 变体,包裹闭包,签名 `Box<dyn Fn(&SemanticNode) -> bool + Send + Sync>`,与 `Composite` 变体,含 `op` 逻辑运算枚举与 `children` 子表达式向量 两变体
- `LogicOp` 枚举含 `And`、`Or`、`Not` 三值
- `CounterExample` 结构沿用 DES-009 L106 三字段
  - `path` `PathBuf` 反例路径
  - `context` `SemanticNode` 反例所处的语义单元树切片(深克隆)
  - `reproduction: String` 复现步骤,确定性程序最小指令集
  - `status: CounterExampleStatus` 枚举含 Active、Superseded { by: String } 两值
- `RuleRegistry` 方法
  - `register(rule: RuleDef) -> Result<(), RegistryError>`
  - `get(id: &str) -> Option<&RuleDef>`
  - `list() -> Vec<&RuleDef>`
  - `validate_all() -> Vec<RuleValidation>` 返回每条规则的有效性(规则是否仍命中所有反例)
  - 所有变更方法发 `RuleEvent` 事件进 `event_stream`(对齐工程基线第四条可验证性约束)
- 反例验证器 `validate_rule` 函数,签名 `fn(rule: &RuleDef) -> RuleValidation`,跑每条反例的 `reproduction` 步骤,当前为字符串描述,实际执行由 `sih-engine/src/rules/reproducer.rs` 占位实现,真实复现脚本归后续 DEC 承载,若规则不再命中反例则反例状态改为 `Superseded`,`by` 字段为 `validation-failure`
- 冲突检测阈值常量 `CONFLICT_DETECTION_THRESHOLD`,类型 `usize` 值为 `10`,规则数超过 10 触发 CI 跑冲突检测(对齐 DES-009 L66)
- 冲突检测当前为占位实现,实际求解器归后续 DEC,本任务包只暴露 `detect_conflicts() -> Vec<ConflictReport>` 接口,空实现
- 单元测试覆盖:规则注册、反例验证、Superseded 标记、闭包表达式、复合表达式(And/Or/Not)
- 集成测试:跑 `sih-engine/src/judge/rules/` 已有四类规则,验证每条规则可注册可验证
- 不引入外部规则引擎(如 `ruler` 等),闭包表达保持栈一致性
- 不引入 DSL 解析器(如 `pest` 等),规则用 Rust 代码表达

## 参考文件 {#references}

- 上游依赖: `sih-engine/state/tasks/task-010-stage-4-judge.md` 阶段 4 任务包
- 上游设计: `sih-engine/doc/design/DES-009-crosscheck-semantic-layer.md` 「第四层 规则定义」段 L62-68 与「规则可证伪性元约束」段 L104-110 与「P1 展开」P1-4 段 L144
- 仓骨架: `sih-engine/Cargo.toml` 与 `sih-engine/src/lib.rs` 与阶段 4 产出的 `src/judge/` 与 `src/event_stream/`
- 业界参考: Z notation 形式化规格语言、Alloy 关系约束声明、Schematron 的 rule 加 assert 加 report 声明式语法(在 DES-009 第四层 L67 引用)
- 哲学定位: `sih-philosophy/convergence/witness-framework.md` P3.1 退化机制(对齐 DES-009 P1-4,llm-friendly-build 知识包未收录,按 AGENTS.md 「哲学仓地位」段回退路径加载)
- 工程基线: `sih-engine/AGENTS.md` 工程基线第一条与第四条

## 交付要求 {#deliverables}

- `sih-engine/src/rules/mod.rs` 模块入口与 RuleDef 与 RuleExpression
- `sih-engine/src/rules/counter_example.rs` CounterExample 与 CounterExampleStatus
- `sih-engine/src/rules/registry.rs` RuleRegistry
- `sih-engine/src/rules/validator.rs` 反例验证器
- `sih-engine/src/rules/reproducer.rs` 复现脚本执行器(占位)
- `sih-engine/src/rules/conflict.rs` 冲突检测(占位)
- `sih-engine/src/rules/event.rs` RuleEvent 事件
- `sih-engine/src/rules/tests.rs` 单元测试与集成测试
- 编译验证:`cargo build` 与 `cargo test` 通过
- doclint 验证:跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0
- 关键决策点:规则表达式为何用 Rust 闭包而非 DSL(避免引入解析器,栈一致性,声明式语义由 `Composite` 枚举承载)、冲突检测阈值为何选 10(对齐 DES-009 L66,工程经验值)
- clarifications:复现脚本执行器与冲突检测求解器为占位,实装归后续 DEC 承载;反例真实复现需 `sih-engine/src/rules/reproducer.rs` 与 shell 解释器集成,本任务包不实现

## 边界 {#boundary}

- 不读任务包外文件
- 不发明新机制,严格按 DES-009 第四层与规则可证伪性元约束实现
- 任务包 doclint 必跑,缺 exit code 等于失败
- 任务包不实现真实复现脚本执行器(占位即可)
- 任务包不实现真实冲突检测求解器(占位即可)
- 任务包不替换阶段 4 `Rule` trait,只在其上加层
- 任务包不引入外部规则引擎与 DSL 解析器
