---
name: "sihankor-redteam"
description: "RETIRED 已退役：redteam 工具代码已于 2026-08-30 前移除，本壳仅存血统档禁止调用。历史：通用 LLM 红蓝紫对抗审查工具的调用入口。"
---

# SiHankor Redteam Skill

> **已退役（2026-08-30 补登记）**：redteam 工具代码已全盘移除，本 skill 仅存血统档，禁止调用。死因与翻案条件见根 AGENTS.md redteam 章。以下原文保留不抹。

通用 LLM 红蓝紫对抗审查工具的调用入口。工具独立于司衡哲学体系，承接 PRO-07 鉴层打破自证循环的精神，但不显式引用哲学命题。

## 概览 {#overview}

LLM 红蓝紫对抗工具的 Agent 调用入口。工具独立于司衡哲学体系，承接 PRO-07 鉴层打破自证循环的精神但不显式引用。

用户在表达"跑一次对抗"、"对某命题做审查"、"redteam 一下"等意图时触发。Skill 引导 agent 准备 needs 包、跑命令、解析 JSON 报告、给用户分级总结，不替用户做最终裁决。

工具的 7 个 commit 已在 `redteam/`,报告按 needs 文件名组织子目录,回归测试在 `redteam/tests/dryrun.py`。

## 触发时机 {#trigger}

用户表达以下意图时触发

- 直接说"跑一次对抗"、"做对抗审查"、"redteam 一下"
- 询问某命题是否成立、某设计是否合理、某提案是否站得住
- 要求对设计决策、命名归类、判据设定做压力测试
- 提到 redteam 工具的入口、报告、need 包

不触发

- 单纯事实查询
- 不涉及证据/锚定的纯讨论
- 仅需改文档/代码而无审查需求

## 前置检查 {#precheck}

执行前先确认

1. `redteam/redteam.py` 与 `redteam/src/` 在项目根存在。缺失则报错给用户，不要替用户实现。
2. `redteam/.env` 存在且含三厂 API key。缺失则提示用户配置。
3. 用户已指明 needs 包路径，或已指明命题内容(无现成 needs 时进入起草流程)。

## 执行流程 {#flow}

按以下顺序执行，不要并行。

### 1. 准备 needs 包 {#step-need}

检查 `redteam/needs/` 目录。

如果用户已指明 needs 文件，直接用该文件。

如果用户给出命题但无现成 needs，按以下规则起草

- 命题写 `proposition` 字段，一句话精确陈述
- 锚定 2 到 5 条，每条含 `path` 相对项目根、`range` 起始行或区间、`note` 选这段的原因
- `context` 命题背景 2 到 5 行
- `rounds` 设为 2

起草后让用户确认锚定是否合理，再进入下一步。

### 2. 跑对抗 {#step-run}

从 SiHankor 项目根目录跑命令。命令形如

`uv run --project redteam python redteam/redteam.py --file redteam/needs/<需求包名>.md`

`uv` 已装在用户机器。如果工具不在项目根，使用 `--project-root` 覆盖锚定解析基准。

### 3. 解析报告 {#step-parse}

报告路径形如 `redteam/reports/<需求包 stem>/<时间戳>.json`。读取该 JSON。

关键字段

- `convergence` 正常完成 / 紫方 token 上限 / 模型不可用 / JSON 解析失败重试后
- `games` 双局数组，每局 2 轮
- `unresolved_attacks.high_confidence` 高可信未解决攻击
- `unresolved_attacks.medium_confidence.game1_only` 局 1 单方命中未防御
- `unresolved_attacks.medium_confidence.game2_only` 局 2 单方命中未防御
- `anomaly.intersection_empty` 双局 anchor_ref 交集空报警
- `dropped_anchors` 锚定剔除记录
- `errors` API 异常详情

### 4. 总结给用户 {#step-report}

按以下结构输出

- 命题与 needs 来源
- 双局完成情况与收敛状态
- 未解决攻击分级，列出锚定行号与一句话理由
- 异常与 API 错误
- Token 消耗
- 报告 JSON 路径

裁决不做。报告里的 unresolved 是"未被任一局防御的锚定"，最终判断由用户在自己上下文里做。

## 工具关键设计 {#design}

锚定白名单。LLM 输出的 anchor_ref 必须落在需求包声明的区间内，否则被 Python 程序机械剔除进 dropped_anchors。攻击必须有证据，无锚不算攻击。

双局两轮。局 1 GLM 红 vs MiniMax 蓝，局 2 角色互换。局间串行避免 GLM semaphore 2 同时被两局占满。

紫方增援。第二轮介入。紫方 schema 严格 extra=forbid，从数据层强制不给完整攻击点，只给方向提示，红方基于提示自行在白名单内找锚定。

裁决分离。工具不裁决只产报告，调用方 agent 读 anchor_ref 真实核验后做判断。

## 常见错误处理 {#errors}

工具未安装。提示用户 `cd redteam && uv sync` 后重试。

.env 缺失或 API key 未配。提示用户在 redteam/.env 写入三厂 API key。

GLM 报 429 或 1113。智谱 GLM-4.7-Flash 可能限流或余额不足。提示用户在 .env 加 `GLM_API_MODEL=GLM-4-Flash`，智谱免费模型。

MiniMax 报 402。MiniMax 账户余额不足，提示用户充值。

锚定被剔除。检查 LLM 是否拼错路径/行号，被剔除的锚定在 dropped_anchors 字段。

anomaly 报警。双局 anchor_ref 集合交集为空。提示用户检查命题是否过于局部、是否需要更宽的锚定范围。

## 当前状态 {#status}

7 个 commit 落地，工具完整可用。报告按 needs 文件名组织子目录，多次跑同 needs 共享子目录。回归测试在 `redteam/tests/dryrun.py`，3 秒跑完无 API key 也能验全链路。
