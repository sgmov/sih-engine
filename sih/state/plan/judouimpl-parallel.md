# judouimpl-parallel：句读实现批

> task-packages 治理任务
> 承接：sess-zcode-260828-judouimpl 三问意图即 2026-08-28 链事件 ab5265cc、SPEC-009 句读落差规格即 2026-08-28 judousdd 批已冻、用户开工令
> 队形：代理编组即并联形 parallel——本批即 parallel 后缀首包与分支前缀 msh 首用，双子代理编辑相加主线串行验证
> 日期：2026-08-28

## 一、问题陈述 {#problem}

句读契约即 sih-tools/parser/CONTRACT.md 立项后目录零代码，SPEC-009 已钉落差全件即语言包三件 schema、空腹 PEG 引擎、四子命令、Rust v1 声明层文法包、md 与 json 对表语言包。本批以 TDD 全量落地即 F-1 至 F-8 先红后绿，同时为并联形首个活体即双子代理在租约下的首次实跑。

## 二、关键设计 {#design}

空腹即引擎零语言知识，语言包三件纯数据零代码，新语言零引擎改动。PEG 族序选择即有序选择无回溯，同输入恒得同一树。容错两层即树级错误节点包裹与语法级数据驱动同步集恢复。v1 即 Rust 声明层全保真与表达式配平节点，九 kind 与寻址 code kind 一一对应。条目对齐寻址十字段，id 派生承 DES-009 即四段 U+0000 连接 sha256。对表基准即寻址窄域包 build 索引件只读调用。

## 三、工作清单 {#work}

- [ ] 红态建桩即 cli 四子命令桩与模块桩与测试骨架，红态输出留痕结果档
- [ ] 引擎核心即词法有序扫描与 PEG 组合子解释器与树三形节点与容错两层
- [ ] Rust v1 文法包三件与 md 与 json 对表语言包三件
- [ ] lint 三查与 entries 十字段投影与 vectors 金向量冻结
- [ ] F-1 至 F-8 全绿含崩溃语料四文件重接零崩与窄域包对表逐字段一致
- [ ] SPEC-009 修订一留痕即 schema 扩展三形（tokens 解码形与映射路径合成形与标量文本形）承实现批发现即冻结 schema 表达不了 json 载体对表
- [ ] CONTRACT 修订三冻结旗标名、管线件过件、双仓段结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 崩溃重接 | 工程治理 | 四 .rs 语料逐件 parse 产树退出码零，错误节点有无如实计 |
| **F-2** 窄域对表 | 工程治理 | 寻址窄域包百九十四文件 entries 与 build 索引件十字段逐字段一致 |
| **F-3** 双跑一致 | 确定性 | 同输入同包两跑 parse 与 entries 输出逐字节相等含残缺两形 |
| **F-4** 零依赖可证 | 工程治理 | uv tree 零分布，引擎源零 tree-sitter 引用，机械对表留输出 |
| **F-5** 纯数据接入 | 空腹 | 最小玩具语言包三件接入 git 对表引擎面零改动即 diff 空 |
| **F-6** 残缺产树 | 容错 | 截断与乱码与空文件 parse 产树退出码零树含错误节点零异常 |
| **F-7** lint 三查 | 工程治理 | 三件齐备与引用闭合含左递归拒与不可达拒与映射可达，违例各形退出码一 |
| **F-8** 退出码只读 | 工程治理 | 三值映射各证一形，源文件对表前后内容哈希不变 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-009-parser-implementation-gap.md 即落差规格全文
- 必读 2：sih-tools/parser/CONTRACT.md 即契约五节
- 必读 3：sih-tools/locator/CONTRACT.md 即对表基准调用面

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 上链前必须等绿
3. 请求写入裸路径
4. 并联形边界即子代理产出属符号材料，入库前过主线验收与工具链
5. 崩溃语料旧件不修不迁即重接测试在验收位

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-8 全绿且红态输出在结果档留痕
- [ ] 结果档落 sih/event/plan/judouimpl-parallel-results.md，管线件认证入链，双仓段结算收约

## 八、风险点 {#risks}

Rust 声明层文法面重即九 kind 全保真与配平节点边界，桩绿之间反复大。以 F-1 语料逐件观察与 F-2 窄域对表双锚兜底。双子代理首跑有文件面相交风险，以模块级文件分工硬边界消解即 X1 引擎与 Rust 包、X2 lint 与对表包与对表壳。

## 九、队形声明 {#formation}

并联形即双子代理编辑相加主线串行验证。X1 写引擎核心与 Rust v1 包与 F-1 F-3 F-6 测，X2 写 lint 与 md json 对表包与 F-2 对表壳与 F-7 测，两文件面不相交；主线写骨架红桩与 entries 与 vectors 与整合测并验收。选形即声明非偏离。

## 十、关联文件 {#related}

- 任务包源：用户开工令承 judousdd 批收口指针
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 ab5265cc
- 规格：sih-engine/doc/spec/SPEC-009-parser-implementation-gap.md

## 十一、请求写入 {#requested-writes}

- sih-tools/parser/
- sih-engine/doc/spec/SPEC-009-parser-implementation-gap.md
- sih-engine/sih/state/plan/judouimpl-parallel.md
- sih-engine/sih/event/plan/judouimpl-parallel-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
