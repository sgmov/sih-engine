# pkexits-solo：三件出泊（确定性核对道）与 A/B 设计一裁（采样道）

> 令源：用户 2026-09-05「全部用新的得一裁继续」——四件裁决改道得一：①②③出泊走确定性核对通道（状态/比对类分道，修订三、四），④A/B 设计走合同模式九发采样（新 test 判据在役首战）
> 范式：T6 单线 solo——出泊执行与测量批，零子代理
> 载荷：gateswitch-solo 三件出泊证据包（pk-048/049/054-exit-evidence.json）与 pk054-ab-gate-design.md

## 一、问题陈述 {#problem}

- 用户令四件裁决全部以得一裁承载，不再逐件人工准：pk-049（换闸在役）、pk-048（114 复核）、pk-054（A/B 批开工）出泊条件机械核对；pk-054 之 A/B 设计判定语义送采样。
- 核对道与采样道结论落裁决材料；stable_clear/核对全过即执行出泊与设计放行，near_threshold 或 boundary 呈人节点。

## 二、关键设计 {#design}

- 核对道三件：机械脚本逐项核出泊条件断言——049（缺省 test 两处在位 + 终签 fb2e6d9d 在链 + 金向量 archived_mismatch=0 凭据在档）、048（复核账本 114 件零漂移字段在档 + 双跑一致凭据）、054（设计文在档含四判据三切换条件 + gateswitch 批开工事实在链）；产出三份 crosscheck 报告即裁决材料。
- 采样道一件：gid m-abdesign-1，命题为 A/B 设计的机制材料断言（切换判据可机械校验满足基线四、硬前置显式），九发合同模式，新 test 判据在役首战；anchor 指 pk054-ab-gate-design.md 与 maturation_gate.py 拨后位。
- 三笔出泊事件 + 名录十二减三为九 + 历史住户增三；出泊裁定文载用户令「全部用新的得一裁继续」与核对/测量结论。

## 三、工作清单 {#work}

- [ ] x-01：三份 crosscheck 报告（确定性核对道）
- [ ] x-02：A/B 设计命题与九发测量（采样道，新判据首战）
- [ ] x-03：三笔出泊落链与名录投影更新
- [ ] x-04：管线认证与双仓收约与对表

## 四、可证伪条件 {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 核对道 | 三份 crosscheck 全绿（条件断言逐项机械核过），任一红即该件出泊冻结呈人 |
| **F-2** | 采样道 | m-abdesign-1 九发 stable_clear 执契，near_threshold/boundary 呈人 |
| **F-3** | 出泊 | 三笔 parking_exited 带 promoted 在链，裁定文载用户令与核对/测量结论 |
| **F-4** | 投影 | 名录九项与链机械一致 |
| **F-5** | 收口 | 双仓 settle、close、reconcile 零新增、verify valid |

## 五、必读文件 {#read}

- sih-engine/sih/event/plan/gateswitch-solo-materials/（三证据包 + pk054-ab-gate-design.md + 复核账本）
- sih-engine/sih/state/skills/sihankor-facet-measure/SKILL.md（分道与判据）
- sih-engine/doc/governance/PARKING-v1.md（出入入口协议）

## 六、约束 {#constraints}

1. 出泊裁定文=用户令「全部用新的得一裁继续」+ 核对/测量结论，逐字入链
2. 核对道零 LLM；采样道合同模式纪律（逐发原文、缺发拒收、identity-report 必挂）
3. 任一道不收敛即该件冻结呈人，零钓鱼
4. A/B 设计批准不解除 pk-044 硬前置（实装批前仍须用户裁独立性来源）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过；CALL-LOG 双笔随批（持锁冲突则 bypass 补笔）

## 八、风险点 {#risks}

- 新判据在役首战即测治理命题：boundary 概率真实存在，停批纪律优先
- 三笔出泊与名录更新次序：先链后投影

## 九、范式偏离声明 {#deviation}

单线 solo 零子代理；保留 T6-D 命名约定、F 锚定、双仓同步。

## 十、关联文件 {#related}

- sih-engine/sih/event/plan/pkexits-solo-results.md（随批产出）
- sih-engine/doc/governance/PARKING-v1.md、GOV-002-mainline-lock-v1.md（v2.2 第四条不动）

## 十一、请求写入 {#requested-writes}

- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/parking/materials/（三件 exit json）
- sih-tools/proposition/topics/、sih-tools/facet/contracts/idenlane-envelope-260905/ 邻位（m-abdesign-1 材料）
- sih-engine/sih/event/plan/pkexits-solo-results.md 与 materials/
- 双仓 trail 与 CALL-LOG 双笔
