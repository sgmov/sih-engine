# sealwin3-solo：双仓封窗界线前移批

> task-packages 治理任务
> 承接：用户 2026-09-01 封窗令、sealwin2-solo 先例、切换批归并点即本批界线锚
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-01

## 一、问题陈述 {#problem}

sealwin2 界线后存量再积：engine unrouted 七笔即 goldfix 三笔直提加 tdfix 两笔加 viewfix 与 viewrider 两笔归并，tools unrouted 五笔即 goldfix 三笔加 ordfound 收尾笔加 viewrider 工具侧，另 tools cert_missing 八笔旧账。用户封窗令下界线前移至切换批归并点，恢复零即净信号，改史零发生即封窗只移界线。

## 二、关键设计 {#design}

三件。一即界线前移：lease commitcore.py 的 SEAL_BASES 双仓界线各前移至切换批在本仓的归并点即 merge commit 哈希，SEAL_EXEMPTS 追认表零改动。二即追认档：结果档全列被封二十笔即逐笔 sha 加主题加定性即直提违章或归并无前缀或认证缺席，透明封窗非沉默掩埋，cert_missing 八笔开工时以 reconcile 明细枚举。三即验证与升版：双仓 reconcile 后 unrouted 零与 cert_missing 零即 sealed 计数如实出，lease 升 1.11.0 与 CONTRACT 修订记录，测试全绿。

## 三、工作清单 {#work}

- [ ] 界线双仓前移与测试
- [ ] 追认档全列
- [ ] CONTRACT 修订与升版
- [ ] 收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 界线 | 工程治理 | SEAL_BASES 双仓各指切换批归并点 merge 哈希，SEAL_EXEMPTS 零改动，测试全绿 |
| **F-2** 追认档 | 工程治理 | 二十笔逐笔 sha 加主题加定性在档，cert_missing 明细开工时枚举零漏 |
| **F-3** 信号恢复 | 工程治理 | 双仓 reconcile 即 unrouted 零与 cert_missing 零即 sealed 计数如实 |
| **F-4** 收口 | 链上治理 | 正规 settle 路径即 worktree 加归并、链 valid、本批产物含链尾全量入册 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/src/lease/commitcore.py 即 SEAL_BASES 与 SEAL_EXEMPTS
- 必读 2：sih-engine/sih/event/plan/sealwin2-solo-results.md 即封窗先例

## 六、约束 {#constraints}

1. 零改史即封窗只移界线不碰任何 git 历史（红线）
2. 界线必须取归并点即 merge commit（红线）
3. SEAL_EXEMPTS 零改动（红线）
4. 本批提交全走 lease commit 正规路径，严禁直提主线（红线）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

界线取非归并点即信号失真，防御即 resolve_default_base 双仓实跑核 base_source 为 seal_line。追认档漏笔即掩埋，防御即 reconcile 明细与档逐笔对表。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-01 封窗令
- 链件：随批意图入当日链
- 关联：sealwin2-solo 先例、切换批归并点、goldfix 直提六笔

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[封窗]: 消解 即 lease 既立术语承 sealwin2 先例
叩问处置[追认档]: 消解 即大白话直述即封窗明细档，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/commitcore.py
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/pyproject.toml
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/CALL-LOG.md
- sih-engine/sih/state/plan/sealwin3-solo.md
- sih-engine/sih/event/plan/sealwin3-solo-results.md
- sih-engine/sih/event/plan/sealwin3-solo-materials/
- sih-engine/sih/event/trail/<当日>.ndjson
- sih-engine/sih/event/inputlog/<当日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
