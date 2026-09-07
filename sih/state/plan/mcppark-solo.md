# mcppark-solo：MCP 实装远景件真泊入界

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「那就进入 mcp 实装的泊界」；讨论源即本会话注入式回锚通用性结论；本件是泊界搁置机制落地的首件生而搁置件即携 gate 键入泊
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- MCP 实装作为注入通用化终局与自有运行时方向，现停于会话讨论无链上登记，违背停有痕
- 该方向属远景即结构性依赖主线推进，未点火前不应入注意力主线即应生而搁置
- 关联件 pk-070 的 gate 触发器即 MCP 实装结算件落 event/plan，本件与其构成先后链须登记关联

## 二、关键设计 {#design}

### 2.1 生而搁置

pk-077 携 gate 键入泊即 gate.trigger 为主线 v2 结算件落 sih-engine/sih/event/plan/，fired_at 缺省，P104 gate_hold 即首跳判向 siding，不占 mainline 计数不产老化告警。

### 2.2 号位与界别

单一名册续号即最高在册号 pk-076 之后取 pk-077，引擎线。关联 pk-070 只读引用不触碰。

### 2.3 投影缓议

PARKING-v1.md 名册投影更新留待泊界复检批收编，真相在链不在文档。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] pk-077.json 泊材料落位携 gate 键，scribe park enter 上链一笔
- [ ] 关键报告件认证入链

### Cluster 2：主线串行验证

- [ ] 心跳复算即 pk-077 须入 siding 告警零即生而搁置生效
- [ ] 管线三步、双仓 settle、放锁收约、reconcile 与链 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 入泊上链 | 数据治理 | 当日链 parking_entered 一笔 entry_id 即 pk-077，重入拒零触发 |
| **F-2** 生而搁置 | 跨族治理 | 心跳复算 pk-077 路由 siding failed P104，引擎线 mainline 计数不含 pk-077，告警零 |
| **F-3** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-4** 真值不越权 | 治理 | 材料只列事实与条件不预设立项裁决，在泊件零触碰 |

## 五、必读文件 {#read}

- 立法源：`sih-engine/doc/governance/PARKING-v1.md` 搁置条款
- 关联件：`sih-engine/sih/state/parking/materials/pk-070.json`（只读）
- 先例：`sih-engine/sih/event/plan/archpark-solo-results.md` 与 `genpark-solo-results.md`

## 六、约束 {#constraints}

1. 零 LLM 调用、零代码改动、零在役判据触碰
2. 出泊唯人节点，本批只进泊携界限不预设出泊裁决
3. 在泊件全量零触碰即 pk-070 只读关联
4. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
5. 守卫在位禁 plain git commit，收约补笔走 bypass 登记
6. 在盘遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-4 全过
- [ ] 双仓 settle 提交号在档，收约后本批零活跃锁
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 mcppark-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- park 写位参数形以 scribe park --help 实测为准，缺锁门参数即补 --locks 与 --sessions 形
- 材料字段形态以 pk-076 在册形对形，gate 键以 parkgate-solo 后正形为准

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（park 与 intent 与 append 写位）
- 跨仓引用：无工具侧写面即 tools 仓只落报告件

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/mcppark-solo.md`
- `sih-engine/sih/state/parking/materials/pk-077.json`
- `sih-engine/sih/event/plan/mcppark-solo-results.md`
- `sih-engine/sih/event/plan/mcppark-solo-materials/`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- worktrees 双仓 mcppark-solo 工地
