# pk023impl-solo：单席位封装五件面实现批

> task-packages 治理任务
> 承接：sess-zcode-260828-pk023impl 三问意图即 2026-08-28 链事件、用户同日批准 SINGLE-SEAT-SPEC v0 四件规格即 pk-023 出泊转正
> 队形：单线 solo——主线亲写零子代理
> 日期：2026-08-28

## 一、问题陈述 {#problem}

pk-023 单厂单模型裁决材料产出经用户拍板出泊，本批两件事。出泊记账即停泊出泊事件落进泊所在的 2026-08-27 链、泊界名录在泊两项改一项、材料投影两除一立。产品实现即规格第 3 节五件面落地为确定性程序：坐席配置单条目、探针跑七命题乘六十发复用既有飞轮驱动、全量跑可选、门双口径照算且零族体温、组装器产双层材料包逐件带哈希。

## 二、关键设计 {#design}

五件面单文件落 facet/singleseat.py 即 CLI 编排零采样逻辑。坐席配置读单条目注册文件，验模型在 reg-llm.yaml 在册与前缀无目录相撞。探针与全量跑经子进程复用 k2t_driver 即零复制零格式漂移，驱动新增 props-ids 精确过滤修源名过滤在重复源命题上多选的缺陷。门即 load_for_gate 加 assess_maturation_v3，v3 传空体温表走绝对阈值语义，strict 自证取熵与贝叶斯上界两腿，boundary 二项与 Bonferroni 两腿记排除并披露缘由。组装器读判定流与门输出与参照产物装双层材料包，探针集取自重计产物 probes 键并按指纹规则复现核验，零手点名。材料零裁决字段即禁词键机械扫描，附复算子命令即数字对不上重跑即整包作废。

## 三、工作清单 {#work}

- [ ] pk-023 出泊事件与泊界名录与材料投影三件
- [ ] singleseat.py 五件面与 k2t_driver props-ids 扩展
- [ ] 测试套件与参照产物重跑同哈希自证
- [ ] 规格状态行与 COURSE v2 段记录与分叉链保全件入仓
- [ ] 管线三件、三证入链、任务包与结果档落位
- [ ] 双仓段结算收约、免参对表、心跳复验

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 出泊合规 | 链上治理 | 出泊事件落 2026-08-27 链即进泊所在文件过无主出拒门，disposition promoted，名录在泊余 pk-026，材料投影 pk-021 与 pk-023 除与 pk-026 立，两链 verify valid |
| **F-2** 五件面落地 | 工程治理 | singleseat.py 子命令可跑即配置验与探针推导与门与组装与复算，探针集自重计产物键加载并按指纹规则复现，零手点名 |
| **F-3** 材料合规 | 工程治理 | 材料双层含单源警示标全文与逐件哈希，禁裁字段键扫描零命中，参照带版本号与输入哈希在包 |
| **F-4** 零族体温 | 工程治理 | 门 v3 传空体温表走绝对阈值，strict 自证只含熵与贝叶斯上界两腿，boundary 与 Bonferroni 记排除，材料零体温数值 |
| **F-5** 测试与参照 | 工程治理 | 测试套件全绿，reference_stats 重跑逐字节同哈希 82f72e6dba84 |
| **F-6** 治理收口 | 治理流程 | 管线三件过，三证入链，双仓段结算收约，对表与心跳绿态，分叉链保全件随批入仓 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/facet/docs/SINGLE-SEAT-SPEC.md 即四件规格与验收判据沿用
- 必读 2：sih-tools/facet/probes/stable_sets_recompute.py 与 sih-tools/facet/probes/maturation_gate.py 即双口径与体温管道

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层，采样归坐席通道
2. 不实跑坐席即本批只落壳与测试，实跑预算与时点归用户
3. 上链前必须等绿
4. 材料不裁决边界随包走

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 结果档落 sih/event/plan/pk023impl-solo-results.md，管线三件认证入链，双仓段结算收约

## 八、风险点 {#risks}

k2t_driver 模块级参数解析使子进程复用为唯一安全路，props-ids 过滤须与既有 props 参数零冲突。他批悬置会话与主线未提交链尾使收约合并窗口有共链变量，处置循并发共链惯例即字节保全与细节重放。

## 九、队形声明 {#formation}

单线 solo 即主线亲写零子代理，治理批默认形，选形即声明。

## 十、关联文件 {#related}

- 规格源：sih-tools/facet/docs/SINGLE-SEAT-SPEC.md 即用户 2026-08-28 拍板
- 链件：sih/event/trail/2026-08-28.ndjson 即意图与三证、2026-08-27.ndjson 即出泊
- 泊界：sih-tools/PARKING-v1.md 即名录改写

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/pk023impl-solo.md
- sih-engine/sih/event/plan/pk023impl-solo-results.md
- sih-engine/sih/event/trail/2026-08-27.ndjson
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-engine/sih/event/trail/2026-08-28-pk023spec-pre-replay-splinter.ndjson
- sih-tools/facet/singleseat.py
- sih-tools/facet/probes/k2t_driver.py
- sih-tools/facet/tests/
- sih-tools/facet/docs/SINGLE-SEAT-SPEC.md
- sih-tools/facet/COURSE-v2.md
- sih-tools/facet/probes/results/
- sih-tools/PARKING-v1.md
- sih-tools/parking/materials/
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
