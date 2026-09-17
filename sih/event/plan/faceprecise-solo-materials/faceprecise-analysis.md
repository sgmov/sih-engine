# pk-074 faceprecise-solo 数据面分析：锁面超宽阈值实证与未用罚口径差值实证

- 批号：pk-074 faceprecise-solo（数据面分析腿，只读）
- 产出日：2026-09-17
- 数据源（全部只读）：
  - `sih-tools/lease/ledger/lockface-bills.ndjson`（1699 笔：open_face_bill 152 / lock_charged 1374 / lock_free 142 / unused_lock_penalty 31；计分面 3401 点 = 开工面 1903 + 加锁 1374 + 未用罚 124）
  - `sih-tools/lease/ledger/locks.ndjson`（9295 行：acquired 4722 / released 4571 / checked 2）
  - `sih-tools/lease/ledger/sessions.ndjson`（1114 行：issued 467 / revoked 470 / close_failed 177）
  - 文件系统 mtime 证据（os.path.getmtime，零写动作）
- 方法声明：全程零仓内写、零 git、零 lease 动作；中间产物与本文均落 `.tmp/pkexits3-work/`。

## 一、超宽阈值实证（LOCKFACE_WIDE_THRESHOLD=20）

### 1.1 宽度取法

账单行结构先读后定：`open_face_bill` 携带 `path_count`（开工 allow 面宽，多路径宽度的唯一载体）；`lock_charged`/`lock_free` 每笔单路径（构造上宽度恒 1）；`unused_lock_penalty` 携带 `unused_count`（罚宽非锁面宽）。故逐笔宽度分布取 `open_face_bill.path_count`；哨兵真实测度（单会话持锁数）另按 `watchcheck.core.lock_face` 精确语义（(path, session_id) 配对、塌缩 path 键 dict、文件序逐行重放）对 locks.ndjson 全量回放取每会话历史峰值。

### 1.2 开工面宽分布（152 笔 open_face_bill）

| 统计 | 值 |
|---|---|
| min / p50 / mean | 0 / 11 / 12.52 |
| p90 | 20.9 |
| p95 | 22.45 |
| p99 | 33.84 |
| max | 42 |

宽度桶频次：{"1-5": 16, "6-10": 53, "11-15": 33, "16-20": 34, "21-30": 14, "41+": 2}

path_count>20 共 **16 笔**（占 10.5%），批名如下：

| 宽度 | 批名 | 时间(UTC) |
|---|---|---|
| 42 | calllog-solo | 2026-09-07T02:53 |
| 42 | constclear2c-solo | 2026-09-08T08:17 |
| 26 | modou-namefit-solo | 2026-09-06T14:03 |
| 26 | closegate-solo | 2026-09-07T04:24 |
| 26 | closegate-solo | 2026-09-07T04:26 |
| 24 | regulamath-solo | 2026-09-06T14:59 |
| 24 | sweepimpl-solo | 2026-09-06T20:24 |
| 23 | mcpnomgate-solo | 2026-09-10T11:01 |
| 22 | leftover-solo | 2026-09-06T16:52 |
| 22 | facefit-solo | 2026-09-07T00:29 |
| 22 | orphanexec-solo | 2026-09-08T00:01 |
| 22 | packenv-solo | 2026-09-08T10:20 |
| 22 | wengumcp-parallel | 2026-09-10T16:45 |
| 21 | ledgerhyg-solo | 2026-09-06T13:53 |
| 21 | confpreempt-solo | 2026-09-07T02:19 |
| 21 | fusadopt-solo | 2026-09-08T16:49 |

### 1.3 哨兵语义每会话峰值持锁分布（401 个曾持锁会话）

| 统计 | 值 |
|---|---|
| p50 | 10 |
| p90 | 18 |
| p95 | 22 |
| p99 | 39 |
| max | 71 |

宽度桶频次：{"1-5": 78, "6-10": 150, "11-15": 103, "16-20": 44, "21-30": 19, "31-40": 3, "41+": 4}

阈值命中表（会话数，哨兵语义）：

| 阈值 | >15 | >18 | >20 | >22 | >25 | >30 | >35 | >40 | >43 |
|---|---|---|---|---|---|---|---|---|---|
| 命中 | 70 | 37 | **26** | 17 | 10 | 7 | 5 | 4 | 3 |

现阈值 20 命中 26/401 = **6.5%**（约每 15 个会话鸣一次，已偏离「起步宁宽防每批自鸣」初衷）。max_held>20 的 26 会话（21 个包）：

