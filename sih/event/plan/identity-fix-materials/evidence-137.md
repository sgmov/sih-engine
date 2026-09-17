# identity 137 缺陷修前证据（2026-09-17 本会话实测）

- 命令：`target/debug/identity verify --at <now> --no-net`（无盐，即生产缺省径）
- 现象：三十秒零输出零退出；RSS 采样（KB）：1457760 → 2871424 → 95920 → 623776 → 794624 → 869296 → 1311248 → 602704 → 234432 → 649600，VSZ 单调增至 452083952；外部 SIGKILL 后退出码 137。
- 热点栈（sample 3s，2506 样本全集中）：main → identity::main (identity.rs:781) → identity::new_salt (identity.rs:163) → std::fs::read::inner。
- 判读：非泄漏型单调增长（RSS 波动系分配器回收复用），是 `fs::read("/dev/urandom")` 对无限流读到 EOF 的失控读，内存上限即物理内存。
- mergeall-closeout 批同象登记：「引擎 bin 裸环境 137 复发，缺陷随批登记」，与本证据同因。
