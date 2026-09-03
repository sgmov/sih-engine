# mathrefmt-solo 结果档

> 批：mathrefmt-solo（数学仓重构格式归一波：calculus 概览优先旧格式）
> 承接：release-audit-2026-09-02 遗产披露第二条
> 队形：单线形 solo——确定性脚本加逐条改写亲写零子代理
> 开工实日：2026-09-03
> 会话号：157e28a3e21154b7
> 冲突模式：与他批（guardrail-solo / mathquote-solo）故意并发，撞锁有限重试上限十次

## 冲突样本节

guardrail-solo（会话 941409ce491354b4，2026-09-03T01:49:55 立约）持 trail 锁至今未释放，系故意并行批共享面冲突（dispatch 声明的链/台账/调用册共享预期）。本批十次上限内逐次重试，不绕行。

| # | 目标锁 | 持锁会话 | 持锁批 | 动作 |
|---|---|---|---|---|
| 1 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 1/10 撞锁 |
| 2 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 2/10 撞锁 |
| 3 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 3/10 撞锁 |
| 4 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 4/10 撞锁 |
| 5 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 5/10 撞锁 |
| 6 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 6/10 撞锁 |
| 7 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 7/10 撞锁 |
| 8 | sih-engine/sih/event/trail/2026-09-03.ndjson | 941409ce491354b4 | guardrail-solo | record 等待，重试 8/10 撞锁 |

## 枚举计数与本波件数

- 全仓盘上 calculus entries 计数：114 条（`ls` 与 `grep` 双法一致）
- 首二级节为概览旧格式条数：开工前 113/114（APP-011 本就定义首节，非全仓旧格式；完全枚举清单实测 113 条）
- 本波三前缀件数：LIM=8、DIFF=32、INT=22，合计=62（与 calculus INDEX 声明 LIM8/DIFF32/INT22 精确对表）
- 枚举清单：2026-09-03-overview-enum.txt（113 条全览）+ 2026-09-03-overview-enum-LIMDIFFINT.txt（62 条三前缀子集，可 grep 复算，F-1 对表索引计数）
- 切片边界申报（开工前概览首节余件留待后续波）：APP(10)/HIS(16)/MUL(10)/NS(3)/SER(5)/SPEC(7) 共 51 条【修正申报误差：此前记「全仓 130 / 余 52」，实为 114 总 / 余 51，详见越线与误差申报】

## 段落去向申报表

逐条改写语义：原「## 概览 {#overview}」块为纯六项目录导航（均为自指锚点，目标节全部原位保留），
原「## 议题」H2 改写为「## 定义 {#definition}」，其正文（原定义引入段）保持原位零改动。
段序逻辑：概览导航块整体移除（六行均为锚点无正文）→ 原话题节头改定义节头 → 正文全量保留。

| 段 | 去向 | 说明 |
|---|---|---|
| 概览六项目录导航（话题/历史脉络/数学表述/关系/应用映射/边界） | 移除 | 纯自指锚点导航，非正文，目标节全部原位存在零弃段 |
| 原「## 话题」H2 及其正文 | 改名为「## 定义 {#definition}」，正文原位 | 话题首段即定义引入，正文零改动 |
| 其余 H2（历史脉络/数学表述/关系/哲学桥接/应用映射/边界/触发问题等） | 原位保留 | 状态行、命题对照、关系节零改动 |

已验证信息零丢失（F-3）：62 条逐条 diff，原概览块全部为目录锚点行（正则全部匹配），话题节改定义后正文与「概览后全部其余内容」逐字节一致，零弃段。

## 三步管线读数

逐条三步（对工地文件，序固定 化格→核阅→检词）：

