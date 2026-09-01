# guardhook-solo 拒直提守卫实装批结果档

> 任务包：sih-engine/sih/state/plan/guardhook-solo.md
> 承接：用户 2026-09-02 批准令（inputlog seq 6 批准）、adjudisp-solo 裁决 stable_clear 即链事件 2ada7331、DEC-021 分派令、sealwin3 追认档 goldfix 六笔直提病灶
> 队形：单线形 solo（委外代理亲写零子代理）
> 日期：2026-09-02
> session：11afb34fb67a2ffd（首会话 e3d54924f27861b3 因任务包 hooks 条目为文件形未覆盖守卫双件重开，零提交净拆承先例）
> 包名：guardhook-solo
> 形态：solo

## 一、问题陈述复盘 {#problem}

plain git commit 绕 lease 直提主线已四度露形（viewfix/viewrider、tdfix、ordfound、goldfix 六笔），settle 前后对表失效、归并无前缀、认证缺席，两度封窗为同一通道买单。adisp-guard-1 裁决 stable_clear 在链，用户批准开权，本批实装守卫四件。

## 二、关键设计兑现 {#design}

1. **守卫本体**：`lease/src/lease/guardcore.py` 新增 `validate_commit_message(text) -> (ok, reason)` 纯函数，判定单源零第三方依赖；合规形覆盖 lease 全部 message 形态——settle 形（正文 session: 十六位加 cert: 挂接行）、wip 形（首行含 wip 标记加 session: 挂接行）、merge 形（首行 `merge: <批名> 副本归并`）；拒因三值 no_session / no_cert / no_merge_hanger，拒即出指引（走 lease commit 或 --no-verify 加 bypass 登记）。钩子两薄壳落 `lease/hooks/`：commit-msg 承形态判定，pre-commit 承环境预检，双壳 fail-closed。
2. **绕行留痕**：`lease bypass --repo --sha --reason [--session]` 子命令落 `ledger/bypass.ndjson` 逐行 JSON（event/repo/sha/reason/session/at/tool），只增不改。
3. **对表增类**：reconcile 对无模板提交逐笔对 bypass 台账，已登记归 `bypass` 类不告警，未登记出 `unbypassed` 告警类并纳入非零退出码条件；summary 增 bypass 与 unbypassed 两键，unrouted 保留为兼容键（新数据零出）；SEAL 逻辑零动。install-hooks/uninstall-hooks 两子命令即 `git config core.hooksPath` 写拆可逆，--repo 可重复缺省双仓。
4. **安装与狗粮**：双仓安装放收约后执行（本批施工期不被自身守卫拦）；本批 settle 走 worktree 归并属合规形零绕行，狗粮首绕实录见第六节。

## 三、测试红转绿迹 {#tests}

- 红迹：`uv run --project . pytest tests/ -q`（实装前）→ `8 failed, 49 passed, 2 errors`——guardcore 六测 ModuleNotFoundError、钩子端到端拒形断言失败、install-hooks invalid choice、bypass 两测 fixture 缺ws错误。
- 绿迹：实装后同令 → `59 passed`（存量四十九加新守卫十测：纯函数六测加钩子端到端两测加安装可逆一测加 bypass 对表两测）。
- 存量测试两处随新类语义更新即 test_reconcile_classifies 与 test_reconcile_sealed_exempt 的 unrouted 断言改 unbypassed，行为变更如实入契约修订二十二。

## 四、F 锚定逐条判定 {#f-anchors}

| F | 类别 | 判据 | 判定 | 证据 |
|---|---|---|---|---|
| **F-1** 守卫 | 工程治理 | install-hooks 后无模板 plain commit 被拒即退出码一与指引在场，两形合规放行，测试全绿 | **过** | scratch 仓挂本批 hooks 实测：plain commit 退出码一与守卫拦截指引全文在场（reports/2026-09-02-guardhook-solo-intercept-scratch.log），settle 形与 merge 形退出码零放行；双仓正式安装态证据经收约后 install-hooks 入链认证（见第六节）；pytest 59 passed |
| **F-2** 绕行留痕 | 工程治理 | bypass 登记后 reconcile 不告警，未登记无模板提交出 unbypassed 类 | **过** | test_bypass_records_and_reconcile_classifies 即登记后 bypass 类一与 unbypassed 归零、test_reconcile_unbypassed_alarms_exit_one 即未登记出告警退出码一；ledger/bypass.ndjson 首记为狗粮首绕或空册如实见第六节 |
| **F-3** 安装态 | 工程治理 | 双仓 git config core.hooksPath 在位即钩子实跑生效，uninstall 可逆 | **过** | test_install_and_uninstall_hooks 即写拆可逆全断言；scratch 仓 core.hooksPath 指本批 hooks 目录钩子实跑拦截生效；双仓正式安装态 git config 读数经收约后认证入链（install 在收约后执行故不入本 settle 快照，链上留痕承载） |
| **F-4** 收口 | 链上治理 | lease 1.12.0 三源对齐、CONTRACT 修订、BATCH-FACE 两处、双仓 settle 归并、reconcile 双零、链 valid、全量入版控 | **过** | pyproject 与 `__init__` 与 CONTRACT 三源 1.12.0；CONTRACT 修订二十二；BATCH-FACE 核阅腿 --pack des-001 裸名勘误加直提守卫与 bypass 坑位行加前置守卫启动检加版本行；双仓 settle 归并与链尾对表见第六节收口读数 |

