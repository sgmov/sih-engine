# 泊界首建与路择时间维度谓词扩展

> T6D-XX task-packages 治理任务
> 承接：sih-tools/proposition/DES/m-parking-mechanics/disposition.md 裁示 + 用户令 T6-D 跑泊界首建和路择时间维度谓词扩展
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— 本任务包无偏离，标准实施类
> 日期：2026-08-25

## 一、问题陈述 {#problem}

- 裁示已立：向泊两界机械执行分载三件套经 facet 测量与人复核定案，即泊界账本归书简款写入载体、到期判定与在泊告警归路择扩展谓词族、心跳报数与结算单必经栏归视图族即远端零实装、向界不设执行件；两依据结合即基线一与基线四并合验收不可只取其一
- 待办两件：工具线泊界从待建转在役即规则文档加书简停泊写入位加首批住户入泊；路择谓词族现八件无时间维度即扩两件并以首批住户为基线实跑
- 边界：出泊唯人节点本批只入不出；视图件远端；引擎侧 SPEC-004 不动即停泊事件随融回收口合流同 certification_completed 先例

## 二、关键设计 {#design}

### 2.1 停泊写入位即 scribe park

第五子命令 park，双动作即 enter 与 exit。事件类型 parking_entered 与 parking_exited，event_class 沿 record_only，actor 沿书简自署 system。enter 记录必载 entry_id、title、exit_condition、ttl_days；exit 记录必载 entry_id、disposition 即 promoted 或 discarded、ruling。写入前校验四项不松，另加配对不变量两道机械门：重入拒即同 entry_id 已在泊再 enter 拒；无主出拒即无在泊 enter 的 exit 拒。doc_id 取 entry_id 即同项进出泊跨事件串联。拒绝零留痕。在泊判定即链上 parking_entered 后无同 entry_id 的 parking_exited。

### 2.2 时间维度谓词族即 packs/parking

材料五字段外加 parking 键即 entered_at 与 ttl_days，缺省或非对象一律判败即 fail-closed 同轮记录款。两件新谓词：time_deadline 到期判定即参照时间越过 entered_at 加 ttl_days 判败走 route_on_fail，参照时间由 route 的 --reference-time 显式给参不读钟，缺参即判败保 fail-closed；parking_aging 在泊告警即不判路恒过，参照时间与 entered_at 差值达 aging_threshold_days 逐件产批级告警，告警位以包声明为门同登记冲突款。报告头在给参时增 reference_time 键，未给参不增即旧输出形态不变。

### 2.3 接线

PARKING-v1.md 即工具线泊界规则文档，四字组落位：停有痕即停泊事件、看有门即 scribe query 检索加文档在泊投影、忘有警即路择时间谓词、出有点即唯人节点出泊事件。COURSE-v1 泊界节改写即 v1.3。AGENTS 索引接线三处。首批住户五件即跨会话未决事项入泊，停泊记录与材料落 sih-tools/parking/ 即 records 与 materials 两目录。

## 三、工作清单 {#work}

### Cluster 1：双子代理并行

- [ ] X1：PARKING-v1.md 草案、COURSE-v1 泊界节改写与版本行、AGENTS.md 三处接线
- [ ] X2：首批住户五件 enter 记录与对应材料 JSON、两工具 CALL-LOG 行草案

### Cluster 2：主线亲写最重件

- [ ] scribe park 子命令加测试加契约修订四
- [ ] selector 时间谓词两件加 packs/parking 加测试加契约修订三升 0.3.0

### Cluster 3：主线串行验证

