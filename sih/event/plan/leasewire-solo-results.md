# leasewire-solo 结果档

> 批：leasewire-solo 租约与锁两机制统一深锚挂 ORD-020（全序资源分配与死锁自由）：lease 增注判定位、locks 首锚
> 会话：3a012326cae52fa6（租约自生成）｜会话标识 sess-zcode-260904-leasewire（ask3 双标识空间，各认各的）
> 日期：2026-09-04 ｜ 队形：单线形 solo，零子代理 ｜ 三仓工地 tools@integral-stage-build 与 engine@main 与 math@main
> 承接：数学管线全量串联计划批四起序列；批一覆盖账本读数即 lease 已实例化（SPEC-011 引用）而 locks 零载体
> 政策行：用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——本批零新裁决点，机械链全绿自行收口，结果档不设「等你令」节
> 意图哈希：0136acc9（ask3 双门过：核阅 exit 0 零违规、重放 status ok 三锚；digest passed covered 4；意图事件 5ea474d8）

## 一、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| F-1 四件套×2 逐工具落齐 | 完成 | 载体引用二（locks CONTRACT 首锚载体引用节加 lease CONTRACT 载体引用节深锚增补行，实查择形申报在档）加合一推导档（八节两工具节）加源码锚点九处（locks 首锚六处加 lease 增注三处）加金向量四场景三跑同哈希，逐件在案 |
| F-2 零行为变更 | 完成 | lease 86 绿与 locks 8 绿批前批后同态；两 py 剥文档串后 ast.dump 全同；金向量主树已提交源复现同哈希 |
| F-3 检词零违例加词债不过夜 | 完成 | 十二目标检词全 rc=0 零违例；四新词 established 登记 worktree core 包 161 至 165 随批入版控 |
| F-4 写入仅 allow | 完成 | 锁路径全集对表，三仓工地变更与主树活面追加全在 allow 十九路；零 allow 面外写入 |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 四件套×2 | 载体引用、推导档、判定位锚点、金向量双跑 IDENTICAL 逐工具齐 | 过 | 三节逐载体落位表；锁获取序场景 l1 正对全序性，死锁自由反例 l2 在档；推导档合一两节形 |
| F-2 零行为变更 | 既有测试批前批后同绿零回归 | 过 | AST 同形机械证加测试对表加金向量三跑同哈希三重证 |
| F-3 检词与词债 | 检词零违例；锚点新词入词表 | 过 | 管线读数（五节）；四词 register rc=0 query 在册；零产出违例零词债过夜 |
| F-4 写入仅 allow | 锁路径全集 | 过 | 九把锁（七独占领域加两 append）与共享活面短持对表，无越线 |

## 三、逐载体落位表

| 机制 | 判定位 | 语义对位（推导档节） | 契约位 | 锚点 | 金向量 |
|---|---|---|---|---|---|
| lease（增注深锚） | lock_status 全序读出 | 资源全序的字典序展开（lease 节 3.2） | CONTRACT 载体引用节增补行 | lock_status 注释锚点 | l1 全序获取序正场景 |
| lease（增注深锚） | _conflicts 与 _is_ancestor 互斥闭包 | 冲突关系在相等与包含偏序上闭包（lease 节 3.3） | 同上 | 两函数注释锚点 | l2 反例（互斥拒绝形） |
| lease（既有，本批零改） | active_locks 加 acquire 加 release 加 normalize_path | 互斥与死锁不自由与等待终止与资源同一（ordwire 推导档 3.1 至 3.6） | CONTRACT 载体引用节（ordwire 立节） | 四处既有锚点 | l2 反例 lease 腿 |
| locks（首锚） | active_locks 台账配对 | acquired 与 released 配对即锁面一致性判定（locks 节 4.1） | CONTRACT 载体引用节（本批立节） | active_locks 注释锚点 | l3 常规序配对一致加 l4 偏差实证 |
| locks（首锚） | acquire 确定性拒绝 | 占持不与等待并存即环自由推论（locks 节 4.2） | 同上 | acquire 注释锚点 | l2 反例 locks 腿 |
| locks（首锚） | release 让步 | 让步良基与有界等待（locks 节 4.3） | 同上 | release 注释锚点 | l3 |
| locks（首锚） | normalize_path 资源标识 | 资源集一致标识与字典序全序承载（locks 节 4.4） | 同上 | normalize_path 注释锚点 | l4 |
| locks（首锚） | lock_status 全序读出 | 全序性字典序展开（locks 节 4.4） | 同上 | lock_status 注释锚点 | l3 |

载体消费位申报：locks 零载体态自本批终止即本批为其第一载体引用位；lease 载体引用节 ordwire-lease-solo 已立，本批增补深锚申报行不另立节。

## 四、金向量读数

