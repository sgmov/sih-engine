# pk057fix-solo 结果档（台账写点唯一化与 flock 串行）

> 批：pk057fix-solo（pk-057 根因硬化，主会亲跑第三任前两任 ENOENT 环境交接零损失承继）
> 会话：b63758e2745f7100（ask3 侧 sess-zcode-260905-pk057fix，双标识空间各认各）
> 日期：2026-09-05。队形：单线形 solo，零子代理。
> 令源：用户 2026-09-05「同意，跑这个修复方案」；当日台账重写竞态四例同根（docmath-b4、facepark、gateswitch、constmodel）。

## 一句话结论

pk-057 根因硬化交付在役：ledgerwrite.py 唯一写点（flock 排他 + O_APPEND 原子整行，行格式字节不变零迁移）与 ledger_lock 合并窗口互斥与 restore_missing 回补网；8 进程×25 行并发压测 200 行零丢零交错，全测试族 110 绿零回归；CONTRACT 1.24.0 修订三十六与 BATCH-FACE 台账写点纪律节随批；pk-057 出泊 promoted 落链（95fa764c），名录八项。

## 一、F 表（完成度表）

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 并发压测 | 8 进程×25 行零丢零交错 | 过 | tests/test_ledgerwrite.py::test_f1，200 行精确、逐行合法 JSON、每 worker 恰 25 |
| F-2 互斥 | 持锁期追加阻塞释放即续 | 过 | test_f2：0.3 秒窗未完成断言 + 释放后完成断言 |
| F-3 回补网 | 缺失行逐行回补 | 过 | test_f3：3/1/2 序（新行前置、备份两行按原序回补） |
| F-4 零迁移 | 行格式字节一致 | 过 | test_f4：_row_bytes 与既有 sort_keys 紧凑形逐字节一致 |
| F-5 回归 | 全测试族绿 | 过 | 110 passed（106 存量 + 4 新增）零回归 |
| F-6 收口 | settle/close/reconcile/verify | 见收口读数 |

## 二、越线与误差申报

1. **TDD 先红后绿偏离**：并发丢行属概率性，红证不可稳定构造——以行为契约测试与多进程压测承载验证，任务包九节偏离申报在案。
2. **零判定语义变更声明**：行格式、接受/拒绝行为、active_sessions 配对语义全部不变；纯写路径机制改造。批内未触发停批过得一裁条件。
3. **CONTRACT 修订三十五勘误**：彼时文内「复用 append_event flock」与实现不符（append_event 本体无 flock），修订三十六补实。
4. **会话现势表入 lockdb 列二期**：本批零触碰，gate 3 与 precheck 的 ndjson 全文件扫描读位维持现状（读操作无竞态损害，性能优化另行）。
5. **适用边界**：flock 在本地 APFS 可靠，NFS 类不承诺——如实声明。

## 三、管线与链

- 三问双门：ask3 三锚（鉴/应/P3.1 原文程序切片逐字节）双门零违规，digest covered 3
- 正身：attest 0 异常；租约 open b63758e2745f7100 锁十路径
- 意图：d274e1eb 在链
- 测试：110 passed 零回归；压测/互斥/回补/字节格式四件全过
- 契约：CONTRACT 1.24.0 修订三十六、BATCH-FACE 台账写点纪律节

## 四、收口读数（close 后回填）

- 待回填：双仓 settle 号、close 读数、reconcile、verify 终读、pk-057 出泊后名录终读、live 并发自证读数