- [ ] scribe 与 selector 测试全绿加兄弟回归
- [ ] 首批住户五件真实入泊上链经 meter 包裹
- [ ] 基线实跑即五住户材料经 parking 包路由入档 CALL-LOG
- [ ] 任务包与结果文件过文档链即化格核阅检词书简四件加 doclint 全程经 meter 包裹
- [ ] 结果文件落盘加双仓 commit

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1 停泊写入位** | 代码修复 | enter 与 exit 各正例上链；重入拒与无主出拒各拒且零留痕；写入前校验四项不松；trail 外零写入；scribe 旧测零改全绿 |
| **F-2 时间谓词族** | 代码修复 | time_deadline 到期判败未到期过缺 parking 键判败缺参照时间判败；parking_aging 达阈值逐件告警未达零告警；--reference-time 给参时报告头带键未给参不带；旧包旧输出逐字节不变即旧测零改 |
| **F-3 双跑一致** | 代码修复 | 同输入同参照时间双跑 selector route 输出逐字节一致；scribe park 同记录同 --at 双跑事件哈希一致 |
| **F-4 首批住户入泊** | 数据治理 | 五件 parking_entered 在链且 verify 链验通过，每件 details 含 exit_condition 与 ttl_days |
| **F-5 基线实跑** | 元层工具 | 五住户材料经 packs/parking 路由，退出码与路由结果如实入 CALL-LOG |
| **F-6 文档链** | 数据治理 | 任务包与结果文件过化格与检词零违例，核阅 des-001 域不含 task-packages 即退出码二为域外标记非违规如实记，书简认证在链；round 7 事件在链即 fbd25519 |

## 五、必读文件 {#read}

- 裁示：sih-tools/proposition/DES/m-parking-mechanics/disposition.md
- 词条：sih-engine/doc/proposal/PRO-007-naming-clearance-draft.md 第 26 行泊界词条
- 契约：sih-tools/scribe/CONTRACT.md 与 sih-tools/selector/CONTRACT.md
- 向界：sih-tools/COURSE-v1.md 泊界节与远端段
- 形制参照：sih-tools/selector/packs/round/manifest.toml 与 routes.toml

## 六、约束 {#constraints}

1. **零 LLM 治理写入**：入泊、验货、上链全部确定性程序执行
2. **两依据结合**：每件实施即执行者确定性即基线一与写入可追溯可机械校验不可篡改即基线四都须满足，单验其一不达标
3. **不扩大 scope**：不出泊即不替用户裁任何住户、不建视图件、不动引擎文档、不重开命名
4. **F 锚定状态入结果文件**：逐条过败如实登记
5. **并行 agent 材料不碰**：sih-math 与探针件不吸收不提交不清理

## 七、验收标准 {#acceptance}

本任务包验收 = 4 项：

- [ ] F-1 至 F-6 全过
- [ ] PARKING-v1.md 与两契约修订与 COURSE-v1 v1.3 在档
- [ ] 双仓 commit 即 sih-tools 与 sih-engine
- [ ] 结果文件 parking-selector-time-t6d-results.md 落盘

## 八、风险点 {#risks}

- 共享 trail 并行写即并行 agent 亦写 trail，追加单调由 _next_timestamp 承载，批内严格单调保证消解同微秒竞态，如实登记
- TTL 语义即到期为复检提醒非自动出泊，出泊仍唯人节点
- parking 键与 round 键并存时互不干扰即两族谓词各查各键

## 九、范式偏离声明 {#deviation}

一处随批修订即 F-6 判据措辞：跑前立文写作过 doclint 退出码零，实跑核对 des-001 域声明即只含 sih-engine/doc，任务包与结果文件为域外件，核阅退出码二为域外标记非违规，与数学审计批同形态先例，判据按化格零改动加检词零违例加书简认证在链执行。余无偏离。X1 文档线 X2 数据线后台并行，主线亲写两件代码即最重件，主线串行验收，结果文件收口。

## 十、关联文件 {#related}

- 命题源：sih-tools/proposition/topics/2026-08-25-parking-mechanics.md
- 意图记录：/tmp/parking-batch/record.json 即 intent_refined 事件 fbd25519
- 工具：sih-tools/scribe/ 与 sih-tools/selector/
- 载体：sih-tools/scribe/trail/2026-08-25.ndjson
