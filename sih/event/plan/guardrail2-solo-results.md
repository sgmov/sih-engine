# guardrail2-solo 结果档（主会代收补笔）

> 批：写入门四闸批（同包唯一 + 重意图拒 + 会话在册验 + 收约预检）
> 会话：b14c0a12024f34e5（委外执行至认证段后停滞，收约由主会代走，本档由主会代笔如实补记）
> 日期：2026-09-03
> 承接：用户同包双跑 locksplit 实测四洞（双开无拒、双意图上链、无租约裸跑、收约半程三例）
> 意图哈希承链上事件 174 即 ask3 记录 2026-09-03-ask3-guardrail2-solo-record.json

## 一、四闸实装与测试读数

| 闸 | 落位 | 测试 |
|---|---|---|
| 闸一同包唯一 | lease open 增 PackageSessionActive（源码 core.py 加 cli.py 五处） | lease 套件含反例，76 过含新增四测 |
| 闸二重意图拒 | scribe intent 增 IntentRecordUsedRejected，--allow-reintent 默认关 | 引擎套件含反例，156 过含新增十二测 |
| 闸三会话在册验 | scribe intent 与 append 增 SessionNotActive，--no-session-reason 默认关，需 --sessions 台账路径 | 同上 |
| 闸四收约预检 | lease close 增 git merge-tree --write-tree 逐仓预检（源码九处） | 含冲突拒反例；本批 close 首跑预检静默通过零半程 |

委外段提交：tools 7bff5436（闸一闸四，CONTRACT 修订升 1.15.0）、engine c5341c1（闸二三，CONTRACT/USAGE 两旗标默认关）。主会代收：lease close 归并 engine 792ddca 与 tools 71ea1c68，会话吊销，零半程。

## 二、主会验收活体探针

1. 重建后闸三两针均拒：缺 --session 即拒；带假会话 deadbeefdeadbeef 加台账路径拒 SessionNotActive 载会话号，详情非空。
2. 闸二探针被闸三先拦（会话验序在重意图验前），重意图拒面由引擎反例测试承载。
3. 闸四本批 close 首跑实战通过（预检静默零半程）。
4. 闸一无安全活探条件（需活跃会话双开），由反例测试与源码位承载。

## 三、越线与误差申报（主会自报）

1. **旧二进制探针笔两枚**：主会首针刺闸时主树 target/debug/scribe 为合并前旧件，闸未拦即追加成功，产生垃圾事件两笔即重复意图 5aa7a3718b67 与假会话认证 704f35c1897，链 append-only 不可删，如实记为探针笔遗迹。教训入坑位：**合并后闸类代码须先 cargo build 主树再探针或调用**。
2. **新必填参数**：闸三使 gated 写入需 --sessions 台账路径，BATCH-FACE 命令面未载，待勘误批捎带。
3. 委外停滞在收约前（认证段后约 28 分钟无动作），主会按悬置处置代走收约，代笔本档。

## 四、F 表（主会代核）

| F | 判据 | 结果 |
|---|---|---|
| F-1 四闸各拦 | 反例实测与测试拒且详情非空 | 过（闸三活针、闸二三四测试与实战） |
| F-2 合法通道不伤 | 先开后收与两旗标与净过回归 | 过（76 加 156 全绿含回归族） |
| F-3 行为兼容 | 既有测试全绿 | 过 |
| F-4 写入仅 allow | 请求写入节所列 | 过（工地 staged 对表） |

## 五、链与台账

链 verify valid 187（含探针笔两枚遗迹）；零活跃会话零在锁；reconcile 收约后双仓 exit 0。
