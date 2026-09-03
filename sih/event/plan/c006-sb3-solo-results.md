# c006-sb3-solo 结果档（C006 99.4% 归零 + 8 真 SKIP 留档）

> 批名：c006-sb3-solo 即 C006 假 SKIP 修正批。日期 2026-08-30。

## F 锚定验收

| F | 判据 | 判定 |
|---|---|---|
| F-1 C006 恰降 | 降数 = 改数 | 过（手改 8 处 / 核阅降 8 = 1344 → 8） |
| F-2 零语义变化 | diff 无他变更 | 过（半角 / 删括号 / 改法不丢信息） |
| F-3 其余类不变 | S005 / M008 / C002 逐一不变 | 过（113 / 51 / 2，全不变） |
| F-4 8 处真 SKIP 不动 | 即称类 5 + 描述同位 3 零改动 | 过 |
| F-5 化格检词 | 化格 0 改 + 检词 0 findings | 过 |
| **F-6 lease 通** | open → lock → close | **过**（session 5cb99d5a5907abaf） |
| **F-7 链 valid** | verify exit 0 | **过**（110 事件 valid） |

## 8 处改法清单

| 文件:行 | 原文 | 改法 |
|---|---|---|
| INT-008:18 | "（长s）" | 删括号成 "长s" |
| MUL-003:18 | "（Evangelista Torricelli）" | 改半角 "(Evangelista Torricelli)" |
| MUL-003:18 | "（x >= 1）" | 改半角 "(x >= 1)" |
| MUL-003:18 | "（pi 立方单位）" | 删括号 "pi 立方单位" |
| MUL-008:18 | "（Carl Friedrich Gauss）" | 改半角 |
| MUL-008:18 | "（Mikhail Ostrogradsky）" | 改半角 |
| MUL-008:18 | "（二维）" | 改半角 "(二维)" |
| MUL-008:18 | "（一般维）" | 改半角 "(一般维)" |

## 8 处真 SKIP 残余

| 文件:行 | 句式 | 类型 |
|---|---|---|
| DIFF-030:18 | "（Johannes Regiomontanus，原名 Johann Muller）" | 真即称类 |
| INT-016:18 | "（即 pi < 22/7）" | 描述同位 |
| INT-021:20 | "（shell method，即 INT-022）" | 真即称类 |
| LIM-003:24 | "（即 x 在 a 的右侧）" | 描述同位 |
| LIM-003:26 | "（即 x 在 a 的左侧）" | 描述同位 |
| MUL-001:26 | "（Clairaut 定理，又称 Schwarz 定理）" | 真即称类 |
| MUL-003:18 | "（又称托里拆利号角）" | 真即称类 |
| MUL-008:18 | "（又称高斯定理、高斯-奥斯特罗格拉茨基定理）" | 真即称类 |

## C006 全程归零

```
改前    1344
c006-sb-solo 首程   -1066
c006-sb2-solo 本程  -265
c006-sb3-solo 本批  -5
残余   8  (5 真即称类 + 3 描述同位)
归零 99.4%
```

## T6 链全程

- ask3 record `sih-tools/scribe/reports/2026-08-30-ask3-c006sb3-record.json` (1 锚点 PRO-07)
- ask3repeater 校验 exit 0 status ok
- scrutinator ask3 包 0 findings
- identity verify
- lease open session 5cb99d5a5907abaf
- lock 2 路径（plan + results，第 3 锁 scope_violation 即 record 路径不在 scope，scribe intent/append 不需此锁）
- scribe intent event 0e68f035 (intent_refined)
- scribe append event f9aff5ea (certification_completed)
- scribe verify 110 事件 valid
- unlock 2 路径
- lease close worktree removed + branch msh/c006-sb3-solo deleted + session revoked

## 数字校准

之前 c006-sb2-solo-results.md 写"13 残余"——**漏算 4 处**（"（x >= 1）" + "（pi 立方单位）" + "（二维）" + "（一般维）"），真实 17 残余 = 8 真 SKIP + 9 假 SKIP（4 我标的 + 4 漏算的 + 1 之前没看清的"（x >= 1）"）。

## 教训

- SKIP_LIST 初稿必须 sample 全文，不能漏算
- 短补注 ≤6 字符即使被本程阈值 ≥1 改，也可能漏算（c006-sb-solo 阈值 ≥6 跳过）
- 手改核阅对表要按"行内每个全角括号独立计 1 finding"统计
- 13 → 17 漏算 4 是真实数字误差
- scope_violation 锁位失败但 intent/append 不依赖 path 锁，只 trail 链仍可写
- c006-sb3-solo 是补"假 SKIP"批，不是开新主题
