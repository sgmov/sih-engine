# 工程基座设计

本设计定义 sih-engine 第一阶段的核心工程结构，为事件流模块与构建测试组织提供确定性依据。本设计不展开 MCP server 架构，不定义其他组件的模块位置，只为事件流提供直接支撑。

## 概览 {#overview}

- Cargo workspace 结构为单 crate，判据承接 DEC-001 源码与工具的工程职责分界::[Cargo workspace 结构](#cargo-workspace)
- 事件流模块含数据结构与三个接口，哈希链计算归确定性程序::[事件流模块设计](#event-stream-module)
- 构建与测试按 Rust 社区规范组织，trail 文件作集成测试基准::[构建与测试组织](#build-test)
- 物理存储约束承接 DEC-001 物理存储最小约束::[物理存储约束](#physical-storage)
- 认识论立场为 design-corollary::[认识论立场](#epistemic-stance)

## Cargo workspace 结构 {#cargo-workspace}

### 结构判定 {#cargo-workspace-decision}

sih-engine 第一阶段采用单 crate 结构，不使用 workspace。

判定依据承接 DEC-001 第 49 行：源码存引擎核心源代码，是 MCP server 与内部模块的承载，编译产出引擎可执行文件。MCP server 架构尚未定义，此刻引入 workspace 层级会导致模块边界过早固化。DEC-001 第 51 行的分界判据表明，工具与源码的分界在于是否需要独立编译分发。事件流是引擎内部模块，不满足独立分发条件，归源码而非工具。

workspace 结构留待 MCP server 架构定义后评估。届时若 MCP server 与事件流需要独立版本迭代，或治理环其他组件需要独立编译，可按 DEC-001 的工程职责分界重新判定。

### 目录布局 {#cargo-layout}

````filetree
sih-engine/
├── Cargo.toml               workspace 根配置（第一阶段即 crate 根）
├── src/
│   ├── lib.rs               库入口，导出事件流公共接口
│   └── event_stream/
│       ├── mod.rs           事件流模块入口
│       ├── event.rs         事件数据结构，含 Actor 子结构
│       ├── hash.rs          哈希链计算，哈希算法确定
│       ├── append.rs        追加写入入口，含四态校验
│       ├── verify.rs        哈希链校验入口，含区间校验
│       └── query.rs         事件检索入口，含过滤与聚合
├── tests/
│   ├── integration_hash_chain.rs   集成测试：加载 trail 文件验证哈希链
│   └── integration_empty_hash.rs   集成测试：验证空哈希过渡态识别
└── fixtures/
    ├── 2026-07-27.ndjson            集成测试基准：完整哈希链
    └── 2026-08-03.ndjson            集成测试基准：空哈希过渡态
````

### Cargo.toml 定位 {#cargo-toml-location}

第一阶段 `Cargo.toml` 位于 sih-engine 根目录，既是 workspace 根也是 crate 根。待 workspace 结构引入时，此文件转为 `[workspace]` 根，当前的 package 信息迁移到 `src/` 下的 `Cargo.toml`。

## 事件流模块设计 {#event-stream-module}

### 模块组织 {#module-organization}

事件流模块由五个子模块组成，按职责分离。

`event.rs` 承载事件数据结构，是模块的核心数据模型。`hash.rs` 承载哈希链计算，计算规则由确定性程序实现，不存在非确定性分支。`append.rs` 承载追加写入入口，是事件进入事件流的唯一合法通道。`verify.rs` 承载哈希链校验入口，提供全量校验与区间校验两种模式。`query.rs` 承载事件检索入口，提供过滤与聚合能力。

五个子模块的依赖关系为：event 是其他四个模块的共同依赖；hash 不依赖其他子模块，自身完整；append 与 verify 依赖 event 与 hash；query 仅依赖 event。

### 数据结构定义 {#data-structure}

事件数据结构定义如下，字段必须严格对齐 SPEC-004 数据契约的字段名称与类型。

#### 事件核心结构 {#event-core-structure}

必须字段共八个，不可增减。

`event_id` 是事件全局唯一标识，类型为字符串，UUID v4 格式。`event_type` 是事件类型标识，字符串，取值按 SPEC-004 事件类型层级。`timestamp` 是 ISO 8601 格式时间戳，字符串，事件流内单调递增。`actor` 是操作者子结构，子结构类型。`details` 是事件负载，JSON 对象，部分事件类型允许省略。`doc_id` 是关联文档标识，字符串，跨事件串联同一治理对象的生命周期。`prev_hash` 是前事件哈希，十六进制字符串，首事件取全零值。`event_hash` 是本事件哈希，十六进制字符串，由本事件除 event_hash 外的全部字段计算得出。

可选字段共两个。

`event_class` 是事件分类，字符串，取值可消费或仅记录，承接 SPEC-004 事件分类字段与 FM-05。`verification_result` 是校验结果，JSON 对象，部分事件类型承载。

#### 操作者子结构 {#actor-sub-structure}

操作者子结构标识事件的发起主体，三字段必须。

`actor_id` 是操作者标识，字符串，须为合法的人类标识或确定性程序标识。`actor_type` 是操作者类型，字符串，取值人类或代理或系统。`invoked_via` 是调用途径，字符串，标识操作者通过何种方式触发此事件。

合法性约束：actor_type 取值代理时，该代理须是确定性程序调用的执行者，不是 LLM 直接调用。LLM 不可作为操作者标识写入治理操作事件。

#### 哈希链计算归属 {#hash-computation-ownership}

哈希链计算归属确定性程序，不存在非确定性分支。

计算规则：每个事件的 event_hash 由该事件除 event_hash 字段外的全部必须字段与可选字段序列化后计算哈希得出。序列化顺序固定为字段名字典序。哈希算法使用 SHA-256。首个事件的 prev_hash 取全零值。后续每个事件的 prev_hash 取前一个事件的 event_hash。

验证规则：从首事件开始，逐个验证每个事件的 prev_hash 是否等于前一事件的 event_hash。任一环节不匹配则哈希链断裂。

篡改检测：篡改任一历史事件的任何字段，该事件的 event_hash 改变，导致后一事件的 prev_hash 不匹配，校验暴露篡改。

当前 trail 文件状态：trail/2026-07-27.ndjson 已实现完整哈希链。trail/2026-08-03.ndjson 的 event_id、event_hash、prev_hash 字段为空字符串，是手动阶段写入未实现哈希计算的过渡态。

### 三个接口定义 {#three-interfaces}

#### 追加写入入口 {#append-interface}

追加写入入口是事件进入事件流的唯一合法通道。

签名：接收一个待写入事件，执行写入前确定性校验，校验通过则追加到事件流末尾并返回写入确认，校验失败则拒绝写入并返回拒绝原因。

输入：待写入事件，含 event_id、event_type、timestamp、actor、details、doc_id。event_hash 与 prev_hash 由追加写入时计算填充。

输出成功时含事件 ID、事件哈希、写入时间戳，失败时含拒绝原因。

校验项：event_id 全局唯一性、时间戳单调递增、prev_hash 匹配前事件哈希、操作者标识合法性。四项校验全部由确定性规则承载，同一输入同一输出。

调用权限：追加写入入口的调用方须为确定性程序，LLM 不可直接调用。

#### 哈希链校验入口 {#verify-interface}

哈希链校验入口验证事件流的哈希链完整性。

签名：接收校验范围，验证哈希链完整性，返回校验结果。

输入：校验范围，支持全量校验与区间校验。区间校验指定起始事件 ID 与结束事件 ID。

输出成功时含校验事件数、首事件哈希、末事件哈希、校验时间戳，失败时含错误类型。错误类型覆盖哈希链断裂，即某事件的 prev_hash 与前一事件的 event_hash 不一致，与事件 ID 缺失，即无法定位校验起点两类。

#### 事件检索入口 {#query-interface}

事件检索入口提供事件查询能力，只读操作。

签名：接收检索条件，返回匹配的事件集合。

输入：filter 检索条件，支持按 event_type 过滤、按 actor_id 过滤、按时间区间过滤、按 doc_id 过滤。aggregate 聚合选项，支持按 event_type 计数、按 actor_id 计数、按时间窗口分桶计数。

输出：EventList，含匹配事件集合与匹配总数。

### 四态状态机 {#four-state-machine}

单次写入操作的生命周期对应四态状态机，承接 SPEC-004 状态转换与 FM-09 写入前校验设计约束。

Pending 是待写入态。事件已构造，等待调用追加写入入口。

Validating 是校验中态。追加写入入口已接收事件，正在执行写入前确定性校验。校验项含 event_id 唯一、时间戳单调递增、prev_hash 匹配、操作者合法。

Appended 是已追加态。校验通过，事件已写入事件流末尾，哈希链已延伸，返回事件 ID 与事件哈希。

Rejected 是被拒绝态。校验失败，事件未写入事件流，返回拒绝原因。被拒绝的事件不产生任何留痕。

状态不可逆退。Appended 态不可回退，已写入事件不可改写或删除。Rejected 态不可回退，被拒绝的事件须重新构造而非恢复。

## 构建与测试组织 {#build-test}

### 单元测试组织 {#unit-test-organization}

单元测试按 Rust 社区规范组织，测试文件置于被测模块同文件，使用 `#[cfg(test)]` 模块。

单元测试覆盖范围：

哈希链计算正确性：给定同一事件的相同输入，哈希计算结果一致。哈希链断裂检测：篡改任一字段后校验返回失败。写入前校验四项：event_id 重复拒绝、时间戳非单调拒绝、前事件哈希不匹配拒绝、操作者非法拒绝。三个接口正常路径与错误路径。

### 集成测试组织 {#integration-test-organization}

集成测试位于 `tests/` 目录，加载 trail 文件作为测试基准。

集成测试基准文件位于 `fixtures/` 目录，与测试代码分离。基准文件与 trail 源文件的关联通过路径引用维护，不复制内容。

#### 哈希链完整性测试 {#integration-hash-chain}

加载 trail/2026-07-27.ndjson，验证事件流哈希链完整性。测试验证从首事件到末事件，每个事件的 prev_hash 等于前一事件的 event_hash。

预期结果：校验成功，返回校验事件数与末事件哈希。

#### 空哈希过渡态测试 {#integration-empty-hash}

加载 trail/2026-08-03.ndjson，验证空哈希过渡态被识别为校验失败。

当前该文件的 event_id、event_hash、prev_hash 字段为空字符串，是手动阶段未实现哈希计算的过渡状态。

预期结果：校验失败，返回哈希链断裂错误。测试验证错误类型为断裂而非 ID 缺失。

### 测试运行方式 {#test-run}

单元测试通过 `cargo test` 运行。集成测试通过 `cargo test --test integration_hash_chain` 与 `cargo test --test integration_empty_hash` 分别运行。

## 物理存储约束 {#physical-storage}

事件流物理存储承接 DEC-001 物理存储最小约束第 278 至 282 行。

事件层单向写入，状态层从事件层合成，不得反向修改事件层。事件层只能追加写，禁止历史行改写。

事件流文件以文本流式追加承载，JSON 单行序列化，人类可读，不依赖专有工具。

registry.json 与 graph.json 等关键状态层文件以人类可读的纯文本表述承载，不依赖外部 schema 解释。

文档区是权威源，状态层是派生物。状态层的损坏不能阻塞文档区的可读性。

## 决策后果 {#decision-consequence}

### 决策正面后果 {#consequence-positive}

单 crate 结构避免 workspace 层级引入导致的模块边界过早固化，为 MCP server 架构定义留出决策空间。事件流模块按职责分离，五个子模块各司其职，依赖关系清晰。哈希链计算归属确定性程序，LLM 不可介入。trail 文件作集成测试基准，验证真实数据。

### 决策负面后果 {#consequence-negative}

单 crate 结构待 workspace 引入时需重构 Cargo.toml，增加迁移成本。集成测试依赖外部 fixtures 目录，路径维护需保持同步。

## 关联 {#relation}

- 源码与工具分界：DEC-001 第 49 行工程职责分界
- 物理存储最小约束：DEC-001 第 278 至 282 行
- 事件流规格：SPEC-004 接口签名、数据契约、状态机、验收依据
- 第一阶段组件协议：DES-003 事件流组件协议
- 通用格式规范：DES-001 文档格式设计
- 失败推导约束：GOV-001 FM-04 异常信号聚合、FM-05 事件分类、FM-09 写入前校验

## 认识论立场 {#epistemic-stance}

本设计的认识论立场为 design-corollary，是工程设计选择，非逻辑必然。

Cargo workspace 结构判定是工程约定，单 crate 还是 workspace 是工程实现选择，非哲学必然。模块组织方式，即 event、hash、append、verify、query 五子模块分离，是工程约定，按职责分离是 Rust 社区惯例。测试组织方式是 Rust 社区规范。

物理存储约束为外部锚定（external-anchor），承接 DEC-001 物理存储最小约束。可证伪条件：若实践中单 crate 结构导致模块边界难以维护，可重新评估 workspace 结构。

## 自检 {#self-check}

### 形式合规自检 {#formal-self-check}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为概览
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于引用治理编号

### 内容自检 {#content-self-check}

- 三条核心内容齐全：Cargo workspace 结构、事件流模块设计、构建与测试组织
- Cargo workspace 结构含判定依据与目录布局，承接 DEC-001 第 49 行与第 51 行
- 事件流模块设计含模块组织、数据结构定义，即事件核心结构八个必须字段、两个可选字段与操作者子结构三字段、三个接口定义、四态状态机，哈希链计算归属确定性程序
- 数据结构字段对齐 SPEC-004 字段名称与类型，无自行增减
- 哈希链计算规则：序列化顺序固定为字段名字典序，哈希算法 SHA-256
- 写入前校验四项：event_id 唯一、时间戳单调递增、prev_hash 匹配、操作者合法
- 构建与测试组织含单元测试覆盖范围、集成测试两项，即哈希链完整性、空哈希过渡态、测试运行方式
- 物理存储约束承接 DEC-001 物理存储最小约束第 278 至 282 行
- 范围收窄：MCP server 架构不定义，其他组件模块位置标注预留
- 认识论立场为 design-corollary

### 自反性结论 {#reflexive-conclusion}

本设计定义 sih-engine 第一阶段核心工程结构，承接 DEC-001 物理存储约束与 SPEC-004 事件流规格，范围严格收窄于事件流直接需要的三条内容，不越界展开 MCP server 架构或完整架构定义。

## 参验基准清单 {#参验基准清单}

参验由确定性程序执行，验证 DES-007 与上游规格的一致性。

参验基准一：字段完整性
: DES-007 定义的事件核心结构必须字段，即 event_id、event_type、timestamp、actor、details、doc_id、prev_hash、event_hash，共八个，与 SPEC-004 数据契约必须字段清单完全对齐，字段名称与类型一致。

参验基准二：操作者子结构字段
: DES-007 定义的操作者子结构三字段，即 actor_id、actor_type、invoked_via，与 SPEC-004 操作者子结构定义一致，actor_type 取值清单，即人类、代理、系统，与 SPEC-004 一致。

参验基准三：三个接口名称与职责
: DES-007 定义的三接口，即追加写入、哈希链校验、事件检索，与 SPEC-004 接口签名三入口，即追加写入入口、哈希链校验入口、事件检索入口，职责对应，无遗漏或额外接口。

参验基准四：写入前校验四项
: DES-007 定义的写入前校验四项，即 event_id 唯一、时间戳单调递增、prev_hash 匹配、操作者合法，与 SPEC-004 验收依据写入前校验第三条校验覆盖完全一致。

参验基准五：哈希链计算确定性
: DES-007 明确哈希链计算归属确定性程序，无非确定性分支，与 DEC-001 物理存储最小约束的事件层约束及 SPEC-004 哈希链结构定义一致。

参验基准六：目录布局不含 MCP server
: DES-007 Cargo.toml 与 src/ 目录布局不含 MCP server 相关模块，与阶段一范围约束，即 MCP server 架构不定义，一致。

参验基准七：集成测试基准文件对应
: DES-007 fixtures 目录引用 trail/2026-07-27.ndjson，即完整哈希链，与 trail/2026-08-03.ndjson，即空哈希过渡态，与任务要求两份 trail 文件作为集成测试基准一致。
