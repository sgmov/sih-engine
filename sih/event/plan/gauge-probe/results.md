# gauge-probe 批结果档：sihmcp 写面与管理台全量实测探针

## 偏差

- 零偏差（本批零实装零规格物变更，全部写痕为测试数据；回撤候人节点签署挂 pk-102）

## 实测记录

- stdio 正面全链：lease_open 加 lease_lock 加 record_intent 加 record_append 加 lease_commit wip 与 settle 加 lease_unlock 加 lease_close，会话 1c0a110366de66e4
- 负面教学：session_not_bound 与 session_already_bound 拒形取证在案
- 测试痕回撤位：pk-102 泊位承载，链笔与台账行与归并件清单候人节点签署

## 链笔实录（2026-09-12）

- intent_refined dff1fddf（record_intent 经 stdio MCP，session 1c0a110366de66e4）
- certification_completed 1721161a（record_append 经 stdio MCP，报告 work/gauge-probe-usage-report.json）
- wip 提交 513500f（lease_commit stage wip，pre-commit 守卫锁面教学拒一次后内容锁三把取锁通过）

## 验收对表

- 验收材料：本档即对表判词档（results），链 verify 待收约后回验 valid