| 步骤 | 工具 | 规则包 | 批次 | 读数 |
|---|---|---|---|---|
| 化格 | formatter 0.2.0 | general-v1 | 62 条 | 62 条 0 改动（格式已合规，无需落笔），exit 0 |
| 核阅 | scrutinator 0.1.0 | des-001-mathe 0.3.0 | 62 条 | 62 条 0 findings 0 domain_mismatches，exit 0（S005 两态域内定义态通过） |
| 检词 | nomenclator 0.2.0 | core 0.9.0 | 62 条 | 62 条 domain 外 skipped（exclude **/llm-friendly-build/**），exit 0 零违例 |

块序归一核验（F-2）：改后 62 条首二级节 `## 定义 {#definition}`，S005 域内零违规。

## 认证清单

三步管线报告逐件 append 主树活链（会话 157e28a3e21154b7）：

| # | 认证对象 | 报告路径 | event_id | exit |
|---|---|---|---|---|
| 1 | 书简意图 intent | ask3 record/validation | 7860b638-fa50-4de9-94b3-dd295df520a5 | 0 |
| 2 | 核阅 scr/des-001-mathe | 2026-09-03-mathrefmt-solo-scr-des001-mathe.json | 4e5d40c8-35b1-4af9-a723-c9a6f68b06d0 | 0 |
| 3 | 化格 fmt/general-v1 | 2026-09-03-mathrefmt-solo-fmt-general.json | a0bdf87b-1461-4916-9c5e-6dd371751b4a | 0 |
| 4 | 检词 nomen/core | 2026-09-03-mathrefmt-solo-nomen-core.json | 8c46ea9a-be9a-4b29-b164-5e3e0cddb575 | 0 |

62 条 entry 与结果档随 settle git commit 留痕；认证先落主树活链，链 settle 前一次性拷工地（遵守防分叉纪律）。

## 三仓 commit 号

- sih-math：a4372c1（62 条 entry 概览归一）
- sih-engine：bbbd09b（plan/results/materials/trail）
- sih-tools：44c19d86（管线报告三件 + CALL-LOG）

## 链 verify 对表 / reconcile 读数 / F 表 / 越线与误差申报

### 链 verify

`target/debug/scribe verify --trail sih/event/trail/2026-09-03.ndjson` → status `valid`，
events=101，first_hash `94f1dd00…503512`，last_hash `72ab6a43…186bb`，链完整首尾衔接，exit 0。

### reconcile 读数（租约收约）

会话 157e28a3e21154b7 全部锁 12 acquired / 12 released 对称闭环，无滞留锁；guardrail-solo
（941409ce491354b4）trail 锁 2026-09-03 交回 released=0 于十次重试内放行（见冲突样本节）。S005 豁免态
申报外：本批三仓 merge 均以备份让位归并对表法闭合，非 identical 即停批，对表一致未触发停批。

### F 表（验收判据核对）

| 判据 | 断言 | 读数 | 结论 |
|---|---|---|---|
| F-1 枚举零漏 | 三前缀清单与 INDEX 对表 | LIM 8 / DIFF 32 / INT 22 = 62，与 INDEX 声明精确一致；枚举子集 62 条逐条可 grep 复算 | 通过 |
| F-2 块序归一 | 改后首二级节 ## 定义 {#definition} | 本波 62 条全部 `## 定义 {#definition}`；仓内定义首节 63 = 62 + APP-011(批外既有) | 通过 |
| F-3 信息零丢失 | 62 条 diff 逐字节比对 | 概览块全为目录锚点行、正文零改动，无弃段 | 通过 |
| 不触碰 | INDEX 计数 / VERSION 基线零触碰 | INDEX 未入 batch commit，VERSION 未改 | 通过 |

### 越线与误差申报

1. **枚举计数申报误差（本批发现，已修正）**：结果档与枚举头注释原记「全仓 130 条 / 首二节均概览 / 余 52 条」。
   收约后按盘上真相复核：全仓 **114 条**（`ls` 与 grep 双法一致），概览首节 **113**、定义首节 1
   （APP-011 批外既有，非全仓旧格式），本波归 62 后余概览首节 **51** 条。130 系旧申报偏高，属实不为
   盘面真相。误差不影响本波变换：62 条三前缀逐件与 INDEX 及 commit a4372c1 双向核对一致，F-1/F-2/F-3
   全部成立。误差定性为枚举汇总口径错报（把预留/申报数混入盘面计数），非变换缺漏，已在枚举节改记
   114/51 并此申报。
2. **APP-011 界定**：APP 共 11 条，但 APP-011 本就定义首节（不在概览旧格式集），故留待后续波的概览
   首节为 APP(10) 非 APP(11)。旧申报把 APP-011 误计入遗留数，与第 1 条同比修正。
3. 无变更型越界：batch commit a4372c1 仅 62 件（32 DIFF/22 INT/8 LIM），零波及 APP/HIS/MUL/NS/SER/SPEC；
   INDEX/VERSION 零触碰；S005 两态域内定义态通过，未触碰豁免域。

> 收约后回填完成。
