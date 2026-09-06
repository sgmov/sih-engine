# basisunion-solo 结果档

> 批:basisunion-solo（facet 依据族并集判据修订与 watchcheck F-5 复算落定）
> 会话:917bbc4e8c948f38（lease 1.25.0，scope_source package）
> 日期:2026-09-06 ｜ 队形:单线形 solo，零子代理 ｜ 两仓工地 tools@integral-stage-build 与 engine@main
> 令源:用户 2026-09-06 裁定原话「watchcheck 改题文本不需要改，只要改判定，依据只要是基线，无论是哪个，都是同一个语义的判决。」即依据族语义同一裁定；任务包 sih-engine/sih/state/plan/basisunion-solo.md 唯一规格源
> 意图:ask3 双门过（scrutinator ask3 零 findings、ask3repeater ok 三锚），叩问七轻信号 digest 全 covered，意图链笔 557a8bfa9dd4eeef

## 一句话结论

basisunion-solo 批交付:maturation_gate.py basis_consensus 判据由 distinct_basis<=1 硬共识改为集合成员语义（合法枚举集从合同各发 system_prompt 声明提取单源，集内即过、集外缺失无法解析仍挂，B1 降级不让位，其余四判据零动，basis_criterion 与 basis_union_version 版本注记在档而 criteria_version 顶层字面不动），TDD 三态夹具先红（11 failed）后绿（12 passed），全测试族 494 绿 6 环境态红（ng_assembler 工地 AGENTS.md 缺席，主树同件 9/9 绿，gateswitch 先例同形零回归）;watchcheck round1 九发原始 responses 零改动零重采样只读复算 verdict 由 boundary 翻 stable_clear，执契三步（check 十二项全过、verify identical、机器终签 e292b2e5）落链，watchcheck-solo F-5 落定结果档补记;判据变更自身经 facet 合同模式得一裁（gid m-basisunion-gate-1 九发 stable_clear，机器终签 c03647da）;双仓 settle 收约对表随批。

## 一、F 表（完成度表）

| F 锚定 | 类别 | 判据 | 结论 | 证据 |
|---|---|---|---|---|
| **F-1** | 新判据 | 集内分散即过、集外即挂、缺失即挂，三态夹具先红后绿 | 过 | tests/test_basis_union.py 十二用例先红（union 调用 TypeError 即红，11 failed 1 passed 留证 tdd-red-2026-09-06.txt）后绿（12 passed）；旧判据挂集内分散的语义红腿由 test_legacy_hard_consensus_still_hangs_dispersion 双相真锚定 |
| **F-2** | 复算翻转 | round1 按新判据复算 boundary 翻 stable_clear，原始 responses 逐字节零改动 | 过 | gate-recompute-2026-09-06.json:verdict_before=boundary、verdict_after=stable_clear、responses_zero_modification=true（tools 主树与 engine materials 双副本 sha256 前后一致对表），n_runs=9 只读消费 |
| **F-3** | F-5 落定 | watchcheck 执契终签在链，结果档补记 F-5 过 | 过 | attractor check 十二项全过（R5 席位当日基线可用且身份核哈希一致）、verify identical、sign 落链 event_hash e292b2e5385d7cae80c968c00b2a5925122daf025fb4c077f1f8d3357750dfe7；watchcheck-solo-results.md F-5 行与第十节补记随批 |
| **F-4** | 自裁 | 判据变更 facet 合同模式过得一裁 stable_clear 过执契，near_threshold 呈用户 | 过（未落 near，无需呈报） | gid m-basisunion-gate-1 九发全 comply 变卦 0% 谨慎 0/9，闸判 stable_clear（非 near_threshold），check 十二项全过、verify identical、机器终签 event_hash c03647daba30fdaa60f825769456af69b8acc9ab7f5366be1e72534dd8414d4a |
| **F-5** | 零回归 | facet 判据族与既有测试全绿 | 过 | 工地全测试族 494 passed 6 failed（6 红全为 test_ng_assembler 读 worktrees/sih-tools/AGENTS.md 不入版控活文件缺席，环境态非本批回归，主树同件 9/9 绿实证；gateswitch 批同款申报在案）；新增 12 用例含 gateswitch 判据族（test_boundary_switch.py 8 用例）零波及 |

