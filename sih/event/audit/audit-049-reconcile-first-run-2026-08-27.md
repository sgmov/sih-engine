# audit-049 对表首跑清点报告

对表时点 2026-08-27，工具 lease 1.1.1 reconcile，两仓全史三方对表即 git log 对会话台账对 trail。本报告是视图不是日志，原始五态明细在 scribe/reports/2026-08-27-reconfirst-两件。

## sih-engine main {#engine}

总量
: 126 笔

路由
: 4 笔即 d2b4a24 修正批段结算、c6dffcf 归位批段结算、fcb7c3b lease11 包档段结算、4110d45 closefix 包档段结算，全部经 lease commit 正身路径

存量
: 122 笔，全部为工具前提交。含 2026-08-26 晨间批次即 bc3c047 家族，其 message 带短形 session 行即八位，属工具前手动形态，按语义归存量不归绕行

首路由界
: d2b4a24 即 2026-08-26 修正批，此后增量共 4 笔全路由即零绕行

异常态
: session_orphan 零，cert_missing 零

## sih-tools integral-stage-build {#tools}

总量
: 320 笔

路由
: 6 笔即四笔副本归并 6b5855c 与 234d615 与 1386ba3 与 923e642、两笔正身段结算 76a77d2 与 207bce8

存量
: 314 笔，为工具线发展史。含白皮书批手工补归并 4adc597，其 message 为 feat 冒头非机械形态，如实归存量即当时正身路径未在役

首路由界
: 6b5855c 即 facet-stats 批副本归并，早于主线首路由界因工具线先接入租约副本流

异常态
: session_orphan 零，cert_missing 零

## 存量口径与裁定请求 {#ruling}

存量定义为工具前事实即各仓首路由界之前的全部提交，两仓合计 436 笔。工具在役后增量即 sih-engine 四笔与 sih-tools 两笔全部路由，零绕行零孤儿零认证缺失。

裁定请求一件：存量 436 笔以各仓首路由界为界封存为工具前事实，不追认不补写不改写，此后任何未路由提交即增量绕行按 DEC-012 裁决七露形处置。本裁定为人节点，裁定后本报告处置节随裁补记。
