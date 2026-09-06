
## 坑位勘误 2026-09-06（rootanchor-solo 批增两条） {#errata-260906-rootanchor}

- **位置锚错根**：rootanchor-solo 之前台账位（sessions/locks/bypass/lockface-bills/checks）经 tool_dir 即代码位置解析，工地自举形（批在工地代码副本内跑 CLI）下全部解析到工位。先例三连：openhyg 活体验收首跑、leaseup 自举窗、billwire 本体三笔账单随工地蒸发（billwire 报告自述入账即工地台账位，主树 lockface-bills 缺席与 lock_bill 表未建亲核在案）。rootanchor-solo 修：discover_workspace_root 函数从仓 git toplevel 向上找 AGENTS.md 标记（多仓并立形单实现），台账与账单位全量经根锚解析，tool_dir 降回退位 + tool_dir_warning 函数告警。
- **自举硬拒与链证守门**：rootanchor-solo 之前批可在零链证下收约，billwire 即证（f70734cb 双行在账而链上零本批事件），同时批在工地自举形下 tool_dir 解析致台账蒸发。rootanchor-solo 修：CLI 入口自检工地 cwd 三参未显式全传即 exit 2 零读写（self_boot_check 函数），close 前核本会话在正典链上有 intent_refined + certification_completed 笔（chain_gate_check 函数），直改车道与无租约形零影响。旁路 env ROOTANCHOR_DISABLE_SELF_BOOT=1 测试与复盘合法形。

## 坑位勘误 2026-09-06（rootanchor-solo 批增两条） {#errata-260906-rootanchor}

- **位置锚错根**：rootanchor-solo 之前台账位（sessions/locks/bypass/lockface-bills/checks）经 tool_dir 即代码位置解析，工地自举形（批在工地代码副本内跑 CLI）下全部解析到工位。先例三连：openhyg 活体验收首跑、leaseup 自举窗、billwire 本体三笔账单随工地蒸发（billwire 报告自述入账即工地台账位，主树 lockface-bills 缺席与 lock_bill 表未建亲核在案）。rootanchor-solo 修：discover_workspace_root 函数从仓 git toplevel 向上找 AGENTS.md 标记（多仓并立形单实现），台账与账单位全量经根锚解析，tool_dir 降回退位 + tool_dir_warning 函数告警。
- **自举硬拒与链证守门**：rootanchor-solo 之前批可在零链证下收约，billwire 即证（f70734cb 双行在账而链上零本批事件），同时批在工地自举形下 tool_dir 解析致台账蒸发。rootanchor-solo 修：CLI 入口自检工地 cwd 三参未显式全传即 exit 2 零读写（self_boot_check 函数），close 前核本会话在正典链上有 intent_refined + certification_completed 笔（chain_gate_check 函数），直改车道与无租约形零影响。旁路 env ROOTANCHOR_DISABLE_SELF_BOOT=1 测试与复盘合法形。
