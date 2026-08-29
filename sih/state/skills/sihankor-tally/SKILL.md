---
name: "sihankor-tally"
description: "执契／tally：facet 裁决材料的确定性基线核对与机器终签。Invoke when a facet measurement needs checking or signing (核对这份材料 / 执契落据 / 终签), or after any facet run that produced a stable_clear verdict, or when the user asks 执契."
---

# 执契调用壳

本文件是 执契（sih-tools/tally）的调用薄壳，只承载触发语义与命令，语义权威即工具契约（sih-tools/tally/CONTRACT.md）与 DES-011，壳与契约冲突以契约为准。

## 触发时机 {#trigger}

- facet 测量产出裁决材料后需要核对（「核对一下这份材料」「执契过一遍」）
- 闸判 stable_clear 后的机器终签落据（修订六：终签主体是确定性脚本，agent 一字不签）
- 用户问某份裁决材料能不能过、处置是什么

## 调用 {#invoke}

```
cd sih-tools/tally 后
uv run tally assemble --gid <gid> --des-root <命题区DES目录> \
  --topics-dir <topics目录 可重复> --baseline <席位当日基线.json> \
  --date <YYYY-MM-DD> --out <裁决材料输出路径>
uv run tally check --material <裁决材料.json>
uv run tally verify --material <裁决材料.json> --report <既报核对报告.json>
uv run tally sign --material <裁决材料.json> --out <报告目录> \
  --trail <引擎事件链> --scribe-binary <引擎scribe路径> \
  --session <会话号> --locks <锁台账>
```

标准流程：assemble → check → verify →（仅裁决通过）sign。退出码 0 过、1 败或拒、2 环境错。

## 材料与前置 {#preconditions}

- assemble 自动发现 DES/<gid>/ 下计分材料，择 responses 哈希与当前响应文件一致的那份（即最新计分）；topic 按合同哈希在 topics 目录匹配发现，匹配不到如实报错不猜
- 席位当日基线必传：温度探针模式一（agent 答自己的模型）或模式二产出，缺席即 R5 如实败
- 判定流超过九发即 R6 如实败（预算上限）

## 边界 {#boundary}

- 处置四值：裁决通过、材料退回、挂起、打回重作；**不收敛（挂起/打回）归人诊断——遗漏条件还是命题错误**，agent 不得改答重测钓鱼
- sign 仅裁决通过放行；agent 一字不签（P3 原文留痕不抹，修订六显式接替常设人签核位）
- 落据经既有 scribe append 认证位；本工具不认证、不写命题区、零 LLM

## 状态 {#status}

- 1.0.0 实装批 tallyimpl-solo 交付（R1 至 R7、四值处置、verify、sign），接线批 tallywire-solo 落 assemble 与调用壳
- 立名承 DEC-019，契约源 DES-011
