# docmath-b2-solo：文档数学化程序批二载体入仓批

> task-packages 治理任务，承 docmath 程序（docmath-full-program-v1.md 与 docmath-task-package-2026-09-04.md）
> 队形：单线形 solo，确定性脚本与主线亲写零子代理
> 日期：2026-09-04

## 概览 {#overview}

- 批性质：载体入仓批，一档两载体一硬测试，零 CLI 变更零行为变更（测试件为新增非修改）
- 交付件：推导档 sih-math/docs/docmath-carriers-derivation-2026-09-04.md、金向量脚本与夹具落 sih-math/docs/docmath-coverage-2026-09-04/、合流性性质测试件入 sih-tools/formatter/tests/
- 载体一：约束完备性与规格充分性，锚 ORD-002 与 ORD-003，与 measure-poly 可识别性各管各的不合载（2026-09-04 人节点裁定）
- 载体二：证伪覆盖度，锚 ORD-010 与 PROB-008
- 硬测试：化格 general-v1 重写合流性性质测试（幂等重跑落零、算子任意交换序同形、金向量语料），承 ORD-023

## 一、问题陈述 {#problem}

内容充分性三角的规格充分性与判据充分性是文档面数学判据的空位：规格何时把实现压到唯一解、验收判据清单何时覆盖全部判定面，当前都是文学判断。批二把两者立为可机械复算的载体，并把化格正则形唯一从描述升为程序自动验证的性质测试。

## 二、关键设计 {#design}

- 规格充分性：约束集为有限格上的单调闭包算子，Kleene 迭代求最小不动点，对偶求极大不动点，两极相等即规格够即实现唯一，两极不等即薄规格红证成立即双模型 witnessed 对，歧义是定理不是文学判断
- 证伪覆盖度：判定面以有限突变集建模，判据清单的覆盖计数为可被某判据红证翻转的突变占比，无红证判据即零信息判据如实暴露
- 合流性硬测试：化格三线性算子（行尾空白剥离、行尾 LF、文末换行）性质测试：幂等（重跑落零）、任意交换序同形、金向量语料冻结，测试先红后绿，不满足即违规
- D-4 四查：引用命题在册（mapping.md 实取行号）、调用位实存（金向量脚本与测试件）、可证伪节在场、双答结构在场

## 三、工作清单 {#work}

- [ ] 推导档一件（两载体接线概览、载体映射、判定语义形式化、可证伪条件、金向量三场景、机械边界声明）
- [ ] 金向量脚本与夹具双跑逐字节一致
- [ ] 合流性性质测试件入 formatter 测试族，先红后绿读数
- [ ] 叩问四信号随批 register 通道登记消解
- [ ] 化格核阅检词三步管线与 checkcite 与认证上链三仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 载体成立 | 工程 | 规格充分性金向量三场景复算全中：充分格唯一解、薄规格双模型红证、矛盾约束零模型 |
| F-2 覆盖度成立 | 工程 | 覆盖度金向量三场景复算全中：全覆盖、部分覆盖带未覆盖突变清单、零信息判据暴露 |
| F-3 合流性在役 | 治理 | 化格性质测试先红后绿全绿入测试族，金向量语料双跑逐字节一致 |

## 五、必读文件 {#read}

- 必读 1：sih-math/llm-friendly-build/mapping.md（载体映射行号实取）
- 必读 2：sih-engine/sih/event/plan/docmath-task-package-2026-09-04.md（五裁定与批序）
- 必读 3：sih-math/docs/gatecap-derivation-2026-09-04.md（推导档文形先例）

## 六、约束 {#constraints}

1. 全部产出脚本可复算，复算不符即失败
2. 数学仓 mapping 与 entries 与 INDEX 零触碰；与 measure-poly 可识别性不合载不抢道
3. 三工具 CLI 与退出码语义零改；formatter 仅增测试件
4. 主树零直写，词债不过夜，禁管道掩退出码

## 七、请求写入 {#requested-writes}

- sih-math/docs/docmath-carriers-derivation-2026-09-04.md
- sih-math/docs/docmath-coverage-2026-09-04/
- sih-tools/formatter/tests/（合流性测试件）
- sih-tools/formatter/CALL-LOG.md、sih-tools/scribe/CALL-LOG.md、sih-tools/lease/CALL-LOG.md、sih-tools/nomenclator/CALL-LOG.md
- sih-engine/sih/state/plan/docmath-b2-solo.md 与 sih-engine/sih/event/plan/docmath-b2-solo-results.md 与批材料目录
- sih-tools/nomenclator/packs/core/terms.json（register 通道四词）
- sih-engine/sih/event/trail/2026-09-04.ndjson 与 sih-tools/scribe/reports/ 与 sih-tools/identity/reports/ 与 sih-tools/meter/counts/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，三仓段结算收约，对表读数在档
