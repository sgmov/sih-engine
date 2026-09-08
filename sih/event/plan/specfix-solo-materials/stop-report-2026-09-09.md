# specfix-solo 批停批读数汇总

批名 specfix-solo，停点：BATCH-FACE 第 7 步管线三步之检词腿（化格 0 与核阅 0 已达成，检词 SPEC-021 rc=1 一笔登记债）。链停于此，未认证未 settle 未收约，锁面保持，候人节点裁决。

## 一、使命进度实况

- 使命判据「消主树核阅存量红两笔」：已达成。主树存量红两笔即 docmath gate 报 SPEC-021 与 SPEC-022 缺内容充分性节（红证源头 mcpspec-solo 批 first-run-2026-09-09 双跑 cmp 逐字节一致），本批工地件已各补充分性节，两档核阅终绿 rc=0 findings 0（读数件 scrut-final-SPEC-021.json 与 scrut-final-SPEC-022.json）。
- 使命判据「管线全绿收约」：未达成。SPEC-021 检词 rc=1 一笔，系批外登记债非本批引入，本批权限内不可修复，停批不硬凑。

## 二、读数全录

- 三问双门：第一门 scrutinator ask3 包 rc=0，第二门 ask3repeater status ok anchor_count 3（补叩问处置后双门复跑仍绿）
- 叩问：3 轻信号（判定性常数、判据红证对表、约束算子）均申报为充分性模板正典原文词面引用不立名，digest passed covered 3
- 正身：anomalies 空
- 租约：open 会话 8daf996a94e659e9 双仓 worktree 登记，十锁全取
- 书简意图：链笔 event_hash 572c027478408e9930b8800c0370d9440ac73faa5dc1c6f8b295ff0a07982b96
- 工地：worktrees/sih-engine/specfix-solo 两档已改（SPEC-021 补充分性节加既有 116 行格式归一去粗体括号注、SPEC-022 补充分性节加既有 3 至 4 行块引用转段与 49 行括号注改即字，全为格式归一不改义，diff 逐行在工地 git 可复算）
- 管线：化格 SPEC-021 rc=0 与 SPEC-022 rc=0（零改动正形）；核阅终跑 SPEC-021 rc=0 findings 0 与 SPEC-022 rc=0 findings 0；检词 SPEC-022 rc=0 findings 0，SPEC-021 rc=1 findings 1

## 三、SPEC-021 检词红一笔的定性证据链

- 红笔：lazy_in_doc medium，词「基线管」，line 25，即一词两物节既有行「共享家位：两物同住基线管即五子模块，规约向量生成后冻结进基线管」
- 官方态读数（nomenclator-query-jianguan.json）：state lazy，since 2026-09-06，source「toolincub-solo 批基线管工作名懒波登记，承 SPEC-021 在用词」
- 来历：gvec-v2-serial 批 2026-09-06 成文 SPEC-021 时结果档 F-2 载「检词 0」过闸（gvec-v2-serial-results.md 62 行）；同日 toolincub-solo 批把「基线管」登记入 lazy 台账（toolincub-solo-results.md 20 行与 35 行），此后 SPEC-021 检词形态收紧为 rc=1。主树基线复跑同红一笔（scrut-firstrun-main-tree-SPEC-021 同日另有 nom 主树红可复现），即登记债非本批改动引入
- 契约语义：nomenclator CONTRACT.md 66 行「懒波存量债，处置承体例补记五原文即存量维持登记在案待立名自然消化、新产文档禁入、无限期强制条款，十八处基线发现即为登记存量债，闭合不另立新规」；SPEC-021 系存量档非新产文档，本批新增充分性节零用懒波词

## 四、三修复通道禁因（本批权限内不可修复）

1. 文档改词消债：铁律「两档既有节零语义改动（格式化格归一除外且不得改义）」——「基线管」是概念指称改写即改义，禁
2. lazy.json 加 exempt 行级豁免：实现上生效（check.py 第 35 行豁免路径对 lazy 条目同走）但契约修订三只正典化 dead 条目 exempt 且 sih-tools/nomenclator/packs/core/lazy.json 不在本批 allow 面，双禁
3. lazy 升格 established：承契约须显式升格登记即立名行为，「立名不能机械化」非本批批职能，禁

## 五、候裁选项呈报

- 裁一：认 lazy_in_doc medium 登记债为存量在案合法形，本批放行续链（认证 settle 收约），验收检词项如实载 SPEC-021 rc=1 一笔登记债——需任务包验收条款「检词 0」的人节点豁免裁
- 裁二：先走懒波债处置通道（词表 exempt 扩容或升格立名流程）由工具线批承办，本批待债清后复链收约
- 裁三：拆批收桌，工地与锁面按收桌程序处置

## 六、停点现状

- 锁面：本会话十锁在持未放
- 链面：今日 trail 本批一笔 intent 在链（572c0274），未认证
- 工地面：worktrees/sih-engine/specfix-solo 分支 msh/specfix-solo 未提交，两档改后件在工地
- 红证：全量在 sih-engine/sih/event/plan/specfix-solo-materials/（scrut-firstrun-main-tree 两件、scrut-run2 两件、scrut-final 两件、nom-firstrun-wt-SPEC-021.json、nomenclator-query-jianguan.json、本件）
