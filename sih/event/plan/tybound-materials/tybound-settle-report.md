# tybound 批结算报告

> 批：tybound（清账并联批簇D）· 会话 9d24fc5bc19dea64 · 工地 worktrees/sih-tools/tybound
> 令源：debtclear-parallel.md 簇D · 留债正典：judouwire-solo-results.md 行 34
> 日期：2026-09-11

## 一、交付总判

parser ty_bounds 深阶兑现：登记之深形语义（join_rules ty_bounds+ty for 全形取名）经源勘探已由 tokencap-solo 落源且 judouwire 探针实证在役，登记原文「现形单取 type 位」为 pre-tokencap 史态（mapping revision 2 name_from IDENT）。本批以源为准不硬造，兑现形转为全形家族深阶真实红修复加钉面补强加旧单形钉死加全套绿，四件俱落地。

## 二、改动清单（工地六件）

- parser/packs/rust/grammar.json：ty_bound 负前缀位 opt QUESTION 扩 choice[QUESTION, BANG]，负 bound 形 impl !Send for Foo 全形出条目
- parser/packs/markdown/tokens.json：增 BULSTAR 词法位 \*(?=[ \t]+[^*\t ])，插序 HR_* 之后 BPLUS 之前
- parser/packs/markdown/grammar.json：li 与 li_in 与 qitem 标记位扩 BULSTAR；五处 inner 二标记位 choice[BPLUS, BULSTAR]（保 - * x 形）；fenceline 与 qfct 选择面收编 BULSTAR
- parser/tests/test_tybound_deep.py：新测十法即 F1 负 bound 两形加深形钉三形加 F2 旧单形钉两形加 md 圆点三形
- parser/tests/test_tokencap.py：BASELINE 指向 rust-stream-baseline-2026-09-11.ndjson
- parser/tests/fixtures/rust-stream-baseline-2026-09-11.ndjson：过滤流基线刷新（329 行，双跑逐字节一致，sha256 62d83aeba02411b6f49da0ad55502745efa484bceae64228ab1af1f93dc8dffb）；pre-tokencap 旧夹具留档零动

## 三、F 判据

- F1 全形用例先红后绿：负 bound 形 impl !Send for Foo 首跑红（零条目带错误节点，tdd-red-first-run.log exit 1 在档）后文法修复转绿
- F2 旧单形零回归：inherent impl Point 与泛型 inherent Foo<T> 与 dash 圆点旧形单钉全绿
- F3 parser 全套绿：工地 94 测 12 子测全绿（批前 82 绿 2 存量红）

## 四、存量红根因处置（F3 支承）

- f2_xref 窄域对表红：leasepatch-solo-results.md `*` 圆点列表我方落 para 对 locator list_item 分叉，即 judouimpl 披露之 commonmark 语料外不齐面一类；md 包文法缺口修复转绿（活体对表基准零夹具改动）
- tokencap 过滤流基线红：引擎语料自然生长 85 条漂；基线刷新转绿。勘探一笔在案：条目 id 哈希料取 --in 参数原文，回落件 sample.rs 之 id 随解析路径形（绝对或相对）变，基线生成须与测试同款解析形，此 quirks 登记供后继批参考

## 五、序与偏差如实申报

- 意图笔后置于租约开约：stem 查册闸要任务包件在册而意图笔闸三要会话在册，先开约（9d24fc5bc19dea64，--new-stem 认领 new_coinage_acknowledged）后上意图（b2f74ae7/dd02797c），与簇A nomsupply 同款实录
- 任务包件 tybound.md 与意图记录与正身件与材料与 results 落公面（引擎 trail 与锁册与 state/plan 与 event/plan），不入 allow
