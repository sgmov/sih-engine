# wikirecall-p12-solo 任务包

- 队形：单线，日期：2026-09-02
- 触发：m-wikirecall 全裁终签，用户令「开始」，P0 已落地（d847026）
- 温故检索：召回工具题零先例如实记；先导三源承方案件

## 目标

P1：sih-tools/wikirecall 三通道确定性召回（词面打触发集、骨架全读清单、关系图一跳），别名首册机械生成，selftest 全绿含逐字节重放断言。
P2：消费闭环审计器 checkcite（引用对书单并集加图闭包），BATCH-FACE 收口段加书单对表步骤。

## F 锚定清单

| F | 验证 | 通过条件 |
| --- | --- | --- |
| F1 | selftest 全绿含重放一致 | JSON 报告 failed 零 |
| F2 | 真仓冒烟 | 对 sih-math 真查询三通道并集非空且重放一致 |
| F3 | BATCH-FACE 步骤 | 化格检词双 0 |
| F4 | 认证与收约 | intent 加认证上链，双仓 settle 归并 close |
| F5 | reconcile 与链 verify | 零新增异常，链 valid |
