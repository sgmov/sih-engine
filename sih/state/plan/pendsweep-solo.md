# pendsweep-solo：挂账清理批（确定性通道三件加投影同步）

> task-packages 治理任务
> 承接：用户 2026-09-03 令即「先把挂账全清了，得一裁一过一」；前置分道依据 sihankor-facet-measure 修订四与 m-sett001-crosscheck 先例——本批四件均为机械核对类与投影同步类，不送采样，裁在确定性判据
> 队形：单线形 solo
> 日期：2026-09-03

## 一、问题陈述 {#problem}

主线挂账四项：tally assemble 未透传 core_hash 使 R5 核配对对现役材料不生效；金向量在 pk-036 重录后经 settlement 与 idcore 两批再历漂移未对表；ga-1 校准窗口自 2026-09-02 结算开窗后无窗满条件与触发链，与 pk-042 差分进化候选的入场券（解析不可行期）悬空；两线泊界名册投影落后于链（pk-043 已出泊未投影、pk-041 出泊已在案记账未清、pk-042 触发链未载）。

## 二、关键设计 {#design}

- 件一 tally 透传：assemble 步骤把 identity 报告的 core_hash 载入材料 JSON，旧报告缺席不判败；R5 判定面（idcore-solo 已就绪）自此对现役材料走核配对，双带即配对、任一缺席回退现行为逐字节同判
- 件二 金向量随冻：12 件 golden 逐件以现行核阅实跑对表，漂移件逐件核漂移合法性——目标被已结算批合法改动即重录（只刷期望输出，断言逻辑零改，承 pk-036 先例），无合法出处即停批报告；重录后 cargo test 全绿
- 件三 校准窗钉死：gauge CONTRACT 修订一条——ga-1 校准窗满条件为结算日 2026-09-02 后累计七个含例行读数的席位日或用户令提前；触发链四步入档即窗满、解析校准尝试、解析不可行、pk-042 出泊复检走得一裁，过即实验批
- 件四 投影同步：两线名册 as-found 修正对链留证；pk-041 记账位载主会裁定即跨天 park 记账走合并视图通道（cat 双链并尾落事件对 prev_hash，承 pk031gap 与 pk013exit 先例），mathpipe-a1 批执行时按此记账

## 三、工作清单 {#work}

- [x] F-1 tally assemble 透传 core_hash，旧报告缺席不判败，tally 测试全绿含新增透传用例与 R5 回放同判不回归
- [x] F-2 金向量逐件对表，重录清单与 cmp 证据入结果档，cargo test 全绿
- [x] F-3 gauge CONTRACT 校准窗与触发链修订一条，编号递增
- [x] F-4 两线名册投影修正，与链对表留证，零泊界事件写入
- [x] F-5 治理收口全链

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 | 实装 | 现役材料带 core_hash 过 assemble；R5 对同日新基线核配对成立；旧材料回放同判 |
| F-2 | 核对 | 12 件对表读数全载；漂移件全数有合法出处或停批 |
| F-3 | 文档 | CONTRACT 修订一条在场，触发链四步逐字可读 |
| F-4 | 投影 | 名册行与链事件逐条对上；pk-043 出泊、pk-041 记账位、pk-042 触发链三行在场 |
| F-5 | 收口 | 链 verify 0；reconcile 双零；管线 findings 亲读留证 |

## 五、请求写入节 {#scope}

- sih-tools/tally/src/tally/cli.py
- sih-tools/tally/CONTRACT.md
- sih-tools/tally/tests/
- sih-tools/gauge/CONTRACT.md
- sih-tools/nomenclator/packs/core/
- sih-tools/PARKING-v1.md
- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/src/scrutinator/fixtures/golden/
- sih-engine/sih/state/plan/pendsweep-solo.md
- sih-engine/sih/event/plan/pendsweep-solo-materials/
- sih-engine/sih/event/trail/2026-09-03.ndjson

禁区：泊界事件零写入（本批零进泊零出泊，出泊唯人节点）；pk-041/043/044 泊件本体零碰；mathpipe-a1 在途批零碰；AGENTS.md 零触碰；identity 源码零碰；金向量断言逻辑零改。
