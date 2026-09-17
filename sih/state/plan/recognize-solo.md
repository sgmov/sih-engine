# recognize-solo：HTTP 读面连接识别域隔离与签发闸

> 令源：用户 2026-09-14 开批裁定（实验侧越权读汇报经开发侧核实后核定，修复清单第五节）
> 范式：T6 单线 solo，主代理亲写零子代理

## 一、问题陈述 {#problem}

- 病灶一：HTTP alpha 读面对一切连接（active 绑域牌、停行牌、无牌）一律投影中央第一域——绑域只约束写面租约，AI-MANUAL 第 8.4 节承诺的绑域读投影未实装。2026-09-14 活体复现在案：正规域牌 sim-aesthetic-workbench 与停行牌 sim-dev 的 critsweep 均回 root 等于 SiHankor 中央根。
- 病灶二：降级形（缺头、停行、未登记）读数未附 identity_notice 教学语（手册第 2 节承诺未兑现），且投影内容为开发侧全量治理态。
- 病灶三：控制台签发无域自举闸——未域自举根（根下无 sih 树）可签出 active 牌（sim-dev 实例在册已撤）。
- 病灶四：locks_read 在 HTTP 面回执退出码 127（lease CLI 解析位失效）。
- 病灶五：引擎租约件甲表认领死锁——attachments.rs 派生对表写成等值判定（kind 不等 actual 即拒），新词段申报 new 而查册实态 unknown 永远对不上，认领通道结构性不可过；围堰 core.py 为非对称判定（new 要求实态 unknown），双跑不一致实锤（2026-09-14 对表在案）。

## 二、关键设计 {#design}

- 2.1 连接读上下文：HTTP tools/call 于读面执行前解析连接身份。active 牌投影所绑域根；绑域未域自举（无 sih 树且非中央根）硬拒教学语，禁回退中央；降级形回受控教学面（identity_notice 加公共字段：server 版本、理由码、立牌途径），不投影任何域数据。
- 2.2 中央第一域读数收敛为绑域即中央根的 active 牌专属。
- 2.3 签发闸：issue 与 confirm-issue 校验目标根已域自举（sih 树在位或即中央根），否则教学拒。
- 2.4 locks_read 修复：lease CLI 以绝对路径解析。
- 2.5 AI-MANUAL 第 2 节与第 8.4 节语义随批修订，漂移守卫对表同步。
- 2.6 引擎甲表对表修复：attachments.rs 派生对表改非对称判定对齐围堰 core.py（new 段要求实态 unknown；established 段要求实态 established）。

## 三、工作清单 {#work}

- [ ] iso-01：读上下文解析与绑域投影
- [ ] iso-02：降级教学面与 identity_notice 三形
- [ ] iso-03：签发域自举闸
- [ ] iso-04：locks_read 退出码 127 修复
- [ ] iso-05：回归四用例收编加既有套件绿
- [ ] iso-06：AI-MANUAL 修订与 HTTP 活体验收
- [ ] iso-07：引擎租约甲表对表修复加回归

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 绑域投影 | active 绑域牌读数 root 即所绑域（sim-aesthetic-workbench 实测） |
| **F-2** | 硬拒 | 绑未域自举根的 active 牌读硬拒教学，零中央回退 |
| **F-3** | 教学面 | 降级三形读回教学面含 identity_notice，零 sih-engine 路径泄漏 |
| **F-4** | 写面不回退 | session_not_bound 与 token_stopped 闸语义原样 |
| **F-5** | 签发闸 | 未域自举根签发被教学拒 |
| **F-6** | locks_read | HTTP 面退出码 0 |
| **F-7** | 套件 | 既有 mcpserver 套件绿加新增回归绿 |
| **F-8** | 甲表对表 | 引擎认领径对围堰同参双跑判词一致，新词认领可过 |

## 五、必读文件 {#read}

- sih-engine/src/mcpserver/httpface.rs 与 server.rs 与 alpha.rs 与 passthrough.rs 与 webface.rs
- sih-tools/mcpline/AI-MANUAL.md
- sih-tools/lease/src/lease/core.py（围堰甲表对表基准）
- sih-engine/sih/event/plan/（越权汇报核实材料随批落 materials）

## 六、约束 {#constraints}

1. stdio 面行为零变化（中央域默认根不变）
2. 写面闸语义零回退
3. 手册修订与实装同批，漂移守卫全绿
4. T6 管线序固定：化格、核阅、检词、认证

## 七、范式偏离声明 {#deviation}

- 主代理单线亲写；CLI identity verify 当日 137 异常（默认形子进程采集链被杀，--claims 形亦复现），正身改走 MCP server 进程签发，异常记档候修。
- 引擎租约甲表死锁阻塞立约本体，属退出码 2 工具自身异常先处置位：对表修复随批（iso-07），修复前立约改走既立词批名通道，双跑不一致即本条申报。

## 八、关联文件 {#related}

- sih-engine/sih/event/plan/recognize-solo-results.md（随批产出）
- sih-engine/sih/event/plan/recognize-solo-materials/（意图件与验收材料）

## 九、请求写入 {#requested-writes}

- sih-engine/src/mcpserver/
- sih-engine/src/bin/lease/attachments.rs
- sih-engine/src/bin/lease/closegate.rs
- sih-engine/src/tools_registry.rs
- sih-engine/tests/recognize_http_gate.rs
- sih-tools/mcpline/AI-MANUAL.md
- sih-engine/sih/event/plan/recognize-solo-results.md
- sih-engine/sih/event/plan/recognize-solo-materials/
