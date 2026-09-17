# identity-fix 批结果档

> 2026-09-17，会话 98f1a21883ddf142，单线 solo。令源：用户问「identity的bug修复了吗」，承同日根因定位视为修复令。缺陷正典证据：本目录 evidence-137.md。

## 一、修了什么 {#what}

引擎 bin `identity` 无盐裸跑挂死（退出码 137）：`new_salt()` 用 `fs::read("/dev/urandom")` 取盐，`fs::read` 读到 EOF 而该设备是无限流，向量无限增长直至被杀。修法：`File::open` 加 `Read::read_exact` 定长读 32 字节（新抽 `salt_from_reader<R: Read>` 位），镜像围堰 `os.urandom(32)` 语义；兜底 UUID 径与身份串与哈希合成位零改动。全仓 grep 确认 urandom 弹点仅此一处，mcpserver 与 calllog 无同型病。

## 二、验收证据 {#evidence}

- 单元测试四件全绿（`cargo test --bin identity`，worktree 实跑）：无限源定长返回（回归钉，修前此形挂死）、短源返 None、连续两跑盐不同（围堰 F6）、盐格式 64 hex。
- 全量回归：`cargo test --workspace` 在 worktree 实跑，结果见本档第三节补记。
- 整合验收：修复后二进制 `identity verify --no-net`（无盐，即修前挂死径）五连跑全 rc=0，耗时 0.19 至 0.78 秒（修前三十秒不退被杀）；盐与盐哈希逐次不同，核哈希稳定（本就是无盐规范形）。
- 红证申报：修前红态不重放烧机（三十秒 GB 级内存代价），红态证据即 evidence-137.md 的 ps 曲线与 sample 栈。

## 三、全量测试补记 {#suite}

- `cargo test --workspace` 在 worktree 实跑：本批目标 `--bin identity` 四件全绿（0.00 秒，无限源钉实证定长读）；全仓其余套件绿。
- 环境性红一族如实申报：`gap_parser_pack_assets` 两败（t1 vectors 零向量、t2 语言包目录解析败），**主树修前同参对照同败在案**（本日实跑 1 过 2 败同形），非本批码坏。候修输入：parser 裸包名资产解析在 temp cwd 下解析不到 packs/parser 语言包目录，疑 exe 派生包根在临时目录场景失锚，归 parser 资产解析候修批。

## 四、偏差申报 {#deviations}

- 正身报告由围堰 identity 只读产出替位（引擎位修前挂死不可自证），mergeall-closeout 先例；本缺陷本体即 SPEC-025 移植件（lease-mergeleg23-parallel 簇E）引入的移植缺欠，已列候修项随本批清偿，修复后引擎位自证恢复。
- 任务包 idf-03 的 SDD 完备度门：单点机械缺陷修复批无独立 DES，close 时若 SDD 门拦按 DEC-024 bypass 留痕申报，TDD 门以新回归测试承载。
- SDDG 首拦补录（如实留痕不抹）：SDDG-3 判偏差条目零承载词形、SDDG-4 判差分缺 tests 面文件，两判均属实——补 tests/identity_salt.rs 端到端回归（15 秒超时闸，旧病回流即红不挂套件）并修订本节承载词形后复约，未动 bypass 通道。

## 五、改动文件 {#changed}

- sih-engine/src/bin/identity.rs（new_salt 定长读加 salt_from_reader 抽取加 salt_tests 四件）
- sih-engine/sih/state/plan/identity-fix.md（任务包）
- sih-engine/sih/event/plan/identity-fix-materials/（意图件、正身报告、evidence-137.md、本档）
