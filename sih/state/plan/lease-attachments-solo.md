# lease-attachments-solo 任务包（单线形）

## 形声明与令源

- 队形：单线 solo，主线亲写零子代理（四件强耦合于引擎 lease 源码上下文，拆簇无收益）。
- 令源：用户 2026-09-14「腿三附件面可以施工了」；承 SPEC-024 三腿切分，腿一锁核绿（lease-lockcore-green-solo）、腿二收约执法绿（lease-commitlaw-parallel），本批腿三收口。
- 温故检索先例：calllog-solo-results.md（import 正典形与 P3 迁移忠实形）、sweepimpl-solo 判据（c2close2 结果档引用）、检索件 work/lease-attachments-materials-recall.ndjson 随批入 materials。

## 目标

SPEC-024 腿三附件腿四件 Rust 对表实装加余量一件：

1. **sweep 子命令**（对表 sweepcore.py 455 行）：五类普查（幻影会话四证取二、僵尸锁、停滞检验文件 pid 探针四态、散位收据、无主工地）三态输出，--fix 走既有通道（修正性 revoked 终笔、锁释放留痕、检验件清除、收据归家），render_lines 人读形，退出码三值。
2. **call-log import 子命令**（对表 calllog_import.py 269 行）：19 册 markdown 逐行分类（header/sep/row/list/blank/text）verbatim 整行入权威腿 ndjson（sort_keys 压缩分隔符），表格行尽力解析失败空值申报，一次性语义（已导入拒），投影腿 verbatim 重组再生；sqlite 索引腿循腿二 bills SQL 同款申报未实装（Cargo 零 rusqlite，A9 依赖面零新增）。
3. **install-hooks 与 uninstall-hooks 子命令**（对表 commitcore.py:461-489）：git config core.hooksPath 写／unset，rc5 already_absent 收敛，缺省双仓。
4. **pk-103 收口**（出泊承批）：引擎 stem 闸词典包路径按 --root 锚定与 cwd 无关；全查路实装（读 nomenclator 词典 JSON 查词六态，未知词 --new-stem 甲表三件认领通道，对表围堰 core.check_stem_gate）。

## 验收判据（可证伪，A9 加 T7）

- F1 sweep 净态退出码零、候裁件三态清单逐件 class/object/state/criteria/channel 四键；--fix 自清件处置行与复普查 post_fix 在案。
- F2 call-log import fixture 双册 verbatim 零丢（重组逐字节还原）与一次性拒与投影腿再生对表。
- F3 hooks 安装拆卸回执四键（hooks_path/installed/repo/detail）与 rc5 形；引擎 pre-commit/commit-msg 钩子件接锁语义对表围堰现行文。
- F4 stem 闸：真根 open 走全查（established 词过闸、unknown 词 --new-stem 认领过闸、裸认领拒）；fixture 形 pack_absent_skip 回归不破。
- F5 主树 lease 全族回归绿（腿一 T2-T4 与腿二 T5-T6 与本批 T7）；当日链 verify valid。
- F6 SPEC-024 修订三 v1.3 腿三执行记录与落差如实申报；T6 三步。

## 范围与边界

- 围堰 sih-tools/lease 源码零改动红线延续（对表面只读）。
- 僵尸锁现势锁面：围堰从 sqlite lockdb 投影读，引擎从 ndjson 锁册 acquired/released 事件序推导——语义同构落差如实申报，不实装 sqlite。
- pk-098 其一（孤儿锁清偿无门）本批只申报设计草案留泊不动 closegate 判序；pk-094 其四（围堰 self_boot）围堰冻结留泊。
- 明确不做：reinit、takeover／ledger-repair 独立子命令（sweep fix 内联通道已覆盖）、heartbeat／wait-turn／preempt-release（候后批按需）。

## 改动文件清单

sih-engine/src/bin/lease.rs（dispatch 四件）、sih-engine/src/bin/lease/sweepcore.rs（新）、sih-engine/src/bin/lease/calllogface.rs（新）、sih-engine/src/bin/lease/attachments.rs（新，hooks＋stem 闸收口）、sih-engine/tests/lease_mergeback_t7_sweep.rs（新）、sih-engine/doc/spec/SPEC-024-lease-mergeback-gap-v1.md（修订三）、本包、结果档与 materials、sih-engine/sih/state/parking/materials/pk-103-exit.json、当日 trail。
