# guardrail-solo：冲突处置机制硬化批（pk-045 样本库承接件）

> task-packages 治理任务
> 承接：用户 2026-09-03 同意护栏立项即 pk-045 首日与次轮样本库的实装转化，两轮链分叉两次撞证
> 队形：单线形 solo——确定性代码与主线亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-guardrail.json 双档零命中如实记

## 一、问题陈述 {#problem}

冲突测试两轮坐实三缺口。其一链分叉两次发生即工地链副本追加认证致主链同位双 valid 续链（entryunique 与 mathpipe-a3 各一次，后者手动合链收场）。其二收约两段式与空详情失败即 close 在共享追加文件两侧并发追加时报冲突或空错且可留半程合并态（ordwire 与主会话事故各一次，主会话事故即空详情后未查态即动手致链文件冲突标记）。其三守卫盲区即批名前缀无会话行提交落主线未被拦（mathpipe-a3 收约后 aea1768 一笔）。

## 二、关键设计 {#design}

1. 防分叉护栏：scribe 写入四入口即 append 与 intent 与 park 与 record，--trail 路径含 worktrees/ 段即拒退出码二载错文工地链副本禁追加即认证先落主链，显式覆写旗标留应急位默认关。单测四件加既有回归。SPEC-006 修订三。
2. 收约原子性：lease close 前置态检查即目标仓在合并态或共享面脏即整批拒绝零部分动作，错误详情全量透出禁空串；半程不可达即要么全归并要么零动作。lease CONTRACT 修订加单测。
3. 守卫补漏：commit-msg 形态判定增一拒类即批名前缀形（subject 以批名模式开头）而无 session 行者拒，wip 与 settle 与 merge 三合法形不变。hooks 加单测回放 aea1768 反例必拦。
4. 余件随批：主树活写未提交件即 ordwire 结果档第 5 条行与工具调用册更正行随批入库。

## 三、工作清单 {#work}

- [ ] 防分叉护栏实装与单测四件
- [ ] close 前置态检查与错误透出与单测
- [ ] 守卫补漏与反例回放
- [ ] 两行余件入库
- [ ] SPEC-006 修订三与 CONTRACT 修订走三步
- [ ] 认证上链多仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 防分叉** | 工程 | 工地路径 trail 四入口全拒退出码二，主树路径行为零变化，红绿在档 |
| **F-2 收约原子** | 工程 | 合并态与脏共享面两反例 close 整批拒且详情非空，中途零部分动作 |
| **F-3 守卫拦直交** | 工程 | aea1768 同形消息被拒退出码一，合法三形全放行 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/src/bin/scribe.rs 与 src/event_stream/
- 必读 2：sih-tools/lease/src/ 与 lease/hooks/
- 必读 3：sih-engine/sih/event/plan/ordwire-lease-solo-results.md 冲突样本节与 mathpipe-a3-solo-results.md 第 5 条
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 认证先落主树活链，链 settle 前一次性拷工地
2. 撞锁有限重试如实计数入冲突样本节，不绕行
3. 五子命令与 close 既有合法形行为零回退，主树 cargo test 与 lease 测试全绿
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-engine/src/bin/scribe.rs
- sih-engine/src/event_stream/
- sih-engine/doc/spec/SPEC-006-scribe-mergeback-gap.md
- sih-engine/sih/event/plan/ordwire-lease-solo-results.md（第 5 条行入库）
- sih-engine/sih/state/plan/guardrail-solo.md
- sih-engine/sih/event/plan/guardrail-solo-results.md
- sih-engine/sih/event/plan/guardrail-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/lease/
- sih-tools/lease/hooks/
- sih-tools/lease/CALL-LOG.md
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 三护栏红绿与反例回放在档
- [ ] 冲突样本节在结果档
- [ ] 认证入链多仓结算收约对表读数在档
