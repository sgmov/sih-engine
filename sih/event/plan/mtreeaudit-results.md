# mtreeaudit-solo：主树直写七笔审计与补证追认结果档

> 承用户 2026-08-31 主树直写修正令，对 scrutmerge-sdd-solo 与 viewimpl-solo 两批七笔直接落于 sih-engine 主树、未经租约副本的提交做机械审计、内容完好性核查与补证追认
> 队形：单线形 solo 即主线亲写零子代理
> 日期：2026-08-31

## 概览 {#overview}

- 七笔机械审计毕：scrutmerge-sdd-solo 四笔与 viewimpl-solo 三笔，逐笔内容清单、任务包状态、链上事件笔数在表::[audit](#audit)
- 内容完好性核查：SPEC-013、GOV-003 v1.4 注记与 src/view 首实装内容完好即引擎测试全绿、核阅化格检词三工具双文档全过、viewer 在位，两批各自链上各有一件核阅退出码二件如实记::[integrity](#integrity)
- 补证批走正门：租约双仓立约收约、二十三事件上链即意图一件与追认证认七件与管线认证九件与完好性认证六件、全经 meter 包裹、提交指副本、收约归并::[ratify](#ratify)
- 对表前后与根因：追认不洗白只补账，七笔分类以实际为准仍 unrouted 如实留，裸 git commit 绕过 lease 拒直提守卫的根因与候选机械位入第六节，裁定归用户::[rootcause](#rootcause)

## 一、七笔审计表 {#audit}

| 序号 | SHA | 批与段 | 本地时刻 | 内容清单 | 任务包 | 本笔链事件 |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | f616b07 | viewimpl-solo 视图组件首实装 | 08-31 08:36 | src/view 四件与 bin/viewer.rs 与 lib.rs 与 GOV-003 v1.4 注记与链 12 行 | 未入仓即 untracked 从未提交 | 12 行即 3 读数与 1 意图与 2 跨核与 6 认证 |
| 2 | 369a80b | viewimpl-solo 主树直写 | 08-31 08:36 | GOV-003 v1.4 注记与链 12 行即与 f616b07 的链 12 行逐字节一致 | 同上 | 同 12 行即两平行提交同内容 |
| 3 | a064652 | viewimpl-solo 段3 结果档 | 08-31 08:38 | viewimpl-solo-results.md 126 行 | 无 | 0 |
| 4 | cd08335 | scrutmerge-sdd-solo 规格先行批 | 08-31 08:56 | SPEC-013 234 行与任务包 119 行 | 已入仓即本笔 | 0 |
| 5 | 4ccd4b5 | scrutmerge-sdd-solo 段2 结果档 | 08-31 08:56 | scrutmerge-sdd-solo-results.md 62 行 | 无 | 0 |
| 6 | 1605b25 | scrutmerge-sdd-solo 主树直写 | 08-31 08:57 | 前三件同一文件集 415 行即 SPEC-013 与任务包与结果档 | 已入仓即与序号四重复 | 0 |
| 7 | e6556f3 | scrutmerge-sdd-solo 段3 结果档入主树 | 08-31 08:57 | 结果档加 1 行即偏离表行自述主树直写三笔与归并一笔未带 session 号 | 无 | 0 |

提交图结构：f616b07 与 369a80b 同父 a2561b0 为平行提交，经副本归并 a39e353 收拢；a064652 在其上。cd08335 与 4ccd4b5 同父 a064652 为链式提交，1605b25 同父 a064652 为平行提交，经副本归并 18a72af 收拢；e6556f3 在其上。

## 二、批会话台账与链上事件 {#chain}

- viewimpl-solo 会话 7818365414442c82，issued 08-31 00:29:30Z，close_failed 四次含 merge_failed 两次即主树 GOV-003 与 trail 本地变更挡归并，revoked 00:36:39Z；链上事件 12 行即 reading_recorded 3 与 intent_refined 1 与 crosscheck_completed 2 与 certification_completed 6，六认证中 scr-vipkg 一件退出码二
- scrutmerge-sdd-solo 会话 fb0e23407bf759f2，issued 00:52:59Z，revoked 00:57:09Z；链上事件 7 行即 intent_refined 1 与 certification_completed 6，六认证中 scr-pkg 一件退出码二，此 7 行经 pkgclose 批 f382dd8 提交入仓
- 两批结果档均自述偏离：viewimpl-solo-results.md 与 scrutmerge-sdd-solo-results.md 偏离表行均在档，即偏离披露在先、本批审计在后

## 三、内容完好性核查 {#integrity}

- 引擎测试：cargo test 全量 102 测全绿即 lib 86 与 viewer 5 与集成 2 与记忆召回 9，退出码零
- 核阅 des-001：SPEC-013 退出码零零发现，GOV-003 现行 v1.5 含 v1.4 注记行退出码零零发现
- 化格 general-v1 加 json-canonical-v1：SPEC-013 退出码零，GOV-003 退出码零
- 检词 core：SPEC-013 退出码零，GOV-003 退出码零
- viewer 二进制：在位，用法串即 alarms 与 heartbeat 与 settle 三子命令，单测五件全绿
- 结论：两批实质内容即 SPEC-013、GOV-003 v1.4 注记、src/view 首实装与 viewer，内容完好，不因流程违规判废；流程欠账即七笔未带 session 号与任务包未入仓，由本批补证追认处置

## 四、补证追认 {#ratify}

- 立约：会话 93dba031db379456，lease 1.9.0，双仓副本即 sih-engine 自 main 与 sih-tools 自 integral-stage-build，请求写入净路径六件，意图件三锚点双腿验收过即 ask3 零发现与 ask3repeater 通过
- 意图事件：e594a895 入链即 2026-08-31 链第 56 行
- 七笔追认证认逐笔入链：f616b07 对 7fed4a87、369a80b 对 464bed5e、a064652 对 9f84bdd9、cd08335 对 96910176、4ccd4b5 对 247771b0、1605b25 对 e7949dc1、e6556f3 对 9c8233ed，各件报告含逐笔出处与审计结论与内容完好判定，落 scribe/reports 2026-08-31-rtf-cert- 七件
- 管线认证九件入链：fmt-pkg 1e9e8531、fmt-res 95d5df92、nom-pkg cc64c080、nom-res 4b0666e1、fmt-res1 fcdc627b、nom-res1 fa633e95 六件退出码零，scr-pkg 78e19c62、scr-res be2ec199、scr-res1 1b8ebbe4 三件退出码二即 des-001 域含 sih-engine/doc 而 plan 文档在域外，与两批先例同款如实记；其中 res 三件为结果档首版，res1 三件为填充版复验，终版复验三件报告入仓即 scribe/reports 2026-08-31-rtf-fmt-res2 与 scr-res2 与 nom-res2 三件，退出码零与零与二，零发现
- 完好性认证六件入链：scr-spec013 8176f348、scr-gov003 87da5bf3、fmt-spec013 10a2b3bd、fmt-gov003 ef341804、nom-spec013 98e06965、nom-gov003 869390af，六件退出码零零发现
- meter 包裹：25 次调用即 23 件 append 与 2 件 verify，计数落 meter/counts 2026-08-31
- 链：本批 23 行与他会话秤星读数 3 行交织即 2026-08-31 链 55 行至 81 行，逐行归属以链上会话标识为准，verify valid 81 件，首件 2ff3b6cb 末件 fa633e95
- 追认语义：追认不洗白只补账，链侧已备七笔逐笔出处与审计结论与内容完好认证，对表分类不因此改判

## 五、对表前后 {#reconcile}

- 补证前即引擎显式起算 a2561b0 至现头 19 件：七笔全 unrouted、routed 10 件含 routed_merge 4 件、另有 unrouted 2 件即 1a778f1 与 88c6b4a 为 viewrider 与 viewfix 批新残迹
- 引擎缺省对表即 1.9.0 封窗界 22550a6：窗口 7 件，routed 5 件，unrouted 2 件即上述两件新残迹，七笔在窗外不计
- 补证后复跑即 20 事件上链后：七笔消息无 session 行，分类机制不变，显式范围内仍 unrouted，缺省界仍窗外，分类侧维持实际、链侧补账，两侧均如实留
- 工具侧缺省对表：154 件，routed 145 件，cert_missing 8 件即链事故存量，unrouted 1 件即 f24248d 为 viewrider 工具侧残迹

## 六、根因与候选 {#rootcause}

- 根因：lease 1.8.2 拒直提守卫位于 lease commit 命令路径，裸 git commit 在主检出直接执行即全在 lease 管辖之外，守卫不触发。两批七笔皆裸 git commit 于主检出
- 候选机械位一：主树钩子拒裸提，主检出设 pre-commit 钩子，非 lease commit 通道注入的会话标记即拒裸提，lease commit 路径放行
- 候选机械位二：引擎侧收 lease commit 为唯一通道并令对表输出对无 session 行直提件加专门机械标识，使此类残迹在对表输出中可机械分辨
- 裁定归用户，本批不动任何钩子与契约

## 七、后续 {#followup}

- 根因裁定归用户，候选机械位见第六节
- viewimpl-solo.md 与 viewimpl-resolo.md 两件任务包 untracked 残件与主树 task-packages 目录残件，属他批清理范围，本批不扫
- 1a778f1 与 88c6b4a 引擎侧两件与 f24248d 工具侧一件为 viewrider 与 viewfix 批残迹，属该批验收范围，本批只如实记录
- 结算提交与归并 SHA 见会话台账与 git log，本档不自引用
- 补证 trail 含 3 行 viewrider 批在基线提交后未入库行，随本批归并入仓，同款即 pkgclose 批 f382dd8 扫入 scrutmerge 7 行先例，逐行归属以链上 session 标识为准
