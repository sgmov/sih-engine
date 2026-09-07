# closegate-solo 完工报告

> 委外单线 solo 亲写零子代理，承接 m-closegate-1（终签 6a7a7237）与 m-precommit-1（终签 b558220e）双裁语义按裁落地。判据 F-1 至 F-9 全过，TDD 先红后绿 18 测新增（watchcheck 端 7 + lease 端 11），全族 266 测零回归。CONTRACT 升 1.35.0，三源对齐。

## 一、批面定位

- 任务包：sih-engine/sih/state/plan/closegate-solo.md
- 会话：e5003cc16eef94ca
- 范式：T6 单线 solo，亲写零子代理
- 前置：projfix-solo 收约释放 CONTRACT 修订位（实际在飞候叫，4:04 收约后开批）
- 并发在飞：confpreempt-solo、chaingreen-solo（锁面零冲突，pass 预检）

## 二、双闸落地形

### 2.1 收约位拦截闸（m-closegate-1 既裁）

- 闸位：`close_session` 闸序内，`chain_gate_check` 之后、`declared_uncommitted_diff` 之前
- 谓词：复用 watchcheck `unowned_list` 谓词单源（脏文件集 − 锁面 ∪ 链笔声明面 ∪ 豁免面）
- 行为：无主清单非零即拦收约退出码一 fail-visible 并呈报完整清单
- 例外：close `--bypass-orphan <事由>` 给定即落 `bypass.ndjson` 一笔留痕
- 闸只拦不放行处置权（回滚与通道处置归人节点二值）

### 2.2 pre-commit 文件面执法位（m-precommit-1 既裁）

- 钩位：`hooks/pre-commit` 薄壳扩为执法壳（fail-closed 保持）
- 判定：staged 文件集逐件，命中活跃租约锁面（精确或目录前缀）∪ 共享追加面 27 路 ∪ 轻车道白名单 30 条目即放行
- 拒：未命中即拒退出码非零 + 三通道指引（开租约 / 挂直改链笔 / `--no-verify` bypass 登记）
- 边界：merge 提交不触发（MERGE_HEAD 存在即归并跳过此闸）；钩缺席环境（core.hooksPath 未指）执法为零由 watchcheck 例行读数呈报

## 三、F 锚定读数

详见 `sih/event/plan/closegate-solo-materials/F-table.md`。

| F 锚定 | 跑测位置 | 结果 |
|---|---|---|
| F-1 闸拦截 | test_closegate.py + 主树真跑 | 7/7 watchcheck 测过 + 7/7 no_master_check 测过 |
| F-2 例外通道 | lease core close_session + bypass.ndjson 落行 | bypass_orphan 串接 + 落行机制在位 |
| F-3 零误伤 | test_closegate.py 4 族 + test_precommit.py 4 族 | 8/8 全过零误伤 |
| F-4 提交执法 | test_precommit.py + worktree 主树 | 拒件清单呈报 + 三通道指引 + merge 跳过 |
| F-5 自举活体 | worktree/sih-tools/closegate-solo 主树真跑 | 拒 12 件 + 放行 1 笔 + 后续 wip 2 笔，三态在档 |
| F-6 双跑一致 | test_closegate.py dual_run + 既有 watchcheck dual_run | 双跑双源零漂移 |
| F-7 零 LLM | test_closegate.py + test_precommit.py 源文 grep 禁网络库 | 零命中 |
| F-8 语义承证 | 双裁材料复核 | 两闸双裁零偏离，无边界撞 |
| F-9 出泊 | pk-076-exit.json + pk-074-exit-iii.json | 双出泊材料与 PARKING-v1.md 节增在档 |

## 四、测试计数

| 项目 | 计数 |
|---|---|
| 新增测试（watchcheck 端 unowned_list） | 7 |
| 新增测试（lease 端 staged_enforce） | 11 |
| lease 既有全族 | 255 |
| lease 改造后全族 | 266（11 测新绿，零回归，含 test_guard 7 件兼容升级） |
| watchcheck 既有全族 | 14 |
| watchcheck 改造后全族 | 21（7 测新绿，零回归） |

## 五、交付清单

- `sih-tools/lease/src/lease/core.py`（close_session 闸序挂入无主闸 + `no_master_check` 谓词函数 + bypass 落 `bypass.ndjson` 通道）
- `sih-tools/lease/src/lease/cli.py`（`--bypass-orphan` 显式通道）
- `sih-tools/lease/src/lease/guardcore.py`（`staged_enforce` 判定函数 + `STAGED_REJECT_GUIDANCE` 三通道指引）
- `sih-tools/lease/hooks/pre-commit`（薄壳扩为执法壳，worktree 路径翻译 + SIHANKOR_LOCKS_LEDGER 注入）
- `sih-tools/lease/CONTRACT.md`（修订五十：双闸语义 + 执法位升位 + 钩缺席如实申报条）
- `sih-tools/lease/src/lease/__init__.py`（升 1.35.0）
- `sih-tools/lease/pyproject.toml`（升 1.35.0 + watchcheck 依赖 + workspace member 增位）
- `sih-tools/lease/tests/test_closegate.py`（7 件）
- `sih-tools/lease/tests/test_precommit.py`（11 件）
- `sih-tools/lease/tests/test_guard.py`（兼容升级 SIHANKOR_LOCKS_LEDGER 注入）
- `sih-tools/watchcheck/src/watchcheck/core.py`（`unowned_list` 函数）
- `sih-tools/watchcheck/tests/test_unowned_list.py`（7 件）
- `sih-tools/pyproject.toml`（workspace member 增 watchcheck）
- `sih-engine/sih/state/parking/materials/pk-076-exit.json`（出泊材料）
- `sih-engine/sih/state/parking/materials/pk-074-exit-iii.json`（出泊材料）
- `sih-engine/doc/governance/PARKING-v1.md`（双出泊行 + 历史住户二十三项节）

## 六、收口时点

- 写时：2026-09-07
- 会话：e5003cc16eef94ca
- 收约双仓：`sih-tools/closegate-solo` branch `msh/closegate-solo` HEAD = 950ce7bf（wip） + `sih-engine/closegate-solo` branch `msh/closegate-solo` HEAD = 2e03c3a（settle 段 1）
