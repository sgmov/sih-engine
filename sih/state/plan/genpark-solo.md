# genpark-solo：生成面三件真泊引擎线泊界

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-06 令「开工」即本会话讨论裁定：生成面三件（SDD 增量语法借鉴、TDD 生成双种拆分、治理文件生成走文规）照 archpark-solo 先例真泊引擎线泊界；讨论源即本会话三轮生成面讨论（含 Fission-AI/OpenSpec 项目考察）与域外草稿三件
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 生成面三件待裁事项停在会话对话，无链上登记，违背泊界四字组之停有痕
- 其一：SDD 文档生成想借鉴 OpenSpec 项目，借鉴范围（差分语法、归档合并纪律）与承接批次未裁
- 其二：TDD 的生成没有想好，且金向量定义两义混用（规约测试与基线金向量）待拆分裁决，与 pk-046 方法论出泊裁相交
- 其三：全引擎治理文件生成应否走文规即文档是投影的通则化范围未裁，与 pk-066 契约漂移回写相交

## 二、关键设计 {#design}

### 2.1 号位与界别

pk-067 至 pk-069 三件全停引擎线（全态泊界）：SDD 增量语法借鉴取舍、TDD 生成双种拆分、治理文件生成走文规通则化。单一名册续号不重号，最高在册号 pk-066。

### 2.2 投影缓议

引擎线 PARKING-v1.md 名册投影更新留待泊界复检批收编（gvec-method-solo 与 archpark-solo 先例），真相在链不在文档。

### 2.3 正形承先例

泊材料按 archpark-solo 正形落位即携路由三字段 id 与 path 与 state（F-3 首跑废轨教训），对形基准 pk-063.json。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 三件泊材料 JSON 落工地，scribe park enter 三笔上链
- [ ] 关键报告件认证入链（ask3 记录、验证件、正身件）

### Cluster 2：主线串行验证

- [ ] 管线三步跑任务包与结果档（state/plan 与 event/plan 属 des-001 域外，如实记档）
- [ ] 泊界心跳复算即路择 parking 包对引擎线材料路由，新件须入 mainline 计数
- [ ] 双仓 settle、放锁收约、reconcile 与链 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 三件全上链 | 数据治理 | 当日链 parking_entered 三笔，entry_id 即 pk-067 至 pk-069 逐件在链，重入拒零触发 |
| **F-2** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-3** 心跳可复算 | 跨族治理 | 路择 parking 包对引擎线材料路由退出码 0，三件新件入 mainline 计数 |
| **F-4** 真值不越权 | 治理 | 三件泊材料只列讨论事实与出泊条件，不预设出泊裁决，不触碰在泊件 |

## 五、必读文件 {#read}

- 讨论源：本会话生成面三轮讨论（域外即对话记录，不入档）
- 立法源：`sih-engine/doc/governance/PARKING-v1.md`（四字组与出入口协议）
- 先例：`sih-engine/sih/event/plan/archpark-solo-results.md`（泊界进泊批先例与正形基准）

## 六、约束 {#constraints}

1. 零 LLM 调用、零代码改动、零在役判据与退出码语义触碰
2. 出泊唯人节点，本批只进泊不预设出泊形态；在泊件 pk-016、pk-039、pk-044、pk-046、pk-053、pk-055、pk-056、pk-058、pk-060 至 pk-066 零触碰不并批
3. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
4. 守卫在位禁 plain git commit，close 通道提交
5. 在盘遗留无主件（watchcheck 清单）不豁免不代清，本批不新增无主写

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-4 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 genpark-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 泊材料字段形态若与 parking 谓词包 P101 形态要求不合即路由落 scrap_track——以 pk-063.json 在册正形为准逐字段对形
- close 归并遇主树未跟踪件（任务包）即按备份让位归并对表法四步处理，真分叉即停批上报

## 十、关联文件 {#related}

- 任务包源：本会话生成面讨论与域外草稿三件（components-expansion、roadmap-components、governance-process，域外）
- 工具：`sih-engine/target/debug/scribe`（park 与 append 与 intent 写位）
- 跨仓引用：`sih-engine/sih/state/parking/materials/`

## 十一、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/genpark-solo.md`
- `sih-engine/sih/state/parking/materials/`
- `sih-engine/sih/event/trail/2026-09-06.ndjson`
- `sih-engine/sih/event/plan/genpark-solo-results.md`
- `sih-engine/sih/event/plan/genpark-solo-materials/`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 genpark-solo 工地
