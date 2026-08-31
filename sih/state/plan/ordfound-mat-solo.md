# ordfound-mat-solo：补提材料迷你批

> task-packages 治理任务
> 承接：ordfound-solo 批任务包请求写入漏列材料目录，guard 正确拦截 staged_out_of_scope，主件先过、材料迷你批补提
> 队形：单线形 solo
> 日期：2026-08-31

## 一、问题陈述 {#problem}

ordfound-solo 批 open 时任务包请求写入清单漏列材料目录 sih-engine/sih/event/plan/ordfound-solo-materials/，guard 拦截引擎 settle commit 的 staged_out_of_scope。主件 settle 已过（engine 45c6926），四件材料 json 已抢救至主检出未跟踪，留盘待本迷你批补提转正。

## 二、关键设计 {#design}

接 ordfound-solo 批两段式治理的第二段。任务包请求写入只列材料目录一行，open 时 allow 自然含材料目录。engine worktree 内复制主检出的四 json 材料、add、settle commit。认证哈希复用链上已有四笔（核阅 b9eb...、检词 54f8...、化格 995a...、结果档 9dd2...，报告内容哈希已在链，cert_not_on_chain 可过）。close 归并后未跟踪杂件转正，清掉主检出未跟踪残留，免挡下一批 meet 归并。

## 三、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 材料转正 | 工程治理 | 四 json 由主检出未跟踪转为 engine 主检出 git 跟踪 |
| **F-2** cert 复用 | 链上治理 | settle commit --cert 取链上既有认证哈希，cert_on_chain 过 |
| **F-3** 主件不动 | 链上治理 | ordfound-solo 主件结果档 45c6926 与 math 7b6341b 内容零改，链上哈希不变 |

## 四、必读文件 {#read}

- 必读 1：sih/event/plan/ordfound-solo-materials/ 四 json 材料
- 必读 2：sih/event/trail/2026-08-31.ndjson 链上既有认证哈希

## 五、约束 {#constraints}

1. 一切编辑在租约副本 worktrees/ 下
2. 认证哈希从链文件取全长 64 位
3. 不回写 ordfound-solo 主件结果档，缺陷记本迷你批结果档
4. 零粉饰，FAIL 就是 FAIL

## 六、请求写入 {#requested-writes}

- sih-engine/sih/event/plan/ordfound-solo-materials/
- sih-engine/sih/event/plan/ordfound-mat-solo-results.md
- sih-engine/sih/event/trail/2026-08-31.ndjson
- sih-engine/sih/state/plan/ordfound-mat-solo.md

## 七、关联 {#related}

- 关联：ordfound-solo 批、sih/event/trail/2026-08-31.ndjson