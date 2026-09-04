# leasewire-solo：租约与锁对挂 ORD-020 载体（全序资源分配与死锁自由）

- 承接：数学管线全量串联计划批四起序列「lease 锁机制对挂 ORD-020（全序资源分配与死锁自由）」；批一覆盖账本读数即 lease 已实例化（SPEC-011 引用）而 locks 零载体——本批两机制统一深锚：lease 增注 ORD-020 判定位，locks 首锚。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-leasewire）｜ 温故检索：materials/recall-leasewire.json 结果如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理。接线与推导为非行为变更批。

## 背景（线索，实跑为准）

- 载体条目 order/entries/ORD-020-total-order-resource-allocation-deadlock-freedom.md（mapping 行实取）。
- 语义对位（推导档承载，判定位以工具实查为准）：租约与锁的全序获取序即资源全序分配，死锁自由即全序性的推论（循环等待在全序获取下不可能）；locks 的 acquired 与 released 台账配对即锁面一致性判定。实体位 sih-tools/lease/ 与 sih-tools/locks/。
- 融回序位注记：DEC-013 复用节预告下一件融回即租约——本批工具侧深锚在前，锚点与推导档随实现过融回门。

## F 清单

- F-1 四件套 ×2（lease 与 locks 各一套）：载体引用（CONTRACT 增载体引用节或源内注记锚点位，实查择一申报）、推导档（合一件 sih-math/docs/leasewire-derivation-2026-09-04.md 分两节）、代码接线（判定位注记锚点，AST 同形零行为）、金向量（双跑逐字节一致；锁获取序场景金向量正对全序性，死锁自由反例场景可选）。
- F-2 零行为变更：lease 与 locks 既有测试批前批后同绿零回归。
- F-3 检词零违例；锚点新词按登记先例入词表，词债不过夜。
- F-4 写入仅 allow。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package leasewire-solo → 取锁（施工面 lease 与 locks 目录 exclusive 长持；共享追加面一律 --mode append 短持即取即放）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 append 主树活链 → settle 前一次性拷工地 → 双仓 settle --cert（tools 与 engine；math 仅推导档则随批三仓）→ 放锁 → close（备份让位归并对表法）→ reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/lease/ 与 sih-tools/locks/（源码注记、CONTRACT、测试、CALL-LOG 各自面）
- sih-math/docs/leasewire-derivation-2026-09-04.md
- sih-engine/sih/event/plan/leasewire-solo-materials/ 与 leasewire-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

lease 与 locks 行为零改动（AST 同形）；数学仓 entries 与 INDEX 与 mapping.md 零触碰；sih-tools/retriever 与 identity 与 tally 与 gauge 与 formatter 与 meter 与 nomenclator 与 parser 零碰；台账 ledger 仅 append 短持；主树零直写；金向量冻结禁工地绝对路径。
