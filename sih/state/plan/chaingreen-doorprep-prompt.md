# chaingreen-doorprep 委外提示词（串行两批）

> 用途：由用户转发给委外执行代理。两份任务包在 sih-engine/sih/state/plan/chaingreen-solo.md 与 doorprep-solo.md，本提示词是执行入口。两批**串行**：chaingreen-solo 全链收约后才开 doorprep-solo。

---

你是司衡工作区的委外执行代理，单线 solo 队形、零子代理，串行执行两批。工作区根：/Users/moc/workspaces/SiHankor。

## 开工三读

1. 读 AGENTS.md（工作区根）——治理宪法与工程基线
2. 读两份任务包 sih-engine/sih/state/plan/chaingreen-solo.md 与 doorprep-solo.md——全部判据与红线
3. 读 sih-tools/BATCH-FACE.md——批机械链逐命令 verbatim 与坑位勘误

## 批一任务一句话（chaingreen-solo，先跑）

修绿主树四查链：八件墓碑（golden 七件加 acceptor/frozen 一件）经批通道入版控（内容零改动，归并 diff IDENTICAL）；检查器 test_golden_replay 改读 basemgr/vectors/checker-golden 新家；terms.json 未提交修改 diff 查源处置（禁默改禁默弃，查证不了即停批候裁）。收口判据唯一：主树 TDD 验收工具判定包端到端退出码零，批内加收约后主树双读数。

## 批二任务一句话（doorprep-solo，批一收约后跑）

两道门备料：其一，流程包数据形成文落 incubation/packs/flow-v1（首例即治理批类：SDD 门吃 sdd-v1 五包、TDD 门吃 tdd-v0 判定包形）；其二，SDD 形任务包模板成文（问题陈述升变更提案、关键设计升技术方案、工作清单升任务清单、F 锚升 R-／S- 场景形、补规格差分一件），硬规则全吸收 TASK-PACKAGE-TEMPLATE（逐路径分行、版本位必填）显式引用不改彼件。以批二自身为变更对象按模板生成样例五件：好样例过检查器全绿、坏样例红证在档——模板用门形机器自证。

## 批机械链全序（两批各自走完整链，照 BATCH-FACE 不跳步）

每批独立：三问双门 → 叩问 → 正身 → 缺陷复现留痕（批一）→ 租约开工（--package 各批名，双仓工地，--allow 按各任务包请求写入节，**逐路径分行**）→ 书简意图 → 工地施工 → 化格核阅检词 → facet 三步 → 得一三步 → 认证 → 双仓 settle（声明件先提交后 close，差集闸在位）→ 放锁收约 → 书单对表 → 对账对表 → 主树复跑回填（批一）→ 结果档与完工报告。批一收约且报备后才开批二。

## 四条命门（违反任何一条即批失败）

1. **墓碑内容零改动**：八件只入版控不改写，归并 diff 非 IDENTICAL 即真分叉停批上报；`.bin` 期望件零触碰。
2. **terms.json 三禁**：禁默改禁默弃禁回退——diff 逐行呈报，可查证即入版控，查证不了即停批候裁。
3. **退出码直读**：一切工具调用退出码直接读，禁管道掩码（genpark-solo 勘误在案）。
4. **零越权**：批二不改 TASK-PACKAGE-TEMPLATE 原件（租约线资产）；两批零引擎零租约零 sih-math 源码写入；在泊件与在盘遗留零触碰；在途 attnanchor 与 confpreempt 批撞面即排队；人节点零代行；守卫禁 plain commit。

## 工程纪律

- 批一请求写入节已逐路径分行（facefit 模板条款）；批二模板成文后，文规线任务包自其收约起一律按 SDD 形书写
- 受检文档用检查器、TDD 验收工具、基线向量管理工具指称，规避懒波词面全称命中
- 主树零直写（每批开工后第一动作把任务包与提示词从主树拷入工地，此后零主树写）
- 每批收约后：链 verify、reconcile 双仓（退出码直读）、泊界心跳、结果档落 event/plan、CALL-LOG 落笔、向用户完工报告（含每步退出码、双仓提交号、终签哈希、越线与误差申报）

## 停批条件

得一裁不收敛；归并 diff 非 IDENTICAL；terms.json 查证不了；模板与既有任务包节的缝无法以映射表承载；需新造中文词；撞锁排队超时或链真分叉。停批即如实呈报现状，不硬闯不绕行。
