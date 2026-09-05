# leaseopt-lockqueue-solo 结果档

> 批：leaseopt-lockqueue-solo（leaseopt 线批五收官：锁排队与僵尸接管）
> 会话：5ca45b06a30f09e1（ask3 侧 sess-zcode-260905-lockqueue）
> 日期：2026-09-05。队形：单线形 solo，主会亲写，零子代理。
> 令源：用户 2026-09-05「开始」。

## 一句话结论

排队调度与接管机制完成：lock --wait 撞锁入队返位次、非队首保序拒、成功出队到位即达（FIFO 良基无饥饿，载体 ORD-011）；takeover 人节点显式接管两态（新鲜拒、停滞清件清锁逐笔只 INSERT 留痕）；102 件测试全绿（100 旧零回归 + 2 新），CONTRACT 1.21.0 修订三十三；得一裁改写链两跳（switch-1 boundary 依据族三值如实入档 → switch-2 收窄重立 near_threshold），终签停闸待人复核，批保持开位主树零归并。

## 一、F 表

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 排队入队保序 | --wait 入队返位次；非队首拒 | 过 | test_lockqueue_wait_enqueue_and_turn：position=1；三会话非队首拒 queued_not_your_turn |
| F-2 到位出队 | 锁放后队首即成并出队 | 过 | 同测试：s1 放锁后 s2 lock 即成 |
| F-3 接管两态 | 新鲜拒；停滞清件清锁 | 过 | test_lockdb_takeover_stale_only：window_active 拒后停滞 takeover 清活件清 doc/z.md 锁行 |
| F-4 回归 | 全族零回归 | 过 | 102 passed（100 旧 + 2 新；unlink 预算 2→3 与 reason 断言两处适配申报） |
| F-6 wait-turn 阻塞形态 | 拿号后阻塞等号零 token，轮到即取锁 | 过 | test_lockdb_wait_turn_blocking_success：子进程阻塞至锁放自动取锁成功退出码零锁行归它；test_lockdb_wait_turn_timeout：超时出队退出码一 |
| F-5 得一裁 | 改写链两跳如实 | 挂起 | switch-1 boundary（依据族三值：基线四×5 基线一×3 基线五×1，起草面过宽如实申报）；switch-2 收窄重立 near_threshold（基线四×7 基线一×2），sign 未执行待人复核 |

## 二、实装要点

- lockdb：enqueue（事务内计数返位次）/ queue_head / dequeue / queue_view / takeover_release（清锁清队逐笔 takeover_released 只 INSERT）。
- lockcore.acquire：wait 参（缺省兼容），撞锁经队列调度三态（入队受理 / 非队首保序拒 / 队首真占拒）；成功出队。
- cli：--wait 旗标、takeover 子命令（两态）、排队位次返回形态。
- HEARTBEAT_STALE_SECONDS=300 维持冻结（推导档申报：超时选值无自然常数，PROB-013 相邻不精确覆盖如实记，清账经用户后裁或后批载体）。

## 三、得一裁挂起详情

改写链两跳如实：switch-1 九发全 comply 稳而依据族三值分散判 boundary（起草位依据面分配过宽，含基线五治理贡献面，如实申报不粉饰）；按修订一/九通道分解收窄重立 switch-2（新 gid、谱系披露、方向对己不利、剥离治理贡献面聚焦纯机制面），判 near_threshold（基线四×7 基线一×2，与批二 m-fixguard-switch-1、批四 m-lockdb-switch-1 同形态同成因，两件已经你「同意」落链放行）。材料指针：facet/contracts/leaseopt-lockqueue-260905/ 两 gid 全件。

## 三点五、wait-turn 增量（用户质询驱动）

用户 2026-09-05 会话质询「先拿号的 agent 会轮询吗」指出排队缺叫号——agent 拿号即退则需外部拉起，原地重试即空转回归。处置即 wait-turn 阻塞形态：agent 被要求排队即运行本命令阻塞等待，等待期是普通进程 sleep 查询（非 LLM 轮询零 token），轮到自己自动取锁返回成功退出码。等待逻辑收进确定性工具（基线一），agent 侧使用流变为：撞锁 → wait-turn 阻塞 → 成功返回即开工。

## 四、越线申报

1. 意图事件两笔（首笔锁前 658db0bc、重追加 cbe02190）——重追加为流程冗余如实申报，留痕不抹。
2. 改写链第 1 次未超 R7 三次护栏；boundary 先例原裁决留痕不抹。
3. recall 先导检索零命中如实记；checkcite pass 在档。

## 五、待复核后收口路径

人节点确认后：confirmation 落链 → 执契 check→verify（sign 机制不可用先例同形）→ 三仓 settle → close 归并 → reconcile → 链 verify → 主树复验 → 收口附记。租约线至此收官，pk-045 出泊呈裁。
