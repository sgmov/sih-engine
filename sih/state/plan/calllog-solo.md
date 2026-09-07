# calllog-solo：调用留痕册转行式账本双轨

> 令源：用户 2026-09-07 裁「CALL-LOG 转 sqlite 双写」加「markdown 也改 jsonl 和链一样」加令「得一裁」；判定语义三件经 m-calllog-dual-1 九发 stable_clear 执契机器终签 cdf3252f 在链（crosscheck-m-calllog-dual-1），机器签署即为人授权
> 范式：T6 单线 solo，委外代理亲写零子代理
> 前置：vecfix-solo 在飞时照规矩排队候叫；本批 allow 面与 vecfix 持锁面（sih-tools/basemgr、sih-tools/checker/tests、scribe/CALL-LOG.md 等）重叠即 wait-turn

## 一、问题陈述 {#problem}

- **问题 1**：19 工具调用留痕册 CALL-LOG.md 为共享 markdown 表格，agent 徒手追加行；租约 auto 缺省下落独占锁（locksplit 契约已名共享追加面但 leaseup auto 白名单遗漏归队），scribe 册独占 acquired 140 次为全场单文件最热，并行批结构性串行。
- **问题 2**：markdown 表格 schema 已漂（lease 册同册两表头摞立、跨工具异构），机器不可核。
- **问题 3**：写入由 LLM 徒手执行，违背基线一精神（写入动作应归确定性程序）。

## 二、关键设计 {#design}

### 2.1 三腿形（P1 已裁成立）

单一写点命令（lease 子命令族 `call-log`）内一个 flock 临界区写三处：

1. **行式账本** `sih-tools/calllog/calls.ndjson`：单册统管带 tool 列，git 版控，**权威腿**；行写入走 ledgerwrite 同款 flock + O_APPEND 原子整行（复用 pk057fix 写点机制，禁任何整文件重写路径）。
2. **sqlite 索引** `sih-tools/calllog/calls.db`：派生可重建，gitignore（照 locks.db 先例加 `sih-tools/calllog/calls.db*` 行）。
3. **markdown 投影**：19 册原位原格式，由追加命令同临界区再生（只再生受影响 tool 的册子）；投影与账本不符时以账本为准。

### 2.2 子命令面（零 LLM 零网络，全部双跑一致）

- `call-log append --tool <t> --occasion <s> --commands <s> --exit <s> [--session <s>] [--note <s>]`：三腿齐落
- `call-log render [--tool <t>]`：从账本再生投影
- `call-log reconcile`：db 与投影对账，缺行回补（restore_missing 同款网）
- `call-log import`：一次性存量迁移（见 2.4）
- `call-log rebuild`：db 从账本确定性重建

### 2.3 锁面归队（P2 已裁成立）

- SCOPE_SHARED_SURFACE 增 `sih-tools/calllog`（数据面）与 19 条 CALL-LOG.md 路径（投影面），auto 缺省即解析 append，并发批互不憋；真并发安全由写点命令内 flock 承载（纯追加形态下充分性经 pk057fix 压测先例）。
- declared_uncommitted_diff 豁免面随白名单自动生效（shared_surface_exempt 单源）。
- DIRECT_LANE_FILE_WHITELIST 两条 CALL-LOG 条目批内核对：追加命令写的面仍需直改车道合法即照录核对，缺即 CONTRACT 修订同步。
- 留痕分级：调用留痕不上 trail 主链（已裁）；trail 为 scribe 单写哈希链日切，不混域。

### 2.4 存量迁移（P3 已裁成立）

- 19 册 696 行（wc -l 实测）：每数据行整行原文入 `verbatim` 载体，日期退出码等类型字段尽力解析、失败即空值如实申报不猜不编。
- 验收门：行数符 + verbatim 零丢（逐行可回对）+ 解析失败清单在档；import 同参双跑逐字节一致。
- 表头行与分隔行不计数据行；lease 册两表头摞立按两段各计。

## 三、工作清单 {#work}

