# pk057fix-solo：台账写点唯一化与 flock 串行（pk-057 根因硬化）

> 令源：用户 2026-09-05「同意，跑这个修复方案」；当日台账重写竞态四连发（docmath-b4、gateswitch、constmodel 缺录、facepark 远因）
> 范式：T6 单线 solo——主会亲跑，零子代理
> 承接：pk-057 出泊条件「用户裁硬化立项」本令达成，出泊随本批落账

## 一、问题陈述 {#problem}

- 台账（sessions/locks ndjson）运行时追加无文件锁：大行（长 allow 面）有撕裂风险，合并窗口无互斥。
- 收约合并机械「备份→checkout 分支快照」窗口与并发追加重叠即丢行——今日四例同根（basefix 的并集超集补笔网只兜部分形态）。
- 无统一写点：core.append_event 与 lockcore.append_event 两套同形代码，新写位易绕行。

## 二、关键设计 {#design}

- 新模块 `ledgerwrite.py`：`append_row(path, row)`（flock 排他 + 单次 os.write 整行原子落盘）与 `ledger_lock(path)` 上下文管理器（合并窗口互斥，锁体为 path+".lock"）。
- core.append_event 与 lockcore.append_event 改为薄委托 ledgerwrite.append_row（行格式字节不变，零迁移）；合并窗口（备份→checkout→回补）套 ledger_lock。
- 回补网：checkout 后比对备份行集，缺失行经 append_row 逐行回补（窗口期活写行的保险带）。
- 会话现势表入 lockdb 列为二期，本批不做（一期 1+2 即杀死竞态）。

## 三、工作清单 {#work}

- [ ] lw-01：ledgerwrite.py 实装（append_row + ledger_lock）
- [ ] lw-02：两处 append_event 委托改造 + 合并窗口互斥 + 回补网
- [ ] lw-03：并发压测与行为测试（多进程零丢行、flock 互斥、回补网）
- [ ] lw-04：CONTRACT 1.24.0 修订三十六与 BATCH-FACE 勘误
- [ ] lw-05：live 并发自证 + pk-057 出泊 + 全链收口

## 四、可证伪条件 {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 并发压测 | 8 进程 × 25 行并发追加，行数精确 200、逐行合法 JSON、零交错零丢行 |
| **F-2** | 互斥 | 合并窗口持锁期间 append_row 阻塞、释放即续，无交错 |
| **F-3** | 回补网 | checkout 覆盖后备份缺失行被逐行回补（构造用例验证） |
| **F-4** | 零迁移 | 行格式字节不变（既有台账全史可读，active_sessions 配对语义不变） |
| **F-5** | 回归 | lease 全测试族绿 + 全引擎仓测试不涉 |
| **F-6** | 收口 | 双仓 settle、close、reconcile 零新增、verify valid；pk-057 出泊 promoted 落链 |

## 五、必读文件 {#read}

- sih-tools/lease/src/lease/core.py（append_event 与合并机械）
- sih-tools/lease/src/lease/lockcore.py、lockdb.py
- sih-tools/lease/CONTRACT.md（1.23.0 现文）
- 泊件：sih-engine/sih/state/parking/materials/pk-057.json

## 六、约束 {#constraints}

1. 零判定语义变更：行格式、接受/拒绝行为、active_sessions 配对语义全部不变；实现中若发现任何判定行为必须改动，即停批过得一裁
2. 零迁移：既有台账不动，新写位只向前生效
3. 会话现势表入 lockdb 列二期，本批零触碰
4. 锁体文件（path+".lock"）不入版控面申报（gitignore 由仓内既有规则覆盖 ledger 目录）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] CONTRACT 1.24.0 与 CALL-LOG 双笔随批
- [ ] pk-057 出泊事件裸调落链 grep 验证

## 八、风险点 {#risks}

- flock 在 NFS 类文件系统不可靠：本仓为本地 APFS，如实声明适用边界
- 合并窗口持锁过长会阻塞并发追加：窗口内只有文件操作，毫秒级，实测申报

## 九、范式偏离声明 {#deviation}

并发正确性的红证难以稳定构造（丢行是概率性），本批 TDD 以行为契约测试与压测代替严格先红后绿，偏离如实申报。

## 十、关联文件 {#related}

- sih-engine/sih/event/plan/pk057fix-solo-results.md（随批产出）
- sih-engine/sih/state/parking/materials/pk-057.json（出泊随批）

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/ledgerwrite.py（新增）、core.py、lockcore.py
- sih-tools/lease/tests/（新增 test_ledgerwrite.py）
- sih-tools/lease/CONTRACT.md（1.24.0）、sih-tools/BATCH-FACE.md
- sih-engine/sih/state/parking/materials/pk-057-exit.json
- sih-engine/sih/event/plan/pk057fix-solo-results.md 与 materials/
- 双仓 trail 与 CALL-LOG 双笔
