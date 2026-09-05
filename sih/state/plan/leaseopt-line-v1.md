# leaseopt 线任务包 v1：租约与多 agent 协调优化（委外线）

> 线：leaseopt（租约优化线，主线 v2 排序第 1 位，用户 2026-09-04 令源直入）
> 令源：用户 2026-09-04「租约的优化也要加入，多agents协调一直撞车」+「锁竞态导致多agents冲突，空转消耗token对我来说比较重要」
> 委外形：本线各批由委外 agent 按本包逐批领取，每批独立立约独立收约，形后缀 solo
> 先导检索：温故 recall --topic 租约冲突多agent协调 零命中如实记（检索面未覆盖，证据底座为台账与样本库与 2026-09-04 晚主会一手查证）

## 一、问题陈述 {#problem}

多 agent 并行批持续撞车：收约失败台账累计 176 笔（sih-tools/lease/ledger/sessions.ndjson close_failed 事件）；pk-045 冲突样本库在泊二十余类；2026-09-04 晚 mainord-solo 批单批三连撞。用户点名两级痛点：锁竞态致批间冲突、撞锁空转重试消耗 token。

## 二、病灶六项（证据底座，委外 agent 逐件核验后开工） {#evidence}

1. **收约守卫假阳性（当晚根因，最高优先证据）**：`lease/src/lease/core.py` `detect_merge_conflicts` 以「盘上内容 ≠ 分支内容」判脏位，不核真实 git 脏位（对 HEAD）；批期内基线前进过的文件（主树干净）被误判脏位 → `is_pure_append_conflict` 判 diverged → 整批拒。2026-09-02 修订「严义不看 base」注释在档。凡并行批基线互进即互卡。
2. **无租约活写零实时告警**：主树直写绕工地只在收约时被动撞见；2026-09-04 晚 closeguard 会话已撤销而进程活写主树 core.py 至 22:54（mainord-solo 收约三撞之源）。
3. **僵尸会话**：会话撤销 ≠ 进程死亡，无回收程序；ledger 有 session_orphan 类在档。
4. **锁竞态与空转 token（用户一级痛点）**：现行补丁纪律为 exclusive 撞锁有限重试上限十次逐次计数、共享追加面 --mode append 短持即取即放、并行批两批上限（用户纪律）——补丁存在本身即病灶证据：每次撞锁重试都是 LLM 轮次即 token 消耗，无排队无挂起唤醒无到位通知。
5. **无协调视图**：谁持何锁、在途批施工面交集、冲突预测，无可视面板；人类异常介入无视图入口。
6. **空转无度量**：meter counts 有逐日调用计数但未与撞车事件交叉，每类撞车的 token 代价无数可依。

## 三、批次拆解（每批单主题承 SPEC-016；批序在批一账本产出后定稿，默认序如下） {#batches}

**批一 leaseopt-audit（盘点批，纯脚本零行为变更，必先）**
- 输入：close_failed 台账 176 笔全量、pk-045 样本库、locks/sessions/bypass 台账、meter counts、2026-09-04 晚三撞时间轴。
- 产出：撞车分类账本——每类频次、处置现状（已硬化 / 裸奔）、平均 meter 调用代价即 token 代理量、修复批次归属映射。零人工填项可复算。
- 验收：复算逐字节一致；账本与台账 diff 对表零漏项；token 代价列有数。

**批二 leaseopt-fixguard（守卫假阳性修复）**
- 修法方向：脏位判定改用真实 git 脏位（status 对 HEAD），干净文件不入冲突面；三分合并自然处理基线前进件。
- 纪律：TDD 先红后绿（2026-09-04 三撞做夹具：基线前进 + 主树净态 + 收约应过）；金向量增「基线前进净态收约」场景；判定语义变更过得一裁；净态收约语义零变化回归。

**批三 leaseopt-precheck（批前冲突预检）——已实施收官：用户 2026-09-05 裁定「直接收了批3吧未来怕忘」独立开批，open 预检闸在役（CONTRACT 1.22.0 修订三十四），详见 leaseopt-precheck-solo-results.md**
- lease open 增 allow 面与活跃会话 allow 面交集检查：交集非空即拒开或转排队，把冲突拦在开工前。
- 纪律：新判定语义过得一裁；拒开退出码与回显事由入契约。

**批四 leaseopt-lockqueue（锁排队与挂起唤醒，用户一级痛点）**
- 方向：锁位排队（位次 + 到位一次性通知），agent 挂起等通知替代轮询重试；确定性退避序列替代随机抖动。
- 验收硬指标：撞锁场景 meter 调用数前后对比下降（批一账本为基线）；唤醒后零丢锁零死锁（金向量）。

**批五 leaseopt-zombie（僵尸会话回收）**
- 会话-进程绑定（PID 登记加心跳）、孤儿扫描、回收程序；撤销后进程残留可检出可清场。

**批六 leaseopt-watch（写时告警与协调视图）**
- 无租约主树写实时告警（监听面或守卫扩展）；viewer 增锁位、在途批施工面交集、冲突预测面板。承工程基线第三条人类只在视图告警时介入。

**收尾批**：pk-045 逐类销账出泊，出泊唯人节点。

## 四、跨批红线 {#redlines}

- 判定语义任何变更过得一裁，机械链全绿才收口
- 主树零直写，一切经工地；close 外提交 --no-verify 加 lease bypass 登记
- 净态收约语义零回归（既有 lease 测试全绿是每批门槛）
- 新增判定性常数零裸奔（载体或冻结登记）
- 并行纪律照现行（两批上限、重试上限十次、append 短持）直至批四在役替代
- 用户优先级：批四（token 空转）与批二（收约互卡）痛点最高；批一账本数据可重排批二至批六次序，重排呈人节点

## 五、必读文件 {#read}

- sih-tools/lease/CONTRACT.md（含 2026-09-04 修订二十九）
- sih-tools/lease/src/lease/core.py（detect_merge_conflicts 与 is_pure_append_conflict 与 close_session）
- sih-engine/doc/spec/SPEC-020-close-merge-harden-v1.md
- sih-engine/doc/governance/PARKING-v1.md（pk-045 条目）
- sih-tools/BATCH-FACE.md（机械链全序与坑位勘误节）
- sih-engine/sih/event/plan/mainord-solo-results.md（2026-09-04 晚三撞实录）

## 六、验收（线级） {#acceptance}

- 撞车分类账本在档且可复算；每类处置状态二选一（已硬化在役 / 冻结登记在泊）
- 收约守卫假阳性夹具绿；净态收约回归全绿
- 批前预检在役；锁排队在役且 meter 对比有降
- pk-045 出泊（唯人节点裁定）
- 全部新增判定性常数载体在册或冻结登记

## 七、请求写入（批一） {#requested-writes}

- sih-tools/proposition/DES/leaseopt-audit/
- sih-tools/lease/CONTRACT.md（版本行随批）
- sih-engine/sih/event/plan/leaseopt-audit-solo-results.md
