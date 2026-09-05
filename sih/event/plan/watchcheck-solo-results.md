# watchcheck-solo 结果档

> 批:watchcheck-solo（watch 对表与异常呈报）
> 会话:445c12fbb3e4283c（lease 1.23.0，scope_source package）
> 日期:2026-09-05
> 队形:单线形 solo，零子代理
> 范式:T6 单线 solo，委外代理亲写零子代理
> 令源:用户 2026-09-05 裁定（leaseoptsettle-solo-results.md 第八节）：零 token 二值协议、通道必经、单协议处置、置信度奖惩；任务包 sih-engine/sih/state/plan/watchcheck-solo.md 唯一规格源

## 一句话结论

watchcheck-solo 批交付：稽 watchcheck 工具首立（对表判定式实装与豁免面十六面冻结登记），TDD 七测先红后绿，活跑双跑逐字节 IDENTICAL（退出码 1/1），无主清单 18 件机械呈报候人节点二值裁决；例行读数挂点接线 BATCH-FACE 登记节与处置协议文在档；例行读数因主树 gauge 被并行批在途改件破面走 HEAD 物化形落链（偏离如实申报）；facet 一裁 round1 九发全 comply 零变卦而 basis_consensus 子判据挂落 boundary，按「boundary 停批、agent 不得代写重写上下文」纪律呈用户候裁，F-5 未过执契，其余五判据全过；双仓 settle 收约对表随批。

## 一、F 表（完成度表）

| F 锚定 | 类别 | 判据 | 结论 | 证据 |
|---|---|---|---|---|
| **F-1** | 判定谓词 | 三族夹具先红后绿：声明内净态绿、无主修改红、豁免面不误报 | 过 | tests/test_watchcheck.py 红相 6 failed（三族全红）后绿相 7 passed；活跑双跑对真实工作区 18 件无主清单红证在案 |
| **F-2** | 双跑一致 | 同参双跑输出逐字节一致 | 过 | 夹具 test_dual_run_byte_identical 绿；活跑 run1/run2 cmp IDENTICAL 退出码 1/1（materials/first-run-2026-09-05） |
| **F-3** | 挂点 | 例行读数后同跑实测在档，BATCH-FACE 登记节在档 | 过 | 例行读数 ga-2 三维落链（convergence 0.714286 / adoption 0.857143 / mergeback 0.043478）后同跑 watchcheck 双跑在档；BATCH-FACE「watch 对表挂点与处置协议」节在档 |
| **F-4** | 零 LLM | 命令全程零模型调用零网络 | 过 | 实现纯 stdlib（json/subprocess/sqlite 弃用/pathlib），零网络调用零模型调用，显式给参禁钟读 |
| **F-5** | 判定语义 | 无主判定谓词与豁免面过得一裁，near_threshold 呈用户 | **未过（boundary 呈用户候裁）** | round1 九发全 comply 变卦 0% 谨慎 0/9，basis_consensus 子判据挂（distinct_basis=3>1）→ verdict boundary；按「boundary 停批、刀锋类须人重写上下文、agent 不得代写」纪律停执契呈报，不代写不重滚；详见第四节 |
| **F-6** | 呈报形 | 输出即人话清单，人节点可二值裁决，零代裁动作 | 过 | 输出形：判定式行加输入面行加结论行加逐件清单（路径加 mtime 加 git 态）；处置二值指引随附；core.py 零写零 restore 零建议 |

## 二、交付清单

- **工具首立**:sih-tools/watchcheck/（core.py 判定核、constants.py 豁免面十六面冻结逐面带出处、cli.py 命令面、CONTRACT.md 判定语义正典、CALL-LOG.md、tests/test_watchcheck.py 七判据、pyproject.toml 0.1.0）
- **实现位择一**:独立工具非 lease 子命令——理由三：(1) 对表只读零写，入 lease 写路径工具即混淆呈报与治理写操作；(2) 挂点在会话启动例行读数旁，与 gauge/selector 同形即独立工具，零会话参数依赖，租约 open 前后均可跑；(3) 锁面读 ndjson 镜像即可见性取数（SQLite 只读在 WAL 活跃下 macOS 实测三连无法开库，gauge gqueue 先例同形），不必 import lease 内部件
- **BATCH-FACE**:「watch 对表挂点与处置协议」节（调用形与零 token 二值协议与回滚操作指引在档）
- **检词登记**:五词条 register 通道四查通过（稽/Watchcheck/watchcheck；呈报/Report/report；声明面/Declaration Face/declaration-face；无主清单/Unowned List/unowned-list；豁免面/Exemption Face/exemption-face；无主与 watch 随词条承载不单独立词），双面逐字节一致，主树与工地 nomenclator 测试族各 30 绿（canonical 在内）
- **材料**:watchcheck-solo-materials/first-run-2026-09-05/（活跑双跑读数）与 facet-round1/（一裁合同与回填与计分与闸复算）

