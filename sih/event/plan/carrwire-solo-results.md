# carrwire-solo 结果档

> 批：carrwire-solo 四新载体接线批（ORD-023→formatter 归一化收敛位、PROB-016→meter 计数对账位、ORD-024→nomenclator 命中跨度位、ALG-012→parser 有序选择位）
> 会话：ba187edb54e1ee7a（租约自生成）｜会话标识 sess-zcode-260904-carrwire（ask3 双标识空间，各认各的）
> 日期：2026-09-04 ｜ 队形：单线形 solo，零子代理 ｜ 冲突模式成员（pk-045 样本库，与 pk037impl-solo 同日并行）
> 承接：newcarr-solo 终签四载体其 F-4 零接线遗留「接线归后续逐工具批」即本批；四件套形制承 latexwire（四载体一工具）与 locatorwire（一载体一工具）先例
> 政策行：用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——本批零新裁决点，机械链全绿自行收口，结果档不设「等你令」节
> 意图哈希：73914923dfe1c644（会话台账 record_sha256 对表一致，ask3 双门过：核阅 exit 0 零违规、重放 status ok 三锚；意图事件 0a573995）

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| F-1 四件套×4 逐工具落齐 | 完成 | 载体引用节四（meter/nomenclator/parser CONTRACT 加 formatter SPEC 择形申报）加合一推导档（七节四载体）加源码锚点八处加金向量三件双跑 IDENTICAL，逐件在案 |
| F-2 零行为变更 | 完成 | 五 py 剥文档串后 ast.dump 全同；四工具测试批前批后同态（formatter 13 绿、meter 16 绿、nomenclator 29 绿 1 存量红同词同理、parser 79 测 2 存量红同名同理）；金向量主树已提交源复现 IDENTICAL |
| F-3 检词零违例加词债不过夜 | 完成 | 十三目标检词十二 rc=0 零违例，replay_golden.py 首跑一处本批产出死档词命中即改复跑 rc=0，nomenclator CONTRACT rc=1 六处系批前存量登记债（主树批前件同词同理）；四新词 established 登记词表 157→161 随批入版控 |
| F-4 写入仅 allow | 完成 | 锁路径全集对表，三仓工地变更与主树活面追加全在 allow；零 allow 面外写入 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 四件套×4 | 载体引用、推导档、判定位锚点、金向量双跑 IDENTICAL 逐工具齐 | 过 | 二节逐载体落位表；formatter 幂等场景（同一输入二遍格式化输出恒同）在案；推导档合一四节形 |
| F-2 零行为变更 | 既有测试批前批后同绿零回归 | 过 | AST 同形机械证加测试对表加金向量双树复现三重证 |
| F-3 检词与词债 | 检词零违例；锚点新词入词表 | 过 | 管线读数（四节）；本批唯一产出违例即改；四词登记 query established 实证 |
| F-4 写入仅 allow | 锁路径全集 | 过 | 十二把锁（八独占领域加四追加）与共享活面对表，无越线（math/docs 锁争议见冲突样本节） |

## 三、逐载体落位表

| 载体 | 工具与判定位 | 语义对位（推导档节） | 契约位 | 锚点 | 金向量场景 |
|---|---|---|---|---|---|
| ORD-023 重写系统确定性范式（mapping.md:209） | formatter apply_ops 归一化收敛位 | 声明序改写列（定理三）、不动点幂等重跑落零（定理二）、表外拒绝（公理二） | SPEC.md 载体引用节（无 CONTRACT 择 SPEC 实查申报） | format.py 模块串加 apply_ops 两处 | f1 幂等（二遍恒同）加 f2 声明序与拒绝 |
| PROB-016 计数测度与可加性（mapping.md:211） | meter cmd_run 与 cmd_count 与 cmd_crosscheck 计数对账位 | 单点赋值（公理一）、划分求和对账恒等式（定理二）、漏计检出（定理三） | CONTRACT.md 载体引用节 | cli.py 三处 | m1 三方恒等式加 m2 漏计检出恰一笔 |
| ORD-024 区间序与串跨度匹配（mapping.md:210） | nomenclator iter_spans 与 run_check 命中跨度位 | 跨度即半开区间（公理一二）、互斥链（定理一）、采纳唯一（定理三形态与公理三） | CONTRACT.md 载体引用节 | matching.py 加 check.py 两处 | n1 升序互斥链加 n2 唯一采纳 |
| ALG-012 形式文法与解析确定性（mapping.md:212） | parser match choice 有序选择位 | 优先全序消歧（定理一）、项代数唯一构造（定理二）、同类先断（公理二） | CONTRACT.md 载体引用节 | peg.py choice 分支一处 | p1 声明序优先与落次候选加 p2 玩具包双跑 |

四载体消费位申报：四条目本批前零工程消费位，本批各为其第一消费位。

## 四、金向量读数

