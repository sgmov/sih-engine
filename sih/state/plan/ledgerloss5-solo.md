# ledgerloss5-solo:会话台账丢失第五笔机制调查（只读取证批）

> 令源:用户 2026-09-06 令「丢失 #5 调查。」
> 范式:T6 单线 solo,委外代理亲写零子代理(主会拉子代理执行)
> 域:只读取证,零生产写,零代码修复(修复归后继批另立)

## 一、问题陈述 {#problem}

- 当日同根丢失五笔,前四笔(2026-09-05 docmath-b4 与 fixguard 与 gateswitch 与 constmodel)发生在 pk057fix 加固前,根因当时判定为运行时追加无锁与合并窗口交错,已由 lease 1.24.0(修订三十六:append_row flock 排他、ledger_lock 合并窗口互斥、restore_missing 回补网)机制收口
- **第五笔发生在加固全线在役之后**:watchcheck-solo 会话 445c12fbb3e4283c 两行于其自身收约窗口丢失,证据链已由主会钉死:
  - 15:27:05Z open 签发,issued 行落主树台账(issued_at 在档)
  - 15:57:45Z 收约前快照 e8d632da(sih-tools 仓 closeguard 提交)在册 1 行(issued)——主会 grep 亲核
  - 15:57:47Z 收约凭据 closed_at(state/plan/watchcheck-solo.lease-check.json,close_session 与 identity_core 三证齐)
  - 16:00:13Z 后继快照 2b7b1ce2(idenlane-human 收约前)该会话行 0——两行俱失
  - 丢失窗口即 15:57:45Z 至 16:00:13Z,恰为 watchcheck 自身 close 归并窗
- 加固三件套在役仍丢行,机制级缺陷未明,不查明则每次收约都是掷骰子

## 二、待证假设（调查对象,跑前立文不预设立场） {#hypotheses}

- **H-1 台账位错**:close 自工地 cwd 运行时 tool_dir() 随工地源码定位,revoked 行落工地位台账而主树台账被 git checkout branch 版整文件替换(分支切于 issued 行追加前即不含该行),union 回补网作用于工地位文件故主树未救——openhyg 活体验收首跑即踩 tool_dir 工地定位坑(其结果档申报一在档,先例同族)
- **H-2 回补网缺陷**:restore_missing 比对键或备份路径错位(备份取错文件、restore 差集算错),主树行未回补
- **H-3 并发覆盖**:16:00 前后有他会话活写交错,锁序未覆盖该窗口(pk057fix 只锁单写点与合并窗,若 close 链内有一步绕开 ledgerwrite 直写即出网)
- **H-4 git 层整文件替换**:close 归并用 git merge/checkout 对 ndjson 整文件换版,纯追加并集超集判定(修订二十八)未触发或触发后超集计算漏行

## 三、工作清单 {#work}

- [ ] T-1 时间轴矩阵:逐提交(15:27Z 至 16:06Z 全部 sih-tools 仓提交)sessions.ndjson 行数与 watchcheck 行在否矩阵,含 worktrees/.close-backups 与工地残件考古(revoked 行去向定位:主树从未有/工地位曾有/从未写)
- [ ] T-2 close 路径源码走读:cli close 与 core close_session 与 allow_and_merge 与 is_pure_append_conflict 与 ledger_lock 与 restore_missing 逐行对时间轴,标出每一步操作的主树/工地文件位
- [ ] T-3 沙盒复现:materials 内建临时 git 仓与最小台账,按 H-1 至 H-4 各构造复现序列,行丢失可机械重现即红证在档(生产台账零触碰)
- [ ] T-4 根因钉死:唯一或复合根因落代码行级,与 H 表对表,证伪项如实记
- [ ] T-5 修复方案呈报:修复方向与影响面落结果档呈用户裁(本批零修复实装,修复批另立过一裁)

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 时间轴 | 全提交行数矩阵在档,revoked 行去向有定论(三选一:主树从未有/工地位曾有/从未写) |
| **F-2** | 复现 | 沙盒仓机械复现行丢失,复现序列与生产时间轴对表可重放 |
| **F-3** | 根因 | 代码行级定位,H 表四项各判成立或证伪 |
| **F-4** | 零生产写 | 生产台账与 locks 与 trail 除本批自身链笔与认证外零写,沙盒件不出 materials |
| **F-5** | 修复呈报 | 修复方向落档呈用户,零实装 |

## 五、必读文件 {#read}

- 事故证据:本包问题陈述节(主会 grep 亲核在案)与 sih-engine/sih/state/plan/watchcheck-solo.lease-check.json
- 加固实装:sih-tools/lease/src/lease/ledgerwrite.py 与 core.py 与 lockcore.py(修订三十六)
- 归并机制:closeguard 相关(修订二十八与二十九与三十)与 cli.py close 路径
- 同族先例:openhyg-solo 结果档申报一(tool_dir 工地定位)、pk057fix-solo 结果档(pk057 附记掩败教训)
- 机械链:sih-tools/BATCH-FACE.md(含全部勘误节)

## 六、约束 {#constraints}

1. 生产零写:生产 sessions/locks/bypass 台账与 locks.db 与主树源码零改动;本批自身链笔与认证除外
2. git 考古只读(log/show/fsck/reflog 零变更命令);沙盒复现仓建在 materials 内,收约前可留档或清场如实记
3. 零代码修复:lease 源码零改,修复批另立
4. 与 idenlane-guard-solo(批 C)并行:批 C 写 hooks 与 guardcore 与 CONTRACT,本批零代码写,写面不相交;共享面(scribe/CALL-LOG 与 scribe/reports 与 identity/reports)按 wait-turn 排队
5. gvec-v2-serial(c41ce2af)在途持锁,open 预检拦即排队候位不抢不绕
6. scribe 写入裸调逐笔 grep 验证,禁 meter 包裹掩败

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle + close + reconcile + verify(结果档与 materials 为本批仅产出)
- [ ] 结果档 sih-engine/sih/event/plan/ledgerloss5-solo-results.md 与 materials/
- [ ] CALL-LOG 双笔随批

## 八、风险点 {#risks}

- 工地已拆则工地位台账考古受限:worktrees/sih-tools/watchcheck-solo 已删,靠 .close-backups 与 git 考古与快照对表
- 时间轴证据若有缺口,如实记缺口不脑补
- 沙盒复现若四假设俱不能复现,即证第五笔另有机制,如实申报进入二轮

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线(本批零判定语义变更无裁点)、双仓同步。

## 十、关联文件 {#related}

- 前四笔:2026-09-05 各批事故申报(结果档与 CALL-LOG 在档)
- 加固批:pk057fix-solo(lease 1.24.0)
- 后继:修复批(本批 T-5 呈报后用户裁另立)

## 十一、请求写入 {#requested-writes}

- sih-engine/sih/event/plan/ledgerloss5-solo-results.md 与 materials/(含时间轴矩阵、走读注记、沙盒复现件)
- sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/2026-09-06.ndjson 与后续日链
