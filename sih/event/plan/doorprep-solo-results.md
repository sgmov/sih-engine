# doorprep-solo 结果档：两道门备料——流程包数据形与 SDD 形任务包模板

> 治理批结果档（立文类，单线形 solo，委外执行）；会话 b5e510eb18a64e57；日期 2026-09-07
> 串行序：批一 chaingreen-solo 收约后方开工（F-4 承载）；上游输入件 regula-design-draft-2026-09-06.md §一§三§四

## 一、备料三件读数（F 表）

| F 锚定 | 判据 | 状态 |
|---|---|---|
| F-1 流程包在位 | flow-v1 数据形成文，首例治理批类映射条目齐 | ✅ `sih-tools/incubation/packs/flow-v1/flow-v1.json`（0.1.0）：任务类治理批类一条，SDD 门 documents 五条各挂 sdd-v1 包名加版本 @0.3.0，TDD 门 checks 四条各挂 tdd-v0 判定包加 TC-001 至 004 |
| F-2 模板在位且自证 | SDD 形模板成文，硬规则显式引用 TASK-PACKAGE-TEMPLATE；好样例五包全绿读数在档，坏样例红证在档 | ✅ `sih-tools/incubation/TASK-PACKAGE-SDD-TEMPLATE.md`（升形映射表七行 + 请求写入节显式引用）；好样例五跑退出码全零 verdict pass 零 findings（checker-reads/ 五件在档）；坏样例退 1 红证（SL-003 violation 重号 S-001、SL-007 missing 缺判据行，bad-scenarios.json 在档） |
| F-3 终签在链 | m-doorprep-1 stable_clear 终签入当日链 verify valid | ✅ §四 |
| F-4 写入仅 allow | 不改 TASK-PACKAGE-TEMPLATE 原件；零源码写入；在泊件与在盘遗留零触碰；串行序 | ✅ 原件 sha256 `487bf0ed…812ee` 对表批前零变（materials/task-package-template-sha256.txt）；sdd-v1 包族零触碰；检查器纯调用 |

## 二、升形映射对表（模板缝承载）

映射表七行落模板件尾节：元信息同位保留（版本位必填）、问题陈述升变更提案（四节位）、规格差分新增（三分类带归因）、F 锚升场景清单（R-／S- 编号加判据行三要素）、关键设计升技术方案（回链 R-）、工作清单升任务清单（引用加证据位）、五必读至十关联结构节保留、请求写入硬规则显式引用。缝以映射表承载零暗缝，缝大即停批条款未触发。

## 三、管线读数

| 步 | 件 | 读数 |
|---|---|---|
| 化格 json-canonical-v1 | flow-v1.json | 退出码 1 已归一改形（复检零改动在档 formatter-1） |
| 化格 general-v1 | 模板加样例六件 | 退出码全零零改动（formatter-2 至 7 在档） |
| 检词 core | 七件 | 退出码全零零违例（nomenclator-1 至 7 在档） |
| 核阅 des-001 | 七件 | 域外退出码 2 如实记档（scrutinator-1 至 7 在档；des-001 域只盖 sih-engine/doc） |

## 四、facet 与得一

- 命题 gid m-doorprep-1，9 发采样，9/9 comply 变卦 0%，谨慎信号 0/9，规约引用 baseline_4，闸门裁决 stable_clear
- 席位当日基线：退役探针 score 模式离线落行（doorprep 正身哈希 1f44f626 配对，核哈希 c0110e40 连认），行移入在册账本照 facefit 先例
- 得一三步：check 退出码 0 零告警；verify identical；sign 终签在链（crosscheck-m-doorprep-1），重放锚 `proposition/DES/m-doorprep-1/m-doorprep-1-signcheck.json`

## 五、双仓结算

| 仓 | 段 1 | 归并 |
|---|---|---|
| sih-tools | `8ce25bbd` | `28587566` merge: doorprep-solo 副本归并 |
| sih-engine | `7e08831` | `05bee47` merge: doorprep-solo 副本归并 |

## 六、收约对表节（收约后回填）

主树读数（2026-09-07 收约后即时）：

| 检查 | 结果 |
|---|---|
| flow-v1 与模板件主树落位 | ✅ `sih-tools/incubation/packs/flow-v1/flow-v1.json` 与 `sih-tools/incubation/TASK-PACKAGE-SDD-TEMPLATE.md` 归并在位 |
| TASK-PACKAGE-TEMPLATE 原件 | ✅ sha256 `487bf0ed…812ee` 与批前对表零变（F-4） |
| 链 verify | ✅ valid，188 笔（收约时点） |
| reconcile | unrouted 零；cert_missing 4/3 与批一收约后同态零新增；unbypassed 98/67（closeguard 机器提交与本批 bypass-orphan 绕行登记 1 笔）；session_orphan 19/24 同态 |
| 泊界心跳 | 工具线绿零告警；引擎线既知告警 1 同态零新增 |
| 收约碰撞 | 无主闸拦 16 件（14 册 CALL-LOG 投影腿加 calls.ndjson 系 calllog 直改车道活面含本批追加，attnanchor/anchor.py 系在途批活写），按闸面第三通道 `close --bypass-orphan` 显式绕行落 bypass 台账留痕，事由全文在 bypass 行 |

**判定**：F-1 至 F-4 全过；两道门备料三件（流程包数据形、SDD 形模板、样例红绿双证）在役，两道门实装批接线对象即此三件。

## 七、越线与误差申报

1. 坏样例落批材料 sdd-samples/ 隔离位，未污染 sdd-v1 正式包目录（约束 2 达成）
2. 模板件落位取 `sih-tools/incubation/TASK-PACKAGE-SDD-TEMPLATE.md`（任务包 §十一 目录级声明之新文件出生地，精确到文件传 allow）
3. meter run 不透传子命令退出码（scribe append 报错时 meter 仍退 0）——首轮 9 笔认证实际未上链（报告件缺席 scribe 报「报告不存在」），以回执 status=appended 逐笔核读后补齐 19 笔；meter 退出码掩码缺陷申报候后继批
4. 批一结果档补笔节的直改链笔被会话闸拒（SessionNotActive）——批一收约后回填走主树直改车道时正值会话已吊销，`--no-session-reason` 系主会处置位零代行，该补笔的链声明候人节点处置（内容与事件依据全在本批链笔与本档引用）
