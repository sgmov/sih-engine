# 任务包 012 阶段 6 NPC orchestration 编排层


> 关闭裁定（批 pkgclose-solo，2026-08-31）
> 裁定：承接闭项
> 承接位：多专家独立审阅由 facet 承接即多 facetor 独立审阅、单 KEY 单 MODEL、compiler 确定性聚合出异质性指标；多代理编排形态由代理编组承接即单线、单发、反思、并联、串行五形、2026-08-28 全签批；裁决材料落链由得一即二层裁决结构承接
> 证据：sih-tools/facet/FACET.md:7 多 facetor 独立审阅加 compiler 确定性聚合；sih-tools/facet/FACET.md:41 单 KEY 单 MODEL；AGENTS.md:279 sihankor-marshalling 代理编组即长程任务范式；sih-engine/doc/decision/020-deyi-component-naming.md:3
> 原文主体零改动，本块为批 pkgclose-solo 追加头部件


## 概览 {#overview}

本任务包实现 8 阶段路径阶段 6,在 `sih-engine` 仓加 Rust NPC 编排模块,按规则调度 NPC 专家团审阅,收集审阅单元。承接 DES-009 第五层 Verifier 与偏离率段与 DES-005 v2 审阅单元结构与 DES-010 单 KEY 多专家范式。

