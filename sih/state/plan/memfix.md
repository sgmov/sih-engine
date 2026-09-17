# memfix：locator_bridge 临时件掺名并行抖动修复

> 令源：2026-09-18 用户令「全都推进，你是主编排，调用多子代理进行并行操作」，候令簿第一件
> 病灶源：snapabsorb 批附带揪出，实证在 w/memfix 工地红证即 8 线程全同路径
> stem 认领：memfix，甲表一件即 zh 检索记忆通道临时件掺名修复、派生 memfix:new

## 问题陈述 {#problem}

sih-engine/src/retriever/locator_bridge.rs 的 index_path 与 canonical pack 临时件名只掺进程 id，Rust 测试框架同进程多线程跑测试时 pid 相同即互踩同一路径，mem_recall 套件并行偶红。同类掺名位 mod.rs:371 retr-env 由并行批 retrline 域内顺带处置，不在本包。

## 关键设计 {#design}

文件名在 pid 后追加线程 id 的 Debug 形，跨进程隔离不回退，函数签名与语义零变。单元测试起 8 线程取 index_path 断言互异，先红后绿。

## 工作清单 {#work}

- [ ] mf-01：两处掺名位追加线程 id
- [ ] mf-02：单元测试 index_path_distinct_across_threads 红转绿
- [ ] mf-03：settle 加 close 加结果档

## 验收 {#acceptance}

cargo test --lib retriever 全绿含新测试；mem_recall_f_suite 零回归；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/src/retriever/locator_bridge.rs
