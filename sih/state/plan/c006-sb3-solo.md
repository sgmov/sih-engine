# c006-sb3-solo：C006 假 SKIP 修正批

> task-packages 治理任务
> 承接：c006-sb2-solo 收口后 13 残余中 4 处"假 SKIP"修正
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

c006-sb2-solo 收口后 C006 残余 13 处 = 9 真 SKIP（即称类 5 + 描述同位 4）+ 4 假 SKIP（英文人名 3 + 短补注 1）。本批修正 4 处假 SKIP 走 T6 链收口，C006 9 处真 SKIP 留人工。

## 二、4 处假 SKIP 清单

| 文件:行 | 原文 | 改法 |
|---|---|---|
| INT-008:18 | "（长s）" | 删括号成 "长s" |
| MUL-003:18 | "（Evangelista Torricelli）" | 改半角 "(Evangelista Torricelli)" |
| MUL-008:18 | "（Carl Friedrich Gauss）" | 改半角 |
| MUL-008:18 | "（Mikhail Ostrogradsky）" | 改半角 |

## 三、关键设计 {#design}

段一手改 4 处（量小，机械改无歧义）。
段二核阅对表：C006 13 → 9 = 100% 归零（机械部分）。
段三化格检词复跑。
段四ask3 记录 + 验收。
段五 lease 全程：open → lock → commit → close。
段六 scribe intent + append + verify。

## 四、可证伪条件

| F | 类别 | 判据 |
|---|---|---|
| F-1 C006 恰降 | 工程 | 核阅 C006 13 → 9 = 降 4 |
| F-2 零语义变化 | 治理 | 4 处改法不丢信息 |
| F-3 其余类不变 | 治理 | S005 / M008 / C002 / N002 计数逐一不变 |
| F-4 9 处真 SKIP 不动 | 治理 | 即称类 5 + 描述同位 4 零改动 |
| F-5 化格检词 | 治理 | 化格 0 改，检词 0 findings |
| F-6 lease 通 | 治理 | open → lock → close 全过 |
| F-7 链 valid | 治理 | scribe verify 退出码零 |

## 五、约束

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 9 处真 SKIP 绝对不动
5. 走 T6 完整链（不开简化）

## 六、请求写入

- sih-math/calculus/llm-friendly-build/entries/INT-008-integration-by-parts.md
- sih-math/calculus/llm-friendly-build/entries/MUL-003-gabriels-horn.md
- sih-math/calculus/llm-friendly-build/entries/MUL-008-divergence-theorem.md
- sih-engine/sih/state/plan/c006-sb3-solo.md
- sih-engine/sih/event/plan/c006-sb3-solo-results.md
- sih-engine/sih/event/plan/c006-sb3-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/2026-08-30-ask3-c006sb3-*.json

## 七、验收

- F-1 至 F-7 全过
- C006 16 → 8 = 降 8（手改 8 处，5 真即称类 + 3 描述同位 留人工）
- 链 valid

## 八、队形

单线形即主线亲写零子代理。

## 九、最终归零进度

- 改前 1344
- c006-sb-solo 首程 -1066
- c006-sb2-solo 本程 -265
- c006-sb3-solo 本批 -5
- 残余 8 (5 真即称类 + 3 描述同位)
- 归零 99.4%
