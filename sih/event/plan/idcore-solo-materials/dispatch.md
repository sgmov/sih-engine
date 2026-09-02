# idcore-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 idcore-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/idcore-solo.md 与决策材料 /Users/moc/workspaces/SiHankor/sih-engine/sih/event/plan/pk041-solo-materials/identity-drift-proposal.md（§4 方案设计、§7 变更清单、§8 迁移路径为准绳），调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md（含坑位注记逐条照办）。凭据：用户 2026-09-03 采纳裁在案（正身修复同意，四裁定随批准采纳），命题 m-idcore/m-idcore2 两轮 boundary 通道错配改道在案（依据 facet-measure 修订一，改道依据与两轮 gate 读数须如实载入结果档）。

## 实装要点（承任务包 §二 与提案 §7，逐文件）

1. sih-tools/identity/src/identity/core.py：新增 core_hash(components) 与核键序常量与 lineage 归一化函数；net_time 探针可选化双轨（--no-net 显式旗标由 cli 传入，采集超时或禁网自动降级为空串加异常标记，不阻断）；detect_anomalies 的 clock_skew 在 net_time 在场时逻辑不动。红线：v3 身份串 identity_string、盐哈希 identity_hash、new_salt、COMPONENT_ORDER 四件逐字节不变。
2. sih-tools/identity/src/identity/cli.py：报告 JSON 增 identity.core_hash 与 identity.core_components（核键值对表，供审计复核）；退出码三值与既有参数不动。
3. sih-tools/lease/src/lease/core.py 的 load_identity：增读 core_hash 入台账 identity 字段；identity_hash、file_sha256、make_session_id 派生、lockcore 三键硬闸全不动。
4. sih-tools/tally/src/tally/cli.py R5 段（约 134 至 169 行）：材料与基线双带 core_hash 即按 core_hash 优先配对；任一缺席回退现有结构（约 161 至 169 行回退路复用），旧材料重放逐字节同判；R5_VALUES 三态映射与输出契约不动。
5. 三工具 CONTRACT.md 各追加一条修订（编号递增，体例承各文件既有修订节）。
6. 测试守卫（承任务包 F-6）：identity/tests/test_identity.py 增核哈希用例组（金标 82f460c2ac2741fc4ee8d104d7ef14803f744da14a24b1d7356bd3bd3531d1dc 逐字节断言、同参双跑一致、五易变件与盐全改核哈希不变、mac/hostname/user/sandbox_id/血统 token 五向单点篡改各变、归一化四例即 host-local-1 与 -2 同 token、三 harness 两两异、空 ancestry 空 token 不判败、boottime 加一秒核变、net_time 置空核不变、存量 39 报告 full 复算逐字节回归）；tally/tests/test_tally.py 增 R5 优先配对与回退回放用例；lease/tests 增 load_identity 读数用例。三工具 pytest 全绿计数入结果档。
7. 检词词债：文档新词（核哈希、连认、身份核等以检词实跑违例为准）先过 sih-tools/nomenclator 登记再入文，terms.json 与 manifest.json 随增订升版。

## 机械链（照 BATCH-FACE 全序，本批参数如下）

ask3 记录（session sess-zcode-260903-idcore；三锚引文程序切片禁手打：07-on-assay.md 第 61 行、06-on-canon.md 第 185 行、08-on-settle.md 应而不藏所在行）→ 双门（scrutinator --pack packs/ask3 退出码 0 + ask3repeater --root 绝对路径退出码 0）→ elicit check --words 核哈希 --words 连认 --words 身份核 各词重复传入加 digest passed → 正身 verify（报告 sih-tools/identity/reports/2026-09-03-idcore-identity.json，不入册）→ lease open --package idcore-solo --repo 两仓绝对路径 --root /Users/moc/workspaces/SiHankor（一切锁操作显式 --session；撞锁即停批报告）→ 取锁按任务包请求写入节逐路径（显式枚举，禁花括号展开）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-03.ndjson 绝对路径；链现 8 事件 valid 尾哈希 14e4d03ff028cf7a，settle 前后 wc -l 与末哈希对表留证）→ inputlog 补录一笔（sih-engine/sih/event/inputlog/2026-09-03.ndjson，seq 递增，sess-zcode-260903-acceptor，逐字）：`正身修复同意，ai-ex沉淀同意`（note 即正身修复采纳裁与 ai-ex 沉淀批准令）→ 工地施工：代码与文档全部在 worktrees/sih-tools/idcore-solo 与 worktrees/sih-engine/idcore-solo，主树零直写；测试从工地树跑（uv run --project 工地路径）→ 管线（笔在核前：化格 uv run formatter --pack packs/general-v1 --write 逐 md；核阅引擎二进制 --pack des-001 裸名——本批引擎侧文档在 sih/event/plan 与 sih/state/plan 属域外，exit-2 如实记不算违规；检词 uv run nomenclator check --pack packs/core 逐 md 零违例；findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 scribe append → 双仓 settle --seq 1 --cert 链上 certification_completed 哈希前八位 → 放锁 → close（备份让位归并对表法，diff identical 留证）→ reconcile 双仓四类双零 → 当日链 verify → 调用册留痕（scribe、formatter、scrutinator、nomenclator、identity、lease、tally 触及者各一行）→ 结果档 sih-engine/sih/event/plan/idcore-solo-materials/idcore-solo-results.md（F 表写全、通道错配改道记录节、越线与误差申报、队形声明）→ 全部产物含链尾随批入版控。

## 红线

v3 身份串、盐哈希、new_salt、COMPONENT_ORDER 逐字节不变（改动即批失败）；lease 三键硬闸与会话号派生不动；tally R5 三态映射与旧材料回放同判不动；退出码三值全工具不动；identity 零写路径（报告只 stdout 由批流程落盘）；历史 seat baselines、台账历史行、存量 reports、泊件 pk-037/039/041/042、intanchor 与 pk-043 相关件零触碰；scope 显式枚举禁花括号展开；守卫在位严禁 plain git commit 直提（--no-verify 加 lease bypass 记账才可绕且须申报）；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批；meter --quiet 位照 BATCH-FACE 坑位注记。

## 完工报告（最终回复直接输出）

意图哈希、F-1 至 F-7 逐项结论（含三工具测试计数、金标断言值、39 报告回归读数、R5 回放同判证据、v3 四件 git diff 为空证明）、CONTRACT 修订号三笔、词债登记清单、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、越线与误差申报、通道错配改道记录在结果档的确认。
