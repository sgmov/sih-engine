# Changelog

体例循 Keep a Changelog；版本语义以 DEC-022（sih-engine/doc/decision/022-semver-release-v1.md）为正典。

## 未发布

### 变更

- 审计勘误与双报告格式归一：2026-09-22 auditfix 与 rulenorm 两批。
- 三份决策文档落据：DEC-027、DEC-035、DEC-036，并归桶迁移。
- 本批（libwave）：哈希归并、工具索引与标准件。

## [0.9.0]（候发布）

### 新增

- 引擎五件套：书简 scribe、核阅 scrutinator、执契路择 attractor、三问 ask3repeater、温故 retriever，源位 `sih-engine/src/bin/`，构建位 `sih-engine/target/debug/`；视图件 viewer 与 snapline 归视图组件线随构建产出，不在五件套枚举内。
- sih-tools 工具带：租约 lease、化格 formatter、检词 nomenclator、计数 meter、叩问 elicit、正身 identity、交叉审阅 facet、路择 selector、级联 cascade、句读 parser、寻址 locator、执契 tally、秤星 gauge、判据扫 critsweep、稽 watchcheck、回锚 attnanchor、书单召回 wikirecall 等。
- mcpline 服务器：`sih-tools/mcpline/` alpha 相只读面，五读数工具 chain_query、chain_verify、critsweep、heartbeat、locks_read，stdio 传输，入口 `python -m mcpline`，零写入红线。
- SPEC-023 契约：`sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md` 随发布对外，五工具名称与入参与出参与错误语义在档为 MCP 面对表正典。
- 视图仓指针：`sih-visual/` 以指针随发布申明，属视觉身份治理探索面，不受 sih-engine 治理约束，不入版本定约辖域。
