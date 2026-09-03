# mathrefmt2-solo 结果档

> 批：mathrefmt2-solo（数学仓重构格式归一波二：calculus 余 51 条概览归一）
> 承接：mathrefmt-solo 波一（62 条 LIM/DIFF/INT 已归一）余件；release-audit-2026-09-02 遗产披露第二条
> 队形：单线形 solo——确定性脚本加逐条改写亲写零子代理
> 开工实日：2026-09-03
> 会话号：12c35fa47a54b93f
> 冲突模式：与 mathquote-calc-solo / goldlim-refreeze-solo 故意并发，撞锁有限重试上限十次逐次计数

## 冲突样本节

| # | 目标锁 | 持锁会话 | 持锁批 | 动作 |
|---|---|---|---|---|
| 1 | sih-math/calculus/.../entries/ 等共享写面 | mathquote-calc-solo | mathquote-calc-solo | 全线撞锁，十次内让位，待其归并后以当刻盘面重跑枚举再施工（工作树含 mathquote-calc 副本归并） |
| 2 | sih-engine/sih/event/trail + 金向量 fixture | goldlim-refreeze-solo | goldlim-refreeze-solo | trail 与 lim001 金向量 fixture 被其占据，等其释放（06:38 释放）后取回 trail 锁 |

并发批处理：mathquote-calc-solo 归并后枚举/施工以含其引文补写的当刻盘面重跑（F-1 对表六前缀计数仍 51）；goldlim-refreeze-solo 放锁未完成 lim001 金向量重冻即归来，见金向量处置节。

## 枚举计数与本波件数

- 全仓盘上 calculus entries 计数：114 条
- 概览首节：51（与 INDEX 对表）| 定义首节：63 | 其他：0
- 本波件数：51 条，前缀 APP=10 / HIS=16 / MUL=10 / NS=3 / SER=5 / SPEC=7
- 枚举清单：materials/2026-09-03-overview-enum.txt（51 条全列，F-1 对表）；变换记录：materials/transform-records.json（51 条，F-3）

## 段落去向申报表

变换承波一逐字同款：概览导航块整体移除（六行均为自指锚点无正文）→ 原「## 话题」H2 改「## 定义 {#definition}」→ 正文零改动。

已有 51 条 transform-records.json 逐条记录：overview_idx/topic_idx/removed_lines 全为目录锚点行、body_lines_after 与删行前正文逐字节一致、sha_std/sha_new 前后哈希在档。已验证信息零丢失（F-3）。

## 三步管线读数

逐条三步（对工地文件，序固定 化格→核阅→检词），51 条：

