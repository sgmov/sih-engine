# lease-commitlaw-parallel：租约融回腿二批——收约执法面对表实装

> 治理任务包 · 承 SPEC-024 v1.1 腿二（commitcore 与 sddgate 与 guardcore 对表）
> 承接：用户 2026-09-13 令「开工，多子代理并行加速推进」（候裁项腿二收约执法面）
> 队形：并联形 parallel——并行簇双子代理各领独立簇，主线簇亲写最复杂件，依赖簇验收串行
> 日期：2026-09-13

## 范式声明 {#paradigm}

- 形：并联 parallel（编组五形之四，编排者主线）
- 并行簇 A（子代理）：sih-tools/lease 收约执法面行为对表分析——commitcore.py 与 sddgate.py 与 guardcore.py 及 cli.py 接线逐件读源码，产出行为规格笔记（参数面与验证序与拒绝教学文案逐字与回执 json 形与台账行形与退出码三值），落 work/leg2-materials/py-parity-notes.md，只读零写仓
- 并行簇 B（子代理）：金向量捕获——fixture 域 work/leg2-fixtures/domain2 跑围堰 Python lease，捕获 commit wip 与 commit settle 与 sddgate 四判据拒绝形与 guardcore 拒绝形回执，落 work/leg2-materials/golden-capture/ 与归一注记 NORMALIZATION.md，fixture 域自持台账零触真账
- 主线簇（主线亲写）：src/bin/lease.rs 收约执法面 Rust 实装即最复杂件
- 依赖簇（串行）：集成测试红转绿与金向量对表验收，待 A 与 B 交付后主线收敛

## 请求写入 {#requested-writes}

- sih-engine/src/bin/lease.rs
- sih-engine/src/lease/fixtures/golden2/
- sih-engine/tests/lease_mergeback_t5_commit.rs
- sih-engine/tests/lease_mergeback_t6_sddgate.rs
- sih-engine/tests/lease_mergeback_t7_guard.rs
- sih-engine/doc/spec/SPEC-024-lease-mergeback-gap-v1.md
- sih-engine/sih/state/plan/lease-commitlaw-parallel.md
- sih-engine/sih/event/plan/lease-commitlaw-parallel-results.md
- sih-engine/sih/event/plan/lease-commitlaw-parallel-materials/
- sih-engine/sih/event/trail/（append 面）

## 必读 {#must-read}

- sih-engine/doc/spec/SPEC-024-lease-mergeback-gap-v1.md（腿二切分与验收判据 A1-A9）
- sih-tools/lease/src/lease/commitcore.py 与 sddgate.py 与 guardcore.py 与 cli.py（融回基准权威）
- sih-tools/BATCH-FACE.md（批机械链命令面与坑位）
- 检索先例切面 /tmp/leg2-recall-topic.ndjson（随批入档 materials）

## 验收判据 {#acceptance}

- F-1 金向量逐字节：commit wip 与 commit settle 与 sddgate 拒绝形与 guard 拒绝形回执，Rust 件对捕获材料同参形归一后逐字节一致
- F-2 退出码三值对表：0 成功、1 拒、2 用法或环境错
- F-3 拒绝教学文案逐字节：SDDG 四判据拒绝串与 closeguard 拒绝串与 commit 四验拒绝串对围堰原文
- F-4 围堰零触碰：sih-tools/lease 源码零改动红线，捕获只跑不改
- F-5 主树验收：cargo test 存量回归加新增三件全绿，主树裸跑为准
- F-6 落差回写：对表中发现的围堰与 SPEC-024 落差逐条回写 SPEC-024 修订记录

## 明确不做 {#not-do}

- 腿三附件面（sweepcore 与 calllog_import 与 hooks）
- SPEC-024 session_id 归一项修订（独立候批）
- 围堰 lease 与 selector 源码任何改动
- 文档 T6 管线域外目标不做核阅（state/plan 与 event/plan 域外 exit-2 如实记）
