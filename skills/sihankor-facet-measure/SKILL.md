---
name: "sihankor-facet-measure"
description: "Measure a governance proposition with the facet flywheel (single-seat sampling + mechanical gate v2). Invoke when the user asks to measure / adjudicate / stress-check a proposition or governance judgment, e.g. 测一下这个命题, facet 量一下, 自动裁定, or when a sih-engine development decision needs a measured verdict."
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
   gid: m-<slug>     # 可选，trail 目录名
   ---

   # 待裁命题

   <命题原文，一句断言，可判定>

   ## anchors

   - path: <相对 sih-tools 根>
     range: <行或区间>
     note: <选此锚定的原因>
   ```

   命题写法要求：一句可判定的断言；涉及"满足基线 N"须写明基线编号；
   anchors 至少一条且指向真实代码/文档行。

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

## 判读纪律 {#discipline}

- 结果不合预期**不重跑同命题**；重测仅限：人重写上下文后、或版本化判据回归
- 同 gid 重复测量 = 飞轮追加（闸看全历史），不是覆盖
- 变卦 ≥ 40% 是真不稳，不是运气差——落 boundary 是正确行为
- facet 测的是机制稳定性，不替代哲学命题推导与人类判断

## 状态 {#status}

- 判据 v2 已采纳（R3a/R3b 验收：漏放 0，误伤 7→2），闸本体在
  `sih-tools/facet/probes/maturation_gate.py`
- 链路验证：两条 AI 起草命题（清晰预期 + 刀锋预期）实测均正确落队列，
  机制端到端通（2026-08-13）
