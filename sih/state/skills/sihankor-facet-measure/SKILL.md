---
name: "sihankor-facet-measure"
description: "Measure a governance proposition with the facet flywheel (single-seat sampling + mechanical gate v2). Invoke when the user asks to measure / adjudicate / stress-check a proposition or governance judgment, e.g. 测一下这个命题, facet 量一下, 自动裁定, or when a sih-engine development decision needs a measured verdict. 无探针环境或框架自带模型采样走合同模式（修订五）。"
---

# SiHankor Facet Measure Skill

facet 上下文风洞的调用入口：给一条有锚定的治理命题做测量，产出机械闸裁决
（判据 v2）+ trail 留痕 + 人话一页。工具在 sih-tools（不受 sih-engine 治理
约束），本 skill 只做调用编排，承接轻量模式与固化判据的全部结论。

## 触发时机 {#trigger}

- 用户说"测一下这个命题 / facet 量一下 / 自动裁定 / 给这个判断做测量"
- 司衡引擎开发中遇到有锚定的治理判断（草案命题、边界裁决），需要测量支撑
- 用户问到 facet 工具的用法

不触发：纯事实查询；无锚定的纯讨论（先补锚定再测）。

## 自动化边界（先读，编码执行） {#boundary}

**允许 agent 全自动完成**：
- 起草命题文件（命题原文 + anchors + ng 强度）
- 调用 `measure.py`、读取结果、转述结论、刷新视图
- 常规命题的**描述性裁定**：清晰命题自动得出符合/违反 + 落队列

**人节点，agent 不得代办**：
- Layer 2 签核（promote/reject/hold）。stable_clear = **待人签核**，不是自动放行。
  agent 代签 = violate（P3 自反裁决 9/9，rg-P3 已签核判定成立）
- boundary（打回重作）的**上下文重写**：刀锋命题须人重写措辞/上下文后重测，
  agent 自行改写重测 = 调参钓鱼邻域，禁止
- 任何超时默认同意 / 默示放行类机制：见 m-skillv-timeout-sign 测量（boundary）

**硬底线**：测量结果出现刀锋命题 stable_clear（漏放）→ 立即报告用户，不等不补。

## 前置检查 {#precheck}

1. `sih-tools/facet/measure.py` 存在（缺失则报错，不替用户实现）
2. `sih-tools/facet/.env` 存在且含 MINIMAX key
3. 命题已有锚定（无锚定先向用户要，或从所指文件定位后复述确认）

## 执行步骤 {#steps}

1. **起草命题文件**（agent 代笔，路径 `sih-tools/proposition/topics/<日期>-<slug>.md`）：

   ```
   ---
   title: <一句话标题>
   authored: <来源>（AI 起草须如实署名）
   ng: medium        # none | medium，默认 medium
   n: 5              # 采样次数，默认 5
   gid: m-<slug>     # 可选，trail 目录标识
   ---

   # 待裁命题

   <命题原文，一句断言，可判定>

   ## anchors

   - path: <相对 sih-tools 根>
     range: <行或区间>
     note: <选此锚定的原因>
   ```

命题写法要求：一句可判定的断言；涉及"满足基线 N"须写明基线编号；
anchors 至少一条且指向真实代码/文档行。命题只承载材料性断言即事实、机制、
状态、结构，不得请求裁决意见即不含处置裁断形（裁X、采纳裁、出泊转正裁），
facet 出裁决材料不出裁决意见，收敛点在确定性程序，裁决权归确定性程序按
已签署规则执行与人节点，承本 skill 修订三。材料性断言再分两形：机制规则类
可入采样；状态完成类即某确定性操作已执行已完成的断言归确定性管线，以命令
输出为证据链，不送采样，祈使尾巴同禁。

受理时前置分道：agent 收到测量请求先判任务类，四类命题不送采样直接改道并
向用户披露依据先例，即裁断形、祈使尾形、状态完成类、跨文档记载比对类。前
两类为起草红线，后两类归确定性管线与确定性核对通道机械执行。跨文档记载比
对类即哈希存在性、条目计数、逐字映射对记载工件的核对，先例即
m-sett001-crosscheck 飞轮四发边界旗同指任务类错配。判据即命题能否被机械操
作核对：能即改道确定性通道；要读出意思、判断机制或措辞的才入采样；混合命
题按认识论通道拆开分道。改道即报用户依据，用户可当场否决，否决即照跑如实
留痕。前置分道由 agent 执行，正当性来自已签署规则加先例不来自判断力，承本
skill 修订四。

