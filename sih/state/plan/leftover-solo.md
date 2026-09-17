# leftover-solo:遗留归位批（未进版控面清账与工地半拆查实）

> 令源:用户 2026-09-06 转 watchcheck 代理审计报告（逐件归属判定:主责执行代理纪律,另立租约硬化批修工具双缺口,本批承载归位与查实）
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:与 declguard-solo（声明守卫批）姊妹批,建议本批先行后批随后
> 前置:锁面查实（toolincub-solo 在途避让）,195 基线

## 一、问题陈述 {#problem}

八件面未进版控与一处工地半拆,俱为执行代理纪律失守产物（审计逐件归属在案）:

- kernelmerge 结果档与材料（engine event/plan 面未跟踪三件）:批正常结算但成文写主树未走工地
- PROB-018/019 条目文件（sih-math）:INDEX 行已提交同目录条目文件漏加
- idenlane 两份结果档（idenlane-solo-results.md 与 idenlane-envelope-solo-results.md）:同 kernelmerge 形
- docmath 任务包副本与 gvecmath 任务包副本及 lease-check 件:漏加或错位
- billwire 双层嵌套错径（sih-tools/lease/.sih-tools/scribe/reports/ 下测试件）:cwd 相对路径拼接错
- rootanchor 双工地半拆:两会话俱规范吊销而工地目录在盘（git worktree 注册已拆目录未拆）,成因待查实
- lease-check.json 全族:引擎仓零 settle 在案（工具自产无主件,归属修法归 declguard-solo,本批只清点）

## 二、关键设计 {#design}

- 归位通道:各件按所属仓 direct 笔或收约补笔先例收编进版控,逐件 commit 号入档;错径件先归正再收编
- rootanchor 工地查实:核对会话 revoked 与 worktree 注册态与目录残留,定论成因（close 半失败或路径形不完整）落档后拆除双目录,git worktree list 复核净态
- lease-check 全族清点:列全清单与所在仓,处置归 declguard-solo 承接（协同条款）,本批零迁移
- 在途避让:toolincub-solo 在途即排队,其面零触碰

## 三、工作清单 {#work}

- [ ] T-1 八件面逐件归位进版控（错径件先归正）
- [ ] T-2 rootanchor 双工地查实定论加拆除加 worktree list 净态复核
- [ ] T-3 lease-check 全族清点清单落档（移交 declguard-solo）
- [ ] T-4 全工作区 untracked 面复扫读数（收尾后剩量如实,lease-check 族除外）
- [ ] T-5 watchcheck 复跑读数与双仓对表（本批零代码即零测试回归面）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 归位 | 八件面逐件 commit 号在档,所属仓 status 对应路径零 untracked |
| **F-2** | 工地查实 | rootanchor 半拆成因定论落档（证据链在案）,双目录拆除,worktree list 净态 |
| **F-3** | 清点移交 | lease-check 全族清单落档零迁移,移交指针明确 |
| **F-4** | 复扫 | untracked 剩量读数如实（预期仅剩 lease-check 族与在途批面）,watchcheck 复跑净态或如实呈报 |
| **F-5** | 零代码 | 本批零源码改动零 CONTRACT 修订 |

## 五、必读文件 {#read}

- 审计报告（用户转,逐件归属表）
- 归位先例:收约补笔与 direct 笔先例（BATCH-FACE 与 CALL-LOG 各批行）
- 工地先例:newcarr 三工地拆除先例与 rootanchor 结果档
- 机械链:sih-tools/BATCH-FACE.md

## 六、约束 {#constraints}

1. 零代码零判定语义变更零一裁（纯归位与查实）
2. 在途批（toolincub-solo 等）面零触碰,锁面实查撞即排队
3. rootanchor 拆除前必须核该会话 revoked 在册与无锁持有,证据齐才拆
4. lease-check 族本批零迁移零清理（归 declguard-solo）
5. scribe 裸调逐笔 grep;退出码真值核;verify 一律主树二进制;任务包绝对路径
6. 归位提交走 direct 笔或收约补笔先例带 bypass 登记如实留痕

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle + close + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/leftover-solo-results.md 与 materials/
- [ ] CALL-LOG 双笔随批

## 八、风险点 {#risks}

- 归位件内容若与他批在途改面相交:撞即让位排队不硬收
- rootanchor 工地内若有未提交残留内容:先 diff 留档再拆,零静默丢弃

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、双仓同步。

## 十、关联文件 {#related}

- 姊妹批:declguard-solo（工具双缺口）
- 上游审计:watchcheck 代理 2026-09-06 归属判定

## 十一、请求写入 {#requested-writes}

- 各未跟踪件归正与收编（所属仓提交）
- worktrees/rootanchor-solo 双目录拆除
- sih-engine/sih/event/plan/leftover-solo-results.md 与 materials/
- sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/ 当日链
