# pk031gap-solo：租约收敛性落差批即四证据定向修

> task-packages 治理任务
> 承接：sess-zcode-260830-pk031gap 三问意图即 2026-08-30 链事件 0e6e5546、用户同日双出泊令即 pk-031 出泊
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

泊件 pk-031 四证据在案：一缺省链清单只枚举引擎侧新家即 cert_missing 免参二加二而八链明示归零、迁链路标无机械位三次实证；二 active_sessions 无序弹同号即重开会被旧 revoked 误抹；三缺省根解析 parents[4] 在围堰即错即两处测试主线绿围堰红；四 commit 可指主检出即绕围堰直提无机械拒。

## 二、关键设计 {#design}

四件各证各立。一 _default_trails 枚举两居所并集即引擎新家加工具侧老家 sorted 去重，迁链路标即枚举位本身。二 active_sessions 改单遍事件序配对即 issued 置位 revoked 仅弹在册同号、重开再置位即旧 revoked 不再误抹。三缺省根解析抽 resolve_root 用双仓祖先搜上承 _gate_root 先例、找不到回落旧式 parents[4] 保显式环境可预测败。四 commit_staged 命中主检出条目即 CommitBlocked 拒并示围堰路径。lease 升 1.8.2 三源对齐。

## 三、工作清单 {#work}

- [ ] 四件先红后绿
- [ ] CONTRACT 修订与版本三源对齐
- [ ] pk-031 出泊 promoted 与名录更新
- [ ] 结果档与管线与认证入链
- [ ] 段结算收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 链清单 | 工程治理 | 免参 reconcile 枚举含两居所、老家链上的 cert 免参可寻、测试绿 |
| **F-2** 同号配对 | 工程治理 | issued revoked issued 序在册含新会话、时序错配拒、测试绿 |
| **F-3** 根与围堰 | 工程治理 | resolve_root 双仓祖先命中、commit 指主检出被拒并示副本路径、测试绿 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/parking/records/pk-031-enter.json 即四证据原文
- 必读 2：sih-tools/lease/src/lease/cli.py 与 core.py 即现场

## 六、约束 {#constraints}

1. 旧链零改写
2. 零动无关面
3. 词债不过夜
4. 上链前必须等绿
5. 并行批共享件按备份还原模式

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链、结算收约、reconcile unrouted 零

## 八、风险点 {#risks}

老家枚举增 reconcile 扫描面即链文件少代价可忽略。主检出直提拒可能挡 legit 场景即手动期批提交全走围堰、main 检出只归并的既定流程不受影响。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 双出泊令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 0e6e5546
- 关联：pk-031 泊件、closeidem-solo 即重复删支已修除名、DEC-013 第三步迁链

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[迁链路标]: 消解 即链迁居指向的直述工作名不作登记
叩问处置[副本路径]: 消解 即围堰工地路径直述不作登记
叩问处置[居所]: 消解 即链文件所在目录直述不作登记
叩问处置[误抹]: 消解 即缺陷描述语直述不作登记
叩问处置[租约]: 消解 即召回面信号经旧件在场、已立名不另记

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/
- sih-tools/lease/tests/
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/pyproject.toml
- sih-tools/lease/CALL-LOG.md
- sih-engine/sih/state/plan/pk031gap-solo.md
- sih-engine/sih/event/plan/pk031gap-solo-results.md
- sih-tools/parking/materials/pk-031-exit.json
- sih-tools/PARKING-v1.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/
