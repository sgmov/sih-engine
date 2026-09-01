# secondcarr-solo：第二载体入闸批

> task-packages 治理任务
> 承接：用户 2026-09-01 批准令 + 得一 m-mathnext 机器终签（重编号机制方向）
> 队形：单线形 solo（子代理产出已在收件台 agent-drafts/epi/）
> 日期：2026-09-01

## 一、问题陈述 {#problem}

复归补强第二波七件绿稿入正身。四件 ORD 重编号 011 至 014 改 015 至 018 让位在册良基条目（得一终签机制方向），三件保留拟派号，正文旧号自指随编号机械改写，mapping 与两子仓 INDEX 登记新行。验收全量 148 件核阅零违规。

## 二、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 恰七件新增 | 工程 | git 恰七新条目加 mapping 加两 INDEX |
| F-2 零违规 | 工程 | 全量 148 件 total=0 |
| F-3 编号无撞 | 治理 | order 子仓 011 至 018 逐号唯一且各文件 H1 与文件名与 mapping 三处一致 |
| F-4 自指改净 | 治理 | 新七件正文无 011 至 014 旧号残留指向自身 |
| F-5 锚点逐字 | 治理 | 七件十四锚原文逐字在册（入闸前已验，入库归一化不改锚） |
| F-6 化格检词 | 治理 | 化格 0 改，检词如实 |
| F-7 lease 链 | 治理 | 双仓全程，verify 零，快照末笔后 |

## 三、请求写入 {#requested-writes}

- sih-math/order/entries/
- sih-math/probability/entries/
- sih-math/algebra/entries/
- sih-math/llm-friendly-build/mapping.md
- sih-math/order/INDEX.md
- sih-math/probability/INDEX.md
- sih-math/algebra/INDEX.md
- sih-engine/sih/state/plan/secondcarr-solo.md
- sih-engine/sih/event/plan/secondcarr-solo-results.md
- sih-engine/sih/event/plan/secondcarr-solo-materials/
- sih-engine/sih/event/trail/2026-09-01.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 四、约束 {#constraints}

零 LLM 于工具执行层；不动七件数学内容；上链遇锁即等待；链快照在末笔后。

## 五、风险 {#risks}

重编号自指改写漏一处即 F-4 拦；检词题名撞词族或再现即存量如实记。