- 三件落 sih-engine/sih/event/plan/carrwire-solo-materials/（golden_cases.json 输入内联、replay_golden.py 重放器、carrwire-solo-golden-vector.json 冻结向量），payload_sha256 960f6093。
- 双跑读数：工地源双跑 IDENTICAL；主树已提交源复现 IDENTICAL（V6 已提交树复现关过——接线 diff 仅注释文档串，重放载荷逐字节不变，即零行为变更的机械证）；注释改词后复跑 IDENTICAL。
- 重放寻径约定：四工具源路径与格式包与玩具包全参数传入，cases 零绝对路径，冻结态携带约定。

## 五、管线读数（化格→核阅→检词，笔在核前）

| 目标 | 化格 | 核阅 | 检词 |
|---|---|---|---|
| 四契约节（三 CONTRACT 加 SPEC） | exit 0 无需改 | exit 2 域外如实记 | exit 0 零违例 |
| 四 CALL-LOG（parser 新立） | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| terms.json | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| 推导档 | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| golden_cases.json 与冻结向量 | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| replay_golden.py | exit 2 域外如实记（general-v1 只盖 md/json/yaml/toml） | exit 2 域外如实记 | 首跑 rc=1 一处（本批注释词面撞死档词，逐字明细在已认证管线报告）→即改→复跑 rc=0 |
| 域内对照件 GOV-003 | — | exit 0（采集面恢复佐证） | — |

nomenclator CONTRACT rc=1 六处存量词全在融回门与修订记录既有节（逐字明细在已认证管线报告），主树批前件同词同理由、行号位移 4 行，系批前存量登记债非本批产出，批前批后同态如实记。核阅与检词首轮采集面各踩一次命令替换吞 `$?`（locatorwire 越线申报 2 同款陷阱），直跑复验订正，读数以订正后为准。词债：四新词 established 入工地 core 包（157→161，外科式插入保持批前字节形态），query 实证 established 四笔；terms.json 规范化形态红测批前批后同态，系并集归并 b2584835 遗留非本批产出。

## 六、测试对表（批前批后同态零回归）

| 工具 | 批前（主树） | 批后（工地） | 判定 |
|---|---|---|---|
| formatter | 13 passed | 13 passed | 零回归 |
| meter | 16 passed | 16 passed | 零回归 |
| nomenclator | 29 passed 1 failed（规范化形态） | 29 passed 1 failed（同测试同理由） | 零回归（存量红同态） |
| parser（unittest） | 79 测 2 failed（f2_xref 与 tokencap 语料漂移） | 79 测 2 failed（同名同理由） | 零回归（存量红同态） |

## 七、认证清单

| 事件哈希 | 对象 | 备注 |
|---|---|---|
| 0a573995 | 意图笔（scribe intent，meter 包裹） | ask3 三锚（07-on-assay L61、06-on-canon L185、08-on-settle L108 程序切片逐字节），digest passed covered 4 |
| 526e33ca | 认证：pipeline 报告 | meter 包裹 append 主树活链 |
| 4ec3e0f1 | 认证：derivation 报告 | meter 包裹 append 主树活链 |
| aad981c6 | 认证：golden 报告 | meter 包裹 append 主树活链 |
| 526a6d32 | 认证：changed-files 报告 | meter 包裹 append 主树活链 |
| 93266461 | 认证：checkcite 报告 | meter 包裹 append 主树活链 |

温故检索：四单词召回并集（确定性范式/计数测度/区间序/解析确定性），词通道各中本批四载体，并集书单 35 件；checkcite pass（cited 四载体全在册零 missing，report 与 plan 落 scribe/reports）；recall-carrwire.json 落 materials 如实记。checkcite 首跑拦截一次如实记：推导档引了工程规范号 SPEC-015（非数学仓概念不在书单），修档去该引用后 pass。

## 八、越线与误差申报

1. 调用面误差一批（trail 短锁未取得）：五笔认证的 trail append 短锁因 --identity 相对路径传参未取得（lock rc=1），五笔 append 在追加共存态直落成功（唯一他持方 pk037impl-solo 为 append 模式按设计共存）；scribe CALL-LOG 笔起修正为绝对路径，锁取得（rc=0）即取即放；零数据面影响（链 verify valid 零分叉）。
2. 采集面误差两笔（命令替换吞 `$?`）：核阅与检词首轮循环在 echo 内联命令替换吃掉 `$?` 误读 exit 0（locatorwire 越线申报 2 同款陷阱），直跑复验订正：核阅七目标实为 exit 2 域外如实记，检词订正后如五节表；读数以订正后为准。
3. 化格首轮采集路径错一笔：引擎工地 json 两件首轮以子壳 cd 后 `$PWD` 拼路径指向不存在位置 exit 2，修正为绝对路径复跑 exit 0 零改动；冻结件零字节变动。
4. 本批产出检词违例一笔（已修）：replay_golden.py 注释一处词面撞死档词（逐字明细在已认证管线报告），改词后复跑 rc=0；金向量复跑 IDENTICAL 佐证零行为面影响。
5. checkcite 首投拦截一笔（已修）：见七节末。
6. unlock 旗标误差一笔：CALL-LOG 锁释放首跑带 --mode 旗标报 rc=2（unlock 无该旗标），去旗标复跑 rc=0。

## 九、冲突样本节（pk-045 样本库）

