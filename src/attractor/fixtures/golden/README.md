# 金向量冻结说明（deyimerge-tdd-solo）

本目录是 SPEC-014 A1 金向量：期望输出全部由围堰 Python 原件跑出冻结
（驱动件：sih/event/plan/deyimerge-tdd-solo-materials/freeze_golden.py），
围堰输出是唯一基准，禁自造期望。冻结于 2026-09-02，冻结后零漂移由
T2 哈希对表测试钉死。

## 目录构成

| 目录 | 类别 | 内容 |
|---|---|---|
| adisp-net/ | 净目标一 | adisp-guard-1 真实判定材料（裁决通过），含合同 emit 金向量 |
| p3xcarr-net/ | 净目标二 | m-p3xcarr 真实判定材料（裁决通过） |
| dirty-suspend-near/ | 脏形一 | gate_verdict=near_threshold 挂起形 |
| dirty-return-r2/ | 脏形二 | topic 篡改致 R2 失败材料退回形 |
| dirty-alarm-r7/ | 脏形三 | 改写链四元超三次 R7 告警形 |
| reject-missing-shot/ | 拒收一 | 缺发 |
| reject-key-mismatch/ | 拒收二 | 键不符 |
| reject-shot-misalign/ | 拒收三 | shot 错位 |
| reject-raw-empty/ | 拒收四 | raw 空 |
| adisp-guard-1-net/、m-p3xcarr-net/ | 计分材料 | 新鲜飞轮工作区跑出的计分材料金向量 |
| contract-emit/ | 合同 emit 输入 | atom.yaml 与 ng 文本种子（clip 后 sha256 对齐合同 pack 字段） |
| stats-values.json | 统计参照值 | 围堰十一函数固定输入电池参照值 |

## 归一形条款

期望件含三 token：`@ROOT@`（工作区根绝对路径）、`@MATERIAL@`（材料实参
串，仅报告 material 字段）、`@TOPIC_ENTRY@`（合同 meta.measurement_entry）。
测试运行时以自身运行时路径反填后逐字节 cmp。归一仅涉路径位，不涉任何
语义字段；活体双跑（`uv run tally check --material 绝对路径` 与引擎二进制
同参运行）cmp 零归一零差，为同参形条款主证
（证据：deyimerge-tdd-solo-materials/live-double-run-cmp.log）。

## 判据域声明

四类契约工件（采样合同 json、计分材料 json、核对报告 json、signcheck
json）取逐字节判据；facet_stats 数值函数面取统计结论等值与容差比对
（1e-9 相对），浮点文本形差不算漂移，承 SPEC-014 腿切分清单边界声明的
显式范畴排除。置换检验 RNG 为引擎侧确定性实现（splitmix64），同 seed
双跑逐字节一致与结论等值承载判据，围堰 MT19937 序列不复刻。
