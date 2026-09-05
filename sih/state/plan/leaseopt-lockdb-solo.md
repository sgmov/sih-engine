# leaseopt-lockdb-solo：SQLite 锁库与开约钥匙与心跳（批四）

> 批：leaseopt-lockdb-solo（leaseopt 线批四，锁面重构：锁表 SQLite 事务化、open 检验钥匙、心跳）
> 令源：用户 2026-09-05「开」（承 2026-09-05 租约模式定稿对话：先立约后起工地、正身检验钥匙、SQLite 锁表事务化、心跳停滞告警、收约凭据落位）
> 范式：T6-D 单线 solo，主会亲写，零子代理
> 批界：排队唤醒与僵尸接管归批五（lockqueue），本批只建队列表不做调度；链 ndjson 零触碰（用户裁定链不入本线）

## 一、问题陈述 {#problem}

锁竞态与空转 token 是用户一级痛点（批一账本：重试连击 97 段、空转 137 条、撞车日调用密度翻倍）；同机同框架多窗口正身一致，无法凭正身区分同跑（gov002v2 批 R5 实证）；锁台账为 ndjson 追加面，收约归并时与链同撞。设计定稿（2026-09-05 对话）：锁状态入 SQLite 现势表（单事务上锁防插队），租约历史入 append-only 事件表（只 INSERT 不 UPDATE）；open 前生成正身检验文件为锁表写入钥匙（原子创建，同名冲突即同跑即退，最少占管线）；检验文件兼心跳载体（固定程序刷新活跃时间戳，文件名不改），停滞即僵尸信号归视图告警人节点；收约时检验文件随合并落任务包同级为跑完凭据。

## 二、关键设计 {#design}

- **lockdb.py**（新模块）：库 ledger/locks.db，WAL 模式。表 lock_state 现势态（path 主键、session、mode、acquired_at、heartbeat_at）；表 lock_event 只 INSERT 历史；表 lock_queue 本批建表批五调度。acquire/release 单事务（BEGIN IMMEDIATE），冲突判定含路径祖先包含与 exclusive/append 互斥（承 1.14.0 语义）。
- **迁移**：首启 locks.db 不存在且 locks.ndjson 存在即导入（活跃行入 lock_state，全史入 lock_event）；locks.ndjson 冻结只读不再写；lockcore.active_locks 改读 SQLite，API 签名不变。
- **检验钥匙**：ledger/checks/<包名>.json（gitignore），open 时 O_EXCL 原子创建，内容含 session、正身核哈希、opened_at、heartbeat_at。同名且心跳新鲜（停滞阈值内）即同跑拒开退出；同名且停滞超阈即报疑似僵尸遗留，提示人节点显式接管（接管命令批五承载，本批不自动接管）。
- **心跳**：新子命令 `lease heartbeat --package --session`，刷新检验文件与 lock_state.heartbeat_at；固定程序定期调用。
- **收约凭据**：close 成功时检验文件转写 sih/state/plan/<包名>.lease-check.json（随批入版控为跑完证据），删除 ledger/checks 下活件。
- **停滞阈值** HEARTBEAT_STALE_SECONDS=300：判定性常数（改它改僵尸判定），本批冻结登记（清账路径指向批五载体批），不裸奔。

## 三、工作清单 {#work}

- [ ] lockdb.py 实装 + 迁移 + lockcore 兼容读
- [ ] open 检验钥匙 + heartbeat 子命令 + close 凭据落位
- [ ] TDD：并发原子性（多进程同路径单胜）、同跑拒开、停滞报僵尸、close 凭据、既有全族零回归
- [ ] CONTRACT 1.20.0 修订三十二 + CALL-LOG + .gitignore
- [ ] 得一裁九发测量（锁后端切换判定语义）+ 执契终签
- [ ] 推导档（锁库原子性载体：ORD-020 全序资源分配与死锁自由既有挂点扩展 + 停滞阈值冻结登记）

## 四、可证伪条件 {#falsifiable}

| F | 判据 |
|---|---|
| F-1 | 并发 acquire 同路径多进程恰一胜，零双持（金向量重放含 locked_elsewhere 场景） |
| F-2 | 同跑：第二 open 撞新鲜检验文件即拒退，第一会话无感 |
| F-3 | 停滞：心跳超阈后 open 报僵尸遗留提示，不自动接管 |
| F-4 | close 凭据：state/plan/<包>.lease-check.json 落位且 ledger/checks 活件清除 |
| F-5 | 既有 lease 全族测试零回归；locks.ndjson 迁移后只读 |
| F-6 | 得一裁 + 执契；阈值常数冻结登记在档 |

## 五、必读 {#read}

- sih-tools/lease/src/lease/lockcore.py（active_locks/acquire/release/normalize_path）
- sih-tools/lease/CONTRACT.md 载体引用节（ORD-020 挂点）
- sih-engine/sih/state/plan/leaseopt-line-v1.md 批四节
- 2026-09-05 租约模式定稿对话（先立约后工地、检验钥匙、心跳、收约凭据设计）

## 六、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/
- sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/pyproject.toml
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/ledger/
- sih-tools/.gitignore
- sih-engine/sih/state/plan/
- sih-engine/sih/event/plan/leaseopt-lockdb-solo-results.md
- sih-tools/facet/contracts/leaseopt-lockdb-260905/
- sih-tools/proposition/DES/leaseopt-lockdb-solo/
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/tally/reports/
- sih-tools/meter/counts/
- sih-engine/sih/event/trail/
- sih-math/docs/leaseopt-lockdb-derivation-2026-09-05.md
