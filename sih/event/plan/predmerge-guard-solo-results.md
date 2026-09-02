# predmerge-guard-solo 结果档

> 谓词融回得一裁批收口档——裁决打回重作，如实停批，不融不强推
> 日期：2026-09-02。会话号：c4561b0a9e4183c5。队形：单线形 solo。
> 承接：用户 2026-09-02 得意裁过了就融令、GOV-002 判据二三五待闭、SPEC-014、adisp-guard-1 先例

## 一、意图哈希与链位

- ask3 记录 sha256：`9f218a251182ecd80a47ebbdedeb28fec46f5e52163a2bc278ec19b9317792cf`
- 意图事件：`intent_refined`，event_hash `ddde802762df157c6fa8c64cf7459016ddd2f5c689c2cae3f3eda6fd073254a7`（前八位 `ddde8027`，event_id 8739e89b-6915-45e8-b721-3cd8c61a5c1d）
- 意图前链对表：134 事件（末哈希 `2d1ea70375af82149596750b1479c53e28de051fcb8afedfdf231a08f760e1a6`），dispatch 基线 120 事件，差 14 件为 deyimerge-switch、assettwave-a/b/c/d、extinv-parallel 各在途批尾随（承尾随申报先例，extinv-parallel 批与本批并行在飞、其链写入经逐件对表无哈希分叉）
- 双门：scrutinator ask3 包零违规 exit 0（findings 0）；ask3repeater status ok（三锚，引文程序切片自 06-on-canon 73 行、07-on-assay 128 行、08-on-settle 114 行逐字节子串）
- 叩问：四词（谓词融回、可插拔、对己不利声明、首战自证）四信号轻级 unregistered，digest passed 4/4，处置行落任务包叩问处置节（前两词包档原有，后两词本批补行）
- 正身：identity verify anomalies 空（identity/reports/2026-09-02-predmg-identity.json，不入版控），identity_hash `c21386eac49d69621f0e02ea843de267ed2e34bd763e5b399cde71e27f99f040`
- inputlog：2026-09-02.ndjson seq 12 补录逐字一笔（sess-zcode-260902-acceptor：「得意裁，过了就融」，note 即谓词融回先裁后融令）

## 二、裁决结论（一句话）

**gate_verdict = boundary（子判据 basis_consensus 挂），disposition = 打回重作，非裁决通过——按 dispatch 不签不停留，本批如实停批：不立项 SDD、不写融回规格，裁决材料全数留档上链，打回重作转候补批归人裁量。**

逐发 decision 分布：comply ×9/9（零翻悔、boundary_flag 全 false）；basis_regulation 分布 baseline_5 ×4、baseline_1 ×3、baseline_4 ×2——decision 全稳定而依据三散，闸子判据 basis_consensus 判挂。

## 三、引擎件全流程证据（F-1）

| 步骤 | 执行件 | 产物（tools 工地 proposition/DES/predmerge-guard-1/） | 退出码 |
|---|---|---|---|
| 出合同 | target/debug/attractor emit-contract | predmerge-guard-1-contract.json（ng_text_sha256 d4f84701 与 medium 正典对表一致） | 0 |
| 席位作答 | 席位亲写零子代理 | predmerge-guard-1-responses.jsonl（九发逐发独立，key 合同内 r1 至 r9） | — |
| 闸判定 | 判据 v3 闸（围堰上游闸，SPEC-014 留堰设计） | flywheel-trail.jsonl 十行（九 flywheel_run 加 gate_assessment） | 0 |
| 计分 | target/debug/attractor score（meter 包裹） | predmerge-guard-1-contract-score-material.json（幂等复入 runs_written 0 如实载） | 0 |
| 装配 | tally assemble（零判断，引擎 lib 面无 CLI 承围堰兼容位） | predmerge-guard-1-tally-material.json（dc_fingerprint 26c41e1d 与闸评估一致） | 0 |
| 核对 | target/debug/attractor check（meter 包裹） | predmerge-guard-1-check-report.json（R1 至 R7 十二项全过零失败零告警，verdict pass，disposition 打回重作） | 0 |
| 终签 | 未调用——红线「不过即不签」；SPEC-014 契约处置非裁决通过即 refused 出 1 零链上事件 | 无 crosscheck 事件即机械正确态 | — |

- seat 基线先行：当日无本席基线可对表（adisp 基线 identity_hash c80d562d 且内部日期 2026-09-01，与本席当日正身哈希不一致，R5 哈希优先配对必挂），按 dispatch 补做当日标定：temp_probe agent 模式 export-pack 出包、席位 20 发作答、程序计分，判定可用、体温 0.0、violate 5/5 全票判违、零漂移零解析失败，基线件 facet_task_packages/predmerge-guard-1/seat-baseline-zcode-2026-09-02.json，identity_hash 与正身件同源配对（R5 过：席位当日基线判定可用且身份哈希一致）
- 对己不利声明前置在案：topic.md 谱系披露节载明判成立即封本席围堰短便、利益指向 violate，如实披露

## 四、闸挂详情与归因如实申报（F-3）

