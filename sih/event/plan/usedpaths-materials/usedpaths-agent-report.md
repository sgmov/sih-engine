# usedpaths 批子代理施工归报

- 工地：w/usedpaths（手工形），移植至租约形 worktrees/sih-engine/usedpaths（分支 msh/usedpaths）
- 施工形：委外子代理亲写零再委外，零 git 写动作零治理命令

## close 收约罚金路径现状梳理

- 罚金唯一写点：`src/bin/lease/closegate.rs` `cmd_close` 末段账单块（原 :1093-1144，紧随吊销行之后）。locked_paths 源自 locks 台账本会话 acquired 事件去重；used_paths 无独立来源，直接以会话记录 allow 数组全集近似（承围堰 core.py bill_close_unused 同形）；unused = locked 减 allow 字符串全等差分，非空即落 unused_lock_penalty 事件八字段入 lockface-bills.ndjson。
- 关键时序：罚金块在归并删支拆本之后执行，彼时分支已删工地已拆，diff 不可取样，故取样点必须前移。
- path_count 与 open_face_bill：引擎位基点 a69fe6b 不存在（全 src 零命中），开工计费面只在冻结围堰。
- 会话记录可用字段：cmd_open 每仓写入 base_branch 与 branch 与 repo 与 worktree，开工基在案。

## 实装点改动摘要（仅 closegate.rs，+132/-9）

| 实装点 | 位置 | 改前 改后 |
|---|---|---|
| used 改源 | 新增 collect_used_paths（:213-281），调用位 :664 归并前取样，罚金块 :1231 消费 | used≈allow 全集 → 收约在会话工地跑 git diff --name-only base..branch 按仓名前缀化为台账同域路径；任一仓 git 失败整体回退 allow 近似并标 used_source=allow_fallback，成功标 worktree_diff |
| 前缀覆盖 | 新增 entry_covered（:197） | 字符串全等 → watchcheck covered 同形：精确命中或祖先目录命中，双向即锁面目目录条目罩 diff 文件件、allow 回退面目录条目罩锁面文件件 |
| 斜杠归一 | 新增 face_entry_map（:179），locked 与 allow 两面同经归一 | 原样字符串集合 → rstrip 斜杠归一去重 |
| schema 兼容 | 罚金事件（:1246-1260） | 既有八字段零删零改义；detail.used_source 唯一新增字段；unused_paths 值为归一形 |

## 测试清单（新件 tests/lease_usedpaths_penalty.rs，承 cascadeclose harness 形）

1. written_outside_allow_not_penalized：allow 声 B 锁 A+B 工地实写 A+B，新口径零罚单（52.4% 过罚退罚主路径）
2. unwritten_within_allow_penalized：声明 A 锁 A 分支零提交，罚 A，used_source=worktree_diff
3. declared_but_unwritten_lock_penalized_despite_other_declared_write：allow A+B 锁 A 实写仅 B，罚 A（§2.3 类三转罚面之钉）
4. dir_lock_prefix_cover_matches_diff_file：锁 dir/ 与 dir/other.txt 实写 dir/file.txt，目录锁前缀覆盖不罚唯未写件入罚
5. slash_variants_dedup_in_unused_differential：p 与 p/ 双锁归一单笔
6. nongit_repo_falls_back_to_allow_approximation：非 git 工地回退 allow 近似且 used_source=allow_fallback 如实在案

全量：lease bin 单测 24/24；lease 系集成 t2-t7 与 openface 与 allow_parse 全绿含 t2 close 回执金向量逐字节一致（罚金改动零扰动回执面）。

## 偏差申报

1. 任务测试例文案与 §2.5 语义相悖处从正典：罚 = 锁面且未被实改集覆盖，allow 内锁了未写仍罚承 §2.3 类三转罚面，测试 3 即按此钉形；若人节点裁「写了任一声明面即全免未用锁」改 entry_covered 一处即可。
2. path_count 处未改：引擎位无开工计费面，rstrip 归一已施于引擎位全部集合运算，围堰侧留解冻批。
3. merge-base 兜底在主仓执行（worktree HEAD 即分支自身，merge-base 须以主仓 HEAD 为第二参）；会话记录实有 base_branch，此兜底正常流不触发。
4. 取样点前移至归并前（:664）：罚金块原位在删支拆本后分支已灭不可 diff，行为等价时序必需。
5. 共享 target 下 lib 测试陈旧二进制缓存碰撞（CARGO_MANIFEST_DIR 内联他人路径）致假红，本工作台全新编即绿；另在 w/usedpaths/target/debug 留四软链指向共享 target（非 git 件随工地拆弃）。
