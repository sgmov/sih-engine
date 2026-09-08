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
- 锁面：11 锁全持（lock 子命令对已持锁重入报 duplicate 系重复非失败，以 lease status 持锁现势为准如实申报）。

## 两席测量读数（F-1）

| gid | 命题摘要 | 采样 | 规约引用 | 闸门 | 终签链笔 |
|---|---|---|---|---|---|
| m-adjudicate2-a1 | 心跳计数语义甲案维持现状谓词零改目录零迁移 | 9/9 合变卦 0% 谨慎 0/9 | baseline_4 主 baseline_1 次 | stable_clear | `e6ab6bd7` |
| m-adjudicate2-a2 | confpreempt 会话壳保留在册不销账不代清 | 9/9 合变卦 0% 谨慎 0/9 | baseline_4 主 baseline_1 与 5 次 | stable_clear | `63df2888` |

- 两席俱清晰稳定零刀锋，执契 check 各十二项全过 disposition 裁决通过、verify identical、sign 两笔 crosscheck_completed 落链经 scribe confirm 单笔确认（index 129 与 130）；重放锚即两 DES 单元格 signcheck 件。
- 席位当日基线：temp_probe 标定 4 命题 × 5 发全票即 safe 组 bdy 0.0、violate 组判违 5/5、knife 组 bdy 1.0，verdict 可用；基线以当日正身 identity.hash（c72a310b）实值配对（R5 两字段并存，identity_hash 配 identity.hash，core_hash 另载）。
- 采样形：emit-contract 零 LLM（seat ZCode:GLM-5.3:self-reported，9 shots，basis 枚举声明）；回填 18 发同席即席作答，谱系披露双声明载 topic authored 行；计分工地 facet 绝对路径形（facepatch 坑位规避）。

## 执行面

- **A1 执行**：PARKING-v1.md 心跳节落注句改裁定形即「甲案承 m-adjudicate2-a1 终签（adjudicate2-solo 批 2026-09-08 九发 stable_clear 执契 crosscheck 链笔 e6ab6bd7）」，乙丙选项文留档不删，谓词零改目录零迁移；文走化格核阅检词管线，读数见管线节。
- **A2 落据**：零写入（会话壳保留即现状），裁定记录入本结果档与链笔 context 即终签 63df2888 与机器可读投影 adjudications2.json；confpreempt 工地与未归并提交零触碰保留为续作现场。

## 管线读数

- 化格：PARKING-v1.md 与任务包与提示词件与结果档过 packs/general-v1（读数随回填）。
- 核阅：des-001 对工地路径一律域外 exit-2 如实记档（doc 类核阅正形在归并后主树路径，读数随回填）。
- 检词：nomenclator check packs/core 逐件（读数随回填）。
- checkcite：任务包与结果档与两 topic 合并单件扫描（读数随回填）。

## 认证清单

| 件 | 认证哈希前 8 |
|---|---|
| ask3 记录 | 待回填 |
| 验证件 | 待回填 |
| 正身件 | 待回填 |
| m-adjudicate2-a1 执契材料 | 待回填 |
| m-adjudicate2-a2 执契材料 | 待回填 |
| checkcite 件 | 待回填 |
| 投影件 adjudications2.json | 待回填 |
| 内容清单件（md 件 sha256） | 待回填 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 两席测量 | 采样道 | 两 gid 九发 stable_clear 执契通过两笔 crosscheck 落链，零刀锋零硬凑 | 通过（e6ab6bd7 与 63df2888） |
| F-2 闸门如实 | 跨族治理 | 两席闸门如实，刀锋即转人诊断不硬凑；误差红证全量记档 | 通过（双清晰稳定零刀锋；红证在批材料） |
| F-3 终签链笔 | 治理 | stable_clear 终签 crosscheck 笔在链 verify identical | 通过（两笔 confirm 在档） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（写入面即任务包两件、PARKING-v1.md、结果档与批材料、facet 合同目录、两 DES 单元格、链文件、报告目录、工地） |
| F-5 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 待收约回填 |

## 越线与误差申报

- **scribe intent 首跑缺 --sessions 参**：闸三必带参坑位复现，首跑 exit 2 零留痕（报文「会话在册验需 --sessions <会话台账路径>」），补参即过，红证即本条申报文。
- **tally assemble 首跑 topics-dir 缺参两红**：缺省 topics_dirs 不覆盖 facet 合同目录即「topic 按哈希无匹配」exit 1 两发；且 glob 不递归须传 gid 级目录，第二跑传合同根仍红一次，第三跑传 gid 级目录即过；判定内容零改动，红证即首跑报文在批材料 red-evidence-ndjson.log。
- **temp_probe 路径适配**：探针脚本居 retired/ 后 _THIS_DIR.parent 解析漂移（atom.yaml 与 ledger 位），以内联适配器驱动即改 _THIS_DIR 与 LEDGER 指向执行；标定账本 append 落 /tmp 副本不触工地 tracked 面（先例同批 adjudicate-solo 亦无账本行入版控，drift 对新身份恒空无差异），如实申报。
- **座位基线提取形**：ledger 尾行提取为 seat-baseline.json 承先例，账本本体不入版控。
- 其余误差零申报。

## 结算读数

- 双仓 settle：读数随回填。
- 认证实录：读数随回填。
- 放锁收约：读数随回填。
- 链 verify：读数随回填。
- reconcile：读数随回填。
- 收约补笔：管线实录与认证清单与 F-5 与结算读数即本笔，经 --no-verify 加 lease bypass 登记通道入版控（先例同形）。

## 大白话节

- **A1 心跳计数（说人话）**：已经办结出泊的事项，其材料文件目前仍留在原来的两个台账目录里，每天的例行盘点按既有规则把它们算进主线数。这次的裁定是：不改规则、不挪文件，一切照旧，理由是所有材料集中在一处最方便日后查账；至于「想在报表上把已办结的和在办的分开显示」这类需求，交给将来的展示层去做，盘点机制本身不动。
- **A2 会话壳（说人话）**：之前一条停滞的工作线经过清理后，它在登记册上的会话记录不删掉，留作以后重启这条线时的接续凭据——重启时依据链上的进度记录和留下的工地现场接着干，干完随这条线一起收口结账。现在不做任何删除，也不代替它清理。
