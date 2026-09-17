# toolhyg-solo 批任务包：工具卫生三件——callloghyg 扩面与 confpreempt 残行处置与 zcode 死旗标漂移记泊

> 令源：用户 2026-09-09 令「继续。多子代理协作」；三件俱队列项：callloghyg（每批 close 双旗 bypass 的根源）、confpreempt 会话残行（本晨摘 worktree 时确认零锁在册）、zcode 死旗标（mcpcold 批漂移申报候工具线处置）
> 形：solo 批独立立约独立收约，与 mcpsec-solo、specfix-solo 并行

## 一、使命三件

**件一 callloghyg 扩面**：sih-tools/lease/src/lease/guardcore.py 的 DIRECT_LANE_FILE_WHITELIST 增两卷 attnanchor/CALL-LOG.md 与 attractor/CALL-LOG.md（21 至 23），消每批 close 双旗 bypass-orphan 与 bypass-calllog 的根源。additive-only；lease 版本号：优先不 bump（行为面 additive 白名单）；若测试契约强制 bump 则全测试族含冻结向量重跑（recclsf 先例：版本嵌入报告，bump 批必重跑全族）。

**件二 confpreempt 残行处置**：会话 09326a760119e4ed（2026-09-07 confpreempt-solo，零锁）停滞在册，经 lease 正道子命令处置落链（takeover 或 ledger-repair 择合规者，事由照录）；禁手改台账 ndjson；正道拒即红证候裁不硬来。

**件三 zcode 死旗标漂移记泊**：zcode 0.16.5 帮助面 --allowed-tools 与 --max-turns 与 --settings 三旗标解析器拒识（mcpcold-solo 批申报），记 sih-tools/PARKING-v1.md 新泊位条目（pk 序号顺延）加 scribe park 停泊笔落链；泊位 context 照录申报原文与复现实测。

## 二、写入面（allow 清单）

- sih-tools/lease/src/lease/guardcore.py 与 sih-tools/lease/tests/（仅件一所需）
- sih-tools/attnanchor/CALL-LOG.md 与 sih-tools/attractor/CALL-LOG.md（批内若被写随批收）
- sih-tools/PARKING-v1.md 与停泊记录件（件三）
- sih-engine/sih/state/plan/toolhyg-solo.md 与 toolhyg-solo-prompt.md
- sih-engine/sih/event/plan/toolhyg-solo-results.md 与 toolhyg-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-toolhyg-1/

禁触：lease 其余模块、critsweep、gauge、scribe、mcpline、sih-engine/doc/**。

## 三、机械链序与铁律

- 全序照 sih-tools/BATCH-FACE.md verbatim；每条命令立即取退出码，失败即停，禁管道掩码
- 并行三批共用今日链：锁面含 trail 路径与 lease 源码路径，冲突走 lease wait-turn，禁绕行禁 preempt
- sih-tools 域外于引擎文档规范；PARKING-v1.md 条目形照现有条目 JSON 形不越
- scribe 二进制一律主树 target/debug；commit 须指向登记 worktree

## 四、验收

- 件一：全测试族绿（版本若 bump 则含冻结向量全族重跑）；此后 close 对 CALL-LOG 两册不再出 bypass 双旗（以本批自身 close 实证）
- 件二：会话残行处置落链有据或红证候裁
- 件三：泊位条目与停泊笔在链
- 认证上链、双仓 settle、reconcile 双零
- 完工回报形：批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、三件各自终值
