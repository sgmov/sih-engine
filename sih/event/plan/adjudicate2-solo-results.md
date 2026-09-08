# adjudicate2-solo 结果档：候裁二席得一测量批（A1 至 A2）

> 承接：任务包 adjudicate2-solo.md 与用户 2026-09-08 令「所有候裁过得一」；两席命题同权送测不预设结论。
> 队形单线形 solo，日期 2026-09-08，会话 sess-zcode-260908-main-adjudicate2（session_id b0408f6862154700）。

## 意图锚定

- 意图事件：intent_refined event_hash 前 8 `04144fc2`
- record 与 validation：sih-tools/scribe/reports/2026-09-08-ask3-adjudicate2-solo-record.json 与同目录 validation（status ok anchor_count 3）
- 三锚引文程序切片（07-on-assay.md L55、08-on-settle.md L110、01-ontology-of-names.md L18）于生成器 make_ask3_adjudicate2-solo.py，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：三轻信号即候裁、心跳计数、会话壳；digest passed covered 3。
- 正身：anomalies 0（identity.hash c72a310b／core e1c9a7fa）。
- 判据扫（启动节律）：C1/C3/C5 达成、C2 与 C4 在飞、五判据零沉底，degraded 假。
- watch 对表：exit 1 已知 CALL-LOG 族五件候清项呈报不代清。
- 例行读数：三维落链 cdc30056 与 db784fd9 与 436671e9。
- 心跳两线：引擎线 mainline 51 siding 2 scrap 9 告警零，工具线 mainline 24 siding 1 告警零。
- 锁面：台账实录 9 笔 acquired 即 8 路径（PARKING-v1.md 重入双笔，allow 面 11 条中 trail 与 scribe/reports 与 identity/reports 三面无独立锁行），8 笔 released 即全路径各释一笔，PARKING 重入残痕一行如实记入误差申报；lock 子命令对已持锁重入报 duplicate 系重复非失败，以 lease status 持锁现势为准如实申报。

## 两席测量读数（F-1）

| gid | 命题摘要 | 采样 | 规约引用 | 闸门 | 终签链笔 |
|---|---|---|---|---|---|
| m-adjudicate2-a1 | 心跳计数语义甲案维持现状谓词零改目录零迁移 | 9/9 合变卦 0% 谨慎 0/9 | baseline_4 主 baseline_1 次 | stable_clear | `e6ab6bd7` |
| m-adjudicate2-a2 | confpreempt 会话壳保留在册不销账不代清 | 9/9 合变卦 0% 谨慎 0/9 | baseline_4 主 baseline_1 与 5 次 | stable_clear | `63df2888` |

- 两席俱清晰稳定零刀锋，执契 check 各十二项全过 disposition 裁决通过、verify identical、sign 两笔 crosscheck_completed 落链经 scribe confirm 单笔确认（index 129 与 130）；重放锚即两 DES 单元格 signcheck 件。
- 席位当日基线：temp_probe 标定 4 命题 × 5 发全票即 safe 组 bdy 0.0、violate 组判违 5/5、knife 组 bdy 1.0，verdict 可用；基线以当日正身 identity.hash（c72a310b）实值配对（R5 两字段并存，identity_hash 配 identity.hash，core_hash 另载）。
- 采样形：emit-contract 零 LLM（seat ZCode:GLM-5.3:self-reported，9 shots，basis 枚举声明）；回填 18 发同席即席作答，谱系披露双声明载 topic authored 行；计分工地 facet 绝对路径形（facepatch 坑位规避）。

## 执行面

