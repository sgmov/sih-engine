# deyimerge-switch-solo 结果档

> 得一融回三步曲第三步切换批收口档
> 日期：2026-09-02。会话号：aae7260a6e51c4c7。队形：单线形 solo。
> 承接：SPEC-014、deyimerge-sdd-solo、deyimerge-tdd-solo、核阅切换先例即 scrutmerge-switch-solo、pk-036、DEC-013

## 一、意图哈希与链位

- ask3 记录 sha256：`cec625348be962d6ae1270928d2179ad8f49dba0450d7154464e42fb1b343a43`
- 意图事件：`intent_refined`，event_hash 前八位 `3178adf3`（事件号 b4e7fab7-a9ea-43ee-9075-04c5bed93374）
- 意图前链对表：91 事件（末哈希 `8c1ff82b`，valid，含本会话例行读数三件）；意图后 92 事件（末哈希 `3178adf3`，valid）
- 双门：scrutinator ask3 包零违规 exit 0；ask3repeater status ok（三锚，引文程序切片自 06-on-canon 73 行、07-on-assay 65 行、08-on-settle 114 行逐字节子串）
- 叩问：四词（换旗、退役标注、强制位、出泊）三信号轻级 unregistered，digest passed 3/3，处置行落包档叩问处置节（换旗与强制位前批已消解，退役标注与出泊本批消解补行）
- 正身：identity verify anomalies 空（reports/2026-09-02-deyisw-identity.json，不入版控）
- inputlog：2026-09-02.ndjson seq 10 与 seq 11 补录逐字两笔（sess-zcode-260902-acceptor，数学仓并线协调与解锁确认；seq 9 前批已载未重录）

## 二、换旗与 diff 自证（F-1）

- BATCH-FACE.md 工地 diff 共 49 行全为追加即 § 五、判定与采样调用面（得一融回后）两段：执契段 check/verify/sign/watch 调用形改指引擎件 `cd $ROOT/sih-engine && target/debug/attractor`，围堰旧形 `uv run tally` 注明仅留兼容只读验证位；facet 测量段承双模并存条款，采样三段留围堰、引擎机械腿经子进程与文件合同调两段式；宪法节与既有十段一字未碰，字节级 diff 自证仅限换旗行
- DEC-020 勘查结论：全文件无调用路径表述位（grep uv run tally / tally check / target/debug / uv run facet / facet measure 零命中），依 dispatch 若有则勘正条款判零碰，主树与工地双对表 HEAD 一致
- 证据件：materials/golden-rerecord.log 同录换旗 diff 统计与 DEC-020 零碰对表

## 三、退役标注（F-2）

- tally/CONTRACT.md 增 § 退役登记：判定职能强制位自 2026-09-02 起 check/verify/sign/watch 以引擎件 target/debug/attractor 为正典，本工具转兼容只读，src 与 tests 零改动，退役的是判定终签的强制执行位不是工具生命，退役非删除
- facet/CONTRACT.md 首立（本批新建）：采样腿契约壳与退役登记，合同模式机械内核已融回引擎件，判定职能强制位改指引擎件，facet CLI 采样观察能力双模并存保留即 GOV-002 判据四字面，measure 与 singleseat 与 dose_driver 三腿 CLI 原位不动
- facet 与 tally 两 CALL-LOG 各落尾行（tally 册同批按时间序回灌主树在途 p3xcarr 一行承 deyimerge-tdd 段2 先例）
- facet 与 tally 源码零改动：两仓 git status 对表 src 与 tests 零碰（本批双仓 settle 只动工地批件与链与账本）

## 四、完成档三查与文档四件（F-3）

- 完成档 mergeback-attractor-completion-2026-09-02.md 落 sih/event/mergeback/，三查全过：一查 SPEC-014 验收判据 A1-A5 逐条证据引 deyimerge-tdd 结果档与材料件；二查接口契约未变即两 CONTRACT 不删接口仅加注、四类 json 工件逐字节（浮点文本形豁免引用腿切分清单边界声明）；三查回迁债清账即 serde_yaml 已声明、腿切分二十三行零偏差、判据 v3 闸未融回如实列明属围堰采样腿边界
- GOV-003 v1.7 结算追加：判定器席实例得一归位引擎侧，组件六席实体全在 src 即三问 ask3repeater、参验 scrutinator、书简 event_stream、判定器 attractor、视图 view、温故 retriever，GOV-002 闭合判据一"任务包 007 至 013 执行完毕并关闭且六组件落地 src"六席子项闭项，GOV-002 判据文本零改，一页为限
- DEC-013 v1.2 修订二：得一融回三步曲第三步执行记录与三查对表
- SPEC-014 修订一：切换执行记录（双跑实录、换旗、退役、金向量重录归档笔）

