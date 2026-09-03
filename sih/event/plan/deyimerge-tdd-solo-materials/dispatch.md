# deyimerge-tdd-solo 执行指令

你在 SiHankor 工作区 /Users/moc/workspaces/SiHankor 执行治理批 deyimerge-tdd-solo。先读任务包 /Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/deyimerge-tdd-solo.md 与本指令，调用面照 /Users/moc/workspaces/SiHankor/sih-tools/BATCH-FACE.md。得一融回三步曲第二步：按 SPEC-014 腿切分清单实装，融回十一行 Python 功能重写为 Rust 落引擎侧，留堰十二行零碰。

## 实装要点

1. **先全读 SPEC-014**（/Users/moc/workspaces/SiHankor/sih-engine/doc/spec/SPEC-014-attractor-mergeback-gap.md）与两工具基准源码：facet 侧融回件 contract_mode.py、facet_stats.py 三件、compiler.py、validators.py、anchors.py、model_utils.py、paradigm_loader.py；tally 侧 src/tally/cli.py 全件与 tests/test_tally.py（随迁作 TDD 基线）。留堰件只读不改。
2. **src/attractor/ 模块形**：子模块 contract_mode（合同 emit 与 load 与严格对表与 dc 装配与计分材料）、stats（metrics 距离四函数、inf 推断校正五函数、conv 收敛二函数）、compiler（聚合与发散发谱与收敛装配）、validators、anchors、model_utils（actor_id 拆解）、paradigm_loader（yaml 装载：Rust 侧引入 serde_yaml 或等价，Cargo.toml 声明）、tally 机械核对（check/verify/assemble/watch/sign 五子命令逻辑，sign 的链上写经子进程调引擎 scribe crosscheck 即围堰 tally sign 同款形态）。lib.rs 导出公共 API。引入 yaml 依赖属回迁债预期内，Cargo.toml 随批声明。
3. **src/bin/attractor.rs**：子命令面 emit-contract / score / check / verify / sign / watch，位置参数承围堰形，退出码对齐（0 合规、1 违规或拒、2 工具异常；tally sign 三态承五修后现文即 refused 1、scribe 失败打 sign failed 透传、signed 0）。
4. **des-011 判定规约包随迁**：从围堰定位 des-011 相关规则数据（若在 facet 侧以数据文件形态存在即随迁 src/attractor/packs/des-011/，manifest 加规则编译期内嵌承核阅先例；若规约形态是代际参数即按 SPEC-014 契约对表节落位，形态如实入结果档）。
5. **金向量冻结（脏目标条款）**：以围堰 Python 原件为唯一基准。取真实判定材料（adisp-guard-1 与 m-p3xcarr 的合同与响应与核对报告是现成净目标）加构造脏目标至少三形（挂起 near_threshold、材料退回 R2-R7 失败项、告警 R7 超三次改写）加合同类拒收基线（缺发、键不符、shot 错位、raw 空至少各一）。围堰件跑期望输出冻结 src/attractor/fixtures/golden/，含合同哈希与响应哈希与终签锚哈希逐字节。浮点文本形豁免按 SPEC-014：facet_stats 数值面浮点文本形不逐字节，结构面（字段名、键序、类型）逐字节。
6. **T1-T6 先红后绿**：T1 金向量冻结（红即 fixtures 不存在）、T2 引擎件对金向量逐字节、T3 退出码五场景对齐围堰（合规 0、单违规 1、挂起 1、域外 2、缺包 2，以围堰实测为准）、T4 多包（des-011 加第二包同载归因）、T5 不变量（机械腿零网络零 LLM：代码审查断言无 http 客户端依赖加测试断言离线可跑）、T6 跨腿契约（score 报告字段与 tally check 输入逐字段兼容断言）。红转绿迹全留。
7. **腿切分逐条验**：融回件 import 面审查零依赖留堰件（llm_client、req、env_loader、engine、runner 等），逐条结论入结果档。

