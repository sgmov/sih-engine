# gauge-probe 批任务包：sihmcp 写面与管理台全量实测探针（测试痕候人节点签署回撤）

> 线：MCP 线（rmcp 载体 sihmcp 现役，8765 HTTP 面与 stdio 面双通道）
> 令源：用户 2026-09-12 令「都要测，测试数据可以留痕，并且通过人节点签署回撤。」
> 形：solo 探针批，stdio 正面全链加负面教学加 HTTP 写面与台面与负面教学，测试痕留链，回撤候人节点签署

## 一、使命

sihmcp 载体换防后的写面实战受试：beta 十写具经 MCP 双通道（stdio 与 streamable HTTP）真实调用一轮，台面三路由（令牌签发停行与开域视图）真实调用一轮，负面教学形（未绑写拒与已绑重开拒与无令牌写拒与 readonly 档拒）如实取证。测试批零实装零规格物变更：所有写痕为测试数据，批名两段 gauge（秤星）与 probe（探针）俱 established 立名段。

## 二、规格消费面申报

本批零实装零代码变更，规格消费面即既有正典只读对表：DES-014（MCP 外部缺省分级）、DES-015（HTTP 面承载）、SPEC-023（MCP 线契约）、lease CONTRACT 修订六十二（SDDG 两道门与 close 链闸）、DEC-024（SDDG 判据正典）。判据受试面：本批 close 即 SDDG 门 GATE_SINCE 后测试批受试样例（零实装批的判据通路）。

## 请求写入

- sih-engine/sih/event/trail/2026-09-12.ndjson
- sih-engine/sih/state/plan/gauge-probe.md
- sih-engine/sih/event/plan/gauge-probe/
- sih-engine/tests/
- worktrees/sih-engine/gauge-probe/
- sih-tools/scribe/reports/
- work/

## 三、验收判据（可证伪）

- G-1 stdio 正面全链八步：open 加 lock 加 intent 加 append 加 commit wip 加 commit settle 加 unlock 加 close 俱退出码零，SDDG 四判据 pass
- G-2 stdio 负面教学：未绑写拒（session_not_bound）与已绑重开拒（session_already_bound）取证
- G-3 HTTP 正面：bearer 握手加 open 加 claim 加 park 加 append 加 close 俱退出码零
- G-4 HTTP 负面：无令牌写拒与 readonly 档拒取证
- G-5 台面：签发两步与停行两步与开域视图两步俱成，令牌台账行与镜像行六字段恒等
- G-6 链 verify valid 且锁读数净态

## 四、回撤位

测试痕清单与回撤通道候人节点签署：链笔（trail 2026-09-12 探针批各笔）与台账行（sessions 加 locks 加 claims 加 tokens 登记册探针行）与归并件（sih-engine 主树探针批归并件）与 /tmp 测试域根。pk-102 泊位承载回撤候裁清单。

## 五、明确不做

- beta 写具语义变更零（透传实测）
- unclaim 与 record_direct 与 takeover 与 bypass 零调用（外部分级不在矩阵行，设计拒）
- mcpline 退役零触碰（候另令）