## 五、pk-036 金向量重录出泊（F-4）

- 重录：des-001-gov003.json 只刷 content_hashes 期望值 6bb38a7f（陈旧）→ 76ef97b1（现行 GOV-003 v1.7 定稿真实内容 sha256，引擎件实跑产出非手造），targets 与 findings 与 summary 与断言逻辑零碰（875 字节 → 875 字节，diff 仅一行）
- 双跑 cmp：主树真实域内路径上围堰件与引擎件 exit 0/0 且 cmp IDENTICAL（围堰为基准的字节平价）；v1.7 内容经引擎件实跑 findings 零、哈希 76ef97b1 与 sha256 和对
- 断言逻辑零改：src/scrutinator/tests.rs 未动，金向量重录只刷 fixtures 数据文件（红线一允许例外如实申报）
- cargo test golden_des001_gov003：归并前主树尚持 v1.6 内容而金向量记 v1.7 哈希即目标内容态先于工地态，红属预期非回归；归并后主树 v1.7 与金向量一致转绿，绿证入 materials/postmerge-verification.log
- 出泊：pk-036-exit.json 落 sih-tools/parking/materials/（promoted，裁决即用户切换放行令出泊本批承接），parking_exited 事件入当日链承无主出拒门（pk-036 进泊事件在链 55513c24），PARKING-v1.md 名册在泊五项改四项、pk-036 转历史住户第五项

## 六、收口对表（F-5）

- 双仓 settle：tools 与 engine 各段1 settle 提交，链上认证哈希为凭
- close：归并对表法 diff identical（主树同名未跟踪包档与材料目录备份让位）
- reconcile：双仓四类全零读数见认证节后对表
- 链 verify：valid（意图前后与 settle 后三对表）
- 调用册六件留痕：scribe、formatter、scrutinator、nomenclator、facet、tally 各一行随批提交

## 七、F 表

| F | 类别 | 判据 | 结论 |
|---|---|---|---|
| F-1 | 工程治理 | BATCH-FACE 执契段与 DEC-020 关联表述改引擎件，宪法节字节 diff 自证零碰 | 过（第二节；BATCH-FACE 追加 49 行零改旧文，DEC-020 无调用位表述零碰双对表） |
| F-2 | 工程治理 | 两 CONTRACT 退役标注与调用册尾行在，src 零改动 | 过（第三节；facet CONTRACT 首立、tally CONTRACT 增节、两 CALL-LOG 尾行、双仓 src 零碰） |
| F-3 | 链上治理 | 三查完成档落 mergeback/、GOV-003 v1.7 判据一闭项表述、DEC-013 与 SPEC-014 修订在场 | 过（第四节；四件全落，管线读数见认证节） |
| F-4 | 工程治理 | 金向量重录与双跑 cmp IDENTICAL 证据在、出泊事件入链、PARKING 名册更新 | 过（第五节；重录仅一行哈希值、双跑 IDENTICAL、出泊事件与名册更新随批） |
| F-5 | 链上治理 | 双仓 settle 归并、reconcile 四类双零、链 valid、全量入版控 | 过（第六节 + 认证节读数） |

## 八、越线与误差申报

1. 金向量重录触及红线一允许例外：des-001-gov003.json 为 fixtures 数据文件非源码，只刷 content_hashes 期望值一行，断言逻辑与 src/scrutinator/tests.rs 零碰，如实申报
2. 链尾对表：dispatch 基线 88 事件；意图前实为 91 事件，差 3 件即本会话会话启动例行读数 gauge 三维落链（reading_recorded 三件，会话启动义务），承尾随申报先例
3. pk-036 cargo 绿证时序：金向量记 v1.7 哈希而主树 GOV-003 在归并前为 v1.6，golden_des001_gov003 归并前红属预期态（目标路径持旧内容），归并后转绿为成立判据，绿证 postmerge-verification.log；归并前绿证以围堰-引擎双跑 cmp IDENTICAL 与 v1.7 实跑零违规承载
4. 包档与完成档与结果档域外 exit-2 如实记：des-001 域只盖 sih-engine/doc，state/plan 与 event/plan 与 event/mergeback 与 sih-tools 目标核阅 exit-2 不属违规，逐件读数入 materials/pipeline.log
5. 工具侧文件不在引擎文档规范域：BATCH-FACE 与两 CONTRACT 走化格归一与检词核查，核阅域外 exit-2 如实记
6. 判据 v3 闸未融回：属围堰采样腿边界，SPEC-014 腿切分清单外不迁，完成档第三查如实列明
7. 包档勾选：任务包工作清单五项随本结果档全勾，验收标准 F-1 至 F-5 全过勾选随批归档