- 任务目标::[目标](#task-objective)
- 项目背景::[背景](#project-background)
- 任务构成规则::[规则](#task-rules)
- 参考文件::[参考](#references)
- 交付要求::[交付](#deliverables)
- 边界::[边界](#boundary)

## 任务目标 {#task-objective}

实现 `sih-engine/src/npc/` 子模块,提供 NPC 专家团调度入口,按 `RuleRegistry` 注册的规则与 `SemanticTree` 节点,调度多个 sub-agent 审阅,收集审阅单元,审计单元六字段对齐 DES-005 v2 审阅单元段。承接 DES-009 第五层 L70-78 与 DES-010 单 KEY 多专家范式 L42-53。

子目标分五块。

第一块,实现 `AuditUnit` 数据结构,六字段严格对齐 DES-005 v2 审阅单元段 L52,含 `stable_id`、`target_id`、`kind`、`severity`、`reason`、`attempt_id`,可选 `expert_id` 字段用于多专家场景。

第二块,实现 `NpcExpert` trait,定义 `review(target: &SemanticNode, rule: &RuleDef) -> Vec<AuditUnit>` 接口,供 NPC 实例实现。

第三块,实现 `ExpertPool` 专家池,支持 1/3/5 三档专家数配置(对齐 DES-010 因子一),所有专家共享同一模型实例(对齐 DES-010 「单 KEY 共享」段 L45)。

第四块,实现 `Orchestrator` 编排器,接收 `SemanticTree` 与 `RuleRegistry`,调度专家池跑审阅,聚合审阅单元进 `AuditUnitCollection`。

第五块,实现 `AuditUnitCollection` 容器,提供 `units_for_target(target_id) -> Vec<&AuditUnit>`、`units_by_attempt(attempt_id) -> Vec<&AuditUnit>`、`units_by_expert(expert_id) -> Vec<&AuditUnit>` 三查询接口,供阶段 7 divergence metric 消费。

## 项目背景 {#project-background}

DES-009 第五层 Verifier 与偏离率段 L70-78 明确,NPC 专家团承接路径二,判定节点内文本语义质量,判定结果切分为审阅单元,每个审阅单元指向产出物的某个语义单元节点,靠 stable id 主定位加位置备查。

DES-005 v2 审阅单元段 L48-56 明确,审阅单元含六字段。`stable_id` 与 `attempt_id` 由确定性程序派生,`kind`、`severity`、`reason` 由 NPC 专家团产出,`target_id` 由 NPC 专家团判定。本任务包实现确定性派生部分与容器,NPC 专家团产出由 sub-agent 进程提供(占位实现)。

DES-010 「关键变量」段 L43-49 明确,单 KEY 共享(五个 sub-agent 共享同一模型实例)、引导式任务包(所有专家接收同一五段任务包)、审阅单元输出(每个专家产审阅单元,`attempt_id` 在多专家实验中替换为 `expert_id`)。

上游依赖。阶段 5 rules 规则定义层,task-011-stage-5-rules.md,产出 `RuleRegistry` 与 `RuleDef`。阶段 3 semantic tree 层,task-009-stage-3-semantic-tree.md,产出 `SemanticTree` 与 `SemanticNode` 与 `target_id` 派生。

下游承接。阶段 7 divergence metric,task-013-stage-7-divergence-metric.md,消费本任务包产出的 `AuditUnitCollection`,计算 K/N 与 Jaccard 与波动系数。

## 任务构成规则 {#task-rules}

### self-check 六项 {#self-check}

1. 主动判断产出性质,这是 NPC orchestration Rust 模块实现,不是 DES 设计
2. 服务原始意图,实现专家调度与审阅单元收集,不发明新调度算法
3. 范畴排除显式,只动 src/npc/ 与 src/rules/(只读消费)与 src/semantic_tree/(只读消费),不破 src/judge/
4. 不逃避当下责任,审阅单元字段缺失须显式错误,不能补默认值
5. 每行去掉会犯错吗,审阅单元结构必须可被阶段 7 机械消费
6. 与哲学仓相容,承接工程基线第五条 LLM 是材料生成器

### 通用格式规范 {#general-format}

子代理写 Rust 代码不必遵守 DES-001,DES-001 只约束文档类产出。本任务包本身的 markdown 文档须遵守 DES-001。实现完跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0,同时 `cargo build` 与 `cargo test` 须通过。

### 类型特异化约束五条 {#type-specific}

1. 结构特异非字符集,AuditUnit 结构与 ExpertPool 是结构层
2. 特异化需支撑,六字段引用 DES-005 v2 审阅单元段 L52,专家数 1/3/5 引用 DES-010 因子一 L35
3. 不照抄通用,NPC 接口不复用 stage 4 `Rule` trait,审阅与判定是不同范畴
4. 不重复定义,不重写 DES-005 v2 已定义六字段,只实现
5. 最小化原则,只实现调度与收集,实际 sub-agent 进程调用占位

### 哲学检索 {#philosophy-search}

承接工程基线第五条 LLM 是材料生成器不是度量权威(NPC 输出是材料,度量归阶段 7)与工程基线第一条确定性程序承载(`stable_id` 与 `attempt_id` 由确定性程序派生)与 T6-PARADIGMS.md 集群范式(单 KEY 多专家引导式任务包)。本任务包无新哲学命题。

### Rust 实现约束 {#rust-constraints}

- 公共 API 暴露 `AuditUnit`、`AuditUnitCollection`、`NpcExpert`、`ExpertPool`、`Orchestrator` 五个公共项
- `AuditUnit` 字段顺序与类型对齐 DES-005 v2 审阅单元段
  - `stable_id` 字符串 由确定性程序派生,格式 `path`、`rule_id`、`target_stable_id`、`content_hash` 四段用半角冒号拼接
  - `target_id` 字符串 指向产出物语义单元节点的 stable_id
  - `kind` `AuditKind` 枚举含 Deviation、Suggestion、ScopeTag 三值(对齐 DES-005 v2 L52 偏差发现、纠正建议、范畴标记)
  - `severity` `Severity` 复用 stage 4 的 Severity 枚举(Error/Warning/Info)
  - `reason` 字符串 一句话事实陈述,不引导判定
  - `attempt_id` 字符串 由确定性程序派生,格式 `attempt-N` 其中 N 为正整数
  - `expert_id` Option 字符串 可选,多专家实验时取 `expert-1` 至 `expert-5`,单专家时取 None
- `AuditUnit` 派生 `Serialize` 与 `Deserialize`,字段名 snake_case 对齐 DES-005 v2 与 SPEC-001 v2
- `NpcExpert` trait 含 `id` 方法,签名 `fn(&self) -> &str`,与 `review` 方法,签名 `fn(&self, target: &SemanticNode, rule: &RuleDef) -> Vec<AuditUnit>`,两方法
- `ExpertPool` 配置 `ExpertCount` 枚举含 One、Three、Five 三值,默认 One
- `ExpertPool::spawn` 方法,签名 `fn(count: ExpertCount) -> Vec<Box<dyn NpcExpert>>`,共享同一 `ModelInstance`,用 `Arc<Mutex<Box<dyn ModelInstance>>>` 包裹
- `Orchestrator` 调度策略:每个 (target_node, rule) 对由 `ExpertPool` 中所有专家各审一次,产出 N 个 `AuditUnit`(N 等于专家数),attempt_id 在同一调度批次内单调递增
- `AuditUnitCollection` 内部用 `Vec<AuditUnit>` 与 `HashMap<String, Vec<usize>>` 索引(target_id 与 attempt_id 与 expert_id)
- 单元测试覆盖:AuditUnit 序列化 round-trip、专家数 1/3/5 配置、Orchestrator 单规则调度、AuditUnitCollection 三查询接口
- 集成测试:用 mock NpcExpert 跑 `sih-engine/doc/design/DES-005-semantic-verification-calculus.md` 整文档,验证 AuditUnit 数量与字段
- 不引入外部 LLM SDK(如 `async-openai` 等),`ModelInstance` trait 为占位,真实模型调用归后续 DEC 承载
- 不引入消息队列(如 `lapin` 等),Orchestrator 同步调度
- 不持久化审阅单元,序列化输出由调用方决定

## 参考文件 {#references}

- 上游依赖: `sih-engine/state/tasks/task-011-stage-5-rules.md` 阶段 5 任务包
- 上游依赖: `sih-engine/state/tasks/task-009-stage-3-semantic-tree.md` 阶段 3 任务包
- 上游设计: `sih-engine/doc/design/DES-009-crosscheck-semantic-layer.md` 「第五层 Verifier 与偏离率」段 L70-78
- 审阅单元结构: `sih-engine/doc/design/DES-005-semantic-verification-calculus.md` 「审阅单元」段 L48-56
- 多专家范式: `sih-engine/doc/design/DES-010-single-key-multi-expert-experiment.md` 「关键变量」段 L43-49 与「实验因子」段 L33-41
- 数据契约: `sih-engine/doc/spec/SPEC-001-divergence-experiment.md` 「语义判断材料」段 L38-42
- 集群范式: `ai-ex/T6-PARADIGMS.md` 单 KEY 引导式专家团
- 任务包模板: `ai-ex/SUBAGENT-TASK-TEMPLATES.md` 模板二(引导式任务包基线)
- 仓骨架: `sih-engine/Cargo.toml` 与 `sih-engine/src/lib.rs` 与阶段 5 产出的 `src/rules/` 与阶段 3 产出的 `src/semantic_tree/`
- 工程基线: `sih-engine/AGENTS.md` 工程基线第一条与第五条

## 交付要求 {#deliverables}

- `sih-engine/src/npc/mod.rs` 模块入口与公共项 re-export
- `sih-engine/src/npc/audit_unit.rs` AuditUnit 与 AuditKind
- `sih-engine/src/npc/expert.rs` NpcExpert trait
- `sih-engine/src/npc/expert_pool.rs` ExpertPool 与 ExpertCount
- `sih-engine/src/npc/orchestrator.rs` Orchestrator
- `sih-engine/src/npc/collection.rs` AuditUnitCollection
- `sih-engine/src/npc/mock_expert.rs` Mock NpcExpert(测试用)
- `sih-engine/src/npc/model.rs` ModelInstance trait(占位)
- `sih-engine/src/npc/tests.rs` 单元测试与集成测试
- 编译验证:`cargo build` 与 `cargo test` 通过
- doclint 验证:跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0
- 关键决策点:专家数为何只支持 1/3/5 三档(对齐 DES-010 因子一 L35,实验设计固定)、`expert_id` 字段为何可选(对齐 DES-010 L99,在多专家实验中替换 attempt_id,单专家时 attempt_id 即足够)、`ModelInstance` 为何用 trait 而非具体类型(栈一致性,真实模型调用归后续 DEC)
- clarifications:真实 sub-agent 进程调用与 LLM SDK 集成归后续 DEC 承载,本任务包 `ModelInstance` 为占位;审计单元持久化策略归 DEC 承载(序列化为 NDJSON 还是 SQLite 等)

## 边界 {#boundary}

- 不读任务包外文件
- 不发明新机制,严格按 DES-009 第五层与 DES-005 v2 审阅单元段实现
- 任务包 doclint 必跑,缺 exit code 等于失败
- 任务包不实现真实 LLM 调用(占位即可)
- 任务包不实现真实 sub-agent 进程编排(同步调度即可)
- 任务包不持久化审阅单元
- 任务包不计算偏离率(归阶段 7)
