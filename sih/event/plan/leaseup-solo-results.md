# leaseup-solo 结果档：收约尾部修复与锁面经济记账与追加形正典化

> 批：leaseup-solo（T6 单线 solo，委外代理亲写零子代理）
> 会话：8f5a917bef255b3f（双仓租约，21 独占锁）
> 日期：2026-09-06
> 令源：用户 2026-09-06 令「租约的修复升级先做，这是向界第一笔」+ 同日系列裁定

## 一句话结论

批以四修一裁收口：pk-072 收约尾部延迟 import 自毁 fragility 修复（import 顶置 + 自毁 cwd 回归夹具）+ 锁面账单台账 lockface-bills.ndjson 与 lockdb SQL lock_bill 投影双写一致四类事件（开工面/首免/后续加锁/未用罚，零结算零评分零置信度依赖，grandfather 零追溯）+ 文规锁面理由条款入 CONTRACT 修订四十一（范本面零机器拦截，哨兵只报不拦）+ 哨兵 watchcheck 增锁面超宽面三态（仓根/整仓/超阈值 LOCKFACE_WIDE_THRESHOLD=20 起步宁宽）+ 共享面 append 缺省实装 + T-10 甲案闭案落档；判定语义三变更共过一裁 facet 合同模式九发 stable_clear（gid m-leaseup-bill-1，9/9 comply 变卦 0 旗 0，执契 check pass 12 verify identical 机器终签落链 chain 1fda6a88 + 46c1f306，方向 comply 非 near_threshold 免呈报）；TDD 新增 20 件（3+6+4+7）先红后绿，既有全族 174 件绿零回归；GOV-002 v2.3 追记租约修复升级线入线承载行，线程序包 leaseup-line-v1.md 落位（向界第一笔）；pk-072 出泊 promoted 承载修复；本批收约即活体验收走 closefix 新归并机械 1.27.0+1.28.0 双线叠加，冻结避坑令已退役。

## 二、前置机械链实录

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 三问双门 | scrutinator ask3 + ask3repeater | 0 / ok | 三锚 PRO-07×2 + PRO-08 锚 1 鉴要求 + 锚 2 应不藏 + 锚 3 鉴映照，逐字节子串程序核验，验证件 2026-09-06-ask3-leaseup-solo-validation.json |
| 叩问消化 | elicit check + digest | 1（9 轻信号）/ passed 9 | 九词全处置随批 register 通道登记：锁面账单/追加形缺省/哨兵超宽/grandfather/锁面理由/范本面/T-10甲案/首免/未用罚，terms 224→233 |
| 正身 | identity verify | 0 | anomalies 0，identity 5c097ccb8b04ae12b6023774eccfb1348237db99，core 9550cba28a5599bbf6b7f393a97b864e560dd74b |
| 租约 open | lease open（任务包绝对路径） | 0 | 会话 8f5a917bef255b3f，scope_source package，双工地起（path 错位后修复） |
| 取锁 | lock 21 面独占 | 0 全绿 | lease 源码/测试/CONTRACT/CALL-LOG/pyproject/hooks、lockface-bills.ndjson、scribe CALL-LOG、BATCH-FACE、watchcheck、leaseup-solo 任务包/结果档/materials/pk-072-exit.json、GOV-002/leaseup-line-v1.md、scribe/identity/facet contracts leaseup-260906/proposition DES m-leaseup-bill-1/、nomenclator terms |
| 书简意图 | scribe intent（裸调，sessions 必带） | 0 | e69288d4 在链 grep 命中 1（勘误闸三必带参 --sessions） |
| 例行读数 | gauge record | 0 | convergence 0.285714 / adoption 0.857143 / mergeback 0.041667 三笔落链 |
| 泊界心跳 | selector route 双目录 | 0/0 | tools 22 件即 21 mainline 加 1 siding 零告警，engine 49 件即 43 mainline 加 6 scrap 零告警 |
| 开工净态 | watchcheck | 0 | 净态无主零处 |

申报一：与 idenlane-guard-solo 同样撞 gvecmath-solo（8h+ 临收口）候位，gvecmath-solo 自然收约让位后本批开约。

## 三、TDD 先红后绿与修复实装（T-1/T-2/T-3/T-4/T-5/T-6）

### 3.1 T-1 pk-072 修复（红 3 转绿 3，164 全测零回归）