- 闸评估读数：decision_stable 过（翻悔 0）、boundary_low 过（boundary_rate 0.0）、basis_consensus 挂（distinct_basis 三值）、foregrounding 不适用；闸注记「子判据挂 basis_consensus → 路由法层 refine」
- 执行因 disclosed：本席九发作答理由逐发择面引用了三条不同基线；对照当日标定四命题各五发全部 basis 一致、adisp 九发 basis 一致的确定性席位语义，本批发散更可能属采样腿执行缺陷（作答行为）而非命题自身性质，但此归因是本席自述、不经机械证明，如实申报不掩饰
- 禁钓样本禁改答红线遵守：闸裁决 boundary 在案后本席零改答零重采，不因判据二三五方向全绿而强推重测；打回重作之路由法层 refine 转候补批

## 五、分流声明（F-4）

**不通过即停：本批不立项 SPEC-015 谓词融回落差规格批，不写融回规格，融回三步曲不启动。** 裁决材料（合同、响应、飞轮链、计分材料、裁决材料、核对报告）与基线件与命题档全数随批入版控，核对报告 verdict pass 承载 R1 至 R7 机械有效，disposition 打回重作承载闸裁决，候补批拟名与材料位已备即 facet_task_packages/predmerge-guard-1/ 与 proposition/DES/predmerge-guard-1/，后续重测须承法层 refine 结论并由人类裁量放行。

## 六、收口对表（F-2、F-4）

- 双仓 settle：tools 与 engine 工地各段1 settle 提交，链上认证哈希为凭（认证节）
- F-2 终签承载如实申报：本裁处置打回重作非通过或挂起，sign 按红线未调用，crosscheck 事件零入链属 SPEC-014 契约机械正确行为；gate_verdict boundary 与 disposition 打回重作以核对报告认证件承载入链，无伪造事件
- 放锁收约、reconcile 双仓四类读数、当日链 verify、调用册留痕：见认证节后对表

## 七、F 表

| F | 类别 | 判据 | 结论 |
|---|---|---|---|
| F-1 | 工程治理 | 全流程经引擎件 attractor 即合同与计分与核对皆引擎件产出，禁用围堰件出裁 | 过（第三节七步表；emit-contract 与 score 与 check 皆 target/debug/attractor 且 meter 包裹，闸判定承 SPEC-014 留堰设计为围堰上游闸、判定终签强制位零触围堰；sign 未调用因处置非通过） |
| F-2 | 链上治理 | crosscheck 事件入链载 predmerge-guard-1、gate_verdict 与 disposition 如实即通过或挂起 | 不适用态如实承载（本裁 disposition 打回重作居三态映射第四值，sign 契约性 refused 位未触发即无事件，链上以核对报告认证件如实承载 boundary 与打回重作，无伪造无掩饰） |
| F-3 | 工程治理 | seat 基线先行有对照、无钓样本、对己不利声明在案 | 过（第三节基线先行与第四节零改答申报；对己不利声明 topic.md 谱系披露节在案并前置） |
| F-4 | 链上治理 | 通过即 SDD 批立项声明入结果档；不通过即停批报告 | 过（不通过支：本档即停批报告，第五节分流声明不立项不写规格，裁决材料留档上链） |

## 八、越线与误差申报

1. 链尾尾随：dispatch 基线 120 事件，意图前实为 134 事件——deyimerge-switch 收口、assettwave-a/b/c/d、extinv-parallel 各批事件尾随入链，承尾随申报先例；extinv-parallel（01a1613630ac8255）与本批并行在飞且持 state/plan、event/plan、trail、inputlog 四目录锁，与本批文件锁重叠并存承 assetwave-a 对 deyimerge-switch 先例，lease 五验零拒绝、链写入逐件无哈希分叉，如实申报
2. gate_verdict 传入形态：判据 v3 闸留围堰是 SPEC-014 明文设计，引擎 score 以显式 --gate-verdict 承接闸上游产出，本批围堰腿仅承载出题后计分半上游闸（measure.py --score /tmp 副本承载，正典材料全数引擎件产出），机械腿只装配不判闸
3. runs_written 0 如实申报：飞轮链九发由围堰闸腿先写（run_id 确定性幂等），引擎 score 幂等复入零重写，runs_written 如实载 0，R4 历史不可篡改由幂等语义承载
4. 包档与结果档域外 exit-2 如实记：des-001 域只盖 sih-engine/doc，topic.md（sih-tools）与 event/plan 结果档核阅 exit-2 不属违规，读数入 materials/pipeline.log
5. 主树任务包处置行：叩问处置补两行落在工地副本（predmerge-guard-solo.md），主树原档零直写，收约归并承备份让位对表法
6. 判定归因自述边界：第四节执行因归因是席位自述非机械证明，闸裁决与处置以机械件为准，不自评翻案
7. identity/reports 与既有存量 untracked 零收编、assettwave-a 在飞工件零触碰、selector 与 facet 与 tally 与引擎源码零改动：全批遵守，双仓 settle 只动工地批件
8. 包档勾选：任务包工作清单三项随本结果档如实勾选（裁决完成、分流走停批支），验收标准按第七节 F 表承载
