# 任务包 T3-3:SPEC-001 v2 关联段补 DEC-005 上游引用

## 一. 任务目标

在 SPEC-001 v2(路径 doc/spec/SPEC-001-divergence-experiment.md)关联段补一段上游引用,登记 DEC-005 阶段零判定为旧数据复用边界依据。

修订动作:
- 位置:SPEC-001 v2 关联段(找上游/下游/承接关系段,具体行号以 sub-agent read full 后定位)
- 新增段(精确字面):「上游:DEC-005 阶段零判定,旧数据复用边界。承接 doc/decision/005-phase-0-data-and-intent.md 子项一,可用文档集二百六十三份、八大类型分层、失败标记清单十份、可复现性五要素。」
- 不动其他章节
- 段标题统一用「上游」承接既有命名

## 二. 项目背景

异质性 deep-read agent 报告判定 T3-3 等于「须补,违反工程基线第四条」。SPEC-001 v2 L190 用二百六十三份文档做实验对象,但未引用 DEC-005 判定二百六十三份可复用的依据。追溯链断(SPEC-001 内不可见决策来源)。

## 三. 任务构成规则

- self-check 六项
- 通用格式规范:DES-001 / F000 / doclint
- 类型特异化约束:DES 修订类,只新增段,不动既有段
- 哲学检索:不涉及

## 四. 参考文件

- 目标文件: /Users/moc/workspaces/SiHankor/sih-engine/doc/spec/SPEC-001-divergence-experiment.md
- 关联决策: /Users/moc/workspaces/SiHankor/sih-engine/doc/decision/005-phase-0-data-and-intent.md(L33 旧数据判定 / L37 数据模型不兼容 / L117 可证伪条件)
- 异质性 agent 报告 T3-3 段
- 工程基线第四条: /Users/moc/workspaces/SiHankor/AGENTS.md 工程基线与禁止条款 §可验证性约束

## 五. 交付要求

- 修订后 SPEC-001 v2 路径
- doclint exit code
- 关键决策点:补段位置(具体行号)
- clarifications(如有)

## 边界

- 不读任务包外文件
- 不发明新机制
- 严格按新增段字面插入
- 必跑 doclint