## 三、活跑实测与无主清单呈报（候人节点二值裁决）

对表读数 2026-09-05（v0.1.0）活跑：脏文件 1177 件，锁面现势 17 面，声明面 8 件，豁免面 16 面，**无主修改 18 件**：

- sih-tools/lease 测试夹具九件（fixtures-intent-closeguard.json、fixtures/close-failures/ 三件、duplicate-run、golden-run、test-local-changes、test-modified-deny、test-untracked）——历史批测试产物未入版控，候裁：入版控或删除
- sih-tools/lease/pk023spec-solo.md、sih-tools/lease/scripts/poll_shares.py——历史遗留件，候裁
- sih-tools/m-wengui-fit-1-check.json、sih-tools/m-wengui-fit-1-signcheck.json（repo 根散件）——docmath-namefit 批执契散件落根，候裁：归位 tally/reports 或删除
- sih-tools/meter-quiet-probe.txt——facepark-solo 探针散件，候裁
- sih-tools/nomenclator/packs/core/.-tmp-entry-programslicing.json——临时登记残留，候裁
- sih-tools/wikirecall/verify_triggers.py——历史遗留件，候裁
- sih-tools/worktrees_placeholder——占位件，候裁

**零代行声明**:本批对上述清单零回滚零清理零代裁，全部候人节点二值（「我的」通道、「不是我的」机械回滚，操作指引见 BATCH-FACE 处置协议节）。

**在途判定实录（packhyg 条款执行）**:会话启动时主树 gauge 八件改件（cli.py 与 pyproject 与 __init__ 与两金向量与 test_contrib 与 CONTRACT）无锁无声明无豁免，机械判定无主；按「不以存在活动会话推断在途」实查锁台账与 mtime（23:10-23:14 活跃写入）判为或正在途，只呈报不代裁不改面；例行读数不可从被破面工作树跑（NameError: K_SIGMA_MISSING——工作树 gd-2 半成品引用 K_SIGMA_DEFAULT 而仅定义 GC_K_SIGMA_DEFAULT），改走 HEAD 物化形（git archive HEAD gauge 卷出只读副本）落链；后经实证该改件由并行批 gaugecontrib-gd2-solo 经其收约入主树（e00edccd），判例收档：watch 沉默期无主活写实际存在，机械对表读数与最终治理归位（批收约）的差值即本工具的存在理由。

## 四、F-5 一裁 round1 实录与候裁事项

- 合同:m-watchcheck-judge-1（ng medium n 9，seat ZCode:GLM-5.3:self-reported，出题半零 LLM 零网络）
- 回填:九发原文回填（materials/facet-round1/responses.jsonl）
- 计分:9/9 comply，变卦 0%，谨慎信号 0/9，规约引用三类（baseline_1×4、baseline_4×3、baseline_5×2）
- 闸裁决（判据 v3）:**boundary**（打回重作）——failed criteria: basis_consensus（distinct_basis 3 > 1 硬共识）；decision_stable 与 boundary_low 与 foregrounding_stable 全真
- 根因申报:basis 三值轮换系回填方法论失误（人工多样性），非命题刀锋（boundary_flag 0/9 即席位未判任何边界性）；distinct_basis<=1 为硬共识，同 gid 补采按全历史汇池不可收敛，新 gid 重测即「重写上下文」位
- 候裁事项（用户二选一）:(a) 裁回填方法论失误成立、授权新 gid 重测（如 m-watchcheck-judge-2，回填依单基线集中）；(b) 裁命题上下文须人重写，按刀锋类处置。agent 不代写不重滚，执契（attractor check + sign）候裁后行
- 纪律依据:facet 判据 v3 boundary = 打回重作（刀锋类须人重写上下文，agent 不得代写）；任务包 F-5 near_threshold 呈用户，boundary 从严同理

