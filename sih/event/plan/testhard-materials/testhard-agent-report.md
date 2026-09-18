# testhard 批子代理施工归报（全文照录）

## 件一：f7 活工作区竞态加固

根因：f7_double_run_identical 对真工作区 trail 背靠背双跑断言逐字节一致；全量跑窗口内当日主链（real_root 实为 SiHankor 工作区根，load_chain 读 sih-engine/sih/event/trail/*.ndjson 全量）被并发会话合法追加，跑间链面变行即假红；单跑必绿（今日实证 301.95 秒绿）。

改动：tests/mem_recall_f_suite.rs（文件头注 :1-10、f7 :194-251、f7_snapshot_root :253-293、f7_copy_flat :295-311）。修法把双跑输入冻结进临时 canonical 城形快照根（trail 全量 ndjson 加 canonical 内嵌记忆包 include 四域语料），两跑同指快照断言逐字节一致，双轴（event 加 topic）皆在面防退化；locator 子进程与链加载与排序序列化全实跑，只冻输入不冻代码。

红证（scratch 实测后即删）：活链双跑、跑间追一行认证事件即红证同参双跑须逐字节一致失败；同场景快照形绿加源链确已变行坐实冻结为唯一护因。绿证：worktree 共享 target 形 f7 ok（5.11s）；自有 target 形 ok（5.36s）；F 套件全跑 9/9（243.06s）。快照忠实性实证：同参查询快照根与真根输出行数相等（1884 行）且首三行逐字节一致。

## 件二：工地假红族扫净（30 件）

根因：lib 单元测试以 CARGO_MANIFEST_DIR/target/debug/<bin> 寻址治理二进制；worktree 形（CARGO_TARGET_DIR 指共享主 target）下 manifest 无 target 目录必红。

改动：新增 src/testbin.rs（52 行，两级解析：运行期 CARGO_BIN_EXE_<name> 在位优先，回落 current_exe 同胞定位 <target>/<profile>/deps/ 上跳两级）；src/lib.rs:20-23 挂 cfg(test) mod testbin；src/scrutinator/tests.rs:16-28（bin()）；src/event_stream/tdd_tests.rs（头注 :1-8、scribe_bin :20-22、t2 祖先上溯寻 trail :60-77、t6/ga1/ga2/ga3/ga4/t7/t11 七处寻位点）；src/event_stream/chainstamp_tests.rs（头注 :1-7、scribe_bin :48-50）。

红证（修前 worktree 形实测，基点 522321a）：scrutinator::tests 21 红（与申报数恰合）、tdd_tests 8 红（t2/t6/t7/t11/ga1/ga2/ga3/ga4，申报单外 ga3 亦红已补扫）、chainstamp_tests 1 红（confirm_three_states），共 30 红留档。绿证：worktree 共享 target 形 --lib 全绿 262/0；主树裸跑形 --lib 258/0；自有 target 形 --lib 262/0。逐用例核对无二进制缺席降级前提用例。

| 套件 | 修前红（worktree 形） | 修后（worktree 共享） | 修后（主树裸跑） | 修后（自有 target） |
|---|---|---|---|---|
| scrutinator::tests | 21 | 39 绿 | 39 绿 | 同左 |
| tdd_tests | 8 | 16 绿（族内） | 16 绿 | 同左 |
| chainstamp_tests | 1 | 4 绿（族内） | 4 绿 | 同左 |
| 合计 | 30 | 全绿 | 全绿 | 全绿 |

## 件三：scribe 泊界重放面收窄

根因：load_parking_scope（src/event_stream/park.rs）read_dir 吃 trail 同目录全部 ndjson，非链件（随手笔记）内 parking_entered 行被误吃进泊界账，污染号源判定。

改动：src/event_stream/park.rs load_parking_scope :182-206 收窄为规范链文件面：<YYYY-MM-DD>.ndjson 日期形 加 日期前缀 splinter 保全件形（is_canonical_chain_name :208-223、is_date_form :225-233，白名单覆盖实件 2026-08-28-pk023spec-pre-replay-splinter.ndjson，去重语义零破坏）加 --trail 显式指定文件自身（首次落笔前未在盘不推入）；新增三测试 :381-437（红转绿钉、splinter 白名单钉、非日期 trail 声明面钉）。

红证：旧全量 filter 下 non_chain_ndjson_ignored_in_parking_scope 红：重放面只认规范链，垃圾 ndjson 假行忽略失败（scope 2 行且假号被误拒）。绿证：收窄后 scope_tests 10/10、event_stream 族 99/0；CLI 侧 pendline 15/0 零回归。

## 全量回归（cargo test --no-fail-fast）

worktree 加共享 CARGO_TARGET_DIR 形：--lib 262 passed 0 failed 8 ignored；mem_recall_f_suite 9/0（243.06s）；defectwave_scrutpath_wform 3/0、pendline 15/0、lease_mergeback_t5/t6 5/0 5/0、lease_cascadeclose_projection 6/0、attractor_contract 7/0、gap_mergeback_uncommitted_landing 3/0。主树裸跑形：--lib 258/0（主树源零触碰基线绿）。worktree 自有 target 形：--lib 262/0 加 f7 1/0（最强形无关证明）。

## 偏差申报

1. CARGO_BIN_EXE 机制偏离：该机制只对 tests/ 集成目标编译期注入，lib 单元测试 env! 形直接编译失败；改用运行期 env::var 优先加 current_exe 同胞解析（语义等价：定位 cargo 实际产出位，与 CARGO_TARGET_DIR 无关），已注记 src/testbin.rs 头。
2. 共享 target 陈旧二进制坑（环境性质非本批缺陷）：主树与 worktree 同包名产物共用同一 artifact 名，后建者胜；mtime 指纹可掩需建重建，双形验收前须自受测树强制重建（touch src/lib.rs）。中途一次 30 假红复现即此因，重建后消解。
3. 件一快照根取 canonical 城形而非 first_domain 镜像：first_domain 形 locator 码根须 symlink，而 locator pyproject 带 parser 相对路径依赖，symlink 形 uv 解析必败（status=2 实证）；canonical 形经祖先上溯用真 sih-tools/locator 零 symlink。真 trail 冒烟形弃留（排序行集等在跑间追加下仍假红，冻结形唯一干净），依任务条款择净申报。
4. 件二范围：tests/lease_mergeback_t3_exit_codes.rs 等仍存 manifest/target 旧寻址（属 lease 面禁改条款），未触碰，留后续批。
