# pk037impl-solo 结果档

> 批：pk037impl-solo retriever 语义升级立项批（确定性统计向量形，词面路径保持缺省，A/B 度量门切换）
> 会话：744d2c9fbcb173a2（租约自生成）｜会话标识 sess-zcode-260904-pk037impl（ask3 双标识空间，各认各的）
> 日期：2026-09-04 ｜ 队形：单线形 solo，零子代理
> 承接：用户 2026-09-04 裁 pk-037「应该升级」并同意路线形「采确定性统计向量、不做向量库、A/B 门切换」；路线设计基线 m-pk037-route-1/2 两轮九发十八发 comply 全收敛零变卦
> 政策行：用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——裁决点全过得一三态机械分流，机械链全绿自行收口
> 意图哈希：636bbfe7（intent 事件，meter 包裹；ask3 双门过：核阅 exit 0 零违规、重放 status ok 三锚；digest passed covered 5）
> 领取登记：lease claim 一笔在先（claimant sess-zcode-260904-pk037impl，ttl 480 分），claims 册在案

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| 件一 pk-037 出泊记账 | 完成 | park exit 一笔 b68b178a（disposition promoted，ruling 载用户裁定原话与两轮测量材料指引）；名册在泊七改六、历史住户十改十一补行含出泊事件哈希；泊材料件 pk-037.json 状态改 exited、pk-037-exit.json 落盘；顺笔修第 25 行存量 C006 全角括号内容改即串语义零改，修后核阅 des-001 域内拷贝 exit 0 |
| 件二 数学载体实查与补立 | 完成 | 实查：TF-IDF/IDF 族无在仓载体、余弦相似度无已建载体、ALG-011 向量空间为已登记待建号；走 newcarr 同款立项，标定 20 发可用 + 两 gid 九发全 stable_clear + attractor check/verify/sign 终签两笔；条目两件、INDEX 两行、mapping 两行落位 |
| 件三 语义层实现 | 完成 | wikirecall/semantic.py 纯 Python 零新增外部依赖，语料内 TF-IDF 加权与余弦相似度，语料规模内全量暴力计算，中间量逐字节可复现；recall.py 增 --semantic K 可选旗标，无旗标输出与批前四计划 cmp 逐字节一致；引擎侧 Rust retriever 零触碰 |
| 件四 A/B 度量门 | 完成 | 判据预登记件考前写死随批入版控；metrics.py 零改动为裁判；正典轮三判据全过 verdict pass；判变件 pk-050 泊界登记零静默（入泊 0b3da7ec），缺省位保持词面 |
| 件五 四件套 | 完成 | 载体引用即 PROB-017/ALG-011；推导档 sih-math/docs/pk037impl-derivation-2026-09-04.md；接线即 recall.py 旗标位；金向量三场景冻结于 fixtures/golden/ 携带重放寻径约定，双跑逐字节一致 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 出泊记账合规 | 出泊唯人节点裁且有据，名册投影对链 | 过 | 用户裁定原话入 ruling；出泊事件 b68b178a 与名册补行哈希一致 |
| F-2 载体不硬挂 | 缺载体走测量立项，禁硬挂 | 过 | 实查零命中申报在案；两 gid 三态 stable_clear 方落位；首测即过零重测 |
| F-3 实现纪律 | 纯 Python 零新依赖、全量暴力、逐字节可复现、零 LLM | 过 | semantic.py 仅标准库 math/re/json；双跑 cmp IDENTICAL；判定路径无任何模型调用 |
| F-4 词面缺省零改 | 无旗标行为逐字节不变 | 过 | 改前主树四计划快照对改后工地四计划 cmp 4/4 一致；既有 selftest 与 metrics_selftest 全绿；引擎侧零触碰 |
| F-5 A/B 门诚信 | 及格线考前写死、禁调参钓鱼、两态如实 | 过 | 预登记件先于实测入版控；首轮词元化缺陷读数留档申报不抹除；正典轮与首轮 verdict 均 pass，判据夹具两轮零改 |
| F-6 判变零静默 | 切换候选与不达标都泊界登记 | 过 | pk-050 入泊事件 0b3da7ec；名册在泊行同步；缺省位保持词面直至用户裁 |
| F-7 机械链纪律 | 闸三闸四 bypass 与禁管道掩码等 | 过（一笔管道自查纠正见误差节） | intent 与 append 全带 --session 加 --sessions；无通道外直提；退出码均实取 |

