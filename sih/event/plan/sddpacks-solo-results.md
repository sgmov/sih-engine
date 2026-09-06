# sddpacks-solo 结果档：SDD 五件格式包族成文批

> 批：sddpacks-solo（文规向界格式包族批，纯数据零代码；委外单线 solo，零子代理）
> 会话：73793a2beef9b433（lease 1.30.0，scope_source package）｜日期：2026-09-07
> 承接：用户 2026-09-07 令转发委外提示词；两道门实装批因门轴在租约而延后，格式包族提前

## 意图锚定

- 意图事件：intent_refined `93f9fa1c-a834-49d7-a196-d6ed63392efa`（event_hash `62ca79ce`）
- record：sih-tools/scribe/reports/2026-09-07-ask3-sddpacks-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-07-ask3-sddpacks-solo-validation.json（ask3repeater status ok 三锚）
- 三锚引文程序切片（03-on-second-tao.md L15、07-on-assay.md L69、08-on-settle.md L52）

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 零违规；引擎 ask3repeater status ok anchor 3。
- 叩问：elicit check exit 1（16 信号全轻）＋digest exit 0 passed covered 16/16。
- 正身：identity attest anomalies 0（identity `1240b2e2`、core `82f460c2`）。
- watch 对表：exit 1 无主 1 件即 billwire 残件（既往在案候人节点二值裁决），零代行零触碰。
- 泊界心跳两线：tools exit 0（22 件：21 mainline＋1 siding）、engine exit 0（52 件：46 mainline＋6 scrap_track），告警零。
- 例扫（exscan 挂点）：rev3 双跑 exit 1/1 一致 findings none、cmp 四路 IDENTICAL；checkmath exit 0 verdict zero_drift（reds 0、greys 6 灰项声明滞后只记不代修）。
- 例行读数：gauge record 三维落链 convergence 0.0／adoption 0.0／mergeback 0.038462（events 89f24da4、826483db、f205d261）。

## 件读数：包族落位

| 件 | 内容 | 判定面 |
|---|---|---|
| manifest.json | 包族声明：五包、五样例、self_validator（退役声明）、out_of_scope 三项范围外申报、pending_notes 只注记 | 结构校验 |
| change-proposal.json | 变更提案包：四节位识别面 | CP-001 至 CP-006 |
| spec-delta.json | 规格差分包：增量三分类＋零增量申报位 | SD-001 至 SD-006 |
| scenario-list.json | 场景清单包：三刀形（编号唯一、判据行三要素、独立成件） | SL-001 至 SL-007 |
| tech-design.json | 技术方案包：回链识别面＋引用可达 | TD-001 至 TD-005 |
| task-list.json | 任务清单包：引用＋证据位 | TL-001 至 TL-006 |
| fixtures/ 五件 | 自反狗粮样例（以本批变更为对象） | validate 全绿 |
| validate.py | 自携校验脚本（标准库零依赖、确定性输出、退役声明） | 双跑逐字节一致 |

落位：sih-tools/incubation/packs/sdd-v1/（围堰期家，检查器立名建目录后一次迁移）。

## 一裁读数

- 命题：m-sddpacks-pack-1（判定面条款逐条满足单锚即可验证性且族自洽样例闭包真闭）
- facet：emit-contract 零 LLM 零网络；回填九发同席即席作答（谱系披露双声明载 topic authored 行）；score 9/9 comply、变卦 0%、谨慎信号 0/9、规约引用 baseline_1/4/5 三类
- seat-baseline：ledger 尾行为 09-06 席位行，按 regulamath 先例正形手工制当日基线（当日正身 identity 1240b2e2／core 82f460c2 配对）
- tally assemble：gate_verdict stable_clear
- attractor check：verdict pass、disposition 裁决通过、passed 12、failed 0（R1 至 R7，r1 材料跳 R8）
- attractor verify：identical
- attractor sign：signed，event f96ea37e（doc_id crosscheck-m-sddpacks-pack-1）

## 管线读数

- 化格：json-canonical-v1 对六 JSON 两轮归一（首跑 exit 1 六件、重建后复跑 exit 1 六件，终态归一形）；general-v1 对五样例 exit 0 零改。
- 核阅：des-001 对十一件全 exit 2 域外如实记档（域只盖 sih-engine/doc）。
- 检词：core 首跑 exit 1（manifest 内"文规检查器"lazy_in_doc 一处）改表述后 exit 0；重建后复跑 exit 0 零违例。
- validate.py：首跑 exit 1（脚本内一行残渣调用 TypeError）红证归档 materials/validate-red-evidence/；修复后双跑 exit 0/0 cmp IDENTICAL verdict pass；重建后复验同绿。

## 事故申报（本批最大申报项）

