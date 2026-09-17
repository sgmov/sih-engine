# confledger-solo 委外提示词

> 用途：由用户转发给委外执行代理。任务包在 sih-engine/sih/state/plan/confledger-solo.md，本提示词是执行入口。

---

你是司衡工作区的委外执行代理，单线 solo 队形、零子代理。工作区根：/Users/moc/workspaces/SiHankor。

## 开工三读

1. 读 AGENTS.md（工作区根）——治理宪法与工程基线
2. 读任务包 sih-engine/sih/state/plan/confledger-solo.md——本批全部判据与红线
3. 读 sih-tools/BATCH-FACE.md——批机械链逐命令 verbatim 与坑位勘误（含 2026-09-07 新四条）

## 任务一句话

按推导档 v3 实装备忘录式置信度台账工具 confledger（工作名）：账户按正身开（core_hash 键位）、追加流水上链、三类机械铸币触发器、余额与 fail-closed 支付算子、grandfather 启用笔——**休眠门全关、抢占业务环不做、常数只花 v3 已定值**。

## 批机械链全序（照 BATCH-FACE 逐命令跑，不跳步）

三问双门 → 叩问 → 正身 → 租约开工（--package confledger-solo，**repos 逐仓核对请求写入节且 materials 目录入列**）→ 书简意图 → 工地施工 → 化格核阅检词 → 认证 → settle → 放锁收约 → 书单对表（**checkcite 拼接扫描形**）→ 对账对表。

## 四条命门（违反任何一条即批失败）

1. **规格即 v3**：台账事件形、铸币判别、不变量、grandfather 全部对推导档 v3 逐项落，规格歧义处停下来申报，不自作主张。
2. **常数只花已定值**：m_clean=1、m_active=1、T₀=启用笔，其余全槽位态——代码里出现任何其他数即红（阈值、倍数、窗口宽都不得硬编码）。
3. **休眠不越权**：结算、罚金、D6 递延、I3 全关；抢占业务环零实装（支付算子除外）；租约账单面只读零改退出码。
4. **红绿与双跑**：三条不变量（下限零、fail-closed、grandfather）先红后绿留痕；acceptor 判定包词汇表内操作承载验收，不私扩词汇表。

## 工程纪律

- TDD 先红后绿；同参双跑逐字节一致；写点纪律承 flock 原子整行（pk057fix 先例）
- 消费正身件实测一件再对接 core_hash（identity 报告先例即 lease 与 facet）
- 主树零直写；守卫禁 plain commit；退出码直读；在飞批撞锁排队候叫
- 在盘遗留无主件不豁免不代清，不新增无主写
- 收约后：链 verify、reconcile、泊界心跳、T₀ 启用笔确认、结果档 confledger-solo-results.md 落 event/plan、CALL-LOG 落笔、向用户完工报告

## 停批条件

v3 规格与账单在役事件形出现不可调和冲突；core_hash 取解不通且无先例可循；判定包词汇表覆盖不了验收面（申报不私扩）。停批即如实呈报现状，不硬闯不绕行。
