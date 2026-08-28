# m3seatrial-solo：单席位产品首跑批

> task-packages 治理任务
> 承接：sess-zcode-260828-m3seatrial 三问意图即 2026-08-28 链事件、用户同日定席即 MiniMax-M3 关思考
> 队形：单线 solo——主线亲写零子代理
> 日期：2026-08-28

## 一、问题陈述 {#problem}

单席位五件面壳在 pk023impl-solo 批落成后，用户定首跑坐席即 MiniMax-M3 关思考。本批用产品壳本身跑七探针命题乘六十发共四百二十发，产门输出与双层材料包并复算。M3 本身是八厂参照种群第八厂，本跑兼作闭环验证即坐席率与该厂厂级率逐题对表披露。

## 二、关键设计 {#design}

坐席注册文件单条目落 facet/singleseat-registry.json 即 seatmm3 前缀与 MiniMax 官方端 MiniMax-M3 与思考关与自报四件，前缀与既有 mm3 厂目录零相撞。采样经 singleseat.py run-probe 子进程复用 k2t_driver，全部落副本工地即七命题目录加 summary 目录随批入仓。跑后 gate、assemble、verify 三步走壳，材料带单源警示标零裁决字段。闭环对表非合格判据即六十发采样噪声量级判读、差值如实披露、异常旗照常机械亮。

## 三、工作清单 {#work}

- [ ] 坐席注册文件落盘并过 check-config
- [ ] run-probe 四百二十发满额、门、组装、复算
- [ ] 坐席率与 mm3 厂级率七题对表披露
- [ ] 管线三件、证入链、任务包与结果档落位
- [ ] 双仓段结算收约、免参对表、心跳复验

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 满额入流 | 工程治理 | 七命题各六十行 flywheel_run 即四百二十发、summary n_props 七、driver 退出码零 |
| **F-2** 门与材料 | 工程治理 | gate 与 material 落盘、verify 退出码零、材料含警示标全文与逐件哈希 |
| **F-3** 材料边界 | 工程治理 | 禁词键扫描零命中、异常旗如实记录不处置 |
| **F-4** 闭环对表 | 工程治理 | 坐席七率与 mm3 厂级率逐题差值披露、方向一致性判读如实 |
| **F-5** 治理收口 | 治理流程 | 管线三件过、证入链、双仓结算收约、对表与心跳绿态 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/facet/docs/SINGLE-SEAT-SPEC.md 即探针集与材料格式
- 必读 2：sih-tools/facet/singleseat.py 即五件面调用面

## 六、约束 {#constraints}

1. 采样只走坐席通道即 MiniMax 官方端，除坐席外零 LLM 调用
2. 材料不裁决即产出止于分布与定位与不确定度
3. 上链前必须等绿
4. 复算不过即整包作废重跑

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 结果档落 sih/event/plan/m3seatrial-solo-results.md，证入链，双仓段结算收约

## 八、风险点 {#risks}

端点限速与瞬时失败由 driver 重试承载，长跑中断有 early-flush 部分报告与续跑幂等兜底。闭环差值超采样噪声即口径漂移嫌疑，处置是核对思考口径与端点而非改数。

## 九、队形声明 {#formation}

单线 solo 即主线亲写零子代理，治理批默认形，选形即声明。

## 十、关联文件 {#related}

- 壳源：sih-tools/facet/singleseat.py 即 pk023impl-solo 批产出
- 链件：sih/event/trail/2026-08-28.ndjson 即意图与证
- 数据位：proposition/DES/seatmm3 七命题与 lw-seatmm3-probe-60-20260828 汇总

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/m3seatrial-solo.md
- sih-engine/sih/event/plan/m3seatrial-solo-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/facet/singleseat-registry.json
- sih-tools/facet/probes/results/singleseat/
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
