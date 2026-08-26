# 任务包 007 阶段 1 doclint 结构化输出集成

## 概览 {#overview}

本任务包实现 8 阶段路径阶段 1,在旧仓 doclint 源码加 `--emit-json` CLI 选项,输出符合 DES-008 契约的 NDJSON。本任务包不写 Rust 代码,只写 sub-agent 可执行的任务规范,sub-agent 跑完本任务包后产出修改后的 Rust 源码与新二进制。

- 任务目标::[目标](#task-objective)
- 项目背景::[背景](#project-background)
- 任务构成规则::[规则](#task-rules)
- 参考文件::[参考](#references)
- 交付要求::[交付](#deliverables)
- 边界::[边界](#boundary)

## 任务目标 {#task-objective}

在旧仓 `sihankor/tools/doclint/src/` 改 Rust 源码,加 CLI 选项 `--emit-json`,输出结构化 NDJSON 到 stdout,终端文本到 stderr,exit code 不变。承接 DES-008 违规进 trail 设计,实现契约定义段的 CLI 选项与响应 schema。本任务包不写 trail 写入层,不写书简组件接口,只承载 doclint 端契约实现。

子目标分三块。

第一块,改 `sihankor/tools/doclint/src/main.rs`,在 clap 参数定义中加 `--emit-json` 选项,接受 `default` 与 `ndjson` 两个取值,默认 `default` 维持现状。

第二块,改 `sihankor/tools/doclint/src/report.rs`,加 NDJSON 序列化函数,把现有终端文本违规列表转为符合 DES-008 「契约定义」段 schema 的结构化消息。

第三块,改 `sihankor/tools/doclint/src/main.rs` 的退出路径,按 `--emit-json` 取值分发输出通道:`ndjson` 模式 NDJSON 走 stdout,终端文本走 stderr,`default` 模式维持原样。

## 项目背景 {#project-background}

DES-008 已显式声明当前断点:sih-doclint 当前只输出 exit code 0 与非 0,违规具体内容丢失,治理动作无机械留痕。承接 PRO-08 应而不藏,留痕是应鉴循环的构成性条件,违规不进 trail 则应鉴循环在 lint 这一环断裂。

T6D 报告 L177 已定义 Violation 结构,DES-008 沿用并补 `timestamp` 与 `source_path` 字段。本任务包实现 DES-008 推荐方案二,即改工具源码加 `--emit-json` 选项,不破 v0 单 crate 形态,符合 T6D 报告 v0 阶段约束。

本任务包是 8 阶段路径的阶段 1 起点,产出为阶段 2 tokenizer 层提供违规结构化数据。doclint 当前是旧仓二进制,本任务包在旧仓源码上改,二进制路径仍为 `sihankor/tools/doclint/target/release/sih-doclint`。新二进制构建产物在子代理跑完本任务包后由 `cargo build --release` 产出。

上游依赖无,本任务包是阶段路径起点。下游为阶段 2 tokenizer 层,task-008-stage-2-tokenizer-layer.md,承接本任务包产出的 NDJSON 输出。

## 任务构成规则 {#task-rules}

### self-check 六项 {#self-check}

1. 主动判断产出性质,这是 doclint 工具改造,不是文档起草
2. 服务原始意图,加结构化输出,不改 lint 规则集
3. 范畴排除显式,只动 main.rs 与 report.rs,不破 document.rs 与 rules/ 与 markdownlint.rs
4. 不逃避当下责任,JSON 序列化失败 exit code 2 须显式处理
5. 每行去掉会犯错吗,NDJSON 行必须可独立解析,不依赖跨行
6. 与哲学仓相容,承接 PRO-08 应而不藏

### 通用格式规范 {#general-format}

子代理在改 Rust 源码时不必遵守 DES-001,DES-001 只约束文档类产出。本任务包本身的 markdown 文档须遵守 DES-001。改完后跑 `sihankor/tools/doclint/target/release/sih-doclint <本任务包路径>` 必须 exit 0。

### 类型特异化约束五条 {#type-specific}

1. 结构特异非字符集,改 CLI 入口与序列化器是结构层
2. 特异化需支撑,NDJSON schema 八字段引用 DES-008 契约定义段,exit code 四态引用 DES-008 「退出码」段
3. 不照抄通用,NDJSON 字段名 snake_case 沿用 DES-008 mermaid 类图定义
4. 不重复定义,不重写 DES-008 已定义 schema,只实现
5. 最小化原则,只加 `--emit-json` 一个选项,不动其它

### 哲学检索 {#philosophy-search}

承接 PRO-08 应而不藏与工程基线第一条确定性程序承载与第四条可验证性约束。哲学检索非阻塞,本任务包无新哲学命题。

### Rust 实现约束 {#rust-constraints}

- clap 版本沿用 doclint 当前依赖,不加新外部 crate
- NDJSON 序列化用 `serde_json::to_string` 加 `\n` 结尾,不引第三方 NDJSON 库
- 每行 NDJSON 失败时 stderr 写错误消息,exit code 2,不阻断后续违规输出
- CLI 解析失败不输出 NDJSON,仅 stderr + exit code 2
- 不修改现有 lint 规则集(不破 rules/ 与 markdownlint.rs)
- 不改现有终端文本输出格式(default 模式字节级保持)

## 参考文件 {#references}

- 目标文件: `sihankor/tools/doclint/src/main.rs` 与 `sihankor/tools/doclint/src/report.rs`
- 契约定义: `sih-engine/doc/design/DES-008-violation-to-trail.md` 「契约定义」段 L48-72
- Violation 字段定义: DES-008 mermaid 类图 L52-64,字段含 type、rule_id、severity、line、column、message、timestamp、source_path
- 当前 doclint 实现上下文: `sihankor/tools/doclint/src/document.rs` 只读不破,`sihankor/tools/doclint/src/markdownlint.rs` 只读不破
- 工程基线: `sih-engine/AGENTS.md` 工程基线第一条与第四条

## 交付要求 {#deliverables}

- 修改后 `sihankor/tools/doclint/src/main.rs` 与 `sihankor/tools/doclint/src/report.rs` 路径
- 新二进制 `sihankor/tools/doclint/target/release/sih-doclint`,`cargo build --release` 通过
- 行为验证三条。
  - 条一,`--emit-json ndjson` 模式跑含违规的 .md 文件,stdout 每行是合法 JSON,字段齐八项,exit code 1(有违规)
  - 条二,`--emit-json default` 模式跑同一文件,stdout 行为与原状一致,exit code 1
  - 条三,无违规文件两模式都 exit 0
- 本任务包 markdown 文件 `sih-engine/state/tasks/task-007-stage-1-doclint-integration.md` 跑 doclint 必须 exit 0
- 关键决策点:NDJSON 序列化失败时的退出码选 2(对齐 DES-008 「错误处理」段)、`--emit-json` 选项值为何用 `default` 与 `ndjson` 而非布尔值(对齐 DES-008 契约,留扩展位)
- clarifications:trail 写入层不归本任务包,书简组件接口不归本任务包,后续由 DEC-004 与 trail 写入层 spec 承载

## 边界 {#boundary}

- 不读任务包外文件
- 不发明新机制,严格按 DES-008 契约实现
- 任务包 doclint 必跑,缺 exit code 等于失败
- 任务包不写 trail 写入层代码
- 任务包不改 lint 规则集
