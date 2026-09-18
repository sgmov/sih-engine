# testhard 批结果档

## 验收判词 {#verdict}

- 三件全先红后绿：f7 快照形根除活链竞态（红证即跑间追行复现假红，快照形同场景绿且快照忠实性行数加字节双证）；三十件假红族清零（修前红名单 30 留档，修后主树与工地与自有 target 三形俱绿）；park 重放收窄（垃圾 ndjson 假行忽略，规范链与 splinter 白名单与声明面三钉在案）。
- 全量回归零新红：worktree 共享 target --lib 262/0 加 F 套件 9/9 加 pendline 15/0 加 lease 系俱绿；主树裸跑 258/0 基线绿。

## 偏差申报 {#deviations}

- CARGO_BIN_EXE 编译期机制对 lib 单测不可用（env! 直接编译失败），改运行期 env::var 优先加 current_exe 同胞解析，语义等价已注记 src/testbin.rs 头（子代理申报一）。
- 共享 target 陈旧二进制环境坑记档：同包名 artifact 后建者胜，双形验收前须强制重建（子代理申报二，环境性质非本批缺陷）。
- f7 快照根取 canonical 城形：first_domain 形 locator 相对路径依赖致 symlink 必败（status=2 实证），canonical 形经祖先上溯零 symlink（子代理申报三）。
- 残留候后续：tests/lease_mergeback_t3_exit_codes.rs 等仍存 manifest/target 旧寻址，属本批禁改条款外留（子代理申报四）。

## 处置记录 {#dispositions}

承用户 2026-09-18 令「那你调2个子代理，把2波都做了」第一波。三件病灶出处即 defectwave 波后全量回归实录与 defectwave 偏差申报与 pendline-results.md 偏差申报七。子代理施工、主会结算。
