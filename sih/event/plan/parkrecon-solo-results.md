# parkrecon-solo 结果档

> 批：parkrecon-solo（泊界账实对表修复批，pk-045 出泊与撞号改正重编）
> 会话：1ee1f144bef914ad（ask3 侧 sess-zcode-260905-parkrecon，双标识空间各认各）
> 日期：2026-09-05。队形：单线形 solo，零子代理。
> 令源：用户 2026-09-05「pk045出泊，pk055展开看看」；在泊授权承用户 2026-09-05「同意。得一归facet那边融回，你的异常和建议可以入泊」；改正重编先例即 pk-043 与 pk-047。

## 一句话结论

pk-045 出泊 promoted 落链（出泊事件 1a5f0799），账实对表查出并修正泊界三笔账：pk-054 进泊事件缺落补录进泊（bec9fa2d）、pk-055 与 pk-056 材料遭活写覆盖误载他件且随 dc4c8fb 误入版控按链复位、两件被覆盖事项承用户在泊授权改正重编 pk-057（3705afcc）与 pk-058（3e8dd6e0）进泊，名录投影更新十三项，四笔停泊事件逐笔 grep 验证、当日链 verify valid。

## 一、F 表（完成度表）

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 出泊事件 | parking_exited pk-045 带 promoted 与 ruling | 过 | 链 2026-09-05 parking_exited#1a5f0799，ruling 载用户令原话与样本销账对表 |
| F-2 补进泊 | pk-054 进泊事件落链材料一致 | 过 | parking_entered#bec9fa2d，材料补 action 与入泊块与补录披露 |
| F-3 改正重编 | pk-057 与 pk-058 落链含撞号披露；pk-055 与 pk-056 材料复位 | 过 | 3705afcc 与 3e8dd6e0 context 含撞号披露照录；两复位件按链上进泊事件细节逐字段重建，复位披露入 context |
| F-4 投影一致 | 名录与链在泊集一致过管线 | 过 | 名录改十三项（减 pk-045 加 pk-054、pk-055 补记、pk-057、pk-058），化格 0、检词 0、核阅域外 exit-2 如实记收口补跑 |
| F-5 例扫双跑 | rev3 双跑一致 checkmath 零红 | 过 | 双跑退出码 1/1（先例同形载体映射声明滞后灰）、四路 cmp 逐字节 IDENTICAL、checkmath zero_drift reds 0 greys 4 |
| F-6 双仓归并 | settle 与 close 与 reconcile 与 verify | 过 | 见收口读数节 |

## 二、账实对表事实（本批发现与修正）

1. **pk-055 与 pk-056 撞号（本批核心发现）**：链为准——链上 pk-055 即「级联边册投影更新机制缺位」（2026-09-04T14:56 进泊 4c249805）、pk-056 即「温故引擎侧语义通道缺位」（2026-09-04T15:20 进泊，名录行 da93d885 记），均真在泊。而材料文件 pk-055.json 与 pk-056.json 被后笔活写覆盖为另两件事项（台账活写覆盖根因硬化、判据观察转 facet 融回），覆盖后的材料随 ledger-repair 批收口附记提交 dc4c8fb 误入版控，链上进泊事件则被重入拒拦零落链。事故根与 pk-057 所载收约活写竞态同源，且 scribe park 经 meter 包裹时写入失败被输出解析掩败是直接推手（本批四笔 park 改裸调逐笔 grep 验证，全数落链）。
2. **pk-054 进泊事件缺落**：材料 2026-09-04 先立（facet LLM 探针退役），全 trail 五文件 grep 零命中——进泊事件从未落链，账实不符同款。本批补录进泊。
3. **两复位件重建源**：pk-055 与 pk-056 被覆盖前内容从未提交（git log 两文件唯一提交即 dc4c8fb），复位按链上进泊事件 details 逐字段重建，不复核不改判既有进泊事件。
4. **主树脏位**：开工时 pk-055.json 与 pk-056.json 在主树为未提交修改态（活写加 action 字段重排），close 前以 git checkout 复位至 HEAD 消脏，链真内容由批分支归并到达，全程申报。

## 三、pk-055 展开（用户令「pk055展开看看」应答材料）

