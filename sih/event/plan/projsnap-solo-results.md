# projsnap-solo 结果档

> 承接：任务包 sih/state/plan/projsnap-solo.md 与用户 2026-09-13 令「做快照层设计」续程；DES-018 测试计划 T1 至 T4 转绿为验收。
> 日期：2026-09-13　会话：17d4b5b08fe44577　范式：单线 solo 委外零子代理

## 执行实录 {#execution}

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 实装 | sih-tools/projsnap 三子命令 build 与 check 与 diff | 0 | pytest 4 passed |
| T1 同参双跑 | pytest t1 | 0 | 双工作区 build 出参 sections 与 sources_hash 恒等，check 出参恒等 |
| T2 手写节守卫 | pytest t2 | 0 | 手写节哈希批前批后一致，标记区外逐行原位零触碰 |
| T3 stale 三形 | pytest t3 | 0 | 决策新增与泊件新增与版本变更逐形 stale 命中，再生后回鲜 |
| T4 再生幂等 | pytest t4 | 0 | 重跑落零 changed false 承 ORD-023 |
| 意图笔 | scribe intent | 0 | d0ad66b2 |

## 交付摘要 {#deliverable}

sih-tools/projsnap 0.1.0：三节再生器（file-index 决策规格索引加 tool-versions 组件版本面加 parked-count 泊界计数投影），逐行原位替换实现手写节字节级零触碰，stale 判定按节哈希对表，失效裁决条款即投影与源不符以源为准构成性在场。

## F 锚定对表 {#f-table}

| 锚定 | 判据 | 结果 |
|---|---|---|
| F-1 T1 至 T4 全绿 | pytest 四测 | 过 |
| F-2 确定性 | 同参双跑逐字节 | 过：T1 承载 |
| F-3 手写节零触碰 | 哈希不变断言 | 过：T2 承载 |

## 偏差申报 {#declarations}

- 偏差一：DES-018 四类可派生节实装三节，project-stage 派生规则候后继批，承 DES-018 投影对象节登记制即登记几节再生几节。
- 偏差二：链 trail 与温故与回锚投影源未接入本批，投影源五路中三路落位，增量纪律承 DES-018 边界节。
- 偏差三：验收 A3 stale 三形以 fixture 三源变承载，真实工作区接入归切换批候令，pk-055 出泊并入切换批。
- 偏差四：首提允许面前缀与包布局错位经改道归位后过闸，工序偏差已自纠，批名对表承 DEC-017 甲表认领纪律。
- 零偏差显式申报位：除上列四条外本批零其他偏差，本节词形承载承 DEC-024 SDDG-3 纪律与 DES-018 边界节。

## 后续待令 {#next}

- 切换批（真实 AGENTS.md 换旗）与剩余投影源接入与 project-stage 规则候令。

## 结算节 {#settle}

- tools wip 62e5cbc5 加 engine wip 96a4411，认证报告经 scribe append 落链锁面内落盘，先例同形。
