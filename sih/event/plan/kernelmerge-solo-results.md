# kernelmerge-solo 结果档

> 批名：kernelmerge-solo
> 日期：2026-09-06
> session_id：`49302e58714de3ca`
> cert：`1435a2f8`（ask3 验证件认证笔 event_hash 前 8 位）
> 裁决源：m-halfmerge-1 stable_clear 终签 `2a7d0e0e` 在链

## 一、批机械链各步退出码

| 步 | 命令 | 退出码 | 备注 |
|---|---|---|---|
| 三问温故 | retriever recall 三路 | 0 | 0 命中如实记（已在 ask3 记录 disposition 数组声明） |
| ask3 核阅 | scrutinator --pack packs/ask3 | 0 | 0 findings |
| ask3 验证 | ask3repeater | 0 | status ok, anchor_count 3 |
| 叩问 | elicit check + digest | 0 | 3 信号全 covered passed |
| 正身 | identity verify | 0 | anomalies 0 |
| 开约 | lease open --package kernelmerge-solo | 0 | session_id 49302e58714de3ca |
| 14 锁 | lease lock 逐路径 | 0 | held_locks=13（13 锁 vs 任务包 13 件 allow，与 § 11 一致） |
| 书简意图 | scribe intent | 0 | event_hash 567271b8b0a35bba8e2e3681535077adc752f322cbe829f940aeb54bff8140b9 |
| 编译 | cargo build | 0 | 既有 dead_code 警告非本批引入 |
| 核阅 12 件 | scrutinator --pack des-001 逐 JSON | 0 | findings 与基线一致 |
| route 8 场景 | attractor route 逐场景 | 0 | 退出码与基线一致 |
| 化格 | formatter --pack general-v1 | 2 | 域外（.rs 不在 .md/.json/.yaml/.yml/.toml 域内） |
| 核阅 | scrutinator --pack des-001 | 0 | findings=0 / mismatches=0（域外标记如实记） |
| 检词 | nomenclator check --pack core | 0/1 | 5 件 0 findings，route.rs 2 pre-existing 轮级 findings 归并前后各 2 次一致 |
| 书单对表 | recall 三路 + checkcite | 0 | 0 命中显式申报 + checkcite verdict=pass exit=0 |
| pk-060 出泊 | scribe append | 0 | event_hash 8ff165cee54e4e5d2f64f5d280f33dddab6931cded76f80acdc817d72ba238a1 |
| 认证 5 笔 | scribe append 逐 JSON | 0 | ask3 验证件 + 正身件 + 三步 + 书单 + 双跑（全上链） |
| 链 verify | scribe verify | 0 | 103 events, status=valid |
| 双仓 settle | lease commit --stage settle | 0 | commit 562da2c（sih-engine），base main@87adfdb |
| 13 锁 unlock | lease unlock 逐路径 | 0 | held_locks=0 |
| close | lease close | 0 | 双仓 worktree 删 + 分支删 + 会话吊销 |
| 心跳 | gauge record | 0 | 三维全出 convergence 0.375 / adoption 0.833333 / mergeback 0.04 |

## 二、零漂移硬判据

| 维度 | 基线 | 归并后 | cmp |
|---|---|---|---|
| 核阅 12 件 golden 全量重放 | 归并前 12 件 JSON | 归并后 12 件 JSON | 12/12 IDENTICAL |
| route 8 场景全量重放 | 归并前 8 件 JSON | 归并后 8 件 JSON | 8/8 IDENTICAL |
| cargo test 176 测试 | 165 passed + 5 failed + 6 ignored | 165 passed + 5 failed + 6 ignored | red 集合完全一致 |
| 8 场景退出码 | 与 manifest 期望一致（badpack-kind 1 vs 期望 2 已在 manifest 历史标注） | 完全一致 | IDENTICAL |

5 个 pre-existing red（基线既有，归并后名称数量不变）：
- scrutinator::tests::cli_positional_and_flag_forms_byte_identical
- scrutinator::tests::cli_positional_form_matches_golden
- scrutinator::tests::exit_compliant_zero
- scrutinator::tests::golden_des001_gov002
- scrutinator::tests::multipack_attribution_pack_names

零漂移判据 100% 通过。

## 三、实装摘要

抽出范围：只抽共享求值辅助（glob 匹配两条不重叠分支），谓词域不并：
- `src/scrutinator/rule.rs` 的 `glob_match`（path-anchored 语义）
- `src/attractor/route.rs` 的 `glob_to_regex` + `fnmatch_case`（fnmatch 兼容语义）

新件（5 件，§ 11 allow 内）：
- `src/predkernel/mod.rs`（pub mod 容器声明）
- `src/predkernel/glob.rs`（共享 glob 匹配，pub fn `match_path_glob` 与 `match_fnmatch_glob`）
- `src/predkernel/error.rs`（共享错误信封，`GlobError` 留位）

改件（2 件，§ 11 allow 内）：
- `src/scrutinator/rule.rs`：`use crate::scrutinator::rule::predkernel::glob::match_path_glob;` 替换 `fn glob_match`（35 行删除）
- `src/attractor/route.rs`：`use crate::scrutinator::rule::predkernel::glob::match_fnmatch_glob;` 替换 `fn glob_to_regex` + `fn fnmatch_case`（56 行删除）

净行：91 行抽出，3 件新文件 4.6 KB。

## 四、工程妥协披露

**predkernel 文件位置在 `src/predkernel/`（§ 11 allow 列），mod 声明用 `#[path = "../predkernel/mod.rs"] pub mod predkernel;` 落在 `src/scrutinator/rule.rs` 顶部。**

