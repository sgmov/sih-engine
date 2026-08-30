# fmtfix-solo：数学仓机械格式归一批（修复计划批三后插批）

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图即 2026-08-30 链事件 87ce7ca5；前案即用户同日令用得一裁加开格式批，采样闸 m-fmtbatch 判 boundary（9/9 合规零变卦，依据三分与边界旗 44% 超阈），按 facet-measure skill 修订一法层精炼改道确定性管线
> 队形：单线形 solo——确定性脚本亲写零子代理零 LLM 语义判断
> 日期：2026-08-30

## 一、问题陈述 {#problem}

全量违规总账 1963 处中 S004 标题缺锚点 143 处与 C001 破折号 51 处属机械类：C001 的修复映射写死在规则消息（换全角冒号），S004 的修复可由冻结映射表机械追加。两类不清，后续内容批的核阅读数混有格式噪声。

## 二、关键设计 {#design}

确定性脚本两遍成：先锚点遍（13 条条目 33 个唯一标题按冻结映射表追加显式锚点，表外标题即报错不猜），后字符遍（全仓 126 条 C001 破折号按规则消息换全角冒号）。验收三件：核阅复跑 S004 与 C001 归零且其余类计数逐一不变、化格检词复跑零违例、git diff 仅锚点追加与标点替换两类变更。m-fmtbatch 采样判决与改道依据全档留痕不抹。

## 三、工作清单 {#work}

- [ ] 冻结映射表（11 个二级标题加 22 个三级标题 slug）
- [ ] 确定性脚本锚点遍加字符遍
- [ ] 核阅复跑全量对表验证
- [ ] 化格检词复跑
- [ ] 认证上链、三仓收约、对表
- [ ] m-fmtbatch 飞轮链与判决随批入档

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 两类归零 | 工程 | 核阅复跑 S004 为零 C001 为零 |
| **F-2** 零语义漂移 | 治理 | 其余各类计数逐一不变，diff 仅两类变更，表外标题零出现 |
| **F-3** 留痕完整 | 工程治理 | m-fmtbatch boundary 判决与九发采样与改道依据随批入档可回放 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/proposition/DES/m-fmtbatch/flywheel-trail.jsonl 即采样判决原件
- 必读 2：sih-tools/scrutinator/packs/des-001-mathe/rules.toml 即 C001 与 S004 规则定义

## 六、约束 {#constraints}

1. 零 LLM 语义判断，映射表外即报错
2. 不动 C006 等其余七类违规
3. 上链遇锁即等待不绕行
4. 词债不过夜

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-math/topology/entries/
- sih-math/probability/entries/
- sih-math/order/entries/
- sih-math/algebra/entries/
- sih-engine/sih/state/plan/fmtfix-solo.md
- sih-engine/sih/state/plan/fmtfix-solo-results.md
- sih-engine/sih/event/plan/fmtfix-solo-results.md
- sih-engine/sih/event/plan/fmtfix-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/proposition/DES/m-fmtbatch/
- sih-tools/proposition/topics/2026-08-30-fmtbatch.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，三仓段结算收约，对表复跑读数在档
