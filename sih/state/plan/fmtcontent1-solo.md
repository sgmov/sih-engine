# fmtcontent1-solo：格式内容卷第一批（LaTeX 修复加 counter 死调用位清偿）

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图（round 10）即 2026-08-30 链事件 f92bd441，用户同日令开格式内容卷并令公式类用 latex-helper
> 队形：单线形 solo——确定性工具与冻结映射表亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

latex-helper validate 全量 128 件发现 2 件 LaTeX 括号错误（HIS-002 混用括号、INT-017 导航行错配）；calculus 条目存 46 实例引用已物理删除的 counter 工具，死调用位违反磁盘实存基准。

## 二、关键设计 {#design}

两件 autofix 按工具修法写回；counter 46 实例按冻结映射表泛化清偿：后继 facet 在场处直挂 facet，其余泛化为机制名多轮对抗审阅，映射表外形态即报错。C006 全角括号本批不动，公式引用类的规则校准（latex validate 已证 $\ref$ 层健康）留用户裁。

## 三、工作清单 {#work}

- [ ] latex-helper autofix 两件加复跑清零
- [ ] counter 冻结映射表清偿加复跑零残留
- [ ] 核阅 C006 计数不增加化格检词复跑
- [ ] 认证上链三仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** LaTeX 清零 | 工程 | latex-helper validate 复跑 128 件零错误 |
| **F-2** counter 清零 | 工程治理 | grep counter 全仓零残留，替换语义逐类对表冻结映射表 |
| **F-3** 无新增伤 | 治理 | 核阅 C006 计数不增且无新类，化格检词零新增 |

## 五、基准 {#baseline}

调用位以磁盘实存为准承用户 2026-08-30 裁定；公式类工具辅助承用户同日 latex-helper 令。

## 六、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-engine/sih/state/plan/fmtcontent1-solo.md
- sih-engine/sih/state/plan/fmtcontent1-solo-results.md
- sih-engine/sih/event/plan/fmtcontent1-solo-results.md
- sih-engine/sih/event/plan/fmtcontent1-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/latex-helper/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，三仓段结算收约
