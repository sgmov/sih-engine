# c006-sb-solo：C006 短注类手改分卷（首程部分完成 + 续批待令）

> task-packages 治理任务
> 承接：用户 2026-08-30 开批令（c006-jc-solo 撤回收口后下一批）
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30
> 状态：**首程完成**（长描述 1139 处机械改）+ 续批待令（中短补注 376 处）

## 一、问题陈述 {#problem}

数学仓 calculus 子仓 C006 违例当前 1344 处（核阅复跑实测）。分四类：
- 真即称类 6 处（c006-jc-solo 撤回收口确认）："X（又称 Y）"、"X（Y 简称 Z）" 等
- 描述同位 3 处："X（Y 描述）" 型但语义不可删括号
- 短注类主体 ~1335 处：括号内是补充说明，可走"删括号改逗号"模式
- 公式引用类 0 处：fmtc2b-solo 已清

本批为短注类主体分卷，**目标 ~1335 处走机械改法**。6 处真即称类 + 3 处描述同位留人工不动。

前批 fmtc2b-solo 成功模式即"X（Y 引用）→ X，Y，"机械改法 + 核阅恰降对表。前批 fmtcontent1-solo 成功模式即"先 sample 句式归类 + 决定模板覆盖率 + 留人工清单"。

## 二、关键设计 {#design}

段一句式归类：机械脚本扫 1335 处短注类，按括号内长度 + 首尾字符 + 内含标点分类，输出 materials/c006-sb-patterns.tsv 句式归类表。

段二模板冻结：按归类表冻结 N 条改法模板，模板覆盖率须 > 50% 否则加批拆卷。

段三改法执行：Python 脚本按模板机械替换 + 留人工清单不动。

段四核阅对表：复跑核阅 des-001-mathe C006 计数恰降 ~1330（短注类主体）+ 6 处真即称类 + 3 处描述同位不动 ≈ 1344 - 1330 = 14 残余。

段五化格检词复跑：化格退出码零，检词零新增违例。

段六书简上链：append 段结算事件，verify 链 valid。

## 三、工作清单 {#work}

- [ ] recall 跑过切面件落 /tmp/recall-c006-sb.ndjson（已落 3 行）
- [ ] 句式归类脚本生成 materials/c006-sb-patterns.tsv
- [ ] 模板冻结表落 materials/c006-sb-templates.tsv
- [ ] 改法脚本执行恰 ~1335 实例
- [ ] 核阅复跑 C006 恰降 ~1330
- [ ] S005 / M008 / C002 / N002 计数逐一不变
- [ ] 化格退出码零
- [ ] 检词零新增违例
- [ ] 书简 append 段结算，verify 链 valid
- [ ] 结果档落 sih/event/plan/c006-sb-solo-results.md
- [ ] 双仓段结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** C006 恰降 | 工程 | 核阅 C006 降数 = 替换实例数（差 1 即回滚） |
| **F-2** 零语义变化 | 治理 | 除括号改法外 diff 无其他变更 |
| **F-3** 其余类不变 | 治理 | S005 / M008 / C002 / N002 计数逐一不变 |
| **F-4** 留人工不动 | 治理 | 6 处真即称类 + 3 处描述同位零改动 |
| **F-5** 检词化格零新增 | 治理 | 化格退出码零，检词零新增违例 |
| **F-6** 链 valid | 治理 | 事件哈希八前缀可回验 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/event/plan/fmtc2b-solo-results.md 即内容修复通道先例
- 必读 2：sih-engine/sih/event/plan/c006-jc-solo-results.md 即 c006-jc-solo 撤回收口教训与数字口径
- 必读 3：sih-engine/sih/event/plan/fmtcontent1-solo-results.md §遗留 即 C006 余量分卷建议

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 只动 sih-math/calculus/llm-friendly-build/entries/ 与本批 materials 与 trail
5. 改法脚本与决议清单留档可回验
6. 留人工处不静默，标句式标签
7. 模板覆盖率 < 50% 即停批，回报用户

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/ 短注类命中文件
- sih-engine/sih/state/plan/c006-sb-solo.md
- sih-engine/sih/state/plan/c006-sb-solo-results.md
- sih-engine/sih/event/plan/c006-sb-solo-results.md
- sih-engine/sih/event/plan/c006-sb-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 认证入链
- [ ] 结果档落位
- [ ] 双仓段结算收约

## 九、风险点 {#risks}

- 短注类句式散布，模板覆盖率可能 < 50%；不达即停批
- 改法脚本误伤即称类或描述类；防御即改法前 grep 二次确认
- 1344 - 1335 = 9 残余与"6+3=9"对得上即 F-1 可核

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 开批令
- 链件：sih-engine/sih/event/trail/2026-08-30.ndjson（本批 append）
- 关联：fmtc2b-solo 内容修复通道、c006-jc-solo 撤回教训

## 十一、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十二、后续 {#next}

- 真内容类分卷 c006-zr-solo 待令（待本批收口）
- 6 处真即称类最终处置待 rulecal 第二程
- 全仓 C006 归零进度跟踪
