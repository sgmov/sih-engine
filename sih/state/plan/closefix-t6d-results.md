# closefix-t6d 结果档

## 概览 {#overview}

- pk-022 出泊修复落链即 close 归并腿改序归并先行于拆本加返回码全检，假绿堵死，版本 1.1.1，二十七测全绿::[判据核对](#criteria)
- 机械链即意图 round1 84160ab9、出泊 5ded133e、会话 5524170ec5213c03、五路径锁::[链上证据](#evidence)

## 链上证据 {#evidence}

意图入链
: round1 84160ab9 即 sess-zcode-260826-closefix，双腿绿一次过三锚点

出泊
: pk-022 出泊事件 5ded133e 即 disposition promoted，首轮 disposition 值 resolved 被书简校验拒即枚举限 promoted 或 discarded，改正后过，材料移除投影改写

会话开工
: 5524170ec5213c03 即双仓 sih-tools 与 sih-engine，包经缺省位解析

五锁
: sih-tools/lease 即 sih-tools/parking 即 sih-tools/scribe 即包两件

修复件
: close_session 改序即锁清零前置、ahead 计数、归并先行返回码全检、失败计 merge_failed 或 branch_delete_failed 不吊销、remove 成功补 result 赋值、删支全检、missing 容忍承前判

测试
: 新增阻塞拦与自愈复跑一件即主线未跟踪文件撞副本提交路径时 close 退出码一计 merge_failed 分支在工地在，清障后复跑归并成删支成吊销成，共二十七绿

契约与版本
: 修订七落字即归并先行与三失败态，版本 1.1.1

管线三证
: 化格核阅检词对 CONTRACT 与包档，哈希随认证事件见 trail 即 2026-08-26 链

段结算提交
: 双仓经 lease commit 正身路径，提交号见 trail 与 git log

## 判据核对 {#criteria}

L1 过即阻塞场景测试拦。L2 过即自愈复跑测试全绿。L3 过即既有收约两态与脏工地自愈测试不破即二十七全绿。L4 过即修订七落字。L5 过即版本 1.1.1。L6 过即管线绿认证上链链 valid。L7 过即双仓提交经正身路径。L8 过即批闭即收约。

## 偏离登记 {#deviations}

偏离一：实施先于部分登记即代码修复落笔在出泊与立约后但在结果档前，时序即出泊即立约即锁即修即档，链完整。

偏离二：编辑位在主检出即副本空转，承编辑位迁移批未行先例。
