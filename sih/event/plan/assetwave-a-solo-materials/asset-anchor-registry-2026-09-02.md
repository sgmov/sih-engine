# 资产回锚登记面

- 批次：assetwave-a-solo
- 日期：2026-09-02
- 裁决依据：m-assetanchor-rev stable_clear，执契终签 8f4065bb，verify identical
- 用途：数学仓载体扩容的输入清单，工程侧经数学仓消费面读取
- 纪律：SETSP 与旧仓只作盘点源，登记出处不转录降格内容；哲学仓原文是唯一规范力来源

## 盘点源名录

| 源 | 位置 | 身份 | 用法依据 |
| --- | --- | --- | --- |
| SETSP 先例库 | .tmp/SihEngineeringTechnologySelectionPrecedent | 沉默参考，只作盘点源 | ai-ex/SETSP-SILENT-REFERENCE-BOUNDARY.md 边界一 |
| 旧仓 sihankor | sihankor/ | 已废弃，失败经验继承失败设计不继承 | AGENTS.md 术语登记 |
| ai-ex 沉淀 | ai-ex/ | 实验沉淀，过程参照 | ai-ex/INDEX.md |
| 外仓 | AiCoder 等 | 候选，本轮不盘点 | 出泊唯人节点 |

## 登记面：机制 x 现行命题锚 x 资产出处 x 载体指向

命题锚格式为 仓内路径:行号，行号经程序切片核verified。

| 机制 | 现行命题锚 | 资产出处 | 载体指向 |
| --- | --- | --- | --- |
| 道层四道降格 | 02-on-first-tao.md:57、03-on-second-tao.md:15、04-on-third-tao.md:15、05-on-fourth-tao.md:15 | SETSP pillars/01 | 载体已在， CALC 与 TOP 不动点系 |
| 法层五法降格 | 06-on-canon.md:185 | SETSP pillars/02 | 统计族缺口， 见 G1-G5 与信任衰减 |
| 鉴层三义降格 | 07-on-assay.md:61 | SETSP pillars/03 | F/G/J 全序 ORD 已有；检验统计缺口一 |
| 应层二相降格 | 08-on-settle.md:5 | SETSP pillars/04 | 应几四触发统计族缺口一 |
| 元层三防御降格 | 09-on-arche.md:322 | SETSP pillars/05 | 隔离 TOP-008 已承接 |
| 信任评分五维与指数衰减 | 06-on-canon.md:185、05-on-fourth-tao.md:15 | SETSP collaboration/13 | 衰减 CALC 已有；权重校准归缺口一 |
| 悲观锁全序防死锁 | 06-on-canon.md:185、09-on-arche.md:322 | SETSP collaboration/14 | 缺口二：全序资源分配无死锁定理 |
| 多Agent工作流与依赖调度 | 02-on-first-tao.md:57、03-on-second-tao.md:15 | SETSP collaboration/15 | 缺口二：依赖 DAG 拓扑序与串并分解 |
| L 级别诚实分级 | 05-on-fourth-tao.md:15 | SETSP governance/16 | 序数量表 ORD 已有 |
| G1-G5 五法度量 | 06-on-canon.md:185 | SETSP governance/17 | 缺口一：相关系数、移动平均、变异系数、平稳性检验 |
| 审计留痕 | 08-on-settle.md:36 | SETSP governance/19 | 哈希链已有 |
| 约束注入 | 03-on-second-tao.md:15 | SETSP operationalization/21 | 前置约束已有机制 |
| 追问引擎四道 | 08-on-settle.md:5 | SETSP operationalization/22 | 缺口三：期望信息增益与序贯问题排序 |
| 验证管道 | 07-on-assay.md:61 | SETSP operationalization/23 | 缺口一：门限的统计标定 |
| 治理状态机 | 08-on-settle.md:36、06-on-canon.md:185 | SETSP architecture/02 | 转移矩阵 ALG 已有 |
| 鉴层规则引擎 | 07-on-assay.md:61 | SETSP architecture/03、旧仓 tools/doclint | F/G/J 全序与一票否决 ORD 已有 |
| 结算应辨四通道 | 08-on-settle.md:5 | SETSP architecture/04 | 缺口一：滑动窗口与异常检测 |
| 编组四形队形 | 06-on-canon.md:185 | ai-ex/T6-PARADIGMS.md、marshalling skill | 缺口二：队形即 DAG，反链宽度即并联度 |
| 检索优先教训 | 06-on-canon.md:185 | ai-ex/SETSP-PRECEDENT-REDISCOVERY.md 等 | 对治已入 skill；机械化缺口归 D 件 |
| 失败复盘经验 | 07-on-assay.md:61 | 旧仓 doc/research/old-repo-failure-analysis.md | 经验继承，不设载体 |

## 载体扩容指向汇总

- 缺口一统计族：假设检验与显著性、估计与置信区间、相关与回归、平稳性与变点检测，落 probability 子仓
- 缺口二调度秩序族：全序资源分配与死锁自由、依赖 DAG 与拓扑序串并分解，落 order 子仓
- 缺口三信息增益：期望信息增益与序贯问题排序，落 probability 子仓

## 勘误

- m-assetanchor-rev topic 锚 06-on-canon.md 标注 184 行实为空行，顺势句在 185 行，见 sih-tools/scribe/reports/2026-09-02-assetwave-a-anchor-erratum.json
