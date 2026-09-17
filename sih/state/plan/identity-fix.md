# identity-fix：正身盐源无限读修复

> 令源：用户 2026-09-17 问「identity的bug修复了吗」（承同日根因定位与实测证据，视为修复令）
> 范式：solo 单线，agent 亲写零子代理
> 前置：readme-plain 批在途（会话独立，不共享锁面）；缺陷实测证据在本批 materials/evidence-137.md
> stem 认领：identity-fix，甲表三件即 zh 正身盐读修复、code 无承、派生 identity:established 加 fix:new

## 一、问题陈述 {#problem}

- 引擎 bin identity 无盐裸跑挂死（退出码 137 即 SIGKILL），mergeall-closeout 批登记为「裸环境 137 复发候修」。
- 根因（本会话 2026-09-17 定位）：`src/bin/identity.rs` `new_salt()` 用 `fs::read("/dev/urandom")` 取盐。`fs::read` 语义为读到 EOF，而 `/dev/urandom` 是无限设备流恒不 EOF，向量无限增长直至被杀。实测 RSS 六秒 2.8GB、三十秒不退；`sample` 调用栈钉死热点即 `std::fs::read::inner`。注释「只取前 32 字节」的 `take(32)` 在读完后才执行，永不可达。
- 影响面：一切无 `--salt` 的引擎位 identity verify 调用全数挂死。围堰位（Python os.urandom 定长）不受影响；全仓 grep 确认 `urandom` 仅此一处，MCP 载体与 calllog 无同型弹点。

## 二、关键设计 {#design}

- 修法：`File::open` 加 `Read::read_exact` 定长读 `SALT_BYTES`（32）字节，镜像围堰 core.py `os.urandom(32)` 语义；兜底 UUID 径不动。
- 可测化：抽 `salt_from_reader<R: Read>` 纯函数位，`new_salt` 只做文件打开与委托；回归测试用 `std::io::repeat` 无限源，修前此测试挂死（红），修后立即返回（绿）——无限源使「读到 EOF」的旧病在测试里必现且有限时暴露。
- 整合验收：`target/debug/identity verify --no-net`（无盐）修前挂死，修后秒级退出 rc=0。
- 红证如实申报：修前复跑挂死属高代价红（三十秒 GB 级内存），红态证据用本会话已测的 ps 曲线与 sample 栈，不重放烧机。

## 三、工作清单 {#work}

- [x] idf-01：worktree 内改 `new_salt` 定长读加 `salt_from_reader` 抽取
- [x] idf-02：回归测试两件（无限源定长返回、盐格式 64 hex）
- [x] idf-03：`cargo test --bin identity` 绿 + 整合验收 `identity verify` 秒回 rc=0
- [ ] idf-04：settle commit + close + 归并 + reconcile

## 四、验收 {#acceptance}

- `cargo test --bin identity` 全绿含新回归测试；引擎 bin 无盐 verify 五次连跑全秒回 rc=0 且盐逐次不同（F6 生产盐随机）；围堰对表语义不变形（identity_string 与哈希合成位零改动）。
