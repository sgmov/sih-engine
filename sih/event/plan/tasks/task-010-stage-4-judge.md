# 任务包 010 阶段 4 judge 判定器


> 关闭裁定（批 pkgclose-solo，2026-08-31）
> 裁定：承接闭项（分布式承接）
> 承接位：规则执行引擎由核阅／scrutinator 承接即 des-001 十二规则码、退出码 0/1/2、findings 结构化入报告；术语引用合法性由检词／nomenclator 承接即死档禁字级禁用与懒波词违例两规则；跨文档一致性由级联／cascade 承接即引用即边、上游洁净不变式；裁决材料与机器终签由执契／tally 即 R1 至 R7 核对加得一即二层裁决结构承接
> 证据：AGENTS.md:158 des-001 包覆盖十二种规则码；sih-tools/nomenclator/CONTRACT.md:37 两规则字符串级；sih-tools/cascade/CONTRACT.md:3 上游洁净不变式即当前内容哈希等于链上最近认证哈希；sih-engine/doc/decision/019-tally-naming.md:1；sih-engine/doc/decision/020-deyi-component-naming.md:3
> 原文主体零改动，本块为批 pkgclose-solo 追加头部件


## 概览 {#overview}

本任务包实现 8 阶段路径阶段 4,在 `sih-engine` 仓加 Rust 判定器模块,在阶段 3 semantic tree 上跑静态语义分析。本任务包不写 Rust 代码,只写 sub-agent 可执行的任务规范。sub-agent 跑完本任务包后产出 `sih-engine/src/judge/` 模块与单元测试。

