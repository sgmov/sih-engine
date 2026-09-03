---
title: meter 调用计数判定语义的计数测度承载（新载体立项）
authored: newcarr-solo 新数学载体立项（2026-09-04，m3clear-solo 处置清单候选行）；改写自 m-newcarr-meter-1（首测 basis 共识子判据挂，引用基线轮换致无共识，重测统一引用基线）
ng: medium
n: 9
gid: m-newcarr-meter-2
---

# 待裁命题

meter 的调用计数判定语义可由概率子仓新条目「计数测度与可加性」（PROB-016）承载：逐调用一记录即计数测度的单点赋值，按日分册即测度对时间区间的分割，crosscheck 对账即可加性恒等式校验（总量等于分部之和，漏记重记破坏可加性被机械检出）；计数测度是测度公理（空集零与可列可加）的最小非负实例，PROB-006 概率测度是其规范化特例；该条目无哲学新桥（工程实证语义），定义先行、可证伪节与双答结构在场（M-4），落位 probability 子仓，工程接线归后续逐工具批零接线。

## anchors

- path: meter/src/meter/cli.py
  range: 41-45
  note: _append_record 逐调用一记录，按日分册 append-only
- path: meter/src/meter/cli.py
  range: 129-134
  note: cmd_crosscheck 对账——分册记录汇总对表