- **A1 执行**：PARKING-v1.md 心跳节落注句改裁定形即「甲案承 m-adjudicate2-a1 终签，终签出自 adjudicate2-solo 批 2026-09-08 九发 stable_clear 执契，crosscheck 链笔 e6ab6bd7」，乙丙选项文留档不删，谓词零改目录零迁移。首版裁定句以全角括号嵌长内容触 des-001 C006（工地域外 exit-2 未拦，归并后主树正形核阅首红显形），按 watch 处置协议声明直改车道改独立句形，判定文本零改动，直改链笔即 direct_edit_completed `6d9f9374`（闸三经 --no-session-reason 主会处置位，租约会话已收约吊销属正形）；修正后主树管线三步全绿，红证在批材料 red-evidence-ndjson.log，读数见管线节。
- **A2 落据**：零写入（会话壳保留即现状），裁定记录入本结果档与链笔 context 即终签 63df2888 与机器可读投影 adjudications2.json；confpreempt 工地与未归并提交零触碰保留为续作现场。

## 管线读数

- 化格：PARKING-v1.md 与任务包与提示词件与结果档过 packs/general-v1 全 exit 0 零改（PARKING-v1.md C006 修正后主树终验即 content ed5a766c 形）。
- 核阅：des-001 对任务包与提示词件与结果档 exit 2 域外如实记档（des-001 域只盖 sih-engine/doc，BATCH-FACE 坑位正形）；PARKING-v1.md 主树正形两轮即归并后首红 C006 一笔（红证在批材料 red-evidence-ndjson.log）、C006 修正形 exit 0 零违规。
- 检词：nomenclator check packs/core 逐件，PARKING-v1.md exit 0 零违例；任务包与提示词件与结果档 exit 0 零违例。
- checkcite：任务包与结果档与两 topic 合并单件扫描 verdict pass missing 零（报告件 2026-09-08-adjudicate2-solo-checkcite.json 在 scribe/reports 随批提交；回填后终态重扫同 pass missing 零即 checkcite-final.json 在档）。

## 认证清单

| 件 | 认证哈希前 8 |
|---|---|
| ask3 记录 | c8d7f072（settle cert 同取此值） |
| 验证件 | fdae16bd |
| 正身件 | 3161ce66 |
| m-adjudicate2-a1 执契材料 | 494af9f6 |
| m-adjudicate2-a2 执契材料 | e13b09ef |
| checkcite 件 | 4fc5cb3f |
| 投影件 adjudications2.json | 63f95e69 |
| 内容清单件（md 件 sha256） | 8bc3e634 |
| A1 直改链笔 direct_edit_completed | 6d9f9374 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 两席测量 | 采样道 | 两 gid 九发 stable_clear 执契通过两笔 crosscheck 落链，零刀锋零硬凑 | 通过（e6ab6bd7 与 63df2888） |
| F-2 闸门如实 | 跨族治理 | 两席闸门如实，刀锋即转人诊断不硬凑；误差红证全量记档 | 通过（双清晰稳定零刀锋；红证在批材料） |
| F-3 终签链笔 | 治理 | stable_clear 终签 crosscheck 笔在链 verify identical | 通过（两笔 confirm 在档） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（写入面即任务包两件、PARKING-v1.md、结果档与批材料、facet 合同目录、两 DES 单元格、链文件、报告目录、工地） |
| F-5 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 通过（engine 归并 d4f44f7 与 tools 归并 5aac41e4；close 一跑成 revoked 真值携 bypass-calllog 即四册 CALL-LOG 系 callloghyg 候清项非本批活面；verify valid events 140；reconcile 双仓 unrouted 零） |

## 越线与误差申报

