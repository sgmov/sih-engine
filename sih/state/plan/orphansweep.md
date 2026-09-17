# orphansweep：孤儿全量收编与裁定落账

> 令源：用户 2026-09-17 令「pk-108 的两个决策是什么？以后没有批的文件自动跟批跑，如果无用直接删除。」
> 范式：solo 单线，agent 亲写零子代理
> 前置：binmerge 批已收约，pk-108 在泊候裁；本批即裁定承载批兼首次执行批
> stem 认领：orphansweep，甲表三件即 zh 孤儿清扫收编、code 无承、派生 orphansweep:new

## 一、问题陈述 {#problem}

- pk-108 在泊候裁定，原出泊条件两钉即 bypass-orphan 事由须附处置指针（泊位或待办）无指针即拒、孤儿件同文件连续两批被点名即升级呈报人节点。用户裁定改立简式缺省处置规：没有批的文件自动跟批跑，如果无用直接删除。两钉被简式规取代。
- 双仓孤儿库存：引擎仓未跟踪一百零六件（批材料、results 档、任务包、泊材料 pk-102 与 pk-104）俱在 sih/ 树内；工具仓登记账面一千二百余行（scribe/reports 九百件、lease 台账与回执、proposition 双目录、parking、facet 契约、work 目录、scribe/trail、AI-MANUAL 语义修正、tokens 活账本）。
- 无用件三类：工具仓四枚零字节锁件与 lock.db 与 mathclose 备份件；引擎仓 trail 目录迷路文件 .ndjson（estcarr 认证笔重复写入，同 report_hash 正笔在 2026-09-13 正链，自成全零孤链，删无损）。

## 二、关键设计 {#design}

- 处置规三元：有值记录件跟批跑即随本批 settle 入版控；活性账本（locks 与 sessions 与 claims 与 bypass 与 tokens 与 lockface-bills）按既册 tracked 形提交现态；无用件直接删。执行从主树正身仓 settle（本批对象即主树在位孤儿，无工地形，承 lease commit 正身仓调用面）。
- pk-108 出泊 promoted：裁定源即用户简式规原话，裁定取代两钉即排除性分类缺口以缺省处置规闭合，本批即首次执行，出泊事件随批上链，PARKING-v1.md 名册补记 pk-102 至 pk-108 七件对齐链。
- PARKING-v1.md 属 doc 域走化格核阅检词三步序固定，批材料含本批处置清单即 inventory 档随批。

## 三、工作清单 {#work}

- [ ] os-01：批三件落位（任务包、意图记录 plain 形、正身报告）加 lease open 甲表认领
- [ ] os-02：删三类无用件；pk-108 出泊笔上链；PARKING-v1.md 名册补记加 T6 三步
- [ ] os-03：双仓 settle（引擎含全部未跟踪 sih/ 件加索引档，工具含全量记录件与账本现态）
- [ ] os-04：close 加 reconcile 加链验加推双远端，双仓 status 转净

## 四、验收 {#acceptance}

- 双仓 git status 零未跟踪零修改（活账本即时增量除外）；pk-108 出泊事件在链且名册投影同步；当日链 scribe verify 判词 valid；reconcile 双仓 unrouted 与 cert_missing 零新增；双远端快进。