| 峰值持锁 | 批名 | 会话 |
|---|---|---|
| 71 | sweepclea6-solo | `7e7f6e476f43c658` |
| 69 | sweepclea6-solo | `ae4bb48718c18c57` |
| 67 | sweepclea6-solo | `a077847269f729ce` |
| 42 | constclear2c-solo | `9b08f4627b93322b` |
| 39 | outslim-solo | `684f5aa373108c9e` |
| 33 | autoflow2-solo | `f7a97a90ba1b0c7c` |
| 32 | cmdface-solo | `4542146955b7c5cd` |
| 30 | dose01-solo | `e8faac477f48c608` |
| 29 | cmdface-solo | `11d1be058ca0bb29` |
| 28 | facepark-solo | `0aa9b5cecdbcaa34` |
| 24 | wengunaming-t6d | `1c488df130b46d16` |
| 24 | tokencap-solo | `26262f0c4075fe2a` |
| 24 | deyimerge-switch-solo | `aae7260a6e51c4c7` |
| 24 | mathclose-solo | `956b39218007e5a3` |
| 23 | pendsweep-solo | `c6a4af77e65e3a5e` |
| 23 | mathclose-solo | `915064470271f50d` |
| 23 | mcpnomgate-solo | `3c8442699d11725e` |
| 22 | tokencap-solo | `5611cc7a0dec9983` |
| 22 | listzero-solo | `5c81a869dedca730` |
| 22 | packenv-solo | `18ee30b8c76a5b53` |
| 22 | wengumcp-parallel | `cbdff9259c1aa6b0` |
| 21 | pk035impl-solo | `193bd10f90553ec9` |
| 21 | guardhook-solo | `e3d54924f27861b3` |
| 21 | guardhook-solo | `11afb34fb67a2ffd` |
| 21 | leaseup-solo | `8f5a917bef255b3f` |
| 21 | fusadopt-solo | `633a768d3e6e4e5c` |

带宽构成：21-24 带 17 会话全是常规宽面 solo 批（合法宽面，纯噪声）；28+ 重尾 9 会话是真正的整树清扫/归并批（sweepclea6-solo ×3 于 67/69/71、constclear2c-solo 42、outslim-solo 39、autoflow2-solo 33、cmdface-solo 32、dose01-solo 30、facepark-solo 28）。28→24 之间存在天然断口（elbow）。

现势核查：locks.ndjson 当前零未释放锁，哨兵今日实跑将静默——26 为历史峰值口径的告警上界预期。

### 1.4 台账伪影注（idwire-solo）

会话 `28d326f924916cd0`（idwire-solo，2026-09-03）原始流 123 条 acquired 仅 11 个不同路径（同路径重复 acquire 至 17 次）。朴素时间序回放会读出 112 并发持锁；哨兵 (path,sid) 键字典语义塌缩后实际 ≤11。结论：阈值实证必须按哨兵语义回放（本文即是）；同时建议引擎侧关注重复 acquire 台账卫生（另批可查）。

### 1.5 调优建议

- **建议阈值调至 25**。依据：哨兵语义分布 p95=22，上取整穿越 21-24 常规宽带并落在 28→24 断口内；命中 26→10（**-61.5%**，6.5%→2.5%），重尾 28+ 九会话（真正的整树批）全数保留呈报。
- 备选保守值 30：命中 7（1.7%），只报极端离群，若哨兵定位收紧到「仅重尾」用此值。
- 不建议低于 20：p90=18，阈值 18/15 将命中 37/70 会话（9.2%/17.5%），重回每批自鸣。

### 1.6 常量位置（改值处，本批未改）

- 引擎位：`sih-engine/src/bin/watchcheck.rs:41` — `const LOCKFACE_WIDE_THRESHOLD: usize = 20;`（判位于 :290/:293，呈报文案 :330；bin 已在 `sih-engine/target/debug/watchcheck`）
- 围堰位：`sih-tools/watchcheck/src/watchcheck/constants.py:21` — `LOCKFACE_WIDE_THRESHOLD = 20`（登记行 :20）
- 下限测试：`sih-tools/watchcheck/tests/test_leaseup_watch.py:25`（断言 ≥10，25 不触线）

## 二、未用罚口径差值实证

### 2.1 现口径重验

现实现 `sih-tools/lease/src/lease/core.py:2230-2256`（close 收约路径）：`used_paths := session.allow 全集`（注释自认「最简近似，未对每个 allow 路径做实际改动追踪」），`locked_paths := locks 台账本会话 acquired 去重`，罚 = locked \ allow。全量重算 31/31 笔 `unused_paths` 与 locked\allow **逐笔全等**——公式复现无误，差值全部来自口径本身。