链上真 pk-055（级联边册投影更新机制缺位）人话展开：引擎侧有一份级联边册的投影文件 doc/CASCADE.json，改文档改代码过完认证后这份投影不会自动跟着更新，级联工具自己也没有陈旧检测子命令（寻址工具有 stale_check 三态而级联没有），投影旧了没人报；租约乐观锁基线对表是它的消费位，读到旧投影就会漏检新边与陈旧边。出泊条件即用户裁三选一：收约批尾重建、级联加 stale 检测子命令接线告警、或消费位实时 build；裁立项即另开实装批，裁不立项即闭项出泊。

用户语境所指的「pk-055」（会话台账活写覆盖根因硬化）即撞号件，本批改正重编为 **pk-057** 人话展开：数学侧代理与租约线并行跑时，收约归并的机械步骤会重写会话与锁两本台账，谁后收约谁覆盖，docmath-b4 与 fixguard 的四行台账因此丢失；补录通道（ledger-repair 1.23.0）已把丢的数据修回来，本件泊的是根因硬化——方向两条即台账面并集超集归并（谁收约都保留对方行）或台账写点唯一化（只有一处能写台账），出泊条件即用户裁硬化立项或随租约线批六视图面顺带承载。

## 四、四笔停泊事件读数

| 事件 | 类型 | event_hash 前八 | 验证 |
|---|---|---|---|
| pk-045 出泊 promoted | parking_exited | 1a5f0799 | grep 一笔在档 |
| pk-054 补进泊 | parking_entered | bec9fa2d | grep 一笔在档 |
| pk-057 改正重编进泊 | parking_entered | 3705afcc | grep 一笔在档 |
| pk-058 改正重编进泊 | parking_entered | 3e8dd6e0 | grep 一笔在档 |

当日链 verify：valid。四笔均裸调 scribe park 不加 meter 包裹，逐笔 grep 复核在档。

## 五、投影对表读数

名录投影（PARKING-v1.md 在泊名录）：当前在泊十三项——pk-016、pk-039、pk-044、pk-046、pk-047、pk-048、pk-049、pk-053、pk-055、pk-056、pk-054、pk-057、pk-058。对链在泊集：parking_entered 无对应 exited 的 entry_id 十三项，机械一致。pk-045 移历史住户第十五项。

## 六、越线与误差申报

1. **核阅域外 exit-2**：PARKING-v1.md 与结果档在工地路径（worktrees/sih-engine/…）不在 des-001 域（include 形 sih-engine/doc/**），两目标核阅 exit-2 如实记档；收约归并后对主树 PARKING-v1.md 补跑 des-001 真判读，读数见收口读数节。
2. **例扫 rev3 退出码 1/1**：非 0/0，findings 一笔载体映射声明滞后（先例 leaseopt-audit 同形判灰），checkmath 独立判定 zero_drift reds 0 双跑逐字节一致，灰项只记不代修。
3. **例行读数 convergence 0.0**：较 09-04 读数 0.428571 下降，机械读数如实转述不解读成因，观察项。
4. **主树复位动作**：pk-055.json 与 pk-056.json 主树脏位以 git checkout 复位至 HEAD（即暂回被覆盖态）后由批归并送达链真内容，动作本身零认证面（工作树操作非提交），全程本档留痕。
5. **本批零测量**：出泊承用户令直执（泊界出泊唯人节点裁决），零机制变更零 CONTRACT 修订，不涉得一裁。

## 七、管线与链

- 例行读数：三维快照落链（convergence 0.0、adoption 1.0、mergeback 0.043478）
- 泊界心跳：引擎 attractor route 双目录退出码 0/0，engine 25 件 tools 19 件零告警
- 三问双门：ask3 三锚（PRO-07 鉴、PRO-08 应、P3.1 注意力预算原文程序切片逐字节子串），scrutinator ask3 包 exit 0、ask3repeater ok anchor_count 3
- 叩问：五词 unregistered 轻信号（出泊、复位、改正重编、账实对表、销账）全数叩问处置不登记，digest passed covered 5
- 正身：identity verify verdict attest anomalies 0
- 租约：open 1ee1f144bef914ad 双仓，锁五路径（governance 与 materials 与结果档与材料目录 exclusive，当日 trail append）
- 管线：化格 0→核阅域外 exit-2 记档→检词 0（PARKING-v1.md；结果档同跑见收口读数）
- 认证：管线报告与关键产物逐笔 append（读数见链）

## 八、收口读数（close 后回填）

- 待回填：双仓 settle 号、主树 des-001 补跑读数、reconcile 读数、链 verify 终读、归并对表与 bypass 登记
