# basefix-solo：地基修复批（locks 互斥绕穿、scribe 追加原子化、close 并集复查、泊界投影同步）

- 令源（用户裁定，2026-09-04）：pk-051 修复插到载体接线前，并行批安静窗口开工——台面已清（活锁零）即本批窗口。
- 承接：pk-051（locks 判定面偏差两笔，leasewire-solo 实查发现并冻结 as-is 金向量 l4）；pk-045 样本库 trail 竞态类两起实案（facepark-solo 丢两笔、leasewire-solo close 丢 pk050sw 认证一笔，后者已由主会处置位补笔复原 7e1aa2，事由在事件内）；DEC-013 融回序位注记（租约将来融回，地基先行）。
- 日期：2026-09-04（trail 用 sih/event/trail/2026-09-04.ndjson，会话号 sess-zcode-260904-basefix）｜ 队形：单线形 solo，零子代理。行为变更批：SDD 先行、TDD 先红后绿、判变申报。

## F 清单

- F-1 locks 互斥绕穿修复：active_locks 放后重取配对洞（他会话再取被放行）与 normalize_path 绝对路径未同一化两笔，SDD 钉死修复形态与不变量（互斥不变式：任一时刻任一路径至多一持锁会话），TDD 先红后绿；l4 金向量判变申报并重冻（as-is 旧向量留档披露不删）；推导注记 sih-math/docs/basefix-derivation-2026-09-04.md（配对洞即 ORD-020 全序性破坏点，修复即回归载体，注记级不立新条目）。
- F-2 scribe 追加原子化：引擎 event_stream append 位由读算整写改并发安全形（SDD 裁形：进程间文件锁或单写原语，二选一申报依据），双进程并发追加竞测先红后绿；语义零变（事件内容与哈希公式不动），引擎全套测试零回归，ABS 同形不适用（行为语义位）但退出码与事件 schema 零变。
- F-3 close 并集复查：lease close 归并前重读现行链重算并集超集，不信任早前快照（leasewire 事故根因即 close 复跑用旧快照 clobber 并行批一笔）；TDD 模拟快照后并发追加场景先红后绿；BATCH-FACE 勘误节一行同步（close 复跑前必复查并集，代码闸已加）。
- F-4 泊界投影同步：名册照链补齐 pk-048 与 pk-049 与 pk-050 与 pk-051 四行与在泊计数、pk-051 出泊行（本批 F-1 即其出泊条件路径，出泊记账随批）、pk-046 行加 SPEC-017 号位撞注记（照录 pk-043 与 pk-047 先例）、materials 补 pk-051.json 逐字段抄链上停泊事件；名册过化格核阅检词三门。
- F-5 判变申报：locks 行为变更只向前生效，历史批锁操作账目零改写；scribe 原子化语义零变不涉判变；两笔申报入结果档。

## 管线与机械链

ask3 记录（三锚程序切片禁手打）→ 双门 → 叩问 elicit → 正身 → lease open --package basefix-solo → 取锁（施工面 locks 与 lease 与引擎 event_stream 源 exclusive 长持；共享追加面一律 --mode append 短持）→ meter 包裹引擎 scribe intent 上链（闸三 --sessions 带）→ 工地施工（SDD spec → TDD 三件 → 推导注记 → 名册与 materials）→ 化格→核阅→检词（findings 亲读）→ 认证逐笔 append 主树活链 → settle 前一次性拷工地 → 三仓 settle --cert → 放锁 → close（备份让位归并对表法，close 自身用主树现行版）→ reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH 前缀；meter 包裹 2>/dev/null 对治；禁管道掩退出码；close 通道外提交 --no-verify 加 lease bypass 登记。

## 请求写入（锁路径全集）

- sih-tools/locks/ 与 sih-tools/lease/（源码、测试、CONTRACT、CALL-LOG）
- sih-engine/src/event_stream/ 与 src/bin/scribe.rs 与引擎测试
- sih-engine/doc/governance/PARKING-v1.md 与 sih-engine/sih/state/parking/materials/pk-051.json
- sih-engine/doc/spec/ 新 spec（号实取下一空位）
- sih-math/docs/basefix-derivation-2026-09-04.md
- sih-tools/BATCH-FACE.md 勘误节一行
- sih-engine/sih/event/plan/basefix-solo-materials/ 与 basefix-solo-results.md
- sih-engine/sih/event/trail/2026-09-04.ndjson（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

历史链文件与已收批账目零改写；数学仓 entries 与 INDEX 与 mapping.md 零触碰；retriever 与 identity 与 tally 与 gauge 与 formatter 与 meter 与 nomenclator 与 parser 与 cascade 与 selector 零碰；主树零直写；金向量冻结禁工地绝对路径；scribe 事件 schema 与哈希公式零触碰（只动写入并发形）。
