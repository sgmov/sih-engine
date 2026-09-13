# 簇 B 取材材料卷：泊界缺陷件 pk-096 / pk-097 / pk-098 / pk-100 逐件对账

批属：mcpdefect-parallel（取材簇 B，只读取材位）
取材日：2026-09-13
取材方式：逐件读泊件原文（含 -exit.json 在档件）后对现行源码逐行核验，判词全挂 file:line，零缺陷描述推断。
背景对位：腿二批 lease-commitlaw-parallel（结算提交 81872a0）在 sih-engine Rust lease 实装 commit 四验（src/bin/lease/commitlaw.rs）、SDDG 闸（src/bin/lease/sddgate.rs）、close 全闸序（src/bin/lease/closegate.rs）、守卫判序（src/bin/lease/guardlaw.rs）；围堰 Python lease（sih-tools/lease/）为融回冻结面。

---

## pk-096 nomenclator_check 相对路径锚定司衡根而非 Bearer 域根

判词：**已被核销（domface-solo 批，非腿二批）**。源码修复与红测俱在档；唯泊件出泊手续缺席（materials/ 下无 pk-096-exit.json，泊件仍 parked 在档），出泊随批办理属批结算事务，非代码缺陷残面。

证据：

- sih-tools/mcpline/src/mcpline/server.py:260-263，相对 target 现锚所绑域根，layout 缺席才回退中央根，注释自署 pk-096：

```python
    tp = Path(t)
    if not tp.is_absolute():
        # domface-solo（pk-096）：相对 target 锚所绑域根，layout 缺席才回退中央根
        base = layout.root if layout is not None else resolve_root()
        tp = base / tp
```

- 同错双患的文侧已同步：server.py:253 params 教学文改「目标文档路径（所绑域根相对或绝对形；layout 缺席时回退中央根）」，缺陷件所述旧文「工作区根相对或绝对形」已不存在。
- 红测在档：sih-tools/mcpline/tests/test_nomenclator_check_anchor.py:1-29，三发——相对 target 锚域根（9-13）、无 layout 回退中央根（16-20）、教学文与实锚一致断言（23-29，含 `"工作区根相对或绝对形" not in src`）。
- 归并批次坐实：sih-tools 仓 git log，提交 eba3b02d「domface-solo 段1 domface-solo nomenclator_check 域锚回退形加所绑域根教学文加域锚测试三发（pk-096）」。

残面：无代码残面。手续残面一条：pk-096-exit.json 缺席，出泊未随 domface-solo 落档，候本批或出泊批结算补。

---

## pk-097 commit repo 形参三形坐实与机械 message 前缀重复新患

判词：**已被核销（pkfix-parallel 批，非腿二批；出泊件在档 pk-097-exit.json，disposition: promoted）**。其一与其二俱已修；腿二 Rust 面 commitlaw.rs 对其二做了行为对等承接（前缀去重同参形），对其一不涉（repo 形参病灶在 MCP 层，CLI --repo 单值形为既裁设计）。新患（message 前缀重复）零残面。

证据：

其一（repo 形参三形）：

- sih-tools/mcpline/src/mcpline/server.py:782-790，MCP 工具签名改双形并自署 pk-094 其一：

```python
@mcp.tool(name="lease_commit", description=_DESC_BETA_COMMIT)
def lease_commit(repo: str | list[str], stage: str, subject: str, ...):
    ...
    # repo 双形（pk-094 其一）：str 与列表皆合法入参形，归一在 writeface tools
```

- sih-tools/mcpline/src/mcpline/writeface/tools.py:513-525，列表归一单仓、多条教学拒，病灶注记（列表形此前被串化成字面量字符串直传 CLI 即 repo_not_in_session）在档：

```python
    repos = [repo] if isinstance(repo, str) else [str(r) for r in (repo or [])]
    if len(repos) != 1:
        return _tool_error(..., "repo 列表形须恰单仓（CLI commit --repo 单值形，一笔提交落一仓）"...)
```

