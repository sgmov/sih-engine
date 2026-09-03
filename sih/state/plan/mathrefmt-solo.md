# mathrefmt-solo：数学仓重构格式归一波（calculus 概览优先旧格式）

> task-packages 治理任务
> 承接：release-audit-2026-09-02.md 遗产披露第二条即 calculus 旧格式首二级节为概览的双格式形态，用户令数学仓重构继续委外
> 队形：单线形 solo——确定性脚本加逐条改写亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-refmt.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，结果档须载冲突样本节

## 一、问题陈述 {#problem}

calculus 旧格式条目首二级节为「概览」而非「定义」，与根索引桥接规约标准块序并存为申报过的双格式（核阅 S005 豁免）。数学仓重构方向即归一到标准块序即定义、公理条件、哲学桥接、在工程的应用、与其他概念的关系，消双格式债。

## 二、关键设计 {#design}

1. 枚举：脚本扫 calculus/llm-friendly-build/entries 即首二级节为概览的条目全量出清单，零人工列漏。
2. 切片：本波做 LIM 与 DIFF 与 INT 三前缀即清单内该三前缀全量，余前缀后续波另批，切片边界申报。
3. 改写：逐条把概览节内容归入定义节（概览首段即定义引入、余段按语义落定义或公理条件），信息零丢失即原概览每一段落去向申报；状态行、命题对照、关系节零改动。
4. 验证：改后条目过 des-001-mathe（S005 两态合法域内）加化格加检词；全仓 INDEX 计数与 VERSION 基线零触碰。

## 三、工作清单 {#work}

- [ ] 枚举脚本与全量清单落 materials
- [ ] LIM 与 DIFF 与 INT 三前缀逐条归一
- [ ] 段落去向申报表
- [ ] 逐条三步管线读数
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 枚举零漏** | 工程 | 清单可 grep 复算，前缀计数与 calculus INDEX 对表 |
| **F-2 块序归一** | 工程 | 本波条目首二级节全为定义，S005 域内零违规 |
| **F-3 信息零丢失** | 治理 | 段落去向逐段申报，原概览无弃段 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列，INDEX 与 VERSION 零触碰 |

## 五、必读文件 {#read}

- 必读 1：sih-math/INDEX.md 桥接规约节即标准块序
- 必读 2：sih-math/docs/release-audit-2026-09-02.md 遗产披露节
- 必读 3：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 认证先落主树活链，链 settle 前一次性拷工地（防分叉纪律）
2. 撞锁有限重试如实计数，不绕行
3. 逐字引文不属于本波即缺引文条目归 mathquote 波，不混做
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/（本波三前缀条目）
- sih-engine/sih/state/plan/mathrefmt-solo.md
- sih-engine/sih/event/plan/mathrefmt-solo-results.md
- sih-engine/sih/event/plan/mathrefmt-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 枚举清单与段落去向申报表在档
- [ ] 冲突样本节在结果档
- [ ] 认证入链双仓结算收约对表读数在档
