# tybound 批任务包：句读 ty_bounds 深阶兑现——全形约束负 bound 真红修复与深形钉面与旧单形钉死与全套绿

## 一、使命 {#mission}

承清账并联批 debtclear-parallel 簇D（正典：sih-engine/sih/state/plan/debtclear-parallel.md），兑现 judouwire-solo-results.md 行 34 留债登记：「句读取 ty_bounds for ty 全形（如 "T for Foo"），旧绑定单取 type 位（"Foo"）——实现批已登记待深阶」。批名 tybound 查册 unknown，--new-stem 认领；概念锚申报：zh 全形约束，派生 ty 即 ty_bounds 域词既立语素加 bound 即界，无既裁 code 形即显式申报无承。

以源为准勘探申报（不硬造令在案）：登记之「现形单取 type 位」为 pre-tokencap 史态（mapping revision 2 name_from IDENT，泛参先取与单取 type 位皆此形）；深形 join_rules（ty_bounds+ty，sep " for "，direct）已由 tokencap-solo 落源（revision 3）且三测钉死、judouwire 探针 15 impl 全形实证。本批兑现形：全形家族深阶红修复与钉面补强与旧单形钉死。

- 件一 F1 全形用例先红后绿：负 bound 形 `impl !Send for Foo {}` 全形现零条目带错误节点（深形家族真实红），md 无涉属 rust 文法数据缺口，修复后全形条目出且名含 bound 位；红证留档先红留痕纪律
- 件二 深形钉面补强：多段路径 bound（std::fmt::Display for X）与 where 从句不染名与泛参不领先（judouimpl 登记原案）三钉补入测试
- 件三 F2 旧单形零回归：inherent impl 名 type 位（Point）与泛型 inherent（Foo&lt;T&gt;）既有钉零变
- 件四 F3 parser 全套绿：两存量红随批根因处置——markdown 包 `*` 圆点列表文法缺口（li 标记位仅 BULLET，leasepatch-solo-results.md 语料实证 para 对 list_item 分叉）与 tokencap rust 流基线语料生长陈旧（85 条新增漂）；金向量再冻结双跑逐字节一致

F 锚定：F1 负 bound 全形用例先红后绿；F2 旧单形用例零回归；F3 parser 全套绿。

## 请求写入 {#requested-writes}

- sih-tools/parser/src/parser/
- sih-tools/parser/packs/
- sih-tools/parser/tests/
- sih-tools/parser/CALL-LOG.md
