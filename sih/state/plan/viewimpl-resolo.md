# viewimpl-resolo：接续 viewimpl-solo 收约阶段未挂段结算的 commit

> task-packages 治理任务
> 承接：viewimpl-solo 批 lease close 后 reconcile 报 unrouted=2 即 f616b07 与 369a80b 两 commit 缺段结算挂载
> 队形：单线 solo
> 日期：2026-08-31

## 一、问题陈述 {#problem}

viewimpl-solo 批 close 阶段因 src/lib.rs 漏列 allow 触发范围闸拦 commit，走手动 worktree commit + 主树直写兜底，close 阶段归并成功但两条 commit 未挂 lease --stage settle 段结算，reconcile 报 unrouted=2。本批补挂。

## 二、关键设计 {#design}

一接续即以同 subject 段结算两 commit 挂 lease commit --stage settle --seq 1 --cert cbdeb8fc 补挂 f616b07 即本批工作分支 commit 落 src/view/ + viewer + GOV-003 + trail，再 --seq 2 --cert 369a80b 补挂主树直写 commit。

## 三、工作清单 {#work}

- [ ] 补挂 f616b07 与 369a80b 段结算
- [ ] reconcile 验零 unrouted
- [ ] 验链 valid

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 段结算挂载 | 治理 | reconcile unrouted=0 |
| **F-2** 链 valid | 治理 | scribe verify status=valid |

## 五、约束 {#constraints}

补挂不得改 src/ 内容；不得改 GOV-003 文本；不得改 trail 内容；纯段结算操作。

## 六、请求写入 {#requested-writes}

- sih-engine/sih/event/trail/2026-08-31.ndjson（lease commit 段元信息追加）
- sih-tools/lease/ledger/sessions.ndjson（开约与收约登记）

## 七、风险 {#risks}

补挂若认证哈希错位会被 reconcile 报 detail 不符。鉴前以 trail 实际末笔哈希为锚。
