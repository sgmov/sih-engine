---
name: "sihankor-calculus-trigger"
description: "Calculus concept retrieval for SiHankor. Invoke when reasoning involves limits, convergence, rate of change, accumulation, infinity, or differential equations. Loads the relevant calculus entry from calculus/llm-friendly-build/ to provide mathematical correctness support."
---

# SiHankor Calculus Trigger

微积分概念检索。遇到需要微积分概念作为推理工具的问题时，加载对应概念条目。

## 触发时机

推理过程涉及以下问题类型时触发：

- 收敛性判定（某序列 / 级数 / 迭代过程是否收敛）
- 变化率分析（某量随时间 / 空间的变化快慢）
- 极限行为（趋于某值时的行为）
- 累积计算（变化量的累加 / 积分）
- 无穷相关（无穷大 / 无穷小 / 无穷级数）
- 动态系统建模（微分方程描述的变化规律）

不触发：纯逻辑推理 / 概念推理 / 命题推导 / 文档格式问题。

## 执行步骤

第一步识别推理问题类型。把当前推理问题抽象为一个名词短语，如收敛率算法的设计。

第二步查检索路径。读 calculus/llm-friendly-build/mapping.md，在左列"推理问题类型"中定位对应行。若找不到精确匹配，按近义词或上位词扩展检索。

第三步确认状态。按右列概念 ID 查 calculus/INDEX.md，确认条目状态（已建 / 待建）。

第四步加载条目。已建则读 calculus/llm-friendly-build/entries/CALC-XXX-<concept>.md 原文，以条目内容为数学推理工具。待建则回已建条目判断是否有可复用的最小工具，或先按条目定义做最小推理。

第五步推理。以微积分概念为推理工具，不替代哲学命题或工程判断，只提供数学正确性支撑。

## 与哲学仓的关系

微积分仓是哲学仓引用数学概念时的支撑层。哲学仓约束治理生成与推理方向，微积分仓约束数学推理正确性。两者不互相支配。

调用微积分仓不改变哲学命题的约束力。微积分概念是工具，被哲学命题引用时生效，不独立约束治理行为。

## 边界

本 skill 只提供数学推理工具的检索路径，不产出数学命题的新推导。微积分仓的条目内容由人类决策者建设，AI 只检索与加载。

Base directory for this skill: /Users/moc/workspaces/SiHankor/sih-engine/sih/state/skills/sihankor-calculus-trigger
Relative paths in this skill are relative to this base directory.
