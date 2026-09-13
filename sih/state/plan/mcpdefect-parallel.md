# mcpdefect-parallel 任务包（并联形）

## 形声明与令源

- 队形：并联形（parallel），主会立约取锁，并行簇双子代理后台取材，主线亲写修复件，依赖簇串行验收。
- 令源：用户 2026-09-13「并行开工」；前文裁定 MCP 缺陷清扫批承 pk-091 至 pk-100 出泊，解司梦域接入拖慢。温故检索先例：debtclear-parallel-results.md（pk-091/092/093 三账合组候批，锁已让出）与 bootstrap-solo-results.md（mcpboot 主体已在役，本批不重做）。

## 目标

pk-091 至 pk-096 加 pk-099 出泊对账修缮：逐件判词（活缺陷／已核销／域归属），活缺陷红转绿修复，测试零新增红。pk-097 与 pk-098 与 pk-100 对账申报：腿二（81872a0）已核销者出泊核销，引擎 Rust 深面者候腿三批，本批零触碰。

## 范围

- 修缮域：sih-tools/mcpline（writeface、runtime、server、httpface）与 sih-tools/nomenclator（check 锚根，pk-096）。
- 对账域：pk-097（commit repo 形参与 message 前缀重复）、pk-098（close 死锁三角与孤儿锁）、pk-100（pre-commit 守卫锁面盲区），只判词不动码。

## 验收判据（可证伪）

- F1 对表留档：pk-091 至 pk-100 十件逐件判词（活／核销／候腿三）与 file:line 证据入 materials。
- F2 活缺陷红转绿：F1 判活的缺陷逐件先红测后修，红证与绿证入档。
- F3 测试零新增红：mcpline pytest 终态相对开工基线零新增红（基线由取材簇捕获在档）。
- F4 检词与链面：当日 trail verify valid；结果档走 T6 三步（化格、核阅、检词）。
- F5 泊位收口：修缮完的 pk 件出泊停泊笔落链，候腿三件如实留泊并注明。

## 明确不做

- 围堰 sih-tools/lease 源码零改动（融回冻结红线延续，pk-097/098 修法候腿三或切换批）。
- reinit／重开域候裁维持；管理台签发与撤销既有动作零变更。
- mcpline 版本升位让位后批（承 bootstrap-solo 协调申报）。
- mcpboot 计划稿零施工：bootstrap 五段链与台面 confirm-open 路由经前批在役，本批结果档如实申报此对表结论即闭项。

## 改动文件清单

sih-tools/mcpline/src/mcpline/（writeface、runtime、server、httpface 逐病灶）、sih-tools/mcpline/tests/、sih-tools/nomenclator/src/ 与 tests/、本包、结果档、materials、当日 trail。
