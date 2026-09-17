# openface-solo 批结果档

## 验收判词 {#verdict}

- tests/openface.rs 四件绿：A 双仓推导零旗标即回执 repos 含 sih-engine 与 sih-tools 且 branch 与 worktree 位俱正、B 旗标与推导面并集、B-prime 重复旗标去重即旧形同仓重挂撞死自愈、C 无仓前缀声明回落缺省即金向量兼容钉。
- 红转绿证据：实现前红态即 1 过 3 败（A 败于单仓缺 sih-tools、B 败于纯旗标丢推导仓、B-prime 败于 branch already exists），实现后 4 过 0 败。
- 全量回归：cargo test --bin lease 24 绿；lease_mergeback t2 金向量 4 绿逐字节比对不破、t3 至 t7 俱绿；lease_open_allow_parse 2 绿；pathfix 3 绿。子代理报告全文存批材料 openface-agent-report.md。

## 偏差申报 {#deviations}

零偏差申报：改动唯一生产码文件即 src/bin/lease.rs 加 43 减 4 行，头注记一行，与任务包声明面一致。子代理施工期一次 cwd 误落主树的只读 git status 查看零写入无痕。

## 处置记录 {#dispositions}

承 pkexits3 批 pk-104 出泊裁定即本批为承载批。排障注记照录：cargo test 只刷 deps 件不刷顶层 bin，首轮演示曾踩陈旧顶层二进制出假单仓回执，cargo build 后新鲜二进制复跑复证俱绿。
