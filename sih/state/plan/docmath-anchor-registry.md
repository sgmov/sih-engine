# docmath-anchor-registry 任务包：docmath 载体回锚登记收尾批

> 队形：单线形 solo，零子代理
> 日期：2026-09-04
> 令源：用户 2026-09-04 令继续数学仓任务并择「docmath 回锚登记收尾」
> 意图：ask3 记录 2026-09-04-ask3-docmath-anchor-registry-record.json（ask3gate 校验 ok，三锚）

## 目标

把 docmath-b2 载体入仓批承载的三枚工程机制载体回锚登记进资产回锚登记面
`sih-math/docs/asset-anchor-registry-2026-09-02.md`，逐件登记载体指向与推导档指针与消费侧，
清 docmath 在数学仓的登记在案账。登记是机械账非新裁决，载体成立已由推导档与金向量与性质测试
D-4 四查齐承载。

三行登记要点（载体指向 mapping 行号实取，推导档 §3.1/3.2/3.3）：

| 机制 | 载体指向 | 推导档 | 消费侧 |
| --- | --- | --- | --- |
| 规格充分性（约束完备性） | ORD-002(mapping.md:75)、ORD-003(:77)、PROB-009(:194 类比) | docmath-carriers-derivation-2026-09-04.md §3.1 | docmath_golden_script.py 场景一至三 |
| 证伪覆盖度 | ORD-010(mapping.md:186)、PROB-008(:193) | 同档 §3.2 | docmath_golden_script.py 场景四至六 |
| 重写合流性硬测试 | ORD-023(mapping.md:209) | 同档 §3.3 | test_confluence_docmath.py 六测 |

章末记协调与勘误：与 measure-poly 可识别性各管各的不合载（2026-09-04 人节点裁定）如实引；
机械边界随载体入档；登记面勘误如实声明。

## 写入面（allow）

- sih-math/docs/asset-anchor-registry-2026-09-02.md（唯一写面）
- sih-engine/sih/state/plan/docmath-anchor-registry.md 与本批结果档与批材料目录
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/meter/counts/（append 短持）

## 红线

- 数学仓 mapping 与 entries 与 INDEX 零触碰（只动 docs/ 登记面）
- 三工具 CLI 与退出码语义零改，des-001 判定位零触碰
- 哲学仓只读不代立命题；不重复立载、不碰 measure-poly 可识别性
- 主树零直写经工地 settle 通道；链文件只经引擎 scribe 写位；禁管道掩退出码
- 守卫在位严禁 plain git commit 直提，close 通道以外提交须 --no-verify 加 lease bypass 登记

## 验收

1. 登记面补记节落位，三行载体指向与推导档与消费侧逐字节可复核
2. 化格 general-v1 exit 0 零改
3. 机械链全绿自行收口；认证清单与链 verify valid；reconcile 无新增残留