## 三、载体终签节

- 标定：temp_probe agent 模式 20 发（safe-1/2 comply 全票 bdy 0、violate 判违全票、knife 判违 bdy 1.0 与家族基线一致、漂移零），判定可用；身份哈希 16ade688 与本会话正身件配对一致；基线件 tally/reports/seat-baseline-zcode-2026-09-04-pk037impl.json。
- m-pk037-idf-1（PROB-017 逆文档频率与自信息加权）：九发 comply、变卦 0、谨慎信号 0/9、依据族 baseline_1 一类；attractor check R1-R7 全过处置裁决通过、verify identical、sign 落据 crosscheck-m-pk037-idf-1 6b3723e8。
- m-pk037-cos-1（ALG-011 向量空间与余弦相似度，2026-09-02 登记面归一待建概念落地）：九发 comply、变卦 0、谨慎信号 0/9、依据族 baseline_1 一类；check 裁决通过、verify identical、sign 落据 crosscheck-m-pk037-cos-1 70da4959。
- 落位：probability/entries/PROB-017-idf-and-self-information-weighting.md 与 algebra/entries/ALG-011-vector-space-and-cosine-similarity.md（定义先行、可证伪节、双答结构在场承 M-4，无哲学新桥申报）；probability/INDEX.md 已建 17 条、algebra/INDEX.md 已建七条各随史行；mapping.md 两新行落在载体行区段，消费面全读对表通过。

## 四、A/B 度量读数节（正典轮）

语料 sih-math 主树批前实态 165 条目；夹具八查询固定（四字面四转述）；K=3；裁判 metrics.py 零改动。判据预登记件：ab-criteria-preregistration.json。

| 聚合读数 | 数值 |
|---|---|
| ΣIG_A（旧词面路径种子通道） | 10.6598 比特 |
| ΣIG_B（新语义路径种子通道） | 23.1121 比特 |
| ΔIG（判据线 0.5 比特） | 12.4523 比特，过 |
| 聚合语义∩词面（C3 线 ≥1） | 8，过 |
| C1 新路径零退化 | 八查询全 active，骨架全 complete，过 |
| verdict | pass（切换候选成立） |

逐查询读数要点：四字面查询两路径全 active，lit-1 与 lit-2 位词面收窄略优如实照录；四转述查询词面通道全哑火（0 命中即退化）而语义通道全 active，头部命中依次为 PROB-011、PROB-014、ORD-020、HIS-015，前三条与查询意图族对位，par-4 头部偏离如实照录。读数双跑逐字节一致；全文见 ab-readings.json。命中率语义为通道对并集贡献占比（label-free），非带标注查全率，解读承 PROB-011 登记边界。

## 五、出泊记账节

- 出泊：pk-037 于 2026-09-04 经 scribe park exit 上链，事件哈希 b68b178a，disposition promoted，ruling 载用户 2026-09-04 裁定原话与 m-pk037-route-1/2 材料指引；泊材料件状态更新为 exited 并落 pk-037-exit.json。
- 名册：当前在泊七改六、历史住户十改十一，pk-037 补行含出泊事件哈希；第 25 行存量 C006（facepark-solo 事故申报全角括号补释）顺笔改即串，语义零改。
- 判变件入泊：pk-050 retriever 缺省位切换裁定，入泊事件 0b3da7ec，名册在泊行同步（六改七），出泊条件即用户裁切换与否；缺省位保持词面。

## 六、认证清单

| 事件哈希 | 对象 | 备注 |
|---|---|---|
| 636bbfe7 | 意图笔 scribe intent | meter 包裹，闸三 --session 加 --sessions 带 |
| b68b178a | park exit pk-037 | 出泊 promoted |
| 6b3723e8 | crosscheck-m-pk037-idf-1 | attractor sign 落据 |
| 70da4959 | crosscheck-m-pk037-cos-1 | attractor sign 落据 |
| 0b3da7ec | park enter pk-050 | 判变件泊界登记 |
| 01d24914 | 认证：measurements 报告 | meter 包裹 append 主树活链 |
| 4bcda053 | 认证：pipeline 报告 | meter 包裹 append 主树活链 |
| d3a81b3d | 认证：changed-files 报告 | meter 包裹 append 主树活链 |

