# adjudisp-solo：裁决分派令立名与得一狗粮回填批

> task-packages 治理任务
> 承接：用户 2026-09-02 两步立刻执行令、主会同日自省即得一 08-30 落地后链上零非自证机器终签的结构性循环
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-02

## 一、问题陈述 {#problem}

得一（判定器席实例，DEC-020）落地后未参与任何治理裁决：链上 crosscheck 终签仅两笔且均为其融回资格自证材料（m-mbgate-scrut 与 m-mbgate-deyi）。裁决权分派规则缺位即哪些裁决必过得一、哪些留人节点、哪些代裁无表可查，判定器能力在场而路由缺位。本批两步即立《裁决分派令》钉死分派表、回填一场真实狗粮产出链上首笔非自证 stable_clear。

## 二、关键设计 {#design}

四件。一即分派令立名：治理名「裁决分派」中文对、代号 adjudisp 小写、英文对 AdjudicationRouting 承英文对理据即平行表达非翻译；本质段即三类分派表——过得一类即验收打回裁定、融回门开关建议、封窗追认、命题类裁决；人节点类即出泊、开门、封窗令本身与一切用户字令；代裁类即纯机械对表与管线读数；三态各带理由。落 DEC-021 体例承 017/019/020 先例。二即狗粮命题：拒直提守卫开不开权即 lease 增 pre-commit 钩子拦无模板 plain commit，显式 --no-verify 留痕放行——命题本身是判定器缺席的产物即自举场。facet 单席位 9 发采样加 tally R1-R7 核对加机器终签，基线先行即温故导出既有 stable_clear 判定材料为对照。三即双投影与登记：分派令词条经检词 register 落册、naming 与 facet-measure skill 投影核验。四即收口全链：任务包、结果档、dispatch 材料、双仓 settle、reconcile 双零保持。

裁决归人类即用户令在案即本批两步令，机器签署路线适用界以本次狗粮实测扩充建议留档不擅自扩界承 deyireg 误差申报二先例。

## 三、工作清单 {#work}

- [x] 查档与推导与 DEC-021 落档走管线
- [x] 检词登记与投影核验
- [x] facet 契约模式测量加 tally 终签入 crosscheck 事件
- [x] 收口对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 分派令 | 工程治理 | DEC-021 落档、三态分派表全、管线 des-001 零违规、检词登记零拒、投影核验一致 |
| **F-2** 狗粮测量 | 工程治理 | facet 契约模式 9 发实跑、tally R1-R7 核对、gate_verdict 出 stable_clear 或挂起归人、全程无钓样本 |
| **F-3** 链上落据 | 链上治理 | crosscheck_completed 事件入当日链、disposition 与 gid 载狗粮命题、非自证即非 mbgate 系列、基线先行有对照材料 |
| **F-4** 收口 | 链上治理 | 双仓 settle、链 valid、reconcile 双零保持、本批产物全量入版控、inputlog 逐字 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/decision/020-deyi-component-naming.md 即得一先例体例
- 必读 2：sih-tools/facet/docs/CONTRACT-MODE-SPEC.md 即契约模式测量两段式
- 必读 3：sih-tools/tally/CONTRACT.md 即 R1-R7 与终签

## 六、约束 {#constraints}

1. 判定器融回门不开即 facet 与 tally 在围堰原位跑测量，引擎零改动（红线）
2. 狗粮命题即拒直提守卫的最终开权仍归用户，本批测量只出裁决材料不替用户裁（红线）
3. 分派令适用界从本批起算即既往裁决不回溯重裁（红线）
4. 上链前必须等绿、findings 亲读、禁管道掩退出码
5. 正规提交路径严禁直提、链尾入版控对表、撞锁显式 --session

## 七、验收标准 {#acceptance}

- [x] F-1 至 F-4 全过

## 八、风险点 {#risks}

facet 采样腿 LLM 输出不稳即挂起归人，防御即挂起亦为有效裁决材料如实入链不钓样本。分派表边界即代裁与过得一的灰区，防御即灰区默认过得一从严。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 两步立刻执行令
- 链件：随批意图入当日链
- 关联：DEC-020、DEC-013 融回门、goldfix 直提六笔、立名 skill 修订记录

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[裁决分派]: 消解 即大白话直述治理名候选，立名批内走推导
叩问处置[狗粮]: 消解 即大白话直述即自产自用实测，非登记术语
叩问处置[分派表]: 消解 即大白话直述即权责清单，非登记术语
叩问处置[自举场]: 消解 即大白话直述即用系统裁系统自身议题，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/decision/021-adjudication-dispatch.md
- sih-engine/sih/state/plan/adjudisp-solo.md
- sih-engine/sih/event/plan/adjudisp-solo-results.md
- sih-engine/sih/event/plan/adjudisp-solo-materials/
- sih-engine/sih/event/plan/sealwin3-solo-materials/dispatch-matcatch2.md
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/nomenclator/packs/core/terms.json
- sih-tools/nomenclator/packs/core/manifest.json
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/facet/facet_task_packages/
- sih-tools/facet/CALL-LOG.md
- sih-tools/tally/CALL-LOG.md
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/identity/reports/