| 步骤 | 工具 | 规则包 | 批次 | 读数 |
|---|---|---|---|---|
| 化格 | formatter 0.2.0 | general-v1 | 51 条 | 51 条 0 改动（格式已合规），exit 0 |
| 核阅 | scrutinator 0.1.0 | des-001-mathe 0.3.0 | 51 条 | 51 条 0 findings 0 domain_mismatches，exit 0（S005 域内定义首节通过） |
| 检词 | nomenclator 0.2.0 | core 0.9.0 | 51 条 | 51 条 domain 外 skipped（exclude **/llm-friendly-build/**），exit 0 零违例 |

块序归一核验（F-2）：改后 51 条首二级节 `## 定义 {#definition}`，S005 域内零违规。

## 金向量联动与 F-4 处置（用户裁定）

金向量实体在 `sih-engine/src/scrutinator/fixtures/golden/des-001-mathe-{mul001,lim001}.json`（tests.rs `golden_des001mathe_*` 经 `golden_path()` 读取作期望）。本批含 MUL 前缀变换引发 mul001 金向量内容哈希漂移；lim001 属波一遗留（goldlim-refreeze-solo 放锁未完成）。

实测 `cargo test --lib golden_des001mathe` 双测 FAILED（mul001 与 lim001 哈希不一致）。

仅批请求写入节不覆盖 fixtures/golden/ 目录，F-5 机械禁止越范围写；扩范围需 close+重立约（wip-commit+释锁+三仓归并+重派生 ask3/id，半破坏性共享面操作）。

用户裁定 2026-09-03：**两个金向量都转遗留**，本批不写金向量目录。

**F-4 判定：本批不满足**（cargo test 无法全绿）。mul001 与 lim001 金向量期望漂移申报为遗留，需后续经允许范围含 fixtures/golden/ 的专用批随批重冻。本批如实申报，禁止漏报纪律满足。

## 认证清单

三步管线报告逐件 append 主树活链（会话 12c35fa47a54b93f）：

| # | 认证对象 | 报告路径 | event_id | exit |
|---|---|---|---|---|
| 1 | 书简意图 intent | ask3 record/validation | c8e95532-43aa-4fb7-850d-3d4a09fb299f | 0 |
| 2 | 核阅 scr/des-001-mathe | 2026-09-03-mathrefmt2-solo-scr-des001-mathe.json | 37f68373-a55d-489c-906d-c1d69ff59e6f | 0 |
| 3 | 化格 fmt/general-v1 | 2026-09-03-mathrefmt2-solo-fmt-general.json | dea1e8e8-43e5-4d6c-b483-97cfd016498b | 0 |
| 4 | 检词 nomen/core | 2026-09-03-mathrefmt2-solo-nomen-core.json | 44a1b040-3373-4aff-9604-db4bfe3480e9 | 0 |

## 三仓 commit 号

- sih-math：56e90c8（51 条 entry 概览归一，settle 三检过）
- sih-engine：本档及其余批文档随 settle 提交
- sih-tools：管线报告与 CALL-LOG 随 settle 提交

## 链 verify / reconcile / F 表 / 越线与误差申报

### 链 verify

`target/debug/scribe verify --trail sih/event/trail/2026-09-03.ndjson` → status `valid`，events=127，
last_hash `fc186170…`，链完整首尾衔接，exit 0。

### F 表

| F | 判据 | 结果 |
|---|---|---|
| F-1 枚举零漏 | 51 条六前缀与 INDEX 对表 | 通过（overview-enum.txt 114 总/51 概览首节六前缀全对） |
| F-2 块序归一 | 51 条首二级节全 `## 定义 {#definition}`，S005 通过 | 通过（des-001-mathe 51 条 0 findings） |
| F-3 信息零丢失 | 删行全锚点、正文逐字节一致 | 通过（transform-records.json 51 条前后哈希在档） |
| F-4 金向量零漂移残留 | cargo test --lib 全绿 | **不满足**：mul001+lim001 转遗留（用户裁定），本批不冻申报 |
| F-5 写入仅 allow | 写入仅请求写入节所列 | 通过（51 条 entry 与批文档在 allow，金向量目录零写） |

### 越线与误差申报

金向量重冻因请求写入节未含 fixtures/golden/ 目录而机械受阻，经用户裁定转遗留（F-4 不满足，如实申报）。计划请求写入节曾临时加注金向量目录后按用户裁定撤回，范围保持原 allow 不变。

## 收约处置（收束，用户裁定「严格 worktree 手工解冲突」）

三仓共享活链/计数器 ndjson（追加型）被 settle 复制入批分支，`git merge-tree` 实证三方归并必然冲突：sih-engine `trail/2026-09-03.ndjson` content 冲突，sih-tools `meter/counts/2026-09-03.ndjson` add/add 冲突（工作树我已验证与批分支逐字节一致，非数据分叉）。

用户 2026-09-03 裁定走严格 worktree 手工解冲突归并：

| 仓 | 冲突文件 | 解冲突取侧 | 归并结果 |
|---|---|---|---|
| sih-engine | sih/event/trail/2026-09-03.ndjson | 分支侧 127（含批 4 认证事件，链末 fc186170） | merge commit `a885f72` 回 main；主树 untracked 成果副本先删后由分支落盘（identical 零净损失） |
| sih-tools | meter/counts/2026-09-03.ndjson | 工作区 56 行超集（分支 52 全含 + 批/并发调 4 行，零丢） | merge commit `91776929` 回 integral-stage-build；本批 7 报告与 meter 由分支落盘 |

两处 merge 提交为直提守卫拒绝，经 `lease bypass --repo --sha --reason` 登记留痕（bypass.ndjson 两条：a885f72、91776929）。

`lease close --force` 收约：三仓全部归并删除分支与 worktree（sih-tools worktree 残留 3 项锁台账/身份报告副本为分支副本非权威，强拆丢弃），session `12c35fa47a54b93f` revoked=true。

对账：`lease reconcile` 三仓 exit 0，sih-engine 与 sih-tools `unbypassed=0`，两 merge commit 均归 bypassed，无未路由提交。

链复核：`scribe verify --trail sih/event/trail/2026-09-03.ndjson` → status `valid`，events=127，last_hash `fc186170…`，exit 0（归并后活链完整首尾衔接）。

### 三仓 commit 号（收束后）

- sih-math：56e90c8（51 条 entry 概览归一，settle 三检过）
- sih-engine：a885f72（本批成果与活链副本归并入 main，trail 冲突取分支侧 127）
- sih-tools：91776929（本批管线报告与 meter 归并入 integral-stage-build）