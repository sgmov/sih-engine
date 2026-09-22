# auditfix 结果档（2026-09-22）

批主：sess-zcode-260922-auditfix；租约会话 0be46912450da167；分支 msh/auditfix（对 main）。
意图笔 event_hash 1b4e0600；施工五子代理（af-doc-report、af-doc-readme、af-code-sddg1、af-code-invar、af-code-meta）工地施工，主会逐批独立核验后采信。

## 交付与验收判词 {#verdict}

- af-01 报告勘误：三十六处勘误就地修正加一处评分表单元格补笔；主会终验 stale 值零在位（陈旧编号仅存于勘误说明与修订记录的历史引用）；化格 exit 0（幂等）；检词 exit 0 零违例；核阅 exit 1 见偏差节第一条。证据：materials/tests/pipeline-verify.txt。
- af-02 README 工具清单：写面清单补 record_direct 与 lease_unclaim 并标注本地可信位，计数改外部分级十加本地可信位二；与 server.rs tool_names/beta_tool_names 21 具 diff 对表零差异（子代理 diff exit 0，主会复核 grep 计数一致）。证据：materials/tests/readme-verify.txt。
- af-03 SDDG-1 内单测：sddgate.rs 既有测试模块补三测，执法语义零改；主会独立重跑 cargo test --bin lease 得 27 passed 0 failed（原 24 加新 3）；grep #[test] 计 9。证据：materials/tests/cargo-test-lease.txt。
- af-04 不变式正测：hash.rs 两测加 append.rs 一测进既有测试模块；子代理证实 serde 于 String 与 BTreeMap 载荷不可失败、should_panic 不可达；主会独立重跑 cargo test --lib event_stream 得 102 passed 0 failed；grep 计 102（基线 99 加 3）。证据：materials/tests/cargo-test-event-stream.txt。
- af-05 meta：Cargo.toml 显式 [[example]] 三段（grep 计 3）；registrydemo.rs 与 tools_registry.rs 旧申报注后各加了结注记一行（原文未删）；主会复跑 cargo check --examples exit 0。证据：materials/tests/cargo-check-examples.txt。
- 治理面：意图笔在链（1b4e0600）；认证笔三笔（核阅、检词、测试证据）在链；settle 加 close 加 reconcile 加链 verify 读数见本文尾节。

## 偏差 {#deviation}

- 核阅 des-001 于勘误后报告实测 536 处违例，七类（C001 破折号 46、C002 强调 70、C006 表格 182、S004 39、F003 176、F005 22、S005 1）全系初版既有行文风格项，勘误未新增违例类别；报告属 assessment 类非 PRO/DES/GOV 类 T6 产出，全量风格归一候批（专项化格批），本批不做半改半留。证据：materials/tests/scrutiny-des001-report.json。
- REC-006 三测落地名与原档建议名不同，且 teaching embed 经子代理核实为 run_gate 拒收输出载荷非输入，三测按直调投影语义落地（拒收载荷在位、申报缺席 fail-closed、形坏 fail-closed），REC-006 验收判据（grep ≥9 与 cargo test --bin lease 全绿）满足。
- REC-016 应编排位修订重写为不变式正测形：三处 expect 为不可达构造性不变式，should_panic 撤除，正测三条钉住；与报告 REC-016 修订节一致。
- append.rs 既有测试 test_mint_non_monotonic_hint_instead_of_reject 带重复 #[test] 属性两枚（主仓同位亦然，可编译），系既有面，本批未清候批。
- 主树未跟踪件收约让位：迭代建议报告原件备份于本 materials（report-original-backup.md），主树原件于 close 前让位由工地版归并接管；姊妹篇评估报告仍无主未跟踪于主树 doc/assessment/，候批直改链笔申报或入册。
- README.md 与本报告批内管线面：README 在 des-001 域外（域外 exit-2 不属违规，未跑化格）；报告核阅读数见偏差第一条。
- 编号水位申报（供后继批对表）：经本批勘误，报告拟号占用 DEC-027 至 DEC-039 与 DES-020 至 DES-022；现存目录编号 DEC 至 026、DES 至 019，占号无冲突。

## 尾读数 {#tail}

settle、unlock、close、reconcile、scribe verify、critsweep、attnanchor 读数由编排位于收约后补记于当日链与会话报告，本档不预书。
