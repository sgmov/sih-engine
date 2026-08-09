# 仓库结构第一阶段扩展

本设计在 DEC-001 仓库结构决策的基础上，定义 sih-engine 第一阶段的物理目录结构。DEC-001 定义结构性元规则即五顶层节点与文档类型清单，本设计定义第一阶段的物理落地。

本设计修正 DEC-001 全节点树的根节点名错位。DEC-001 全节点树以 sihankor/ 为根节点名，但身份已改为 sih-engine。物理目录以 sih-engine/ 为根。

## 概览 {#overview}

- 第一阶段物理目录承接 DEC-001 五顶层节点，部分节点第一阶段不建::[物理目录总则](#physical-general)
- 文档区七大类型第一阶段建四个，其余三个待需要时建::[文档区目录](#doc-area)
- 治理区第一阶段建事件层 trail 子节点，状态层待第二阶段::[治理区目录](#sih-area)
- 源码与测试与工具第一阶段按最小原则建::[工程区目录](#engineering-area)
- DEC-001 全节点树根节点名错位待后续修订::[命名错位修正](#naming-fix)

## 物理目录总则 {#physical-general}

第一阶段物理目录承接 DEC-001 的五顶层节点：文档区、治理区、源码、测试、工具集。第一阶段不追求全节点覆盖，只建最小闭合治理环所需的节点。

物理存储承接 DEC-001 物理存储最小约束。事件单向写入，状态层从事件层合成。事件层只能追加写，禁止历史行改写。人类降级阅读不依赖专有工具，文本工具可直接查阅。文档区是权威源，状态层是派生物。

第一阶段物理目录如下。

```filetree
sih-engine/
├── doc/                           被持续引用的活跃文档
│   ├── decision/                  决策文档    DEC-NNN-slug
│   ├── design/                    技术设计    DES-NNN-slug
│   ├── governance/                规约        GOV-NNN-slug
│   ├── knowledge/                 知识库      KNOW-NNN-slug
│   ├── requirement/               需求        待需要时建
│   ├── spec/                      规格        待需要时建
│   └── proposal/                  提案        待需要时建
│
├── sih/                           治理层
│   ├── event/
│   │   └── trail/                 治理动作留痕    事件流追加写
│   └── state/
│       └── calibration/           标定基准集      参验机制的标定数据
│
├── src/                           源代码        待第一阶段实现时建
├── test/                          测试          待第一阶段实现时建
├── tools/                         独立 CLI 工具 待工具链迁移时建
├── tmp/                           过程性产出，不进治理链条
└── AGENTS.md                      工程仓入口，组件索引与术语映射
```

## 文档区目录 {#doc-area}

文档区七大类型按 DEC-001 定义。第一阶段已建四个：decision 承载 DEC-000 与 DEC-001、design 承载 DES-001 至 DES-004、governance 承载 GOV-001、knowledge 承载 KNOW-001。

第一阶段未建三个：requirement 需求、spec 规格、proposal 提案。这三个类型待对应文档需要起草时建目录。不预先建空目录，避免空节点膨胀。

文档区根目录不存放任何文档。所有文档按类型归位到对应子目录。文档身份由存放路径与文件名共同确定，放根目录则身份模糊。KNOW-001 已从 doc/ 根目录归位到 doc/knowledge/。

## 治理区目录 {#sih-area}

治理区按 DEC-001 事件与状态二分。第一阶段只建事件层的 trail 子节点。

事件层 trail 子节点承载治理动作留痕，即 DES-003 定义的事件流。事件流以 JSON 序列化的单行文本追加写，含哈希链。物理路径 sih/event/trail/，按日期分文件承载，文件名格式 YYYY-MM-DD.ndjson。

事件层 experiment 子节点已废弃。实验数据经范畴重判为参验机制的标定基准，归状态层 calibration 子节点，不再归事件层。详见 DEC-001 状态层 calibration 节点声明。

第一阶段不建事件层的 feedback、challenge、report 子节点。这三个子节点待对应治理动作产生时建。

状态层 calibration 子节点承载参验机制的标定基准集，即 DES-005 定义的方法学与 SPEC-001 定义的执行规格的产出。物理路径 sih/state/calibration/，按标定批次分目录。calibration 的实验设计受微积分与概率论与数理统计启发，用 N 次隔离采样标定语义判断的收敛基准。calibration 是已标定的基准值，不是探索性实验。

第一阶段不建状态层的 registry、graph、view、plan 子节点。registry 文档身份与状态登记、graph 文档关系投影、view 派生视图是事件层的派生物，plan 待执行意图编排投影待对应场景产生时建。第一阶段的事件流规模小，人类可直接查阅事件流，不依赖状态层合成。

## 工程区目录 {#engineering-area}

源码、测试、工具按 DEC-001 工程职责分界。

第一阶段不建 src 目录。源代码即意图锚定组件、符号材料生成接口、参验、事件流写入器的实现，待第一阶段进入代码实现时建。第一阶段当前是手动阶段，协助人类决策者模拟司衡引擎的治理动作。

第一阶段不建 test 目录。测试待源码实现时建。

第一阶段不建 tools 目录。静态审计工具暂借旧仓二进制即 sihankor/tools/doclint/target/release/sih-doclint。工具链迁移到 sih-engine 时建 tools 目录。

tmp 目录承载过程性产出，不进治理链条。任务包、失败复盘等过程性文档放在 tmp/，通过审阅后其内容拆解为正式的 decision、design、proposal 文档。

## 命名错位修正 {#naming-fix}

本节记录一次立名本体论的工程应用。DEC-001 全节点树根节点名与 DEC-000 术语表脚注原为 sihankor/，与工程产物身份 sih-engine 错位。根 AGENTS.md 项目身份小节此前已修正为 sih-engine，但 DEC-000 与 DEC-001 内的锚定未同步，形成部分修订留下的不一致。

本轮已修正三处。DEC-000 术语表脚注由司衡引擎 sihankor 修正为司衡引擎 sih-engine。DEC-001 全节点树根由 sihankor/ 修正为 sih-engine/。DEC-001 正文由司衡引擎 sihankor 修正为司衡引擎 sih-engine。

修正的性质是命名承诺的同步，不是撤回。sihankor 作为哲学身份即司衡 SiHankor 的命名承诺不被撤回。sih-engine 作为工程产物的身份锚定承接哲学身份，两者分层不冲突。

## 关联 {#relation}

- 元规则：DEC-001 仓库结构决策
- 通用规范：DES-001 文档格式设计
- 组件协议：DES-003 第一阶段组件清单与协议
- 失败推导约束：GOV-001 失败推导的禁止条款与设计约束

## 认识论立场 {#epistemic-stance}

本设计为 design-corollary，是工程设计选择。物理目录的第一阶段最小化是工程约定，承接 DEC-001 结构性元规则。命名错位修正是立名本体论的工程应用，名字须与本体一致。

可证伪条件：若第一阶段实践中发现某个待建目录在第一阶段即需要，则须提前建立。若状态层在第一阶段即需要合成，则须提前建立状态层。

## 自检 {#self-check}

### 形式合规自检 {#formal-self-check}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为概览
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于单一治理编号

### 内容自检 {#content-self-check}

- 物理目录承接 DEC-001 五顶层节点，第一阶段最小化建节点
- 文档区七大类型第一阶段建四个，未建三个标注待需要时建
- 治理区第一阶段只建事件层 trail，状态层标注待第二阶段
- 命名错位修正独立承载，以 sih-engine 为物理根，DEC-001 修订建议标注不在本设计范围

### 自反性结论 {#reflexive-conclusion}

本设计定义第一阶段物理目录结构。本设计本身经过自我审视，未发现违反 DES-001 格式规范的形态。
