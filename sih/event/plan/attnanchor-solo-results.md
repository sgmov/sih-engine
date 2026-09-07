# attnanchor-solo 结果档：注入式回锚 v1

> 承接：任务包 attnanchor-solo.md 与用户 2026-09-07 令「一定不能是再烧一轮token，应该是一个机械化的东西注入单窗口的上下文」加开工令。产出即 attnanchor 工具三件与钩子注册与任务锚首例。
> 队形单线形 solo，日期 2026-09-07，会话 sess-zcode-260907-attnanchor（session_id b2540a947d8b6484）。

## 意图锚定

- 意图事件：intent_refined `5f9d3776-aeaa-4529-a0d3-777e3d6f7bfc`（event_hash `5f92dbf2...`）
- record：sih-tools/scribe/reports/2026-09-07-ask3-attnanchor-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-07-ask3-attnanchor-solo-validation.json
- 三锚引文程序切片于生成器 make_ask3_attnanchor-solo.py。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：五词出信号，契约内五条处置后 digest passed covered 5。
- 正身：identity verify attest 零异常。

## 施工读数

- 工具三件落 sih-tools/attnanchor/ 即 anchor.py 注入组合器与 CONTRACT.md 与 tests/test_anchor.py 六测与 pyproject 0.1.0。
- 钩子注册：工作区 .zcode/config.json 即 hooks.enabled true 加 events.UserPromptSubmit 无 matcher，command 形指向 anchor.py，timeout 25 秒域，模板变量 ZCODE_PROJECT_DIR 定根，mcp 节原样保存在位。
- 任务锚首例：.session-anchor.md 即当前任务 attnanchor-solo 收口段。

## 验证读数

| 验证 | 实态 |
|---|---|
| F-1 双跑一致 | 同输入双跑 stdout 逐字节一致即测试过 |
| F-2 输出形与降级 | 严格 JSON 单键 additionalContext 即五行锚在飞泊界链令；缺锚显式告警行即测试过；真根实跑五面全活 |
| F-3 零 LLM | 静态扫描零网络零模型调用 token 即测试过 |
| F-4 注册在位 | config 即 enabled true 与 UserPromptSubmit 条目与 timeout 秒域，快照入批材料 |
| 时延实测 | 真根全量 0.35 秒含两线 selector 路由，远低于 25 秒预算 |
| 实弹形态 | 真根跑出样本入批材料即 injection-live-sample.txt；钩子实弹首注随下一条用户消息或下一会话自证，或须客户端重载如实申报 |

## 管线读数

- 化格：任务包与结果档与 CONTRACT 过 packs/general-v1。
- 核阅：des-001 对三件即 tools 与 state/plan 与 event/plan 皆域外退出码二如实记档。
- 检词：nomenclator packs/core 对三件。

## 认证清单

| 件 | 类型 | 状态 |
|---|---|---|
| ask3 记录与验证件 | 意图记录 | 书简认证 |
| 正身件 | 身份报告 | 书简认证 |
| 注入验证件 | 批验证 | 书简认证即测试与时延与实跑样本读数 JSON |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 双跑一致 | 工程 | 同输入双跑逐字节一致 | 通过 |
| F-2 输出形与降级可见 | 工程 | 严格单键五行、缺锚告警、降级标注 | 通过 |
| F-3 零 LLM | 工程 | 静态扫描零网络零模型调用 | 通过 |
| F-4 注册在位 | 数据治理 | config 即 enabled 与条目与 timeout | 通过 |
| F-5 写入仅 allow | 治理 | 写入仅请求写入节 | 通过 |

## 越线与误差申报

- 生成器首版经正则改造旧件再折一次即处置串内方括号截断非贪婪匹配，整写重生成后过双门，如实记档，同错二犯记教训。
- 无越线项。其余误差零申报。

## 结算读数

- 双仓 settle 与放锁收约与对表读数随收口补记。

## 缺陷披露（候人节点裁）

- 钩子配置或须客户端重载才在本会话实弹，实弹首注自证顺延下一会话的情形在案。
- 读钟例外已登记即钩子无人值守取实日，与谓词件禁读钟条款的边界差异在 CONTRACT 声明。
