# refseal-solo 结果档：critsweep 裁判面封印与对径互核闸入规批

> 批：refseal-solo（单线 solo，零子代理）。会话：sess-zcode-260911-refseal（session_id 059c405722355286）。
> 令源：用户 2026-09-11 「开始修复」。前置双终签在链：m-dualpath-gate-r2（crosscheck 726c1405）与 m-referee-seal-r2（crosscheck 6c076d44）。

## 一、交付面

1. `sih-tools/critsweep/sweep.py` 裁判面封印：`DEFAULT_THRESHOLD` 常量化；非缺省 `--threshold` 须携 `--threshold-reason`，缺申报结构性拒（exit 1，reason_code threshold_reason_missing，教学载荷载闸位与正典）；携事由即 flock 追加登记行入 `critsweep/ledger/referee.ndjson`（at、threshold、reason、tool），出参增 `threshold_registered`。
2. `sih-tools/critsweep/tests/test_referee_seal.py`：TDD 先红（7 failed）后绿（7 passed）——缺事由拒、空事由拒、携事由过、缺省过、登记行字段、两跑两行追加不覆写、常量与 argparse 对表。
3. `sih-tools/critsweep/tests/test_sweep.py` 一处契约连带：test_threshold_boundary 非缺省调用携事由（零断言语义改动，任务包排除条款的应然偏离如实申报）。
4. `sih-tools/COURSE-v2.md` 追加两节：对径互核闸（双径独立取数机械对表，禁静默平均，新载体验收表须有对径位）与裁判面封印（读面比例形：非缺省越窗须事由加登记；写面逃逸阀仍按三件套）。

## 二、F 锚定

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 | 裁判面 | 缺事由越窗结构性拒 | 通过（test_enforce_blocks_non_default/blank_reason） |
| F-2 | 裁判面 | 携事由登记行在册且追加不覆写 | 通过（test_register_appends_row/not_overwrites） |
| F-3 | 零回归 | 既有 critsweep 测试面全套绿 | 通过（30 passed，含契约连带一处） |
| F-4 | 入规 | COURSE-v2 两闸节在案 | 通过（追加节对表） |
| F-5 | 结算治理 | intent 笔与认证笔在链、双仓 settle 落、锁零会话吊销 | 通过（intent 12995e07、cert 见链、close 后双零） |

## 三、边界申报

1. test_sweep.py 一行契约连带与任务包「既有测试零改动」排除条款相抵，属契约变更的应然连带，如实申报。
2. 主树 state/plan 任务包副本在 open 时落、worktree 同名件随批提交，内容逐字一致。
3. close 无主闸与 CALL-LOG 闸预期拦既有环境红五件与在途 CALL-LOG，走旗标绕行事由在册（非本批产物）。