## 二、交付清单

- **判据修订**:sih-tools/facet/probes/maturation_gate.py——basis_consensus 位双语义门控（_basis_union_assess：basis_allowed 缺省 None 即旧硬共识逐字节不变；给参即集合成员语义，集外/缺失硬挂、near 带退场、B1 不让位）＋_BASIS_UNION_VERSION="v1-2026-09-06-union" 版本注记＋B1 两处降级位让位守卫；criteria_version 顶层字面 "v3" 不动（引擎 tally R3 对表位）
- **枚举提取单源**:sih-tools/facet/contract_mode.py declared_basis_enum——逐发提取 system_prompt 输出格式行 basis_regulation 枚举声明，缺席与各发不一致即报错不猜，不新造判定常数零裸奔
- **计分半接线**:sih-tools/facet/measure.py score_contract_mode 自动提取合同枚举走集合成员语义；直跑腿 measure() 无合同缺省 None 旧语义零改动
- **三态夹具**:sih-tools/facet/tests/test_basis_union.py 十二用例（判据语义三态＋B1 不降级＋确定性双跑＋其余判据零动＋枚举提取三态＋计分半产线接线）
- **CONTRACT 修订**:sih-tools/facet/CONTRACT.md 修订记录节补判据变更登记笔
- **watchcheck 落定**:watchcheck-solo-results.md F-5 行补记与第十节补记；DES 单元格 m-watchcheck-judge-1 计分材料随批
- **材料**:basisunion-solo-materials/（ask3 相关链下件在 scribe/reports 与 identity/reports；TDD 红绿实录、复算件与脚本、席位基线、双 gid 执契四件套、标定包与回填）

## 三、判据变更明细

- 旧:distinct_basis <= 1 即九发引用唯一基线才过（basis_near = distinct == 2）
- 新（basis_allowed 给参，合同模式产线形）:引用基线全部落在合同声明枚举集内即过（集内语义同一承用户裁定）；引用落集外、缺失（含空转形）、无法解析仍挂（硬挂非 near）；集内分散无 near 带（同一语义判决的等价依据不构成贴近不一致）；B1_basis_degenerate 降级两处（v2 段与 v3 重放段）在集合成员语义下让位（B1 属旧硬共识语义面，裁定明文集外/缺失仍挂即不吃降级）
- 版本注记:basis_criterion（"hard"/"union"）与 basis_union_version（"v1-2026-09-06-union"）随闸返回 dict 与 trail gate_assessment 在册，下游 verifier 凭此复算；在档历史 verdict 零改写，生产行为变更只向前生效（gateswitch 拨闸先例同形）
- 引擎侧查证:grep sih-engine/src 无 basis_consensus 判据位（contract_mode.rs/tally.rs/compiler.rs 仅载 basis_regulation 字段透传），两侧同改条件不成立，引擎零改动零重编（gateswitch 先例同查法）

## 四、watchcheck F-5 复算与执契读数

- 复算（只读，零重采样）:gid m-watchcheck-judge-1，n_runs 9，verdict boundary → stable_clear；criteria 四真（decision_stable/boundary_low/basis_consensus/foregrounding_stable）；basis_criterion=union、basis_failure=null、basis_missing=0、basis_out_of_set=[]、distinct_basis=[baseline_1,baseline_4,baseline_5]；boundary_flag_sum=0；contract sha256 3ec129b8 与 responses sha256 02157c66 与原计分材料逐字节一致（tools 主树 facet/contracts/watchcheck-260905/ 与 engine materials 双副本前后哈希对表）
- 产线复分:measure.py --score 九发入 trail 实写 0（run_id 幂等跳过，飞轮 9 轮历史不变），gate_assessment 追加新读数，计分材料 gate_verdict=stable_clear 落 DES 单元格（openhyg 先例形）
- 席位基线:temp_probe 模式一（agent 侧标定，4 命题 × 5 发，温度 0.0 准确性过零漂移告警判定可用），ledger 尾行提取 seat-baseline-zcode-2026-09-06-basisunion.json（identity d4018b16、core 82f460c2 配对）
- 执契三步:check 十二项全过（R1-R7 含 R2 三哈希复算一致、R3 dc_fingerprint 复算一致、R5 核哈希配对、R6 九发未超预算）→ verify identical → sign 裁决通过落链 event_hash e292b2e5385d7cae…（doc_id crosscheck-m-watchcheck-judge-1），重放锚 m-watchcheck-judge-1-signcheck.json

