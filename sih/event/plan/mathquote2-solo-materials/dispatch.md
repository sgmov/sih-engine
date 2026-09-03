# mathquote2-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 mathquote2-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/mathquote2-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。

背景一句话：原批 mathquote-solo 把 34 件引文补写与管线三步全做完了，但死在上链之前（零链上事件零结算），主会已强拆工地吊销会话，抢救 diff 落在本批材料目录。你的活是把这批已验证的施工合法落链，机械验证全部本批重跑、读数不继承。

## 施工

1. 枚举对表：materials/queue.json 记四子仓桥接条目 47 件、已锚 13、缺锚 34。本批重跑枚举对表当前盘面（mathrefmt-solo 只动了 calculus，四子仓应无位移；有位移即如实申报再动）。
2. 施工源：materials/salvage-34quotes-2026-09-03.diff（442 行）apply 到 sih-math 工地，或照原脚本从头重做，二选一自报。apply 后逐件跑 materials/reverify_quotes.py 子串断言，断言不过之件零容忍即该件重做。主会已对原 diff 全 34 件复算子串全过，但你的读数必须自己跑出来。
3. 引文落形照原包：原文「引文」见 文件:行号，命题对照保留不替换，哲学仓只读禁手打，calculus 零触碰，已锚 13 件零触碰。

## 机械链（照 BATCH-FACE 全序）

ask3 记录（三锚引文程序切片禁手打）→ 双门（scrutinator packs/ask3 exit 0 + ask3repeater --root 绝对 exit 0）→ 叩问 elicit check --packs sih-tools/nomenclator/packs/core --words 逐字引文 --words 抢救 diff digest passed → 正身 identity verify（reports 落位）→ lease open --package mathquote2-solo --repo 三仓绝对路径（一切锁操作显式 --session）→ 取锁（请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail 主树绝对路径；护栏一在役即工地链路径会被拒，认证一律先落主树活链）→ 工地施工 → 管线三步（化格→核阅→检词，笔在核前判在书简前，findings 亲读）→ 认证逐件 meter 包裹 append 主树活链 → 链文件 settle 前一次性拷工地 → 双仓 settle --seq N --cert 链上哈希前八位 → 放锁（先放锁后拆工地）→ close（备份让位归并对表法，diff 非 identical 即停批上报）→ reconcile → 当日链 verify。

## 冲突模式（承 pk-045）

若与其他批并发：撞锁有限重试上限十次逐次计数；冲突样本节必载（冲突点/时点/对方批/机械响应/解决路径/重试计数），并引用原批死亡样本即施工全绿零上链即死。

## 红线

抢救 diff 是盘点源不是引用源；上链前零结论；34 件外零触碰；主树零直写（治理窄域外文档类落档除外）；守卫在位严禁 plain git commit 直提；findings 亲读；禁管道掩退出码；identity/reports 与既有存量 untracked 零收编。

## 完工报告（最终回复直接输出）

意图哈希、施工源选择（apply 或重做）与复验脚本全绿读数、管线三步读数、认证清单（事件哈希）、三仓 commit 号、链 verify 前后对表、reconcile 读数、F 表（F-1 至 F-5）、冲突样本节、越线与误差申报。
