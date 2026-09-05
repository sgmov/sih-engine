# leaseopt-ledger-repair-solo：会话台账补录通道与 docmath-b4 四行修复

> 令源：数学侧 agent 2026-09-05 事故报告（用户转呈）；主会裁决第三路径即实装正式补录命令程序化导入（基线一：台账不由 LLM 手写）
> 事故：docmath-b4 与 fixguard 两批的会话台账四行遭并行批收约活写覆盖丢失（pk-047/facepark 先例同根），恢复载荷已由数学侧逐字节取自版控史（/tmp/docmath-b4-ledger-repair/lost-lines-verbatim.ndjson）

## 一、设计 {#design}

新子命令 `lease ledger-repair --input <载荷.ndjson> --reason <事由> [--at]`：确定性导入通道——逐行校验 JSON 合法性与事件形态（issued/revoked）、与现台账逐字节去重（已有即跳过并计数）、每行注入 repair 标记字段（{"repair": {"reason":…, "repaired_at":…, "source": "verbatim-from-vcs"}}，显式补录非伪装原笔，时间戳不可恢复者在标记中声明）、原子追加（复用 append_event flock）。跑后自检：status 对导入会话认账。幂等可重放。CONTRACT 1.23.0 修订三十五。

## 二、F 条件 {#falsifiable}

F-1 四行导入后 status 对 75c5a058 与 eaf80aa8 认账；F-2 重复导入幂等（跳过计数）；F-3 非法行拒收；F-4 全族零回归；F-5 得一裁+执契。

## 三、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/
- sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/pyproject.toml
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/ledger/
- sih-engine/sih/state/plan/
- sih-engine/sih/event/plan/leaseopt-ledger-repair-solo-results.md
- sih-tools/facet/contracts/leaseopt-ledger-repair-260905/
- sih-tools/proposition/DES/leaseopt-ledger-repair-solo/
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/tally/reports/
- sih-tools/meter/counts/
- sih-engine/sih/event/trail/
- sih-math/docs/leaseopt-ledger-repair-derivation-2026-09-05.md
