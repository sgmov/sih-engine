# auditfix：迭代建议报告出入全量修复批

> 令源：用户 2026-09-22 令「按照你的规划，你拉起多子代理进行全量修复。你作为主编排，不写代码，只负责分配子代理任务。所有的子代理报告必须自己核查才可相信。直至所有的问题都修复完成。所有的内容必须遵循司衡的纪律和要求」
> 输入：sih-engine/doc/assessment/2026-09-22-sih-engine-iteration-recommendations.md（初版）与同日四子代理核查出入清单（十七组出入在案）与序列复审结论（三处）
> stem 认领：auditfix，甲表三件即 zh 核查修复批、code 无承、派生 audit:new,fix:new（查册双 unknown 在案）

## 问题陈述 {#problem}

迭代建议报告初版经四子代理逐条对表核查，出入十七组：一条建议核心前提过时（REC-015 腿四接线已落地）、一处决策编号硬冲突（REC-011 拟用 DEC-009 已被占用）、一组计数系统性偏差（src/bin 实数 36 非报告 41/43）、DES 顺延跳号（实数 19 应自 020 起）、十余处行号与体积与口径偏差、两处内部矛盾。本批修全部未阻塞件：报告勘误、README 工具清单对表、零依赖代码三批、registry 申报注对表收尾。

## 关键设计 {#design}

五子批一次立约：af-01 报告勘误（正文就地修正加修订记录一笔，修复 REC-015 整节重写为收尾批与 REC-016 重写为不变式正测形）；af-02 README 工具清单与 server.rs 21 具对表（record_direct 加 lease_unclaim 以本地可信位入列）；af-03 sddgate.rs 内单测补 SDDG-1 三条（teaching embed 在场拒、缺席 fail-closed 拒、形坏 fail-closed 拒）；af-04 event_stream 三处构造性不变式改正测钉住（serde 于 String 与 BTreeMap 实际不可失败，should_panic 不可达，正测即全字段可哈希、payload 确定非空、append 后链尾在）；af-05 Cargo.toml 显式 [[example]] 三段加 registrydemo 与 tools_registry 旧申报注对表收尾（腿四已落地，加日期注记了结非删除）。施工全在租约工地副本，主树零直写；SDDG-4 测试证据落批材料 tests 子目录。

## 工作清单 {#work}

- af-01：报告勘误（af-doc-report 子代理）
- af-02：README 工具清单对表 21（af-doc-readme 子代理）
- af-03：sddgate.rs SDDG-1 内单测三条（af-code-sddg1 子代理）
- af-04：event_stream 不变式正测三条（af-code-invar 子代理）
- af-05：examples 显式声明加 registry 申报注收尾（af-code-meta 子代理）
- af-06：编排位核验、管线、认证上链、settle、放锁收约、对账对表（主会）

## 验收 {#acceptance}

报告勘误后核阅 des-001 exit 0 与检词零新增违例；cargo test --bin lease 全绿含新增三条；cargo test --lib event_stream 全绿且总数 99 加 3 即 102；cargo check --examples 过且 [[example]] 恰三段；当日链 valid；reconcile 零新增；收约 SDDG 四判据全过。

## 请求写入 {#requested-writes}

- sih-engine/doc/assessment/2026-09-22-sih-engine-iteration-recommendations.md
- sih-engine/README.md
- sih-engine/Cargo.toml
- sih-engine/src/bin/registrydemo.rs
- sih-engine/src/tools_registry.rs
- sih-engine/src/bin/lease/sddgate.rs
- sih-engine/src/event_stream/hash.rs
- sih-engine/src/event_stream/append.rs
- sih-engine/sih/event/plan/auditfix-materials/
