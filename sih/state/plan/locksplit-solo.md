# locksplit-solo：lease 锁机制三洞修复批（路径同一化 + 追加态锁 + 路径包含判定）

> task-packages 治理任务
> 承接：用户裁定 2026-09-03 插队令原话「链上冲突，可以插队，并不存在太大的冲突点」与同日质询「锁竞态导致现在无法协作了」；批群死复盘与 caswire 加 scriwire 实况坐实三洞即路径形态绕穿互斥（绝对与相对路径在锁账本中为不同资源）、共享追加面独占串行（追加本无害并发）、目录与文件包含不判（mathclose 目录锁与 scriwire 文件锁共存）；载体承 ORD-020 全序资源分配（ordwire-lease-solo 已接线），本批为其资源同一性与锁型语义的机械修正
> 用户裁定 2026-09-03 政策行照录「等我裁的东西首先过得一，得一有问题的异常才让我看」——纯机械即机械链全绿即放行
> 队形：单线形 solo，**本批单飞不与任何批并发**（修锁的批不能踩自己修的锁）
> 日期：2026-09-03
> 温故检索：materials/recall-locksplit.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，本批单飞如实记零并发

## 一、问题陈述 {#problem}

其一，normalize_path 只去 ./ 前缀不折算绝对相对（lockcore.py:262-268），两会话以不同形态锁同一路径互斥被绕穿（实况 24 笔绝对与 16 笔相对混用）。其二，追加面（trail 文件、CALL-LOG、reports、counts、台账、core 包）被独占锁串行化，并发批互憋即用户所谓无法协作。其三，目录锁与文件锁无包含判定，同底层面可双持。

## 二、关键设计 {#design}

1. **路径同一化**：normalize_path 升级为工作区根相对规范形即绝对路径剥根前缀、去 ./ 前缀、尾斜杠统一；锁账本新笔一律落规范形；互斥判定对历史笔双侧规范化后比较（账本 append-only 零改写）。
2. **追加态锁**：lease lock 增 --mode exclusive|append 缺省 exclusive 向后兼容；判定规则即 append 与 append 可共存、exclusive 与任何形态互斥；unlock 按持位放；scribe lockgate_guard 对 trail 追加操作认 append 持位。任务包共享追加面声明追加态即机械承载插队裁定。
3. **路径包含判定**：取锁冲突判定增包含规则即他方所持路径为本路径祖先目录或后代即冲突，堵目录文件双持洞。
4. CONTRACT 修订（锁型语义与三洞修复记录）、推导档增补（ORD-020 推导档补资源同一性与锁型节）、金向量四场景（同路径双 append 共存、append 与 exclusive 互斥、异形态同路径互斥即修复证、目录与文件包含互斥即修复证）。
5. 既有测试全绿即行为兼容面零破坏（缺省 exclusive 下旧调用形零变）。

## 三、工作清单 {#work}

- [x] 三洞修复实装与单测（lockcore 路径同一化 + 追加态锁 + 包含判定 + active_locks 多持位，56 测绿含新测五件；lockgate 十测绿含新测三件）
- [x] 金向量四场景双跑逐字节一致（append_coexistence / append_exclusive_conflict / normalization_same_path_conflict / inclusion_dir_file_conflict，冻结于 materials/locksplit-solo-golden-vector.json）
- [x] CONTRACT 与推导档增补（修订二十五升 1.14.0；ORD-020 推导档补 §3.4-3.6 资源同一性与锁型语义与路径包含判定）
- [x] 三步管线读数（CONTRACT 与推导档：化格零改动、核阅域外 exit-2 如实记、检词零违例）
- [x] 认证上链收约对表（五报告认证入链 28fc28a3/7c05a8ec/25e6a465/b966a148/23b92e32，三仓 settle 放锁 close 归并，reconcile 零新增违规，链 verify valid 173 事件）

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 同一化** | 工程 | 异形态同路径互斥生效，账本新笔全规范形，历史笔比较零改写 |
| **F-2 追加态** | 工程 | 双 append 共存、append 与 exclusive 互斥、缺省形向后兼容 |
| **F-3 包含判定** | 工程 | 目录锁与文件锁互斥生效 |
| **F-4 行为兼容** | 工程 | lease 既有测试全绿加新测全绿 |
| **F-5 写入仅 allow** | 治理 | 写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/src/lease/lockcore.py 即 normalize_path 与判定位
- 必读 2：sih-math/docs/ordwire-lease-derivation-2026-09-03.md ORD-020 接线推导
- 必读 3：sih-tools/lease/CONTRACT.md
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 本批单飞，不与任何批并发
2. 账本 append-only 零改写历史
3. 缺省 exclusive 向后兼容，旧调用形零破坏
4. 认证先落主树活链即插队合法（本批单飞无并发面）
5. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-tools/lease/
- sih-math/docs/
- sih-engine/sih/state/plan/locksplit-solo.md
- sih-engine/sih/event/plan/locksplit-solo-results.md
- sih-engine/sih/event/plan/locksplit-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [x] F-1 至 F-5 全过（F-1 同一化 / F-2 追加态 / F-3 包含判定 / F-4 行为兼容 / F-5 写入仅 allow，判据见四节，证据见结果档）
- [x] 金向量四场景在档可重放（materials/locksplit-solo-golden-vector.json 冻结，replay_golden.py 双跑逐字节一致）
- [x] 认证入链结算收约对表读数在档（五报告认证 28fc28a3/7c05a8ec/25e6a465/b966a148/23b92e32，三仓 settle 放锁 close 归并，reconcile 零新增违规，链 verify valid 173 事件）