2. **跑测量**：`cd sih-tools && python3 facet/measure.py proposition/topics/<文件>.md`
   墙钟约 100s（5 发）。不要向用户披露采样/闸内部细节。

3. **读结论**（输出四行关键）：
   - 判定方向（符合/违反 + 采样一致性）
   - 谨慎信号 x/N、规约引用类数
   - 闸门裁决三态：清晰稳定（**待人签核**）/ 接近临界（待人复核）/ 打回重作

4. **汇报**（人话，三句内）：判定方向与稳定性、落进哪个队列、下一步是人节点
   （签核/复核/重写）还是无需动作。stable_clear 时提示用户可用 layer2_signoff 签核。

5. **留痕**：trail 自动落 `proposition/DES/<gid>/`；如需全局面貌跑
   `python3 facet/views/dashboard.py`。

## 合同模式（无探针通用调用形态，修订五） {#contract-mode}

facet 调用面两半切分（CONTRACT-MODE-SPEC v1，sih-tools/facet/docs）：facet 侧
确定性内核四件即出题、计分、闸门、验证，零 LLM 调用零网络零 key；框架侧一
动作即执行采样合同。模式一的「采样归 agent、计分归程序」由此从温度标定特例
升格为通用调用形态，温度探针两模式保留为标定特例。

1. **出题**：`python3 facet/measure.py <topic.md> --emit-contract <out.json> --seat <框架:模型> [--shots N]`，产采样合同（每发 system_prompt + user_prompt，装配确定性）。探针集规模用 `python3 facet/singleseat.py emit-contract --registry <reg.json> --date <YYYYMMDD> --out <dir>`，剂量格用 `python3 facet/probes/dose_driver.py emit-contract`。
2. **执行（框架侧）**：把合同内每条提示词原样发给框架自己的模型，模型完整原文逐发写响应 jsonl（每行恰 key/shot/raw 三键），不改写、不摘要、不挑选——与模式一 cal-responses 纪律同款。
3. **计分**：`python3 facet/measure.py <topic.md> --score <responses.jsonl> --contract <contract.json>`，响应严格对表（缺发即拒收整批）、逐发入飞轮 trail、过判据 v3 闸；探针集 `singleseat.py score-contract` 后 gate/assemble/verify 照常；剂量格 `dose_driver.py score`。计分材料带合同哈希与响应哈希，复算不符即作废。
4. **裁决权不变**：stable_clear 待人签核、前置分道、boundary 次序照旧；红线延采不自动触发，落带时以更多响应重跑 score 即飞轮追加。

## 温度探针（席位基线标定，agent 调用入口） {#temp-probe}

两种模式，按**被测席位的归属**选：

**模式一（agent 框架适配，默认）**——量的是 agent 自己的模型，走 agent 自己的通道，
不经过 facet 后台 key。采样归 agent，计分归程序：

1. 导出标定包：`python3 facet/probes/temp_probe.py export-pack --out cal-pack.json`
2. agent 按包内 `response_contract` 执行：对每条命题把 system_prompt + user_prompt
   原样发给**自己的模型**（温度 0），每命题 5 发，把模型完整原文逐发记入
   `cal-responses.jsonl`（`{"key":..., "shot":1..5, "raw":<模型原文>}`）。
   agent 不得改写、摘要、挑选模型输出。
3. 程序计分：`python3 facet/probes/temp_probe.py score --responses cal-responses.jsonl --seat <框架:模型:版本>`

**模式二（facet 后台席位）**——量我们注册表内的席位（走 .env key）：

```bash
python3 facet/probes/temp_probe.py --model MiniMax-M2.7   # 默认免费席
```

判读（程序自动输出，agent 只转述）：
- `判定=可用`：该席位当日基线有效，谨慎信号按其体温做相对判读
- `判定=漂移告警`：与既往账本（同席位同 pack_version）比 modal 翻转或体温 |Δ|≥0.2
  ——历史数据不可比，相关判据重估，报告用户（OQ-20：交付后每日重测，观察 30-90 天）
- `判定=基线异常`：清晰越权命题未 5/5 判违——该席位当日不得用于治理裁决，立即报告
- 基线绑定席位+版本+日期+pack_version，任一变更即重量（多主题重跑已证跨家族 modal 会翻）
- 模式二非 MiniMax 席位产生积分消耗（GLM 类思考型先 10 发试水）

