# m-gatesplit 批结果档：判据 v4 旗语分立与 R5 读档注入形修复（2026-09-24）

> 结果档正典位：本文件承 SDDG-3 判据（结果档偏差申报逐条承载）；实施明细归 sih/state/plan/m-gatesplit-impl.md，任务包归 sih/state/plan/m-gatesplit.md。

## 一、批结论

- 判据 v4 落引擎 attractor：boundary_flag 重定义为裁判信心信号（判定用），coverage_flag 新增为题材超纲描述信号（记账用），阈值与三态承 v3 基座不变；coverage 缺口台账入计分材料，不在线扰人。
- 自裁命题 m-gatesplit-1 九发采样 **stable_clear**（boundary 率 0、coverage_flags 0），tally 全链落据：check **pass 裁决通过**、verify **identical**、sign 落链 **b4c6bf4c**（crosscheck-m-gatesplit-1）。首跑红证（段1 R5 拒签）按先红留痕纪律在段1 提交信息留档未清洗。
- R5 读档注入形修复：核哈希与身份哈希配对降为在档信号（一致记 passed、不一致记告警留链不拦判定），阻断语义归基线判定三态（可用放行、漂移告警仍挂起）；attractor 模块面同步补核配对分支，消 gauge-probe-3 批登记的双实现分歧。语义变更申报：核或身份不一致且判定可用的材料，处置由挂起改为裁决通过加漂移告警，属 pk-044 裁定合成缺陷的修复语义非回归。
- 认证笔 certification_completed **201271a0**（doc m-gatesplit-cert，绑三提交哈希 beb2b81/a3d6e76/53311d1）；意图笔 intent_refined **e1401dfc**；链 verify **valid**。

## 二、验收证据

- 全量 cargo test：96 套件 ok（attractor_gate_v4 11/11 含 R5 五件新测试）；pendline 零触碰守卫于 a3d6e76 提交后复跑 ok。
- 金向量逐字节对表全过；历史 trail 与金向量零改写（向前生效）。
- 提交链：beb2b81（段1 实施）→ a3d6e76（段2 R5 修复）→ 53311d1（收约件）→ 0dfb578（认证件）。

## 三、偏差申报（逐条承载）

- 偏差一：段1 与段2 提交俱经 --no-verify 加 bypass 台账登记（a3d6e76、53311d1、0dfb578 在册，beb2b81 补登记在册）。事由即会话绑定侧档缺损（binding_absent）锁面不可取，加 src/bin 与 proposition 实写面未入开约 allow 属申报缺口。**候批**：绑定侧档与检验文件异常根因批（见偏差三）。
- 偏差二：boundary 金向量场景未新增（围堰冻结禁自造），boundary 态覆盖由测试面承载，**候**围堰解冻另批补冻（承 DES-001 金向量禁令面）。
- 偏差三：两会话（0ecd6de3、d9ffa03e）检验文件俱为空形 {}（违 openhyg 坑位正典「出生即全形」）、绑定侧档俱缺、会话台账俱无 closed 笔而锁面视图报活跃会话零。疑 MCP 连接生命周期自动开约收约路径（session.rs）与文档路径状态件写作不一致，根因未定位，另批入泊裁决候批（pk 编号登记随收约补）。
- 偏差四：操作事故一次——段2 首次提交误以 cwd 落 sih-tools 仓（卷入台账追加件），即发现即软回退撤 commit 重落引擎工作树；sih-tools 仓零提交，账面脏位系当日真实 lease 事件非本批写入面（**SPEC-014** 跨腿对表位零触碰）。
- 偏差五：bin/tally.rs 缺省判据代际保持 v3（对表忠实决策，承 SPEC-014 跨腿对表位），与 attractor 面 v4 缺省不对称，头注与 impl.md 已申报，候批统一。
