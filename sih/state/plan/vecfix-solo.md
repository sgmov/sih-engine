# vecfix-solo：向量可移植性修复与四查链复绿

> 治理任务包（工程类，单线形 solo，DEC-018；**委外执行**：单代理持本任务包与提示词连续执行全链）
> 承接：用户 2026-09-07 令「出委外」即基线向量管理工具批缺陷修复；缺陷证据即同日收约后主树复验（重放实跑可复现，命令形见 §一）；两道门批以本批收口为前置（门不挂红链）
> 日期：2026-09-07

## 一、问题陈述 {#problem}

basemgrimpl-solo 批收约后主树复验发现两处实质缺陷：

- **缺陷一（七支向量嵌死工地路径）**：vectors 下 12 支中 7 支（checker-golden 六支加 self/cli-help 一支）的冻结命令串嵌已拆除的工地绝对路径 `worktrees/sih-tools/basemgrimpl-solo/...`。原令重放必败（实测退出码 1，drift_fact 即退出码 2 期望 0、零字节输出对期望 40 字节）；路径正确映射回主树后重放即绿（实测退出码 0，verdict pass）——内容与机制无损，坏在可移植性。属 SPEC-021 明文 V6 教训复发（冻结夹具绝对路径），病根即 freeze 操作不做归一。
- **缺陷二（迁移依赖闭包漏角，四查链红）**：迁移把检查器 manifest 进位 0.3.0，但检查器测试 `test_packs_machineized` 钉 `version == "0.2.0"` 字面（实测 1 failed），连锁把 TDD 验收工具判定包 TC-003（先红后绿的当前绿腿即检查器测试套）跑红——判定包端到端实测退出码 1（TC-001、TC-002、TC-004 仍过）。四查链现状红。

复现形：`basemgr replay --vector <七支之一> --command-json <冻结命令> --cwd <四元组 command_cwd> --root <工作区根>` 退出码 1；`cd sih-tools/checker && uv run pytest tests/ -x -q` 1 failed；`cd sih-tools/acceptor && uv run acceptor --pack packs/tdd-v0-sddchecker.json --root <工作区根>` 退出码 1。

## 二、关键设计 {#design}

### 2.1 七支向量归一重冻（期望字节零变）

命令串内工作区根前缀改 `{ROOT}` 展开形（对齐 TDD 验收工具判定包先例）。重冻的修复不变量：**重冻前后 expected_sha256 逐支一致**——路径归一是寻径层不是语义层，哈希漂移即停批上报（说明差异超出纯路径面）。`.bin` 期望件零触碰。向量 kind 标注不在本批修复面，如实申报不重标。

### 2.2 freeze 侧归一规则

基线向量管理工具的 freeze 操作输出命令时对工作区根前缀自动 `{ROOT}` 归一（引擎最小改动面）；实装后新冻结一支验证归一生效。replay 的 `--root` 展开通道既有，零改。

### 2.3 数据面守卫与版本钉修复（先红后绿）

- 基线向量管理工具测试面新增 grep 守卫：`vectors/` 数据零 `worktrees/` 字面——与 src 面空腹测试互补即数据面设防。守卫测试先行在污染数据上跑红（红证归档），修复后绿。
- 检查器测试版本钉改从 manifest 动态读断言一致性（零硬编码字面），同样先红后绿。

### 2.4 收口判据（唯一）

四查链复绿：TDD 验收工具判定包端到端退出码零、TC-001 至 TC-004 全过——批内绿加**收约后主树复跑绿**双读数（收约后主树补跑先例即 checkerimpl 幂等补跑形，读数回填结果档补笔节）。

### 2.5 命题与机器门

一命题 gid m-vecfix-1，单锚即可验证性。facet 三步加得一三步，stable_clear 即落据；boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门（三锚引文程序切片）、叩问、正身、watch 对表（在盘遗留如实转述不代清）、泊界心跳两线、例行读数
- [ ] 缺陷复现三项（§一复现形）留痕入批材料即修复前红态基线

### Cluster 2：施工

