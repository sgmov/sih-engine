# sddpacks-solo：SDD 五件格式包一族成文

> 治理任务包（立文类，单线形 solo，DEC-018；**委外执行**：单代理持本任务包与提示词连续执行全链）
> 承接：用户 2026-09-07 令「租约还在修复中。我们继续走我们的文规向界」——两道门实装批因门轴在租约而延后，格式包族提前（时序重排非方向变化）；上游输入件即工作区根域外草稿 regula-design-draft-2026-09-06.md §二§三 与孵化登记件 sih-tools/incubation/regula-line-tools-2026-09-06.md（包 schema 契约）
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 文规线四件地基已齐：数学三件在册（ALG-013/014/015）、三工具契约登记（toolincub-solo）、绿基线（lib 170 加 0 加 6）、设计稿成文；两道门实装批候租约修复落地
- SDD 五件格式包（变更提案、规格差分、场景清单、技术方案、任务清单）是检查器（孵化登记件工作名）的食物与测试夹具：先包后机，检查器实装批可拿本批五包与样例先红后绿
- 本批纯数据零代码：包按已登记契约的包 schema 三面成文，检查器未建不影响包的成文与结构自洽校验

## 二、关键设计 {#design}

### 2.1 五包一族

落位 `sih-tools/incubation/packs/sdd-v1/`：manifest 一件加五包件。包形按孵化登记件契约草案；契约未定形处用 JSON 过 json-canonical-v1（理由申报入结果档）。迁移点声明：检查器正式立名建目录后一次迁移登记，本落位是围堰期家。

### 2.2 包 schema 三面（承登记件契约）

- 识别面：声明包认的文档语法（标题层级、节位、编号形）。场景清单包语法即 OpenSpec 骨架三刀形（pk-067 裁定落形）：R-／S- 编号全局唯一、Requirement 主体句 SHALL、Scenario 的 WHEN 加 THEN 加判据行，判据行三要素即检验程序、期望退出码、期望产物。规格差分包语法即 ADDED／MODIFIED／REMOVED 三分类加零增量申报位。任务清单包语法即每步引用 R-／S- 编号加证据位（退出码、产物哈希、链事件引用）。变更提案包语法即问题、意图、范畴排除、令源四节位。技术方案包语法即技术选择逐条回链 R- 编号。
- 判定面：逐条规则谓词，**全部机械可验**（单锚精神），每条带 D-2 三态失败定位声明（缺件、违规、断链）。语义层检查（如规格层禁写技术栈的层间纪律）不伪装成机械条款，显式入范围外申报。
- 出处面：包版本、适用文档类、D-4 基线向量挂锚位（检查器实装后回填实测向量，本批登记锚位）。

### 2.3 自反狗粮样例五件

fixtures 一套五件以**本批自身为变更对象**成文（新增格式包族即变更），承 SPEC-021 自反性狗粮条款先例。样例即检查器实装批的首组测试夹具。

### 2.4 自携校验脚本

`validate.py`（Python 标准库零依赖）按 manifest 对五包做结构自洽检查：节位在位、编号格式与唯一、fixtures 内引用可达（任务清单引用的 R-／S- 在场景清单在位、技术方案回链的 R- 在位）、判据行三要素在位。双跑逐字节一致。检查器实装后此脚本退役由真机接管，声明在包 manifest。

### 2.5 命题与机器门

一命题 gid m-sddpacks-pack-1，单锚即可验证性——判定面条款逐条只问能不能机械验。facet 三步加得一三步，stable_clear 即落据；boundary 或 violate 即停批如实呈报。

## 三、工作清单 {#work}

### Cluster 1：前置读数

- [ ] 三问双门（三锚引文程序切片）、叩问 elicit、正身 identity
- [ ] watch 对表（在盘遗留如实转述不代清：sih-math 未跟踪件、billwire 残件、engine 未跟踪结果档族、rootanchor 陈旧工地）
- [ ] 泊界心跳两线；例行读数 gauge record

### Cluster 2：成文施工

