# parkreplay-solo 结果档

> 批：parkreplay-solo 书简泊跨链重放修复批
> 会话：2cf3ee457a53ff9e
> 日期：2026-09-03
> 队形：单线形 solo，零子代理

## F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 跨天路径红绿 | 测试链目录先日链 enter 加当日链 exit，修前拒修后通 | 过 | 认证 be1b1f5d 即 red-stdout OrphanExit exit 1 修前，appended c054ae12 exit 0 修后，双日链各自 verify valid |
| F-2 同日行为不回退 | 既有同日 enter 至 exit 与重入拒单测全绿 | 过（附披露） | scope_tests 3/3 绿；全量 cargo test 127 过 3 败 6 忽略，三败为主树 HEAD 同样复现的金向量漂移见披露节 |
| F-3 语义窄改 | 五子命令输出与退出码零变化，追加面仍单链 | 过 | 旧新二进制 verify 与 query 输出 cmp IDENTICAL；diff 仅四件即 SPEC-006 与 scribe.rs 与 mod.rs 与 park.rs；追加面 genesis 前哈希断言过 |
| F-4 写入仅限 allow | 本批写入仅请求写入节所列 | 过 | 双工地 git status 对表，主树零直写即 PARKING 名册注记与任务包两件为批前既有未跟踪件归 mathpipe-a1 批 |

## 管线读数

化格 exit 0 无需改，核阅 exit 0 零发现（经工地 fixtures/corpus 副本即在域内，跑后即删），检词 exit 0 零违例。词债四件登记入 core 包工地副本随批入版控。

## 认证读数

意图 ec838967；认证五笔即 fmt d27111b9 与 scr 5137994a 与 nom 96792311 与 redgreen be1b1f5d 与 tests 11c69c48（exit-code 1 如实即全量套非全绿）。

## 披露

金向量三败即 golden_des001_gov003 与 golden_des001mathe_lim001 与 golden_des001mathe_mul001，在主树 HEAD 同样复现，根因即金向量冻结 sih-math 活文件内容哈希而 09-02 数学批合法改动该些文件，非本批引入；金向量重录另批承 pk-036 先例。编译警告一件先在即 retriever mod 的 integration_root 死函数，非本批引入。

## 候选申报

号源唯一门：本日 pk-043 撞号暴露单一名册续号不重号无机械门，enter 不查已用号即已出泊号可复用。实装与否待人节点裁，本批只登记词不实装。

## 承接

mathpipe-a1 件 0 即 pk-041 出泊记账以本批交付的重放面为前置，记账位即开工实日链。
