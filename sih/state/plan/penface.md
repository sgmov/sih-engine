# penface 批任务包：mcpline 写面修——MCP 写面五缺陷修复

## 一、使命 {#mission}

承 gatefix-parallel 并联批簇M（正典：sih-engine/sih/state/plan/gatefix-parallel.md），兑现 pk-091 与 pk-092 与 pk-093 三账出泊条件「用户令立批走引擎域纪律」中 mcpline 写面五件。批名 penface 查册 unknown，--new-stem 甲表认领；概念锚申报：zh 写面修，派生 pen 即书简笔既立语素加 face 即面既立语素（甲表机械承载形：stem 单段 penface 认领 new，语素语义由本锚与意图件承载），无既裁 code 形即显式申报无承。

五件：

- 件一 lease_commit MCP 透传补 trail 与 root 参数对齐 CLI，缺省发现语义零变，DES-014 修订随件
- 件二 valid_params 教学补齐：lease_open.intent 实为意图件路径、record_append.report 实为 JSON 报告文件路径，两错误路径载荷四字段喂满
- 件三 runtime 读面缺省分支分形：trail_path 按 layout_form 解析即 first-domain 与 canonical 两形（critsweep detect_layout 先例），scribe_bin 解析 profile 无关即 debug 与 release 兜底序，域令牌绑定分支零变
- 件四 隐式占位会话归一：首笔写隐式绑定为既成事实则 open 与 close 同一绑定查找位，close 能收隐式会话，两处错误载荷教学互指隐式占位事实
- 件五 httpface.py path_outside_domain 提示语改正：现教「域根相对形」而实际合法形是点号列表与绝对域内路径，改教真实合法形

F 锚定：F1 MCP lease_commit 带 trail 到 CLI argv 单测先红；F2 两错误路径 valid_params 非空且教学串钉死；F3 fixture canonical 域根无令牌场景 runtime 缺省读数命中域 trail 先红后绿；F4 隐式绑定连接 close 收得先红；F5 误教提示改正先红；F6 mcpline 全套绿基线 151。

## 请求写入 {#requested-writes}

- sih-tools/mcpline/src/mcpline/
- sih-tools/mcpline/tests/
- sih-tools/mcpline/README.md
- sih-tools/mcpline/AI-MANUAL.md
- sih-tools/mcpline/pyproject.toml
- sih-tools/mcpline/CALL-LOG.md
- sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md
- sih-engine/sih/state/plan/penface.md
- sih-engine/sih/event/plan/penface-results.md
- sih-engine/sih/event/plan/penface-materials/

## 约束 {#constraints}

- 工地 lease worktree 主树零直写，worktree 根即仓内容根；TDD 先红后绿，红证认证在链；触 lease 的测试显式 --ledger 与 --locks 并 ROOTANCHOR_DISABLE_SELF_BOOT=1
- 禁区：不碰 lease 树（在飞簇L）与 locks 树（在飞簇K）与 attnanchor 与 parser 文件；三 pk 账出泊笔归主线不归簇；外部面 stdio 19 具 HTTP 19 具计数零变（无新工具只加参数）；first_domain 形零变；DES-016 外部只教不拒零变；不 push；bypass 仅既有通道 reason 必填；工具 exit 2 停手如实报
- mcpline 版本位 0.10.0 升 0.11.0 双点位（pyproject.toml 与 __init__.py）
- 文档管线：化格与检词与核阅三步；DES-014 在 sih-engine/doc 域内核阅必须 rc0