- [ ] 租约开工：--package vecfix-solo，双仓工地（sih-engine base main、sih-tools base integral-stage-build），--allow 按请求写入节；改动面仅基线向量管理工具 src 与 tests 与 vectors、检查器 tests、批件面
- [ ] 任务包与提示词拷入工地 state/plan（批输入件先例）
- [ ] 守卫测试先行写、污染数据上跑红、红证归档
- [ ] 七支向量归一重冻（逐支 expected_sha256 前后对表入档）
- [ ] freeze 归一规则实装（新冻结一支实测归一）
- [ ] 检查器版本钉改动态对表
- [ ] 批内验证：守卫绿、检查器套全绿、判定包端到端退出码零

### Cluster 3：管线、facet 与结算

- [ ] 化格：JSON 过 json-canonical-v1、Markdown 过 general-v1；检词 core 零违例（懒波词面全称规避）
- [ ] 核阅 des-001：域外 exit-2 如实记档
- [ ] m-vecfix-1 facet 三步加得一三步，stable_clear 终签入当日链
- [ ] 温故 recall 底稿；结果档 vecfix-solo-results.md 落 event/plan
- [ ] 双仓 settle（差集闸在位，声明件先提交后 close）→ 放锁收约（撞主树同名未跟踪件按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔
- [ ] **收约后主树复跑**：判定包端到端退出码零读数回填结果档补笔节（§2.4 双读数之后半）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 七支归一 | 工程 | vectors/ 零 `worktrees/` 字面（grep 守卫在档且绿）；重冻前后 expected_sha256 逐支一致对表在档 |
| **F-2** freeze 归一规则 | 工程 | 新冻结一支命令自动 `{ROOT}` 归一实测在档 |
| **F-3** 检查器套复绿 | 工程 | pytest 全绿零失败（版本钉动态对表零硬编码） |
| **F-4** 四查链复绿 | 治理 | 判定包端到端退出码零 TC 全过，批内与收约后主树双读数在档 |
| **F-5** 终签在链 | 治理 | m-vecfix-1 stable_clear 终签入当日链，verify valid |
| **F-6** 写入仅 allow 且不越权 | 治理 | 改动面仅 §三所列；零引擎零租约零 sih-math 源码写入；`.bin` 期望件与 kind 标注零触碰；在泊件与在盘遗留零触碰 |

## 五、必读文件 {#read}

- 缺陷对象：`sih-tools/basemgr/`（CONTRACT、vectors、tests）、`sih-tools/checker/tests/test_engine.py`
- 判定包：`sih-tools/acceptor/packs/tdd-v0-sddchecker.json`（收口判据载体）
- 归一形先例：该判定包内 `{ROOT}` 用法
- V6 教训条款：`sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md` 第一层与环境指纹节
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 期望字节零变：任一支 expected_sha256 漂移即停批上报，不重冻掩盖
2. 最小改动面：只动基线向量管理工具 src 与 tests 与 vectors、检查器 tests、批件面；先红后绿（守卫与版本钉测试先行红证归档）
3. 零租约零引擎零 sih-math 源码写入；`.bin` 期望件与 kind 标注零触碰
4. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
5. 在泊件与在盘遗留零触碰不并批；checkerimpl 红证原档只读
6. 中文零新造正式词；受检文档用基线向量管理工具与 TDD 验收工具指称规避懒波词面

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 vecfix-solo-results.md 落 event/plan 含缺陷复现红态节与主树复跑绿读数补笔节

## 八、风险点 {#risks}

- 重冻遇哈希漂移即真语义差异，停批上报不掩盖
- 归一规则须只归一工作区根前缀，不得误伤非工作区绝对路径（如系统解释器路径）——归一范围以工作区根为界并测试覆盖
- 收约后主树复跑若遇环境差异红，如实分列不硬闯（先例即幂等补跑分列）
- 与在途批共享面：撞锁排队如实呈报，链归并按纯追加并集超集通道，真分叉停批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`
- 跨仓引用：`sih-tools/basemgr/`、`sih-tools/checker/`、`sih-tools/acceptor/`、`sih-tools/proposition/DES/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/vecfix-solo.md` 与 `vecfix-solo-prompt.md`（批输入件经工地落位）
- `sih-tools/basemgr/`（src 归一规则、tests 守卫、vectors 七支归一）
- `sih-tools/checker/tests/`（版本钉动态对表）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/vecfix-solo-results.md` 与 `vecfix-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`、`sih-tools/facet/`、`sih-tools/proposition/DES/`
- worktrees 双仓 vecfix-solo 工地