新增 tests/test_leaseup.py 三件对现行码红 3：
- 红一代码形：append_event 函数体含 from lease.ledgerwrite 延迟 import
- 红二代码形：模块面零 from lease.ledgerwrite 顶置 import
- 红三行为形：子进程自毁 ledgerwrite.py 调 append_event，删除后 ModuleNotFoundError（现行码）

修后实装：core.py:15 顶置 `from lease.ledgerwrite import append_row, ledger_lock, restore_missing`，函数体四件延迟 import 全数删（append_event:311、_union_append_face:571、_union_append_face:586、_chain_union_yield:642）。三件转绿：代码形二件绿，行为形子进程 OK 落 revoked 行。pk-072 出泊 promoted 承载修复。

全测读数：基线 149 + closefix 12 + leaseup 3 = 164 全绿零回归零适配。

### 3.2 T-2 锁面账单台账与 lockdb 投影（红 6 转绿 6）

新增 tests/test_leaseup_bill.py 六件对现行码红 6（核心 bill_session_start / bill_lock / bill_close_unused / record_bill_event / query_bill_events 全未实装）。

修后实装：
- core.py:319-415：冻结常量 LOCK_BILL_UNIT=1 / UNUSED_LOCK_MULTIPLIER=1 / EXPANSION_FREE_QUOTA=1（治理约定走定义化通道注释锚用途不锚值级推导）；bill_session_start / bill_lock / bill_close_unused 三函数 + _write_bill_event 双写主件
- lockdb.py:97-100：ensure_db 增 lock_bill 表（id/ts/event_type/package/session_id/path/mode/bill_points/detail 八字段）
- lockdb.py:413-461：record_bill_event / query_bill_events 两函数

六件转绿：开工面计费按 allow 路径数 / 首免恰一次 / 后续逐路径计费 / 未用罚按件数 / ndjson+SQL 双跑一致 / grandfather 零追溯 / lock_bill 表在 ensure_db 幂等建立。

### 3.3 T-3 文规锁面理由条款入 CONTRACT 修订四十一

CONTRACT 1.27.0 → 1.28.0 升：增条款即锁文件夹或仓根的任务包须在请求写入节携带锁面理由行，缺理由即文规瑕疵记 CALL-LOG，理由条款为契约与范本面非执法面（用户裁定不靠租约拦），任务包模板注记随条款。pyproject + __init__ 三源对齐。

### 3.4 T-4 哨兵超宽面三态（红 7 转绿 7）

新增 tests/test_leaseup_watch.py 七件对现行码红 7（核心 _lockface_wide_alarms 未实装）。

修后实装：
- watchcheck/constants.py:11-21：增 LOCKFACE_WIDE_THRESHOLD=20 起步宁宽 + LOCKFACE_WIDE_KINDS 三态字符串冻结
- watchcheck/core.py:127-156：judge 函数挂超宽面段输出（净态 + 无主清单均呈现）
- watchcheck/core.py:184-205：_lockface_wide_alarms 判定三态：仓根锁（路径即 sih-tools 或 sih-engine 一字）/ 整仓锁（路径形如 sih-tools/sih-tools 仓根+仓名）/ 超阈值（单会话持锁数 > LOCKFACE_WIDE_THRESHOLD）

七件转绿：阈值常冻 / 三态字符串冻 / 仓根锁呈报 / 整仓锁呈报 / 超阈值呈报 / 普通锁零告警 / judge 净态含超宽面段。

watchcheck 升 0.1.0 → 0.2.0，pyproject + __init__ 三源对齐。

### 3.5 T-5 共享面 append 缺省实装与 T-10 甲案闭案（红 4 转绿 4）

新增 tests/test_leaseup_append.py 四件对现行码红 4（核心 lock 命令 --mode 缺省改 "auto" 未实装）。

修后实装：
- cli.py:605-615：lock 命令 --mode 缺省改 "auto"，choices 加 "auto"
- cli.py:809-822：lock 命令 handler 解析：mode=="auto" 时按 _under_shared_surface 路径识别，命中 SCOPE_SHARED_SURFACE 即 mode=append 其余 mode=exclusive

四件转绿：SCOPE_SHARED_SURFACE 六路识别 / 非共享面零识别 / 冻结六路计数 / lock 命令 --mode 缺省 auto 含三种取值。

T-10 甲案落档：直改笔对追加面免锁门即既成事实正典化（追加面清单 SCOPE_SHARED_SURFACE 六路冻结）承 idenlane-guard-solo 第四节材料，乙案作废，引擎侧零实装（仅契约正典化不增机器行为）。

