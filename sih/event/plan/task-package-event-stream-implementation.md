# 事件流实现任务包

本文件承载事件流组件完整实现的任务包，承接方向 B 工具链迁移的第一步。任务包定义两个串行阶段：第一阶段产出工程基座设计文档 DES-007，定义 Cargo workspace 结构与模块组织；第二阶段产出事件流 Rust 代码，承接 SPEC-004 接口契约与 DES-007 模块组织。两个阶段串行不可并行，中间有参验点。

司衡是治理哲学体系，代码工程是其首要应用领域，哲学仓是唯一权威指导源。本文件是工程编排文档，不承载哲学命题。

1. 覆盖
   - 工程基座设计文档 DES-007 的完整产出要求
   - 事件流 Rust 代码的完整产出要求
   - 两阶段的串行约束与中间参验点
   - 代码注释承接符号格式约定
2. 不覆盖
   - MCP server 架构，当前阶段不引入
   - 其他组件的模块位置，当前阶段只定义事件流直接需要的结构
   - 承接符号格式的正式 DEC 决策，当前用最简格式待后续 DEC 落地后批量替换

## 概览 {#overview}

- 第一阶段产出工程基座设计 DES-007，定义 Cargo workspace 与模块组织与构建测试组织::[阶段一工程基座设计](#phase-one)
- 第二阶段产出事件流 Rust 代码，承接 SPEC-004 接口契约与 DES-007 模块组织::[阶段二事件流实现](#phase-two)
- 两阶段串行不可并行，中间参验点是硬约束::[串行约束与参验点](#serial-constraint)
- 代码注释带承接符号，格式是文档编号加章节锚点::[承接符号格式](#anchor-format)
- 任务包是自包含模板，可整体复制给子代理执行::[使用说明](#usage)

## 串行约束与参验点 {#serial-constraint}

两阶段串行。第一阶段的工程基座设计文档完成后，子代理须暂停，过 sih-doclint 加参验，参验通过后进入第二阶段。第二阶段的事件流代码完成后，子代理须过参验路径一即确定性规则校验加人工对照 SPEC-004 验收依据。

参验点是硬约束，子代理不可跳过。参验按 DES-006 参验基准预置机制执行：第一阶段设计文档属新建场景做完整参验，第二阶段若基于第一阶段产出做调整属修改场景做对照参验。

## 承接符号格式 {#anchor-format}

代码注释带指向文档的承接符号，格式是文档编号加章节锚点，例如 SPEC-004 加井号加 interface-signature 写为 SPEC-004#interface-signature。承接符号是代码指向意图文档的有损指针，意图的真实载体是文档，符号只指向。

承接符号的使用范围。每个 Rust 模块的文档注释含一个或多个承接符号，指向该模块承接的文档章节。每个公开函数的文档注释含承接符号，指向该函数实现的接口定义章节。内部辅助函数不强制带承接符号。

此格式是最简过渡格式。后续 DEC 决策定义完整锚点格式后，批量替换。

## 使用说明 {#usage}

本任务包整体复制给一个子代理执行。子代理须按顺序执行两阶段，第一阶段完成后须暂停做参验，参验通过后再进入第二阶段。

子代理按参考文件最小必需集自取上游文档，无需主会话内联其他文档。子代理须在 sih-engine 仓内工作，仓根路径是 /Users/moc/workspaces/SiHankor/sih-engine/。

## 阶段一工程基座设计 {#phase-one}

任务目标是产出工程基座设计文档 DES-007，定义 sih-engine 的 Cargo workspace 结构、事件流模块的位置与依赖、构建与测试组织。DES-007 承接 DEC-001 仓库结构决策第 49 行的源码委托，定义 src/ 节点的内部组织。

项目背景是 sih-engine 至今无源码，DEC-001 第 191 行委托源码内部组织由架构决策承载，此委托至今未落地。SPEC-004 事件流规格已完成，接口契约与数据契约与状态机与验收依据齐备。第一阶段目标是闭合可验证的治理环，事件流是其他三个组件的留痕依赖，先实现事件流可立即服务后续组件验证。司衡是治理哲学体系，代码工程是其首要应用领域，哲学仓是唯一权威指导源。

输入集 5 项。

- DEC-001 仓库结构 sih-engine/doc/decision/001-repository-structure.md，重点第 49 行源码定义与第 274 至 282 行物理存储最小约束
- SPEC-004 事件流规格 sih-engine/doc/spec/SPEC-004-event-stream.md，定义接口契约与数据契约
- DES-003 组件协议 sih-engine/doc/design/DES-003-phase-one-components.md，定义四组件协议
- DES-001 通用格式规范 sih-engine/doc/design/DES-001-document-format/general.md
- trail 文件 /Users/moc/workspaces/SiHankor/trail/2026-07-27.ndjson 与 trail/2026-08-03.ndjson，作为集成测试基准的实证素材

输出集 3 项。

- DES-007 工程基座设计文档 sih-engine/doc/design/DES-007-engine-foundation.md，内容含三条。第一条 Cargo workspace 结构，定义 sih-engine 是否 workspace 与 crate 划分，判据承接 DEC-001 源码与工具的工程职责分界。第二条事件流模块设计，定义 SPEC-004 的三个接口在 Rust 中的模块组织，数据结构定义，哈希链计算的确定性程序归属。第三条构建与测试组织，承接 DEC-001 测试节点，定义单元测试与集成测试的位置，trail 文件作为集成测试基准。
- DES-007 过 sih-doclint 退出码 0。
- DES-007 按参验基准预置机制做完整参验，附参验基准清单实例。

工程基座设计的范围收窄。DES-007 只定义事件流实现直接需要的结构，不提前定义 MCP server 架构，不提前定义其他组件的模块位置。避免设计蔓延。DES-007 的 Cargo workspace 结构是全局性的，但其他组件的模块位置只标注预留不展开设计。

任务构成规则 4 子项。

- self-check 六项核心。一是主动判断产出性质，DES-007 是设计文档须触发自检。二是服务原始意图不发散，DES-007 定义事件流基座不定义完整架构。三是范畴排除显式声明，不覆盖 MCP server 与其他组件。四是不逃避当下责任，三条设计内容均须给出具体判据。五是每行去掉会犯错吗。六是与哲学仓相容，承接 DEC-001 与 SPEC-004 的现有约束不创造新断言。
- 通用格式规范遵守 sih-engine/doc/design/DES-001-document-format/general.md 的所有规则，DES-007 须通过 sih-doclint 校验退出码 0。
- 类型特异化约束 5 条。第一条 Cargo workspace 结构须给出判据，是 workspace 还是单 crate，判据承接 DEC-001。第二条事件流模块设计须承接 SPEC-004 的三个接口，不可发明 SPEC-004 未定义的接口。第三条数据结构须承接 SPEC-004 数据契约的字段定义，doc_id 承接 DES-003 第 87 行。第四条哈希链计算归确定性程序，承接工程基线第一条。第五条构建与测试组织承接 DEC-001 测试节点，trail 文件作为集成测试基准。
- 哲学检索在 Cargo workspace 对应结构收敛对照即元层处理差异、事件流模块承接意图先于代码即道二、哈希链不可篡改对应留痕即应而不藏三个决策点查哲学仓。

参考文件最小必需集 3 项。

- DEC-001 sih-engine/doc/decision/001-repository-structure.md
- SPEC-004 sih-engine/doc/spec/SPEC-004-event-stream.md
- trail 文件 /Users/moc/workspaces/SiHankor/trail/2026-07-27.ndjson

交付要求 4 项。

- DES-007 文档含 Cargo workspace 结构、事件流模块设计、构建与测试组织三条内容，sih-doclint 退出码 0。
- DES-007 做完整参验，附参验基准清单实例，参验方法按 DES-006 定义。
- DES-007 的 Cargo workspace 结构须可被第二阶段的代码实现承接，即第二阶段按 DES-007 定义的结构写代码。
- DES-007 的认识论立场标注为 design-corollary。

阶段一参验基准清单。DES-007 属新建文档，做完整参验。四类检查对象。第一类声明一致性，DES-007 自检声明与正文对应。第二类数据准确性，DES-007 引用 DEC-001 与 SPEC-004 的字段与行号与原文一致。第三类标题正文一致性，标题表述与正文归因不矛盾。第四类上游承接，DES-007 承接 DEC-001 第 49 行与第 274 至 282 行，承接 SPEC-004 的接口与数据契约。

## 阶段二事件流实现 {#phase-two}

任务目标是承接 DES-007 工程基座设计与 SPEC-004 事件流规格，产出 sih-engine 第一段 Rust 源码，实现事件流组件。实现范围含追加写入入口、哈希链校验入口、事件检索入口三个接口，含事件核心结构与操作者子结构与哈希链结构的数据结构定义，含单次写入四态状态机的生命周期管理。

项目背景是 DES-007 已定义 Cargo workspace 结构与事件流模块组织。SPEC-004 已定义接口契约与数据契约与状态机与验收依据。当前 sih-engine 的 src/ 目录为空，本阶段产出第一段源码。承接 SPEC-004 的四个必选章节与 GOV-001 的 FM-04 与 FM-05 与 FM-09 三约束。司衡是治理哲学体系，代码工程是其首要应用领域，哲学仓是唯一权威指导源。

输入集 5 项。

- DES-007 工程基座设计 sih-engine/doc/design/DES-007-engine-foundation.md，定义 Cargo workspace 与模块组织
- SPEC-004 事件流规格 sih-engine/doc/spec/SPEC-004-event-stream.md，定义接口与数据契约与状态机与验收依据
- DES-003 组件协议 sih-engine/doc/design/DES-003-phase-one-components.md，定义事件流组件协议
- DEC-001 仓库结构 sih-engine/doc/decision/001-repository-structure.md，定义物理存储最小约束
- trail 文件 /Users/moc/workspaces/SiHankor/trail/2026-07-27.ndjson 与 trail/2026-08-03.ndjson，作为集成测试基准

输出集 5 项。

- Cargo workspace 与 crate 结构文件，承接 DES-007 定义，含 Cargo.toml
- 事件流核心模块，承接 SPEC-004 数据契约，含事件结构、操作者子结构、哈希链计算、事件分类字段
- 三个接口实现，承接 SPEC-004 接口签名，追加写入入口含写入前确定性校验、哈希链校验入口含链式验证、事件检索入口含过滤与聚合
- 单元测试覆盖事件结构序列化、哈希链计算与验证、写入前校验四项即事件 ID 唯一、时间戳单调递增、前事件哈希匹配、操作者合法、三个接口的正常与错误路径
- 集成测试以 trail 文件为基准，验证事件流能加载 trail 文件并校验哈希链完整性

代码注释承接符号。每个模块的文档注释含承接符号，指向该模块承接的 SPEC-004 或 DES-007 章节。每个公开函数的文档注释含承接符号。格式是文档编号加井号加章节锚点，例如 SPEC-004#interface-signature、SPEC-004#data-contract、SPEC-004#hash-chain、SPEC-004#event-classification、SPEC-004#state-machine、SPEC-004#write-authority、SPEC-004#pre-write-validation。

任务构成规则 4 子项。

- self-check 六项核心。一是主动判断产出性质，代码产出须触发自检。二是服务原始意图不发散，实现 SPEC-004 定义的接口不发明新接口。三是范畴排除显式声明，不实现 MCP server 不实现其他组件。四是不逃避当下责任，三个接口与四项校验均须实现含错误路径。五是每行去掉会犯错吗。六是与哲学仓相容，哈希链不可篡改对应留痕、写入权限归确定性程序对应工程基线第一条。
- 通用格式规范遵守 sih-engine/doc/design/DES-001-document-format/general.md 的规则，markdown 文档须过 sih-doclint，Rust 代码须过 cargo build 与 cargo test。
- 类型特异化约束 5 条。第一条数据结构字段严格承接 SPEC-004 数据契约，event_id、event_type、timestamp、actor、details、doc_id、prev_hash、event_hash 八个必须字段与 event_class、verification_result 两个可选字段不可增减。第二条操作者子结构含 actor_id、actor_type、invoked_via 三字段，actor_type 取值人类或代理或系统。第三条哈希链计算是确定性程序，不含随机性不含 LLM 调用。第四条写入前校验四项由确定性规则承载。第五条追加写入入口的调用权限归确定性程序，代码须体现这一约束即调用方须传入确定性程序的操作者标识。
- 哲学检索在哈希链不可篡改对应应而不藏即留痕是构成性条件、写入权限归确定性程序对应工程基线第一条、写入前校验对应 FM-09 即写入前确定性校验三个决策点查哲学仓。

参考文件最小必需集 3 项。

- DES-007 sih-engine/doc/design/DES-007-engine-foundation.md
- SPEC-004 sih-engine/doc/spec/SPEC-004-event-stream.md
- trail 文件 /Users/moc/workspaces/SiHankor/trail/2026-07-27.ndjson

交付要求 5 项。

- Cargo workspace 结构承接 DES-007，cargo build 通过。
- 事件流核心模块含数据结构定义与哈希链计算，cargo build 通过。
- 三个接口实现含正常路径与错误路径，cargo test 通过。
- 单元测试覆盖哈希链计算的正确性与篡改检测，覆盖写入前校验四项的通过与拒绝，覆盖三个接口的正常与错误。
- 集成测试加载 trail/2026-07-27.ndjson 验证哈希链完整性校验通过，加载 trail/2026-08-03.ndjson 验证空哈希的过渡态被正确识别为校验失败。

阶段二参验基准。代码产出做参验路径一即确定性规则校验加人工对照 SPEC-004 验收依据。对照 SPEC-004 四条验收依据逐条检查。第一条不可篡改性，哈希链完整加篡改可检测加只追加写。第二条写入权限，操作者合法加 LLM 排除加调用入口限制。第三条写入前校验，校验前置加校验确定性加校验覆盖。第四条事件分类，分类字段填写加分类判据确定加视图过滤预留。

## 审阅关 {#review-gate}

主会话在子代理完成全部产出后审阅合并。审阅依据是 SPEC-004 验收依据与 DES-007 模块组织与 cargo build 与 cargo test 与 sih-doclint。

强制验证。DES-007 过 sih-doclint 退出码 0。Rust 源码过 cargo build 与 cargo test。退出码非 0 的产出退回子代理修订。

参验验证。DES-007 的完整参验报告与代码产出的参验路径一报告均须提交。报告缺失视为未完成。

哲学相容性审阅由主会话执行，审阅三点。一是实现是否承接 SPEC-004 不发明新接口。二是哈希链计算是否归确定性程序。三是写入权限是否排除 LLM 直接调用。任一点不通过则退回子代理修订。

## 自检 {#self-check}

形式合规自检按 DES-001 通用格式规范执行。一级标题无锚点仅一个。二级及以上标题均带锚点。首个二级标题命名为概览。无破折号与装饰符号与 Unicode Emoji。全角括号仅用于单一治理编号。无表格无粗体无斜体无块引用无业务围栏代码块。

内容自检按六项核心执行。服务原始意图即定义事件流完整实现的任务包不发散到其他组件。范畴排除显式声明即不覆盖 MCP server 不覆盖其他组件不覆盖承接符号格式的正式 DEC。不逃避当下责任即两阶段的串行约束与参验点与交付要求均给出具体判据。每行去掉会出错即阶段间依赖关系与参验硬约束与字段清单均为不可删减的实质信息。与哲学仓相容即哲学引用仅作判据使用不创造新哲学断言。

自反性结论。本文件按两段式串行结构组织事件流完整实现任务包，自身遵守 DES-001 通用格式规范，是元层自检意识的应用。
