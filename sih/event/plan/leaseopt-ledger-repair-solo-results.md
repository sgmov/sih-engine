# leaseopt-ledger-repair-solo 结果档

> 批：leaseopt-ledger-repair-solo（会话台账补录通道与 docmath-b4 四行修复）
> 会话：8c1c470ad154743f。日期：2026-09-05。单线形 solo，主会亲写。
> 令源：数学侧 agent 事故报告（用户转呈），主会裁决第三路径即实装正式补录命令程序化导入。

## 一句话结论

补录通道完成：ledger-repair 子命令（校验/去重/repair 标记/原子追加四位一体）实装，106 件测试全绿；得一裁 near_threshold 挂起待人复核（第五次同形态），复核通过后即执行 docmath-b4 四行修复载荷导入与三重核验，批保持开位主树零台账改动。

## 一、F 表

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 导入认账 | status 对补录会话认账 | 待执行 | 复核通过后对真实载荷执行，读数入收口附记 |
| F-2 幂等 | 重放全跳过 | 过 | test_ledger_repair_import_and_idempotent：二跑 repaired 0 skipped 2 |
| F-3 拒收 | 非法行拒 | 过 | 同测试：broken json 拒收清单在案 |
| F-4 回归 | 全族零回归 | 过 | 106 passed |
| F-5 得一裁 | near_threshold 挂起 | 挂起 | m-ledgerrepair-switch-1 九发全 comply 稳、边界旗零、依据族基线四×6 基线一×3 |

## 二、事故与修复事实（如实申报）

- 事故：docmath-b4（75c5a058）与 fixguard（069a640d 全两行、eaf80aa8 issued 行）共四行会话台账遭并行批收约活写覆盖丢失，pk-047 facepark 先例同根；恢复载荷由数学侧 agent 逐字节取自版控史（/tmp/docmath-b4-ledger-repair/lost-lines-verbatim.ndjson，四行主会亲核与报告一致）。
- 修复：ledger-repair 子命令——repair 标记显式（补笔非伪装原笔）、时间戳不可恢复者在标记声明、去重幂等、拒收清单如实。
- 根因（收约机械重写台账竞态）不在本批：建议立泊件（台账活写覆盖根因硬化——收约归并对台账面的并集超集通道），随批六或独立批。

## 三、判据观察申报（不改变判据，呈人节点知悉）

连续五件机制/修复类命题同落 near_threshold，成因恒为依据族两值分散（基线四与基线一）——此类命题的依据面天然双源，basis_consensus 子判据对其结构性偏严。是否为机制/修复类命题增设判据通道（或人节点确认通道常设化），呈人节点裁，本批不动判据。

## 四、待复核后收口路径

confirmation 落链 → 执契 → 真实载荷导入 + status/reconcile/verify 三重核验 → 三仓 settle → close → 收口附记。人裁退回则改写链重立。

## 六、收口附记（close 后回填）

- 人节点复核：用户 2026-09-05 会话令「同意」落链（confirmation a63fdcc5dcb8a296）并附两点裁决——判据观察归 facet 融回（pk-056 入泊承载）、异常与建议入泊（pk-055 活写覆盖根因硬化入泊承载）。
- 真实导入读数：首次 repaired 4 skipped 0 rejected 0；幂等重放 repaired 0 skipped 4；其中 069a640d issued 行去重豁免（本就在账，数学侧报告计为丢失属多计，去重机制正确处理如实申报）——真实修复三行即 eaf80aa8 issued、75c5a058 issued、069a640d revoked，全带 repair verbatim 标记。
- 三重核验：reconcile 三仓 unrouted 全零（session_orphan tools 21→19、engine 23、math 10 为历史累积类），链 verify valid 19 事件，主树亲跑全族 106 件绿，1.23.0 三源对齐。
- 流程事故申报：任务包 allow 行折叠（多路径挤一行）致 allow 解析三折叠行全不匹配，两轮重开（force close 与 open 静默失败各一次后裸跑定位），settle 形变为 wip 直提加 bypass，全程留痕在案。