- 任务目标::[目标](#task-objective)
- 项目背景::[背景](#project-background)
- 任务构成规则::[规则](#task-rules)
- 参考文件::[参考](#references)
- 交付要求::[交付](#deliverables)
- 边界::[边界](#boundary)

## 任务目标 {#task-objective}

实现 `sih-engine/src/judge/` 子模块,提供静态语义分析入口,在 `SemanticTree` 上跑四类检查,违规输出沿用 DES-008 NDJSON schema 进 trail。承接 DES-009 第三层判定器段与 P1-3 违规进 trail Rust 接口。

子目标分五块。

第一块,实现 `Rule` trait,定义 `check(node: &SemanticNode, ctx: &JudgeContext) -> Vec<Violation>` 接口,供规则注册。

第二块,实现四类内置规则,分别为节点存在性(候选方案节点必须存在)、必填性(候选方案必须含拒绝理由子节点且非空)、引用合法性(命题编号引用必须能在哲学仓索引定位)、跨文档一致性(同一语义单元被多文档引用时一致性校验)。

第三块,实现 `Judge` 编排器,遍历 `SemanticTree` 调用已注册规则,跳过 `data.unsliced = true` 节点(对齐 DES-009 「混合结构前提」段 L118)。

第四块,实现 `Violation` 结构,字段对齐 DES-008 「契约定义」段 mermaid 类图,含 type、rule_id、severity、line、column、message、timestamp、source_path 八字段。

第五块,实现 NDJSON 序列化与 stdout 输出,沿用 DES-008 契约,exit code 复用 doclint 四态(0/1/2/124)。

## 项目背景 {#project-background}

DES-009 第三层判定器段 L48-58 明确,判定器在语义单元树上跑静态语义分析,查跨节点语义约束,判定范围含节点存在性、必填性、引用合法性、跨文档一致性四类。判定器不查节点内文本语义质量,那归路径二 NPC 专家团。判定器是 NPC 专家团 Verifier 角色的外部确定性证据之一,不替代 NPC 的语义判定。

DES-009 违规进 trail 段 L58 明确,判定器输出 NDJSON 沿用 DES-008 schema,经 trail 写入层进 trail。本任务包实现判定器端 NDJSON 序列化,trail 写入层不归本任务包(对齐 DES-008 「trail 集成」段 L73-78)。

DES-009 P1-3 段 L142 明确,`Violation` 结构含八字段,字段名与 DES-008 完全一致,本任务包严格按此实现。

上游依赖。阶段 3 semantic tree 层,task-009-stage-3-semantic-tree.md,产出 `SemanticTree` 与 `SemanticNode` 与 `NodeData`。阶段 1 doclint 集成,task-007-stage-1-doclint-integration.md,产出 NDJSON schema 参考。

下游承接。阶段 5 rules 规则定义层,task-011-stage-5-rules.md,消费本任务包产出的 `Rule` trait 与 `Violation` 结构,声明式定义规则。

## 任务构成规则 {#task-rules}

### self-check 六项 {#self-check}

1. 主动判断产出性质,这是 judge Rust 模块实现,不是 DES 设计
2. 服务原始意图,实现四类内置规则与 Rule trait,不发明新判定范围
3. 范畴排除显式,只动 src/judge/ 与 src/semantic_tree/(只读消费),不破 src/tokenizer/ 与 src/event_stream/
4. 不逃避当下责任,引用合法性检查失败须显式 violation,不静默跳过
5. 每行去掉会犯错吗,违规字段必须可被 trail 写入层机械消费
6. 与哲学仓相容,承接 PRO-08 应而不藏

### 通用格式规范 {#general-format}

子代理写 Rust 代码不必遵守 DES-001,DES-001 只约束文档类产出。本任务包本身的 markdown 文档须遵守 DES-001。实现完跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0,同时 `cargo build` 与 `cargo test` 须通过。

### 类型特异化约束五条 {#type-specific}

1. 结构特异非字符集,Rule trait 与 NDJSON 序列化是结构层
2. 特异化需支撑,四类规则引用 DES-009 第三层 L52,Violation 字段引用 DES-008 契约定义段
3. 不照抄通用,Rule trait 不模仿 doclint Rule 结构,语义层与格式层分离
4. 不重复定义,不重写 DES-008 已定义 schema,只实现
5. 最小化原则,只实现四类内置规则与编排,声明式规则定义归阶段 5

### 哲学检索 {#philosophy-search}

承接 PRO-08 应而不藏(留痕是应鉴循环的构成性条件)与 PRO-07 鉴层检验职能(判定器是鉴层在工程层的载体之一)与工程基线第一条确定性程序承载。本任务包无新哲学命题。

### Rust 实现约束 {#rust-constraints}

- 公共 API 暴露 `Judge::new` 构造函数,`judge.run` 方法,签名 `fn(tree: &SemanticTree) -> JudgeReport`,`judge.add_rule` 方法,签名 `fn(Box<dyn Rule>)`
- `Rule` trait 含 `id` 方法,签名 `fn(&self) -> &str`,与 `check` 方法,签名 `fn(&self, node: &SemanticNode, ctx: &JudgeContext) -> Vec<Violation>`,两方法
- `Violation` 字段顺序与类型对齐 DES-008 mermaid 类图
  - `type` 字符串 必填,缺则视为协议违规
  - `rule_id` 字符串
  - `severity` Severity 枚举含 Error、Warning、Info
  - `line` u32、`column` u32
  - `message` 字符串
  - `timestamp` `chrono::DateTime<Utc>` 序列化 ISO 8601
  - `source_path` `PathBuf` 序列化字符串
- 四类内置规则实现位置
  - `rules/node_existence.rs` 候选方案节点存在性
  - `rules/required_child.rs` 必填子节点(拒绝理由等)
  - `rules/reference_validity.rs` 命题编号引用合法性
  - `rules/cross_doc_consistency.rs` 跨文档一致性
- `JudgeContext` 含 `philosophy_index: HashMap<String, String>`,键为命题 ID,值为命题路径,引用合法性检查查此索引
- 哲学索引当前为占位实现,实际加载逻辑归 `sih-engine/src/judge/philosophy_index.rs`,哲学仓未实装时用 `HashMap::new()` 兜底
- `Judge::run` 跳过 `data.unsliced = true` 节点(对齐 DES-009 L118)
- 跨文档一致性检查当前用文件级缓存(`HashMap<stable_id_content_hash, Vec<PathBuf>>`),实际跨文档查索引归阶段 5 规则定义
- 单元测试覆盖:每类规则各一个正例一个反例,unsliced 跳过,NDJSON 序列化 round-trip
- 集成测试:跑 `sih-engine/doc/design/DES-005-semantic-verification-calculus.md` 整文档,验证 violation 列表结构
- 退出码语义沿用 DES-008 L66:0 无违规、1 至少一条违规、2 内部错误、124 超时

## 参考文件 {#references}

- 上游依赖: `sih-engine/state/tasks/task-009-stage-3-semantic-tree.md` 阶段 3 任务包
- 上游依赖: `sih-engine/state/tasks/task-007-stage-1-doclint-integration.md` 阶段 1 任务包(NDJSON schema 参照)
- 上游设计: `sih-engine/doc/design/DES-009-crosscheck-semantic-layer.md` 「第三层 判定器」段 L48-58 与「P1 展开」P1-3 段 L142
- NDJSON 契约: `sih-engine/doc/design/DES-008-violation-to-trail.md` 「契约定义」段 L48-72
- 哲学索引: `sih-engine/src/judge/philosophy_index.rs`(本任务包实现,占位)
- 仓骨架: `sih-engine/Cargo.toml` 与 `sih-engine/src/lib.rs` 与阶段 3 产出的 `src/semantic_tree/`
- 工程基线: `sih-engine/AGENTS.md` 工程基线第一条与 PRO-08 应而不藏

## 交付要求 {#deliverables}

- `sih-engine/src/judge/mod.rs` 模块入口与 Rule trait
- `sih-engine/src/judge/violation.rs` Violation 结构
- `sih-engine/src/judge/context.rs` JudgeContext 结构
- `sih-engine/src/judge/judge.rs` Judge 编排器
- `sih-engine/src/judge/serialize.rs` NDJSON 序列化
- `sih-engine/src/judge/philosophy_index.rs` 哲学索引占位实现
- `sih-engine/src/judge/rules/node_existence.rs` 等四类规则各一文件
- `sih-engine/src/judge/tests.rs` 单元测试与集成测试
- 编译验证:`cargo build` 与 `cargo test` 通过
- doclint 验证:跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0
- 关键决策点:哲学索引为何用占位 `HashMap::new()` 而非真实加载(因哲学仓未实装,占位让规则可运行,实装后由独立 DEC 承载)、跨文档一致性为何用文件级缓存(避免每次检查重扫全文档,缓存由失效机制保证)
- clarifications:trail 写入层不归本任务包(对齐 DES-008 「trail 集成」段),实际跨文档索引归阶段 5 规则定义

## 边界 {#boundary}

- 不读任务包外文件
- 不发明新机制,严格按 DES-009 第三层与 P1-3 实现
- 任务包 doclint 必跑,缺 exit code 等于失败
- 任务包不写节点内文本语义质量判定(归阶段 6 NPC)
- 任务包不写声明式规则定义(归阶段 5)
- 任务包不写 trail 写入层
- 任务包不实装真实哲学索引加载