### 2.2 文件系统证据对表（124 条罚路径逐条）

窗口 = sessions 台账 issued_at..revoked_at；判定：目标 mtime 在窗内=写证；目录则加扫子文件（≤3 层）任一在窗内=写证。

| 判定 | 条数 | 占比 | 含义 |
|---|---|---|---|
| mtime 在窗内 | 23 | 18.5% | 实写，近似口径**误罚** |
| 目录子文件在窗内 | 42 | 33.9% | 实写，近似口径**误罚** |
| 窗内无任何写证据（含 pre_existing 10 / missing 4 / 目录后触 39 / 文件后触 4） | 57 | 46.0% | 候审真未用 |
| worktree 类（收约即删不可判） | 2 | 1.6% | 不可判 |
| 合计 | 124 | 100% | |

**正向写证据合计 65/124 = 52.4%**：近似口径把过半罚点罚在了实际写过（或强烈疑似写过）的路径上。31 笔罚单中 **21 笔含至少一条写证路径**、10 笔全程无写证。旁证信号：124 条罚路径中 **99 条（79.8%）是 lock_charged 付费加锁**——agent 为之付账的施工面扩张，实写概率远高于闲置。

### 2.3 两类差值实例分布

**「锁了但没写」（近似口径误罚面）**：65 条正证据路径分布于 21 笔罚单。典型例：regulamath-solo 7 条（含 m-regulamath-proof/rel/sub 三目录、facet contracts、materials 目录全在窗内实写）、toolincub-solo 3 条、mergeall-closeout 3 条（sih-engine/src/、packs/、critsweep/ 整目录窗内实写）、binmerge 的 tests/bscomplete.rs。

**「声明了但没锁」（近似口径盲区面）**：计费时代（152 个有 open_face_bill 的会话）allow 声明 1907 条中 **435 条（22.8%）从未加锁**，波及 92/152 会话；开工面按 allow 路径数计费即对这 435 条每条计 1 点，且未用罚对它们零追溯（未锁不在 locked 集内）。全台账时代（含 grandfather 前）6427 条声明中 2114 条（32.9%）未锁。

**「锁了且在 allow 内但无写证」（近似口径漏罚面）**：6 条分布于 5 笔（如 sih-math/sih/event/plan/regulamath-solo.md 缺席、reroute-solo-prompt.md 缺席、refseal 的 sih-tools/proposition 未触）——精确口径下应转罚。

### 2.4 机械瑕疵顺带发现

open_face_bill.path_count 未做斜杠归一去重：9 笔存在「同路径带/不带尾斜杠重复计费」，合计 **23 条重复条目**（如 halfmerge-solo path_count=18、归一后仅 13）。未用罚集合差分同样未归一，斜杠变体可致双向误差。

### 2.5 精确化建议（口径改形）

1. **used_paths 改源为实际改动文件集**，机械来源两选一：
   - 首选：收约时在双仓 worktree 上 `git diff --name-only <base>..<branch>`（收约机械本就跑 git 归并，零新增依赖，最准）；
   - 零 git 备选：复用 watchcheck `pen_face` 同源——当日 trail `direct_edit_completed.details.files` 并集（`sih-engine/sih/event/trail/<日>.ndjson`），与哨兵声明面天然同源对表。
2. **比对形改前缀覆盖匹配**：allow 面文件与目录混载，采 watchcheck `covered()` 同形（路径命中或其祖先目录命中即算用），弃字符串全等。
3. **集合运算先 `rstrip('/')` 归一**：开工计费与未用差分两处同改，消 23 条重复计费与斜杠变体误差。
4. **经济重算预期**：精确口径下罚点 124 → ≤59（65 点正证据退罚、57 点无证据保留、2 点 worktree 不可判挂起），即现近似口径**过罚至少 52.4%**；同时补上 435 点开工面「声明未锁」与 6 点「漏罚」的对账通道。

## 三、局限声明

- mtime 证据单向性：窗内 mtime 证写成立；窗后 mtime 不证未写（后批可再触），57 条无证据组是「候审真未用」上界而非确判；worktree 类 2 条不可判。
- sessions 台账 allow 与账单 detail.paths 存在 4 条级微差（1907 对 1903 计费点），账单时代以账单为准。
- 历史峰值口径的告警预期是上界：哨兵只在实跑时刻采样现势锁面。
