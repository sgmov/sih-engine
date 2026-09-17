# chaingreen-solo 结果档：四查链主树复绿收口

> 治理批结果档（工程类，单线形 solo，委外执行）；会话 f3770e02d8bcafe0；日期 2026-09-07
> 收口判据唯一：主树 TDD 验收工具判定包端到端退出码零，TC-001 至 TC-004 全过，批内加收约后主树双读数

## 一、红态基线节（先红留痕）

修复前三项红证归档 `sih/event/plan/chaingreen-solo-materials/red-baseline-20260907/`：

| 红证 | 件 | 实测 |
|---|---|---|
| 检查器套 | first-red-pytest.txt | 6 passed 1 failed，test_golden_replay KeyError 'findings'（读旧 golden 墓碑形缺键） |
| 判定包端到端 | first-red-acceptor.json | verdict fail 退出码 1，TC-003 violation，TC-001/002/004 pass |
| git status | first-red-gitstatus.txt | 八件墓碑 M 加 terms.json M（与任务包申报一致） |
| 检词套存量红 | first-red-nomenclator-canonical.txt | test_core_pack_shipped_canonical 红——lazy.json 存量非规范（toolincub 在途带入，facefit-solo 结果档同款勘误），非本批面零触碰 |

## 二、查源节（terms.json 三禁处置）

diff 逐行呈报：terms.json 未提交修改为纯新增两词条（namespace-face、subpath-reservation，signed 2026-09-07），零删改零回退。

查源对证：`sih-tools/calllog/calls.ndjson` 348 行 facefit-solo 批（会话 612c797612debac0）occasion 载「NAMESPACE_FACES 冻结三面……子路径保留锁」已签项，与词条 source 字段「facefit-solo 批已签项 承用户 2026-09-06 锁文件夹问答裁定」逐字对上——查证成立。

处置：经本批通道同内容入版控（cmp IDENTICAL 携带），化格 json-canonical-v1 复核零改动（exit 0 已规范形），检词 canonical 单验 terms 过（四 json 中唯 lazy.json 红系 toolincub 在途存量，如实转述零代修）。

## 三、施工读数（批内绿读数）

- 检查器套（工地）：`7 passed in 0.55s`，退出码 0——test_golden_replay 改读基线向量管理工具新家（expected-*.md.bin 与 expected-bad-scenarios.bin，哈希对表 index 在档）后全绿
- 八件墓碑带工地：cmp 逐件 IDENTICAL（七件 golden 加一件 frozen）
- 判定包四 TC 等价命令（工地，读数落 green-first-20260907/）：TC-001 双跑退出码 0/0 cmp IDENTICAL；TC-002 对冻结全报告 cmp IDENTICAL；TC-003 pytest 七测绿；TC-004 五场景期望退出码全零、判程序即 TC-001 命令退 0、覆盖差集零
- 判定包端到端批发地试跑：退出码 2（{ROOT} 形路径按工作区根展开，工地布局无 sih-tools 段不可执行）——按 vecfix-solo 先例以工地等价命令承载批内读数，端到端留主树收约后跑（先例在档如实记）

## 四、facet 与得一

- 命题 gid m-chaingreen-1，9 发采样，9/9 comply 变卦 0%，谨慎信号 0/9，规约引用 baseline_4，闸门裁决 stable_clear
- 得一三步：attractor check 退出码 0 零告警 direction comply；verify identical 裁决通过；sign 终签在链 event_hash `e409a0f6b305b5ab`（crosscheck-m-chaingreen-1，竞态重建后笔，初笔 04d43560 随 §七 7 事故卷走已申报），重放锚 `proposition/DES/m-chaingreen-1/m-chaingreen-1-signcheck.json`
- 席位当日基线：退役探针 score 模式（离线零消耗）为本席位落标定行（核哈希 c0110e40 配对承 idcore-solo），行移入在册账本照 facefit-solo 先例如实记；断裂与垫片申报见 §七

## 五、管线读数

| 步 | 目标 | 读数 |
|---|---|---|
| 化格 json-canonical-v1 | terms.json（工地） | exit 0 零改动已规范 |
| 化格 general-v1 | topic.md、结果档、提示词存档 | 见认证节逐件 |
| 核阅 des-001 | 各 MD 件 | 域外 exit-2 如实记（des-001 域只盖 sih-engine/doc） |
| 检词 core | topic.md、结果档 | 见认证节逐件 |

## 六、F 表

| F 锚定 | 判据 | 状态 |
|---|---|---|
| F-1 墓碑归位 | 八件经批通道入版控，主树 M 态清零，归并 diff IDENTICAL | 收约后回填 §八 |
| F-2 旧位依赖清零 | test_golden_replay 读新家，检查器套全绿 7/7 | ✅ 批内；主树复跑见 §八 |
| F-3 terms 查源 | diff 逐行呈报在档，处置留痕 | ✅ §二 |
| F-4 四查链复绿 | 主树判定包退出码零 TC 全过，双读数在档 | 批内绿 ✅，主树见 §八 |
| F-5 终签在链 | m-chaingreen-1 stable_clear 终签入当日链 verify valid | ✅ §四；链 verify 见 §八 |
| F-6 写入仅 allow | 零引擎零租约零 sih-math 源码写入；.bin 零触碰；在泊件与在盘遗留零触碰 | ✅ |

