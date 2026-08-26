# 任务包 008 阶段 2 tokenizer 分词层

## 概览 {#overview}

本任务包实现 8 阶段路径阶段 2,在 `sih-engine` 仓加 Rust 分词层模块,把治理文档从纯文本切分为语义单元。本任务包不写 Rust 代码,只写 sub-agent 可执行的任务规范。sub-agent 跑完本任务包后产出 `sih-engine/src/tokenizer/` 模块与单元测试。

- 任务目标::[目标](#task-objective)
- 项目背景::[背景](#project-background)
- 任务构成规则::[规则](#task-rules)
- 参考文件::[参考](#references)
- 交付要求::[交付](#deliverables)
- 边界::[边界](#boundary)

## 任务目标 {#task-objective}

实现 `sih-engine/src/tokenizer/` 子模块,提供 `tokenize` 入口函数,把输入的 markdown 路径列表切分为语义单元树,每单元含 stable_id、type、content、position 四字段,稳定识别符由确定性程序派生。承接 DES-009 第一层分词段,工具栈选型沿用 `markdown-tokenization-toolchain-survey.md` 结论,使用 `pulldown-cmark` 加 `comrak` Rust 栈。

子目标分四块。

第一块,加 `sih-engine/Cargo.toml` 依赖,引入 `pulldown-cmark` 与 `comrak`,不引入 Node 工具链。

第二块,实现 `sih-engine/src/tokenizer/mod.rs` 模块入口,导出 `tokenize`、`SemanticUnit`、`SemanticUnitTree`、`TokenizeError` 四个公共项。

第三块,实现语义单元切分函数,遍历 comrak 产出的 AST,识别标题与段落与列表项与代码块与表格,每种节点类型映射到 `type` 字段。

第四块,实现 `stable_id` 派生函数,规则为「文件路径加节点类型加节点序号加内容 hash」,碰撞由内容 hash 兜底。

## 项目背景 {#project-background}

DES-009 第一层分词段明确,治理文档须从纯文本切分为语义单元,每个单元带类型标记与 stable id。工具栈选型已经过调研,`sih-engine/sih/state/calibration/markdown-tokenization-toolchain-survey.md` 结论是 Rust 栈 `pulldown-cmark` 加 `comrak`,理由含栈一致性与可复现性与 CommonMark 覆盖。

DES-009 「混合结构前提」段明确,治理文档是混合结构,含结构化语义单元与大段叙事文本。无法切分的大段叙事应标 `unsliced` 标志,绕开后续判定器直接走 NPC Verifier。本任务包在节点元数据层支持 `unsliced` 标记,具体消费由阶段 3 语义树层决定。

上游依赖。阶段 1 doclint 集成,task-007-stage-1-doclint-integration.md,产出的 NDJSON 输出对 tokenizer 是并列路径(doclint 查格式违规,tokenizer 切分语义单元,两者输入同一文档集)。

下游承接。阶段 3 语义树层,task-009-stage-3-semantic-tree.md,消费本任务包产出的 `SemanticUnitTree` 重建含父引用的语义树。

## 任务构成规则 {#task-rules}

### self-check 六项 {#self-check}

1. 主动判断产出性质,这是 tokenizer Rust 模块实现,不是 DES 设计
2. 服务原始意图,实现分词器接口,不发明新切分算法
3. 范畴排除显式,只动 src/tokenizer/ 与 Cargo.toml,不破 src/event_stream/
4. 不逃避当下责任,AST 解析失败须显式错误,不能静默吞掉
5. 每行去掉会犯错吗,stable_id 必须可复现(同输入同输出)
6. 与哲学仓相容,承接工程基线第一确定性程序承载

### 通用格式规范 {#general-format}

子代理写 Rust 代码不必遵守 DES-001,DES-001 只约束文档类产出。本任务包本身的 markdown 文档须遵守 DES-001。实现完跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0,同时 `cargo build` 与 `cargo test` 须通过。

### 类型特异化约束五条 {#type-specific}

1. 结构特异非字符集,模块结构与 trait 边界是结构层
2. 特异化需支撑,语义单元四字段引用 DES-009 第一层分词段 L36,stable_id 派生规则引用 DES-009 第一层 L30-31
3. 不照抄通用,tokenizer 接口不模仿 doclint 接口,语义层与格式层分离
4. 不重复定义,不重写 markdown-tokenization-toolchain-survey.md 已选型结论
5. 最小化原则,只实现切分,不做语义判定(归阶段 3 与 4)

### 哲学检索 {#philosophy-search}

承接工程基线第一条确定性程序是治理操作唯一执行者与第四条可验证性约束。本任务包无新哲学命题,语义切分由确定性程序完成,LLM 不参与切分(对齐 DES-009 第一层 L28)。

### Rust 实现约束 {#rust-constraints}

- 公共 API 暴露 `tokenize` 函数,签名 `fn(paths: &[Path]) -> Result<SemanticUnitTree, TokenizeError>`
- `SemanticUnit` 结构含 `stable_id` 字符串、`unit_type` 枚举、`content` 字符串、`position` 结构 四字段
- `SemanticUnitType` 枚举含 Heading、Paragraph、ListItem、CodeBlock、Table、TableRow、Other 七值
- `SourcePosition` 结构含 `line` u32、`column` u32、`offset` u32 三字段
- `stable_id` 派生规则,格式 `path`、`type`、`index`、`hash` 四段用半角冒号拼接
- 路径读取用 `std::fs::read_to_string`,UTF-8 解码失败返回 `TokenizeError::Encoding`
- 大段叙事无法切分时,节点 `unit_type` 取 `Other`,内容保留整段,无 `unsliced` 字段(本任务包只切分不标 unsliced,unsliced 标志由阶段 3 语义树层决定)
- 单元测试覆盖:标题识别、列表项识别、代码块保留、stable_id 复现性、空文档处理
- 集成测试:跑 `sih-engine/doc/design/DES-005-semantic-verification-calculus.md` 整文档,验证切分节点数与类型分布
- 不引入 `tree-sitter-markdown`(对齐调研结论,自述不追求正确性)
- 不引入 Node 工具链(对齐调研结论,栈一致性)

## 参考文件 {#references}

- 上游依赖: `sih-engine/state/tasks/task-007-stage-1-doclint-integration.md` 阶段 1 任务包
- 上游设计: `sih-engine/doc/design/DES-009-crosscheck-semantic-layer.md` 「第一层 分词」段 L26-36 与「混合结构前提」段 L112-119
- 上游选型: `sih-engine/sih/state/calibration/markdown-tokenization-toolchain-survey.md` 「本仓栈选型判定」段
- 仓骨架: `sih-engine/Cargo.toml` 与 `sih-engine/src/lib.rs`
- 业界参考: tree-sitter 具体语法树、remark mdast、IETF XML DOM 树(在 DES-009 第一层 L44 引用)
- 工程基线: `sih-engine/AGENTS.md` 工程基线第一条与第四条

## 交付要求 {#deliverables}

- `sih-engine/Cargo.toml` 加 `pulldown-cmark` 与 `comrak` 依赖
- `sih-engine/src/tokenizer/mod.rs` 模块入口
- `sih-engine/src/tokenizer/semantic_unit.rs` 数据结构定义
- `sih-engine/src/tokenizer/parse.rs` AST 遍历与切分函数
- `sih-engine/src/tokenizer/stable_id.rs` stable_id 派生函数
- `sih-engine/src/tokenizer/tests.rs` 单元测试与集成测试
- 编译验证:`cargo build` 与 `cargo test` 通过
- doclint 验证:跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0
- 关键决策点:stable_id 拼接格式为何用 `path:type:index:hash` 而非 UUID(对齐 DES-009 L30,确定性可复现)、`SemanticUnitType` 枚举为何七值而非更细(对齐 DES-009 「混合结构」段,无法切分归 Other 兜底)
- clarifications:unsliced 标志由阶段 3 语义树层决定而非本任务包,因语义切分与语义判定边界应分清

## 边界 {#boundary}

- 不读任务包外文件
- 不发明新机制,严格按 DES-009 第一层与 markdown 工具栈调研实现
- 任务包 doclint 必跑,缺 exit code 等于失败
- 任务包不写语义判定代码(归阶段 3 与 4)
- 任务包不写 stable_id 之外的稳定识别机制
- 任务包不引入 tree-sitter-markdown 与 Node 工具链
