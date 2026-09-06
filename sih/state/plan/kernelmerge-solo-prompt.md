# kernelmerge-solo 委外提示词

> 用途：由用户转发给委外执行代理。任务包在 sih-engine/sih/state/plan/kernelmerge-solo.md，本提示词是执行入口。

---

你是司衡工作区的委外执行代理，单线 solo 队形、零子代理。工作区根：/Users/moc/workspaces/SiHankor。

## 开工三读

1. 读 AGENTS.md（工作区根）——治理宪法与工程基线
2. 读任务包 sih-engine/sih/state/plan/kernelmerge-solo.md——本批全部判据与红线
3. 读 sih-tools/BATCH-FACE.md——批机械链逐命令 verbatim 与坑位勘误

## 任务一句话

把核阅谓词引擎（src/scrutinator/rule.rs）与得一路由谓词机（src/attractor/route.rs）的求值内核归并为引擎内一个共享模块，两消费方改调共享件，**行为逐字节零漂移**。机器裁决已过（m-halfmerge-1 stable_clear 终签 2a7d0e0e 在链），你只做实装与验收。

## 批机械链全序（照 BATCH-FACE 逐命令跑，不跳步）

三问双门 → 叩问 → 正身 → 租约开工（--package kernelmerge-solo，--allow 按任务包请求写入节）→ 书简意图 → 工地施工 → 化格核阅检词 → 认证 → 双仓 settle → 放锁收约 → 书单对表 → 对账对表。

## 四条命门（违反任何一条即批失败）

1. **零漂移是唯一硬判据**：统一前后，核阅 golden 12 件金向量与 route golden 8 场景全量重放逐字节一致；同参双跑两机 CLI 输出 cmp IDENTICAL 且退出码一致。有漂移即停批如实呈报，不迁就不硬凑。
2. **命令面冻结**：子命令、旗标、退出码语义零变化。统一的是求值内核，不是谓词域——文档谓词与路由谓词各自保留，发现语义重叠以包装适配不以内核吞并。
3. **退出码直读**：一切工具调用退出码直接读，禁管道掩码（genpark-solo 勘误在案：管道尾退出码掩盖真实退出码一）。
4. **pk-060 出泊唯在验收全过后**：pk-060-exit.json 写入与 scribe park exit 仅在 F-1 至 F-4 全过后执行，exit 载裁决指向即用户 2026-09-06 委外令＋m-halfmerge-1 终签 2a7d0e0e。

## 工程纪律

- 先红后绿：共享模块新单元测试先证红再证绿，留痕结果档；既有测试零回归
- 引擎件合并主树后先 cargo build 重编再作任何调用（旧二进制按旧码作答，BATCH-FACE 勘误）
- 主树零直写，待提交件经工地 settle 通道；守卫在位禁 plain git commit
- 在盘遗留无主件（watchcheck 清单）不豁免不代清，不新增无主写
- 收约后：链 verify、reconcile 双仓（退出码直读）、泊界心跳、结果档 kernelmerge-solo-results.md 落 event/plan、CALL-LOG 落笔、向用户完工报告

## 停批条件

零漂移判据失败且无法以包装适配修复；或 cargo test 出现非本批引入的既有红；或撞锁排队超时。停批即如实呈报现状，不硬闯不绕行。
