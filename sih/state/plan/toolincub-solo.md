# toolincub-solo：文规线三工具孵化立项登记

> 治理任务包（立文类，单线形 solo，DEC-018；**委外执行**：与 redkeep-solo 同一委外令、同一代理**串行**执行，本批在前；与在途 regulamath-solo 数学批**并行**，路径零交叠）
> 承接：用户 2026-09-06 令「同意」即并行开外切立项批＋redkeep 批同一委外令串行；上游输入件即工作区根域外草稿 regula-design-draft-2026-09-06.md §二§三§四§八§九
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 文规设计稿 §九 第二批：文规检查器（格式包验证机）、TDD 验收器（四查执行机）、基线管（金向量双种共享家位）三工具的外切方向已由用户裁定，须按孵化环第二三步登记接口契约与验收判据，防融回时接口重谈判（回迁债预防）
- 三工具均未立名，命名次序须按孵化纪律先行裁定并留痕
- 译段工具（散文条款译机械条款）不在本批：其外切与否待用户第二席裁决（设计稿 §九 待裁节点）
- 在途 regulamath-solo 批独占 sih-math，本批零 sih-math 写入即并行无撞

## 二、关键设计 {#design}

### 2.1 命名次序裁定记录

用户 2026-09-06 同意本立项方案时批准**工作名先行**：三工具以工作名运行即文规检查器、TDD 验收器、基线管，正式立名后置走立名流程。连带改写面明示：正式立名时孵化登记件的正式名节回填、后继实现目录与命令名的命名对齐；本批**不立任何可改名目录位**（级联纪律路径即 id，登记件用带日期不可变文件名，零改名面）。检查器承不承文规名脉属立名会话事项，本批只注记不预判。

### 2.2 登记件落位与形

登记件一件落 `sih-tools/incubation/regula-line-tools-2026-09-06.md`，三工具各一节，节形照化格 INCUBATION.md 先例裁剪：外切判定记录（判据四条对表：通用、纯确定性、跨仓可复用、可带退出码）、本质段草案（一句话定义、不变职责、可变扩展、边界即不做什么逐条、认识论状态即已裁待裁各列）、契约草案、验收判据。引擎侧引用由结果档与链认证承载。若检词报未登记新词，工作名按懒波登记处置（word、since、source 三必填）。

### 2.3 契约面要件（三工具共同）

- 命令面：子命令与参数最小形声明
- 退出码三值语义：零过、一违规、二工具自身异常，与三现役工具（核阅、化格、检词）插座形状一致，冻结声明
- 输入输出形：报告 JSON 形与双版本戳（引擎版本加包版本）声明
- 锁步语义：无状态或显式状态位声明，禁隐式状态
- D-4 必携基线向量位：挂锚规则必携，SPEC-021 定案

### 2.4 三工具契约要点（源自设计稿，不得自行改设计）

- 文规检查器：吃格式包验证 SDD 文档族与扩展件族，判定面带 D-2 三态失败定位（缺件、违规、断链），场景语法即 OpenSpec 骨架三刀形（R-／S- 编号、THEN 判据绑定、独立成件）
- TDD 验收器：跑四查即先红后绿、场景覆盖、基线冻结、双跑一致，全退出码零人裁，甲乙红线继承 SPEC-021（基线向量只主张没变不主张对）
- 基线管：双种向量共享家位五子模块（承 gvec-v2 设计稿），冻结四元组与环境指纹位，重冻分种即基线种走漂移归因三态、规约种走先红再成立

### 2.5 待裁注记（只注记不预判）

pk-061 包容器格式（三包形并列现状，统一候裁）、pk-062 引擎共享单元层家位、pk-064 工具线公共库家位（共享代码落位候裁）、名脉（检查器承不承文规名）。注记入登记件待裁节，不以本批形态预判裁决结果。

### 2.6 命题与机器门

一命题 gid m-toolincub-contract-1，单锚即可验证性——契约条款与验收判据逐条只问能不能机械验。facet 三步加得一三步，stable_clear 即落据；boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门（核阅 ask3 包＋引擎 ask3repeater，三锚引文程序切片）
- [ ] 叩问 elicit check，信号如实处置后 digest passed
- [ ] 正身 identity attest
- [ ] watch 对表：在盘遗留如实转述（sih-math 未跟踪件与 gvecmath 残件，不豁免不代清零触碰）
- [ ] 泊界心跳两线

