# refseal-solo 任务包：critsweep 裁判面封印与对径互核闸入规

> 令源：用户 2026-09-11 「开始修复」；前置双终签在链 m-dualpath-gate-r2（726c1405）与 m-referee-seal-r2（6c076d44）。
> 队形：单线 solo，零子代理。会话：sess-zcode-260911-refseal。域：sih-tools 主。

## 交付面

1. `sih-tools/critsweep/sweep.py` 裁判面封印：非缺省 `--threshold` 须携 `--threshold-reason <事由>`，缺申报结构性拒（exit 1，reason_code threshold_reason_missing）；携事由即登记追加行入 `sih-tools/critsweep/ledger/referee.ndjson`（at、threshold、reason），出参增 `threshold_registered` 字段。
2. `sih-tools/critsweep/tests/test_referee_seal.py`：TDD 先红后绿（缺事由拒、携事由登记行、缺省零登记、两跑两行追加不覆写）。
3. `sih-tools/COURSE-v2.md` 追加节：对径互核闸（双径独立取数机械对表，不合告警禁静默平均，新载体验收表须有对径位）与裁判面封印（读面比例形：非缺省越窗须事由加登记）。
4. 批结果档 `sih-engine/sih/event/plan/refseal-solo-results.md`。

## 明确不做

sweep 判据算法本体零改动；critsweep 既有测试零改动（只增不改）；无主五件环境红零触碰。

## F 锚定

| F | 判据 | 验法 |
|---|---|---|
| F-1 | 缺事由越窗结构性拒 | pytest 红→绿 |
| F-2 | 携事由登记行在册且出参回显 | pytest + 对表 |
| F-3 | 缺省路径零登记零破坏 | pytest |
| F-4 | 既有 critsweep 测试面零回归 | 全套绿 |
| F-5 | COURSE-v2 追加节两闸在案且 settle 链笔齐全 | 对表 + 链 verify |