- 教学文补位在档：tools.py:491-493 params 写明「工位相对路径形为成功形，点号形会撞主检出直提拒；str 与列表双形接受」——缺陷件所述三形（列表形／点号形／工位相对路径形）之后果教学俱落。
- 腿二 Rust 面对表：sih-engine/src/bin/lease/commitlaw.rs:392 `let repo_arg = one(m, "repo")` 单值形、441-470 repo/worktree 配对与 commit_must_target_worktree 拒，与 CLI 单值设计一致，无列表形患位。

其二（build_message 机械前缀重复）：

- sih-tools/lease/src/lease/commitcore.py:119-145 增 `_dedup_subject_prefix`（剥序自长及短：wip 形与段N 形先于裸 stem 形，`{stem}` 非整记号前缀防误剥），155 行 build_message 内 `subject = _dedup_subject_prefix(stem, subject)` 先剥再拼，缺陷件所引旧 126/131 行同字样双现患已消。
- 腿二 Rust 对等承接：sih-engine/src/bin/lease/commitlaw.rs:149-175 `dedup_subject_prefix` 三正则循环剥除，187 行 build_message 首行 `let subject = dedup_subject_prefix(stem, subject);`——Python 权威与 Rust 镜像同参形。
- 红测在档：sih-tools/lease/tests/test_pkfix_parallel.py:33-79，六发（settle 剥 stem 前缀、wip 剥 wip/段N 前缀、meta 行不动、洁 subject 逐字节不变、`{stem}` 整词边界不误剥）。
- 归并批次坐实：sih-tools 仓 git log，提交 f3bdfdf7「pkfix-parallel 段1 四簇落码：甲 mcpline repo 形参兼容……乙 build_message 前缀去重……」。

残面：无。

---

## pk-098 close 解绑先于锁守卫死锁三角与孤儿锁清偿无门加 retriever_recall archive 枚举不可发现

判词：**两分。其一：部分核销＋活缺陷残面，候腿三批（设计裁决位），不在本批修缮域。其二：已被核销（domface-solo 批，非腿二批）。**

### 其一（close 死锁三角与孤儿锁清偿）

已核销部分——「close 先解绑连接再撞锁守卫」的三角闭合形态已被 penface 批（pk-093 其一）拆解：

- sih-tools/mcpline/src/mcpline/writeface/tools.py:562-571，close 的连接解绑改挂在 `_finish` 的 `on_success` 回调，即 CLI 收约成功后才解绑；收约被锁守卫拒时连接仍绑定，unlock 通道不闭：

```python
    argv = lease_close_argv(conn.layout, package, reason, None)

    def _mark() -> None:
        conn.mark_client_closed()
        # 隐式占位归一（pk-093 其一）：收约成即隐式占位事实清零，连接可显式
        # 重开（open 与 close 同一绑定查找位的生命周期闭环）。
        conn.auto_open_error = None

    return _finish(conn, "lease_close", argv, LEASE_BIN_TIMEOUT,
                   what=what, params=params, on_success=_mark)
```

- tools.py:117-122，隐式占位态放行 close 透传（此前该态即 session_not_bound 拒，隐式会话成收不回孤儿）——缺陷件所引旧 tools.py 118 行的 session_not_bound 拒对 close 已豁免，现拒行移至 123-130 且只拦其余写工具。
- 结构卫三点现位：session_already_bound 在 tools.py:96-105；session_not_bound 在 tools.py:123-130；close 解绑后置在 tools.py:564-571。原三角「close 先解绑 → unlock 拒 → open 拒」的第一边已不存在。

活缺陷残面——孤儿锁清偿无门未修，属 lease 生命周期设计变更，泊件 exit_condition 明文「须先裁后修」：

- 锁清零闸仍是「先 unlock 后 close」设计：sih-engine/src/bin/lease/closegate.rs:439-460（腿二实装），会话持锁即拒收约：

