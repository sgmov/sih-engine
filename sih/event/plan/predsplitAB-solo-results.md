# predsplitAB-solo 结果档

> 谓词融回双路重裁批收口档——路 A 拆三子命题与路 B refine 重投四场全裁，四场全过全签，四象限如实入档
> 日期：2026-09-02。会话号：85428d726fc6539a。队形：单线形 solo，委外代理亲写零子代理，两路串行一个会话。
> 承接：用户 2026-09-02 双令（两条重测路各走一遍并进、命题选题写法规范将入向界）、predmerge-guard-solo boundary 打回重作前裁、GOV-002 判据二三五、SPEC-014

## 一、意图哈希与链位

- ask3 记录 sha256：`ca941d6a4697af208f829076125be1e490b0c3e023b045fc3cabeb5d580aca4d`
- 意图事件：`intent_refined`，event_hash `541da5424d750847a019fb65508e592cd989369829e612d3e7fdb933ad9c2575`（前八位 `541da542`，event_id 307fd33a-d69f-495a-9d3b-b8d907edee0a，当日链第 160 位）
- 意图前链对表：159 事件（末哈希 `1bf008a693821a73c105f5bf58f2103fc6b56f223f948a7d521a25bdd3248e04`），dispatch 基线约 146 事件，差 13 件为 degladder-solo 等在途批尾随（承尾随申报先例；degladder-solo 会话 1d99d3257616c8ce 于 03:25 发出 03:27 收约撤销，与本批锁面零冲突）
- 双门：scrutinator ask3 包零违规 exit 0（findings 0）；ask3repeater status ok（三锚，引文程序切片自 06-on-canon 73 行、07-on-assay 128 行、08-on-settle 114 行逐字节子串）
- 叩问：四词（单基线纪律、四象限、法层 refine、子命题）四信号轻级 unregistered，digest passed 4/4，处置行两词任务包原有，后两词随批补入任务包叩问处置节（工地副本）
- 正身：identity verify anomalies 空（identity/reports/2026-09-02-predab-identity.json，不入版控），identity_hash `58e220703201d895f5e1685c8aba873297dc989d6d7bedb514940ff31007e09f`
- inputlog：2026-09-02.ndjson seq 13 补录逐字一笔（sess-zcode-260902-acceptor：「2条路，调用2个子代理都各走一遍。……」全句在录，note 即双路重裁与命题规范意向令）

## 二、裁决结论（一句话）

**四场 gate_verdict = stable_clear，disposition = 裁决通过，direction = comply，四场全签——路 A 三子命题（predsplit-a1/a2/a3）与路 B refine 命题（predmerge-refine-b1）九发依据族各自单一，basis_consensus 全过，对照前裁 predmerge-guard-1 依据三散打回重作，双路重测皆翻案成立。**

| gid | 路 | gate_verdict | disposition | 逐发 decision | basis 分布 | boundary_flag | crosscheck 事件 |
|---|---|---|---|---|---|---|---|
| predsplit-a1 | A | stable_clear | 裁决通过 | comply ×9 | baseline_5 ×9 | 全 false ×9 | `17cf212b`（event_id ba28ca76-7c34-4af6-a674-46e5c182d334） |
| predsplit-a2 | A | stable_clear | 裁决通过 | comply ×9 | baseline_5 ×9 | 全 false ×9 | `77834b28`（event_id b0845d9e-7f3b-4948-9d32-228f0a66ee9c） |
| predsplit-a3 | A | stable_clear | 裁决通过 | comply ×9 | baseline_4 ×9 | 全 false ×9 | `90ddbea3`（event_id 363513aa-6567-4efe-a3f8-bf333f9446d2） |
| predmerge-refine-b1 | B | stable_clear | 裁决通过 | comply ×9 | baseline_5 ×9 | 全 false ×9 | `54dc6f87`（event_id d5ddb7c0-b09b-4233-9b84-f16ff1e4824e） |

dc_fingerprint：a1 `f66ab87bd08a04d8`、a2 `9e92b3ab9583d7eb`、a3 `5cbf6d8a23e9809b`、b1 `4cad315bec74700f`，与各场闸评估与核对报告复算一致。前裁归因（三事捆绑命题过宽且各事裁决基线不同）经双路重测验证：拆分单锚（a1/a2/a3 各单主题）与法层归约（b1 单基线锚）两形都使依据族收敛，命题写法确是 basis_consensus 的支配变量。

## 三、引擎件全流程证据（F-1）

四场同构，每场七步，串行执行（路 A 三场毕再路 B）。工件在 tools 工地 proposition/DES/<gid>/，合同在 facet_task_packages/<gid>/topic.md。

