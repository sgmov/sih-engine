# 任务包 009 阶段 3 semantic tree 语义树层


> 关闭裁定（批 pkgclose-solo，2026-08-31）
> 裁定：部分承接闭项
> 承接位：语义树本体即 containment 与 order 关系、树必产由句读／parser 承接即 PEG 语法树、错误处包节点永远产树，与寻址的 markdown 语义单元切分配套；unsliced 节点级切分标记无直接承接位
> 证据：sih-tools/parser/CONTRACT.md:18 错误处包节点永远产树；sih-engine/sih/event/trail/2026-08-28.ndjson:19 事件 9e990fc3；grep 全仓 unsliced 仅存于校准调研 sih-engine/sih/state/calibration/markdown-tokenization-toolchain-survey.md:51 与 91，无活动实装，部分承接如实记
> 原文主体零改动，本块为批 pkgclose-solo 追加头部件


## 概览 {#overview}

本任务包实现 8 阶段路径阶段 3,在 `sih-engine` 仓加 Rust 语义树层模块,把阶段 2 tokenizer 输出的语义单元组织为含 containment 与 order 关系的语义树。本任务包不写 Rust 代码,只写 sub-agent 可执行的任务规范。sub-agent 跑完本任务包后产出 `sih-engine/src/semantic_tree/` 模块与单元测试。