- [ ] T-1 calllog 模块与 append 三腿原子形（flock 临界区内 ndjson + sqlite + 投影）
- [ ] T-2 render / reconcile / rebuild 三件
- [ ] T-3 SCOPE_SHARED_SURFACE 扩面 + auto 解析 append + 差集闸豁免（测试锁定）
- [ ] T-4 import 存量迁移与对表报告
- [ ] T-5 并发压测（多进程并发 append，零丢零交错，pk057fix 8进程×25行同形）
- [ ] T-6 gitignore 行 + DIRECT_LANE_FILE_WHITELIST 核对 + CONTRACT 修订
- [ ] T-7 BATCH-FACE 调用留痕纪律节更新（徒手追加退役、追加命令在役、旧撞锁坑位注记改勘误）
- [ ] T-8 本批自身留痕 dogfooding：批内 lease 治理调用即用新 append 命令记录

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 写点原子 | 三腿齐落或齐不落；并发压测零丢零交错 |
| **F-2** | 重建确定 | rebuild 与 render 同参双跑逐字节一致 |
| **F-3** | 迁移忠实 | 696 行对表行数符 + verbatim 零丢 + 失败清单在档 |
| **F-4** | 锁面归队 | 新面 auto 解析 append（测试锁定）+ 差集闸豁免实测 |
| **F-5** | 双跑一致 | 全部子命令同参双跑输出逐字节一致 |
| **F-6** | 零 LLM | 命令族全程零模型调用零网络 |
| **F-7** | 语义承证 | 实装不偏离 m-calllog-dual-1 三件已裁语义；偏离即停批呈报不自行改裁 |

## 五、必读文件 {#read}

- 已裁语义与材料：sih-tools/proposition/DES/m-calllog-dual-1/（topic.md 与计分材料）；重放锚 sih-tools/facet/contracts/zc-glm53-20260907-calllog/sign-reports/m-calllog-dual-1-signcheck.json
- 写点母本：sih-tools/lease/src/lease/ledgerwrite.py
- 白名单现行：sih-tools/lease/src/lease/core.py 379-387
- 契约两照录：sih-tools/lease/CONTRACT.md 第 17 行（locksplit 共享追加面）与第 127 行（leaseup auto 缺省）
- 台账写点纪律：sih-tools/BATCH-FACE.md 427-429 节
- 迁移对象：19 册 */CALL-LOG.md（schema 漂移实证见 topic 照录）

## 六、约束 {#constraints}

1. TDD 先红后绿；既有全族测试零回归（现 227 绿基线）
2. 权威腿唯一：ndjson 之外任何腿不得成为事实权威；禁整文件重写账本
3. 判定语义零新裁：三件已裁即按裁落地，遇边界停批呈报
4. 架构常量零裸奔：新冻结常量（如路径族）随 CONTRACT 登记留源
5. 收约机械链全序照 BATCH-FACE；拆工地前先 cd 出去（勘误在册）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-7 全过
- [ ] lease CONTRACT 修订与 CALL-LOG 家族登记随批
- [ ] BATCH-FACE 纪律节更新随批
- [ ] 结果档 sih-engine/sih/event/plan/calllog-solo-results.md（走 T6 管线：化格核阅检词）与 materials/
- [ ] 双仓 settle + close + reconcile + verify 全绿

## 八、风险点 {#risks}

- 投影再生是整册重写：册子体量增长后再生成本线性——现 696 行量级毫秒级，如实申报不预优化
- 19 条投影路径白名单化后新增工具须 CONTRACT 修订才入族——宁窄勿宽是既裁方向，如实登记
- vecfix-solo 在飞撞锁即排队候叫，不绕行不 bypass

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 F 锚定、得一裁红线、双仓同步；判定语义已前置一裁（m-calllog-dual-1）非本批内裁。

## 十、关联文件 {#related}

- 命题与裁决：sih-tools/proposition/topics/2026-09-07-calllog-dual.md；DES/m-calllog-dual-1/
- 合同与材料：sih-tools/facet/contracts/zc-glm53-20260907-calllog/
- 结果档（随批产出）：sih-engine/sih/event/plan/calllog-solo-results.md

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/（calllog 模块 + cli 接线 + core 白名单扩面）
- sih-tools/lease/CONTRACT.md 与 sih-tools/lease/tests/
- sih-tools/calllog/（数据面新建）
- sih-tools/.gitignore
- 19 册 sih-tools/*/CALL-LOG.md（投影再生，随 import 与 append）
- sih-tools/BATCH-FACE.md（纪律节）
- sih-engine/sih/event/plan/calllog-solo-results.md 与 materials/
- sih-engine/sih/event/trail/（收约链笔）
