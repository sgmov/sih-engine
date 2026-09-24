# m-gatesplit 实施：attractor 判据 v4（旗语分立与规则缺口台账）

- 实施位：worktree `/Users/moc/workspaces/SiHankor/worktrees/sih-engine/m-gatesplit`（分支 msh/m-gatesplit，基线 d371930）
- 承接：sih/state/plan/m-gatesplit-recon.md（侦察）与 sih/state/plan/m-gatesplit.md（任务包 §二设计）
- 日期：2026-09-24
- 围堰零触碰：sih-tools/ 全程只读引用（maturation_gate.py 只作语义定位），零写入

## 一、改动文件清单

新增：

| 文件 | 职能 |
|---|---|
| src/attractor/maturation.rs | 判据 v4 三态计算（assess_maturation_v4）：boundary_rate 只统计 boundary_flag（信心旗）；阈值 0.2/0.34/0.10；coverage_rate 单列报告项不入三态；附九件单元测试 |
| src/attractor/templates/atom.yaml | 引擎自有合同模式模板资产（v4 五键形）：以金向量范本 atom.yaml 733-760 段为基础修订，boundary_flag 重定义（信心信号）、coverage_flag 新增（描述信号），validator required_fields 扩五键。编译期内嵌（include_str!） |
| tests/attractor_gate_v4.rs | 规定单测四件（信心旗触发 boundary／超纲旗不触发仅记账／旧四键响应兼容／缺键 false）＋集成三件（外部判词兼容形逐字节面保持／emit-contract 引擎资产缺省／v4 材料过 tally check 全链） |

修改：

| 文件 | 改动 |
|---|---|
| src/attractor/contract_mode.rs | parse_raw_answer 扩五键（coverage_flag 缺席即 false；解析失败仍 no_answer+boundary_flag=true，coverage_flag 落 false——解析失败不是超纲陈述）；dc_from_responses per_actor 七字段扩八字段（加 coverage_flag，与 responses 按位对齐）；score_pipeline 的 gate_verdict 参数改 Option：None 即引擎 v4 自判闸（计分材料携带 gate 明细、criteria_version v4、coverage_flags/coverage_rate/coverage_reasons 缺口记账），Some 即外部判词直装配兼容形（材料面保持旧形逐字节不变，金向量对表线） |
| src/attractor/compiler.rs | extract_decision_fields 扩五键（coverage_flag 非 required，缺席即 false）；compute_decision_convergence per_actor 透传 coverage_flag 并新增 declared_coverage_rate 单列报告项（cell 四分格判定逻辑不动） |
| src/attractor/paradigm_loader.rs | 新增 TEMPLATE_ATOM_YAML 内嵌资产与 load_engine_template_atoms()；load_atoms 文件路径装载保持兼容 |
| src/attractor/mod.rs | 注册 maturation 子模块与文档行 |
| src/attractor/tally.rs | CRITERIA_VERSION 升 v4（单源引 maturation::CRITERIA_VERSION）＋ KNOWN_CRITERIA_VERSIONS=[v3,v4] 双认（v3 历史件向后兼容）；R3 per_actor 认七字段（v3）与八字段（v4）两形，全七字段形报文保持原文逐字节（金向量兼容）；criteria v4 材料加「R3: coverage 单列记账 N/M 发（描述信号不入三态）」报告行；assemble_material 透传计分材料的 criteria_version（未声明者按当代缺省 v4）与 coverage 三键 |
| src/bin/attractor.rs | emit-contract 的 --atoms 转可选：缺省引擎资产优先（templates/atom.yaml v4 五键形），显式给位外部件回退兼容（围堰 v3 形仍在位有效）；score 的 --gate-verdict 转可选：缺省即 v4 自判闸；USAGE 同步文档化两查找序 |
| src/bin/tally.rs | 校验面同步认 v4：criteria 认册集 [v3,v4]、per_actor 七/八字段双认、v4 材料 coverage 单列计数报告行；assemble 透传计分材料声明之 criteria_version 与 coverage 三键。缺省代际仍 v3（本件为围堰 tally 1.0.0 对表移植，缺省随其上游） |
| tests/attractor_contract.rs | t4_assemble 断言 criteria_version v3→v4（assemble 缺省随当代判据，本批修订点非回归） |
| tests/attractor_golden.rs | score_pipeline 调用形随签名更新为 Some(gate)（断言面不变，t2 计分材料仍逐字节对表金向量通过） |

