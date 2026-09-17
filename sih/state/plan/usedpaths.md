# usedpaths：收约未用锁罚口径精确化

> 令源：2026-09-18 用户令「全都推进，你是主编排，调用多子代理进行并行操作」，候令簿第二件
> 实证源：pk-074 faceprecise-solo 批 faceprecise-analysis.md 第二章，过罚 52.4% 实证
> stem 认领：usedpaths，甲表一件即 zh 收约未用罚口径精确化、派生 usedpaths:new

## 问题陈述 {#problem}

引擎 closegate.rs 罚金块把 used_paths 近似成 allow 声明全集，罚 = locked 减 allow 字符串全等差分。31 笔罚单 124 条罚路径经文件系统 mtime 对表，52.4% 实际在会话窗内写过，属过罚；同时声明未锁与锁了未写在 allow 内无写证两类差值俱无对账通道。

## 关键设计 {#design}

承 faceprecise-analysis.md §2.5 四条：used_paths 改源收约时在会话工地跑 git diff 基于开工基 base_branch，git 不可用回退 allow 近似并 detail.used_source 如实标注；比对改前缀覆盖即 watchcheck covered 同形；集合运算先尾斜杠归一；事件 schema 既有八字段零删零改义只增 used_source。取样点前移归并前因罚金块原位在删支拆本后分支已灭。口径钉形：罚 = 锁面且未被实改集覆盖，allow 内锁了未写仍罚承 §2.3 类三转罚面。

## 工作清单 {#work}

- [ ] up-01：collect_used_paths 与 entry_covered 与 face_entry_map 三纯函数实装
- [ ] up-02：测试六件即主路径与反向钉与目录前缀与斜杠归一与非 git 回退
- [ ] up-03：settle 加 close 加结果档

## 验收 {#acceptance}

新套件六件绿；lease 系既有套件零新红；t2 回执金向量逐字节不破；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/src/bin/lease/closegate.rs
- sih-engine/tests/lease_usedpaths_penalty.rs
