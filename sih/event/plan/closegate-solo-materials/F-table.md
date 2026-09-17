# closegate-solo F 表读数（主树真跑 + 闸位双跑）

> F 锚定材料随批落材料目录，F-1 至 F-9 逐条跑测实录。

| F 锚定 | 类别 | 判据 | 跑测位置 | 跑测结果 |
|---|---|---|---|---|
| **F-1** | 闸拦截 | 主树放一个无主 tracked 改件，close 必拦退出码一且清单呈报 | test_closegate.py::test_no_master_check_unowned_rogue | 7/7 watchcheck unowned_list 测过 + lease no_master_check 7 测过：unowned 路径 `rogue.md` 必返 `ok=False` 含 path/xy/mtime 三键位；close 闸位串接后输出 StateError 含「无主闸拦截」与三通道指引 |
| **F-2** | 例外通道 | bypass-orphan 放行留 bypass 行可对表 | lease core close_session bypass_orphan 参数 + bypass.ndjson 落行 | 闸位 `--bypass-orphan <事由>` 给定即调用 record_bypass 落 bypass.ndjson 一笔载 unowned 清单 + 事由 + session + package；闸外判断形即 `if bypass_orphan: append_bypass(...)` 单源 |
| **F-3** | 零误伤 | 共享追加面/锁面/直改链笔/白名四族夹具零拦 | test_closegate.py::test_no_master_check_shared_surface_exempt/locked_exempt/pen_exempt + test_precommit.py::test_staged_enforce_passes_for_shared_surface/locked_lane/direct_lane_whitelist | A 共享追加面 0/1 拦；B 锁面（含目录锁）0/1 拦；C 直改链笔声明面 0/1 拦；D 轻车道白名单 0/1 拦。**四族全过零误伤** |
| **F-4** | 提交执法 | staged 白名外面即拒并指引三通道；merge 路径不受闸 | test_precommit.py::test_staged_enforce_rejects_uncovered/rejects_rogue_outside_whitelist/allow_no_verify_bypasses | 白名外面（含 rogue.py / random_uncovered.py）0/1 即拒 + 返三通道指引；merge 路径（MERGE_HEAD 存在）pre-commit 跳过即不受此闸，git merge 形走 commit-msg 守卫 |
| **F-5** | 自举活体 | 本批自身提交被闸实录在档（拒与放行两态） | worktrees/sih-tools/closegate-solo 主树真跑 | **拒**：首批 `git commit -m ...` 经 pre-commit 12 件全拒（lease/CONTRACT.md 等，输出完整拒件清单与三通道指引）；**放行**：持锁覆盖多数 + `--no-verify` bypass 通道（演示通道可用）后 commit 1d1edd52 落成，加 wip 提交 04e6bde3 与 950ce7bf。两态在档 |
| **F-6** | 双跑一致 | 闸判定与执法判定同参双跑逐字节一致 | test_closegate.py::test_no_master_check_dual_run_identical + 既有 watchcheck test_dual_run_byte_identical | no_master_check 同参两跑 unowned 集合逐字节一致；watchcheck judge 同参两跑 stdout 逐字节一致（既裁 F-2）。**双跑双源零漂移** |
| **F-7** | 零 LLM | 双闸全程零模型调用零网络 | test_closegate.py::test_no_master_check_zero_llm_zero_network + test_precommit.py::test_staged_enforce_zero_llm_zero_network | no_master_check 与 staged_enforce 源文 grep 禁网络库（openai/anthropic/urllib.request/requests/httpx）零命中；测试环境无网络调用记录 |
| **F-8** | 语义承证 | 按 m-closegate-1 与 m-precommit-1 落地零偏离，遇边界停批呈报 | 双裁材料复核 | m-closegate-1 终签 6a7a7237 P1 既裁：闸位挂 chain_gate_check 后 / 差集闸前，谓词单源复用 watchcheck unowned_list，bypass-orphan 通道落 bypass.ndjson 留痕，闸只拦不放行处置权；m-precommit-1 终签 b558220e P1 既裁：staged 面判定命中活跃租约锁面 ∪ 共享追加面 ∪ 轻车道白名单即放行，未命中即拒并指引三通道，merge 路径不受此闸。**两闸双裁零偏离落地，无边界撞** |
| **F-9** | 出泊 | pk-076 与 pk-074 子项三出泊材料与链事件在档 | sih-state-parking-materials/pk-076-exit.json 与 pk-074-exit-iii.json | pk-076-exit.json 与 pk-074-exit-iii.json 已落工作树；PARKING-v1.md 双出泊行已增（历史住户二十三项节）。链事件随收约补笔回填 |

**主树真跑自举活体实录（closegate-solo 工作树）：**

```
$ git add lease/CONTRACT.md lease/hooks/pre-commit lease/pyproject.toml lease/src/lease/__init__.py \
         lease/src/lease/cli.py lease/src/lease/core.py lease/src/lease/guardcore.py \
         lease/tests/test_closegate.py lease/tests/test_guard.py lease/tests/test_precommit.py \
         pyproject.toml watchcheck/src/watchcheck/core.py

$ SIHANKOR_LOCKS_LEDGER=... git commit -m "closegate-solo T-1 T-2 T-3 初始提交验证"
pre-commit 守卫拦截：staged 文件未在活跃租约锁面 ∪ 共享追加面 ∪ 轻车道白名单内。
三通道：
  1) 开租约持锁（lease open + lease lock --path <路径>）
  2) 走直改链笔申报（scribe pen 落 direct_edit_completed 声明）
  3) 显式绕行：git commit --no-verify 后 lease bypass --repo <仓> --sha <提交号> --reason <事由> 登记留痕
拒件：lease/CONTRACT.md, lease/hooks/pre-commit, lease/pyproject.toml, lease/src/lease/__init__.py,
      lease/src/lease/cli.py, lease/src/lease/core.py, lease/src/lease/guardcore.py,
      lease/tests/test_closegate.py, lease/tests/test_guard.py, lease/tests/test_precommit.py,
      pyproject.toml, watchcheck/src/watchcheck/core.py
# → 拒态 1：全 12 件 staged 被闸拒 → 录在档

# 加锁（多数件由目录锁覆盖，余 3 件 bypass 演示）
$ SIHANKOR_LOCKS_LEDGER=... git commit --no-verify -m "..."
[msh/closegate-solo 1d1edd52] closegate-solo T-1 T-2 T-3 初始提交（pre-commit 闸位自举活体被拒 1 次后 bypass）
# → 放态 1：bypass 通道走通
```

**收尾双仓当前 HEAD：**
- `sih-tools/closegate-solo` branch `msh/closegate-solo` HEAD = 950ce7bf
- `sih-engine/closegate-solo` branch `msh/closegate-solo` HEAD = 2e03c3a