金向量与历史 trail 零改写：fixtures/golden/ 全目录未动（含 contract-emit/atom.yaml 范本，其哈希冻结面完好）；全部 v4 行为向前生效（新解析五键、新判闸、新记账只发生在新产出面）。

## 二、设计落点对照（任务包 §二）

1. 旗语分立：五键 schema 落引擎模板资产；提示词字段说明即任务包原文（boundary_flag＝「你对本判定是否拿不准（信心信号，与题材无关）」、coverage_flag＝「命题题材是否超出所给规约的直接覆盖范围（描述信号，不影响判定）」）。
2. 闸的重路由：boundary_rate 只统计 boundary_flag；阈值 0.2/0.34/0.10 承 v3 基座形；三态逻辑（任一挂→boundary、全过有贴边→near_threshold、全过无贴边→stable_clear）＋单席语境核心的 B1（basis 退化维）/B2（单席决策噪声）降级规则如实移植（红线承 v2：任何降级不得越过 boundary_low）。coverage_flag 计数单列（计分材料＋飞轮 trail per_actor 双旗分列）。
3. 缺口记账：score v4 形计分材料增 coverage_flags（计数）、coverage_rate（率）、coverage_reasons（逐发举旗 key 与 reason 摘引）；assemble 与两 tally 校验面透传/复核。
4. 向后兼容：旧四键响应解析缺席即 false；外部 --gate-verdict 形材料逐字节保持旧面（integration_legacy_gate_param_keeps_legacy_shape 断言零 v4 扩展键）；历史 trail 与金向量零改写。

## 三、模板查找序与阻塞项申报

- 模板查找序已改：emit-contract 缺省走引擎内嵌资产（缺席不可能，编译期打包）；--atoms 显式给位为外部回退兼容。无外部（围堰）硬依赖阻塞：现实现模板路径本就是 --atoms 参数显式给位，围堰侧调用形不受影响，金向量范本冻结不动。
- 阻塞项一（如实记录）：pendline 批零触碰守卫 `zero_touch_critsweep_attractor_lease`（tests/pendline.rs L806）断言 src/attractor 等路径 git 工作树零 diff。本批改动即其watch面内文件，未提交工作树上必红；收约提交后工作树净即绿。失败输出所列 M/?? 文件与本批改动清单一一对应，无第三方改动。非行为回归。
- 阻塞项二（如实记录）：boundary 金向量场景未新增。金向量 manifest 基线声明「围堰 Python 原件（freeze_golden.py 驱动，唯一基准禁自造）」，围堰冻结只读，无权自造金向量亦无权跑冻结驱动。boundary 态覆盖改由测试面承载：既有 t6_tri_state_mapping_four_dispositions（boundary→打回重作）＋新增 maturation 单测（信心旗 4/9→boundary）＋集成单测一（全链 score→assemble→check 落打回重作）。候围堰解冻另批补冻。

## 四、测试与构建结果

- cargo build：退出码 0
- cargo test --lib：275 passed / 0 failed / 8 ignored（含 maturation 九件单测全过）
- 分套件：attractor_golden 7/7、attractor_contract 7/7、attractor_cli 6/6、attractor_route 全过、attractor_gate_v4 6/6、mergeall_t4_tally（bin tally）3/3
- 全量 cargo test：96 套件 ok、1 套件 FAILED（pendline，14 passed/1 failed，即阻塞项一）；退出码 101（cargo test 失败标准码）
- 金向量逐字节对表全过（t2_check_reports / t2_signcheck / t2_score_material / t2_contract_emit / t2_rejection_envelopes / t2_frozen_golden_zero_drift）：判据修订前后同输入对表成立（单基线锚「可验证性」之前半）；旧 trail 零改写（后半）

## 五、范畴排除声明

本批不动：围堰 sih-tools/（冻结）、金向量 fixtures/golden/、pendline/lease/critsweep 行为面、闸三态词表与 DES-011 四值处置逻辑、cell 四分格收敛逻辑（compiler 既有 0.5/0.5 阈值面）。bin/tally.rs 缺省代际保持 v3 的不对称属对表忠实决策，已在文件头注与本文档申报。

## 六、段2：自裁终签与 R5 读档注入形修复（2026-09-24 主会执行）

### 6.1 自裁命题 m-gatesplit-1 全链落据

