# parkgate-solo：泊界搁置升级带锚落地

> 治理任务包（实装类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「本次会话的向界是向泊二界数学回锚波和泊界升级（带锚落地）」批二；令源二即同会话裁定「pk-070是mcp实装的时候才决定的事情，现在迁移手动阶段无法模拟了。另外泊界需要升级，这种远景pk或者有明显界限的不应该抢占注意力」；批一登记面 sih-math/docs/anchorwave-parking-course-2026-09-07.md 在案
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 远景件与有明显界限的件抢占注意力：pk-070 裁决结构性依赖 MCP 实装无日期，却对 ttl 时钟计老化将于约 2026-10-06 报假到期
- 泊界无搁置维度：四字组与心跳谓词只承时限不承触发，触发器点火件与不可裁件同挤 mainline
- 批一登记面已申报乙类零命中并预置第五常数补登记位，批二即带锚落地

## 二、关键设计 {#design}

### 2.1 三路枚举零改

搁置复用 siding 路即停放语义本位，不新增第四路，PRO-007 已签词条三路承接面零触碰。

### 2.2 首败定路天然免判

parking 包 P104 gate_hold 前置于 P102 time_deadline，未点火搁置件首败定路走 siding，P102 不再评估即到期免判零引擎侵入。

### 2.3 停计语义

parking_agings 对 gate 键在场且 fired_at 缺省的件零告警，siding 计数仍可见即停计不隐藏。

### 2.4 锚

gate 语义挂 ORD-022 邻接锚即回升必经独立重校验谓词 ρ 门控与自动回升属实现违例；点火判定归后继批或人节点，本批只落搁置位。

## 三、工作清单 {#work}

### Cluster 1：工地写

- [ ] selector 源改即 predicates.py 加 check_gate_hold 与 parking_agings 停计、pack.py 注册 kind
- [ ] parking 包 routes.toml 加 P104 前置于 P102、manifest 版本进位
- [ ] CONTRACT.md 修订六登记新谓词与停计语义
- [ ] pk-070.json 加 gate 键承裁定原话
- [ ] PARKING-v1.md 加搁置条款
- [ ] tests 新增 gate 用例与 fixtures

### Cluster 2：主线串行验证

- [ ] 测试套全绿
- [ ] 心跳复算即两线材料路由，pk-070 须入 siding 告警零
- [ ] 旧件零漂移即除 pk-070 外既有材料路由与告警输出与批前一致
- [ ] 管线三步、checkcite、认证、双仓 settle、放锁收约、reconcile 与链 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** pk-070 入 siding | 数据治理 | 心跳复算 pk-070 路由 siding 不在 mainline 计数，引擎线告警零 |
| **F-2** 停计生效 | 数据治理 | 构造超龄未点火搁置标本即参照时间越老化阈值，零 parking_aging 告警 |
| **F-3** 旧件零漂移 | 跨族治理 | 除 pk-070 外既有材料路由与告警输出与批前逐字段一致 |
| **F-4** 全测绿 | 工程 | selector 测试套全过含新增 gate 用例 |
| **F-5** 枚举零改与写入仅 allow | 治理 | 三路枚举与既有谓词语义零改即新谓词独立登记，写入仅请求写入节所列路径 |

## 五、必读文件 {#read}

- 批一登记面：`sih-math/docs/anchorwave-parking-course-2026-09-07.md`
- 锚条目：`sih-math/order/entries/ORD-022-*`
- 契约：`sih-tools/selector/CONTRACT.md`
- 立法源：`sih-engine/doc/governance/PARKING-v1.md`
- 裁定源：pk-070.json 与本会话用户裁定原话

## 六、约束 {#constraints}

1. 三路枚举与 PRO-007 词条承接面零触碰
2. 出泊唯人节点零触碰，pk-070 只加 gate 键不预设出泊
3. 触发器点火回归的自动检测不在本批，点火判定归后继
4. 其余在泊件零触碰
5. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
6. 守卫在位禁 plain git commit，收约补笔走 bypass 登记
7. 在盘遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 parkgate-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- parking_aging 停计若误伤无 gate 键材料即 F-3 红线拦截，测试覆盖正向反向
- PARKING-v1.md 属 des-001 域内核阅须 exit 0，条款措辞按在役文体
- 心跳对 pk-070 的路由变化须与 F-3 旧件一致性和对表声明，pk-070 是唯一预期变化件

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`
- 后继：点火判定批或人节点裁、乙类载体候选归数学仓
- 关联泊件：pk-063 即触发器已点火件的对照例，其回归属后继批

## 十一、请求写入 {#requested-writes}

- `sih-tools/selector/src/selector/predicates.py`
- `sih-tools/selector/src/selector/pack.py`
- `sih-tools/selector/packs/parking/routes.toml`
- `sih-tools/selector/packs/parking/manifest.toml`
- `sih-tools/selector/CONTRACT.md`
- `sih-tools/selector/tests/`
- `sih-engine/doc/governance/PARKING-v1.md`
- `sih-engine/sih/state/parking/materials/pk-070.json`
- `sih-engine/sih/state/plan/parkgate-solo.md`
- `sih-engine/sih/event/plan/parkgate-solo-results.md`
- `sih-engine/sih/event/plan/parkgate-solo-materials/`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- worktrees 双仓 parkgate-solo 工地
