# mergeback-tdd-t6d 结果档

## 概览 {#overview}

- TDD 六组先红后绿即红态六败留痕、绿态七测全过加全套绿，红转绿记录在档::[判据核对](#criteria)
- 三入口落 src/event_stream 即 certify 与 intent 与 park，scribegate 五子命令退出码三值，向量集五类边界冻结::[链上证据](#evidence)
- 生产 trail 四文件在引擎侧全量复验 valid 即跨实现一致性的事实证明::[链上证据](#evidence)

## 链上证据 {#evidence}

意图入链
: hash 4862b32d 即 sess-zcode-260827-mbtdd，双腿绿一次过三锚点

会话开工
: 4115a431b52e8bac 即双仓副本 worktrees 下 mergeback-tdd-t6d，八锁，包档登记 wip 即 5b60764

先红
: 测试六组先行即 src/event_stream/tdd_tests.rs 承 cfg(test) 模块，桩模块三件返 NotImplemented，cargo test 红态即 mergeback_tdd 六测全败，红因即向量集未冻结、三入口未建、scribegate 未构建

后绿
: certify.rs 即八项负载与 sha256 报告哈希与 doc_id 取报告 stem、intent.rs 即双件消费零发现放行有发现拒加负载十二项、park.rs 即配对不变量链序重放两道门、scribegate.rs 即五子命令加 vectors 冻结、fixtures/golden/event-stream-vectors.json 即五向量，绿态即六组七测入库内模块全套四十三测绿零警告

伴生扩展
: EventInput 增可选 verification_result 字段承 SPEC-006 报告消费透传，append 透传入 Event，既有测试随补，属只扩展不重构

生产复验
: T2 即 sih-tools/scribe/trail 四文件经 load_events 加 verify 全量 valid，事件由工具侧按引擎哈希公式所写即跨实现一致性的复演证明

管线三证
: 化格核阅检词对本档，认证事件见 trail 即 2026-08-27 链

段结算提交
: 双仓经 lease commit 正身路径，提交号见 trail 与 git log，收约归并号随收约落

## 判据核对 {#criteria}

L1 过即红态先行留痕即六败在档。L2 过即向量集五类边界冻结复算一致。L3 过即生产四文件全量复验 valid。L4 过即三入口行为合 SPEC-006 即 T3 与 T4 与 T5 全过。L5 过即 T6 退出码三值即零与一与二皆有实证。L6 过即管线绿认证上链链 valid。L7 过即双仓段结算经正身路径、收约归并成、零在役锁。

## 偏离登记 {#deviations}

偏离一：向量集生成用 scribegate vectors 子命令即实现自身冻结，防漂移性质即未来任何哈希改动对此五向量破裂，跨实现性由 T2 生产复验承担。

偏离二：EventInput 扩字段承 SPEC-006 透传需求，未动既有四验与哈希公式。
