# gateswitch-solo：boundary 判据换闸、判变逐件复核与探针退役 A/B

> 令源：用户 2026-09-05 令「②换闸命题重新立题测量，过了就拨闸；③114 件逐件复核；④探针退役开关」委外实施
> 范式：T6 单线 solo，委外代理亲写零子代理
> 设计源：pk-049（换闸挂起）、pk-048（判变预览清单）、pk-054（探针退役）、pk-058（判据观察，联动停批条件）

## 一、问题陈述 {#problem}

- **问题 1（②）**：boundary 判据切换命题 facetmath-switch-1 九发全合规但依据族三值分散落 boundary 打回，切换挂起在泊 pk-049；需按修订一通道分解重新立题测量，过得一裁 stable_clear 执契终签后拨 `boundary_criterion` 缺省为 "test" 在役。
- **问题 2（③）**：切换在役须承接判变逐件复核入档——pk-048 冻结清单 114 件（74 件 near→boundary、40 件 stable_clear→boundary，零由严到宽）逐件确认收紧合理。
- **问题 3（④）**：facet LLM 探针退役与上下文注入替代（pk-054），A/B 门切换批开工即承接出泊条件。

## 二、关键设计 {#design}

### 2.1 换闸重测（段一）

- 新 gid 立题，谱系披露照录（前例 facetmath-switch-1 boundary 在档），方向对己不利：命题改为机制规则类材料性断言形，禁裁断形禁祈使尾（修订三、四前置分道自查）。
- 合同模式九发：`--emit-contract` 出题，框架侧逐发原文回填，`--score` 计分必挂 `--identity-report`（pk-035）。
- **stable_clear → 执契四步（attractor check→verify→sign）机器终签 → 拨缺省**：`maturation_gate.py` 的 `boundary_criterion` 缺省 "ratio" 改 "test"，生产行为变更随批；引擎件 attractor 若内嵌同款判据，两侧同改同测同重编。
- **再落 boundary → 停批呈报**：若依据族分散签名复现（pk-058 同源），不得加采不得重跑钓鱼，停批把 pk-058 评估立项呈用户裁。

### 2.2 判变逐件复核（段二）

- 载荷：sih-engine/sih/event/plan/facetmath-solo-materials/facetmath-golden-dual-audit.json（sha256 ca9dda80cfe8bbe8 前十六位冻结）。
- 确定性脚本逐件复算双判据结果，与冻结清单对表零漂移；逐件给出收紧合理性核记（判由、证据指针），产出复核账本落批材料。
- 双跑逐字节一致；发现任何由严到宽漂移即停批上报。

### 2.3 探针退役 A/B（段三）

- 按 pk-054 意图：温度探针标定路径退役，上下文注入替代；A/B 门设计批内先呈（老路径与新路径同场对比、判据、切换条件），设计过得一裁后实装。
- 退役涉及键位与标定账本面（模式二 MiniMax 席），处置须与"键不随件走"方向一致，实情如实申报。

## 三、工作清单 {#work}

- [ ] gate-01：换闸命题重新立题与九发测量
- [ ] gate-02：stable_clear 后执契终签与缺省拨闸（ratio→test）
- [ ] gate-03：114 件复核账本（确定性脚本双跑）
- [ ] gate-04：探针退役 A/B 设计呈裁与实装
- [ ] gate-05：pk-048/049/054 三件出泊条件证据包备齐（出泊事件本身呈用户裁，批内不代落）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 换闸测量 | 新 gid 九发测量 stable_clear 或停批呈报，测量材料哈希绑定在 facet/contracts/ |
| **F-2** | 执契拨闸 | attractor check→verify→sign 全过，缺省 test 在役，七用例与全测试族零回归，金向量重放一致 |
| **F-3** | 复核账本 | 114 件逐件复算与冻结清单零漂移，复核核记入档，双跑逐字节一致 |
| **F-4** | A/B 门 | 设计呈裁在档，实装后对比读数在档，切换判据显式 |
| **F-5** | 出泊证据 | pk-048/049/054 出泊条件达成证据包在档，出泊事件零代落 |
| **F-6** | 停批纪律 | 若依据族分散复现，停批呈报含 pk-058 联动分析，零钓鱼重跑 |

## 五、必读文件 {#read}

- 泊件：sih-engine/sih/state/parking/materials/pk-048.json、pk-049.json、pk-054.json、pk-058.json
- 判变清单：sih-engine/sih/event/plan/facetmath-solo-materials/facetmath-golden-dual-audit.json
- 闸本体：sih-tools/facet/probes/maturation_gate.py（boundary_criterion 356/544-556 行）
- 测量先例：sih-tools/facet/contracts/facetmath-260904/（facetmath-switch-1 材料）
- 纪律：sih-engine/sih/state/skills/sihankor-facet-measure/SKILL.md（修订一至九全读）

## 六、约束 {#constraints}

1. 判定语义变更（拨闸、A/B 切换判据）过得一裁；stable_clear 执契机器终签即人授权（修订七），agent 一字不签
2. 同 gid 不得重跑；改写须新 gid 加谱系披露；改写超三次 R7 自动告警
3. 合同模式纪律：逐发原文不改写不摘要不挑选，缺发拒收整批
4. 复核段纯确定性零 LLM；全部新判定常数零裸奔
5. 出泊唯人节点：三泊件出泊事件批内零代落，证据包呈主会

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] CONTRACT 版本行随批（若 facet 契约有载闸位）
- [ ] CALL-LOG 双笔随批，allow 面含 CALL-LOG 两笔

## 八、风险点 {#risks}

- 换闸命题再落 boundary（pk-058 同源概率不低）：按 F-6 停批，不硬拨不钓鱼
- 拨闸后生产测量全变严：114 件复核先行即此风险的预案，两段次序不可倒置
- 引擎 attractor 侧判据同步遗漏：旧二进制事故先例，重编后双侧重测

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 T6-D 命名约定、F 锚定、得一裁红线。

## 十、关联文件 {#related}

- sih-engine/sih/event/plan/gateswitch-solo-results.md（随批产出）
- sih-engine/sih/state/parking/materials/ 四泊件
- sih-tools/facet/contracts/（测量材料落位）

## 十一、请求写入 {#requested-writes}

- sih-tools/facet/probes/maturation_gate.py（缺省拨闸）
- sih-tools/facet/contracts/gateswitch-260905/（测量与复核材料）
- sih-engine/sih/event/plan/gateswitch-solo-results.md 与 materials/
- 引擎件 attractor 判据位（若同步属实）与重编
- sih-tools/BATCH-FACE.md、sih-tools/scribe/CALL-LOG.md、sih-tools/lease/CALL-LOG.md
