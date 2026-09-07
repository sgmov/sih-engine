# attnanchor-solo：注入式回锚 v1 即回合锚机械注入

> 治理任务包（实装类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 令「我的意思也是强制过一遍，但是一定不能是再烧一轮token，应该是一个机械化的东西注入单窗口的上下文」＋开工令；设计源即本会话注入式回锚三层案之 v1
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 单窗口注意力偏导的强制位缺在对话回合层：机械链管写入不管注意，软纪律靠自律无执法
- 已有的机械注入通道未被使用：本环境 UserPromptSubmit 钩子可确定性注入 additionalContext，零 LLM 生成
- 罗盘件未建，回合内回算锚无承载位

## 二、关键设计 {#design}

### 2.1 注入位

工作区 .zcode/config.json 即 hooks.enabled true 加 events.UserPromptSubmit 指向 attnanchor 脚本，每条用户消息触发一次，钩子输出严格 JSON 单键 additionalContext 入上下文。

### 2.2 注入内容

五行即任务锚（.session-anchor.md 首行，agent 每任务切换一写非每回合一写）、在飞面（会话与锁账本回算）、泊界面（两线 selector 路由回算）、链面（当日 trail 笔数与末笔）、纪律令（三岔路由静态行）。

### 2.3 机器纪律

零 LLM 零网络纯标准库；退出码恒零即注入永不阻断；内部失效降级可见不静默即行内标注不可用；双跑逐字节一致；读面闸与罗盘件归 v2 与 v3 不在本批。

## 三、工作清单 {#work}

### Cluster 1：工地写

- [ ] sih-tools/attnanchor/ 立 anchor.py 即注入组合器与 CONTRACT.md 与 tests
- [ ] 工作区 .zcode/config.json 注册 UserPromptSubmit 钩即 hooks.enabled true
- [ ] .session-anchor.md 立首例任务锚
- [ ] 关键报告件认证入链

### Cluster 2：主线串行验证

- [ ] 脚本双跑逐字节一致、缺锚告警路径、严格 JSON 单键断言、零 LLM 静态扫描、时延实测
- [ ] 管线三步、认证、双仓 settle、放锁收约、reconcile 与链 verify
- [ ] 实弹首注自证形态申报即下一条用户消息或下一会话观察

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 双跑一致 | 工程 | 同输入双跑输出逐字节一致 |
| **F-2** 输出形与降级可见 | 工程 | 输出严格 JSON 单键 additionalContext 即五行；任务锚缺场时首行为显式告警不静默；selector 不可用时泊界行标注不可用 |
| **F-3** 零 LLM | 工程 | 脚本静态扫描零网络零模型调用即无 curl 与 urllib.request 与 openai 与 anthropic 字样 |
| **F-4** 注册在位 | 数据治理 | .zcode/config.json 即 hooks.enabled true 且 UserPromptSubmit 条目指向脚本且 timeout 在秒域 |
| **F-5** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |

## 五、必读文件 {#read}

- 钩子规范：zcode-guide 诊断 skill 即七事件与严格 JSON 单键与模板变量
- 账本事实源：sih-tools/lease/ledger/ 两 ndjson 与两线泊材料目录
- 先例：lease install-hooks 即守卫装位先例

## 六、约束 {#constraints}

1. 零 LLM 调用零网络即脚本纯标准库确定性
2. 退出码恒零即注入链路永不阻断会话，失效只降级可见
3. 读面闸与罗盘件不在本批即 v2 与 v3 后继
4. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
5. 守卫在位禁 plain git commit，收约补笔走 bypass 登记
6. 工作区根两件即 .zcode/config.json 与 .session-anchor.md 属仓外配置与约定件，非 git 域，随批声明

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后本批零活跃锁
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 attnanchor-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 钩子配置或须客户端重载才生效即本会话未必实弹，实弹自证顺延下一会话如实申报
- uv 冷启动时延即泊界行两线路由实测若超预算则 v1 降级单线或缓存归 v2 声明
- 严格 JSON 单键即多余键整笔废弃，输出形断言必须先过

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`
- 后继：v2 读面闸（PreToolUse 挂 Read）、v3 罗盘件合流
- 关联泊件：无触碰

## 十一、请求写入 {#requested-writes}

- `sih-tools/attnanchor/`
- `.zcode/config.json`
- `.session-anchor.md`
- `sih-engine/sih/state/plan/attnanchor-solo.md`
- `sih-engine/sih/event/plan/attnanchor-solo-results.md`
- `sih-engine/sih/event/plan/attnanchor-solo-materials/`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- worktrees 双仓 attnanchor-solo 工地
