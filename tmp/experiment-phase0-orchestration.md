# 阶段零编排说明

本说明不是任务包，是给编排者即人类的操作指引。本说明承接 SPEC-001 § 阶段零，定义 6 份锚定小集文档的隔离 session 采样执行编排。

本说明在 v2 版做了根本性修订。v1 采用单上下文连续采样，诊断实证证明该模式产出虚假的 100% 稳定，因自我一致性偏差。v2 改为隔离 session 采样，每个 session 只跑 1 个采样序号，session 间零状态共享。

## 概览 {#overview}

- 6 份文档 3 对对照，每对含 1 份失败标记文档与 1 份对照文档::[锚定小集](#anchor-set)
- 每份文档用 10 个隔离 session 各跑 1 个采样序号，6 份共 60 个 session::[执行方式](#execution)
- 每个 session 的 prompt 由任务包全文加 1 行路径与序号指定构成::[session prompt 构成](#prompt-assembly)
- 全部产出后按文档聚合 10 个 session 的判断，对每对对照做统计比较::[阶段零判定](#phase0-judgment)
- session 意外中断时按采样序号重启，不靠 session 状态::[续接扫描与重启](#resumption-scan)

## 锚定小集 {#anchor-set}

6 份文档，3 对对照。每对含 1 份失败标记文档与 1 份对照文档。

第一对 design 类型。失败标记文档 design/DES-058-multi-agent-collaboration-framework.md，555 行，标记 FM-01 至 FM-06，失败类型为范畴归属层。对照文档 design/DES-017-brain-borrowing-mechanism.md，407 行，未标记。

第二对 decision 类型。失败标记文档 decision/DEC-008-know-sediment.md，55 行，标记 FM-12，失败类型为结构性盲区。对照文档 decision/DEC-004-review-progression-plan.md，87 行，未标记。

第三对 draft 类型。失败标记文档 draft/sprint-plan-setsp-reuse-2026-07-21.md，141 行，标记 FM-11，失败类型为范畴归属层。对照文档 draft/INTENT-DRIFT-INVENTORY-V2.md，144 行，未标记。

## 执行方式 {#execution}

每份文档用 10 个隔离 session 执行任务包，每个 session 跑 1 个采样序号即 seq1 到 seq10。6 份文档共 60 个 session。

隔离是采样独立性的工程保证。每个 session 是独立实例，上下文里只有任务包与文档，没有其他 session 的产出。编排者启动 session 时确保不传入其他 session 的判断记录。

60 个 session 可以并行启动。并行不影响实验有效性，因为每个 session 的判断是独立的。并行受限于编排工具的并发能力，可分批启动。

每个 session 接收任务包全文作为 prompt，加上 1 行文档路径与采样序号指定。session 产出 1 个 jsonl 文件，含本次采样序号下 4 项检查的判断记录。

输出路径按文档分子目录：

- sih-engine/sih/event/experiment/phase0-v2/DES-058/ 下 DES-058-seq1.jsonl 到 DES-058-seq10.jsonl
- sih-engine/sih/event/experiment/phase0-v2/DES-017/ 下 DES-017-seq1.jsonl 到 DES-017-seq10.jsonl
- sih-engine/sih/event/experiment/phase0-v2/DEC-008/ 下 DEC-008-seq1.jsonl 到 DEC-008-seq10.jsonl
- sih-engine/sih/event/experiment/phase0-v2/DEC-004/ 下 DEC-004-seq1.jsonl 到 DEC-004-seq10.jsonl
- sih-engine/sih/event/experiment/phase0-v2/sprint-plan/ 下 sprint-plan-seq1.jsonl 到 sprint-plan-seq10.jsonl
- sih-engine/sih/event/experiment/phase0-v2/INTENT-DRIFT/ 下 INTENT-DRIFT-seq1.jsonl 到 INTENT-DRIFT-seq10.jsonl

## session prompt 构成 {#prompt-assembly}

每个 session 的 prompt 由两部分组成。第一部分是任务包全文即 experiment-phase0-task-package.md。第二部分是 1 行文档路径、输出路径与采样序号指定，格式如下。

你负责检查的文档路径是 /Users/moc/workspaces/SiHankor/sihankor/doc/某文档.md。你的判断记录输出到 /Users/moc/workspaces/SiHankor/sih-engine/sih/event/experiment/phase0-v2/某文档/某文档-seqN.jsonl。你的采样序号是 N，全部记录的 sample_seq 填 N。

## 续接扫描与重启 {#resumption-scan}

session 意外中断后，按本节扫描并重启，不靠 session 状态。

扫描由编排者执行，是确定性操作。扫描检查每个文档子目录下 seq1 到 seq10 的文件是否存在。文件存在且非空视为该序号完成。文件缺失或空视为该序号需重启。

重启 session 的 prompt 与常规 session 相同，任务包全文加路径与序号指定。重启的 session 与原 session 完全隔离，产出独立的 seqN.jsonl 文件。

扫描结果为某文档 10 个序号全齐则该文档完成。扫描结果为有缺失则重启缺失序号的 session，补齐后再扫一次，直到全齐。

## 阶段零判定 {#phase0-judgment}

6 份文档全部产出后，按文档聚合 10 个 session 的判断，对每对对照做统计比较。

聚合由编排者用脚本执行，按 check_item 与 target 对齐 10 个 session 的判断，计算每个对象的跨 session 一致率。聚合脚本的可用实现在会话中提供，不写入驻编说明正文。

比较维度是判断分布与跨 session 一致率的差异。失败标记文档与对照文档在四个检查项上的跨 session 一致率须有统计差异。

三种结果承接 SPEC-001。全部三对有差异则 prompt 模板通过，进入阶段一。部分有差异则调整对应检查项的 prompt 模板后重跑。全部无差异则 prompt 模板区分力不足，须重新设计。

## 目录准备 {#dir-prep}

阶段零 v2 执行前须创建输出目录。路径 sih-engine/sih/event/experiment/phase0-v2/ 下 6 个文档子目录。