## 五、管线与链（close 后回填收口读数）

- 泊界心跳:selector route 双目录退出码 0/0（tools 19 件零告警、engine 35 件零告警）
- 例行读数:gauge record ga-2 三维落链（HEAD 物化形，偏离申报见第七节）
- 三问双门:scrutinator ask3 包 0 findings + ask3repeater ok（三锚 PRO-08 应几 L13、PRO-07 鉴 L55、PRO-08 应而不藏 L52，程序切片逐字节）
- 叩问:elicit check 六轻信号 + digest passed covered 6
- 正身:identity verify anomalies 0（identity_hash f536ad565f15…）
- 租约:open 445c12fbb3e4283c，allow 十一路径含 facet/contracts/watchcheck-260905/；锁 7 独占 + trail append（与并行批共存）
- 意图:e96d3f44 / b9808ea8 在链，envelope session=445c12fbb3e4283c identity_hash=f536ad56…（裸调 grep 逐笔验证）
- 管线三步与认证:见认证清单
- settle/close/reconcile/verify:收口读数 close 后回填

## 六、越线与误差申报

1. **例行读数 HEAD 物化形偏离**:主树 gauge 工作树被并行批（gaugecontrib-gd2-solo）在途改件破面（NameError），例行读数改自 git archive HEAD 只读副本跑，命令形与参数不变；落链三笔 reading_recorded 属实。canonical 调用形恢复待 gauge 批自身收口
2. **facet 工具内 trail 落 allow 域外**:计分半 flywheel trail 落 sih-tools/proposition/DES/m-watchcheck-judge-1/（facet 工具自身台账位，非本批 allow 面；承 m-idenlane-pen-1 同形如实申报）
3. **首次 open 两连撞**:allow 面含并行批独占的 sih-tools/scribe/CALL-LOG.md 撞开工预检（open_precheck_conflict），按 idenlane-envelope 先例排除撞路径重开；再撞同包活跃窗口（同包检验文件自残留，session_id null），清零后重开成功（idenlane-envelope §七先例同形）
4. **本批零锁零提交首会话拆除让位**:allow 面漏 facet/contracts/ 致拆会话 1d463181f4bf7570 重开（deyimerge 零锁零提交让位先例同形）；其 close 触发 closeguard pre-close 工作树提交 e9dc3cfb（台账 ndjson 两件，批名标签取活跃包名），机制形如实记档
5. **命题稿落位偏离**:topic.md 落 facet/contracts/watchcheck-260905/（allow 面不含 proposition/topics/，与 m-idenlane-pen-1 的 topics 落位异形），合同 meta.measurement_entry 指向同目录，验收线「得一裁材料在 facet/contracts/」满足
6. **CALL-LOG scribe 笔候位**:sih-tools/scribe/CALL-LOG.md 与 lease/CALL-LOG.md 被并行批 idenlane-human-solo（be316fe3596bf16e）独占，本批 scribe CALL-LOG 笔收约时候位或走 bypass 补笔先例形

## 七、关联

- 任务包:sih-engine/sih/state/plan/watchcheck-solo.md
- 协议裁定:sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 第八节
- 前置批:idenlane-envelope-solo（声明面 direct_edit_completed 来源，主树 a099478）
- 承载泊件:pk-059（病灶二首账；出泊 promoted 1186efa3 由 viewline-solo 承办，本批为承载面首账交付，出泊材料 pk-059-exit.json 在泊位档案）
- 工具面:sih-tools/watchcheck/（CONTRACT.md 判定语义正典）

## 八、收口读数（close 后回填）

- 双仓 settle:tools eaa95689 / engine f268d32（段 1 单段，cert 9a44ab2d / 2ea30651 在链）
- close:445c12fbb3e4283c revoked true，双工地与分支双拆归并（tools merge 17ec7a34 入 integral-stage-build，engine merge 0b71f3e 入 main），双仓零锁
- reconcile:双仓 unrouted 0；tools cert_missing 1（e6880a63/526e2be，2026-09-03 entryunique-solo 批陈项，批前已有零新增）；unbypassed 61/31 为历史累积非本批新增
- 链 verify:valid 134 事件，首 29c7dd29 尾 9a44ab2d
- 收约补笔:本节与 scribe CALL-LOG 笔经 bypass 通道入版控（facepark/packhyg/idenlane-envelope 先例同形）
