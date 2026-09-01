# scrutmerge-goldfix-solo 第二轮整改指令

主会复核裁定：实质面全过即双跑三目标亲跑零差加 107 测试绿加工具件零改动；打回两件即规则包平价被破与版控尾巴第三回。重开同号任务包 `sih-engine/sih/state/plan/scrutmerge-goldfix-solo.md`（承 tdfix2 同号重执先例），机械链照旧全走。

## 整改一：规则包平价回正

1. 回滚引擎侧两处包数据改动：`src/scrutinator/packs/des-001/rules.toml` 与 `src/scrutinator/packs/des-001-mathe/rules.toml` 的 S004 消息模板恢复 `{{#锚点}}` 双花括号原样，与工具侧逐字节一致。
2. 改引擎消息渲染器（在 `src/scrutinator/rule.rs` 或 `report.rs` 的模板替换位）：识别 `{{` 渲染为 `{`、`}}` 渲染为 `}`，其余 `{name}` 占位照常替换，语义对齐 Python str.format 转义。
3. 验证三件缺一不可：`cargo test` 仍全绿；三目标（SPEC-013、DEC-020、GOV-002）同参形双跑仍 IDENTICAL 且退出码一致；`diff` 引擎与工具的 des-001 与 des-001-mathe 两 rules.toml 零差。
4. 金向量零重冻即基准是工具件输出，工具件没动，金向量不许碰。

## 整改二：版控尾巴收编

1. engine 仓提交完整当日链：`sih/event/trail/2026-09-01.ndjson` 当前工作区版本（55 事件）随本轮 settle 入册。
2. tools 仓收编 untracked：`scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-record.json` 与 `-validation.json` 与 `2026-09-01-scrutmerge-goldfix-solo-elicit-signals.ndjson`，顺带 `2026-09-01-ask3-cmdface-r2-record.json` 与 `-validation.json`。
3. 结果档追加第二轮记录：两轮始末、包平价回正、版控收编清单，外加一处误差如实申报即第一轮 tools 提交信息声称含 ask3 record 实际未含。

## 红线与机械链

工具件 scrutinator 的 src 与 tests 与 packs 零改动；引擎改动限 src/scrutinator/ 域；金向量冻结后零漂移；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即报显式带 --session；改引擎源码后 settle 前必 cargo build 加实跑探针；本轮全部产物含链尾随批入版控。

## 完工报告

渲染器改动位、三件验证证据（测试计数、双跑 cmp、包 diff 零差）、收编文件清单、认证清单、双仓 commit 号、链 verify、误差申报。
