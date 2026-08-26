# structmig-001-t6d：归位批

> T6D task-packages 治理任务
> 承接：2026-08-26 用户归位批开工令、DEC-001 修订二至五已签映射、意图记录 2026-08-26-ask3-structmig
> 范式：T6-D 范式——本任务包偏离：机械迁移类，主线串行，无子代理
> 日期：2026-08-26

## 一、问题陈述 {#problem}

- DEC-001 已签归位映射七件未执行即物理载体仍在旧位，路径引用与痕迹对账缺基准。
- 顶层 task-packages/ 混装活跃与签毕与待审三态，两态分流未落地。
- 活引用散布即根 AGENTS 与 skill 权威源与实体投影与 README 与 TEMPLATE 指旧路径。

## 二、关键设计 {#design}

### 2.1 七件迁移 {#migrate}

按 DEC-001 映射节逐件 git mv 即 doc/CASCADE.json 迁 sih/state/graph、state/tasks 迁 sih/state/plan、task-packages 两态分流、parking 迁 sih/state/parking、skills 迁 sih/state/skills、OPEN-QUESTIONS.md 迁 sih/state/open-questions.md、sih/static/audit 迁 sih/event/audit。迁移复制不搬改即文件内容零变化，唯活引用同步例外。

### 2.2 逐 stem 分类 {#classify}

判据即消费完成度，证据三类即已签文档、根 AGENTS 已裁条目、用户令本体。

签毕归 sih/event/plan 即 20 stem
: ask3-formalization 即 SPEC-005 立约在役、ask3-src 即根 AGENTS 引擎源码条目 2026-08-25、critical-path-migration 即根 AGENTS 工具线条目、dual-sign 即用户签署令本体、facet-audit 即 facet 范式在役、facet-migration-batch 即根 AGENTS facet 条目、fix-failures 即治理边界现状消费、fix-governance-boundaries 即根 AGENTS 治理边界节、gov004-commit-takeover 即 GOV-004 与 DEC-012 已签、lease-merge 即 lease 立名用户签署 2026-08-26、mechanism-scripts 即审计线发现被后续修复消费、meter-cutout 即根 AGENTS 2026-08-25 裁定、mid-priority-migration 即根 AGENTS 条目、parking-selector-time 即泊界心跳接线裁定、scrutinator-json-kinds 即 json kinds 在役、selector-cutout 即根 AGENTS 谓词路由条目、selector-round-level 即自检机械臂在役、structrev-001 即 DEC-001 修订二至五已签、task-packages-doc-format 即 README 与 TEMPLATE 在役、worktree-2locks 即 DEC-011 已签

旧包同迁
: sih/state/plan/task-package-event-stream-implementation.md 即 DEC-001 映射明文同迁 sih/event/plan

待审与活件归 sih/state/plan
: ab-audit-paradigm 即无结果档未完、corr012 即今日批待人结算、README 与 TEMPLATE 即活模板随行、state/tasks 十三件即 DEC-001 明文逻辑归 sih/state/plan

不动件
: task-packages/f-anchors-x11-t6d.md 即 facet 未跟踪件不迁不删

### 2.3 活引用同步 {#refs}

根 AGENTS 四处即心跳双目录参数、全态泊界材料路径、Skill 入口路径、redteam 路径。skill 权威源两处即 sihankor-t6d 与 sihankor-proposition-defense 探针。实体投影同步即 .agents/skills 对应件。README 与 TEMPLATE 内部路径。

## 三、工作清单 {#work}

### Cluster 1：迁移执行（主线写）

七件 git mv 按映射，签毕 51 文件入 sih/event/plan，待审活件入 sih/state/plan。

### Cluster 2：引用同步（主线写）

根 AGENTS 四处、skill 权威源与投影、README 与 TEMPLATE 内部路径。

### Cluster 3：管线与认证（主线串行验证）

化格核阅检词对编辑件逐件跑，域外如实记录，报告落盘逐件上链。

### Cluster 4：段结算提交（主线写）

机械 message 提交即迁移落链，会话收约。

## 四、判据 {#criteria}

- L1 七件迁移落位即新路径全存在旧路径仅余 f-anchors 与空壳
- L2 逐 stem 分类表与实际落位一致即签毕全在 event 待审全在 state
- L3 迁移文件内容零变化即 git 显示纯 rename 除活引用同步件
- L4 活引用零残留即根 AGENTS 与 skill 与投影无旧路径
- L5 管线绿认证上链链 valid
- L6 机械 message 动态取 cert 哈希
- L7 会话收约即本批不留在役锁

## 五、结果留档 {#results}

结果见 structmig-001-t6d-results.md，随批更新。

## 六、请求写入 {#requested-writes}

- sih-engine/task-packages/structmig-001-t6d.md
- sih-engine/task-packages/structmig-001-t6d-results.md
- sih-engine/task-packages 即全部迁移件
- sih-engine/state/
- sih-engine/sih/
- sih-engine/skills/
- sih-engine/parking/
- sih-engine/OPEN-QUESTIONS.md
- sih-engine/doc/CASCADE.json
- AGENTS.md
- sih-tools/scribe/
