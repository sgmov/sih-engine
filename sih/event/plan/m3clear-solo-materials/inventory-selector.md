# 盘点档：selector（路择）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）；线索：疑已随后续批有载体（ALG-002 第二消费位）

## 机制职责一句话

谓词路由：逐材料按包内谓词声明顺序机械求值，首败定路、全过走 pass_route，产出主线/停放/丢弃三路归位与逐谓词过败记录（selector/src/selector/route.py:18-45 route_material）。

## 线索核验（源码引用位实查）

**已坐实有载体。** selwire-solo 批（2026-09-04）接线 ALG-002 两处载体引用位（本批 2026-09-04 grep 实查）：

| 引用位 | 位置 | 承接语义 |
|---|---|---|
| 谓词装载位 | selector/src/selector/pack.py:78-79 | ALG-002 等价关系与商集隔离（sih-math/algebra/entries/ALG-002-equivalence-relation-and-quotient-isolation.md，mapping.md:197） |
| 逐件判定位与三路归位位 | selector/src/selector/route.py:24-30 | 逐谓词机械求值即等价类归类判定；首败定路与全过走 pass_route 即商集三分块归位，route 即商映射像，failed_predicate 即类边界谓词；推导档 sih-math/docs/selwire-selector-derivation-2026-09-04.md |

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 首败定路 | selector/src/selector/route.py:41-44 `if failed_predicate is None and not passed` | 首个失败谓词决定改路 | 判定性（已承 ALG-002 类边界谓词） |
| 全过走 pass_route | selector/src/selector/route.py:40 `route = pack.pass_route` | 缺省路归位 | 判定性（已承 ALG-002 商集分块） |
| 谓词装载 | selector/src/selector/pack.py:78 注释锚定位 | 包谓词族装载 | 判定性（已承 ALG-002） |
| 参照时间 | route 谓词包 reference-time 参数（时间谓词专用显式给参） | 时间谓词求值参照 | 资源性（显式给参不读钟，环境参数形） |

## 既有归属判

判定性。三判据全属判定性且已由 selwire 批接线 ALG-002 载体；参照时间是显式给参的环境形（泊界心跳调用面在用），不改判定逻辑归资源性备注。

## 载体候选分析

- 线索核实结论：rev1「无可指认载体」已被 selwire-solo 批（2026-09-04）清账——ALG-002 等价关系与商集隔离（mapping.md:197，已建实存）两注释锚点位接线在源码，推导档在 sih-math/docs/selwire-selector-derivation-2026-09-04.md，金向量八场景冻结（引擎侧 fixtures）双跑 IDENTICAL 在案。
- 本批零补施工：账清，无后续接线批输入。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 selector；summary-rev1.md:31/36 对挂核验 selector 零命中如实记（账本时点读数，被 selwire 批推进）。
- 消费面：sih-math/llm-friendly-build/mapping.md:197 ALG-002 状态已建。
