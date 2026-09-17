# tybound 批结果档

> 承接：debtclear-parallel 簇D任务包、judouwire-solo-results.md 行 34 留债登记、意图 dd02797c 与 b92a648e、会话 9d24fc5bc19dea64 让位于 e3b8e8db5a830d7c
> 令源：用户 2026-09-11 令「拉起子代理逐项清账，可以并行」
> 队形：并联四簇之 D 簇，委外代理亲写零子代理
> 日期：2026-09-11

## 一、总判 {#verdict}

ty_bounds 深阶债兑现。以源为准勘探申报（不硬造令在案）：登记原文「句读取 ty_bounds for ty 全形，旧绑定单取 type 位——实现批已登记待深阶」中，深形取名（join_rules ty_bounds+ty，sep " for "，direct）已由 tokencap-solo 落源（mapping revision 3）且三测钉死、judouwire 探针 15 impl 全形实证在役；「现形单取 type 位」为 pre-tokencap 史态（revision 2 name_from IDENT，judouimpl 披露之泛参先取同源）。本批兑现形转为：全形家族真实红修复与深形钉面补强与旧单形钉死与全套绿，四件俱落地，六件入主树，归并 0b42be70 经 merge 69044b22，链 verify valid。

## 二、先红留痕 {#red}

首跑红证在档（tybound-materials/tdd-red-first-run.log，认证 24eb8c12）：

- 负 bound 全形红：`impl !Send for Foo {}` 零条目带错误节点——ty_bound 无 `!` 前缀位，全形落错误恢复，连旧单形名都不可得
- 负 bound 于 where 从句红：`where T: !Send` 错误节点
- md 圆点红：`* ` 列表我方落 para，对 locator list_item 分叉（leasepatch-solo-results.md 语料实录，即 judouimpl 披露之 commonmark 语料外不齐面一类）
- 钉面七绿先行为设计内（多段路径 bound 与 where 不染名与泛参不领先与旧单形两钉与 dash 圆点）

## 三、绿态与 F 判据 {#green}

- F1 全形用例先红后绿：rust 文法 ty_bound 负前缀位 opt QUESTION 扩 choice[QUESTION, BANG]，负 bound 全形出条目且名含 bound 位（`!Send for Foo`），where 从句负 bound 不染名，红转绿
- F2 旧单形零回归：inherent impl `Point` 与泛型 inherent `Foo<T>` 与 dash 圆点旧形全绿，tokencap 既有三钉零动零破
- F3 parser 全套绿：工地 94 测 12 子测全绿（批前主树 82 绿 2 存量红，本批 +10 测），主树重跑同读数（F-1 主树真跑，maintree-rerun.log 认证在材料）

深形钉面补强三钉（多段路径 bound `std::fmt::Display for GlobError`、where 从句不染名 `Trait for Foo<T>`、泛参不领先 `From<T> for Foo`，末者即 judouimpl 登记原案）。

## 四、两存量红根因处置（F3 支承） {#legacy-reds}

- f2_xref 窄域对表红（活体基准非陈旧）：md 包 `*` 圆点列表文法缺口。修：tokens 增 BULSTAR 词法位 `\*(?=[ \t]+[^*\t ])`（插序 HR_* 之后 BPLUS 之前，保 `* * *` 走 hr）；grammar li 与 li_in 与 qitem 标记位扩 BULSTAR、五处 inner 二标记位 choice[BPLUS, BULSTAR]（保 `- * x` 形）、fenceline 与 qfct 选择面收编。包 lint 三件零违例
- tokencap 过滤流基线红（语料自然生长 85 条漂）：基线刷新 rust-stream-baseline-2026-09-11.ndjson（329 行，双跑逐字节一致，认证 aa359eae）；pre-tokencap 旧夹具留档零动作史证。勘探一笔：条目 id 哈希料取 `--in` 参数原文，回落件 sample.rs 之 id 随解析路径形（绝对或相对）变——基线生成须与测试同款相对回落形，quirks 登记供后继批

## 五、序与偏差如实申报 {#deviation}

- 意图笔后置于租约开约：stem 查册闸要任务包件在册（state/plan/tybound.md 本批自立，同簇A/B先例）而意图笔闸三要会话在册，先开约后上意图，簇A nomsupply 同款实录
- allow 形勘误重开一笔：首开 --allow "parser/"（repo 相对）与直提守卫工作区相对 "sih-tools/parser" 不匹，commit 前守卫拒、换锁 scope_violation；处置循既有通道——改件备份、工地还原、空会话 9d24fc5bc19dea64 收约（bypass-orphan 让位四件批前在盘无主件，事由留痕 bypass 台账）、重开 e3b8e8db5a830d7c（allow 自包文件逐路径分行解析五路）、意图 r2（b92a648e）、改件逐字节复归、复验 94 绿、settle 认证 fcb6873e 后 commit。红绿证与基线双跑认证系前会话所出（24eb8c12 与 1b93d470 与 aa359eae），会话吊销不撤链笔，两会话轨迹全在台账
- 无主四件（facet/probes 两件与 lease/tests/frozen 金向量与 mcpline/ledger/tokens.ndjson）mtime 均 2026-09-10 早于本批，批前在盘非本批产物零触碰，候人节点裁（watchcheck 协议）
- results 档与材料落主树引擎域（公面），本档自身未经链认证（会话已收，认证窗已闭），读数以材料与回执为准

## 六、段结算 {#settle}

tools 主树 69044b22（0b42be70 归并）：六件即 rust/grammar.json 与 markdown/grammar.json 与 markdown/tokens.json 与 test_tybound_deep.py（新）与 test_tokencap.py 与 rust-stream-baseline-2026-09-11.ndjson（新）。reconcile unrouted 0 与 cert_missing 0；链 verify valid 42 笔末笔 fcb6873e。CALL-LOG 三腿齐落（lease call-log append，会话 e3b8e8db5a830d7c）。材料八件落 tybound-materials：红绿日志与基线双跑与主树重跑与三 JSON 认证件与结算报告。批名 tybound 经 --new-stem 认领在册。
