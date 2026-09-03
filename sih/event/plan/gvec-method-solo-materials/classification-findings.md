# gvec-method-solo 件一：金向量用法审计分类档

> 只分类不改，逐断言附原文行。甲类=变更检测（重放一致/行为零变更/漂移报警，声明范围内健全）；乙类=隐含正确性主张（由重放一致推出正确或无误，越权）。乙类清单即本批漏洞清单。

## 甲类（变更检测，声明范围内健全）

### A1. engine scrutinator 金向量逐字节对表（src/scrutinator/tests.rs L86-210）
逐件断言 stdout==金向量逐字节。此为重放一致/漂移报警，在「引擎件输出相对冻结基线不再漂移」声明范围内健全。
- 证据行：L92 `assert_eq!(stdout, expected, "…输出与金向量逐字节不一致")`（14 件同形）

### A2. engine attractor 冻结期望逐字节对表（tests/attractor_golden.rs t1/t2）
复放冻结期望件逐字节一致。变更检测。
- 证据行：t2_check_reports_byte_identical_all_scenarios 等

### A3. engine route 金向量逐字节对表（tests/attractor_route.rs t2_golden_byte_identical）
对表围堰冻结值逐字节 + 退出码对齐。变更检测。
- 证据行：L115 `assert_eq!(String::from_utf8_lossy(&out), expected_text, …)`

### A4. engine event_stream 哈希复算（src/event_stream/tdd_tests.rs t1_golden_vectors_recompute）
compute_event_hash 复算==向量 expected_hash。密码学重放一致性，纯变更检测。
- 证据行：`assert_eq!(compute_event_hash(&event), v["expected_hash"])`

### A5. tools vectors 目录（locator/elicit/parser）
工具自测跑冻结向量 failed==0 / 逐字节对表 expected。变更检测（输入→期望输出可复现）。
- 证据行：locator test_vectors.py L6 `assert rep["failed"] == 0`

### A6. attractor stats 统计结论等值（tests/attractor_golden.rs t2_stats_conclusion_equivalence）
统计结论等值+容差比对，自声明范畴排除（浮点文本形显式排除，不做字节级）。声明范围内健全，接近甲。
- 证据行：L267 注释「不取字节级（浮点文本形显式范畴排除）」

## 乙类（隐含正确性主张＝越权，本批漏洞清单）

### 漏洞 V1 — scrutinator 退出码语义标签断言（src/scrutinator/tests.rs）
当 `assert_eq!(code, 1, "goldfix-001-multiflag 应退出 1（违规）")` 把金向量目标内容判为「违规」，以及 L90/L110/L120/L130/L140 把净目标判为「应退出 0」，是由重放金向量一致推出该目标被正确判定为合规/违规——隐含正确性主张。金向量只是冻结基线，其自身内容是否确为「违规样例」取决于当初构造时的真值，未另查。
- 证据行：src/scrutinator/tests.rs:157（goldfix-001 应退出 1 违规）、:107-113（净目标 0）

### 漏洞 V2 — exit_compliant_zero 显式正确性声明（src/scrutinator/tests.rs L216-222）
L218 注释「合规场景：GOV-002 在 des-001 域内，零违规，退出码 0」，L221 `assert_eq!(code, 0 …)` 由重放 gov002 金向量推出该目标「合规零违规」——把金向量当合规标准，隐含正确性主张。
- 证据行：src/scrutinator/tests.rs:218、:221

### 漏洞 V3 — attractor route 退出码语义对表（tests/attractor_route.rs t2_golden_byte_identical / t3_exit_code_table）
`assert_eq!(code, expected_code, "{scenario} 退出码须对齐围堰冻结值")` 与 t3 「有告警须 1」「包缺席须 2」把路由退出码（0/1/2）当作对输入语义归属的裁决。退出码 0/1/2 含裁决含义，对齐冻结值即由重放推出「该输入确应归此路」——隐含正确性主张，未另查输入的真值归属。
- 证据行：tests/attractor_route.rs:114、:159、:175

### 漏洞 V4 — 结果档 F 表以金向量零漂移为合格判据（sih-engine/sih/event/plan/*-results.md 29 件/114 处）
F 表以「金向量零漂移/逐字节回归通过」作为批验收通过判据，如 scrutmerge-goldfix-solo F-1 `6 件脏目标金向量…冻结…净目标 6 件 cargo test 续过零漂移`。这是由重放一致推出「批合格/工具正确」的隐含正确性主张：金向量零漂移只证重放稳定性，不证锚集真值即金向量自身内容正确。
- 证据行：sih-engine/sih/event/plan/scrutmerge-goldfix-solo-results.md:20（F-1）

### 漏洞 V5 — 锚集形式化缺口（ORD-007 锚集＝金向量，锚绿≠链真）
工程注意事项第 1/4 条：锚集先行登记（规则包版本与金向量哈希）与局部通过不推全局（终局判定前须独立核查锚集真值如规则包对金向量回归）。当前金向量用例普遍缺「锚集真值独立核查」这一剩余条件，乙类即此缺口的具体形态。
- 证据行：sih-math/order/entries/ORD-007-derivation-soundness-and-mirror.md 工程注意事项 1、4

## 计数

- 枚举金向量位点（六族）：engine scrutinator 14 断言 / attractor 9 test / route 10 test / event_stream 5 向量 / tools vectors 3 处 / results F 表 29 件 114 处
- 甲类：A1-A6 六族（变更检测）
- 乙类（漏洞清单）：V1-V5 五件，具体断言位点见各行

> 结论：金向量在全仓承重的本质是重放一致性（甲类可单独承重），但任何把重放一致当正确性证据的地方（V1-V5）即锚集真值缺口，须由方法论规范补足，此即件二 SPEC-017 立项动因。
