# scrutmerge-switch-solo 重开指令（含 sealwin3 连跑）

用户已批准切换重开与封窗。两批连跑：先切换批重开，收约干净后接 sealwin3-solo 封窗批，界线锚即切换批自己的归并点。两批各走完整租约生命周期，前批收约干净才开后批。

## A 部：切换批重开

任务包 `sih-engine/sih/state/plan/scrutmerge-switch-solo.md` 原口径不变，重开新会话执行。goldfix 已把双跑障碍清尽即包平价零差、107 测试绿、三目标同参形双跑零差、SPEC-013 修订四已立同参形条款。

### 主体步骤

1. **双跑判据**：cargo build 加 cargo test 全绿；三目标即 SPEC-013 与 DEC-020 与 GOV-002 同参形双跑（双侧绝对路径、包名字符串一致）cmp 零差且退出码一致，证据入材料件。
2. **换旗**：AGENTS.md 三处核阅位改指引擎件即概览行、审计节核阅调用行、文件索引核阅行，措辞承书简先例；宪法节与两条启动命令一字不碰，改动前后字节级 diff 自证仅限核阅位。BATCH-FACE.md 核阅腿命令改引擎件路径。
3. **工具件退役**：`sih-tools/scrutinator/CONTRACT.md` 加退役标注承书简先例，CALL-LOG 落尾行。src、tests、packs 零改动。
4. **完成档与追记**：mergeback-scrutinator-completion-<日期>.md 按书简完成档体例载三查（含挂起与 goldfix 两轮始末引用）；GOV-003 v1.6 即参验席实例归位；SPEC-013 修订五即切换执行记录。
5. **版控收编**：切换自件四件即 scribe/reports 下 `2026-09-01-ask3-scrutmerge-switch-solo-record.json` 与 `-validation.json` 与 `2026-09-01-scrutmerge-switch-solo-elicit-signals.ndjson` 加 `2026-09-01-cmdface-r2-elicit-signals.ndjson`；engine 侧 `sih/state/plan/scrutmerge-switch-solo.md` 包档与 `cmdface-solo.md` 的叩问补笔改动。
6. **路标段正式收口**：`sih/event/plan/scrutmerge-tdfix-solo-results.md` 路标段改为正式收口记录，事实即接手代理撞锁跳链后主会 2026-08-31 代偿补链，重跑双门绿，补意图 993abd97 与十九笔认证入 08-31 链即当时 118 事件 valid，recall-face 件如实拒认非报告形。
7. **scribe 收尾笔**：核验 `sih-tools/scribe/CALL-LOG.md` 的 viewreg-solo 行已付与否，未付即补。
8. **inputlog 三笔**：当日 inputlog 追加，会话号 sess-zcode-260901-acceptor，逐字转录：`完工报告`、`sih-engine/sih/event/plan/scrutmerge-goldfix-solo-r2-report.md — 含渲染器改动位、三件验证证据、收编清单、认证清单、双仓 commit、链 verify、误差申报、越线与后续动作。`（该句用户中转两次即记两笔同文）、`切换批准重开，封窗`。
9. **本批管线核阅腿即引擎件**：切换狗粮位，T6 文档核阅实跑 target/debug/scrutinator。

### 红线

本批提交必须全走 lease commit 正规路径即 worktree settle 加 close 归并，**严禁 plain git commit 直提主线**即 goldfix 六笔教训，归并提交必带「merge: 批名 副本归并」形；链尾提交必须等于工作区现链即 settle 前后 wc 对表；引擎源码零改动只 build 与测试；工具件 scrutinator 的 src 与 tests 与 packs 零改动；金向量零漂移；AGENTS 宪法节零触碰；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即报显式带 --session。

tally 期票句 rider 注销即 crossgraph 批已兑付，不欠。

### 机械链

照旧一步不缺：ask3 新记录（引文程序切片）→ 双门 → elicit 加 digest → 正身 → lease open（--package scrutmerge-switch-solo，双仓，--allow AGENTS.md）→ 取锁 → meter 包裹引擎 scribe intent → 工地施工 → 管线笔在核前（化格工具件、核阅引擎件、检词）→ 认证 → 双仓 settle → 放锁 → close（备份让位归并对表法）→ reconcile（容忍既有 unrouted 与 cert_missing 存量，零新增即可，封窗在 B 部）→ 链 verify → 调用册 → 结果档。

## B 部：sealwin3-solo 封窗批

A 部收约干净后立即开。任务包 `sih-engine/sih/state/plan/sealwin3-solo.md`，读全文。

1. 取 A 部两仓归并点 merge 哈希即 `git log --merges -1` 各仓。
2. `sih-tools/lease/src/lease/commitcore.py` 的 SEAL_BASES 双仓界线改指各自归并点，SEAL_EXEMPTS 零改动，lease 升 1.11.0 三源对齐即 pyproject 与 __init__ 与 CONTRACT，CONTRACT 修订记录承封窗令与界线前移。
3. 追认档落结果档：被封存量逐笔全列即 engine unrouted 七笔与 tools unrouted 五笔与 tools cert_missing 八笔（开工时以 reconcile 明细枚举），逐笔 sha 加主题加定性即 goldfix 直提违章三笔各仓、tdfix 两笔、viewfix 与 viewrider 归并无前缀、ordfound 收尾笔、认证缺席旧账。
4. 验证：双仓 reconcile 即 unrouted 零与 cert_missing 零即 sealed 计数如实；resolve_default_base 实跑核 base_source 为 seal_line；lease 测试全绿。
5. 机械链同款全走，正规路径红线同 A 部。

## 完工报告

两批各一段：意图哈希、双跑 cmp 与测试计数、认证清单、双仓 settle 与归并 commit 号、链 verify、reconcile 前后读数对照、追认档笔数、F 表、越线与误差申报。
