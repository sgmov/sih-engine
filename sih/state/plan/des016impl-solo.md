# des016impl-solo 任务包

## 元信息（版本位必填）

- 批名：des016impl-solo
- version: v1
- 日期：2026-09-10
- 范式：单线形 solo
- 令源：用户 2026-09-10 令「开工」承 DES-016 设计档终裁

## 一、问题陈述

- DES-016 五点判词候实装：意图闸结构校验中性化、scribe plain 通道、CONTRACT 修订五十七、DES-014 修订二、init plain 模板

## 二、关键设计

- 按 DES-016 唯一正典逐点落地；lease 闸改结构校验（plain/ask3 双形同一必填集）；scribe intent --validation 对 plain 形豁免；mcpline 模板描述版本三源对齐；DES-014 修订二
- 本批 ask3 双门照跑：工具摘除哲学强制不豁免本域纪律

## 三、工作清单

- [ ] T-1 仪式链（ask3 双门叩问正身立约意图）
- [ ] T-2 lease 三件：core.py 结构校验、CONTRACT 修订五十七、1.41.0 三源对齐、测试
- [ ] T-3 scribe plain 通道：intent.rs 改、cargo 构建绿
- [ ] T-4 mcpline 三件：record_intent 描述与 validation 可选、init 模板与 README 行、0.6.0 三源对齐、测试
- [ ] T-5 DES-014 修订二＋管线三步
- [ ] T-6 结果档＋认证＋双仓 settle＋主树重编接线验收＋收约对账＋完工回报

## 四、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 | 工程 | plain 形意图记录过 lease open 闸（零哲学引文）；缺字段拒教学形；invalid JSON 拒 |
| F2 | 工程 | scribe intent plain 记录零 validation 上链成功；ask3 形缺 validation 拒；trail 笔 anchor_count 0 |
| F3 | 工程 | mcpline 模板件落地断言过；record_intent validation 可选；两仓测试族全绿零回归 |
| F4 | 治理 | CONTRACT 修订五十七与 DES-014 修订二与版本三源对齐在档；DES-014 走管线三步俱绿 |
| F5 | 接线 | 主树 cargo 重编后 scribe plain 形真跑成功；lease 与 mcpline 主树裸调用绿 |
| F6 | 治理 | settle 归并 close，reconcile 双零，链 verify 全文 valid |

## 五、必读文件

- sih-engine/doc/design/DES-016-mcp-pure-tool-intent-form-v1.md（唯一正典）
- sih-tools/lease/src/lease/core.py validate_intent 段
- sih-engine/src/event_stream/intent.rs

## 六、约束

- 五验与链闸与 closeguard 与理由码与授权矩阵零触碰；SPEC-023 零触碰；主树零直写；每命令即取退出码禁管道掩码

## 七、验收标准

- F1 至 F6 全过；测试族全绿零回归

## 八、风险点

- scribe Rust 改动涉引擎二进制，cargo 构建时长与主树重编时点如实申报
- mcpline test_init_domain 落地断言随模板件增需同步

## 九、范式偏离声明

- 无偏离

## 十、关联文件

- DES-016 设计档与 m-des016-2 测量工件（终签 756effb1）

## 十一、请求写入（收窄条款示范位）

sih-tools/lease/src/lease/core.py
sih-tools/lease/src/lease/__init__.py
sih-tools/lease/pyproject.toml
sih-tools/lease/CONTRACT.md
sih-tools/lease/tests/
sih-tools/mcpline/src/mcpline/httpface.py
sih-tools/mcpline/src/mcpline/writeface/
sih-tools/mcpline/src/mcpline/init.py
sih-tools/mcpline/src/mcpline/__init__.py
sih-tools/mcpline/pyproject.toml
sih-tools/mcpline/tests/
sih-engine/src/event_stream/
sih-engine/doc/design/DES-014-mcp-beta-security-model-v1.md
sih-engine/sih/state/plan/des016impl-solo.md
sih-engine/sih/event/plan/des016impl-solo-results.md
sih-engine/sih/event/plan/des016impl-solo-materials/
（目录级声明理由：测量与测试工件出生地，批量面）
sih-tools/scribe/reports/
sih-tools/identity/reports/
sih-engine/sih/event/trail/2026-09-10.ndjson
（链文件只经引擎 scribe 写位）