## 七、越线与误差申报

1. 退役探针标定：temp_probe score 模式离线落基线行（atom.yaml 寻径断裂以材料侧垫片补齐，零源码改动；行移入在册账本照 facefit-solo 先例），垫片与 20 发 responses 全档在 facet-calibration/
2. 判定包工地端到端退出码 2（路径形不配），按 vecfix 先例以等价命令承载批内读数
3. 批材料初落主树 event/plan 后即纠偏迁入引擎工地随批提交，主树零残留
4. watch 对表呈 30 件无主修改：八件墓碑加 terms.json 系本批批面（收约即归位），其余 19 册 CALL-LOG 加 calls.ndjson 加两件 pyproject 系在盘遗留（calllog-solo 与 toolincub 在途带入），按协议零代行如实转述
5. 例扫读数：rev3 双跑退出码 1/1（漏项核对失败 2 笔与 sddpacks-solo 批今日读数逐字同款既知态），四路 cmp 全 IDENTICAL，checkmath zero_drift 零红 6 灰
6. 管道掩码自纠：租约锁批量循环内一次 `$(…|tail)` 掩码读数即改直读复跑（检词套主树红证 NOM_EXIT=1 直读在档）
7. **链面竞态事故（申报候裁）**：前身会话 f3770e02d8bcafe0 的吊销形 close 触发让位归并，主树当日链被回退至分支基线态，卷走 42 行（本批 17 笔加他批在途笔）至 `worktrees/.close-backups/chaingreen-solo/f3770e02d8bcafe0/`。恢复经重建链笔承载（新事件新哈希、报告件与材料不变）：r2/r3 ask3 记录三轮双门、十二笔认证三次上链、裁决材料基线寻径改指工地材料位后 check 绿 verify identical 重签。可线性复原的补笔已试（主链前 140 行与备份逐字节同），因并行批活写使缺失段与现存段自分叉点起无法线性并回，verbatim 复原不可达即按补笔复认证先例重建
8. **事故后果申报**：reconcile 双仓 cert_missing 各新增 1（段 1 提交 c4b56ec 与 ead18e5 所引认证 b15cddcf 随竞态卷走，段 2 已换用在链认证 aa254139； commit 信息承载历史不改，治理域历史不可改起点）；unbypassed（95/66）与 session_orphan（19/24）为存量累积非本批新增形态
9. DES 命名空间面：任务包 §十一 裸目录行被解析为 `<面>/<批名>/` 保留位，而得一格子实为 m-gid 名形，拆会话重开以 `--namespace sih-tools/proposition/DES/m-chaingreen-1/` 兜底（namespace-face 词条所述兜底场景本批首例实录），字节全量备份零丢失

## 八、主树复跑补笔节（收约后回填）

主树归并（2026-09-07 收约后即时）：

| 仓 | 归并提交 | 段 1 | 段 2 |
|---|---|---|---|
| sih-tools | `478ca1f7` merge: chaingreen-solo 副本归并 | `c4b56ece` | `4119f5f6` |
| sih-engine | `0c66d1b` merge: chaingreen-solo 副本归并 | `ead18e5` | `2f323d4` |

主树复跑实测：

| 检查 | 结果 | 说明 |
|---|---|---|
| 判定包端到端（收口判据唯一） | ✅ 退出码 0，verdict pass，TC-001/002/003/004 全过 | 双读数之主树腿，红态基线 verdict fail 退 1 对照在档 |
| 检查器套 | ✅ 7 passed in 0.50s，退出码 0 | test_golden_replay 读新家主树全绿（F-2） |
| 八件墓碑 M 态 | ✅ git status 清零 | 归位完成（F-1） |
| 归并 diff 对表 | ✅ 八件加 terms.json 逐件 IDENTICAL | cmp 对表批内带入件（F-1 内容零改动） |
| 链 verify | ✅ valid，159 笔（收约时点） | 当日链含本批 rebuilt 链笔（F-5） |
| reconcile | ⚠️ 双仓 exit 1 | unrouted 零；cert_missing 4/3 含本批事故新增 2 笔（b15cddcf 段 1 认证被竞态卷走，§七 8 申报候裁）；unbypassed 与 session_orphan 存量 |
| 泊界心跳 | 工具线绿零告警；引擎线既知告警 1（siding_surplus 计 2 触阈 2） | 与批前同态零新增 |

**判定**：收口判据唯一达成（主树判定包端到端退出码零 TC 全过，批内加收约后双读数）；F-1 至 F-6 全过；链面竞态事故与 reconcile 增量如实申报候裁，不硬闯不绕行。
