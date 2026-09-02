# 得一融回完成档 2026-09-02

本档是 DEC-013 融回门机制判定器组件（得一）的关闭凭证，三查对表逐项证据，融回门关闭。承 SPEC-014 规格批与 deyimerge-tdd-solo 实装批两步主会验收、用户 2026-09-02 切换放行令。

## 概览 {#overview}

- 三查全过即 SPEC-014 验收判据五条逐条证据、接口契约对表、回迁债清账::[三查对表](#checks)
- 全流程三批即 SDD 立制、TDD 先红后绿、切换执行::[时间线](#timeline)
- 服役与交割概览即围堰两件采样观察腿与机械核对腿分工、写入位书简管线不变、facet CLI 采样双模并存::[交割](#handover)

## 三查对表 {#checks}

第一查验收判据全过，逐条对 SPEC-014 § 验收判据 A1-A5，证据引 deyimerge-tdd-solo 结果档（sih/event/plan/deyimerge-tdd-solo-results.md）与材料件（sih/event/plan/deyimerge-tdd-solo-materials/）。

A1 金向量逐字节一致
: 四类工件金向量 64 文件冻结落 src/attractor/fixtures/golden/，净二（adisp-net 合同 39597 字节、p3xcarr-net）加脏三（挂起形、材料退回形、告警形）加合同拒收基线四形，围堰原件为唯一基准由 freeze_golden.py 驱动跑出零自造；T2 九件逐字节断言全绿。

A2 退出码对齐
: check 三值与 sign 三态与 verify 两值与 watch 两值全表对齐；活体五场景 check 同参形双跑退出码 0/0/0/1/0 全对齐，证据入材料件 live-double-run-cmp.log。

A3 des-011 判定规约随迁
: R1 至 R7 七规则与闸三态映射四值处置随迁，rules_version 承 des-011-r1 形与引擎 is_rules_version 一致；同输入引擎件与工具件处置四值与方向与通过失败清单逐字段一致，T4/T6 测试承载；多 gid 材料顺序加载处置互不串扰。

A4 机械腿不变量
: src/attractor/ 与 src/bin/attractor.rs 零网络零 LLM 零 key 读取零目标仓写入；T5 源码扫描禁词表零命中加 Cargo 依赖扫描零网络零 LLM 面，活体 score 双跑 CMP-IDENTICAL（live-double-run-score.log）。

A5 跨腿契约逐字段兼容
: assemble 从计分材料与合同逐字段拼 tally-check-input，引擎 signcheck 经既有 guard_crosscheck 零改通过；watch 对混合 signcheck 目录重放行为一致，T4 测试承载。

本批切换补证：pk-036 金向量重录按现行 GOV-003 真实内容以围堰件为基准刷期望输出 content_hash（6bb38a7f 陈旧值改 76ef97b1 现行值），断言逻辑零改，重录前后该目标双跑 cmp 围堰件与引擎件 IDENTICAL，golden_des001_gov003 转绿，pk-036 出泊。

第二查接口契约未变，facet 与 tally 两 CONTRACT 退役登记不删接口仅加注：六子命令面 emit-contract/score/check/verify/sign/watch 与 lib 面 assemble 全保留；核对报告十二字段冻结面与 crosscheck 事件十五字段负载零动；采样合同与计分材料与核对报告与 signcheck 四类 json 工件围堰件与引擎件输出逐字节（同参形条款承载，浮点文本形豁免即腿切分清单边界声明显式范畴排除——facet_stats 数值函数统计结论等值与退出码一致判据，不取字节级）；契约源 CONTRACT-MODE-SPEC v1 与 tally CONTRACT v1.3.0 现行文不改。

第三查回迁债已清账：serde_yaml 依赖随 TDD 批在 Cargo.toml 声明（回迁债预期内，零网络零 LLM 依赖）；腿切分清单二十三行零偏差即融回十一行全落 src/attractor/ 且零依赖留堰件（T4 t4_leg_split_manifest 机械钉死）、留堰十二行与 facet 与 tally 源码 git 对表零改动；判据 v3 闸（probes/maturation_gate.py）未融回如实列明属围堰采样腿边界即 SPEC-014 腿切分清单外不迁，引擎 score 子命令以显式 gate_verdict 参数承接闸上游产出，机械腿只装配不判闸。

## 时间线 {#timeline}

三批全流程：

1. **SDD 立制** deyimerge-sdd-solo 批：SPEC-014 得一融回落差规格立，家位与模块形、接口契约对表、双模并存条款、腿切分清单、验收判据 A1-A5、金向量脏目标与同参形条款、回迁债、测试计划 T1-T6。
2. **TDD 先红后绿** deyimerge-tdd-solo 批：src/attractor/ 十三件 4794 行与 src/bin/attractor.rs 六子命令与金向量 64 件与三测试件 903 行先红后绿，红态 12 errors（sih_engine::attractor 缺位）转绿态 attractor 测试 28 件全绿，活体双跑五场景 cmp 全 IDENTICAL。
3. **切换执行** deyimerge-switch-solo 批（本批）：pk-036 金向量重录转绿、判定位换旗即 BATCH-FACE 执契与 facet 测量命令段改指引擎件 target/debug/attractor、facet 与 tally 两 CONTRACT 退役标注转兼容只读（facet CLI 采样双模并存保留）、完成档与 GOV-003 v1.7 与 DEC-013 修订二与 SPEC-014 修订一随批落档、pk-036 出泊、双仓 settle 归并、链 valid。

## 交割 {#handover}

判定器组件服役与交割：围堰 facet 采样观察腿与 tally 机械核对腿自孵化期至本日，机械腿全量融回引擎件 target/debug/attractor 为正典调用形，围堰两件转兼容只读；facet CLI 采样观察能力双模并存保留即 GOV-002 判据四字面，measure 与 singleseat 与 dose_driver 三腿 CLI 原位不动，组件层零 LLM 纪律的物理承载留堰；退役的是判定终签的强制执行位不是工具生命，退役非删除。

写入位不变即 sih-engine/sih/event/trail/<日期>.ndjson 由引擎件 scribe intent 与 append 与 crosscheck 写入，attractor sign 经子进程交书简管线，本融回零新增事件类型零扩十五字段面。

封装件即 mergeback-attractor-completion-2026-09-02.md 即本档归档入 sih-engine/sih/event/mergeback/ 名随源不改。

## 边界 {#boundary}

- 本档不含规则内容的裁决，判定规约增删改走 rules_version 版本管理
- 围堰 facet 与 tally 的 Python 实现不删不改语义，工具侧退役只标注与转兼容只读
- 判据 v3 闸留堰即采样腿边界，引擎侧不判闸只装配
- 上链前必须等绿；读 findings 不只看退出码
- 锁被他在持即报不绕行

## 关联文件 {#related}

- 任务包：`sih-engine/sih/state/plan/deyimerge-switch-solo.md`
- 规约：`sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md`
- 决策：`sih-engine/doc/decision/013-mergeback-gate.md` 与 `sih-engine/doc/decision/020-deyi-component-naming.md`
- 全态：`sih-engine/doc/governance/GOV-003-fullstate-course-v1.md`
- TDD 证据：`sih-engine/sih/event/plan/deyimerge-tdd-solo-results.md` 与 `sih-engine/sih/event/plan/deyimerge-tdd-solo-materials/`
- 金向量重录证据：`sih-engine/sih/event/plan/deyimerge-switch-solo-materials/golden-rerecord.log`
- 工具件退役标注：`sih-tools/facet/CONTRACT.md` 与 `sih-tools/tally/CONTRACT.md` § 退役登记
