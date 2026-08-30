# c006settle-solo 结果档（C006 四批结算补办与勘误上链）

> 批名：c006settle-solo 即结算补办批。日期 2026-08-30。会话 ae277451ef469ab7（lease 1.8.2 双仓副本）。
> 意图事件 3d553aa5（intent_refined，锚 PRO-08 应而不藏）。

## F 锚定验收

| F | 判据 | 判定 |
|---|---|---|
| F-1 sih-math 结算 | 113 件经副本提交归并，主检出对应文件洁净，移植前后逐件哈希一致 | 过（113/113 SHA-256 一致，范围清单 materials/c006settle-scope.tsv） |
| F-2 材料结算 | 四任务包与三结果档与两材料目录入库，他人线改动零扫入 | 过（10/10 哈希一致；proposition-defense SKILL、inputlog、task-packages/f-anchors-x11 未扫入） |
| F-3 链补真 | 勘误事件在链，历史结果档原文未改 | 过（append 认证承载本档哈希，四批历史档零改动） |
| F-4 三闸如实 | 化格 check 过、核阅与检词报告随材料含版本与哈希，读数不粉饰 | 过（化格 0 改 exit 0；核阅 exit 1 读数如实；检词 exit 0 skipped 113） |
| F-5 链 valid | 末笔追加后快照，verify 退出码零 | 过（见链段） |
| F-6 lease 通 | open 加 lock 加 commit 指副本加 close 全过 | 过（详见链段） |
| F-7 reconcile | 双仓对表无新疤，既有疤如实申报 | 见尾段 |

## 四批实况勘误 {#corrections}

以下为盘面核验与各批结果档自记之实况，与用户所接「4 批 1503 处改全程走通 T6 链上链，链 110 事件 valid」报账的差异逐条留痕。

- c006-jc-solo 即称类批：**机械撤回**，其结果档自记真即称类改法必然「删括号改语义」或「留括号不降 C006」二选一，F-1 与 F-2 不能兼过，承 rulecal-solo 撤回先例。链上零事件。
- c006-sb-solo 短注类首程：F-1 **部分过**即降 1066 不等于改 1217，差约 151；F-6 自记**链未跑**留待续批；其改法脚本设「阈值 ≥6 字符」跳过短括号，致大量短括号未改，是机械读数远高于其账面数的根因。链上零事件。
- c006-sb2-solo 中补注第二程：F-1 **部分过**即改 281 降 265；意图腿与 ask3 校验两笔在链；内容认证缺失。
- c006-sb3-solo 假 SKIP 修正批：其结果档标题宣称 1344 改 8 即 99.4% 归零，但其「数字校准」段自认真实残量 **17** 即 8 真 SKIP 加 9 假 SKIP，且自记 13 改 17 漏算 4 的数字误差；意图腿与 ask3 校验两笔在链，内容认证缺失。**「99.4% 归零」声明证伪。**

## 机械读数对表 {#mechanical-readings}

已提交规则包 des-001-mathe 0.2.0 下核阅读数即唯一账面。

| 时点 | C006 | S005 | M008 | C002 | 总 |
|---|---|---|---|---|---|
| fmtc2b 收口后（上轮基线，约数） | 约 1490 | — | — | — | 1657 |
| c006settle 结算态（本批实读） | **154** | 113 | 51 | 2 | 320 |

C006 实降约 1336，降幅约九成，四批的内容改动本身真实有效。154 残 = 8 真 SKIP 结构性留置（jc 二选一两难，待规则语义另裁）+ 约 146 待改（短括号约 137 加假 SKIP 9）。

## 三闸与材料 {#gates}

- 化格 formatter 0.1.0 packs general-v1 加 json-canonical-v1：128 件 check exit 0，0 改
- 核阅 scrutinator packs/des-001-mathe 0.2.0：128 件 exit 1，读数见上表，报告 materials/scrutinator-report.json 含内容哈希
- 检词 nomenclator packs/core 0.3.0：113 件 exit 0，域外 skipped 113，findings 0，报告 materials/nomenclator-report.json
- 温故 recall：两次调用均零字节空产出，如实记于 materials/recall-*.json，工具侧疑空转待查
- ask3：record 与零发现核阅报告随材料

## 教训 {#lessons}

- 账面数必须取机械读数，不得以改数递推：sb 批降 1066 记 1217、sb3 批账面 8 实读 154，同根同病
- 分卷阈值跳过的件必须入 SKIP 台账留痕，不得静默出账
- 意图腿上链不等于批走通 T6：内容认证事件缺席即链面无痕
- 改动落主检出工作树即不可结算形态：commit 拒直提新规下必须回围堰，本批即按此补办

## 链与收口 {#chain}

- scribe intent 3d553aa5（intent_refined）
- scribe append 本档认证事件：见 trail 末笔
- trail 快照复制发生于末笔追加之后
- lease commit 双仓指副本，close 归并拆本，reconcile 双仓，详见 lease CALL-LOG

## 后续 {#next}

- 余量 154 即 8 真 SKIP 加 146 待改手改分卷，待令
- 温故 retriever 空转疑点转工具侧待查
- sih-tools 144 件未提交面属他线批次，不在本批
