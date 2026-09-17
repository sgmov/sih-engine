# openface 子代理施工报告存档（2026-09-17）

实现：KNOWN_REPOS 双已知仓常量；derive_repo_names 声明面仓前缀解析（对表 closegate repo_entries 形）；compose_repo_names 并集去重保序；cmd_open 组装位改写即纯旗标缺省段换 compose 形。回落缺省 sih-engine 保 t2 金向量（包档声明 sih/ledger/* 无仓前缀）。

测试四件红转绿证据：A 实现前 left ["sih-engine"] right ["sih-engine","sih-tools"]；B 实现前 left ["sih-tools"] 即纯旗标丢推导仓；B-prime 实现前红于 git worktree add failed: branch already exists；C 兼容钉实现前即绿。实现后 4 过 0 败。

全量：cargo test --bin lease 24 绿；mergeback t2 金向量 4 绿不破、t3 2、t4 1、t5 5、t6 5、t7 7；allow_parse 2；gap_mergeback 3；pathfix 3。回执示例（双仓声明加域外件零旗标）：allow 含四路径，repos 双仓各载 msh/demo 分支与 worktree 位，scope_source package，域外件入 allow 不增仓。
