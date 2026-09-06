# confrulegate-solo：置信度规则面三件得一裁批

> 治理任务包（测量类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「得一裁」即规则面三件（标定程序认可、提案四立则、D6 定形）不落人节点，灰区默认从严过得一（DEC-021 执行令）；命题 m-confrule-1 已起草于 sih-tools/proposition/topics/2026-09-07-confrulegate.md
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 置信度规则面三件停在"待用户裁"形态，与路由令冲突：规则裁决应过得一机器测量加终签，人节点只接收 boundary 转诊断件
- 命题已自包含起草，测量链（合同、采样、计分、核对、终签）未跑

## 二、关键设计 {#design}

- 命题 gid m-confrule-1，三判定点 P1 程序认可、P2 提案四立则、P3 D6 定形，单锚 baseline_4 可验证性
- 席位 ZCode／GLM-5.3（先例一致），九发采样，逐发携 system_prompt 即裁定者角色与裁定总则
- 计分携正身件（pk-035 条款）；核对走引擎 attractor check 十二项、verify identical、sign 三态
- stable_clear 件机器终签落据成规则；boundary 件转人诊断呈报，不硬凑不重采样凑数

## 三、工作清单 {#work}

- [ ] 命题落命题区（topics 已在位即核形），DES 材料单元 m-confrule-1 建于 sih-tools/proposition/DES/
- [ ] emit-contract 出合同，回填九发，score 携正身件
- [ ] attractor check／verify／sign 三步落链
- [ ] 结果档记三件裁定值与落位；boundary 件呈人诊断
- [ ] 管线三步、书单对表、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 命题自包含 | 数据治理 | 命题引文与推导档 v2 逐字节对得上，元数据齐 |
| **F-2** 采样携正身 | 治理 | 九发计分材料携 identity_hash，席位与正身不混同 |
| **F-3** 三态终签 | 跨族治理 | check 十二项全过、verify identical、sign 三态如实（refused 透传不掩） |
| **F-4** 落据不越权 | 治理 | stable_clear 件落据零人挑零改式；boundary 件转诊断不代裁；本批零数值断言零代码 |

## 五、必读文件 {#read}

- 命题：`sih-tools/proposition/topics/2026-09-07-confrulegate.md`
- 规范源：`sih-math/docs/confmodel-derivation-2026-09-07.md`（v2）
- 先例：`sih-engine/sih/event/plan/confconst-solo-results.md`（一裁实录与坑位）

## 六、约束 {#constraints}

1. 零代码改动，零公理改动，零推导档改写（裁定落据后改式归后继修订批，本批只裁不改）
2. 采样从工地 facet 跑（勘误候选即此条，勿重蹈主树误写）
3. 主树零直写经工地 settle 通道，链文件只经引擎 scribe 写位
4. 在泊件零触碰；在盘遗留无主件不豁免不代清
5. boundary 即如实转呈，不重采样凑共识

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增，checkcite pass
- [ ] 结果档 confrulegate-solo-results.md 落 event/plan，三件裁定值随档

## 八、风险点 {#risks}

- 采样席位漂移或正身缺失即计分拒（pk-035）——开工前核席位形与正身件
- checkcite 多 --cited 单值坑（勘误候选）——拼接扫描形调用

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/attractor`（合同、核对、终签）、`sih-tools/facet/measure.py`（采样腿双模）
- 接口：裁后落位批（D6 定形与 P2 改式归后继修订批）

## 十一、请求写入 {#requested-writes}

- `sih-tools/proposition/DES/m-confrule-1/`
- `sih-tools/facet/contracts/confrulegate-260907/`
- `sih-engine/sih/state/plan/confrulegate-solo.md`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/confrulegate-solo-results.md` 与 `confrulegate-solo-materials/`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 confrulegate-solo 工地