- **事实**：清理误建的嵌套 sih-tools/ 前缀位时，把当时落位于嵌套位的包族十二件一并误删（建件时写入工地嵌套前缀位，而本仓工地内容直出工地根，嵌套位非正位；rm 时误判嵌套位为纯错径）。
- **补救**：正位 incubation/packs/sdd-v1/ 原内容重建十二件（内容执行代理全量持有），化格归一、validate 双跑绿、检词零违例全数复验。
- **影响**：零主树影响、零他批影响、零链影响；终签材料（topic、contract、tally-material）在直出位未受牵连，哈希链完好。命题文内"sih-tools/incubation/packs/sdd-v1/"含一笔嵌套前缀路径字样，归并后实际落位与设计一致。
- **红证**：本申报与批 materials 全读数即红证，先红留痕纪律适用。

## 越线与误差申报

1. 闸三坑（已处置）：首次 scribe intent 缺 --sessions 被闸拒（exit 2 零留痕），补参重跑过（BATCH-FACE 勘误在案）。
2. facet score 路径坑（已处置）：score 从 facet 目录跑致 responses_path 相对形错位，assemble 择件报"响应哈希不一致"；按 gvec 勘误重跑 score 携绝对路径后过（responses 未变，哈希 afbe3db4 前后一致；飞轮 trail 按命题累计不覆盖首跑，先红留痕纪律适用）。
3. attractor verify 首跑 divergent（已处置）：check 传绝对路径、verify 传相对路径致 report 内 material 字段不同形；同参形（双绝对路径）重跑 identical。首跑 divergent 输出即红证在案。
4. R5 首跑挂起（已处置）：基线携 09-06 正身哈希跨日不一致；按 regulamath 先例手工制当日 seat-baseline 重装配，check 裁决通过。
5. 红证 traceback 非 JSON 被认证闸拒（如实申报不硬闯）：validate 首跑红证留 materials 归档随批入版控，不另走链认证。
6. settle 首跑 nothing_staged（已处置）：漏 BATCH-FACE §9 第一步 git add，补 stage 重跑双仓过。
7. unlock 首跑缺 --identity exit 2（已处置）：补参十一锁全放（archpark 先例同形）。
8. tally-material 装配与 check 与 sign 等工件写主树 DES 单元格与合同目录（直出位即工地内，非越线；facet/contracts 与 proposition/DES 均为 allow 面内追加活面）。
9. 例扫 rev3 exit 1/1 双跑一致 findings none（工具语义退出码，非违规；checkmath zero_drift 红零灰六，灰项既往申报同款）。

## 认证清单

| 件 | 类型 | 认证 |
|---|---|---|
| intent_refined | 意图 | `62ca79ce` |
| crosscheck-m-sddpacks-pack-1 | 得一终签 | `f96ea37e` |
| pipeline-report-2026-09-07.json | 管线报告 | 链 append |
| changed-files-2026-09-07.json | 变更清单 | 链 append |
| m-sddpacks-pack-1-signcheck.json | 重放锚 | 链 append（尾笔 `706f96a4`） |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 五包落位 | 工程 | sdd-v1 目录 manifest 加五包在位，schema 三面逐包齐 | 通过（正位十二件全在位，包 schema 三面齐，规则 id 全唯一、三态定位全合法） |
| F-2 样例过校验 | 工程 | validate.py 跑绿，双跑逐字节一致 | 通过（exit 0/0、cmp IDENTICAL、verdict pass、findings 0；重建后复验同绿） |
| F-3 终签在链 | 治理 | m-sddpacks-pack-1 stable_clear 终签入当日链 | 通过（f96ea37e，verify identical，doc_id 在链） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（含事故件误删自建位即本批 allow 面内工件，重建落正位；零 sih-math、零租约源码） |
| F-5 委外不越权 | 编组治理 | 零代码改动、零人节点代行、零 plain commit、待裁项只注记、遗留零触碰 | 通过（validate.py 为批内脚本落批目录；closeguard 兜底一笔同款同形如实申报） |

## 收口读数

- 双仓 settle：tools `fba51e0b`（归并 `b0e43e8a`）、engine `6c2238a`（closeguard `817c67d`＋归并 `700d177`）；cert 取 `706f96a4`。
- 放锁收约：十一锁 unlock 全放（首跑缺 --identity 补参重放）；close exit 0，会话 73793a2beef9b433 revoked，零活跃锁。
- 链 verify：2026-09-07 当日链 valid，last_hash `706f96a4`。
- reconcile：双仓 unrouted 0；engine cert_missing 0；tools cert_missing 1 与 unbypassed（tools 79／engine 47）与 session_orphan 为历史累计账面项，本批贡献即 closeguard 兜底一笔（engine，今日各批同款同形），相比批前零新增。
- 收约补笔：本结果档经 bypass 通道入版控（redkeep／regulamath 先例同形）。

## 邻件与悬案对表

- pk-061 包容器格式、检查器名脉：manifest pending_notes 只注记不预判。
- 译段工具：不在本批（候用户第二席裁决）。
- 检查器实装批：可拿本族五包与样例先红后绿（先包后机达成）。
