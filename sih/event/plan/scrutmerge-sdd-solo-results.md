# scrutmerge-sdd-solo 结果档（核阅融回规格先行批）

> 批名：scrutmerge-sdd-solo。日期 2026-08-31。
> 意图事件哈希 64b2d87d...（hash 头 64b2d87d，sess-zcode-260831-scrutmerge-sdd）。
> 队形：单线 solo — 主线亲写零子代理。

## F 锚定验收

| F | 判据 | 判定 | 证据 |
|---|---|---|---|
| **F-1** 家位与模块形 | 规格钉死引擎侧 src/scrutinator/ 库模块加 src/bin/scrutinator.rs 命令行面，承 DEC-007 组件层归属 | 过 | SPEC-013 § 家位与模块形。库路径与二进制同位精简避免检词死档 "Scrutiny" 整词匹配，承检词死档 discipline |
| **F-2** 接口契约对表 | 工具 CONTRACT.md 六件机器形态加五判据加融回门三查逐条对表，零语义漂移 | 过 | SPEC-013 § 接口对表。第一件空腹/材料轴/词表/规则即配置/多规则包/json 报告/三值退出码/双版本戳与治理域五判据融回门三查共六件机器形态逐条对表 |
| **F-3** 规则包家位 | manifest.toml 加 rules.toml 纯数据随件迁引擎侧，三包全量迁，工具侧保留不删 | 过 | SPEC-013 § 规则包迁移。des-001 0.1.0（十二件）+ des-001-mathe 0.3.0（十四件）+ ask3 0.1.0（二十三件）全量迁引擎侧家位 |
| **F-4** 验收判据 | 同包同目标下引擎件与工具件输出 JSON 报告逐字节一致 + 退出码三值 + content_hashes 与 scribe append 认证位兼容 | 过 | SPEC-013 § 验收判据。A1 同包同目标逐字节一致（A1 注明 engine.version 不要求逐字节一致）；A2 退出码三值；A3 报告 content_hashes 与 scribe append 认证位兼容；A4 多包加载与归因；A5 不变量 |
| **F-5** 回迁债 | TOML 解析依赖加 Cargo.toml 必加 toml 依赖 + 双跑对表基线 + DEC-001 围堰归位映射核阅行 | 过 | SPEC-013 § 回迁债。TOML 解析必加 toml = "0.8"；金向量基线家位 src/scrutinator/fixtures/golden/；围堰归位映射核阅行承 DEC-001 第 118 行归位至 src/scrutinator/ 与 src/bin/scrutinator.rs |

## 链事件号清单

| 序 | 事件类型 | 事件哈希 | doc_id | 备注 |
|---|---|---|---|---|
| 1 | intent_refined | 64b2d87d... | sess-zcode-260831-scrutmerge-sdd | 意图入链（事件哈希前 8 位） |

末笔哈希待认证 append 后追记。

## 管线报告

- 化格 SPEC-013：0 changes
- 核阅 des-001 SPEC-013：0 findings, 0 mismatches（exit 0）
- 检词 SPEC-013：0 findings（exit 0）
- 化格 任务包：0 changes
- 核阅 des-001 任务包：0 findings, 1 mismatch（exit 2 任务包在 sih/state/plan/ 域外非违规）
- 检词 任务包：0 findings（exit 0）

## 修规记录

初版 SPEC-013 三处违规已修
- C001 charset_forbid 触发：规则消息本身含 "——" 全角破折号，改用汉字"半角破折号" 描述
- C002 charset_allow 触发：同上破折号字符
- dead_ban "Scrutiny" 触发：路径 src/scrutiny/ 整词匹配 PRO-007 死档，改用 src/scrutinator/（与二进制同名），家位令源段补叙理由
- C006 forbid_pattern 触发：家位令源段含全角括号，改用直陈

## 偏离与期票

| 类别 | 项 | 详情 | 期票 |
|---|---|---|---|
| 家位 | src/scrutiny/ → src/scrutinator/ | 检词死档 discipline 强制路径改名 | 已发原因为令源落 SPEC-013 |
| 域外 | 任务包核阅 1 mismatch | 任务包在 sih/state/plan/ 不在 des-001 域 | 已知非违规如实记 |
| 范围 | TDD 批未跑 | 属批二待放行执行 | TDD 批按 SPEC-013 逐判据先红后绿 |
| 范围 | 切换批未跑 | 属批三待放行执行 | 工具件转兼容只读与 T6 管线核阅腿认证位改指引擎件 |

## 后续

- 批二 TDD：写 src/scrutinator/ 库与 src/bin/scrutinator.rs 二元，按 SPEC-013 五判据先红后绿
- 批三 Switch：等用户放行令后切换 T6 核阅腿认证位与工具件转兼容只读
- GOV-003 编排节结算追加：v1.4 → v1.5 由批三 Switch 完成后追记

## 链与收口

- 意图事件哈希 64b2d87d259bd4f8081be82cd4c785ba3ecd1cca539118e3cc44f0b14ebfbdf3
- 双仓 worktree: msh/scrutmerge-sdd-solo 分支
- session_id fb0e23407bf759f2
- 收约后验链 valid
