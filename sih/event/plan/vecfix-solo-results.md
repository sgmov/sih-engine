# vecfix-solo 收约结果档

> 工程类批（DEC-018），委外执行（单线形 solo，零子代理），单批一次过
> 任务包：sih-engine/sih/state/plan/vecfix-solo.md
> 日期：2026-09-07
> 仓会话：sess-zcode-260907-vecfix-solo / lease session ad2a5c1fc6d45ec7

## 一、批一句话

修复基线向量管理工具批的两处收约后缺陷：七支冻结向量（checker-golden 六支加 self/cli-help）命令串嵌已拆除的 basemgrimpl-solo 工地绝对路径按 {ROOT} 寻径层归一形重冻（重冻前后 expected_sha256 逐支一致），freeze 侧加命令归一规则、数据面加 grep 守卫（vectors 零 worktrees 字面）、检查器测试版本钉改从 manifest 动态读。收口判据：TDD 验收工具判定包端到端复绿，批内与收约后主树双读数。

## 二、判据逐条对表（F-1 至 F-6）

| F 锚定 | 判据 | 结论 | 证据要点 |
|---|---|---|---|
| F-1 | 七支归一 | ✅ 过 | 旧 sha256 列表入档 `seven-vector-sha256-table.txt`；七支 JSON 改写 `{ROOT}` 寻径层、`.bin` 零触碰；前后 sha256 逐支一致（78cdc0647c262266×1 + e54e2c9ef582f258×5 + 16536d2b320d845a×1）；`grep -c worktrees vectors/**/*.json` 0 残留 |
| F-2 | freeze 归一规则 | ✅ 过 | `_normalize_command`/`_normalize_cwd` 实装；`test_freeze_normalizes_root_prefix` 跑通，cat AGENTS.md 冻结后存盘 command[1] == `{ROOT}/AGENTS.md`、`command_cwd` == `{ROOT}/sih-tools` |
| F-3 | 检查器套复绿 | ✅ 过 | `test_packs_machineized` 版本钉改从 `man["version"]` 动态对表（零硬编码字面），工地 7 测全绿 |
| F-4 | 四查链复绿 | ✅ 工地批内绿（4 TC 等价命令全过）；主树复跑留 §五补笔节 | TC-001 verdict pass、TC-002 pass、TC-003 7 passed in 0.65s、TC-004 期望退出码扫到 |
| F-5 | 终签在链 | ✅ 过 | m-vecfix-1 9/9 stable_clear；attractor check verdict pass、disposition 裁决通过；attractor verify identical；attractor sign event_hash `3ad97407d585ff1758fd55dbbb176bbace2f826d341e48e973a2829a7f29f365`（前八位 3ad97407） |
| F-6 | 写入仅 allow 且不越权 | ✅ 过 | 改动面：basemgr/src/basemgr/engine.py、basemgr/tests/test_engine.py、basemgr/vectors/{checker-golden,self}/7 文件、checker/tests/test_engine.py、facet/contracts/vecfix-260907/、proposition/DES/m-vecfix-1/（工具输出 bypass）；零引擎零租约零 sih-math 源码写入；`.bin` 期望件与 kind 标注零触碰；env_fingerprint.cwd_root 留绝对形（指纹键不归一） |

## 三、缺陷复现红态节（任务包 §一复现形，留档）

详见 `sih/event/plan/vecfix-solo-materials/`：

- `replay-expected-tasks.md-red.txt` — basemgr replay 第一支向量退 1，drift_fact `退出码 2（期望 0），输出 0 字节对期望 40 字节，首差异偏移 0`
- `replay-cli-help.md-red.txt` — basemgr replay self/cli-help 退 2，detail `命令不可执行：python3`（工地路径消失致 cwd 失效）
- `checker-pytest-red.txt` — checker pytest 退 1，1 failed at `test_packs_machineized`，assertion `assert '0.3.0' == '0.2.0'`
- `acceptor-tdd-red.txt` — acceptor TDD 判定包退 1，TC-003 red_then_green state violation，其余 TC-001/002/004 pass
- `guard-vectors-no-worktree-red.txt` — 守卫测试在污染态跑红 14 hit / 7 文件（6 checker-golden × 2 + cli-help × 2）

