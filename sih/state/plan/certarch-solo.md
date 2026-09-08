# certarch-solo 任务包：cert_missing 家族账面考古批

> 令源：用户 2026-09-08 令即「出一个账面考古批把全家族一次清掉——每件先机械试补（盘上若还有载原始内容哈希的报告件即补笔，标注补录语境）；不可重构的即销账登记（bypass 台账形，事由如实）。要么数学要么登记，账面不留悬空引用。你后台拉子代理做」。
> 队形：单线 solo，主窗后台拉起子代理执行，主窗独立复算。批日 2026-09-08，会话号 sess-zcode-260908-main-certarch。

## 范围 {#scope}

1. 家族盘点：三仓各跑 lease reconcile 全量形态，枚举 cert_missing 逐件清单（已知成员含 chaingreen、projfix、hookfix、confpreempt 即 4ff60c8f 全链缺席已证实；以 reconcile 当跑实态为准不预设名单）。
2. 逐件机械试补：每件在盘上考古载有原始内容哈希或原始报告体的报告件即 CALL-LOG、批材料、settle 回执、results 档；可重构者走补笔通道即构造补录报告 JSON 载原始哈希与补录语境与现盘复核语句，经引擎 scribe append 落认证笔，标注补录即事件属补录非当日原笔。
3. 不可重构者销账：经 lease bypass 通道逐件登记即 --repo 与 --sha 与事由，事由照录考古结论与不可重构依据，不虚构补笔。
4. 收口读数：三仓 reconcile 复跑 cert_missing 归零或余件各附不可清事由呈人裁；结果档含全家族对账表即件名、提交号、处置态、证据指针。
5. 两态纪律：每件处置必居其一即补笔或销账，零悬空零例外不申报即跳。

## 产出 {#outputs}

任务包本件与提示词件（主窗预落）、补录报告 JSON 逐件、销账登记逐件、三仓 reconcile 前后读数、对账表、ask3 记录与验证件、正身件、租约与锁实录、意图链笔、认证清单、结果档 sih-engine/sih/event/plan/certarch-solo-results.md 含大白话节、机器可读投影 certarch-ledger.json 认证上链、双仓或三仓 settle 提交号、链 verify 与 reconcile 读数、越线与误差申报。

## 边界与红线 {#redlines}

- 零代码改动零规则改动零在泊件触碰；考古只读面即批材料与报告件与台账与链文件。
- 补笔通道只经引擎 scribe 写位，销账通道只经 lease bypass，禁手写台账。
- 主树零直写（两件预落除外），禁管道掩退出码即每命令即捕 RC 且失败即停，误差红证如实记档。
- 历史链文件零改写即补录笔落当日或最新 trail 不回写旧日文件，补录语境在事件属显式可见。
