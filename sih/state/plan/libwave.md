# libwave：哈希归并加工具索引加标准件批

> 令源：用户 2026-09-23 令「继续」承接 2026-09-22「得一裁，继续推进」，按完工报告所列当下无阻塞三件开波
> 输入：auditfix 勘误后迭代建议报告 REC-008 与 REC-010 与 REC-012 与 REC-018 余项、审计实测（sha256_hex 13 bin、src/bin 36 件、lib 12 mod）
> stem 认领：libwave，甲表三件即 zh 库归并波、code 无承、派生 libwave:new（查册 unknown 在案）

## 问题陈述 {#problem}

13 个内部工具 bin 各自手抄 fn sha256_hex（实现相同），改哈希算法须改 13 处；引擎 AGENTS.md 只列 4 加 2 阶段组件，36 个内部工具 bin 与 12 个 lib 模块无索引，新人误读仓面貌；CONTRIBUTING 与 CHANGELOG 标准件缺席；README 与 RANTS 的 DES-001 适用界未成文，RANTS 风格豁免未标注。

## 关键设计 {#design}

四件合一批：其一 DEC-031 落据（hashutil 本批落地加 cliargs 候后批）与 DEC-033 落据（README 三例外加 RANTS 豁免）与 DES-020 设计稿（AGENTS.md 两段索引政策）；其二 src/hashutil.rs 新件载 pub fn sha256_hex 并转换 13 bin 调用位，转换前提是 13 份实现逐字等价核验（有异文即停批上报），新件头注载 DEC-031 正典指针；其三引擎 AGENTS.md 补第二段内部工具索引（36 bin 名加行数加一句话职能，取各件头注首行）与 12 mod 清单，全文控制在 200 行内；其三 CONTRIBUTING 摘要件与 CHANGELOG 0.9.0 候发布节；其四 RANTS.md frontmatter 豁免标注。AGENTS.md 与三份根级件均在 des-001 域外，管线不适用照实记。

## 工作清单 {#work}

- lw-01：三文档落据（lw-dec 子代理）
- lw-02：hashutil 归并 13 bin（lw-hash 子代理）
- lw-03：AGENTS.md 索引补全（lw-index 子代理）
- lw-04：CONTRIBUTING 加 CHANGELOG 加 RANTS 标注（lw-std 子代理）
- lw-05：编排位核验加双跑对表抽检加认证收约（主会）

## 验收 {#acceptance}

grep fn sha256_hex src/bin 零命中且 lib 恰一处；cargo build 全绿加 cargo test --bin lease 加 --lib event_stream 全绿；gauge 与 formatter 双跑对表抽检逐字节一致；三文档核阅 exit 0（DES-020 与 DEC-031 与 DEC-033）；AGENTS.md 行数不超 200；当日链 valid；收约 SDDG 四判据全过。

## 请求写入 {#requested-writes}

- sih-engine/doc/decision/031-hashutil-cliargs-merge.md
- sih-engine/doc/decision/033-des-001-scope-clarification.md
- sih-engine/doc/design/DES-020-agents-md-internal-bins-index.md
- sih-engine/src/hashutil.rs
- sih-engine/src/lib.rs
- sih-engine/src/bin/acceptor.rs
- sih-engine/src/bin/cascade.rs
- sih-engine/src/bin/formatter.rs
- sih-engine/src/bin/basemgr.rs
- sih-engine/src/bin/confledger.rs
- sih-engine/src/bin/identity.rs
- sih-engine/src/bin/incubation.rs
- sih-engine/src/bin/lease.rs
- sih-engine/src/bin/gauge.rs
- sih-engine/src/bin/projsnap.rs
- sih-engine/src/bin/locator.rs
- sih-engine/src/bin/parser.rs
- sih-engine/src/bin/tally.rs
- sih-engine/AGENTS.md
- sih-engine/CONTRIBUTING.md
- sih-engine/CHANGELOG.md
- sih-engine/RANTS.md
- sih-engine/sih/event/plan/libwave-materials/