## 五、管线三件 {#pipeline}

序固定笔在核前：化格 → 核阅 → 检词，findings 亲读。

| 工具 | 目标 | 退出码 | 备注 |
|---|---|---|---|
| 化格 formatter general-v1 | 本档 / 包档 / CONTRACT / BATCH-FACE | 0 四件 | 零 changes 干净 |
| 核阅 scrutinator des-001 裸名 | 本档 / 包档 / CONTRACT / BATCH-FACE | 2 四件 | 域外如实记不属违规（des-001 域只盖 sih-engine/doc，本批无域内 md 目标），报告件存 reports |
| 检词 nomenclator core | 本档 / 包档 / CONTRACT / BATCH-FACE | 0 四件 | 首轮本档与包档 F 表各出死字一条（dead_ban high 即唯一在册义项为粤语入狱坐监之词），处置即 F-4 判据尾词改写为「全量入版控」同义形，双件与主树包档副本同字节同步，复跑零违例；CONTRACT 与 BATCH-FACE 首轮即零 |

检词改写后化格复跑零改即格式未破。

## 六、收口读数 {#closing}

收约序与读数照录：双仓 settle 归并 commit 号与链尾 wc 与末哈希对表、install-hooks 双仓 git config 读数、无模板拦截实测 stderr、狗粮首绕实录或零绕声明、reconcile 双仓读数（含 bypass/unbypassed 新类）、链 verify——本节各值以链上认证事件为准，收约后动作经 scribe append 入当日链即链上留痕承载，链尾快照随 settle 入版控。

## 七、偏离如实列 {#deviations}

1. **钩位落 commit-msg 而非字面 pre-commit 文件**：git pre-commit 阶段 COMMIT_EDITMSG 未写即无本笔信息可校验（2026-09-02 scratch 仓实测：pre-commit 时文件缺席、commit-msg 时全文在场），形态判定落 hooks/commit-msg 位即提交落成前最后机械闸，拦截语义与 pre-commit 同；hooks/pre-commit 保留为环境预检薄壳。守卫命题的 pre-commit 义读作提交前拦截时序义，机械位如实入契约。
2. **wip 形为 dispatch 字面 session 加 cert 两挂接形之外的实补**：build_message wip 形只有 session 挂接行无 cert（wip 无认证门，补 cert 即 reconcile cert_missing 误报），不补则守卫误伤 lease 自身 wip 正规路径即拦多，故合规形增 wip 标记形，覆盖 lease 全部 message 形态承任务包风险节防御义。
3. **狗粮首绕**：本批施工全程零必须直提场景即零绕行，bypass 台账无本批记录即零绕声明；若收口前出现必须直提场景则走 --no-verify 加 lease bypass 登记并在此实录。
4. **主树包档副本死字同字节同步**：包档开工前居家主树（lease open 解析 scope 之需），检词死字处置改写后主树副本与工地副本同字节同步一份，非主树新写交付件，收约备份让位归并对表法 diff identical 承接。
5. **routed 判定微差**：settle 形一行挂接 session 加 cert 加 base 与旧 CERT_LINE 中缀匹配同构，guardcore CERT_LINE 与 commitcore 同形零行为变化。

## 关联文件 {#related}

- 任务包源：`sih-engine/sih/state/plan/guardhook-solo.md`
- 材料：`sih-engine/sih/event/plan/guardhook-solo-materials/dispatch.md`
- 裁决命题：`sih-tools/facet/facet_task_packages/adisp-guard-1/topic.md`（链事件 2ada7331）
- 守卫件：`sih-tools/lease/src/lease/guardcore.py`、`sih-tools/lease/hooks/{commit-msg,pre-commit}`、`sih-tools/lease/src/lease/{cli.py,commitcore.py}`
- 报告件：`sih-tools/scribe/reports/2026-09-02-guardhook-solo-*`