## 七、越线与误差申报

1. 误差一笔（词元化实现缺陷）：语义层首实现偏离推导档登记口径，偶数长汉字段多出尾单字元；发现于自测断言对表，修正归位后重冻金向量三场景并重跑 A/B；首轮读数（ΔIG 12.7472，verdict 亦 pass）原样留档 ab-readings-v1-tokenizer-defect.json，判据与夹具两轮零改动，非调参钓鱼；正典轮为本档口径轮。
2. 调用顺序一笔：叩问 elicit check 先于 ask3 记录落笔跑信号（只读），信号五条如实回填 ask3 记录 elicitation 处置后再过第一门与 digest 与第二门；处置与信号一一对应，门与 digest 结果全过，顺序偏离如实申报。
3. 域内校验拷贝一笔：名册 des-001 域 glob 不匹配工地路径，核阅以域内校验拷贝（src/scrutinator/fixtures/corpus/check/ 下同名拷贝，内容逐字节同）承检 exit 0，拷贝验毕即删不入提交；主树归并后复验见收口附记。
4. 标定账本写入位一笔：temp_probe score 追加标定账本落在 tools 工地内（worktrees/sih-tools/pk037impl-solo/facet/probes/calibration/ledger.jsonl），随批 settle 入版控，与 newcarr 先例的主树活写位不同，属共享追加史的工地提交形，如实申报。
5. 误差一笔（管道掩码自查）：泊界心跳首跑经管道取码被掩，即改重定向落盘复跑取真码（tools 线 exit 0、engine 线 exit 0），属调用面误差非数据面误差。
6. 烟测与夹具重合一笔：件三功能烟测查询「点估计的可信范围」与后登记夹具 par-1 同串，读数方向与正典轮一致，如实申报。

## 八、冲突样本节（pk-045 样本库）

单飞批但有并行批交错写链：carrwire-solo（会话 ba187edb54e1ee7a）在本批施工期间向同日链写入意图一笔与认证五笔（链上事件 102 与 106 至 110），与本批八笔交错；链哈希连续、verify valid、双方事件零覆盖零丢失，即书简追加位安全交错形，与 pk-045 链事件活写覆盖丢失旧样本对照在案。其会话在册但零锁在持，与本批二十锁零重叠，exclusive 施工面零争用；sih-tools/scribe/reports 共享追加面两会话并发追加，append 锁共存形未拦即该锁型的设计语义。本批零撞锁零重试，交错实测一笔样本贡献如实申报供 pk-045 归档。

## 九、收口读数（认证时点）

- 链：sih-engine/sih/event/trail/2026-09-04.ndjson 认证时点 verify valid，批前 99，本批新增意图一笔、出泊一笔、终签两笔、入泊一笔、认证三笔共八笔，另有并行批 carrwire-solo 六笔交错写链，认证时点链 113 事件。
- 锁：二十把（九独占十一追加）在持，待 settle 后放锁收约。
- 工地：批件已拷三仓工地（math：条目两件与 INDEX 两件与 mapping 与推导档；tools：wikirecall 四件新增加 recall 接线加合同与 DES 与 tally 报告与标定账本；engine：名册与泊材料件三件与任务包与 dispatch 与 materials 与结果档）。
- close 与 reconcile 与终态 verify 读数与三仓提交号见文末收口附记（close 后补记）。

## 十、队形验证

单线形 solo 成立：本批全部写入由会话 744d2c9fbcb173a2（sess-zcode-260904-pk037impl）亲写，零 Agent/Task 子代理调用；标定 20 发与两载体 18 发为合同模式围堰席逐发回填（零 LLM 堆叠替代确定性验证），三态由确定性判据闸承载，终签由 attractor R1-R7 确定性核对承载；A/B 门判定路径零 LLM 调用，工程基线第五条全批在役；链写入经引擎 scribe 闸三与 attractor sign 专属 crosscheck 通道，零直写链文件。

## 收口附记（close 后补记）

（待 close 后回填：close 读数、撞名让位记录、三仓提交号、reconcile 读数、终态链 verify、主树名册复验）