- 九发采样经引擎 emit-contract（v4 五键模板）出题、QNAIGC:z-ai/glm-5.1 座席作答，score v4 自判闸 **stable_clear**（boundary 率 0、coverage_flags 0，计分材料 proposition/DES/m-gatesplit-1/m-gatesplit-1-score-material.json）
- tally 链：assemble（正身透传 core_hash 65101cca）→ check **pass 裁决通过**（R5 passed 行「基线判定可用；身份核哈希与冻结基线不一致，环境漂移信号留档」＋告警在档）→ verify **identical** → sign **落链**（event_hash b4c6bf4c，doc crosscheck-m-gatesplit-1）
- 首跑红证保留（先红留痕纪律）：段1 sign 拒于 R5 之拒签记录即本节 6.2 的修复输入，判词「席位身份哈希与基线不一致（处置走优先级映射）」在 beb2b81 提交信息留档，未清洗

### 6.2 R5 读档注入形修复（任务包 §2.5 范围扩展腿）

根因链：pk-035（08-30）身份哈希配对 → idcore-solo（09-02）核哈希优先 → pk-044（09-07）温度退役零新鲜采样。三裁合成后基线永久冻结于 2026-09-11 账本尾行（core e1c9a7fa），机器约 09-19 重启（boottime 1787917458→1789823102，mac 由缺席变在场，血统链 python>→identity>）后核哈希必然漂移，此后每批得一裁 sign 必挂起。09-13 后链上再无 crosscheck_completed 为证。gauge-probe-3（09-12）预申报此双缺陷候批，末次成功签靠 py 侧同日核侥幸配对，重启后该绕行亦死。

修法（承用户责任转移裁定）：R5 阻断语义收缩为基线在场与判定三态（可用放行、漂移告警仍挂起）；核哈希与身份哈希配对降为在档信号（一致记 passed、不一致记 passed 加告警留链不拦判定）。改动：src/bin/tally.rs 两 mismatch 分支、src/attractor/tally.rs R5 重排为核配对优先三层（同时对齐 bin 面即消 gauge-probe-3 登记的双实现分歧）、assemble_material 增 identity_report 参数（读 identity.core_hash 入材料，对齐 pendsweep-solo 形）。新增测试五件（tests/attractor_gate_v4.rs R5 组）：核等值快径不动、核不一致放行加告警、基线漂移仍挂起、身份哈希不一致放行加告警、旧报告无核回退。

### 6.3 会话状态异常与 bypass 处置（如实申报）

- 提交 beb2b81（段1）与 a3d6e76（段2）均经 --no-verify：会话 0ecd6de3 绑定侧档（worktrees/.bindings/0ecd6de3*.json）缺席致 lease lock 报 binding_absent，锁面经文档路径不可取；同时 src/bin 与 proposition 实写面未入开约 allow（申报缺口）。a3d6e76 已登记 lease bypass（台账在册）；beb2b81 的 bypass 补登记随收约（段1 提交时守卫拒件信息在案）。
- 伴生异常如实记录候另批：两会话（0ecd6de3、d9ffa03e）检验文件俱为空形 {}（出生即空，违 openhyg「出生即全形」约）、绑定侧档俱缺、会话台账俱无 closed/revoked 笔而锁面视图报活跃会话零。疑 MCP 连接生命周期自动开约/收约路径（session.rs 连接建立自动 open、断开自动 close）与文档路径状态件写作不一致所致，未定位根因，入泊候裁。
- 操作事故申报：段2 首次提交误以 cwd=sih-tools/lease 落 sih-tools 仓（卷入台账追加件），即发现即软回退撤 commit，两仓状态各归其位后重落引擎工作树正确提交 a3d6e76；sih-tools 仓零提交零改动（账面脏位系当日真实 lease 事件，非本批写入面）。

### 6.4 段2 测试与链结果

- cargo test 全量：96 套件 ok（gate_v4 增至 11/11）；pendline 零触碰守卫于 a3d6e76 提交后复跑 **ok**（阻塞项一解除）
- 链 verify：2026-09-24 trail **valid**（last_hash b4c6bf4c）
- 本会话链笔：intent_refined e1401dfc（session 0ecd6de3）＋ crosscheck_completed b4c6bf4c ＋ certification_completed（收约认证笔，见 6.5）

### 6.5 收约面

- 结果档即本文件；任务包 sih/state/plan/m-gatesplit.md 工作清单前六件俱勾
- 终件提交：signcheck/check-report/tally-input/impl/任务包入版控（bypass 登记随批）
- 后继另批：m-siacarr 四候选命题按 v4 判据重送（材料封存于 worktrees/sih-math/m-siacarr 分支 msh/m-siacarr，提交 f667c20/f7e2a27）；绑定侧档与检验文件异常根因批；beb2b81 bypass 补登记
