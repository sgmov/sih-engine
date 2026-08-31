# ordfound-mat-solo 结果档（材料补提迷你批）

> 批名：ordfound-mat-solo。日期 2026-08-31。会话 dfab19aef148285b（lease 1.9.1 单仓 engine）。
> 意图事件见链段（锚材料目录补提，ask3repeater status ok）。承接 ordfound-solo 批两段式治理第二段。

## 缺陷记录 {#defect}

ordfound-solo 批任务包请求写入清单漏列材料目录 sih-engine/sih/event/plan/ordfound-solo-materials/，guard 正确拦截 engine settle commit 的 staged_out_of_scope。执行未绕行即未改会话档 allow、未 unstaging 隐瞒、未手改 sessions.ndjson。主件先过，材料经本迷你批 ordfound-mat-solo 补提转正，两段都有链。此即 FAIL 就是 FAIL 的标准件。

## F 锚定验收

| F | 判据 | 判定 |
|---|---|---|
| F-1 材料转正 | 四件 json 由主检出未跟踪转为 engine git 跟踪 | 过（本批 commit 提交） |
| F-2 cert 复用 | settle commit --cert 取链上既有认证哈希，cert_on_chain 过 | 过（复用结果档认证 2edaeeaa） |
| F-3 主件不动 | ordfound-solo 主件结果档与 math 7b6341b 内容零改 | 过（未触碰，链上哈希不变） |
| F-4 零损失 | 主批 close 时 tdfix2 在途 15 笔事件经并集抢救零事件损失 | 过（终态与存底 shasum 一致 de1659bf） |

## 材料清单

- ordf-scrutinator.json：核阅报告（141 targets 全量 total 0）
- ordf-nomenclator.json：检词报告（8 件全 exit 0 零 findings）
- ordf-formatter.json：化格 report（ORD-011 落笔 1 行后全 0 改）
- recall-ordf.json：温故 recall（主题良基零命中）

## 链与收口

补提 intent 在链（001b7665）；结果档认证在链（2edaeeaa）；settle commit cert 复用链上既有认证哈希；close 归并后材料转正入主检出 git 跟踪。reconcile 双仓多链口径待令。

## 后续

主件 ordfound-solo 两段闭环完成，材料补提转正。遗留为共享日链多批并发正常态即 tdfix2 在途段由 tdfix2 批自身 close 提交。