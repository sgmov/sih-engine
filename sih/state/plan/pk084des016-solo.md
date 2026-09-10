# pk084des016-solo 任务包

## 元信息（版本位必填）

- 批名：pk084des016-solo
- version: v1
- 日期：2026-09-10
- 范式：单线形 solo（测量先行：3 发得一裁裁蓝图，过则同批起草 DES-016 设计档）
- 令源：用户 2026-09-10 令「我觉得没有问题，得一裁，裁一过一执行一，不过的入泊」承 pk-084 蓝图呈报；同日令「候裁处置范式流程应添加司衡引擎内，得一裁可选 3/6/9 三档或用户关闭，默认 3 低档」

## 一、问题陈述

- pk-084 在泊：MCP 纯工具化缺口——意图闸机械强制 ask3 哲学引文形（lease validate_intent 内嵌 scrutinator ask3 包加 ask3repeater，scribe intent 硬校验验证件），外部域写链被迫内嵌司衡哲学内容，违用户「哲学不侵入其他项目」裁
- 出泊蓝图已呈报候裁：意图形分离设计（plain 与 ask3 形分派、机械闸结构校验中性化、ask3 双门归司衡本域批纪律、scribe intent plain 通道、CONTRACT 与 DES-014 修订、init plain 模板）
- 用户另令将「候裁处置范式」（得一裁 3/6/9 档或关默认 3、裁一过一执行一、未过入泊）立入引擎，候立项

## 二、关键设计

- 测量先行：facet 3 发采样（用户裁低档默认）裁命题 m-des016-1（DES-016 蓝图规约相容性），stable_clear 经 attractor 查验签机器终签后同批起草 DES-016 八问设计档；非 stable_clear 即如实入泊
- 命题负载限规约相容性：意图形分离是否与基线一四五（确定性程序、可验证性、减少人参与）及铁律零 LLM 裁决位相容；不裁实现细节
- 同席披露：采样席位 ZCode:GLM-5.3-Flash:self-reported 与命题起草同席，无跨席隔离如实申报

## 三、工作清单

- [ ] T-1 ask3 双门＋叩问＋正身＋立约＋意图上链（主线）
- [ ] T-2 facet 3 发：topic＋contract＋回填＋score＋attractor 查验签（主线）
- [ ] T-3 stable_clear 则起草 DES-016-mcp-pure-tool-intent-form-v1.md 八问档＋管线三步
- [ ] T-4 pk-084-exit.json 出泊材料（engine 泊位）与 pk-085.json 立项泊位（tools 泊位）与名录行
- [ ] T-5 结果档＋认证＋双仓 settle＋收约对账＋回锚完工回报

## 四、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 | 测量 | facet 3 发合同与作答与计分与查验签全件在档可复算，判词如实转述 |
| F2 | 治理 | stable_clear 方起草 DES-016；非 stable_clear 零设计档只入泊如实记 |
| F3 | 设计 | DES-016 八问逐问有判词与依据，判定语义变更候用户裁零实装 |
| F4 | 治理 | pk-084 出泊与 pk-085 立项俱走停泊事件上链，名录行在档 |
| F5 | 治理 | settle 归并 close，reconcile 双零新增，链 verify 全文 valid |

## 五、必读文件

- sih-tools/parking/materials/pk-084.json（出泊对象）
- sih-tools/lease/src/lease/core.py validate_intent 段（病灶）
- sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md 与 DES-015（修订对象正典）
- sih-tools/BATCH-FACE.md 第五节执契与 facet 调用面

## 六、约束

- 设计批零实装代码；意图闸现行行为零改，判定语义变更候用户裁
- facet 自工地 facet 目录跑，飞轮 trail 落工地
- InferServer 域与中央登记册零触碰；每条命令即取退出码禁管道掩码

## 七、验收标准

- F1 至 F5 全过；判词如实转述；DES-016 候裁形在档

## 八、风险点

- 3 发低档测量强度有限，同席无跨席隔离——如实申报，用户裁定的默认档
- DES-016 若涉 scribe 引擎件（Rust）修订面，实装批代价高，设计档须如实申报改面

## 九、范式偏离声明

- 无偏离：单线 solo 实跑

## 十、关联文件

- 温故检索档：sih-engine/sih/event/plan/pk084des016-solo-materials/recall-topic-pk084.md
- 命题先例：sih-tools/proposition/DES/m-mcpauth-2/（九发形对照）

## 十一、请求写入（收窄条款示范位）

sih-engine/sih/state/plan/pk084des016-solo.md
sih-engine/doc/design/DES-016-mcp-pure-tool-intent-form-v1.md
sih-engine/sih/event/plan/pk084des016-solo-results.md
sih-engine/sih/event/plan/pk084des016-solo-materials/
（目录级声明理由：测量工件与检索档等新文件出生地，批量面）
sih-engine/sih/state/parking/materials/pk-084-exit.json
sih-tools/proposition/DES/m-des016-1/
（目录级声明理由：facet 合同工件出生地）
sih-tools/parking/materials/pk-085.json
sih-tools/PARKING-v1.md
.session-anchor.md
sih-tools/scribe/reports/
sih-tools/identity/reports/
sih-engine/sih/event/trail/2026-09-10.ndjson
（链文件只经引擎 scribe 写位）