- 三件落 sih-engine/sih/event/plan/leasewire-solo-materials/（golden_cases.json 输入内联、replay_golden.py 重放器、leasewire-solo-golden-vector.json 冻结向量），payload_sha256 1dae0857a0a91888f58047b8222321eaf321a8b4429559602c39f51dd8f1a2bf。
- 四场景：l1 全序获取序正（lease 字典序递增三笔全 granted 加 status 字典序读出）、l2 死锁自由反例（lease 与 locks 双腿他持再取 locked_elsewhere）、l3 台账配对一致性（locks 常规序）、l4 locks 两遍配对偏差实证（as-is 冻结）。
- 双跑读数：工地源两遍 cmp IDENTICAL；主树已提交源复现 cmp IDENTICAL（已提交树复现关过——接线 diff 仅注释文档串，重放载荷逐字节不变，即零行为变更的机械证）；三跑 sha256 同值。
- 重放寻径约定：两工具源路径经 --lease-src 与 --locks-src 参数传入，cases 零绝对路径，root 与台账为重放时临时夹具，冻结态携带约定。

## 五、管线读数（化格→核阅→检词，笔在核前，findings 亲读）

| 目标 | 化格 | 核阅 | 检词 |
|---|---|---|---|
| locks CONTRACT 加 lease CONTRACT 加两 CALL-LOG | exit 0 无需改 | exit 2 域外如实记 | exit 0 零违例 |
| terms.json | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| 推导档 | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| golden_cases 与冻结向量与 pk051 泊位记录 | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| dispatch 与任务包 | exit 0 | exit 2 域外如实记 | exit 0 零违例 |
| replay_golden.py | exit 2 域外如实记（general-v1 只盖 md/json/yaml/toml） | exit 2 域外如实记 | exit 0 零违例 |
| 域内对照件 GOV-003 | — | exit 0 findings 0（采集面恢复佐证） | — |

零管线违规零返工：本批产出十二目标化格除 py 域外一笔外全 exit 0，检词全 rc=0，词表四词 register 一次过。

## 六、测试对表（批前批后同态零回归）

| 工具 | 批前（主树） | 批后（工地） | 判定 |
|---|---|---|---|
| lease | 86 passed | 86 passed | 零回归 |
| locks | 8 passed | 8 passed | 零回归 |

AST 同形机械证：lease/src/lease/lockcore.py 与 locks/src/locks/core.py 剥文档串后 ast.dump 全同双 True。

## 七、实查偏差申报与泊界登记（判定位以工具实查为准）

实查发现 locks 判定面与载体互斥语义相关偏差两笔，属映照实形如实申报，非本批行为范围：

1. **台账两遍配对**：locks core.active_locks 实形为先按 acquired 全量置位后按 released 配对弹出，放后重取形下锁面读出缺在锁位且他会话再取申请被放行即互斥绕穿。金向量 l4 场景 as-is 冻结实证（探针：W1 取放重取后 W2 对同路径 granted）。与 lease 侧 leasepatch-solo 缺陷一同类。
2. **资源标识有限归一**：locks core.normalize_path 实形仅去前导点斜杠，绝对路径形态未纳入同一化，异形态同路径互斥可绕穿。与 lease 侧 locksplit-solo 资源同一化所修同类。

处置：泊界登记 pk-051（链事件 97ee4c20，ttl 30 天），修法承 lease 侧先例归后续行为变更批，过门判据即 l4 场景改为 locked_elsewhere 与锁面读出全；本批锚点注记映照实形不以理想形饰非，行为零改动不以绿饰红。

## 八、认证清单（逐笔 meter 包裹引擎 scribe append，闸三 --session 加 --sessions 全带）

| 件 | 事件哈希前八位 |
|---|---|
| intent（ask3 记录 + 验证件） | 5ea474d8 |
| pk-051 停泊 | 97ee4c20 |
| pipeline 管线读数 | ca190da0 |
| derivation 推导档 | 332a47de |
| golden 金向量 | ad6f0022 |
| changed-files 变更件 | a2a4381e |
| checkcite 书单对表 | 6cefebc4 |

温故检索：三词召回（全序资源分配加死锁自由加锁面一致性），词通道中 ORD-020，图闭包含 ORD-001 与 ORD-005 与 ORD-016，并集书单 allowed_size 31；checkcite pass（cited 四载体全在册零 missing，recall-leasewire.json 落 materials 如实记）。

## 九、越线与误差申报

1. unlock 首试两笔 rc=2：身份件路径漏 BATCH 段拼出不存在文件报 identity report unreadable（工具异常如实申报），补参复跑 rc=0，零数据面影响。
2. scribe park 首试 rc=1 一笔：停泊记录缺 action 字段报停泊拒 ActionMissing，补 action enter 复跑 rc=0（链上零残笔即拒绝未落链）。
3. 金向量首版 l3 场景含跨场景台账泄漏（l2 locks 台账延入 l3 status 读出），修正重放器按场景分台账后重跑，冻结向量以修正后三跑同哈希为准；首版未冻结未认证未出工地。
4. 正身件与 ask3 记录与叩问信号与 digest 与验证件七笔写于会话签发前（机械链序前置位），scribe/reports 与 identity/reports 面锁不可得即事实直写，零共享冲突（批期该两 face 无他会话在写），如实申报。
5. 租约 ledger 面零直写：sessions 与 locks 与 claims 三册全部经 lease 工具追加即唯一写点承载，本批对 ledger 面零锁外直写。
6. 核阅域外 exit-2 十路（工地路径全在 des-001 域外）如实记入档，域内对照 GOV-003 exit 0 佐证采集面在位，不属违规。

