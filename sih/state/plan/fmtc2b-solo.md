# fmtc2b-solo：公式引用括号内容修复批

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图（round 12）即 2026-08-30 链事件 44c01fe7；前置即 rulecal 撤回改道
> 队形：单线形 solo——确定性脚本亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

72 处公式引用括号待清偿；规则校准经三次构造未达可证伪条款撤回，改走内容修复通道。

## 二、关键设计 {#design}

确定性脚本将公式引用括号形态（X见公式 Y）机械替换为 ，X见公式 Y，，双逗与句逗相连两形归一，语义零变化。验收即核阅复跑 C006 恰降 72 且其余类逐一不变，差一即回滚。规则包维持 0.2.0。

## 三、工作清单 {#work}

- [ ] 替换脚本执行恰 72 实例
- [ ] 核阅对表加化格检词
- [ ] 认证上链双仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 恰 72 | 工程 | 替换实例数 72 且核阅 C006 恰降 72 |
| **F-2** 零语义变化 | 治理 | 除括号改逗号外 diff 无其他变更，其余类计数逐一不变 |
| **F-3** 规则包不动 | 治理 | des-001-mathe 维持 0.2.0 零改动 |

## 五、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-math/topology/entries/
- sih-math/probability/entries/
- sih-math/order/entries/
- sih-math/algebra/entries/
- sih-engine/sih/state/plan/fmtc2b-solo.md
- sih-engine/sih/state/plan/fmtc2b-solo-results.md
- sih-engine/sih/event/plan/fmtc2b-solo-results.md
- sih-engine/sih/event/plan/fmtc2b-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/meter/counts/

## 六、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，双仓段结算收约
