# facepatch-solo：BATCH-FACE 勘误四条累积批

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：2026-09-06 至 09-07 四批实测沉淀的四条勘误候选，累积一次入档；令源即 confmath／confconst／confrulegate／confrevise 四批结果档的误差申报与建议节
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 四条坑位散在四份结果档，未入命令面速查正典，后续批会重踩
- 勘误条目零入档即漂移：BATCH-FACE.md 自身声明「每批跑完后对照本档命令，不一致即登记漂移」

## 二、关键设计 {#design}

BATCH-FACE.md 坑位勘误区追加一节四条（版本追加制，零改动既有条款）：

1. **facet 测量必须从工地 facet 跑**（confmath-solo 主树误写事故根因：score 自主树执行致 trail 误落主树，虽已清理，纪律须成文）
2. **checkcite --cited 是单值参数**：多传仅末位生效即假阳性空转，引用对表须拼接扫描形（confconst-solo 实测：修复后真过且真拦 PROB-015 缺书单）
3. **F-1 红可修复即修复重核、不可修复才停批**：判定材料与规范源失配时，采样前修复为逐字节形且判定文本零改动即可推进，否则停批（confrulegate-solo 实读，主会评估在案）
4. **委外批 open 前按请求写入节逐仓核对 --repo 覆盖**：漏传仓即该仓写入面无工地通道，被迫走收约补笔 bypass 兜底（confrevise-solo 漏 --repo sih-tools 实录）

## 三、工作清单 {#work}

- [ ] 四条入 BATCH-FACE.md 勘误节（追加形，逐条注明令源批名）
- [ ] 管线三步、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 四条入档 | 数据治理 | 勘误节新增四条零遗漏，逐条携令源批名 |
| **F-2** 零改写在役条款 | 治理 | 既有命令与坑位零触碰（diff 仅新增） |
| **F-3** 管线绿 | 治理 | 化格核阅检词三步读数在档 |

## 五、必读文件 {#read}

- 修订对象：`sih-tools/BATCH-FACE.md`（勘误区形态先例即既有各勘误节）
- 令源：四批结果档（confmath／confconst／confrulegate／confrevise）误差申报节

## 六、约束 {#constraints}

1. 零代码改动，零在役判据与退出码语义触碰
2. 追加制不删不改既有条目；勘误只入 BATCH-FACE 不外溢其他文档
3. 主树零直写经工地 settle 通道；在泊件零触碰；遗留无主件不代清

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过，settle 在档，链 verify valid，reconcile 零新增
- [ ] 结果档 facepatch-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 勘误条目与既有条目语义重叠 → 入档前 grep 既有节核对，重叠即合并注记不重复立法

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（写位）

## 十一、请求写入 {#requested-writes}

- `sih-tools/BATCH-FACE.md`
- `sih-engine/sih/state/plan/facepatch-solo.md`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/facepatch-solo-results.md`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 facepatch-solo 工地