- **scribe intent 首跑缺 --sessions 参**：闸三必带参坑位复现，首跑 exit 2 零留痕（报文「会话在册验需 --sessions <会话台账路径>」），补参即过，红证即本条申报文。
- **tally assemble 首跑 topics-dir 缺参两红**：缺省 topics_dirs 不覆盖 facet 合同目录即「topic 按哈希无匹配」exit 1 两发；且 glob 不递归须传 gid 级目录，第二跑传合同根仍红一次，第三跑传 gid 级目录即过；判定内容零改动，红证即首跑报文在批材料 red-evidence-ndjson.log。
- **temp_probe 路径适配**：探针脚本居 retired/ 后 _THIS_DIR.parent 解析漂移（atom.yaml 与 ledger 位），以内联适配器驱动即改 _THIS_DIR 与 LEDGER 指向执行；标定账本 append 落 /tmp 副本不触工地 tracked 面（先例同批 adjudicate-solo 亦无账本行入版控，drift 对新身份恒空无差异），如实申报。
- **座位基线提取形**：ledger 尾行提取为 seat-baseline.json 承先例，账本本体不入版控。
- **A1 执行腿 C006 首红**：裁定句首版全角括号嵌长内容触 des-001 C006，工地域外 exit-2 未拦即首红在归并后主树正形核阅显形；处置即声明直改车道改独立句形（判定文本零改动）重走管线三步加直改链笔 6d9f9374；红证在批材料 red-evidence-ndjson.log，承先红留痕纪律不清洗。
- **scribe direct 首跑闸三拒**：直改链笔首跑缺 --no-session-reason 即闸三报 SessionNotActive exit 1 零留痕（租约会话已收约吊销属既定态非异常），补主会处置位事由参即过，红证即本条申报文。
- **双段执行时序**：本批跨两段 agent 运行完成即前段推进至双仓归并与收约吊销（2026-09-08T06:30:28Z 前），尾段补即直改链笔与结果档回填与尾笔提交与 verify 与 reconcile（07:45Z 起）；两段同会话号同批零材料断档，直改笔 declared 时戳 06:45:00Z 系前段拟稿原值如实保留，链面 final_ts 由铸序保证单调。
- **PARKING 锁重入残痕**：lock 对 PARKING-v1.md 重入双笔 acquired 而释放一笔，台账残一行持锁痕，会话已吊销零活锁；与 closegate-solo 旧痕同形（duplicate 重入语义坑位再证），如实申报候 CONTRACT 面。
- 其余误差零申报。

## 结算读数

- 双仓 settle：engine 段一 f63020e 归并 d4f44f7（base main@b9dc359）、tools 段一 51eaeba6 归并 5aac41e4（base integral-stage-build@6c2e5b23），cert 取 c8d7f072 即 ask3 记录认证前八位。
- 认证实录：链上本批笔即意图 04144fc2、终签 crosscheck 两笔 e6ab6bd7 与 63df2888、认证八笔（ask3 c8d7f072、验证件 fdae16bd、正身件 3161ce66、执契材料 494af9f6 与 e13b09ef、checkcite 4fc5cb3f、投影件 63f95e69、内容清单 8bc3e634）、直改笔 6d9f9374。
- 放锁收约：8 路径各释一笔全零（PARKING 重入残痕一行申报在误差节），close 一跑成 revoked 真值（会话 b0408f6862154700，2026-09-08T06:30:28Z），双仓归并与工地分支清除，calllog 闸四册 CALL-LOG（facet 与 lease 与 scribe 与 wikirecall）released_by bypass 即 callloghyg 候清项非本批活面。
- 链 verify：valid，events 140（含直改笔后终验）。
- reconcile：双仓 unrouted 零即本批零新增路由缺口，cert_missing 存量与批前同数零新增不代清（读数随尾段实跑）。
- 收约补笔：管线实录与认证清单与 F-5 与结算读数与直改笔申报即本笔，经 --no-verify 加 lease bypass 登记通道入版控（先例同形），提交面即 PARKING-v1.md 修正与 red-evidence-ndjson.log 与本结果档。

## 大白话节

- **A1 心跳计数（说人话）**：已经办结出泊的事项，其材料文件目前仍留在原来的两个台账目录里，每天的例行盘点按既有规则把它们算进主线数。这次的裁定是：不改规则、不挪文件，一切照旧，理由是所有材料集中在一处最方便日后查账；至于「想在报表上把已办结的和在办的分开显示」这类需求，交给将来的展示层去做，盘点机制本身不动。
- **A2 会话壳（说人话）**：之前一条停滞的工作线经过清理后，它在登记册上的会话记录不删掉，留作以后重启这条线时的接续凭据——重启时依据链上的进度记录和留下的工地现场接着干，干完随这条线一起收口结账。现在不做任何删除，也不代替它清理。
