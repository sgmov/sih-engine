# 多 agent 独立审阅机制实验（2026-08-16）

## 概览 {#overview}

- 实验验证 PRO-07 鉴层破自证循环的机制独立假设，即同一 LLM 加不同 prompt 视角验证能否达到多 LLM 厂商异质同等效果::[实验目的](#purpose)
- 五份审阅产出全部独立给出限定交付判定，同 LLM 加不同 prompt 也能破自证循环，机制独立性成立::[结论](#conclusion)
- 最小可执行形态即独立 prompt 加独立视角加独立验证，不需要多厂商多池多 key::[最小形态](#minimal-form)
- 机制独立不等于机制完美，实验同时暴露十一个真问题，元层盲区需哲学层审视::[警示](#warnings)

## 实验目的 {#purpose}

验证 PRO-07 鉴层"破自证循环"的机制独立假设：多 agent 独立审阅即同一 LLM、不同 prompt 视角验证，能否达到多 LLM 厂商异质同样的破自证循环效果。

核心问题：机制独立于 LLM 厂商数量吗，还是必须多 LLM。

## 实验设计 {#design}

- 标的：facet 工具当前状态，即五次 commit 之后
- 机制：单 LLM 即 MiniMax，加五份独立审阅产出
- 独立性维度：prompt 不同、验证路径不同、视角框架不同
- 判定：五份报告的关键发现是否独立、是否互为补充、是否一致

## 5 份审阅产出 {#five-reports}

- 一号：用户初始审计报告，同 LLM，用户口径 prompt，通用视角，理论
- 二号：root session 初判，同 LLM，root 自己起草 prompt，三层审加错位批评视角，理论
- 三号：外部 agent 1，同 LLM，root 派 verifier 起草 prompt，三层审加独立验证视角，理论
- 四号：外部 agent 2，同 LLM，用户自己派 prompt，三层审加漏层检查表视角，理论
- 五号：外部 verifier 3，同 LLM，root 派 verifier 起草 prompt，三层审加代码实测视角，实测

五份全部同 LLM，用户也是同款 agent，机制独立性靠 prompt 独立加视角独立加验证独立。

## 关键发现 {#key-findings}

五份交叉验证，逐发现记各号命中与共识：

- 限定交付判定：一号至五号全部命中，共识五比五
- 195 测试：全部命中，五比五
- 贡献度 NOT PASS 即 C=3601：全部命中，五比五
- 6/7 文档立题层缺：全部命中，五比五
- 7/7 文档 governance 层缺：一号四号五号命中，二号三号部分，共识二比五加一部分
- F3.4 幂等性盲区：四号指出，五号实测，一号三号未命中，二号部分，共识一实测加一理论
- multiview 不满足 A-A3.1：二号三号五号命中，三比五
- 双权威即 v1 与 v3 矛盾：二号三号五号命中，三比五
- methodology.yaml 文字失实：二号三号四号命中，三比五
- 同源预训练风险：二号四号五号命中，三比五
- N 加 C 剔除 flywheel_run = 966：四号五号命中，二比五
- terminology drift：四号五号命中，二比五
- stage2-03 与 stage2-04 矛盾：五号实测证实，二号四号部分，一号三号未命中，共识一实测加一理论
- user_posture 装饰：仅五号实测，一实测
- route v1 与 v3 不一致：仅五号独立发现，一实测

## 机制独立验证结论 {#conclusion}

五份报告全部独立给出"限定交付"判定：同 LLM 加不同 prompt 也能破自证循环。

机制独立性的关键证据：

1. 结论一致即五比五都说限定交付，不是五份互相对抗，是五份独立收敛
2. 细节互为补充即没有两份报告抓到完全相同的盲区，每份都有独到发现
3. 实测与理论一致即 verifier 写代码跑了四个实验，结论与理论分析一致
4. 跨视角不串扰即用户派的 agent 与 root 派的 verifier 独立产出，互不引用

## PRO-07 鉴层最小可执行形态 {#minimal-form}

单 LLM 加多独立 prompt 加多视角加独立验证，即破自证循环。

不需要多 LLM 厂商，不需要多 model pool，不需要多 API key。

只需要三条：独立 prompt 即每份不引导结论，独立视角即每份用不同审视框架，独立验证即独立跑数据不照搬前审。

## 对治理工程的工程意义 {#significance}

- 成本降低：多 LLM 厂商 API 成本是单 LLM 的 N 倍，多 agent 是单 LLM 加 N 份 prompt
- 可复现性：单 LLM 加多 prompt 模式可复制到任何场景
- 可验证性：机制本身可被元层审计，独立视角即独立证据

## 关键警示 {#warnings}

机制独立不等于机制完美。本次实验暴露的十一个真问题详见 stage2-03-program-signoff-report.md 等：

- 多 agent 独立审阅破的是"自证循环"，但不保证抓到所有真问题
- 真正能抓全问题的是多 agent 独立审阅加跨视角加代码实测加文档代码一致性 check
- 元层盲区如 A-A3.1 跨家族同源风险需要哲学层审视，不靠 agent 抓

## 后续 {#followup}

- 立项：用此实验结果支持 sihankor-proposition-defense skill 的多 agent 独立审阅模式
- 工具：写固定程序计算贡献度即已立 probes/contribution_metric.py，加三层漏层检查即已立 probes/check_three_proposition_audit.py
- 流程：commit hook 挂漏层检查加 cron 跑贡献度，已设
- 文档：本报告是 PRO-07 鉴层"破自证循环"机制的工程实证

## 原始 5 份审阅报告位置 {#sources}

1. 用户初始审计报告：用户会话上下文，未持久化文件
2. root session 初判含十一个真问题：本会话上下文
3. agent 1 报告：mvs_9a88cab3 session 产出，任务超时后用户接管
4. agent 2 报告：用户自己派的，未持久化为文件
5. verifier 3 报告：task_id bg_e8396576-372b-457c-9c0a-099d19aad5fd

## 实验日期与作者 {#meta}

- 日期：2026-08-16
- 作者：Mavis 即 root session，加三个外部 agent 审阅
- 关联文档：sih-engine/skills/sihankor-proposition-defense/methodology.yaml 即方法学真源、sih-engine/skills/sihankor-proposition-defense/probes/check_three_proposition_audit.py 即机械漏层检查、sih-tools/facet/probes/contribution_metric.py 即贡献度计算、sih-tools/facet/docs/contribution-metric-design.md 即贡献度设计