## 五、判据变更自裁读数（得一裁）

- 立题形:机制规则类材料性断言（gateswitch 命题形同源）——「集合成员语义使依据族判定基准成为合同声明枚举集显式在册的可机械复算对象（满足基线四），distinct<=1 硬共识在机制类命题天然多基线并立时把同一语义判决的等价依据误伤为不共识」；gid m-basisunion-gate-1（ng medium n 9，seat ZCode:GLM-5.3:self-reported，出题半零 LLM 零网络）
- 回填:九发原文回填（contracts/basisunion-260906/responses.jsonl），逐发独立判定无人工轮换（watchcheck 批回填方法论失误教训在案：真判读单源即单源，不制造多样性）
- 计分:9/9 comply，变卦 0%，谨慎信号 0/9，规约引用 1 类（baseline_4——判据变更的主承重面是基线四可验证性，如实单源）
- 闸裁决:stable_clear（集内单源即过，basis_near 未触发）；非 near_threshold，按任务包不触发呈报分支
- 执契三步:check 十二项全过 → verify identical → sign 裁决通过落链 event_hash c03647daba30fdaa…（doc_id crosscheck-m-basisunion-gate-1）

## 六、管线与链

- 例行读数:gauge record ga-2 三维落链（convergence 0.142857 / adoption 0.5 / mergeback 0.041667，事件 204762cf/a1fad0cd/d8dd71e7）
- 泊界心跳:selector route 双目录退出码 0/0 零告警（tools 22 件 mainline 21 siding 1 即 pk-042 校准窗停泊既有态、engine 45 件 mainline 39 scrap 6）
- watch 对表:watchcheck check --at 2026-09-06 净态零无主修改（exit 0）
- 三问双门:scrutinator ask3 包 0 findings（record sha b2af9302）+ ask3repeater status ok 三锚；叩问 elicit 七轻信号 + digest passed covered 7（词债处置:七词不单独立词随批档行文承载，gateswitch 先例同形，本批零词表写入面）
- 正身:identity verify anomalies 0（identity_hash d4018b16、core_hash 82f460c2）
- 租约:open 917bbc4e8c948f38，allow 十一路径；锁 11 面（facet 独占、DES/CALL-LOG 双笔/trail/reports/ledger 追加、三结果面独占）；意图链笔 557a8bfa（裸调 grep 逐笔验证）
- 管线三步与认证:见第八节认证清单
- settle/close/reconcile/verify:收口读数 close 后回填（第九节）

## 七、越线与偏差申报

1. **工地脚本导径坑**:site-packages .pth 指向主树 facet，工地外脚本 import facet 模块若不把工地 facet 根显式列 sys.path 首位即静默解析主树旧码（本批 recompute 首跑实测 AttributeError 复现）；pytest 不受影响（rootdir 前插）。已修:recompute_gate.py 显式三段前插并在件内注记；BATCH-FACE 未随批（allow 面未含），坑位以此档承载候后继 face 批收编
2. **席位基线当日现制**:calibration ledger 当日尾行（并行批所写）无 core_hash 且 identity 异于本批，R5 直用即身份不配对挂起；处置:temp_probe 模式一现制本席当日基线（identity d4018b16 + core 82f460c2 双带），core 配对过 R5。基线现制是 openhyg 先例正形非偏离，申报点在「当日尾行不可直接复用」这一坑位
3. **facet 工具内 trail 与 DES 工件落位**:飞轮 trail 追加与计分材料落 sih-tools/proposition/DES/（facet 测量管线标准落位，watchcheck/gateswitch 先例同形，allow 面已含 proposition/DES/ 与 facet/，域内非越线；此笔为落位申报非违规申报）
4. **watchcheck 结果档跨批补记**:本批写 watchcheck-solo-results.md F-5 行与第十节（该档属 watchcheck-solo 批档面），经 allow 面 + 独占锁 + 任务包 §十一明列写入位，任务包令源在档
5. **引擎侧零改动**:attractor/scribe/scrutinator 零触碰零重编（判据位引擎侧无承载，grep 查证在第三节）

