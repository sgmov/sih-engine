# parksync：泊界名册投影对链同步

> 令源：2026-09-18 用户令「全都推进，你是主编排，调用多子代理进行并行操作」，候令簿第五件
> 欠账源：orphansweep 名册补记披露投影欠账十一件，加 2026-09-17 两波出泊二十三笔未投影
> stem 认领：parksync，甲表一件即 zh 泊界名册对链同步、派生 parksync:new

## 问题陈述 {#problem}

PARKING-v1.md 当前在泊行还写十三项，链面轧差真实在泊仅 pk-026 与 pk-078 两件；2026-09-17 pathfix 一笔加 pk-016 一笔加 pkexits2 十笔加 pkexits3 十一笔俱未投影，名册与链差距已聚大。

## 关键设计 {#design}

承 acceptclose 先例逐笔对链：全量重放泊事件轧差推真态；外科手术式改文件即在泊行改写加名册补记节逐件列事件 id 与 disposition 与承载，历史散文快照照录不动；工具线 PARKING-v1.md 漂移只呈报随本批 sih-tools 侧提交带走。

## 工作清单 {#work}

- [ ] ps-01：链面轧差推真态
- [ ] ps-02：在泊行改写加补记节
- [ ] ps-03：T6 三步全零
- [ ] ps-04：settle 加 close 加结果档

## 验收 {#acceptance}

在泊行与链面逐件吻合；补记节逐件有事件 id；三步退出码全零；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/doc/governance/PARKING-v1.md
- sih-tools/PARKING-v1.md