## 判读纪律 {#discipline}

- 结果不合预期**不重跑同命题**；重测仅限：人重写上下文后、或版本化判据回归
- 同 gid 重复测量 = 飞轮追加（闸看全历史），不是覆盖
- 变卦 ≥ 40% 是真不稳，不是运气差——落 boundary 是正确行为
- facet 测的是机制稳定性，不替代哲学命题推导与人类判断

## 修订记录

2026-08-22 修订一，boundary 处理次序。用户裁定 facet 目的为人节点退化与反洪流，闸门返回打回重作时 agent 第一动作是法层精炼而非人节点重写：按认识论通道分解命题，文档一致性类断言重立为反向搜寻形即不一致搜寻，新 gid 测量且方向须对己不利以防调参钓鱼；机械可验性类断言路由确定性管线即 golden 对拍与测试；权力链事实类断言路由登记簿。原裁决留痕不抹。原「刀锋命题须人重写上下文」限位为法层精炼无法消解时的兜底，不再为第一跳。若反向搜寻形仍 boundary 且反对集中于任务类本身即跨文档比对属确定性可核任务，则裁定为通道错配即 facet 拒绝受理该任务类，命题落确定性核对通道机械执行，不三跑。

2026-08-22 修订二，确定性通道的执行纪律与人话汇报。用户同日裁定：令 facet 裁即人节点退位，boundary 经通道分解落确定性通道后，凡有已签署规则可据即按该规则默认执行并留痕，不再向用户补设签认节点；签认节点仅存于 facet 自身闸门的 stable_clear 即 P3 已核之位。agent 汇报一律大白话完整句，不用仓内压缩语体，用户不是本仓代码的作者。

2026-08-25 修订三，命题形态纪律。用户裁定：facet 出裁决材料，不给裁决意见，裁决权利与责任归确定性程序和人节点，确定性程序是过 facet 裁决材料的收敛点，承 PRO-07 之 A-A4.1 候选建议生成器与 A-A4.2 裁决权归确定性引擎，同 OPEN-QUESTIONS 原 OQ-22 二层结构。命题红线：不得含处置裁断形，只承载材料性断言即事实、机制、状态、结构；facet 产出为裁决材料，收敛由确定性程序按已签署规则机械执行，未尽处归人节点。触发即同日三件泊界出泊命题连打回，15 发齐指 AI 起草含裁字即 LLM 出裁定，形态错误被飞轮自身拦截。同日补条：状态完成类断言即已删除已承接已完成形，证据链职权在确定性程序命令输出，LLM 散文断言送采样属类别错误，触发即 counter 可删性命题三发违票齐指，连同当日裁断形与祈使尾形两度拒收，三形证据俱在案。

2026-08-25 修订四，受理时前置分道固化。用户裁定：四类命题在受理时即拒绝送采样直接改道，即裁断形、祈使尾形、状态完成类、跨文档记载比对类，前两类为起草红线，后两类归确定性管线与确定性核对通道，先例即 m-sett001-crosscheck 即反向搜寻形仍 boundary 且四发边界旗同指跨文档比对属确定性可核任务。改道动作即披露依据先例、机械执行、结果作裁决材料交人节点，用户可当场否决照跑留痕。前置分道由 agent 执行，正当性来自已签署规则加先例不来自判断力，每次改道报依据。

2026-08-29 修订五，合同模式升格。用户批准 facet 无探针改造即调用面两半切分：facet 侧确定性内核四件加框架侧一动作执行采样合同（CONTRACT-MODE-SPEC v1，contract01-solo 批实现）。模式一采样归 agent 计分归程序的特例升格为通用调用形态，温度探针两模式保留为标定特例；合同执行纪律即逐发原文不改写不摘要不挑选、响应齐套才计分缺发拒收整批、计分材料哈希绑定复算不符即作废；裁决权与人签核与前置分道全不变。落锚依据：recall「合同模式调用面」「采样合同」零直接先例如实记，先例即本 skill 模式一与 SINGLE-SEAT-SPEC v1 三腿规格与 T5 跨模型底稿同包收敛实证。

## 状态 {#status}

- 判据 v2 已采纳（R3a/R3b 验收：漏放 0，误伤 7→2），闸本体在
  `sih-tools/facet/probes/maturation_gate.py`
- 链路验证：两条 AI 起草命题（清晰预期 + 刀锋预期）实测均正确落队列，
  机制端到端通（2026-08-13）