- [ ] 租约开工：--package sddpacks-solo，双仓工地（sih-engine base main、sih-tools base integral-stage-build），--allow 按请求写入节；零 sih-math 工地、零租约源码面
- [ ] 任务包与提示词从主树拷入工地 state/plan（批输入件先例，此后零主树写）
- [ ] manifest 加五包成文（§2.2 三面逐面落）
- [ ] 样例五件成文（§2.3 自反狗粮）
- [ ] validate.py 成文并跑绿（双跑一致，读数入批材料）

### Cluster 3：facet 与得一

- [ ] m-sddpacks-pack-1：emit-contract（零 LLM 零网络）→ 回填即席作答（谱系披露双声明）→ score 携正身件
- [ ] attractor check → verify（重放 identical）→ sign，stable_clear 终签入当日链

### Cluster 4：管线与结算

- [ ] 化格：JSON 件过 json-canonical-v1，Markdown 件过 general-v1
- [ ] 核阅 des-001：sih-tools 与 state/plan 与 event/plan 均域外 exit-2 如实记档
- [ ] 检词 core 零违例（历史死档词一律禁用）
- [ ] 温故 recall 底稿；结果档 sddpacks-solo-results.md 落 event/plan
- [ ] 双仓 settle（cert 取 ask3 记录认证哈希前八位）→ 放锁收约（撞主树同名未跟踪件按备份让位归并对表法）→ 链 verify → reconcile 双仓 → 心跳复算 → CALL-LOG 落笔

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 五包落位 | 工程 | sdd-v1 目录 manifest 加五包在位，schema 三面逐包齐 |
| **F-2** 样例过校验 | 工程 | 样例五件在位，validate.py 跑绿即引用全可达、编号全唯一、判据行三要素全在位，双跑逐字节一致 |
| **F-3** 终签在链 | 治理 | m-sddpacks-pack-1 stable_clear 终签入当日链，verify valid |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径，零 sih-math 写入、零租约源码写入 |
| **F-5** 委外不越权 | 编组治理 | 零代码改动、零人节点代行、零 plain commit、待裁项只注记、在泊件与在盘遗留零触碰 |

## 五、必读文件 {#read}

- 上游设计稿：工作区根 `regula-design-draft-2026-09-06.md` §二§三
- 包 schema 契约：`sih-tools/incubation/regula-line-tools-2026-09-06.md`
- 场景语法先例：设计稿 §3.1 场景清单代码块（OpenSpec 骨架三刀形）
- 自反先例：`sih-engine/doc/spec/SPEC-021-golden-vector-dual-method.md` 自反性狗粮条款
- 命令面：`sih-tools/BATCH-FACE.md`

## 六、约束 {#constraints}

1. 闸不过即停批，不硬闯不改判据不重采样凑收敛
2. 纯数据批零代码改动（validate.py 是批内校验脚本非引擎工具，落批目录不落工具位）
3. 判定面条款全部机械可验，语义检查显式申报范围外，禁伪装数学与伪装机械
4. 与登记件契约不合即停批上报，契约修订须申报不私改
5. 零租约源码写入（租约修复中，避开采修面）；零 sih-math 写入
6. 主树零直写（批输入件先例）；守卫禁 plain commit；退出码直读禁管道掩码
7. 在泊件与在盘遗留零触碰不并批
8. 中文零新造正式词，需新词停批走立名

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本包）
- [ ] 链 verify valid，reconcile 双仓相比批前零新增
- [ ] 结果档 sddpacks-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 契约草案与包成文之间的缝：契约未定形处以 JSON 加申报处置，缝大即停批上报
- 样例自反狗粮的引用闭包要真闭（本批任务包与提示词的编号引用须在样例场景清单内可达）
- 与在途租约修复批共享 sih-tools：撞锁即排队如实呈报，链活面归并按纯追加并集超集通道，真分叉停批

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`、`sih-engine/target/debug/attractor`、`sih-tools/facet/measure.py`、`sih-tools/lease`
- 跨仓引用：`sih-tools/incubation/`、`sih-tools/proposition/DES/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/sddpacks-solo.md` 与 `sddpacks-solo-prompt.md`（批输入件经工地落位）
- `sih-tools/incubation/packs/sdd-v1/`（manifest、五包、fixtures 五件、validate.py）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/sddpacks-solo-results.md` 与 `sddpacks-solo-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`、`sih-tools/facet/`、`sih-tools/proposition/DES/`
- worktrees 双仓 sddpacks-solo 工地