```rust
    // 锁清零
    let held = lock_face_paths(&locks_ledger);
    ...
    if !sid_held.is_empty() {
        ...
        fail(1, json!({"error": format!("locks held by session, unlock first: {}", list)}));
    }
```

  Python 权威面同形：sih-tools/lease/src/lease/core.py:1799 `raise StateError(f"locks held by session, unlock first: {sid_held}")`。
- 放锁要求会话在册且五验过：sih-tools/lease/src/lease/lockcore.py:293-311，`release` 先 `select_session`（会话已吊销即 session_not_active，lockcore.py:86-98）再 `verify_five`——已吊销会话名下孤儿锁无 release 通道。
- 锁生命周期未绑定会话收口：closegate.rs 收约序（943-964 吊销行、966-1017 未用锁罚）只计罚不清锁，吊销后 locks.ndjson 残行无机械清偿。
- MCP 面强拆不透传为 DES-014 设计：tools.py:562 `lease_close_argv(conn.layout, package, reason, None)`（第四参 force 位恒 None），server 端不代行人节点强拆。
- CLI 人节点位现状：sih-tools/lease/src/lease/cli.py:865 `close_cmd.add_argument("--force", ...)` 注记为「脏工地强拆」（对应 closegate.rs:880-886 worktree remove --force），非孤儿锁清偿位；孤儿锁 CLI 强拆通道与指引入手册的修法未落。

修法（承司梦原案，候人节点裁后修，本批不动）：

1. 设计变更案：锁生命周期绑定会话收口——close 吊销行落档时同事务将会话名下未 released 锁行清偿（或落 orphan-released 注记行），closegate.rs 锁清零闸改「残余他锁才拒」；须先裁（DES-014 锁面语义变更）。
2. 通道显式化案：人节点 CLI 孤儿锁强拆位（如 `lease lock --orphan-release --session <sid>`，仅限已吊销会话名下锁行）加 AI-MANUAL 指引。

红测草案（随裁随修）：

- 锁行残留态下 close 吊销后，locks_read 六面核算该会话锁行清零（或 orphan 注记行在档）；
- 已吊销会话孤儿锁经人节点强拆位 release 成功且 released 行 session_id 保留原 sid；
- 活会话持锁 close 仍拒（unlock first 语义对活会话不变，防修法扩面误伤）。

### 其二（retriever_recall archive 枚举不可发现）

已核销。sih-engine/src/retriever/mod.rs:142-148，Blocked 载荷现附五档合法值全表（缺陷件所述「仅档名非枚举值 X 不附清单」已消）：

```rust
        let parsed = Archive::parse(name).ok_or_else(|| {
            RecallError::Blocked(format!(
                "档名非枚举值 {name}（合法值：fact、conclusion、experience、parked、intent）"
            ))
        })?;
```

五档枚举正典位：mod.rs:17-45（`enum Archive` 与 parse 全表）。归并批次坐实：sih-engine 仓 git log，提交 7d0c3f3「domface-solo 段2 domface-solo archive 报错附五档全表（pk-098 其二）加任务包与结果档」。

零粉饰注记：缺陷件修法原文为「附枚举全表**与各档语义一行**」；现行载荷附全表、未附各档语义一行，属修法的部分落形（核心不可发现患已消，语义行候随批补，不构成活缺陷判词）。

---

## pk-100 pre-commit 守卫工位锁面读数盲区

判词：**已被核销（pkfix-parallel 批，非腿二批；出泊件在档 pk-100-exit.json，disposition: promoted）**。守卫锁账本解析经 git common dir 锚定主树中央锁面，工位盲区已消；腿二 Rust 面（closegate.rs／commitlaw.rs）不涉此位（pre-commit 守卫属 Python 钩子面）。

证据：