本批与 pk037impl-solo（retriever/温故向，744d2c9fbcb173a2，00:23 开工）同日并行，用户侧筹备面在 dispatch 申报为「施工面与本批四工具不相交」；实测五处交锋逐条如下：

1. **exclusive 撞锁有限重试（未决项，见收口附记）**：sih-math/docs/ 推导档目标 exclusive 锁被其 exclusive sih-math/docs 压制，重试 4 次逐次计数在案，settle 前继续重试至十次上限，超限即停批申报不硬闯。
2. **施工面改道**：sih-tools/meter/ 目录 exclusive 被其 append sih-tools/meter/counts 压制，改取子路径三把 exclusive（src/ 与 CONTRACT.md 与 CALL-LOG.md，与 meter/counts 互不包含）全得；施工面不相交的 dispatch 申报经改道成立。
3. **共享追加面共存**：其 append 长持面（trail 与 scribe/reports 与 identity/reports 与 meter/counts 与 engine event/plan）与本批 append 短持即取即放共存零拒录；链共笔全程 verify valid 零分叉（首哈希 05a8a75e 不变，批期链 99→110 事件含他批 parking 笔共笔）。
4. **追加态锁多持**：本批 engine materials 与 results 两子路径 append 锁在其父目录 append 长持下取得成功（rc=0），追加共存按设计工作。
5. **lease open 撞领登记**：本包 claim 一笔在先（00:30:12，ttl 480 分），open 正常签发零争用；claims_warning 如实回显本批自有 claim。

## 十、收口读数（认证时点）

- 链：sih-engine/sih/event/trail/2026-09-04.ndjson 认证时点 verify valid 110 事件（批前 99，本批新增意图一笔加认证五笔；首哈希 05a8a75e 与批前一致零分叉）。
- 锁：十二把在持（formatter/ 与 nomenclator/ 与 parser/ 与 meter/src/ 与 meter/CONTRACT.md 与 meter/CALL-LOG.md 与 state/plan/carrwire-solo.md 七独占长期持加 materials/ 与 results/ 与 CALL-LOG 各面追加），待 settle 后放锁收约。
- 工地：三仓批件落齐（tools 十四件变更加 parser CALL-LOG 新立；math 推导档；engine materials 与结果档与任务包与链拷贝）。
- close 与 reconcile 与终态 verify 读数与三仓提交号见文末收口附记（close 后补记）。

## 十一、队形验证

单线形 solo 零子代理全程成立：本批全部写入由会话 ba187edb54e1ee7a（sess-zcode-260904-carrwire）亲写，零 Agent/Task 子代理调用；锚点定位与接线与金向量全由确定性程序承载（ast.dump 机械对表、cmp 双跑、测试对表、checkcite 并集书单），链写入经引擎 scribe 闸三（--session 加 --sessions）零直写链文件。

## 收口附记（close 后补记）

- math/docs 锁终局：pk037impl-solo exclusive sih-math/docs 于本批批中释放，第 9 次重试得手（01:08:51），取得即证面空闲；close 前按锁清零要求放回，随即 close 归并，未越他 session 主张。
- close 三跑：首跑拒于本会话 math 锁在持（锁清零前置）；二跑拒于闸四前置态（引擎 trail 工地快照与主树活链分叉，并行批续写所致），按并集超集条款以主树为权威源刷新快照、scribe verify 过、wip 提交 19434f3 消解；三跑引擎腿 merge_failed（主树 trail 活文件修改态加同名未跟踪件撞），tools 与 math 两腿先并；按备份让位归并对表法四步：备份 trail 与 dispatch 与任务包三件（/tmp/carrwire-yield-backup）、让位、close 复跑成（failed 空、会话 ba187edb54e1ee7a 吊销）、备份与归并结果逐件 cmp 全数 IDENTICAL 零丢失零漂移。
- 三仓提交号：sih-tools 段1 84341269 加归并 52a830b5；sih-engine 段1 9655b71 加 trail 刷新 wip 19434f3 加归并 bd023b5；sih-math 段1 d502ef1 加归并 9c07bd2。
- reconcile（close 后）：sih-engine unrouted 0 与 unbypassed 0 与 cert_missing 0 全净；sih-tools unrouted 0 与 unbypassed 0 与 cert_missing 1（entryunique-solo 批前存量）；sih-math unrouted 0 与 unbypassed 1（零号基线初始提交存量）与 cert_missing 2（mathfix2-solo 与 fmtfix-solo 批前存量）——相比批前零新增，本批贡献零。
- 链 verify（close 后）：status valid 114 事件，first_hash 05a8a75e 与批前一致零分叉，last_hash f9520776；本批六笔即意图 0a573995 加认证五笔（526e33ca 与 4ec3e0f1 与 aad981c6 与 526a6d32 与 93266461），他批共笔并存。
- 共享活面对表：引擎 trail 与 tools 锁册与会话册与 claims 册与 meter 计数册与两 CALL-LOG 留主树活写（共享追加面，归主会会计通道），本批 scribe CALL-LOG 一笔主树直append（短锁在持）。
- 本笔回填提交：close 通道外 wip 形（--no-verify 加 lease bypass 登记），queueing 9806c1d 与 newcarr 先例同形。
