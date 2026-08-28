# pk021settle-solo：pk-021 出泊结算批

> task-packages 治理任务
> 承接：sess-zcode-260828-pk021settle 三问意图即 2026-08-28 链事件 6107375076024ede、用户裁定即 2026-08-28 批准裁定包并附两条注记即 k2t 版本落后重测留点与 mm27 代际能力注记
> 队形：代理编组即单线形 solo——主线亲写零子代理，治理批默认形
> 日期：2026-08-28

## 一、问题陈述 {#problem}

pk-021 八厂双口径重算已出数即稳定集重算报告落 probes/results/eight-factory-20260828，用户已批准裁定包并附两条注记，岛存废须落地为出泊转正与白皮书数字层填表，g53 与 zfo 两厂收工数字与承继比对与数据归档与向界结算同批收口，下一段指针即 pk-023 单厂产品化。

## 二、关键设计 {#design}

双口径并行登记即 v3 口径岛三元组 c1-09 与 c2-02 与 p-12 转正加 strict 口径岛 p-12 单点转正，Bonferroni 族规模 m′ 取预注册确认性假设族两件即 v3 岛复现与 strict 六厂交集检验，strict 校正后 0.0071×2=0.0142 存活，重算表其余六十六个子集判探索性扫描不作确认性检验。白皮书填表遵出数不预写与改数不改文即只动第 7 章数字与槽位状态，唯一越线申报项为 6.5 节 F-M5 状态行待证改已证。c1-05 违例数行按八厂统一口径重计即 flywheel_run 判定流逐发机械提取，旧五厂值系当期提取口径照录留痕不混用。

## 三、工作清单 {#work}

- [ ] 停泊记账两笔即 pk-021 出泊 promoted 落 25 日链加 pk-026 k2t 重测留点入泊落 28 日链
- [ ] PARKING-v1.md 名录更新即在泊两项改 pk-023 与 pk-026 加历史住户增 pk-021
- [ ] 白皮书第 7 章填表即 g53 行与 zfo 行新增加第 1 至 3 与第 5 行刷新加槽位状态已填加代际注记
- [ ] 6.5 节 F-M5 状态行越线申报改已证并在结果档声明
- [ ] X24 承继比对报告落 facet/docs 即 glm-5.3 默认思考对 5.1 跨三端点同口径一致性
- [ ] 数据归档即 gitignore 增补可再生跑目录加工件入批即重算结果与两厂 summary 与 reg-llm 与 driver 收编
- [ ] facet COURSE 结算换版 v2 即 v1 删加 v2 立含 pk-023 指针与在泊现势
- [ ] 管线三件如实携退出码加三证入链加双仓段结算收约加免参对表零
- [ ] 结果档落 sih/event/plan/pk021settle-solo-results.md

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 出入泊记账 | 链上治理 | pk-021 parking_exited 在 25 日链载 promoted 与裁决文，pk-026 parking_entered 在 28 日链，双链 verify valid，PARKING-v1 在泊名录改两户 |
| **F-2** 填表合规 | 工程治理 | 白皮书第 1 至 6 章除 6.5 状态行外零字节变化，第 7 章槽位全填且数字可溯源到重算报告与判定流重计，两条用户注记在册 |
| **F-3** 承继比对 | 数学证据 | X24 报告数字全部可由 flywheel_trail 机械复算即三端点 Pearson 与均值绝对差与 c1-05 四方表与岛成员方向连续 |
| **F-4** 归档干净 | 工程治理 | git status 零未预期未跟踪即可再生跑目录被 ignore，g53 与 zfo summary 与重算结果与 reg-llm 与 driver 驱动补丁入批 |
| **F-5** 向界换版 | 轨迹治理 | COURSE-v1 删与 v2 立，v2 含 pk-023 指针与结算记录，心跳重跑绿态 |
| **F-6** 收约对表 | 链上治理 | 双仓段结算经 commit 正身路径，close 三检过，双仓 reconcile 免参退出码零，当日链 verify valid |

## 五、必读文件 {#read}

- 必读 1：facet/probes/results/eight-factory-20260828/stable-sets-recompute.md 即裁定包数字源
- 必读 2：facet/docs/FACET-WHITEPAPER.md 第 7 章 7.1 与 7.2 即填表槽位与填表纪律

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 上链前必须等绿
3. 他人批工件不收编
4. k2t 重测只留点不执行
5. ANOVA 不重跑只照录标注

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 结果档落 sih/event/plan/pk021settle-solo-results.md，三证入链，双仓段结算收约

## 八、风险点 {#risks}

主线脏件两笔即 reg-llm 与 driver 系前批产线残留即批前采 diff 副本后还原主线防乐观基线违例；未跟踪工件与归并腿撞名即副本先携入主线后除防 close 阻塞复发；停泊出泊事件写进泊所在文件即 25 日链过无主出拒门。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理，治理批默认形。选形即声明即非偏离。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-28 裁定即批准裁定包附两条注记
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 6107375076024ede
- 数字源：facet/probes/results/eight-factory-20260828/ 即八厂双口径重算
- 后续：pk-023 单厂产品化规格批即下一段指针入 COURSE-v2

## 十一、请求写入 {#requested-writes}

- sih-tools/PARKING-v1.md
- sih-tools/.gitignore
- sih-tools/facet/docs/FACET-WHITEPAPER.md
- sih-tools/facet/docs/X24-GLM-SUCCESSION-REPORT.md
- sih-tools/facet/COURSE-v1.md
- sih-tools/facet/COURSE-v2.md
- sih-tools/facet/reg-llm.yaml
- sih-tools/facet/probes/k2t_driver.py
- sih-tools/facet/probes/results/eight-factory-20260828/
- sih-tools/proposition/DES/lw-zhipu-glm5-3-3000-20260826/
- sih-tools/proposition/DES/lw-zhipu-flash-off-3000-20260827/
- sih-tools/scribe/reports/
- sih-engine/sih/state/plan/pk021settle-solo.md
- sih-engine/sih/event/plan/pk021settle-solo-results.md
- sih-engine/sih/event/trail/2026-08-25.ndjson
- sih-engine/sih/event/trail/2026-08-28.ndjson
