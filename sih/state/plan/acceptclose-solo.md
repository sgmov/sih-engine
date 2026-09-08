# acceptclose-solo：主窗验收收口批（CALL-LOG 收编＋名册对表＋计账归置）

> 治理任务包（立文类＋归账类合一，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：用户 2026-09-08 令「执行 acceptclose-solo 批（主窗验收收口批）」三腿；bypass 台账 2026-09-08 两笔点名候归属窗
> 日期：2026-09-08

## 一、问题陈述 {#problem}

- CALL-LOG 族七件（facet/lease/scribe/selector/attractor/critsweep 六册投影与 calls.ndjson 权威腿）系前窗批（reroute/parktune/orphanexec/sitruling/pk063split）收约后 call-log append 直改车道活写，bypass 台账两笔点名候主窗收口，现悬无主态
- PARKING-v1.md 名册投影对 09-05 以来密集进出泊（十余笔出泊与 pk-077/pk-078 等进泊）零对表，漂移未量；出泊件材料经 P101 仍计 mainline 的包面语义无呈报记录
- closeguard 六笔 pre-close 提交（critsweep/reroute/sitruling/pkexits2/pk063split/constclear2 六批）未入 bypass 台账，reconcile unbypassed 计数悬高

## 二、关键设计 {#design}

- 三腿全归置零新功能：收编即提交现状字节（calls.ndjson 753 行只认既有 append 内容，收编前后 SHA256 对表证零重写）；名册对链即 09-05 至 09-08 全部 parking_entered 与 parking_exited 事件逐笔对名册，漂移逐笔修正，主线计数语义三选项呈报候人裁（甲 出泊件留册计 mainline 维持现状／乙 出泊件材料 state 改 exited 加谓词分轨／丙 出泊材料归档移位），缺省落注不改谓词；六笔 sha 经 lease bypass 逐笔登记
- 全部六笔 pre-close 提交与七件 CALL-LOG 面均引擎仓与工具仓归属已核（六笔全 engine，CALL-LOG 面 tools）

## 三、工作清单 {#work}

- [ ] ask3 双门、叩问 digest、正身、租约、意图入链
- [ ] 腿一：calls.ndjson 收编前后哈希对表，七件复制入工具仓工地随批提交
- [ ] 腿二：链面泊事件全清单提取（09-05 至 09-08），名册对表，漂移修正入引擎工地 PARKING-v1.md，计数语义落注与呈报
- [ ] 腿三：六笔 bypass 登记
- [ ] 结果档、管线三步、checkcite、认证、双仓 settle、放锁 close、对账对表、verify、回填、CALL-LOG

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 七件收编 | 数据治理 | 收编后七件 git status 净；calls.ndjson 收编前后 SHA256 一致即零重写；行数 753 只增由 append 命令承载 |
| **F-2** 名册对链 | 数据治理 | 09-05 至 09-08 泊事件全清单与修正后名册逐笔一致，对表材料含修正前后 diff；计数语义三选项呈报在档，谓词包零改动 |
| **F-3** 计账归置 | 治理 | 六笔 sha bypass 台账逐笔在册，reconcile unbypassed 较批前降 6（engine 侧） |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-5** 链面全绿 | 治理 | 双仓 settle 在档，close 零失败，verify valid，reconcile unrouted 零新增 |

## 五、必读文件 {#read}

- `sih-engine/doc/governance/PARKING-v1.md`（名册现文）
- `sih-tools/lease/ledger/bypass.ndjson`（两笔点名与既有格式）
- `sih-tools/BATCH-FACE.md` bypass 台账坑位与台账写点纪律

## 六、约束 {#constraints}

1. 零子代理；calls.ndjson 禁整文件重写；谓词包零触碰
2. 主树零直写（七件收编经工地随批提交）；守卫在位禁 plain commit；禁管道掩退出码
3. confpreempt 面零触碰；watch 其余无主件不代清；anchor.py 与 .zcode 与 .session-anchor.md 零触碰
4. 正身件保持主树在场直至 unlock 毕
5. 出泊裁决零预设，计数语义候人裁

## 七、验收标准 {#acceptance}

F-1 至 F-5 全过；收约后零本批活跃锁零活跃会话且七件 git 净；结果档 acceptclose-solo-results.md 落 event/plan。

## 八、风险点 {#risks}

- 并行批若在本批期间追加 calls.ndjson，收编快照与合并即差量——以收编时点哈希对表如实申报，差额归 append 命令承载不手工补
- close 无主闸或 CALL-LOG 闸再拦即按闸三通道处置，本批 allow 面已含七件应免

## 九、范畴排除 {#exclusions}

- 计数语义不裁只呈报；pk-053 清账与 confpreempt 收口不属本批；投影腿再生（render）不跑

## 十、请求写入 {#requested-writes}

- sih-tools/facet/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/selector/CALL-LOG.md
- sih-tools/attractor/CALL-LOG.md
- sih-tools/critsweep/CALL-LOG.md
- sih-tools/calllog/calls.ndjson
- sih-tools/lease/ledger/bypass.ndjson
- sih-engine/doc/governance/PARKING-v1.md
- sih-engine/sih/state/plan/acceptclose-solo.md
- sih-engine/sih/event/plan/acceptclose-solo-results.md
- sih-engine/sih/event/plan/acceptclose-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
- worktrees/sih-tools/acceptclose-solo
- worktrees/sih-engine/acceptclose-solo