## 八、认证清单

认证两笔上链（全 meter 包裹闸三 --session 与 --sessions 双带，裸调 grep 逐笔验证）:

- 管线报告 pipeline-report-2026-09-06.json:event_hash bcee0627f06d0dcf585c3e2426e35b409a904c151416bfca06acbb257304ce3b（化格三件 0、核阅两件 2 域外如实记、检词三件 0 零违例、scribe 三笔哈希与例行读数三笔在册）
- 复算件 gate-recompute-2026-09-06.json:event_hash 5c1a9d10fcff009bdb244ccf77fcf6bc4c1c5496434e337870b3ad3e4290025e（verdict 翻转与零改动对表读数上链）

书单对表:recall（sih-math，主题「facet 判据 依据族 集合成员语义」）+ checkcite 退出码 0（cited 空——本批引用件零 sih-math 推导档 ID，allowed 76），报告在 scribe/reports/2026-09-06-basisunion-solo-checkcite.json。

## 九、收口读数（close 后回填）

- 双仓 settle:tools 段 1 dc8e929b（cert bcee0627）；engine 段 1 f314789（cert bcee0627）与段 2 a2911ff（cert 5c1a9d10，收编落包前温故检索两件消真分叉）
- 台账冻结笔:9e05a523（lease/ledger 三台账 sessions/locks/bypass.ndjson 工作面冻结，bypass.ndjson 首次入版控，收约冻结避坑令执行在档）
- close:917bbc4e8c948f38 revoked true，双工地与分支双拆归并（tools merge 6943032 入 integral-stage-build、engine merge 351317a 入 main），双仓零锁；首跑两撞真分叉（主树未跟踪活面:tools 侧 DES m-watchcheck-judge-1、engine 侧本批 materials 目录）按备份让位归并对表法处置——DES 前十行逐字节 identical 加两笔 gate_assessment 追加纯超集零丢失、recall 两件 cmp identical，备份在 /tmp/basisunion-des-backup-2026-09-06 与 /tmp/basisunion-engine-materials-backup，如实申报
- 台账镜像实测申报:close 的 closeguard pre-close 工作树提交（tools 5f2460a、engine c221f35）将本会话 11 行 acquired 镜像行清出 locks.ndjson（核对删除行 session 集恰为本会话零旁伤）；正典 lockdb 全史在册（acquired 11 与 released 11、现持 0），冻结笔 9e05a523 保全镜像快照，sessions 台账 issued 与 revoked 两行在册——非 ledgerloss5 旁伤亡形，实测形态如实记档候台账线复核
- reconcile:双仓 unrouted 0 零新增；tools cert_missing 1（e6880a63 entryunique-solo 陈项批前已有零新增）；unbypassed 累积含本批 closeguard 两笔自动提交（历代批同形未登记通道）
- 链 verify:valid 49 事件，首 7ac5aefc 尾 5c1a9d10
- 收约后 watchcheck 复跑:净态零无主修改（exit 0）

## 十、CALL-LOG

- sih-tools/facet/CALL-LOG.md:一笔（maturation_gate.py 判据修订与 contract_mode 枚举提取与 measure 接线）
- sih-tools/scribe/CALL-LOG.md:一笔（本批意图与认证写入）
- sih-tools/lease/CALL-LOG.md:一笔（本批租约收约）

## 十一、队形验证

单线形 solo 零子代理，委外代理亲写全程，T6-D 命名约定与 F 锚定与得一裁红线与双仓同步保留，范式零偏离。