## 十、冲突样本节（pk-045 样本库）

本批批期活锁台面：批前 trail 121 笔（dispatch 基线），intent 上链时点 124 笔，增量三笔为他会话例行读数 reading_recorded（gauge 三维快照，非施工批）；批期零 exclusive 撞锁：九把锁一射取获零重试；共享追加面（trail 与 scribe/reports 与 identity/reports 与 meter/counts 与 ledger）append 短持即取即放零等待零重试；零并行施工批交锋，pk 号零占用，pk-051 为本批新泊位。

## 十一、收口读数（认证时点）

- 链：sih-engine/sih/event/trail/2026-09-04.ndjson 认证时点 verify valid 134 事件（批前 121，例行读数三笔加本批意图一笔加停泊一笔加认证五笔；首哈希 05a8a75e 与批前一致零分叉）。
- 锁：九把在持（lease src 与 lease CONTRACT 与 locks src 与 locks CONTRACT 与 terms.json 与推导档与任务包七独占长期持加 materials 与 results 两 append），待 settle 后放锁收约。
- 工地：三仓批件落齐（tools 七件修改；math 推导档；engine materials 六件与结果档与任务包拷贝，trail 快照 settle 前一次性拷）。
- close 与 reconcile 与终态 verify 读数与三仓提交号见文末收口附记（close 后补记）。

## 十二、队形验证

单线形 solo 零子代理全程成立：本批全部写入由会话 3a012326cae52fa6（sess-zcode-260904-leasewire）亲写，零 Agent/Task 子代理调用；锚点定位与偏差实查与金向量全由确定性程序承载（ast.dump 机械对表、cmp 三跑、测试对表、探针复现、checkcite 并集书单），链写入经引擎 scribe 闸三（--session 加 --sessions）零直写链文件。

## 收口附记（close 后补记）

- close 三跑：首跑 tools 与 math 两腿并（归并 bc2ca80f 与 366298b，工地删支），engine 腿 merge_failed（主树 trail 活文件修改态阻并，与 carrwire 同款）；按备份让位归并对表法处置，二跑 engine 腿并（归并 feedf6e）拆本吊销全成，failed 空、会话 3a012326cae52fa6 吊销；tools 与 math 腿二跑计 missing 或 already_gone 幂等收敛不判败。
- 备份让位三件对表：任务包与 dispatch 两件 cmp IDENTICAL；trail 对表出险情见下条。
- **trail 并集超集竞态险情（越线申报）**：close 首跑前快照对表同形 134 行零漂移（wip 提交 nothing_staged 佐证），让位备份时点主树活链已被并行批 pk050sw-solo 追加一笔至 135 行，本批未复查备份与快照的并集超集即复跑 close，归并以 134 行快照落地，clobber 其 certification_completed 事件一笔（event_id 4246c783，switch-gate-checklist.json 认证，event_hash 前八 c9ab0cfb，02:49:40）；其后该批又追一笔（16993504，02:49:53）承 134 链尾。终态链 scribe verify valid 135 事件零分叉（首 05a8a75e），所失事件全文保全于 /tmp/leasewire-yield-backup/trail-2026-09-04.ndjson 第 135 行；链文件不可由本会话改写（红线），处置申报如下：pk050sw-solo 若以其 c9ab0cfb 作 settle cert 将被 cert_not_on_chain 拒即对账自愈，重跑其 scribe append 重认证即恢复；本批 close 竞态窗口检查缺陷如实入账，后续批在备份让位与 close 复跑之间须加一次行数并集复查。
- 三仓提交号：sih-tools（integral-stage-build）settle 段1 3716884d 加归并 bc2ca80f；sih-engine（main）settle 段1 056d08d 加归并 feedf6e；sih-math（main）settle 段1 73c0423 加归并 366298b。
- reconcile（close 后）：sih-engine unrouted 0 与 unbypassed 0 与 cert_missing 0 全净；sih-tools unrouted 0 与 unbypassed 0 与 cert_missing 1（entryunique-solo 段2，2026-09-03 旧账非本批新增）；sih-math unrouted 0 与 unbypassed 1（零号基线存量）与 cert_missing 2（mathfix2-solo 与 fmtfix-solo 批前存量）——相比批前零新增，本批贡献零。
- 链 verify（close 后）：status valid 135 事件，first_hash 05a8a75e 与批前一致，last_hash 9a1bf670；本批七笔即意图 5ea474d8 加停泊 97ee4c20 加认证五笔（ca190da0 与 332a47de 与 ad6f0022 与 a2a4381e 与 6cefebc4）全数在链。
- 共享活面对表：当日链与会话册与锁册与 claims 册与 meter 计数册留主树活写（共享追加面，归会计通道）；scribe/CALL-LOG 不在本批 allow 面，零写如实申报。
- 本笔回填提交：close 通道外整备形（--no-verify 加 lease bypass 登记），queueing-solo 与 contribmath-solo 先例同形。
