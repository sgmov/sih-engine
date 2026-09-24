# m-gatesplit：采样闸旗语分立与规则缺口台账（引擎 attractor 判据修订批）

> 治理任务包（判据修订类，队形 parallel 委外子代理，正典执行位 sih-engine/src/attractor）
> 承接：用户 2026-09-24 方向裁定原话「在司衡体系内，人与LLM同权，我引入得一就是要把LLM的责任转给固定程序，而固定程序又是人的授权。按照这个逻辑，闸门的修复应该也是同样的方向，而不是又丢给人」＋「开工」令
> 病灶证据：m-siacarr 批三轮采样（18 发全合零变卦、牌率 33–55% 常驻警示带），根因定性即裁判提示词 boundary_flag 定义「本命题是否落在规范边界（基线不直接回答、须推理适用）」把话题属性（超纲）与裁判信心（拿不准）缝在一根旗上；辅证即裁判规约枚举仅 baseline_1/4/5，注意力基线不在裁判视野

## 一、问题陈述 {#problem}

- 诚实裁判对"规则未覆盖题材"的命题必然举旗如实报超纲，闸把牌率当不确定信号，≥1/3 即打回要人——内容类命题永驻警示带
- 闸无"人已裁"输入通道；且修法方向经用户裁定为**责任转移给固定程序**（非逐案丢给人）
- 判据正典执行位在引擎 attractor（围堰 facet 冻结只读），修订须走引擎批

## 二、关键设计 {#design}

### 2.1 旗语分立

裁判输出四键扩为五键：decision／basis_regulation／reason／**boundary_flag（重定义：裁判对判定本身的信心，拿不准举 true）**／**coverage_flag（新增：命题题材是否落在规约未直接覆盖区，超纲举 true）**。提示词字段说明随改：boundary_flag＝「你对本判定是否拿不准（信心信号，与题材无关）」；coverage_flag＝「命题题材是否超出所给规约的直接覆盖范围（描述信号，不影响判定）」。

### 2.2 闸的重路由（判据 v4）

- boundary_rate 只统计 boundary_flag（信心信号）；阈值与三态逻辑承 v3 不变
- coverage_flag 计数单列（coverage_rate 报告项），**不参与三态判定**；计分材料与飞轮 trail 记录两旗分列
- 规则缺口台账：score 输出侧新增逐 gid 的 coverage 记录聚合（计分材料 json 内 coverage_flags 计数与举旗 reason 摘引），供立法时批阅——机械记账，不拦判定，不在线扰人

### 2.3 向后兼容

旧响应无 coverage_flag 键：解析缺席即 false（boundary_flag 旧语义数据在历史 trail 中不重算不改写，向前生效承 basisunion 先例）。围堰侧 measure.py 零改动（冻结）；引擎 emit-contract/score 双命令行为对齐新五键。

### 2.4 裁决命题

gid `m-gatesplit-1`：判据 v3 之 boundary_flag 单旗语义应修订为 v4 双旗分立（信心旗判定用、超纲旗记账用），阈值与三态不变。单基线锚：可验证性（修订前后同输入可对表、旧 trail 零改写、双态可证伪）。证伪条件：任一既有测试或金向量在新判据下行为回归即整批退回。

## 三、工作清单 {#work}

- [ ] 定位：src/attractor 内 boundary_flag 的提示词生成、响应解析、assess 成熟度判据三处落点，产出改动面清单
- [ ] 实施：五键 schema、双旗解析、v4 判据（boundary_rate 只看信心旗）、coverage 记账入计分材料
- [ ] 测试：单测覆盖（信心旗触发 boundary、超纲旗不触发、旧响应兼容、缺键 false）；既有测试与夹具回归
- [ ] 自裁命题 m-gatesplit-1 走新判据采样九发 stable_clear
- [ ] 后继：m-siacarr 批按新判据重送（另批不承诺本批内）