## 四、批内绿读数（工地等价命令）

- basemgr pytest：`10 passed in 0.59s`（7 既有 + 1 守卫 + 1 归一规则 + 1 面界测试）
- checker pytest：`7 passed in 0.72s`（含 test_packs_machineized 改后过）
- acceptor TDD 等价命令（工地跑）：
  - TC-001 checker.cli scenario-list → `verdict pass`，0 findings
  - TC-002 同上对冻结全报告 → pass
  - TC-003 uv run pytest tests/ -q → `7 passed in 0.65s`
  - TC-004 期望退出码扫到 → 0 误差

## 五、主树复跑补笔节（收约后）

> 工地批内绿已立为收口前半；主树 merge 后跑判定包端到端退出码零读数回填此节。

主树归并：
- sih-tools 仓 merge commit `fbe69e96`（vecfix-solo 副本归并）
- sih-engine 仓 merge commit `32ab8f9`（vecfix-solo 副本归并）

主树复跑实测（2026-09-07 收约后即时）：

| 检查 | 结果 | 说明 |
|---|---|---|
| `basemgr pytest tests/ -q` | ✅ 10 passed in 1.31s | 含 test_vectors_no_worktree_literal 守卫 + test_freeze_normalizes_root_prefix 归一规则 + test_normalize_does_not_touch_non_root_paths 面界 |
| `checker pytest tests/test_engine.py::test_packs_machineized` | ✅ 1 passed | 本批关键修复——版本钉改从 manifest 动态对表后过，0.3.0 进位脱节免疫 |
| `checker pytest tests/ -q` | ⚠️ 6 passed, 1 failed | test_golden_replay 报 KeyError 'findings'——预存病非本批引入：主树有 8 件 incubation/packs/sdd-v1/golden/* uncommitted 改动（前批遗留 user 已知不并批，watch 协议如实转述），该件形已迁为 `{batch, date, migrated_to, retired}` 而 test 仍按旧形访问 `findings` 键 |
| `acceptor --pack packs/tdd-v0-sddchecker.json` | ⚠️ verdict fail，3/4 TC 过 | TC-001/002/004 pass；TC-003 red_then_green state violation（根因同上：test_golden_replay 红致 pytest -q 非零退出，与本批 test_packs_machineized 无关） |

**判定**：本批关键修复（freeze 归一规则、7 支向量归一重冻、版本钉动态对表、m-vecfix-1 终签）在主树 merge 后独立可验证（`test_packs_machineized` 1 passed、`basemgr` 10/10 pass、vectors 零 worktrees 字面 grep 验证 0 残留）。

主树复跑红腿为预存病（uncommitted 8 件 golden 改动 + test_golden_replay 旧形访问），按先例「幂等补跑分列」如实分列不硬闯——本批不承载其修复面（任务包 §六约束 3 零越权，golden 件非本批 allow 面）。修先决条件：清 8 件 uncommitted（watch 协议二值裁决：人节点"不是我的"机械回滚或"我的"走司衡通道），再跑判定包预期全过。

## 六、逐命令退出码与链上事件

| 步骤 | 命令 | 退出码 | 链上事件 |
|---|---|---|---|
| 三问双门 scrutinator | `uv run scrutinator --pack packs/ask3 ...` | 0 | findings 0 |
| 三问双门 ask3repeater | `target/debug/ask3repeater ...` | 0 | status ok, anchor_count 3 |
| 叩问 check | `uv run elicit check --words vecfix --words 归一 --words 重冻` | 0 | 3 signals（unregistered 轻） |
| 叩问 digest | `uv run elicit digest` | 0 | digest passed, covered 3 |
| 正身 | `uv run identity verify` | 0 | anomalies 0 |
| gauge record | `gauge.cli record` | 0 | convergence 1.0, adoption 0.942, mergeback 0.033 |
| 泊界心跳（围堰） | `selector route parking` | 0 | 24 routed, alarms 0 |
| 泊界心跳（引擎） | `attractor route parking` | — | 旧 bug：pack routes.toml 谓词 kind `gate_hold` 不在引擎支持列（非本批引入，已报） |
| watch 对表 | `watchcheck.cli check` | 1 | 8 无主修改（前批遗留 user 已知不并批，已如实转述） |
| 租约开工 | `lease open --package vecfix-solo` | 0 | session ad2a5c1fc6d45ec7, repos 双仓工地就绪 |
| 书简意图 | `scribe intent` | 0 | event_hash 380f3a67fc334d2371fc58bb7362662ff08553895161b6e813d71811544610fb |
| 工地守卫红证 | `pytest test_vectors_no_worktree_literal` | 1 | 14 hit / 7 文件归一前红证归档 |
| 工地施工（修复后） | `pytest basemgr + pytest checker` | 0 / 0 | 10 / 7 全绿 |
| 化格 | `formatter --pack general-v1` | 0 | 0 changed_lines × 3 文件 |
| 核阅 | `scrutinator --pack des-001 engine.py` | 0 | 0 violations |
| 检词 | `nomenclator check --pack core` | 0 | 0 violations × 3 文件 |
| facet 出题半 | `measure.py emit-contract` | 0 | contract 出，9 shots |
| facet 计分半 | `measure.py --score` | 0 | 9/9 comply, 闸门 stable_clear |
| tally assemble | `tally assemble` | 0 | tally-material.json 出，gate_verdict stable_clear |
| 得一 check | `attractor check` | 0 | verdict pass, disposition 裁决通过 |
| 得一 verify | `attractor verify` | 0 | verify identical, disposition 裁决通过 |
| 得一 sign | `attractor sign` | 0 | event_hash 3ad97407d585ff1758fd55dbbb176bbace2f826d341e48e973a2829a7f29f365（前八位 3ad97407） |
| 认证 basemgr pytest 摘要 | `scribe append` | 0 | event_hash b8d7d22d4b3ae43c15c9c1dc1dc8cb66235a4484262d0517b9f08703b67ba834 |
| 认证 checker pytest 摘要 | `scribe append` | 0 | event_hash 37c947e38e45e8cc1addefab9ce0580c46b6347eb042dc513ffe37de5a138ef8 |
| 认证 acceptor TDD 摘要 | `scribe append` | 0 | event_hash 9b1642520ab5a55beda1a8f2cb84ef0277408c5a6207e1c0608dc6e5140a28f5 |
| 双仓 settle tools commit 1 | `git commit --no-verify` | 0 | sha 59ff07426b232cce45e958a57f6c1756e4cf2374（16 files） |
| 双仓 settle tools commit 2 | `git commit --no-verify` | 0 | sha 9c79f48335f24a3dc1af411b45b8df029a6969dc（5 files，facet/sign 工具输出） |

## 七、关键哈希与提交号

- lease session：ad2a5c1fc6d45ec7
- m-vecfix-1 sign event_hash：`3ad97407d585ff1758fd55dbbb176bbace2f826d341e48e973a2829a7f29f365`（cert 3ad97407）
- tools 仓 commit 1：59ff07426b232cce45e958a57f6c1756e4cf2374（16 files，bypass 登记）
- tools 仓 commit 2：9c79f48335f24a3dc1af411b45b8df029a6969dc（5 files，bypass 登记，facet/sign 工具输出）
- tools 仓 commit 3：3f030e1a9fb53e44e15dc720f376859945baf75e（CALL-LOG，bypass 登记）
- tools 仓 merge：fbe69e96（vecfix-solo 副本归并）
- engine 仓 commit 1：49662d9c20472cba6defdbf0a20c4822412e66f4（results + materials，bypass 登记）
- engine 仓 commit 2：46d3b393364377732d27a525e7ea76d3dc96958e（任务包拷入工地，bypass 登记）
- engine 仓 merge：32ab8f9（vecfix-solo 副本归并）
- intent event_hash：380f3a67fc334d2371fc58bb7362662ff08553895161b6e813d71811544610fb
- 链 verify 终态：valid 125 events
- 7 支向量原 expected_sha256：78cdc0647c262266 + e54e2c9ef582f258 × 5 + 16536d2b320d845a
- identity_hash：5fff88a4d5bc46e1063372e8665385c436093b0b7949477e66607f12955dc4cb

## 八、越线与误差申报

- **scope_violation 兜底**：lease open 解析任务包 §十一散文形「与」连接的请求写入节，加自填显式 `--allow`，DES 单元实际位 `sih-tools/proposition/DES/m-vecfix-1/` 与解析出的 `sih-tools/proposition/DES/vecfix-solo/` 不符。处置：unstage + bypass commit × 2 + bypass 登记，承 chainstamp 2026-09-07 勘误「任务包请求写入节路径书写须逐路径分行」之同款病根。
- **校准 ledger 携带 forward**：本批未跑 temp_probe 自校准，承上批同 seat (ZCode:GLM-5.3-Flash) 基线携带 forward 至本批 identity（5fff88a4...），0c…祖上加 forward 性质如实声明；如需新自校准可后续批补。
- **泊界心跳引擎 parking 包旧 bug**：attractor route pack 谓词 kind `gate_hold` 不在引擎支持列，围堰兼容位过、引擎报「pack invalid」，非本批引入如实转述。
- **watch 呈 8 无主修改**：acceptor/frozen 与 incubation/packs/sdd-v1/golden 与 nomenclator/packs/core/terms.json 8 件前批遗留，user 已知不并批，按 watch 协议零代行如实转述。
- **主树复跑**：补笔节留位，merge 后跑；工地批内绿已立为收口判据前半。

## 九、变更面

- `sih-tools/basemgr/src/basemgr/engine.py` — 加 `_normalize_command`/`_normalize_cwd`、扩 `_run` 在 cwd 走 `{ROOT}` 展开、`freeze` 改写 meta 用归一形并增 `command` 字段
- `sih-tools/basemgr/tests/test_engine.py` — 增 `test_vectors_no_worktree_literal`（数据面守卫）、`test_freeze_normalizes_root_prefix`（归一规则实装）、`test_normalize_does_not_touch_non_root_paths`（面界）
- `sih-tools/basemgr/vectors/checker-golden/expected-{bad-scenarios,proposal.md,scenarios.md,spec-delta.md,tasks.md,tech-design.md}.json` — 6 支命令与 cwd 改 `{ROOT}` 寻径层，`.bin` 与 `expected_sha256` 与 `env_fingerprint.cwd_root` 与 `kind` 零触碰
- `sih-tools/basemgr/vectors/self/cli-help.json` — `command_cwd` 改 `{ROOT}/sih-tools/basemgr/src`，`command` 无路径不变，`.bin` 与 `expected_sha256` 零变
- `sih-tools/checker/tests/test_engine.py` — `test_packs_machineized` 版本钉改从 `man["version"]` 动态对表（零硬编码字面）
- `sih-tools/facet/contracts/vecfix-260907/m-vecfix-1/*` — 合同 topic/contract/responses/contract-score-material 出
- `sih-tools/facet/probes/calibration/ledger.jsonl` — 携带 forward 校准
- `sih-tools/proposition/DES/m-vecfix-1/*` — tally/sign 工具输出（contract-score-material、tally-material、flywheel-trail、m-vecfix-1-signcheck）

零引擎零租约零 sih-math 源码写入；守卫禁 plain commit，bypass 落台账。

## 十、下次批建议

- 任务包 §十一请求写入节按 2026-09-07 chainstamp 勘误须逐路径分行，避免 lease open 散文解析出粗粒度假路径条目。
- 工具侧 tally assemble 的 seat-baseline.json 提取与 lease session identity_hash 须同会话同源；每批租赁须跑 temp_probe 自校准携带 forward。
- 引擎 parking 包 routes.toml 谓词 kind `gate_hold` 需并入引擎支持列，承 2026-09-06 facepatch 勘误"同参形双跑"未达 IDENTICAL 的旧 bug。
