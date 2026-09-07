# projfix-solo：投影腿 markdown 原格式修复

> 令源：用户 2026-09-07 令「开」；判定语义 m-calllog-proj-1 九发 stable_clear 机器终签 2564e16b 在链（markdown 表格原格式形认可，jsonl 行堆形为不取形）；验收背景即 calllog-solo 六项缺陷实录（CONTRACT 修订四十八追记）
> 范式：T6 单线 solo，委外代理亲写零子代理
> 前置：在飞批撞锁即 wait-turn 排队候叫（confpreempt-solo sih-tools 工地在飞）；与 closegate-solo 串行即本批先行，CONTRACT 修订互斥位后批候叫

## 一、问题陈述 {#problem}

- **问题 1**：投影腿现行实现为 jsonl 行堆（calllog/core.py _render_projection_for_tool），与权威腿逐字节同构即人读面缺位，已裁为不取形。
- **问题 2**：import 投影再生步把 19 册源册覆写为 jsonl 行堆（迁移自毁源档），禁跑令在册（BATCH-FACE 坑位勘误 2026-09-07）。
- **问题 3**：dogfooding 钩子无跨根卫——工地 shell 开 SIHANKOR_CALLLOG_DOGFOOD=1 时 resolve_root 跳回真根直写主树（137 行历史被抹实证）。
- **问题 4**：dogfooding 冻结中，追加命令族半瘫痪。

## 二、关键设计 {#design}

### 2.1 markdown 表格渲染器（T-1，推荐 verbatim 重组形）

- 渲染形推荐：**verbatim 重组**——投影再生即该 tool 的账本行按序取 `verbatim` 字段重组为册子文本。import 落账的行 verbatim 即原册原始行（含表头、分隔行、空行），重组即逐字节还原原册；append 新行落账时 verbatim 字段写该行的表格渲染形（日期 | 事由 | 调用 | 退出码 | 会话 | 备注，按册 schema 批内定形），重组即含新表格行。
- 此形零 schema 解析、渲染器极简、双跑一致易证；批内可另择形但须过同样 F 锚定。
- 册尾差异如实申报（重组形与 HEAD 的逐字节差异仅允许已知尾行类）。

### 2.2 import 去自毁（T-2）

- run_import 投影再生步改为 2.1 渲染器（或迁移后册子与源逐字节一致的同效形）；迁移后 19 册与迁移源 cmp 一致即零自毁可证。
- one-shot 守卫保持（ndjson 在即 skip）。

### 2.3 跨根 dogfood 卫（T-3）

- _dogfood_calllog 增卫：resolve_root 结果与当前进程 CWD 所在工作区根不一致（工地形）即 refuse 留痕（stderr 一行「跨根卫拒：工地 dogfood 禁直写真根」，零写入），显式 SIHANKOR_CALLLOG_DOGFOOD_ROOT 指定根时放行（自担申报）。
- 红绿夹具：伪工地形（root 指他处）拒 + 主树形过。

### 2.4 dogfooding 解冻（T-4）

- 上述三件落地后：BATCH-FACE 坑位勘误 2026-09-07 节更新即 import/render 解禁、dogfooding 缺省仍关但主树可用；本批批内 lease 治理调用即用 append 命令留痕（T-8 先例，注意 2.3 卫下工地期不开环境变量、留痕归收约后主会补或批尾主树补，批内如实申报）。

## 三、工作清单 {#work}

- [ ] T-1 markdown 表格渲染器（verbatim 重组形）替 jsonl 行堆渲染
- [ ] T-2 import 投影步去自毁（迁移后与源逐字节一致）
- [ ] T-3 跨根 dogfood 卫（工地拒、主树过、显式根放行）
- [ ] T-4 BATCH-FACE 勘误更新与 dogfooding 解冻申报
- [ ] T-5 主树真跑一次 render 对 19 册逐字节对表 HEAD（验收硬性项：工地条件验证不算接线第四案教训）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 渲染忠实 | render(账本) 对 19 册输出与 git HEAD 逐字节一致（已知尾行差异类外零差） |
| **F-2** | 迁移零自毁 | import 全流程后册子与源逐字节一致（临时根夹具 + one-shot 守卫保持） |
| **F-3** | 跨根卫 | 伪工地形拒零写入、主树形正常、显式根放行三夹具红转绿 |
| **F-4** | 追加渲染 | append 后投影含新表格行且历史行零变动 |
| **F-5** | 双跑一致 | 全子命令同参双跑逐字节一致 |
| **F-6** | 零 LLM | 命令族全程零模型调用零网络 |
| **F-7** | 语义承证 | 按 m-calllog-proj-1 落地零偏离，遇边界停批呈报 |

## 五、必读文件 {#read}

- 已裁语义：sih-tools/proposition/DES/m-calllog-proj-1/（topic.md 与计分材料）
- 现行实现：sih-tools/calllog/src/calllog/core.py（_render_projection_for_tool、append_call）与 sih-tools/lease/src/lease/calllog_import.py（投影再生步）
- 坑位勘误：sih-tools/BATCH-FACE.md 坑位勘误 2026-09-07 节
- 权威腿现态：sih-tools/calllog/calls.ndjson（697 行主会重建在档，账本为准）

## 六、约束 {#constraints}

1. TDD 先红后绿；既有 lease 238 + calllog 8 全绿零回归
2. 权威腿 calls.ndjson 与索引腿零改动（只修投影腿与 import 与卫）
3. 判定语义零新裁：按 m-calllog-proj-1 落地，遇边界停批呈报
4. 主树验收硬性项：F-1 须在主树真跑，工地读数不算
5. 收约机械链全序照 BATCH-FACE；拆工地前先 cd 出去

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-7 全过（F-1 主树真跑读数在档）
- [ ] CONTRACT 修订与 BATCH-FACE 勘误更新随批
- [ ] 结果档 sih-engine/sih/event/plan/projfix-solo-results.md（T6 管线）与 materials/
- [ ] 双仓 settle + close + reconcile + verify 全绿

## 八、风险点 {#risks}

- 各册表头 schema 异构（lease 册两表头摞立）：verbatim 重组形天然规避，另择形须逐册过 F-1
- 重组形对 append 新行的表格渲染须按册 schema——批内定形并在 CONTRACT 登记

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 F 锚定、得一裁红线、双仓同步；判定语义已前置一裁（m-calllog-proj-1）非本批内裁。

## 十、关联文件 {#related}

- 命题与裁决：sih-tools/proposition/DES/m-calllog-proj-1/；材料 sih-tools/facet/contracts/zc-glm53-20260907-callproj/
- 验收背景：sih-engine/sih/event/plan/calllog-solo-results.md 验收追记节
- 下游批：closegate-solo（CONTRACT 串行位）

## 十一、请求写入 {#requested-writes}

- sih-tools/calllog/src/calllog/core.py（渲染器）与 sih-tools/calllog/tests/
- sih-tools/lease/src/lease/calllog_import.py 与 sih-tools/lease/src/lease/cli.py（跨根卫）与 sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md（修订随批）
- sih-tools/BATCH-FACE.md（勘误更新）
- sih-engine/sih/event/plan/projfix-solo-results.md 与 materials/
- sih-engine/sih/event/trail/（收约链笔）
