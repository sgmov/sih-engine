# lockguard-solo：锁位守卫批

> task-packages 治理任务
> 承接：sess-zcode-260828-lockguard 三问意图即 2026-08-28 链事件 29451051、用户同日两令即「证据要落，另外锁的机制也要优化」
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-28

## 一、问题陈述 {#problem}

2026-08-28 链分叉事证暴露锁机制三缺口：书简写路径不读锁台账即他会话持锁照样落笔、持锁人迁走共享路径无路标义务、共享日链追加惯例只存台账考古无明文。同批事证未成档即仅存会话记忆。三缺口同源即守卫与惯例不在机制与载体，在自觉与记忆。

## 二、关键设计 {#design}

守卫加在书简写路径起点即 append 与 intent 与 park 增可选 --locks 与 --session 两旗标，台账含 active 锁于 trail 路径且非本会话即拒写退出码一并报持锁方，台账不可读即退出码二不静默，旗标缺席行为逐字节不变即向后兼容硬约束，向量回归绿。事证文档落 scribe reports 位含精确时序与三缺口与回放归位验证即 85f700ff 与 feb9114d 与 fa81dc2c 绑定三报告哈希逐一相符。惯例明文三步即先锁或查锁位、追加、验链入 AGENTS 与书简 skill，skill 陈旧工具侧旧调用一并修除即其自 2026-08-27 切换起仍教已退役调用形属错链隐患种子。租约本体零改动。

## 三、工作清单 {#work}

- [ ] lockgate 模块与 scribe 接线与测试与重编与向量回归绿
- [ ] 事证文档落盘含归位验证记录
- [ ] AGENTS 与书简 skill 根域直改披露在案
- [ ] 管线两件在域即化格检词，核阅零在域目标如实记
- [ ] 认证两笔走新旗标狗粮即先锁后加后验再解全环
- [ ] 任务包与结果档落位、双仓段结算收约、六链对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 守卫进机制 | 工程治理 | 他会话持锁时 append 拒退出码一报持锁方，本会话持锁放行，旗标缺席行为逐字节不变，向量回归绿，cargo 测试绿 |
| **F-2** 事证落盘 | 链上治理 | 事证文档在 scribe reports 位，回放三证与三报告哈希对表在档，本批链件走新旗标 |
| **F-3** 惯例明文 | 工程治理 | AGENTS 与 skill 三步惯例与调用形更新落位，双仓段结算与六链对表收口 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/ledger/locks.ndjson 即台账行形与 acquired 与 released 语义
- 必读 2：sih-engine/src/bin/scribe.rs 即写路径三命令现形

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 旗标缺席行为逐字节不变
5. 分叉行原件与回放件俱不删不改即历史不可改

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 结果档落 sih/event/plan/lockguard-solo-results.md，认证入链，双仓段结算收约

## 八、风险点 {#risks}

守卫改写路径即拒写误伤面。台账缺席或路径解析差异两类边界先钉即台账文件缺席视为零锁放行、相对路径按台账上三级为工作区根解析，测试各证一形。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-28 两令
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 29451051
- 关联：regconf-sdd-solo 结果档披露节即事证源起，pk021settle-solo 段3 即回放归位前案

## 十一、请求写入 {#requested-writes}

- sih-engine/src/bin/scribe.rs
- sih-engine/src/event_stream/lockgate.rs
- sih-engine/src/event_stream/mod.rs
- sih-engine/src/event_stream/tdd_tests.rs
- sih-engine/sih/state/plan/lockguard-solo.md
- sih-engine/sih/event/plan/lockguard-solo-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