| 步骤 | 执行件 | 产物 | 退出码 |
|---|---|---|---|
| 出合同 | target/debug/attractor emit-contract | <gid>-contract.json ×4（ng_text_sha256 d4f84701 与 medium 正典对表一致 ×4） | 0 |
| 席位作答 | 席位亲写零子代理 | <gid>-responses.jsonl ×4（九发逐发独立重采，四场 36 发零复用前裁响应） | — |
| 闸判定 | 判据 v3 闸（围堰上游闸，SPEC-014 留堰设计，measure.py --score /tmp 副本承载） | flywheel-trail.jsonl ×4（各十行：九 flywheel_run 加 gate_assessment） | 0 |
| 计分 | target/debug/attractor score（meter 包裹） | <gid>-contract-score-material.json ×4（幂等复入 runs_written 0 如实载 ×4） | 0 |
| 装配 | tally assemble（零判断，引擎 lib 面无 CLI 承围堰兼容位） | <gid>-tally-material.json ×4 | 0 |
| 核对 | target/debug/attractor check（meter 包裹） | <gid>-check-report.json ×4（R1 至 R7 十二项全过零失败零告警，verdict pass ×4） | 0 |
| 终签 | target/debug/attractor sign | signcheck ×4 + crosscheck 事件 ×4 入链 | 0 |

- emit-contract 全流程经引擎件 attractor（禁用围堰出题半），四场合同九 shots 同提示词同 contract 形与前裁正典对表
- 判定依据族四场收敛：a1 与 a2 与 b1 归 baseline_5，a3 归 baseline_4，各场 distinct_basis 单值，basis_consensus 过，decision_stable 过（翻悔 0），boundary_low 过（boundary_rate 0.0）
- R5 席位基线：见第四节重标定申报

## 四、seat 基线对表与重标定申报（F-3）

- 当日 predmerge-guard-1 标定件（identity_hash c21386ea…）按 dispatch 先试复用对表：本批正身 identity_hash 实为 `58e22070…`，两哈希不一致——十二组件正身哈希随机器状态漂移，同席同日不保证同哈希，R5 哈希优先配对必挂，**按 adisp 先例 temp_probe 重标定**
- 重标定执行：temp_probe agent 模式 export-pack 出包（pack_version 1，四命题 × 5 发）、席位 20 发逐发作答、程序计分：判定可用、体温 0.0、violate 5/5 全票判违、零漂移零解析失败、knife 5/5 全票判违且 boundary 全 true；标定件 facet_task_packages/predsplit-a1/seat-baseline-zcode-2026-09-02.json（a2/a3/b1 目录各存同件副本供各场材料引用），cal-pack.json 与 cal-responses.jsonl 存 predsplit-a1/，账本行入 facet/probes/calibration/ledger.jsonl
- 四场 R5 全过：席位当日基线判定可用且身份哈希一致（58e22070 四场同源配对）
- 对己不利声明前置在案：四 topic.md 谱系披露节各载明判成立即封本席围堰短便、利益指向 violate，如实披露

## 五、四象限对照表与各象限处置建议（F-3）

本批实测落位：**A 过 B 不过 = 双过**象限（路 A 三场全过、路 B 一场全过）。

| 象限 | 本批实测 | 处置建议（供用户与命题写法规范批取数） |
|---|---|---|
| 双过 | **本批**：路 A 三场全过且路 B 过 | 两形皆有效：拆分单锚（一命题一主题一依据族）与法层 refine（捆绑命题归约单基线锚）都能救 basis_consensus。规范批可立两条正向写法：子命题拆分时各锚单基线族；捆绑命题必须显式单基线锚不得三族并述。融回候令如实报（融属后批，本批不写规格） |
| 双不过 | 未落 | 两路同挂即命题对象本身未熟（非写法问题），处置同前裁：停批留档归人裁量，不硬凑第三种写法重耗采样预算 |
| A 过 B 不过 | 未落 | 拆分可裁而捆绑 refine 不可裁即三事在单锚下确有异质依据张力，处置：按子命题各签各融，捆绑形废弃，规范批记「拆分优先」 |
| B 过 A 不过 | 未落 | 归约单锚可裁而拆分不可裁即子命题各自单薄难立，处置：按捆绑单锚签，规范批记「单锚捆绑可行、拆分需最小命题粒度」 |

- 逐发 basis 全载见第二节表（本批四场逐发 basis 皆单值分布，无散布）；规范批取数基面：前裁 predmerge-guard-1 散布形（baseline_5×4/baseline_1×3/baseline_4×2，decision 全 comply）加本批四场收敛形，两形对照即「命题写法决定依据族收敛」的经验证；
- a1 双述（baseline_4 与 baseline_5 并述于命题文）实测仍收敛 baseline_5：双述不必然致散，散的支配变量是命题对象跨主题（前裁三事跨主题 vs 本批 a1 单主题双面），此读数如实供规范批复核

## 六、分流声明（F-4、F-2）

**四场全过即候融：predsplit-a1、predsplit-a2、predsplit-a3、predmerge-refine-b1 四 gid 裁决通过并终签，融回三事（路择谓词包融判定器、截流谓词族融三问、过程件归零）获全流程引擎件裁断成立，候实装融回批放行——本批红线「过了就融的融属后批」在位，本批不写融回规格、不立项 SDD 批、selector 与 facet 与 tally 与引擎源码零改动。** 裁决材料（四合同、四响应、四飞轮链、四计分材料、四裁决材料、四核对报告、四 signcheck）与四命题档与标定件全数随批入版控，crosscheck 事件四枚在链，融回实装的立项与规格归人裁量放行。

