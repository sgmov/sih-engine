# 任务包 T2-1:SPEC-001 v2 L80 均值基准样本集定义补全

## 一. 任务目标

在 SPEC-001 v2(路径 doc/spec/SPEC-001-divergence-experiment.md)L80 段「波动系数等于标准差除以均值」后,补一句样本集定义,消除均值歧义。

修订动作:
- 位置:L80 段波动系数定义句之后
- 新增句(精确字面):「波动系数的样本集等于同一文档同一检查项在指定维度(跨次审阅或跨专家审阅)下的 K 由 N 除序列。」
- 不动 L80 原有句
- 不动其他章节

## 二. 项目背景

异质性 deep-read agent 报告判定 T2-1 等于「待补,非阻塞但应补」。跨次审阅与跨专家审阅用同一公式但样本定义不同(同一文档同一检查项的多次 K/N vs 多专家 K/N),L80 当前未显式区分。

## 三. 任务构成规则

- self-check 六项:逐项跑,不过则修订
- 通用格式规范:DES-001 / F000 围栏代码块(mermaid 与 filetree only)/ doclint 必跑
- 类型特异化约束:DES 修订类,只动目标段,不自创新段
- 哲学检索:本次修订不涉及哲学命题,跳过

## 四. 参考文件

- 目标文件: /Users/moc/workspaces/SiHankor/sih-engine/doc/spec/SPEC-001-divergence-experiment.md
- 异质性 agent 报告 T2-1 段(无独立文件,本会话上下文)
- 任务包模板参考: /Users/moc/workspaces/SiHankor/ai-ex/SUBAGENT-TASK-TEMPLATES.md 模板二

## 五. 交付要求

- 修订后 SPEC-001 v2 路径(同四)
- doclint exit code:跑 /Users/moc/workspaces/SiHankor/sihankor/tools/doclint/target/release/sih-doclint <产物> 并贴 exit code
- 关键决策点(一到两条)
- clarifications(开放问题)

## 边界

- 不读任务包外文件
- 不发明新机制
- 严格按修订句字面插入,不增不减
- 必跑 doclint,缺 exit code master agent 直接回退
