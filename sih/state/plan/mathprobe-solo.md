# mathprobe-solo：数学仓健壮摸底批

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图即 2026-08-30 链事件 a490d08e，用户同日令即工程侧以命题到数学条目到代码链能否机械校验为准，数学仓承接不了是数学仓的问题
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

数学仓是哲学到工程的正式桥梁，但当前无版本控制无基线，一切机械治理挂载不上；章程与仓内实体两处口径矛盾（文件索引称 calculus 113 个已建条目，数学仓地位节称已建 4 其余 89 待建）；非微积分 13 条目实体文件数与各子仓 INDEX 已建数漂移；条目双答结构与哲学锚点从未逐条体检；des-001-mathe 规则包 2026-08-25 审计档记 401 违规而 ai-ex 记 0 违规，两源矛盾未定性。桥梁承接能力不可知即阶段二补全无从排序。

## 二、关键设计 {#design}

零号动作先 git init 全量基线提交使仓可治理，此后全部改动走 git。底数盘点以机械清点为准（find 加 wc），条目分已建与待建两态以双答结构在场性判定。体检四查即数学正确性自查留验算痕迹、双答齐不齐、哲学锚点回原文逐字核验、工程调用位活性，逐条产出可复算清单。三处一致性即各子仓 mapping 与 INDEX 与实体条目互查，漂移列勘误清单。核阅 des-001-mathe 跑全部 entries/*.md，违例如实报。摸底报告五段即底数表、零号发现、体检结果、勘误清单、补全优先级建议，经三管后认证上链。章程勘误只建议不代改，归用户裁。

## 三、工作清单 {#work}

- [ ] 零号：git init 全量基线提交（已毕 c556abb）
- [ ] 底数盘点：五子仓登记数、实体数、已建待建分类表
- [ ] 体检四查：13 条非微积分条目全查，calculus 按双答在场性分类后对已建类抽全查
- [ ] 三处一致性互查与勘误清单
- [ ] 核阅 des-001-mathe 跑全部 entries
- [ ] 摸底报告起草、三管校验、认证上链
- [ ] 双仓段结算收约、六链对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 基线与底数 | 工程 | sih-math 为 git 仓且基线提交在，底数表每格可复算，章程两处矛盾各出实测勘误建议 |
| **F-2** 体检四查 | 治理 | 13 条非微积分条目逐条四查结果在档，哲学锚点引文逐字比对留证据位，工程调用位逐条验存在性 |
| **F-3** 一致性与核阅 | 工程治理 | mapping 与 INDEX 与实体三处互查漂移全列清单；核阅跑全部 entries 结果如实入档，违例数与 2026-08-25 审计档两源矛盾定性 |

## 五、必读文件 {#read}

- 必读 1：sih-math/INDEX.md 即总索引与桥接规约
- 必读 2：sih-math/docs/T6-FINAL-AUDIT-2026-08-25.md 即前轮审计基线
- 必读 3：根 AGENTS.md § 数学仓地位即章程口径

## 六、约束 {#constraints}

1. 数学仓不产出治理命题
2. 不改哲学仓原文，疑误记报告交人
3. 不动 sih-engine 与 sih-tools 正本（报告与日志件除外）
4. 章程勘误只建议不代改
5. 零 LLM 调用于工具执行层
6. 词债不过夜，上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 摸底报告落 sih-math/docs/，认证入链，结果档落 sih/event/plan/mathprobe-solo-results.md，三仓段结算收约

## 八、请求写入 {#requested-writes}

- sih-math/
- sih-engine/sih/state/plan/mathprobe-solo.md
- sih-engine/sih/state/plan/mathprobe-solo-results.md
- sih-engine/sih/event/plan/mathprobe-solo-results.md
- sih-engine/sih/event/plan/mathprobe-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 九、风险点 {#risks}

- 核阅 des-001-mathe 路径模式只覆盖子仓 entries/*.md，calculus 条目实居 calculus/llm-friendly-build/entries/，可能全数漏跑，属包覆盖缺口如实记不改包（改包走版本管理另批）
- 共享日链有并行会话在写，收约归并可能遇分叉，按既有路标先例重放
