# mcpserv-solo TDD 红绿时间线（先红留痕纪律：首跑红证全量归档，重跑不覆盖首跑记录）

> 承 gvecmath 先红留痕纪律（BATCH-FACE 2026-09-06 增）：任何重测之前的首跑红证必须归档，禁止删除或清洗后重跑。
> 本件为五跑时间线转录，红证关键报文逐条如实转录自会话实跑输出；最终全绿日志见同目录 pytest-final-green.log。

## 第 1 跑（红，RC=2）：mcp 2.x 移除 FastMCP

- 命令：`uv run --project . python -m pytest tests/ -v`（依赖 `mcp>=1.2.0` 无上界，uv 拉到 mcp 2.x）
- 红证：`ModuleNotFoundError: No module named 'mcp.server.fastmcp'. This is mcp 2.x, where FastMCP was renamed to MCPServer`，`Interrupted: 2 errors during collection`（tests/test_tools_unit.py 与 tests/test_zero_write.py 俱导入失败）
- 修复：任务包第四节钉「官方 mcp SDK（FastMCP）」即 v1 线 API，pyproject 依赖钉 `mcp>=1.2.0,<2`

## 第 2 跑（红，RC=1，12 绿 2 红）：VIRTUAL_ENV 继承污染

- 红证：`test_locks_read_happy` 与 stdio 冒烟内 locks_read 俱报 `lease status 退出码 2: warning: \`VIRTUAL_ENV=.venv\` does not match the project environment path \`/Users/moc/workspaces/SiHankor/sih-tools/.venv\` and will be ignored`
- 修复：runtime `_clean_env` 剔除位由 PYTHONHOME 与 PYTHONPATH 扩为含 VIRTUAL_ENV

## 第 3 跑（红，RC=1，12 绿 2 红）：lease 工地自检卫（报文在 stdout，stderr 空）

- 红证：locks_read 报 `lease status 退出码 2`（stderr 空）；探针复现 stdout 全报文：`{"action": "传三参即可放行", "error": "self_boot_rejected", "reason": "工位（worktrees/ 下）跑 CLI 必须三参显式全传：--ledger + --locks + --bills", ...}`
- 判定：rootanchor-solo 勘误位（BATCH-FACE 工地自举硬拒），查册是只读形非工地自举写账；主树部署实态 cwd 非工位
- 修复：locks_read 子进程 cwd 定工作区根（非工位），承接参数形零改动

## 第 4 跑（红，RC=1，13 绿 1 红）：bogus-root 下 cwd 缺席 OSError 未收编

- 红证：`test_locks_read_error_bogus_root` 报 `FileNotFoundError: [Errno 2] No such file or directory: '/private/tmp/mcpline-nonexistent-root'`（subprocess cwd 不存在即抛 OSError 而非返 rc）
- 修复：runtime `run_readonly` 收编 OSError 为 rc=127 形，错误载荷四字段语义不变

## 第 5 跑（绿，RC=0，14 绿）

- `14 passed in 2.18s`：stdio 冒烟 1 + 单元 11 + 零写入双证 2，全绿日志 pytest-final-green.log
