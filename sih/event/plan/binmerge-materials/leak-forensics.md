# 六 bin 漏因查证（binmerge 批，2026-09-17）

## 事实链

- 2026-09-15 18:26 至 18:29（mtime）：动态工作流 dwfrun-be5d655b 把六腿最终版写入主树，含六 bin 既有件修改（formatter 加 locator 加 nomenclator 加 parser 加 selector 加 tally，共约 678 行）与三 results 档批尾回填。
- 2026-09-16 02:30：mergeall-closeout 开约，allow 即 sih-engine 全仓。其任务包 2.1 节原文：「git 对账分类：本批六腿件与两文档入库；并发批 M 件（formatter/locator/nomenclator/parser/selector/tally 六 bin）、sih/event 与 sih/state 旧档、sih-tools 台账面一律零触碰留原批」。
- 2026-09-16：批 commit 9ed766e 与 3da9c1d 与 508267b 共 71 件入库（含其他 bin 即 askroute 与 attnanchor 与 critsweep 等），六 bin 的修改未入。git log -S "gap-tally-parity" --all 零命中：该内容不存在于任何提交任何分支。
- 账册复核：2026-09-15/16 会话台账仅两笔即 mergeall-closeout（allow 全仓）与 adoptface（17:43，allow 为 README 与 guides 与 sih.rs 与 mcpserver，不含六 bin）。批 M 不存在于任何账册。

## 漏因三层

- 直接因：误分类。dwfrun 的 bin 既有件改动被结算批记为「并发批 M 件留原批」，批 M 系虚构；「零触碰」纪律被正确执行在错误前提上。
- 结构因：排除性分类无登记面。结算纪律允许把件排除在入库面外，但不强制给被排除件一个追踪柄（泊位、待办、具名批任一），排除即成无主孤儿。孤儿闸一点五日内每次收约都正确点名，但 bypass 通道不要求处置指针，告警重复而不收敛。
- 放大器：事由模板复读。bypass 事由首写「并发批在途件」后，后续各批（含本会话 readme-plain 加 identity-fix 加 readme-adopt 加 stdio-auto 加 adopt-guide 加 bscomplete 六批）照抄该虚构，无人翻账册验真，直至用户问「他们有什么用」触发 git log -S 复核。

## 过程缺口与建议（pk-108 承载）

建议两钉：其一 bypass-orphan 事由须附处置指针（泊位号或待办位），无指针即拒；其二孤儿件带龄告警，同一文件连续两批以上被点名即升级呈报人节点。候闸面改造裁定。
