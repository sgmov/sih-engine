# resarch-solo 结果档

> 批：批余件入库归档批
> 会话：37e3bbc2ffd0f6c7
> 日期：2026-09-03
> 队形：单线形 solo，零子代理
> 承接：debtswp-solo 验收披露的未提交余件，用户令委外即冲突模式并发启动（pk-045 样本库参与者）

## 一、问题还原

debtswp-solo 收尾后余未提交件三类。其一引擎 inputlog 2026-09-03 即 seq 6 与 7 两笔活写未入版控（seq 6 即批二 α 裁定原话）。其二工具仓五条 ledgrev 调用册行即让位抢救后为活写态。其三引擎 PARKING-v1 名册 pk-045 行为主树未提交态。余件入库即账实归一。

## 二、机械链

ask3 双门 → 叩问三信号 → 正身 → 租约 open 双仓 → 取锁十四路 → 书简意图 → 工地双仓三件逐字节拷入 → 词债三登记 → 管线三步 → 认证上链 → 双仓 settle → 放锁 close（备份让位对表）→ reconcile 双仓 → 链 verify

## 三、三件对表读数（逐字节 identical 证明）

| 件 | 行进 | 读法 | 结果 |
|---|---|---|---|
| inputlog 2026-09-03.ndjson | seq 1 至 7 逐笔 | 全序连续无缺号 | seq6=批二人节点裁定 α=0.05，seq7=entryunique 派工令，口读在档 |
| 工具仓五条 ledgrev 调用册行 | lease/nomenclator/formatter/scribe/scrutinator | 逐条在档末尾 | 五条均含 ledgrev-solo 批（会话 c8744a784725b2b1）登记行 |
| PARKING-v1 名册 pk-045 行 | 当前在泊五项 | 与链上入泊事件对表 | pk-045 行记「入泊事件 e46ea82a」与链 event 63 pk-045 parking_entered 哈希前八位 e46ea82a 一致 |

入库读写校验：三件及其相关文件拷入工地后逐字节 sha256 比对 IDENTICAL（inputlog / parking / plan / dispatch / recall / 五条 CALL-LOG 全 IDENTICAL），零改字。

## 四、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 逐字节归档** | 治理 | 三件入库前后逐字节 identical | 过 | 拷入工地后 sha256 逐件 IDENTICAL（见三节读写校验），零改字 |
| **F-2 对表零差** | 工程 | inputlog 序列连续、调用册五条在、名册行与链事件一致 | 过 | seq 1-7 连续无缺号；五条 ledgrev 调用册行在档；pk-045 行入泊事件 e46ea82a 与链一致（见三节） |
| **F-3 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 双工地 git status 对表，未越 allow 冻结面 |

## 五、认证清单

意图 54e0d2e5（sess-zcode-260903-resarch）；链 verify valid；认证报告哈希见链尾。

## 六、词债登记

叩问三信号即批余件入库 / 账实归一 / 冲突样本节，经 nomenclator register 登记入 core 包工地副本（state=established），词表 104 → 107。

## 七、冲突样本节（pk-045 样本库）

本批为 pk-045 多 agent 冲突测试样本库参与者。冲突模式声明即撞锁等限重试、认证先落主树活链、串文件 settle 前一次性拷工地、调用册只追加永不覆写。实测冲突点与响应逐条如下：

1. 撞锁计数：本批执行时点双仓零活跃会话（既往全 revoked），十四路取锁零撞锁，重试计数零。
2. 共享面活写态：五条 ledgrev 调用册行即跨批调用册行丢失样本的抢救结果（让位覆盖经备份抢救后为活写态），本批将其逐字节归档入版控，解决路径即现状入库零改写。
3. 词债撞名：登记三词时与已立词条核对零冲突，未撞已立零撞懒波零撞死档。
4. 环境位移：scrutinator 首跑撞 PYTHONHOME 污染 encodings 缺失，处置即按 BATCH-FACE 坑位加 `env -u PYTHONHOME -u PYTHONPATH` 前缀重跑 exit 0，属环境态处置如实记。
5. 核阅退出码越线自纠：管线第一步曾以管道截尾读退出码（触「禁管道掩退出码」红线），即察觉重跑同参取真码 exit 2 即目标 event/plan 不在 des-001 治理域显式不属违规，disclosure 如实记入档。

管线三步读数：化格 exit 1（结果档落格改 1 行，化格写在治理窄域不属修改范畴）；核阅 exit 2（域外如实记，零内容违规）；检词 exit 0（词表 107 覆盖）。

## 八、披露

本批纯归档零行为变更，发现的任何内容问题申报不改（零申报）；分叉与悬会话为既有已收口态与本批无关。

## 九、验收

- [x] F-1 至 F-3 全过
- [x] 冲突样本节在结果档
- [x] 认证入链，双仓结算收约，对表读数在档