### Cluster 2：登记件成文

- [ ] 租约开工：--package toolincub-solo，双仓工地（sih-engine base main、sih-tools base integral-stage-build），--allow 按请求写入节；**零 sih-math 工地**
- [ ] 任务包与提示词两件从主树位拷入工地 state/plan（批输入件先例，此后零主树写）
- [ ] 孵化登记件成文（§2.2 形，三节加待裁节加命名次序裁定节）
- [ ] 工作名懒波登记（仅检词报需时）

### Cluster 3：facet 与得一

- [ ] m-toolincub-contract-1：emit-contract（零 LLM 零网络）→ 回填即席作答（谱系披露双声明即同席采样与成本加重）→ score 携正身件
- [ ] attractor check → verify（重放 identical）→ sign，stable_clear 终签入当日链

### Cluster 4：管线与结算

- [ ] 化格 general-v1 对登记件与任务包与结果档
- [ ] 核阅 des-001：sih-tools 与 state/plan 与 event/plan 均域外，exit-2 如实记档不属违规
- [ ] 检词 core 零违例
- [ ] 温故 recall 底稿；结果档 toolincub-solo-results.md 落 event/plan
- [ ] 双仓 settle（cert 取 ask3 记录认证哈希前八位）→ 放锁收约（撞主树同名未跟踪件按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔
- [ ] **收约后向同一代理确认串行下一批**即 redkeep-solo（另一任务包）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 登记件落位 | 工程 | sih-tools/incubation/ 登记件在位，三节齐即外切判定、本质段、契约草案，验收判据逐条机械可验 |
| **F-2** 终签在链 | 治理 | m-toolincub-contract-1 stable_clear 终签入当日链，verify valid |
| **F-3** 写入仅 allow | 治理 | 写入仅请求写入节所列路径，零 sih-math 写入 |
| **F-4** 委外不越权 | 编组治理 | 零代码改动、零人节点代行、零 plain commit、待裁项只注记、在泊件与在盘遗留零触碰 |
| **F-5** 并行无撞 | 跨族治理 | 与在途 regulamath-solo 零锁冲突；撞锁即排队如实呈报；链活面归并按纯追加并集超集通道，真分叉停批上报 |

## 五、必读文件 {#read}

- 孵化纪律权威源：`sih-engine/sih/state/skills/sihankor-incubation/SKILL.md`（环四步与命名次序裁定）
- 上游设计稿：工作区根 `regula-design-draft-2026-09-06.md` §二§三§四§八§九
- 登记形先例：`sih-tools/formatter/INCUBATION.md`
- 金向量双种：`sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md`
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 闸不过即停批，不硬闯不改判据不重采样凑收敛
2. 本批纯登记零代码改动，工具实现属后继批
3. 零 sih-math 写入（在途数学批独占）
4. 契约与判据源自设计稿与 SPEC-021，不得自行改设计；待裁项只注记
5. 主树零直写（批输入件先例）；守卫在位禁 plain git commit；退出码直读禁管道掩码
6. 在泊件与在盘遗留零触碰不并批
7. 中文零新造正式词，工作名按需懒波登记，正式立名后置

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 toolincub-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- facet 回填同席位采样，谱系披露双声明必须
- attractor 相对路径按调用 cwd 解析（openhyg 勘误）
- 与在途数学批共享当日链：close 归并遇链活面按纯追加并集超集通道，真分叉停批
- 登记件用带日期文件名防改名（级联纪律），不立可改名目录位

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`
- 跨仓引用：`sih-tools/incubation/`、`sih-tools/proposition/DES/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/toolincub-solo.md` 与 `toolincub-redkeep-prompt.md`（批输入件经工地落位）
- `sih-tools/incubation/regula-line-tools-2026-09-06.md`
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-engine/sih/event/plan/toolincub-solo-results.md` 与 `toolincub-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`、`sih-tools/facet/`、`sih-tools/proposition/DES/`、`sih-tools/nomenclator/packs/core/`（仅懒波需时）
- worktrees 双仓 toolincub-solo 工地
