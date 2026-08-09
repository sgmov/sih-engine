# 任务包 T2-4:SPEC-001 v2 L98 偏离率与收敛关系独立判定语义补全

## 一. 任务目标

在 SPEC-001 v2(路径 doc/spec/SPEC-001-divergence-experiment.md)L98 段「独立判定」措辞处,补一句说明,消除「独立等于无关」误读。

修订动作:
- 位置:L98 段独立判定措辞处
- 新增句(精确字面):「独立判定意味着两套指标不互为否决;偏离率高加收敛态等于稳定告警(高偏离告警态),偏离率低加收敛态等于可能漏判(单向有效性)。」
- 不动 L98 原有句
- 不动其他章节

## 二. 项目背景

异质性 deep-read agent 报告判定 T2-4 等于「非冲突,是预期态,但独立判定措辞易误读为互不关联」。SPEC-001 L96-102 偏离率阈值与 L104 收敛双指标独立判定但语义关联:高加收敛等于稳定告警(DES-005 v2 L94 高偏离率态),低加收敛等于可能漏判(DES-005 v2 L66 单向有效性)。

## 三. 任务构成规则

- self-check 六项
- 通用格式规范:DES-001 / F000 / doclint 必跑
- 类型特异化约束:DES 修订类,只动目标段
- 哲学检索:不涉及,跳过

## 四. 参考文件

- 目标文件: /Users/moc/workspaces/SiHankor/sih-engine/doc/spec/SPEC-001-divergence-experiment.md
- 上游设计: /Users/moc/workspaces/SiHankor/sih-engine/doc/design/DES-005-semantic-verification-calculus.md(L66 / L94)
- 异质性 agent 报告 T2-4 段

## 五. 交付要求

- 修订后 SPEC-001 v2 路径
- doclint exit code(同前)
- 关键决策点(一到两条)
- clarifications(如有)

## 边界

- 不读任务包外文件
- 不发明新机制
- 严格按修订句字面插入
- 必跑 doclint
