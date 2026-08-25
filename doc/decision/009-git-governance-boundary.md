# DEC-009 git commit 治理边界决策

本决策裁定 git commit 治理边界的终裁时点与当前阶段原则，闭原 OQ-05 与 pk-006。承用户 2026-08-25 裁定「git 随 worktree 加 2locks 启动时一起定」与意图流 round 12 事件 e8a9c379。

## 概览 {#overview}

- 终裁时点即 worktree 与 2locks 组件启动时一并定案，本决策只定时点与阶段原则不预判终裁内容::[决策集](#decisions)
- 当前阶段原则为内在自限即选择性暂存纪律与提交信息承载治理内容，外加锁不预实装::[决策集](#decisions)
- lease 即文件租约归属随终裁一并定，AGENTS 适用域声明的交叉引用随本案闭合::[决策后果](#consequences)

## 决策集 {#decisions}

### 终裁时点 {#final-ruling-point}

git commit 治理边界的终裁随 worktree 与 2locks 组件启动时一并定。理由：commit 资格控制的治理性条款依赖锁件的实际形态，预判即重复裁定；2locks 经向界签署的撞词处置甲款独占锁字，组件名位有据。终裁对象含 SETSP 既有 pre-commit 设计的取舍，该设计留作终裁候选件不在本决策评价。

### 当前阶段原则 {#current-stage-principle}

当前手动阶段的 git 治理以内加自限为准：选择性暂存纪律即操作者只提交自己的路径并如实披露共享载体上的他方内容；提交信息承载治理内容即批次、事件哈希与偏离说明随提交入档；真实性权威在 trail 链即提交是快照不是真相。本原则是对既有实践的追认，不是新设义务。外加锁机制不预实装。

### lease 归属 {#lease-attribution}

TRAE 时代的文件租约 Tool 即 acquire_lease 与 release_lease 未承接，与锁同族，其归属与形态随终裁一并定，AGENTS 适用域声明中归 pk-006 的指针改归本决策。

## 决策后果 {#consequences}

- pk-006 出泊转正，泊界清零，段结算闸门开启
- 终裁触发条件登记即 worktree 或 2locks 任一立项时，git 治理边界随其 DEC 一并载入
- 选择性暂存纪律升格为登记原则，违反即偏离须披露

## 备选方案 {#alternatives}

现在终裁：无组件实态即无判据，判了也是纸面锁，弃。废弃不裁：张力真实存在且 lease 挂账，弃。

## 关系图谱 {#graph}

本决策上承原 OQ-05 推导与 SETSP 沉默参考，侧承向界签署的锁字处置与 AGENTS 适用域声明，下接 pk-006 出泊与 worktree 与 2locks 组件的未来 DEC。

## 认识论立场 {#epistemic-stance}

本决策是时点裁定不是内容裁定。约束内在化是哲学立场，外加锁是工程手段，两者的取舍须在手段的实际形态在场时裁，现在能定的只有时点与追认。
