# mathquote-solo：数学仓重构逐字引文补强波一（四子仓早期条目）

> task-packages 治理任务
> 承接：release-audit-2026-09-02.md 遗产披露第一条即 134 条哲学桥接无原文逐字引文，用户令数学仓重构继续委外
> 队形：单线形 solo——程序切片加逐条补写亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-quote.json 双档零命中如实记
> 冲突测试：pk-045 样本库参与者，结果档须载冲突样本节

## 一、问题陈述 {#problem}

134 条（多为 calculus 旧格式与早期子仓条目）哲学桥接以命题对照形态承载而无「原文「…」」逐字引文，合最低规约但弱于复归段波产条目。补强即逐条回哲学仓原文程序切片引文入桥接段，恢复可机械核验的原文锚。

## 二、关键设计 {#design}

1. 枚举：脚本扫五子仓全部已建条目即哲学桥接节缺原文「标记的条目清单。
2. 切片：本波做 topology 与 probability 与 order 与 algebra 四子仓全量缺引文条目，calculus 部分后续波另批，切片边界申报。
3. 补写：逐条按其命题对照的 PRO 编号回 sih-philosophy/emanation/proodos 原文定位引文，程序切片逐字节子串禁手打，落形即原文「引文」见 文件:行号，命题对照保留不替换。
4. 机械复验：补后跑复验脚本即全部新增引文对原文做子串断言全过，此脚本入 materials 可重跑。

## 三、工作清单 {#work}

- [ ] 枚举脚本与四子仓缺引文清单
- [ ] 逐条程序切片补写
- [ ] 复验脚本全绿读数
- [ ] 逐条三步管线读数
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 枚举零漏** | 工程 | 清单可 grep 复算，四子仓已建计数对表 |
| **F-2 引文逐字节** | 工程 | 复验脚本全绿即全部引文为原文子串，重跑一致 |
| **F-3 对照保留** | 治理 | 原命题对照零删改，引文为增不换 |
| **F-4 写入仅 allow** | 治理 | 写入仅请求写入节所列，calculus 零触碰 |

## 五、必读文件 {#read}

- 必读 1：sih-philosophy/emanation/proodos/ 原文即切片源
- 必读 2：sih-math/docs/release-audit-2026-09-02.md 遗产披露节
- 必读 3：sih-math/order/entries/ORD-015 即引文落形先例
- 必读 4：sih-tools/BATCH-FACE.md 调用面与坑位

## 六、约束 {#constraints}

1. 哲学仓只读，引文程序切片逐字节子串禁手打
2. 认证先落主树活链，链 settle 前一次性拷工地
3. 撞锁有限重试如实计数，不绕行
4. 词债不过夜 findings 亲读

## 七、请求写入 {#requested-writes}

- sih-math/topology/entries/
- sih-math/probability/entries/
- sih-math/order/entries/
- sih-math/algebra/entries/
- sih-engine/sih/state/plan/mathquote-solo.md
- sih-engine/sih/event/plan/mathquote-solo-results.md
- sih-engine/sih/event/plan/mathquote-solo-materials/
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
- [ ] 复验脚本与清单在档
- [ ] 冲突样本节在结果档
- [ ] 认证入链双仓结算收约对表读数在档
