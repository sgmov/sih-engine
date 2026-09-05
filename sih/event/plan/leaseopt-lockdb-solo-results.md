# leaseopt-lockdb-solo 结果档

> 批：leaseopt-lockdb-solo（leaseopt 线批四，SQLite 锁库与开约钥匙与心跳）
> 会话：e75ca21380a19042（ask3 侧 sess-zcode-260905-lockdb）
> 日期：2026-09-05。队形：单线形 solo，主会亲写，零子代理。
> 令源：用户 2026-09-05「开」；租约模式定稿对话（2026-09-05）。

## 一句话结论

锁面重构完成：SQLite WAL 锁库为判定正典（单事务取放锁，插队机制性不可能），ndjson 判定退役镜像保留（引擎 lockgate 与既有读者照旧可见，审计账入版控照旧），正身检验钥匙使同机多窗口同跑开工即拒，心跳子命令与停滞读数就位，收约凭据落任务包同级；100 件测试全绿（94 旧零回归 + 6 新），CONTRACT 1.20.0 修订三十二；得一裁 near_threshold 挂起待人复核，批保持开位主树零归并。

## 一、F 表

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 并发原子性 | 同路径多轮取锁恰一胜 | 过 | test_lockdb_concurrent_atomic：8 轮 1 胜 7 拒，现势恰一持位 |
| F-2 同跑拒开 | 钥匙闸同跑即退 | 过 | test_lockdb_key_gate_same_run_and_stale：新鲜心跳二开 reason=same_package_active_window 载在营窗口会话号 |
| F-3 停滞报僵尸 | 超阈不自动接管 | 过 | 同测试：心跳回拨 600s 后 reason=stale_check_file 提示人节点接管 |
| F-4 close 凭据 | 落任务包同级清活件 | 过 | test_lockdb_heartbeat_command_and_close_receipt：plan/demo-pkg.lease-check.json 在，checks 活件清 |
| F-5 回归与迁移 | 全族零回归迁移幂等 | 过 | 100 passed（94 旧四件适配 + 6 新）；迁移幂等双跑 holds 同判；heartbeat 双处刷新 |
| F-6 常数与终签 | 阈值冻结登记；得一裁 | 挂起 | HEARTBEAT_STALE_SECONDS=300 冻结登记 CONTRACT 修订三十二；m-lockdb-switch-1 near_threshold（九发全 comply 稳、边界旗零、依据族基线四×7 基线一×2），sign 未执行待人复核 |

## 二、实装要点

- lockdb.py：lock_state（复合主键 path+session_id 承 append 多持位）+ lock_event（只 INSERT）+ lock_queue（建表批五调度）；BEGIN IMMEDIATE 事务判定；首启自 ndjson 迁移幂等。
- lockcore.acquire/release：判定正典换 lockdb，验五验保留，ndjson 镜像同步追加（镜像只作可见性不作判定，落后窗口如实声明）。
- cli：open 前置钥匙闸（原子创建/同跑拒/停滞报僵尸）、heartbeat 子命令（双处刷新）、close 收约凭据（转写 plan/<包>.lease-check.json 清活件）。
- .gitignore：locks.db 与 checks/ 不入版控（审计账由镜像承载）。

## 三、得一裁挂起详情

gid m-lockdb-switch-1 九发：decision_stable T、boundary_low T、basis_consensus F（基线四×7、基线一×2）、boundary_rate 0.0。依据族双源为修复真实面（可校验性与可重复程序）如实作答无挑选。与批二 m-fixguard-switch-1 同形态同成因——该件已经你 2026-09-05「同意」落链放行（confirmation 7495c5c9 在链），本件同形态待你复核。材料指针：facet/contracts/leaseopt-lockdb-260905/m-lockdb-switch-1/。

## 四、缺陷与越线申报

1. 实装中发现并修复 lock_state 初版单列主键无法承 append 多持位的真 bug（复合主键修正），红在既有测试 test_append_release_own_hold 先红后绿。
2. 既有测试四件适配如实申报：双开拒绝断言并集 reason 集（钥匙闸先行于 PackageSessionActive）、零写表面 unlink 预算 1→2（checks 活件清除新面）与 open 模式 "x" 豁免（原子创建）、gate1 详情按 reason 分支断言。
3. recall 先导检索零命中如实记；checkcite pass 在档；HEARTBEAT_STALE_SECONDS 冻结登记清账路径批五。

## 五、待复核后收口路径

人节点确认后：confirmation 件落链 → 执契 check→verify（sign 因 near_threshold 机制不可用，同批二先例如实）→ 三仓 settle → close 归并 → reconcile → 链 verify → 主树复验 → 本档收口附记。人裁退回则命题改写链重立。
