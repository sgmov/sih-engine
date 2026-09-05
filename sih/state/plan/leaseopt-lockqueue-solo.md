# leaseopt-lockqueue-solo：锁排队与僵尸接管（批五，租约线收官）

> 批：leaseopt-lockqueue-solo（leaseopt 线批五：排队唤醒、僵尸接管命令、心跳阈值清账）
> 令源：用户 2026-09-05「开始」；租约模式定稿对话（2026-09-05）与批四遗留（lock_queue 建表未调度、接管命令批五承载、HEARTBEAT_STALE_SECONDS 冻结清账）
> 范式：T6-D 单线 solo，主会亲写，零子代理
> 批界：视图面板归批六；链 ndjson 零触碰

## 一、问题陈述 {#problem}

撞锁空转是用户一级痛点（批一账本：重试连击 97 段、空转 137 条、每失败条目摊 6.03 次 meter 调用）；批四锁库在役后撞锁即拒仍无排队，agent 重试仍烧 token；僵尸检验文件与僵尸锁的显式接管命令缺位（批四只报不接）。

## 二、关键设计 {#design}

- **排队**：`lock` 增 `--wait` 旗标（缺省 off 兼容）：撞锁时写 lock_queue 行返回 {queued:true, position:N}；队列存在时无 wait 的 lock 撞队即拒 queued_not_your_turn（保序公平：后来者不再反复撞墙）；acquire 成功即清除自己在该路径的队列行；`status` 增队列视图。
- **到位语义**：批 agent 无常驻进程，唤醒=入队凭据+保序+下次尝试即达（队首且锁空时 lock 必成）；重试循环被「一次入队+按位递进」替代，token 空转消灭。
- **takeover**：新子命令 `lease takeover --package --reason`：检验文件新鲜即拒（接管即同跑禁）；停滞即清活件+清该包僵尸锁行（lock_event 记 takeover_released）+输出可重开，人节点显式命令，动作由调用方上链。
- **心跳阈值清账**：HEARTBEAT_STALE_SECONDS=300 维持冻结（无自然常数，超时选值为工程裁量），推导档申报维持冻结经用户后裁或后批载体，非裸奔。

## 三、可证伪条件 {#falsifiable}

| F | 判据 |
|---|---|
| F-1 | --wait 撞锁入队 position 正确；队列存在时非队首 lock 拒 queued_not_your_turn |
| F-2 | 锁放后队首 lock 即成并出队；acquire 成功清自己队列行 |
| F-3 | takeover：新鲜文件拒；停滞文件清件清锁输出可重开 |
| F-4 | 既有全族（100 件）零回归 |
| F-5 | 得一裁九发 + 执契；阈值维持冻结经用户裁呈报 |

## 四、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/
- sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/pyproject.toml
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/ledger/
- sih-engine/sih/state/plan/
- sih-engine/sih/event/plan/leaseopt-lockqueue-solo-results.md
- sih-tools/facet/contracts/leaseopt-lockqueue-260905/
- sih-tools/proposition/DES/leaseopt-lockqueue-solo/
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/tally/reports/
- sih-tools/meter/counts/
- sih-engine/sih/event/trail/
- sih-math/docs/leaseopt-lockqueue-derivation-2026-09-05.md