## 机械链（照 BATCH-FACE 全序）

自写 ask3 记录（三锚引文程序切片：建议 06-on-canon 损补、07-on-assay 映照、08-on-settle 留痕；引文禁手打）→ 双门（cd sih-tools/scrutinator && uv run scrutinator --pack packs/ask3 <记录> 必须 0；sih-engine/target/debug/ask3repeater <记录> --root /Users/moc/workspaces/SiHankor 必须 0）→ elicit check --packs sih-tools/nomenclator/packs/core --words 词 --words 词（建议查：随迁、重写、金向量、腿切分、归因）加 digest passed（信号在任务包叩问处置节逐条落处置行）→ 正身（reports/2026-09-02-deyitdd-identity.json，不入册）→ lease open --package deyimerge-tdd-solo --repo 两仓绝对路径 --root /Users/moc/workspaces/SiHankor（**一切锁操作显式 --session <本批会话号>；现存陈旧会话 viewrider 系与本批无关；撞锁即停批报告**）→ 取锁（任务包请求写入节逐路径）→ meter 包裹引擎 scribe intent 上链（--trail /Users/moc/workspaces/SiHankor/sih-engine/sih/event/trail/2026-09-02.ndjson 绝对；链现 79 事件 valid，settle 前后 wc -l 与末哈希对表留证；主树链尾若有在途批事件尾随本批入册承 guardhook 先例并在误差申报）→ inputlog 补录一笔（seq 递增，会话号 sess-zcode-260902-acceptor，逐字）：`批准，照旧你拉子代理跑`（note 注明即 TDD 批放行令）→ **工地施工**：引擎全部实装在 worktrees/sih-engine/deyimerge-tdd-solo 内写与 build 与 test（cargo build 加 cargo test 全在工地跑，settle 前 cargo build 后实跑 attractor 一行行为探针留证）；包档与结果档同工地；tools 侧 CALL-LOG 与 counts 在 worktrees/sih-tools/deyimerge-tdd-solo。主树零直写 → 管线（化格工具件 --write 位置参数；核阅引擎件 --pack des-001 裸名：SPEC-014 若有改动域内必须零违规、包档域外 exit-2 如实记、结果档域外如实记；检词 core 包零违例；findings 亲读禁管道掩退出码）→ 认证逐件 meter 包裹 append → 双仓 settle --seq 1 --cert 链上哈希前八位 → 放锁 → close（主树碰撞备份让位归并对表法 diff identical；归并形天然合规守卫）→ reconcile 双仓四类双零 → 当日链 verify → 调用册留痕（scribe、formatter、scrutinator、nomenclator 四件）→ 结果档 F 表写全 → 全部产物含链尾随批入版控。

## 红线

留堰十二行与 facet 与 tally 全部源码零改动（基准侧冻结）；融回件零依赖 llm_client/req/env_loader；金向量冻结后零漂移；围堰输出是唯一基准禁自造期望；引擎既有五组件源码零碰（只增 attractor 与 lib.rs 导出行与 Cargo 依赖）；守卫在位严禁 plain git commit；上链前必须等绿；findings 亲读；禁管道掩退出码；撞锁即停批；identity/reports 与既有存量 untracked 零收编（batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages、sealwin3-solo-materials、leasepatch/proposition-defense 两 M、adjudisp 与 guardhook 与 deyimerge-sdd 与 listzero 与 p3xbridge 与 legacytwo 既有件）。

## 完工报告（最终回复直接输出）

意图哈希、模块行数统计（src/attractor/ 总行数与各子模块行数）、腿切分逐条验结论（23 行逐行融回/留堰/零依赖确认）、金向量清单（件数与脏形覆盖）、红转绿迹与 cargo test 计数、构建探针证据、认证清单、双仓 commit 号、链 verify 前后对表、reconcile 读数、F 表、越线与误差申报。
