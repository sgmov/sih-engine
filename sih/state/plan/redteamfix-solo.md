# redteamfix-solo：概率卷 redteam 调用位修正批

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图（round 7），用户同日质询 redteam 退役属实
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

用户质询 redteam 退役。查证属实：工具代码全盘不在（sih-tools、工作区根、旧仓皆无），退役从未登记（死档册、泊件、决策档无记录），余留双 skill 壳与 AGENTS.md 旧章幽灵服役。批二新条目 PROB-006 与 PROB-007 及既有 PROB-003 携 redteam 调用位，摸底批更将其核为最实调用位，属核验失察即只信章程未查磁盘。

## 二、关键设计 {#design}

三条删节即修：PROB-006 与 PROB-007 整节删除 redteam 应用节并保 born green，PROB-003 删 redteam 小节；结果档记失察与幽灵退役状态；AGENTS.md 旧章与双 skill 壳的清理涉及章程与引擎 skill 面，留用户裁。调用位核验自此以磁盘实存为唯一判据。

## 三、工作清单 {#work}

- [ ] 三条删节
- [ ] 核阅保 PROB-006 与 007 零违规、三条无新增类
- [ ] 化格检词复跑
- [ ] 认证上链双仓收约
- [ ] 幽灵退役清单位（AGENTS.md 章、双 skill 壳）列入待用户裁

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 死调用位清零 | 工程 | 三条全仓 grep redteam 零残留 |
| **F-2** born green 保持 | 工程治理 | PROB-006 与 007 核阅仍零违规 |
| **F-3** 认账完整 | 治理 | 失察与幽灵退役状态入结果档，清理项列用户裁清单 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/state/skills/sihankor-redteam/SKILL.md 即幽灵壳现状

## 六、约束 {#constraints}

1. 不动章程与 skill 壳
2. 上链遇锁即等待不绕行
3. 链快照复制在最后一笔追加之后

## 七、请求写入 {#requested-writes}

- sih-math/probability/
- sih-engine/sih/state/plan/redteamfix-solo.md
- sih-engine/sih/state/plan/redteamfix-solo-results.md
- sih-engine/sih/event/plan/redteamfix-solo-results.md
- sih-engine/sih/event/plan/redteamfix-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/meter/counts/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，双仓段结算收约