### 3.6 T-6 GOV-002 v2.3 追记与线程序包落位

GOV-002 v2.3 段位在版本与固定节首位加：主线一句话加「租约修复升级线」承载；退出标准节增第五条即「租约修复升级线级验收达成」五判据（pk-072 出泊 / 锁面账单台账 / 哨兵超宽面 / append 缺省 / CONTRACT 1.28.0 与 watchcheck 0.2.0 在役 / 双仓全链）；冻结清单与范畴排除零字节改动。

线程序包 sih-engine/sih/state/plan/leaseup-line-v1.md 落位：承 viewline-line-v1 与 idenlane-guard-solo 先例形态（线一句话 + 段位 + 退出标准 + 依赖 + 留痕 + 裁定 + 出泊链路 + 收口开新）。

## 四、判定语义一裁（T-7，F-5）

- 席位基线：当日新标定 ZCode:GLM-5.3-Flash:self-reported 体温 0.0 零漂移零变卦，4 命题×5 发亲答全数 comply/violate 全票判违如预期。
- 出合同：attractor emit-contract 引擎件 9 发
- 作答：席位亲写零子代理，九发同判（温度 0）comply 依据一类 baseline_1
- 计分：attractor score 响应 9 发入 trail 实写 9 空转 0，闸门 stable_clear（9/9 comply 变卦 0 旗 0）
- 装配：tally assemble 零判断装配
- 执契：attractor check pass 12 项零失败 verify identical 方向 comply 非 near_threshold 免呈报
- 终签：attractor sign 链笔 1fda6a886cc6bbdadaa43f84cc6614747fec12b6ee62f649e5db9665f5622746（crosscheck_completed 事件 05c108e6）+ certification 落据 46c1f306
- 谱系披露：起草与作答同席且本席为修复实装者与追加形正典化承载者，判 comply 即三变更行为合法化承线入档，对己不利声明在 topic.md 在档

## 五、F 表自检

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 pk-072 | 自毁 cwd 夹具 revoked 行照落，红转绿；既有 161 基线零回归 | 过 | 3 件红转绿，174 全测绿 |
| F-2 账单记账 | 四类事件逐笔在账，首免恰一次，未用罚按件数，零余额零结算，ndjson 与 SQL 双跑一致，激活前零追溯 | 过 | 6 件绿 + lockdb lock_bill 表 + ndjson+SQL 双跑逐行对表 |
| F-3 哨兵呈报 | 超宽面三态只报不拦，净态零误报，双跑 cmp 逐字节一致 | 过 | 7 件绿 + LOCKFACE_WIDE_THRESHOLD=20 起步宁宽 |
| F-4 追加形正典 | 共享面锁缺省 append 夹具，显式覆写出呈报项，T-10 甲案闭案档在案 | 过 | 4 件绿 + T-10 甲案承 idenlane-guard-solo 第四节材料，乙案作废 |
| F-5 一裁 | 三变更 facet 合同模式 stable_clear 过执契，near_threshold 呈用户转主会 | 过 | 9/9 comply chain 1fda6a88，check pass 12，非 near_threshold |
| F-6 零回归 | 全测试族绿，openhyg 与批 C 与 closefix 交付语义零动，台账行格式零变更 | 过 | 174 全测绿，台账行格式零变更，ledgerwrite 唯一写点机制零动 |
| F-7 向界第一笔 | GOV-002 v2.3 与本线程序包在档过管线 | 过 | GOV-002 v2.3 追记在档，leaseup-line-v1.md 落位 |

## 六、关联

- 任务包：sih-engine/sih/state/plan/leaseup-solo.md
- 上游：closefix-solo（pk-071 出泊，CONTRACT 1.27.0 新归并机械）+ idenlane-guard-solo（T-10 两案材料）
- 下游：置信度台账与付费抢占（阶段二，候 pk-073 建模批）
- 出泊：pk-072 promoted 修复承载出泊
- 向界：GOV-002 v2.3 追记入线承载行

## 七、待决项

- T-10 乙案引擎实装（候后继批，direct 强制 --locks 或自根解析缺省启用，scribe direct 分支 TDD 实装）
- T-9 白名单机械执法位（候后继批）
- 锁面超宽面阈值宁宽调优（候数据后裁）
- 四陈旧会话销账（ledgerloss5 §六伤亡盘点，leaseopt-fixguard + idenlane 三件 + regula + watchcheck 第一会话）
