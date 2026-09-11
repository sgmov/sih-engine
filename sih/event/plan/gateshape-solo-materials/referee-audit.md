# 裁判面查己清单（gateshape 批 F4）

> 判据参数面 = 能改变工具判定结果的参数、常数、环境阀与判据包载体。封印三件套判据：持锁验身（修改通道绑定会话与身份）、显式登记（谁、何时、为何、新旧值在册）、落链留痕（修改事件在 trail 可查）。扫描面：sih-tools 全子工具。边界：sih-engine 引擎件（scribe、scrutinator、retriever）不在本清单，引擎侧自查候后继批。
> 日期：2026-09-11；批：gateshape（会话 1bcd413b6d914b4f）

## 清单

| # | 工具 | 判据面 | 形态 | 封印三件套现状 | 判档 | 建议 |
|---|---|---|---|---|---|---|
| 1 | critsweep | `--threshold` 沉底窗口（缺省三） | CLI 参数 | 非缺省强制 `--threshold-reason` 事由，登记 ledger/referee.ndjson，失败降级可见（refseal-solo 批）；本批活体验收：无事由传 7 即拒，教学 JSON 齐全 | **sealed**（正面标本） | — |
| 2 | gauge | `--union-threshold` 承重件数 N（union_count ≥ N 即过，GD_UNION_THRESHOLD_DEFAULT=5，constclear2c 登记行在案） | CLI 参数覆写 | 缺省值有登记出处，**覆写零事由零登记** | **unsealed**（活体标本一） | 照 refseal 形补 `--union-threshold-reason` 与 ledger 登记（候后继批） |
| 3 | lease | `ROOTANCHOR_DISABLE_SELF_BOOT=1` 自举硬拒闸全 bypass | 环境阀 | 触发即静默过闸，零登记零留痕（lease core.py:217） | **unsealed**（活体标本二） | env 触发时回执显式 bypass 行并落登记（候后继批） |
| 4 | lease | `--bypass` 守卫绕行 | CLI 动作 | bypass.ndjson 强制 reason+sha+时间戳，本批开约前后即有在册用例 | **sealed** | — |
| 5 | lease | `reconcile --limit` 对账尾窗（零即不限） | CLI 参数 | 限窗收缩审计覆盖面，读数不申报覆盖度 | 疑似 | reconcile 输出增 coverage 申报字段（候后继批） |
| 6 | tally | BUDGET_PER_GID、R1-R7 判据族 | 源码冻结常数 | 变更即改源码，经租约锁面与链笔（by construction） | **sealed** | 防缩水断言候补：规则数下限断言（jsonl-db golden 防删除断言同形） |
| 7 | nomenclator、selector | `--pack` 判据包载体 | CLI 路径参数 | 正典包 git 版控；任意外包路径零警示（自带判据即自带裁判） | 疑似 | 非正典包路径时输出 pack provenance 警示行（候后继批） |
| 8 | mcpline | 面清单常数（EXPECTED_TOOLS）与 SIH_ROOT、SIH_MCPLINE_CODE_ROOT | 源码常数+env 结构参 | 输出自申报 root 与面清单字段 | sealed-by-design（结构输入非判据） | — |
| 9 | identity | SIH_SANDBOX_ID、SIH_SESSION_ID | env 组件输入 | claims/verify 声明对表机制在 | sealed-by-design | — |
| 10 | tally | R9 证据登记闸（本批 F2 增） | 材料输入字段 | 条件触发、锚材料哈希绑定链（R2 复算） | sealed（边界：evidence 数组本身系材料自报，独立锚定候后继） | evidence 项逐条哈希锚定候后继批 |

## 结论

- sealed 六件：critsweep 阈值（refseal-solo）、lease bypass、tally 常数族、mcpline 与 identity 结构输入、R9（本批新增）
- unsealed 两件活体标本：gauge union-threshold、ROOTANCHOR 环境阀
- 疑似两件：reconcile --limit 覆盖收缩、pack 任意外包
- 后继批候令（可合一批「refseal2」）：gauge 阈值照 refseal 形封印、ROOTANCHOR env 告警与登记、reconcile coverage 字段、pack provenance 行、R9 evidence 逐条哈希锚定

## 方法附注

- 扫描命令：argparse 判据参 grep（threshold、budget、max、limit、tolerance）加 os.environ 全量对表加冻结常数清单
- 上游：zcode workflow 三测试包拆解（2026-09-11 主会话）——jsonl-db 包 JQL_UPDATE_GOLDENS 逃逸阀零登记即本清单的外部对形；对方病我们自家一样有，查己即本清单职能
