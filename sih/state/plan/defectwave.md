# defectwave：五缺陷并一修复批

> 令源：用户 2026-09-18 令「同意，我希望你作为主编排，拉起子代理顺序完成一 二 三」第二件；四件系同日五批结算披露缺陷，密五件系越限侦察批读数
> 实证出处：usedpaths-results.md 偏差申报、des017 与 parksync 两批结果档偏差申报、越限侦察会话档案、cascadeclose 基点复现实证
> stem 认领：defectwave，甲表三件即 zh 五缺陷并一修复波、code 无承、派生 defectwave:new

## 问题陈述 {#problem}

五件缺陷：一即 lease open 对正身与意图件缺席零校验即空哈希签发（usedpaths 坏会话 b2a65416071e1f06 在案）；二即收约罚金取样窗口假罚（merge-then-close 形下 close 时取样读已归并空差分，实批 9 点误罚在账）；三即 scrutinator 工地路径域匹配缺口（des017 与 parksync 两批首跑 exit-2 绕行在案）；四即 cascadeclose no_worktree_carrier 存量红（pk-104 推导形与测试预期错位，基点复现实证）；五即 attnanchor 域覆盖不做 last-row-wins（撤牌域告警不灭，侦察批判词在案）。

## 关键设计 {#design}

一：validate_open_inputs 前置一切副作用零残留，校验存在与非空与 JSON 可解析与 identity_hash 非空。二：取样前移 settle 成功位写缓存件 usedpaths/<session_id>.json，close 缓存优先缺席回落，collect_used_paths 抽纯函数两边复用，回执零新增顶层键且 sampled_at 仅入罚单 detail。三：normalize_worktree_rel 补 w/<名>/ 至 sih-engine/ 归一，域清单零扩面。四：纯测试改即 open 后 close 前以 git worktree remove 正形拆引擎工地使 skip 形真可达，断言面原样验证力不降，生产码零改。五：registry_domains 改按 domain_root 文件序末行 status 判定，仅最终态 active 入覆盖面，输出域根排序确定性面。

## 工作清单 {#work}

- [ ] dw-01：缺陷一 open 输入闸
- [ ] dw-02：缺陷二罚金取样前移
- [ ] dw-03：缺陷三 scrutinator w 形归一
- [ ] dw-04：缺陷四 cascadeclose 测试改形
- [ ] dw-05：缺陷五 attnanchor last-row-wins
- [ ] dw-06：settle 加 close 加结果档

## 验收 {#acceptance}

五件各先红后绿红证在案；lease 系与 usedpaths 系与 cascadeclose 系与 scrutinator 系全量回归零新红；真台账验证撤牌域告警灭；settle 加 close 以本批新二进制 dogfood 即缓存写读全链在案；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/src/bin/lease.rs
- sih-engine/src/bin/lease/closegate.rs
- sih-engine/src/bin/lease/commitlaw.rs
- sih-engine/src/scrutinator/rule.rs
- sih-engine/src/scrutinator/tests.rs
- sih-engine/src/bin/attnanchor.rs
- sih-engine/tests/defectwave_open_input_gate.rs
- sih-engine/tests/defectwave_scrutpath_wform.rs
- sih-engine/tests/defectwave_attnanchor_coverage.rs
- sih-engine/tests/lease_usedpaths_penalty.rs
- sih-engine/tests/lease_cascadeclose_projection.rs
- sih-engine/tests/lease_mergeback_t5_commit.rs
- sih-engine/tests/lease_mergeback_t7_sweep.rs
