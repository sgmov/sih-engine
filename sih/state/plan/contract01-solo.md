# contract01-solo：facet 合同模式调用面改造批

> task-packages 治理任务
> 承接：sess-zcode-260829-contract01 三问意图即 08-29 链事件、用户同日批准合同模式方案即「facet 如何做成无探针、框架可调用」三步走
> 队形：单线 solo——主线亲写零子代理
> 日期：2026-08-29

## 一、问题陈述 {#problem}

facet 现行调用面绑死后台 key 与探针脚本，外部 agent 框架无法接入。用户批准改造方案：把调用面劈成两半，facet 侧剩确定性内核四件即出题、计分、闸门、验证，零 LLM 调用零网络零 key；框架侧只实现一件事即执行采样合同，把每条提示词原样发给自己的模型、原文逐发写回。本批立规格并实现三腿的两段式调用面，探针与数据飞轮不动。

## 二、关键设计 {#design}

采样合同是接口工件即 json 载每发 system_prompt 与 user_prompt、ng 装配哈希、判据包版本、席位自报；响应工件即 jsonl 每行 key、shot、raw 三键，承温度探针模式一既有先例。计分时合同哈希绑定响应、响应哈希绑定裁决材料，verify 全量重算。观察腿走 measure.py 与 singleseat.py 合同模式，干预腿 ng_assembler 本是纯函数直接内嵌装配结果，调整腿 dose_driver.py 两段式。闸门三态与人签核与前置分道全不变，出裁决材料不出意见。

## 三、工作清单 {#work}

- [ ] 规格 CONTRACT-MODE-SPEC.md v1 落 facet/docs
- [ ] measure.py 两段式 emit-contract 与 score，一次性模式保留兼容
- [ ] singleseat.py 合同模式产合同、gate 照跑，与直跑提示词逐字节一致
- [ ] dose_driver.py 两段式 emit 与 score，响应计分与直跑同构
- [ ] 零网络合同往返测试全绿
- [ ] facet-measure skill 修订五，state 权威源与 .agents 实体投影双写
- [ ] 管线三件、三证入链、任务包与结果档落位
- [ ] 双仓段结算收约、免参对表、心跳复验

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 规格定稿 | 工程治理 | CONTRACT-MODE-SPEC v1 落 docs，载两半切分与两工件格式与三腿接线与范畴排除四条与 F 判据与血统行 |
| **F-2** 观察腿合同化 | 工程实现 | measure.py emit-contract 与 score 两段过零 key 零网络往返；singleseat 合同模式产合同与直跑提示词逐字节一致、gate 照跑 |
| **F-3** 调整腿合同化 | 工程实现 | dose_driver 两段式 idempotent，响应计分与直跑同构 |
| **F-4** 测试 | 工程实现 | 合同往返零网络测试全绿，含三层哈希绑定 verify 重算 |
| **F-5** skill 修订五 | 工程治理 | facet-measure SKILL.md 模式一升格为通用调用形态、写明合同执行纪律，state 与投影双写一致 |
| **F-6** 治理收口 | 治理流程 | 管线三件过、三证入链、双仓段结算收约、对表与心跳绿态 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/facet/measure.py 即现行一次性调用面
- 必读 2：sih-tools/facet/singleseat.py 即五件套壳与观察腿
- 必读 3：sih-engine/sih/state/skills/sihankor-facet-measure/SKILL.md 即模式一先例与修订一至四

## 六、约束 {#constraints}

1. 探针与数据飞轮不动，底稿一字不改
2. 不做 SDK 或在线服务，文件合同是全部接口
3. 裁决权不变，stable_clear 仍待人签核
4. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 结果档落 sih/event/plan/contract01-solo-results.md，三证入链，双仓段结算收约

## 八、风险点 {#risks}

measure.py 现行代码体量未知，两段式重构须保兼容模式行为逐字节不变；singleseat 直跑与合同模式须共享同一提示词构造函数防双源漂移；闸门消费的响应格式与温度探针 cal-responses 先例对齐防造第二格式。

## 九、队形声明 {#formation}

单线 solo 即主线亲写零子代理，治理批默认形，选形即声明。

## 十、关联文件 {#related}

- 链件：sih/event/trail/2026-08-29.ndjson 即意图与三证
- 规格前身：sih-tools/facet/docs/SINGLE-SEAT-SPEC.md 即三腿规格 v1
- 先例：facet/probes/temp_probe.py 模式一 export-pack 与 score 即采样归 agent 计分归程序
- skill：sih-engine/sih/state/skills/sihankor-facet-measure/SKILL.md 即修订五落点

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/contract01-solo.md
- sih-engine/sih/event/plan/contract01-solo-results.md
- sih-engine/sih/event/trail/2026-08-29.ndjson
- sih-engine/sih/state/skills/sihankor-facet-measure/SKILL.md
- sih-tools/facet/docs/CONTRACT-MODE-SPEC.md
- sih-tools/facet/measure.py
- sih-tools/facet/singleseat.py
- sih-tools/facet/probes/dose_driver.py
- sih-tools/facet/probes/k2t_driver.py
- sih-tools/facet/contract_mode.py
- sih-tools/facet/tests/
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/ledger/
