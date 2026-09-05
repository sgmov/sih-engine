# leaseopt-precheck-solo：开工施工面交集预检（批三，用户 2026-09-05 裁定独立实施）

> 令源：用户 2026-09-05「直接收了批3吧，未来怕忘」（裁定不搭车独立开批）；线纲 leaseopt-line-v1.md 批三节
> 范式：T6-D 单线 solo，主会亲写

## 一、设计 {#design}

open 增开工预检闸（钥匙闸旁）：本批 allow 面逐路径与全部活跃会话现势独占持锁面做交集判定——交集非空即拒开（reason open_precheck_conflict，detail 载交集与持有者清单与 wait-turn/takeover 提示）；治理共享追加面白名单（SCOPE_SHARED_SURFACE：trail、scribe/reports、identity/reports、tally/reports、meter/counts、lease/ledger）与 append 持位豁免。预检为只读探测不取锁不副作用，真锁仍由 agent 锁阶段取。判定语义新增（open 新拒因）过得一裁。SCOPE_SHARED_SURFACE 冻结登记（架构常量非数值）。

## 二、F 条件 {#falsifiable}

F-1 活跃会话独占持锁与我 allow 交集即拒开载清单；F-2 共享白名单与 append 持位豁免；F-3 无持锁放行；F-4 全族零回归；F-5 得一裁+执契。

## 三、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/
- sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/pyproject.toml
- sih-tools/lease/src/lease/__init__.py
- sih-engine/sih/state/plan/
- sih-engine/sih/event/plan/leaseopt-precheck-solo-results.md
- sih-tools/facet/contracts/leaseopt-precheck-260905/
- sih-tools/proposition/DES/leaseopt-precheck-solo/
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- sih-tools/tally/reports/
- sih-tools/meter/counts/
- sih-engine/sih/event/trail/
- sih-math/docs/leaseopt-precheck-derivation-2026-09-05.md
- sih-engine/sih/state/plan/leaseopt-line-v1.md
