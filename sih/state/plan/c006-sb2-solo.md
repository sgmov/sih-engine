# c006-sb2-solo：C006 中补注类手改分卷第二程

> task-packages 治理任务
> 承接：c006-sb-solo 首程部分完成（1066 降，差 151）+ 用户 2026-08-30 开 A 令
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

c006-sb-solo 首程机械改长描述 1217 处，C006 1344 → 278 降 1066。差 151 = 短补注 6 + 中补注 145。本批取**中补注 6-15 字符**为第二程。短补注 6 处留 c006-sb3-solo。9 处真即称类与描述同位留人工。

## 二、关键设计 {#design}

段一句式归类：扫 370 处中补注按首尾字符与内含标点分类，输出 materials/c006-sb2-patterns.tsv。

段二模板冻结：
- "X（Y 描述）"型（描述/同位）→ 删括号加逗号（与首程同款）
- "X（Y 引用 ID）"型（如 "X（INT-022）"）→ 删括号 + 改"X，Y"或"X Y"
- "X（Y 简注）"型（如"X（y 轴）"但 6-15 字符）→ 改半角括号（X(y 轴)）或保留
- "X（数学条件）"型（如"X（x >= 1）"）→ 改半角或保留

段三改法执行：Python 脚本按句式类型分流处理。

段四核阅对表：核阅 C006 降数 = 替换数，差 1 回滚。

段五化格检词：化格 0 改，检词 skipped。

段六本程收口：通过 lease 走通 SOP（open → lock → commit → close）+ scribe append 上链，承 c006-sb-solo 首程 + c006-sb2-solo 续批合并上链。

## 三、工作清单 {#work}

- [ ] 句式归类脚本生成 materials/c006-sb2-patterns.tsv
- [ ] 模板冻结表落 materials/c006-sb2-templates.tsv
- [ ] 改法脚本执行恰 ~370 实例
- [ ] 核阅复跑 C006 降 ~370
- [ ] 化格 0 改
- [ ] 检词 skipped
- [ ] lease open（c006-sb2-solo 包）
- [ ] lease lock 路径锁
- [ ] 改法执行
- [ ] 核阅 + 化格 + 检词
- [ ] lease commit
- [ ] lease close
- [ ] scribe append 上链
- [ ] scribe verify 链 valid
- [ ] 结果档落 sih/event/plan/c006-sb2-solo-results.md
- [ ] 双仓段结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** C006 恰降 | 工程 | 核阅 C006 降数 = 替换实例数 |
| **F-2** 零语义变化 | 治理 | 除括号改法外 diff 无其他变更 |
| **F-3** 其余类不变 | 治理 | S005 / M008 / C002 / N002 计数逐一不变 |
| **F-4** 留人工不动 | 治理 | 6 处真即称类 + 3 处描述同位 + 6 处短补注 零改动 |
| **F-5** 化格检词零新增 | 治理 | 化格 0 改，检词 skipped |
| **F-6** lease 通 | 治理 | open → lock → commit → close 全过 |
| **F-7** 链 valid | 治理 | 事件哈希可回验，verify 退出码零 |

## 五、必读文件 {#read}

- 必读 1：c006-sb-solo-results.md 即首程结果与教训
- 必读 2：c006-jc-solo-results.md 即数字口径
- 必读 3：sih-tools/scribe/CONTRACT.md 即上链契约

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 改法须按句式分流，不能一刀切
5. 留人工处不静默，标句式标签
6. 本程必须走通 lease 全程

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/ 中补注命中文件
- sih-engine/sih/state/plan/c006-sb2-solo.md
- sih-engine/sih/state/plan/c006-sb2-solo-results.md
- sih-engine/sih/event/plan/c006-sb2-solo-results.md
- sih-engine/sih/event/plan/c006-sb2-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-7 全过
- [ ] 链 valid
- [ ] lease close
- [ ] 双仓段结算收约

## 九、风险点 {#risks}

- 句式分流漏判某类导致误改
- lease 流程在手动阶段可能卡 identity/intent 缺
- 续批与首程合并上链需保持 prev_hash 正确

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 开 A 令
- 链件：sih-engine/sih/event/trail/2026-08-30.ndjson（c006-sb-solo 首程 + 本程）
- 关联：c006-sb-solo 首程、c006-jc-solo 撤回收口

## 十一、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十二、后续 {#next}

- c006-sb3-solo 短补注 6 处续批（待令）
- 6 处真即称类 + 3 处描述同位最终处置（待 rulecal 第二程）
- 全仓 C006 归零进度跟踪
