# facetmath-solo：facet 判据切换收口（mathpipe 程序批二落地件）

- 承接：mathpipe-full-program-v1 批二（facet 判定数学化）；用户 2026-09-04 裁定原话「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——判据切换命题过得一裁三态机械分流，不向用户呈裁。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-facetmath）｜ 温故检索：materials/recall-facetmath.json 零命中如实记 ｜ pk-045 参与者（并行批 covrefresh/m3clear 同跑）
- 队形：单线形 solo，零子代理。

## 背景（线索，实跑为准）

- facet_stats_inf.py 已有检验判据函数族（boundary_low_by_test，α 显式给参，0.34 重定义为翻桌率基线 DEFAULT_BOUNDARY_BASELINE，源内 A2 标记）；probes/r3a_gate_v2.py 为切换探针先例（仅替换 boundary_low 判据位、α 显式给参不读钟）。
- 生产判定路径（facet_stats.py 与 engine.py 判定位）仍用经验占比阈值 0.34，切换未落——本批即收口。
- 程序批二验收硬约束（照录）：显著性水平 α 注册为模型参数（正确性由数学检验承载）；**判变申报：切换若改变任何既有裁决结果，逐件申报，不静默改判**。

## F 清单

- F-1 生产切换：facet 生产判定路径 boundary 判据位切换为 boundary_low_by_test（α 显式给参缺省 0.05，来源注记统计惯例承 DEFAULT 注记先例），占比阈值位退役为基线参数不再作判据；diff 亲读入结果档。
- F-2 判变审计零静默：对在档可复算 facet 裁决材料（facet/facet_task_packages/ 与 proposition/DES/ 下 responses+score 可复算件）双判据对跑，逐件比对 verdict；判变件逐件列表，**不改写任何在档 verdict**，判变清单入泊界登记（scribe park 一笔加泊材料件，号从链取）——历史裁决在档不动，切换只对新测量生效。
- F-3 得一裁：命题「facet boundary 判据由占比阈值切换为二项检验判据（α=0.05）」走 facet 测量九发三态：stable_clear → 切换在役收口；boundary → 切换挂起进泊界（生产路径保持旧判据，F-1 改动回退或门控关闭），泊界登记。三态机械分流，零人工呈裁。
- F-4 金向量：切换后判据位 fixture 双跑逐字节一致（复用 r3a 探针形定格）。
- F-5 测试：facet 测试族全绿零回归 + 新增判据切换测试先红后绿（旧判据测试改型如实申报）。
- F-6 推导档：sih-math/docs/facetmath-derivation-2026-09-04.md（二项检验判据语义、α 语义、与占比基线关系、翻桌率基线保留逻辑），承载概念从 facet_stats_inf 源内标记核实（PROB 族在仓命中为准，零命中显式申报不硬挂）。
- F-7 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package facetmath-solo → 取锁（施工面 sih-tools/facet/ exclusive 长持；共享追加面 append 短持）→ meter 包裹引擎 scribe intent 上链 → 工地施工（主树零直写）→ 化格→核阅→检词 → 认证逐笔 append → settle 前一次性拷工地 → 双仓 settle --cert → 放锁 → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/facet/src/（facet_stats.py、engine.py 判据位、facet_stats_inf.py 参数注记）与 facet/tests/ 与 facet/probes/
- sih-math/docs/facetmath-derivation-2026-09-04.md
- sih-engine/sih/state/parking/materials/ 判变登记件（如有）与泊界 scribe park 一笔（如有）
- facet 得一材料落命题区（facet/contracts/ 本批命题目录）
- sih-engine/sih/event/plan/facetmath-solo-materials/ 与 facetmath-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

历史 verdict 材料零改写（F-2 硬约束）；不碰 sih-math/docs/mathpipe-coverage-*（covrefresh 面）与 m3clear 命题区件；主树零直写。