- sih-tools/lease/src/lease/guardcore.py:12-14 模块头自署：「pkfix-parallel 批（pk-100）：锁账本解析与活跃锁投影入本模块单源……工地（worktrees/）上下文锁账本缺省解析经 git common dir 锚定主树中央锁面，修工地侧空账本盲区」。
- 锚定实现：guardcore.py:162-186 `_central_ledger_via_common_dir`——`git rev-parse --git-common-dir` 取主 .git（工位返回绝对形），主树锁面 `<主树根>/lease/ledger/locks.ndjson` 在盘即返，git 不可用返 None 如实：

```python
    common_path = Path(common)
    if not common_path.is_absolute():
        common_path = (Path(workdir).resolve() / common_path).resolve()
    candidate = common_path.parent / "lease" / "ledger" / "locks.ndjson"
    return candidate if candidate.is_file() else None
```

- 解析单源优先序：guardcore.py:189-213 `resolve_locks_ledger`——显式注入 → 根形（主树与 canonical 域照旧优先，零行为变更）→ git common dir 工位锚定 → 旧缺省形回落。
- 活跃锁投影单源：guardcore.py:216-247 `parse_active_locks`（acquired/released 按序配对）。
- 钩子装配位消费坐实：sih-tools/lease/hooks/pre-commit:24（注记「解析单源 guardcore.resolve_locks_ledger（pkfix-parallel 批 pk-100）」）、:38（导入）、:104（`locks_ledger = resolve_locks_ledger(...)` 装配调用）。
- 红测在档：sih-tools/lease/tests/test_pkfix_parallel.py:82-178，六发——工位上下文解析出中央账本（117）、根形优先不变（126）、显式 env 最先（140）、工位内中央锁行可读且执法生效（148）、released 行不入活跃投影（166）。
- 归并批次坐实：sih-tools 仓 git log，提交 f3bdfdf7 pkfix-parallel 段1 乙簇「guardcore 工位锚定」。

残面：无。

---

## 四件判词汇总表

| 泊件 | 判词 | 核销批属（实证） | 出泊件 | 修缮域归属 |
|---|---|---|---|---|
| pk-096 | 已被核销 | domface-solo 段1（eba3b02d，server.py:260-263 + 红测三发） | 缺席（parked 在档，出泊手续残） | 非本批；出泊候结算 |
| pk-097 | 已被核销（其一与其二俱修） | pkfix-parallel 段1（f3bdfdf7：甲簇 repo 双形＋乙簇前缀去重）；腿二 Rust commitlaw.rs:149-175/187 行为对等承接 | 在档（promoted） | 非本批 |
| pk-098 其一 | 部分核销＋活缺陷残面，候腿三批（设计裁决位） | 三角闭合形态已拆（penface 批 08fbb0cf，pk-093 其一，tools.py:564-571/117-122）；孤儿锁清偿无门未修（closegate.rs:439-460、lockcore.py:293-311、cli.py:865） | 缺席（parked 在档） | 候腿三批，先裁后修，本批不动 |
| pk-098 其二 | 已被核销 | domface-solo 段2（7d0c3f3，retriever/mod.rs:142-148 五档全表入载荷） | 缺席（随其一同件 parked） | 非本批；「各档语义一行」候随批补 |
| pk-100 | 已被核销 | pkfix-parallel 段1（f3bdfdf7：guardcore.py:162-213 工位 common dir 锚定 + hooks/pre-commit:104 装配 + 红测六发） | 在档（promoted） | 非本批 |

取材结论一句话：四件中三件半已在先期批（domface-solo、pkfix-parallel、penface）核销，均非腿二 lease-commitlaw-parallel 批核销（腿二仅对 pk-097 其二做 Rust 镜像对等承接）；唯一活缺陷残面是 pk-098 其一孤儿锁清偿无门，属 lease 生命周期设计变更候人节点裁决，不在本批修缮域。本批（mcpdefect-parallel）对四件的可修缮域为零，本卷即对账凭据。
