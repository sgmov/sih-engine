# parkrecon-solo：泊界账实对表——pk-045 出泊与撞号改正重编

> parkrecon-solo 泊界对表修复批
> 令源：用户 2026-09-05「pk045出泊，pk055展开看看」；在泊授权承用户 2026-09-05「同意。得一归facet那边融回，你的异常和建议可以入泊」
> 范式：T6-D 偏离单线 solo——零子代理确定性泊界操作批

## 一、问题陈述 {#problem}

- **问题 1**：pk-045（多 agent 冲突测试样本库）出泊条件已达成，用户令出泊。出泊裁定材料与链上 parking_exited 事件缺位，名录投影未更新。
- **问题 2**：pk-054（facet LLM 探针退役）材料在盘但链上零进泊事件（全 trail 五文件零命中），进泊事件从未落链，账实不符。
- **问题 3**：pk-055 与 pk-056 撞号。链为准：链上 pk-055 即「级联边册投影更新机制缺位」（09-04 14:56 进泊）、pk-056 即「温故引擎侧语义通道缺位」（09-04 15:20 进泊），均真在泊；而材料文件 pk-055.json 与 pk-056.json 被后笔活写覆盖为另两件未落链事项（台账活写覆盖根因硬化、判据观察转 facet 融回）且随批误入版控。两件未落链事项承用户在泊授权，须改正重编新号进泊（pk-043、pk-047 先例）。
- **问题 4**：名录投影缺口：链上真在泊的 pk-055（级联投影件）不在名录，pk-054 补进泊后亦缺。

## 二、关键设计 {#design}

### 2.1 处置五件

- pk-045 出泊：disposition promoted，ruling 载用户令与样本销账账本对表（八类已硬化在役销账、九类由租约线批二至批五收口销账、三类转批六视图批跟踪），出泊材料 pk-045-exit.json 随批。
- pk-054 补进泊：材料补 action 与入泊块，scribe park 落链。
- pk-055 复位：材料按链上进泊事件细节复位（级联投影件），活写覆盖内容撤出。
- pk-056 复位：材料按链上进泊事件细节复位（温故语义通道件），同上。
- pk-057 与 pk-058 改正重编进泊：台账活写覆盖根因硬化件编 pk-057、判据观察转 facet 融回件编 pk-058，context 内撞号披露照录，scribe park 落链。

### 2.2 投影更新

- 名录在泊计数改十三项：原十项减 pk-045，加 pk-054、pk-055 补记、pk-057、pk-058。
- pk-045 移历史住户节，出泊事件哈希 close 后回填。
- pk-055 补记与 pk-054 与 pk-057 与 pk-058 名录行随批入投影。

## 三、工作清单 {#work}

- [x] parkrecon-01：账实对表定事实（链五文件停泊事件全量对表、git diff 双件、pk-057/058 号位空位核实）
- [ ] parkrecon-02：四笔停泊事件落链并逐笔 grep 验证
- [ ] parkrecon-03：双材料复位与双新件与出泊件落工地
- [ ] parkrecon-04：PARKING-v1.md 投影更新过管线三步
- [ ] parkrecon-05：结果档含 pk-055 展开说明与撞号披露

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 出泊事件 | 链上 parking_exited pk-045 带 disposition promoted 与 ruling，scribe verify valid |
| **F-2** | 补进泊 | 链上 parking_entered pk-054 在档，材料与链一致 |
| **F-3** | 改正重编 | 链上 parking_entered pk-057 与 pk-058 在档，context 含撞号披露；pk-055 与 pk-056 材料逐字等于链上进泊事件细节字段 |
| **F-4** | 投影一致 | PARKING-v1.md 名录十三项与链在泊集一致，过化格核阅检词 |
| **F-5** | 双跑一致 | 例扫 rev3 双跑四路 cmp 逐字节 IDENTICAL、checkmath 零红 |
| **F-6** | 双仓归并 | 双仓 settle、close 收约、reconcile 判据项零新增、当日链 verify valid |

## 五、必读文件 {#read}

- 泊界向界：sih-engine/doc/governance/PARKING-v1.md
- 出泊先例：sih-engine/sih/state/parking/materials/pk-051-exit.json
- 撞号先例：PARKING-v1.md 名录 pk-043 与 pk-047 撞号披露照录节
- 盘点账本：sih-tools/proposition/DES/leaseopt-audit/census-ledger.json（pk045.categories 销账对表源）
- 线总纲：sih-engine/sih/state/plan/leaseopt-line-v1.md（批六跟踪指针）

## 六、约束 {#constraints}

1. 出泊唯人节点裁决：本批只承载执行，裁定文即用户 2026-09-05 令原话
2. 链为准：材料与投影与链不符处一律按链复位，不复核不改判既有进泊事件
3. 零代码改动：本批不触任何工具与引擎代码，CONTRACT 零修订

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 四笔停泊事件逐笔落链验证在档（禁 meter 包裹掩败，逐笔 grep 复核）
- [ ] 名录投影与链在泊集机械对表一致

## 八、风险点 {#risks}

- 主树双材料脏位：close 前以 git checkout 复位至 HEAD 消脏，链真内容由批分支归并到达，全程留痕申报
- 停泊事件写入失败被掩：scribe park 裸调不加 meter 包裹，逐笔 grep 链文件验证

## 九、范式偏离声明 {#deviation}

确定性泊界操作批，零子代理单线 solo；保留 T6-D 命名约定与 F 锚定与双仓同步。

## 十、关联文件 {#related}

- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/parking/materials/pk-045.json 等六件
- sih-engine/sih/event/plan/parkrecon-solo-results.md（随批产出）
- sih-engine/sih/event/plan/leaseopt-audit-solo-results.md 第五节 pk-045 对表

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/parking/materials/（pk-045-exit.json 新增、pk-054.json 改、pk-055.json 复位、pk-056.json 复位、pk-057.json 新增、pk-058.json 新增）
- sih-engine/sih/event/plan/parkrecon-solo-results.md 与同目录 parkrecon-solo-materials/
- sih-engine/sih/event/trail/2026-09-05.ndjson（书简停泊与认证追加）