- 任务目标::[目标](#task-objective)
- 项目背景::[背景](#project-background)
- 任务构成规则::[规则](#task-rules)
- 参考文件::[参考](#references)
- 交付要求::[交付](#deliverables)
- 边界::[边界](#boundary)

## 任务目标 {#task-objective}

实现 `sih-engine/src/semantic_tree/` 子模块,提供 `build_tree` 入口函数,消费阶段 2 tokenizer 输出的 `SemanticUnitTree`,重建含父节点引用的语义树,表达单元间的 containment 与 order 关系。承接 DES-009 第二层语义树段与 P1-2 自由文本容器工程化(mdast 节点挂 `data.unsliced` 布尔属性识别未切分节点)。

子目标分四块。

第一块,实现 `SemanticTree` 根结构,含 `root_nodes: Vec<SemanticNode>` 与 `node_index: HashMap<stable_id, SemanticNode>` 两字段。

第二块,实现 `SemanticNode` 节点结构,含 `stable_id: String`、`unit_type: SemanticUnitType`、`content: String`、`position: SourcePosition`、`parent: Option<Box<SemanticNode>>`、`children: Vec<SemanticNode>`、`data: NodeData` 七字段。

第三块,实现 `build_tree` 函数,从 comrak 产出的 AST 直接建树而非从 `SemanticUnitTree` 重排,保留 comrak 自身的 containment 关系(标题内嵌段落、列表项嵌套等),以 comrak AST 为单一权威源。

第四块,实现 `unsliced` 标记函数,识别无法切分的大段叙事节点,挂 `data.unsliced = true` 标志,理由写入 `data.unsliced_reason`。

## 项目背景 {#project-background}

DES-009 第二层语义树段明确,语义树表达单元间的包含与顺序关系,工具承载是 `pulldown-cmark` 与 `comrak` 的 AST 树,经 Rust serde Serialize 输出为 JSON。承接工程基线第一确定性程序承载,语义树不依赖 LLM 生成。

DES-009 「混合结构前提」段 L112-119 明确,治理文档是混合结构,含结构化语义单元与大段叙事文本,无法切分的大段叙事应标 `unsliced` 标志绕开判定器。DES-009 P1-2 自由文本容器工程化 L124-140 给出 JSON 结构示意,`Node` 含 `UnslicedData` 子结构,本任务包按此实现。

上游依赖。阶段 2 tokenizer 层,task-008-stage-2-tokenizer-layer.md,产出 `SemanticUnitTree`。本任务包选择从 comrak AST 直接建树而非从 `SemanticUnitTree` 重排,因 `SemanticUnitTree` 已经丢失层级关系,重排会引入伪 containment。

下游承接。阶段 4 judge 判定器,task-010-stage-4-judge.md,消费本任务包产出的 `SemanticTree`,在节点上跑静态语义分析。阶段 4 跳过 `data.unsliced = true` 节点。

## 任务构成规则 {#task-rules}

### self-check 六项 {#self-check}

1. 主动判断产出性质,这是 semantic tree Rust 模块实现,不是 DES 设计
2. 服务原始意图,实现建树接口,不发明新树结构
3. 范畴排除显式,只动 src/semantic_tree/ 与 src/tokenizer/(只读消费),不破 src/event_stream/
4. 不逃避当下责任,父节点引用循环须显式检测,不能无限递归
5. 每行去掉会犯错吗,containment 关系必须真实(comrak AST 父子关系)
6. 与哲学仓相容,承接工程基线第一确定性程序承载

### 通用格式规范 {#general-format}

子代理写 Rust 代码不必遵守 DES-001,DES-001 只约束文档类产出。本任务包本身的 markdown 文档须遵守 DES-001。实现完跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0,同时 `cargo build` 与 `cargo test` 须通过。

### 类型特异化约束五条 {#type-specific}

1. 结构特异非字符集,树结构与父引用是结构层
2. 特异化需支撑,UnslicedData 字段引用 DES-009 P1-2 mermaid 类图 L126-140
3. 不照抄通用,语义树接口不模仿 tokenizer 接口,层级关系由 comrak AST 承载
4. 不重复定义,不重写 comrak 自身的父子关系
5. 最小化原则,只建树与标记 unsliced,不做判定(归阶段 4)

### 哲学检索 {#philosophy-search}

承接工程基线第一条确定性程序是治理操作唯一执行者与工程基线第五条 LLM 是材料生成器不是度量权威。本任务包无新哲学命题,语义树由确定性程序从 comrak AST 构建。

### Rust 实现约束 {#rust-constraints}

- 公共 API 暴露 `build_tree` 函数,签名是输入 markdown 字符串,返回 `Result` 包裹的 `SemanticTree` 与错误,以及 `mark_unsliced` 函数,签名是输入 `SemanticNode` 引用与 reason 字符串
- `SemanticNode` 字段名与字段顺序对齐 DES-009 P1-2 mermaid 类图
- `NodeData` 结构含 `unsliced` 布尔、`unsliced_reason` Option 字符串 两字段
- `build_tree` 实现,遍历 comrak AST,创建 `SemanticNode`,`parent` 字段填直属父节点 ID,`children` 字段填直接子节点列表
- 父节点引用用 `Box<SemanticNode>` 避免栈溢出(深嵌套场景如长有序列表)
- `serde::Serialize` derive 加在 `SemanticNode` 与 `NodeData` 与 `SemanticTree`,字段名与 DES-009 P1-2 一致
- `unsliced` 标记判定:段落节点 `content.len() > 2048` 字节且无结构化子节点(列表、表格、代码块)时,自动标 `unsliced = true`,理由为「长段叙事无法结构化切分」
- 阈值 `2048` 是参考值,实际阈值在 `sih-engine/src/semantic_tree/config.rs` 常量化,留待 SPEC-001 校准
- 单元测试覆盖:单标题节点、嵌套标题、列表项层级、表格节点、unsliced 自动标记、循环引用检测
- 集成测试:跑 `sih-engine/doc/design/DES-005-semantic-verification-calculus.md`,验证树深度与节点数
- 不引入新的 markdown 解析器(沿用阶段 2 选型的 comrak)
- 不修改阶段 2 tokenizer 模块,只读消费其数据结构

## 参考文件 {#references}

- 上游依赖: `sih-engine/state/tasks/task-008-stage-2-tokenizer-layer.md` 阶段 2 任务包
- 上游设计: `sih-engine/doc/design/DES-009-crosscheck-semantic-layer.md` 「第二层 语义树」段 L38-46 与「混合结构前提」段 L112-119 与「P1 展开」P1-2 段 L124-140
- 仓骨架: `sih-engine/Cargo.toml` 与 `sih-engine/src/lib.rs` 与阶段 2 产出的 `src/tokenizer/`
- 业界参考: tree-sitter 具体语法树、remark mdast、IETF XML DOM 树(在 DES-009 第二层 L44 引用)
- 工程基线: `sih-engine/AGENTS.md` 工程基线第一条与第五条

## 交付要求 {#deliverables}

- `sih-engine/src/semantic_tree/mod.rs` 模块入口
- `sih-engine/src/semantic_tree/node.rs` 节点结构定义
- `sih-engine/src/semantic_tree/build.rs` 建树函数
- `sih-engine/src/semantic_tree/unsliced.rs` unsliced 标记函数
- `sih-engine/src/semantic_tree/config.rs` 阈值常量(2048 字节)
- `sih-engine/src/semantic_tree/tests.rs` 单元测试与集成测试
- 编译验证:`cargo build` 与 `cargo test` 通过
- doclint 验证:跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0
- 关键决策点:为何从 comrak AST 直接建树而非从 `SemanticUnitTree` 重排(因后者已丢失层级)、unsliced 阈值为何选 2048 字节(参考 DES-009 P3-1 退化机制,长段叙事多承载哲学阐发难以切分,2048 是工程经验值待 SPEC-001 校准)
- clarifications:阈值 2048 是工程占位,SPEC-001 校准数据到位后由 DEC 承载修订

## 边界 {#boundary}

- 不读任务包外文件
- 不发明新机制,严格按 DES-009 第二层与 P1-2 实现
- 任务包 doclint 必跑,缺 exit code 等于失败
- 任务包不写判定器代码(归阶段 4)
- 任务包不改 comrak 选型
- 任务包不修改阶段 2 tokenizer 模块