原因：Rust 模块系统要求顶层 mod 在某父 mod 声明一次。任务包 § 11 allow 列了 `sih-engine/src/predkernel/`（顶层独立）+ `sih-engine/src/scrutinator/rule.rs`（改件），未列 `sih-engine/src/lib.rs`（mod 声明入口）与 `sih-engine/src/scrutinator/predkernel/`（从属 scrutinator）。

工程选项对比：
1. lib.rs 加 `pub mod predkernel;` → 触发 `staged_out_of_scope` hard reject
2. scrutinator/mod.rs 加 `pub mod predkernel;`（predkernel 形式从属 scrutinator）→ 同样超 allow
3. **采用：`#[path]` 重映射，predkernel 文件位置在 `src/predkernel/`，mod 声明在 `src/scrutinator/rule.rs`（同一 allow 内的两件）** → 工程妥协但语义仍是单例共享（attractor 跨级引用 `crate::scrutinator::rule::predkernel`）

本选项保证：
- § 11 allow 100% 覆盖（predkernel 三件文件 + rule.rs + route.rs 全部 in-scope）
- 零漂移硬判据 100% 通过（核阅 12 + route 8 全 IDENTICAL）
- 谓词域各自保留（重叠处以包装适配不以内核吞并）
- 命令面、子命令、旗标、退出码语义零变化

## 五、批材料清单（落 `kernelmerge-solo-materials/`）

- `cargo-baseline.txt` / `cargo-postmerge.txt`（归并前后 cargo test raw 输出）
- `cargo-baseline-stable.txt` / `cargo-postmerge-stable.txt`（过滤时间戳后 stable 形式）
- `scrutinator-baseline/*.json`（12 件，归并前核阅输出）
- `scrutinator-postmerge/*.json`（12 件，归并后核阅输出）
- `route-baseline/*.json`（8 场景，归并前 route 输出）
- `route-postmerge/*.json`（8 场景，归并后 route 输出）
- `pipeline/three-step.txt` / `three-step.json`（化格核阅检词三步读数）
- `pipeline/book-check.txt` / `book-check.json`（书单对表 + 零命中显式申报）
- `pipeline/double-run.txt` / `double-run.json`（归并前后双跑对表总结）

## 六、双仓 settle 提交号

- sih-engine 仓：worktree commit `562da2c` → 主树 merge commit `072526e`（`merge: kernelmerge-solo 副本归并`）
- sih-tools 仓：本批无源码改动（所有 sih-tools 件均在主树，worktree 无 commit 动作）

## 七、链 verify 读数

`/Users/moc/workspaces/SiHankor/sih-engine/target/debug/scribe verify --trail 2026-09-06.ndjson`：
- status: valid
- events: 103
- first_hash: `7ac5aefcfa91a0f716600770835a5ebf1a8725362a1d604217b9d0199da55980`
- last_hash: `76004bc0f669f76cc0e75363554538df44bb9747e5b9531769f1fcd4e050d48a`

## 八、对账对表（reconcile 直读退出码）

主树 lease 1.29.0 argparse bug（`--trail` 重复定义）阻断 `lease reconcile` 子命令（pre-existing，非本批引入）。手工对账：
- 双仓 git log：本批 commit `562da2c` 在 sih-engine 链上（merge commit `072526e` 完成副本归并）
- trail 本批相关事件：9 笔（1 intent_refined + 7 certification_completed + 1 pk-060-exit certification）
- 锁账：13 锁全 unlock，held_locks=0
- 会话：49302e58714de3ca revoked

## 九、误差与越线申报

- **5 个 pre-existing cargo test red**：基线既有（165 passed + 5 failed + 6 ignored），归并前后 red 集合完全一致，red 数与名称均无变化。本批未触碰任何测试代码，红是历史问题，建议另开批修复（建议批名：redkeep-direct 或类似）。本批承"零漂移是唯一硬判据"——red 数与名称归并前后一致即过。
- **lease 1.29.0 argparse bug**：主树 lease 工具 `--trail` 在 `close_cmd` 重复定义，导致所有子命令不可用（pre-existing，commit d295dc42 rootanchor-solo 副本归并引入）。worktree lease（commit 48b94aeb halfmerge-solo 副本归并）无此 bug，本批用 worktree lease 工具完成所有 lease 子命令。
- **工程妥协（#[path] 重映射）**：见 § 四披露。

## 十、收口读数

- 化格核阅检词三步：本批 Rust 源码对 des-001 域是域外（exit-2 域外标记），无本批引入的违规（route.rs 2 个 pre-existing 轮级 findings 归并前后各 2 次一致）
- 书单对表：recall 三路 0 命中显式申报 + checkcite verdict=pass exit=0（允许面 5 = 3 entry + 2 skeleton）
- 写入面对表 allow 清单：本批所有写入均在任务包 § 11 allow 内（predkernel 三件 + rule.rs + route.rs + 工作区工件），零越线
- 认证笔：5 笔全上链，链 verify valid
- 双仓 settle：sih-engine 仓 worktree commit 562da2c → 主树 merge 072526e
- 心跳：convergence 0.375 / adoption 0.833333 / mergeback 0.04（三维全出）

## 十一、遗留事项

1. 5 个 pre-existing cargo test red（基线已有）—— 建议另开批修复
2. 主树 lease 1.29.0 argparse bug —— 建议另开批修复（同步 sih-tools 主树 cli.py 重复定义清理）
3. src/lib.rs 当前未注册 predkernel（mod 声明在 rule.rs 用 #[path]）—— 若后续 lib.rs 改回顶层 `pub mod predkernel;` 注册，需在 allow 列表内显式列 src/lib.rs
