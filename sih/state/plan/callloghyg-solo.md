# callloghyg-solo：CALL-LOG 留痕归账与台账卫生批

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：confpreempt-solo 收约阻塞呈报二（2026-09-07）即 close 无主闸拦出 16 件，其中 14 件 CALL-LOG.md 主树脏态构成即刻触发；台账卫生旧账（cert_missing 存量与陈旧会话销账）并入本批一次清
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 14 件工具目录 CALL-LOG.md 主树脏态（修改或未跟踪），各行归属已结算批但未随批入版控——收约补笔 bypass 通道未覆盖全，构成无主闸反复拦 close 的主因
- 台账旧账存量：cert_missing 历史笔对账、在册未收陈旧会话销账
- 根因无成文规则：活数据文件（calls.ndjson 类累积面）与 CALL-LOG 主树追加的入版控节奏缺位，并行批时代每批尾追加必再撞同类

## 二、关键设计 {#design}

### 2.1 CALL-LOG 归账

逐件逐行归属披露：每行载批名可回查结果档与链记录，行内批名与链上对得上方准入账；按 confrevise 收约补笔先例形 bypass 通道入版控。本批零新增 CALL-LOG 行（自身行随本批 close 走自身通道）。

### 2.2 对账与销账

cert_missing 存量逐笔对账补链或如实申报不可补缘由；陈旧会话按现势清点走 lease ledger-repair 销账通道（ledgerhyg-solo 先例）。

### 2.3 规则面提案（申报不代裁）

两件候选规则：活数据文件节奏（gitignore 化或批节奏提交二择）；CALL-LOG 追加强制随批入版控（收约检查位）。本批只提案，裁归得一裁或人节点，不在本批实施。

### 2.4 执行窗口

排在活跃会话收约之后开工，避免同类尾追加合并冲突；完工判据即 confpreempt 无主清单清零解除阻塞。

## 三、工作清单 {#work}

- [ ] 现势清点：无主清单逐件（14 件 CALL-LOG＋2 件他批生产物处置态确认）
- [ ] 逐行归属披露与 bypass 归账；cert_missing 对账；陈旧会话销账
- [ ] 规则面两提案落结果档候裁
- [ ] 复跑 watchcheck 与 close 无主闸预检读数入档；管线三步、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 归账清零 | 数据治理 | CALL-LOG 无主件清零，watchcheck 与 close 预检复跑读数在档 |
| **F-2** 归属零错配 | 数据治理 | 逐行批名与链上记录对表全过，对不上的行如实申报不入账 |
| **F-3** 旧账处置在档 | 数据治理 | cert_missing 与陈旧会话逐笔处置读数在档 |
| **F-4** 未裁零代清 | 治理 | 19 件 watchcheck 遗留与任何在飞批生产物零触碰零清除，维持呈报态 |
| **F-5** 规则只提案 | 治理 | 两件规则面零实施零代裁 |

## 五、必读文件 {#read}

- 触发源：`sih-engine/sih/state/plan/confpreempt-solo.md` 与其阻塞呈报（对话在案）
- 先例：`sih-engine/sih/event/plan/ledgerhyg-solo-results.md`（销账与卫生先例）、confrevise 收约补笔通道先例
- 协议：`sih-tools/BATCH-FACE.md` watch 对表挂点与处置协议节

## 六、约束 {#constraints}

1. 他批在飞生产物（attnanchor／calllog 等）零触碰——归属方自收；停滞即呈人节点二值裁决
2. 19 件 watchcheck 遗留不豁免不代清维持候裁
3. bypass 通道逐笔登记台账；主树零直写除 bypass 登记面外
4. 在泊件零触碰

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过，settle 在档，链 verify valid，reconcile 零新增
- [ ] confpreempt-solo 无主阻塞解除读数在档（其 close 归其自身批完成）
- [ ] 结果档 callloghyg-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 活跃会话迟迟不收 → 窗口顺延如实申报，不抢跑
- 行归属对不上链记录 → 该行如实申报悬置不入账，不硬归

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（写位）、`sih-tools/lease`（ledger-repair）
- 受益：confpreempt-solo 收约解除阻塞

## 十一、请求写入 {#requested-writes}

- `sih-tools/`（各 CALL-LOG.md 归账面）
- `sih-engine/sih/state/plan/callloghyg-solo.md`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/callloghyg-solo-results.md` 与 `callloghyg-solo-materials/`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md` 与 `sih-tools/lease/CALL-LOG.md`
- worktrees 相关仓 callloghyg-solo 工地
