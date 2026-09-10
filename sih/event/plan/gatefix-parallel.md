# gatefix-parallel 域面闸组与 MCP 写面修并联批任务包

> 形：并联 parallel，第二例。令源：用户 2026-09-11 令「多子代理全量修复」，兑现 pk-091 与 pk-092 与 pk-093 三账出泊条件即「用户令立批走引擎域纪律」。三写簇按树互斥：lease 树、mcpline 树、locks 树。批机械链全序正典：sih-tools/BATCH-FACE.md 逐命令 verbatim 承接。温故切面：recall-scope 与 recall-teaching 与 recall-chain 三组在 gatefix-parallel-materials。

## 欠账到簇映射

- 簇L scoperoot：pk-091 其一其二即 commit 范围验根锚失效与 lock 与 commit 归一互斥 + pk-093 其二即 build_message wip 形丢 note + pk-093 其三即请求写入解析污染
- 簇M penface：pk-091 其三其四即 MCP trail 透传缺位与 valid_params 教学缺位 + pk-092 真患二即 runtime 读面缺省未分形 + pk-093 其一其四即隐式占位会话互斥与 repo 校验误教
- 簇K trailhome：pk-092 真患一即 locks 缺省链册单居所漏拦
- 主线保留：三 pk 账出泊笔与 campaign results 档与 commit；map MCP 投影与批词汇终裁与环境红隔离批不在本批

## 簇L scoperoot 批

批名 scoperoot 查册 unknown，--new-stem 甲表认领：zh 范围归一根形，派生 scope 即范围既立语素、root 即根锚定，无既裁 code 形显式申报无承。正典输入：sih-engine/sih/state/parking/materials/pk-091.json 与 pk-093.json、sih-tools/lease/src/lease/commitcore.py 282 至 330 行、lockcore.py 101 至 107 与 157 行、core.py 743 至 762 行解析位与 declguard-solo 冻结常数注。

- 件一 归一共形：commit 与 lock 两闸路径先归一再 scope_allows 比对，归一式单一函数两闸共用即剥前导 ./ 与条目同剥；repo_rel 点号形即受检路径取仓相对裸形；allow 条目点号语义定为仓根递归全容
- 件二 存量兼容：司梦实证 allow 显式 ./ 前缀条目现可过 commit 闸，归一后此类条目剥前缀等价不破
- 件三 build_message wip 形纳 note：有 note 即入 message，settle 形零变
- 件四 请求写入解析净化：列表项遇「——」即截断取路径部，理由散文不入 allow；此系 declguard-solo 冻结常数，走 lease CONTRACT 修订不裸奔
- F 锚定：F1 根锚定夹具仓即 repo 恰为 root 细粒度 allow 提交过且先红；F2 allow 点号形子路径取锁与提交两闸俱过且先红；F3 legacy 点号斜杠前缀 allow 等价过；F4 wip note 入 message 先红后绿；F5 解析夹具即司梦原例行 api/migrations/ —— 目录级声明 散文不入 allow；F6 lease 全套绿；F7 CONTRACT 修订管线过

## 簇M penface 批

批名 penface 查册 unknown，--new-stem 甲表认领：zh 写面修，派生 pen 即书简笔既立语素、face 即面既立语素，无既裁 code 形显式申报无承。正典输入：pk-091.json 与 pk-092.json 与 pk-093.json、writeface tools.py 74 与 89 行、errors.py 209 行、httpface.py path_outside_domain 位、runtime.py 97 至 103 行、critsweep detect_layout 先例。

- 件一 lease_commit MCP 透传补 trail 与 root 参数对齐 CLI，缺省发现语义零变，DES-014 修订随件
- 件二 valid_params 教学补齐：lease_open.intent 实为意图件路径、record_append.report 实为 JSON 报告文件路径，两错误路径载荷四字段喂满
- 件三 runtime 读面缺省分支分形：trail_path 按 layout_form 解析即 first-domain 与 canonical 两形，scribe_bin 解析 profile 无关即 debug 与 release 兜底序，域令牌绑定分支零变
- 件四 隐式占位会话归一：首笔写隐式绑定为既成事实则 open 与 close 同一绑定查找位，close 能收隐式会话，两处错误载荷教学互指隐式占位事实
- 件五 repo 校验误教改正：path_outside_domain 提示语改教合法形即点号列表与绝对域内路径
- F 锚定：F1 MCP lease_commit 带 trail 到 CLI argv 单测先红；F2 两错误路径 valid_params 非空且教学串钉死；F3 fixture canonical 域根无令牌场景 runtime 缺省读数命中域 trail 先红后绿；F4 隐式绑定连接 close 收得先红；F5 误教提示改正先红；F6 mcpline 全套绿基线 151

## 簇K trailhome 批

批名 trailhome 查册 unknown，--new-stem 甲表认领：zh 链册双居所，派生 trail 即链既立语素、home 即居所，无既裁 code 形显式申报无承。正典输入：pk-092.json、sih-tools/locks/src/locks/cli.py 27 至 30 行与 lease/cli.py 214 至 219 行双居所先例。

- 件一 locks _default_trails 改双居所并集与 lease 对齐即引擎新家加工具老家，迁链后新家链证入乐观锁基线
- F 锚定：F1 fixture 新家 trail 含哈希脏上游被拦先红；F2 老家 trail 零回归；F3 locks 全套绿

## 全簇约束

- 工地 lease worktree 主树零直写，worktree 根即仓内容根；identity 正身先行；批名 --new-stem 甲表认领走 nomsupply 后现行闸形即概念锚加派生申报裸认领拒；锁冲突 wait-turn 或 60 秒轮询至多 30 分钟
- TDD 先红后绿，红证认证在链；触 lease 的测试显式 --ledger 与 --locks 并 ROOTANCHOR_DISABLE_SELF_BOOT=1
- 文档产出化格、核阅、检词三步；核阅锚工作区根跑 sih-engine/target/debug/scrutinator --pack des-001，des-001 域只盖 sih-engine/doc，域外 rc2 如实记；检词命令 uv run --project sih-tools/nomenclator nomenclator check --pack sih-tools/nomenclator/packs/core
- settle 双仓加链 verify 加 reconcile 加主树重跑；CALL-LOG 追加；results 档落 sih-engine/sih/event/plan/<批名>-results.md
- 禁区：不出本簇请求写入范围；bypass 仅既有通道 reason 必填；工具 exit 2 停手如实报；不 push；不动他簇文件；三 pk 账出泊笔归主线不归簇；DES-016 外部只教不拒零变；first_domain 形零变
- 回执必含：批名、会话号、认证哈希、测试数、F 逐条、改动文件、未决项

## 请求写入

- 簇L：sih-tools/lease/src/lease、sih-tools/lease/tests、sih-tools/lease/CONTRACT.md、sih-tools/lease/CALL-LOG.md
- 簇M：sih-tools/mcpline/src/mcpline、sih-tools/mcpline/tests、sih-tools/mcpline/README.md、sih-tools/mcpline/AI-MANUAL.md、sih-tools/mcpline/CALL-LOG.md、sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md
- 簇K：sih-tools/locks、sih-tools/locks/CALL-LOG.md
- 主线：sih-engine/sih/state/plan、sih-engine/sih/event/plan、sih-engine/sih/state/parking/materials
- 公面随批写不入 allow：trail 与锁册与回执与绑定件

## 主线验收

三簇回执后 F 抽查实跑、三套件主树重跑、链 verify、reconcile、三 pk 账出泊笔、campaign results 档、commit 全部。mcpline 版本位升 0.11.0 归簇M，lease 升 1.45.0 归簇L。