## 七、收口对表（F-2、F-4）

- F-2 终签承载如实申报：四场 disposition 裁决通过居三态映射通过值，sign 四场各落 crosscheck_completed 事件一枚（161 至 164 位），signcheck 四件为重放锚；a1 首次 sign 因 material 传绝对路径被跨方核毕守卫拒（字段 material 形态违例非相对 json 路径），零链上事件零落据，改相对路径重签成立——失败尝试与重签全程见第九节申报
- 双仓 settle：tools 与 engine 工地各段1 settle 提交，链上认证哈希为凭（认证节）
- 放锁收约、reconcile 双仓四类读数、当日链 verify、调用册留痕：见认证节后对表

## 八、F 表

| F | 类别 | 判据 | 结论 |
|---|---|---|---|
| F-1 | 工程治理 | 三子命题各全流程引擎件出裁，各 gate_verdict 与 disposition 如实，stable_clear 即签即 crosscheck 事件各异 gid | 过（第三节七步表：emit-contract 与 score 与 check 与 sign 皆 target/debug/attractor，四场 gate_verdict stable_clear 与 disposition 裁决通过如实，crosscheck 事件四枚 gid 各异） |
| F-2 | 链上治理 | refine 命题单基线重投全流程，结论如实 | 过（b1 锚死 baseline_5 单基线重投全流程，stable_clear 即签 `54dc6f87`，九发独立重采零复用前裁响应） |
| F-3 | 工程治理 | 四象限对照表入结果档，各象限处置建议在场 | 过（第五节四象限全表，本批落位双过如实，未落三象限处置建议同载；seat 基线对表不过即重标定按先例执行在第四节） |
| F-4 | 纪律 | 禁钓样本、对己不利声明、seat 基线对表、链尾对表、reconcile 四零 | 过（四场 36 发逐发独立重采零改答零钓样；对己不利声明四档前置；基线对表挂即重标定在案；链尾对表第一节与第九节；reconcile 读数见对表节） |

## 九、越线与误差申报

1. sign 首试被拒如实申报：a1 首次 attractor sign 以绝对路径传 --material，跨方核毕守卫拒（字段 material 形态违例非相对 json 路径），scribe_exit 1 零链上事件零落据；改工地相对路径重签成立，四场自此全用相对路径形。失败形与正典形已对表 BATCH-FACE sign 节（「--material <裁决材料.json 相对 json 路径>」在案），零漂移
2. 链尾尾随：dispatch 基线约 146 事件，意图前实为 159 事件——degladder-solo（1d99d3257616c8ce，03:25 至 03:27 已收约撤销）等批事件尾随入链，承尾随申报先例；本批在链期间链尾无他会话新事件尾随，意图 160 位与四签 161 至 164 位连续在案
3. gate_verdict 传入形态：判据 v3 闸留围堰是 SPEC-014 明文设计，四场闸判定经 measure.py --score /tmp 副本承载（正典材料全数引擎件产出，围堰 /tmp 侧计分材料弃置），引擎 score 以显式 --gate-verdict 承接闸上游产出，runs_written 0 如实载（飞轮链九发由围堰闸腿先写，run_id 确定性幂等，R4 历史不可篡改由幂等语义承载）
4. tally assemble 经围堰兼容 CLI：装配为零判断确定性拼装（引擎 lib 面无 CLI，SPEC-014 跨腿契约字段对表执行），前裁同形先例在案，判定终签强制位零触围堰
5. 包档与结果档域外 exit-2 如实记：des-001 域只盖 sih-engine/doc，四 topic.md（sih-tools）与 state/plan 任务包与 event/plan 结果档核阅 exit-2 不属违规，读数入 materials/pipeline.log
6. 主树任务包处置行：叩问处置补两行（法层 refine、子命题）与工作清单三项勾选落在工地副本（predsplitAB-solo.md），主树原档零直写，收约归并承备份让位对表法，预期差异即此三处随批写入，diff 对表按此申报执行
7. identity 重标定误差：dispatch 预案为当日基线同席同日可复用，实测正身哈希已从 c21386ea 漂至 58e22070（十二组件含机器状态量，同席同日不保同哈希），复用对表不过即按 adisp 先例重标定，标定读数全载第四节；此哈希漂移现象本身如实供身份工具批参考
8. identity/reports 与既有存量 untracked 零收编、assettwave-a 与 extinv-parallel 在飞工件零触碰、selector 与 facet 与 tally 与引擎源码零改动：全批遵守，双仓 settle 只动工地批件；scribe/reports 批件（ask3 记录、验证件、叩问契约与信号）承前裁诸批惯例留主树不入版控
9. 判定语义边界：四场作答是本席逐发独立判定，a1 双述面与 a3 的基线五近邻面在作答中如实按单值依据判定，未为收敛而协调逐发措辞；四场 basis 单值分布是实测结果非预设指标，对照表如实载不硬凑

## 十、包档勾选

任务包工作清单三项随本结果档如实勾选（路 A 三子命题起草与三场裁决、路 B refine 命题重写与一场裁决、终签入链与四象限对照档），验收标准 F-1 至 F-4 按第八节 F 表承载。
