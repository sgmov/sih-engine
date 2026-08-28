# pk023spec-solo：单厂单模型裁决材料规格批

> task-packages 治理任务
> 承接：sess-zcode-260828-pk023spec 三问意图即 2026-08-28 链事件待记、用户令即 2026-08-28 批准启动 pk-023
> 队形：代理编组即单线形 solo——主线亲写零子代理，治理批默认形
> 日期：2026-08-28

## 一、问题陈述 {#problem}

pk-023 facet 单厂单模型裁决材料产出经用户批准启动。普通开发者手上只有单厂单模型，facet 对该场景的产品是裁决材料产出即材料不裁决。七厂对照实证已立在案即框架层多角度发散在单模型内部塌缩为零异质性、实质参数层异质性剧烈但形状私有、共识线只在子集上存在。本批立四件规格即锚定探针集、参照种群统计、单席位封装、材料格式，并落参照种群的机械重计脚本与数据产物。规格成稿后提用户拍板，拍板前 pk-023 不出泊。

## 二、关键设计 {#design}

锚定探针集机械推导即共识锚为岛三元组 c1-09 与 c2-02 与 p-12（八厂违例率 1.00/1.00/0.00 零离散）加指纹锚为 n≥30 全覆盖 30 命题子集逐主题群体标准差最大者即 c1-06 与 c2-04 与 e1-01b 与 p-06，共七命题每命题 60 发与参照口径一致。参照种群统计由重计脚本从已入册判定流机械产出即逐命题八厂违例率带 min/med/max 与岛期望与双口径稳定集规模分布加可得性披露。偏置定位规则即指纹锚坐席值对参照带判合群或越界方向、共识锚越期望方向界即异常旗。材料格式双层即机器 json 加人读 md 恒挂单源警示标，三层自证即判定分布测量加严格口径自证不含族体温假象加锚定探针偏置定位。单席位封装只立件面即配置与驱动复用与门与组装器与确定性边界，实现归后续批。

## 三、工作清单 {#work}

- [ ] 重计脚本 reference_stats.py 落 facet/probes 即从判定流产出参照统计
- [ ] 参照统计产物落 facet/probes/results/reference-population-20260828/ 即 json 加 md
- [ ] 规格 SINGLE-SEAT-SPEC.md 落 facet/docs 即四件加验收判据加实现批边界
- [ ] 任务包与结果档落 sih 两 plan 位、三问与三证入当日副本链
- [ ] 双仓段结算收约、免参对表零、批后心跳绿态

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 探针集机械可导 | 数学证据 | 七命题可由重计脚本按声明规则从 committed 判定流复现即岛三元组加逐主题最大标准差者，零手点名 |
| **F-2** 参照统计可复算 | 数学证据 | reference-stats json 与 md 全部数字由脚本重跑逐字节可复现，产物带生成参数与输入哈希 |
| **F-3** 材料边界 | 工程治理 | 规格与材料骨架含单源警示标与三层自证且零裁决字段，grep 可验 |
| **F-4** 确定性边界 | 工程治理 | 封装件面零 LLM 调用新增即模型调用止于探针与全量跑两面，规格明示 |
| **F-5** 治理收口 | 链上治理 | 三证入链、双仓段结算收约、免参对表退出码零、心跳绿态 |

## 五、必读文件 {#read}

- 必读 1：facet/probes/results/eight-factory-20260828/stable-sets-recompute.md 即参照种群数字源
- 必读 2：sih-tools/PARKING-v1.md 第 21 行即 pk-023 在泊条目与出泊条件

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 上链前必须等绿
3. 八厂参照数据只读
4. pk-023 本批不出泊
5. 规格只立文不实现壳

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 结果档落 sih/event/plan/pk023spec-solo-results.md，规格提用户拍板

## 八、风险点 {#risks}

探针推导规则若依赖手选即 F-1 不可复现，以脚本内声明规则收口；参照带在低发数格子上的可信度以可得性披露兜底；单源材料被误读为裁决即以恒挂警示标与零裁决字段双防线压住。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理，治理批默认形。选形即声明即非偏离。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-28 批准启动
- 数字源：facet/probes/results/eight-factory-20260828/ 与八厂判定流
- 后续：用户拍板四件规格后 pk-023 出泊与实现批另立

## 十一、请求写入 {#requested-writes}

- sih-tools/facet/docs/SINGLE-SEAT-SPEC.md
- sih-tools/facet/probes/reference_stats.py
- sih-tools/facet/probes/results/reference-population-20260828/
- sih-tools/scribe/reports/
- sih-engine/sih/state/plan/pk023spec-solo.md
- sih-engine/sih/event/plan/pk023spec-solo-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
