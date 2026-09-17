# trailhome 批结果档

> gatefix-parallel 并联批簇K执行代理落档，2026-09-11。令源 gatefix-parallel.md 簇K行与 pk-092 出泊条件，用户 2026-09-11 令「多子代理全量修复」。批机械链全序承 sih-tools/BATCH-FACE.md。

## 一、批身份与链证 {#identity}

- 批名：trailhome，查册 unknown，--new-stem 甲表认领；概念锚申报 zh 链册双居所，既裁 code 形显式申报无承，甲表机械形派生 trailhome:new（闸按 [-_.] 分段整词单段），语素级派生申报 trail 即链既立语素加 home 即居所以意图件与本档双层承载（nomsupply 先例同形）
- 会话号：4b7dd5c8f1c66849（lease 1.44.0 签发）
- 意图笔：9c8f99498ee8525a 在链（plain 形 validation 豁免，DES-016）
- 认证两笔在链（2026-09-11 trail）：518cfe30（tdd-red-green-cert 封装）与 b1aa9de0（suites-report）
- settle 提交：tools 仓 30678798（cert b1aa9de0，base integral-stage-build@0f0c0b21）；engine 仓 e4bbeec
- reconcile 双零：tools unrouted 0 加 cert_missing 0；engine unrouted 0 加 cert_missing 0
- 链 verify：2026-09-11 trail 全链 valid（47 events）

## 二、F1–F3 逐条实跑 {#f-results}

- F1 fixture 新家 trail 含哈希时脏上游被拦且先红：过。test_default_trails_engine_new_home_dirty_blocked 实跑，红段即单居所旧形对仅新家认证的上游判 writable 漏拦（payload 无 error 键，KeyError 断言红），修复后 blocked 加 dirty 即 decision/DEC-007-layer.md；夹具形即老家播他件认证（OTHER-999）保旧形可跑、目标上游仅新家链认证，精确复现 pk-092 fail-open 漏拦语义
- F2 老家 trail 零回归：过。test_default_trails_legacy_home_no_regression 实跑，老家 sih-tools/scribe/trail 链证照旧入缺省基线，上游脏即 dirty_upstream 拦，修复前后俱绿
- F3 locks 全套绿：过。工地铁 12 passed（批前基线 10，净增 F1 与 F2 两测）；主树批前基线真跑 10 passed 零回归，归并后主树重跑 12 passed 见第六节

## 三、一件落形 {#item}

- sih-tools/locks/src/locks/cli.py _default_trails 单居所（只枚举 sih-tools/scribe/trail）改双居所并集即引擎新家 sih-engine/sih/event/trail 加工具老家，与 sih-tools/lease/src/lease/cli.py 214 至 228 行先例逐形对齐（is_dir 跳过缺席居所加 resolve 去重加 sorted 全序）；病灶闭合即迁链后新家链证入乐观锁基线，check 缺省零参形对新认证脏上游由 writable 漏拦改 blocked 拦截；显式 --trail 形与老家链证形与 unknown_target 注记形零变
- 版本位随批升：locks 0.1.0 升 0.2.0 双点位（pyproject.toml 加 __init__.py）
- CONTRACT.md 修订三：缺省链枚举双居所语义与判变边界在档

## 四、套件数字 {#suites}

- locks 工地铁 12 passed 0 failed（批前基线 10 passed，净增两测全绿）
- locks 主树批前基线真跑 10 passed 0 failed（零回归），归并后主树真跑 12 passed 0 failed（第六节）

## 五、改动文件清单 {#changed-files}

- sih-tools/locks：src/locks/cli.py 加 tests/test_locks.py 加 pyproject.toml 加 src/locks/__init__.py 加 CONTRACT.md
- sih-engine：sih/event/plan/trailhome-materials/ 三件（tdd-red-green.log 加 tdd-red-green-cert.json 加 suites-report.json）；sih/state/plan/trailhome.md 批任务包（开约前置件，包名解析必需，nomsupply 先例同形）；本结果档
- CALL-LOG 两笔：locks 加 scribe（走 lease call-log append 三腿，直改车道）

## 六、误差申报与未决项 {#deviations}

- 首跑红证覆盖一笔如实申报：首跑红（夹具 v1，rc2 形 cascade check errored 即旧形单居所老家缺席 trails 空）被夹具 v2 红跑以覆盖形写 log，v2 红形（writable 漏拦，即 pk-092 病灶精确形）与绿段在 tdd-red-green.log 在档，封装认证件 red_first_note 申报覆盖事实，不清洗不虚报，候人节点裁
- 甲表机械形与语素级申报双层：stem 闸按 [-_.] 分段整词单段，甲表机械形 trailhome:new 过闸入回执，语素级派生（trail 即链既立语素加 home 即居所）以意图件与本档承载
- 批任务包 self-serve 一笔：簇K任务书内嵌于 gatefix-parallel.md 而裸 stem 开约要求包文件在盘，trailhome.md 照 nomsupply 先例自落 state/plan 并列入本批改动清单，与请求写入节主线面交叠如实申报
- results 档与本包文件主树直写（收约后交付件形）：不在分支提交面，零撞归并，候主线 campaign commit
- suites-report.json 材料件 main_tree 字段写 pending-settle-rerun（落盘于归并前），主树终数 12 passed 由本档第六节与本回执承载，材料件随支已归并不改写
- 未决项：批词汇（trailhome 加链册双居所）候立名程序人节点终裁登记，本批零写 terms.json；locks 版本位 0.2.0 归并生效候主线；pk-092 出泊笔归主线不归簇
