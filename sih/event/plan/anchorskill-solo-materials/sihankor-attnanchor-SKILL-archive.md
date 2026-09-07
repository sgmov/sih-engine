---
name: "sihankor-attnanchor"
description: "SiHankor attnanchor 注入式回锚。Invoke at session start after self-check (会话启动第四件), after batch settlement (批结算收约后), or when the user says 回锚, 锚一下, anchor. Zero-LLM recall of five anchor lines."
---

# attnanchor 回锚壳

本文件是 attnanchor（sih-tools/attnanchor）的调用薄壳，只承载触发语义与命令，语义权威即工具契约，壳与契约冲突时以契约为准。2026-09-07 起 UserPromptSubmit 钩子退役，触发层为本壳即行为承载锚，静默失效以完工回显补偿。

## 调用

```
python3 sih-tools/attnanchor/anchor.py    # 回算五行锚，严格 JSON 单键 additionalContext
```

## 触发位

- 会话启动：自检加载后、例行读数前跑一次
- 批结算收约后跑一次，完工报告回显当日五行锚
- 任务切换时改写 .session-anchor.md 首行（任务锚由 agent 每任务一写非每回合一写）

## 读数

五行即任务锚、在飞面、泊界面、链面、纪律令。退出码恒零永不拦会话；缺任务锚即告警行、账本或路由或链不可用即对应行降级标注，逐项如实转述不静默；锚行是回算事实投影不是建议，越界读须一句申报。权威即 sih-tools/attnanchor/CONTRACT.md。
