# 0.9.0 发布清单

本文承载司衡引擎 0.9.0 首发布的发布内容面与执行步骤与已知边界与回滚法，令源即 release09-solo 批任务包件二。版本语义与 1.0.0 晋升判据以 DEC-022 为正典，见 `sih-engine/doc/decision/022-semver-release-v1.md`。本清单立清单不执行：打 tag 与推送与发布动作归主窗终验后，本批零执行。相名转写声明：本文承域字符集闸一律拉丁转写相名，alpha 相与 beta 相指称希腊字母原名的同一相，承 SPEC-023 转写先例。

## 概览 {#overview}

- 发布内容面五项即引擎五件套、sih-tools 工具带、mcpline 服务器、SPEC-023 契约、视图仓指针::[发布内容面](#content)
- 执行步骤九步序固定每步退出码零方进下一步，打 tag 与推送与发布宣告三步归主窗终验后执行::[执行步骤](#steps)
- 已知边界五项即 beta 写面未开、GOV-002 v3 候裁、出参超集申明、帮助面漂移、f2 引用抽取存量红::[已知边界](#boundaries)
- 回滚法四腿即 tag 腿、版本位腿、MCP 注册腿、零数据迁移申明::[回滚法](#rollback)

## 发布内容面 {#content}

引擎五件套
: 治理五二进制即书简 `scribe` 与核阅 `scrutinator` 与执契路择 `attractor` 与三问 `ask3repeater` 与温故 `retriever`，源位 `sih-engine/src/bin/`，构建位 `sih-engine/target/debug/`。视图件 `viewer` 与 `snapline` 归视图组件线随构建产出不在五件套枚举内。

sih-tools 工具带
: 工具线全带随 0.9.0 对外：租约 lease、化格 formatter、检词 nomenclator、计数 meter、叩问 elicit、正身 identity、交叉审阅 facet、路择 selector、级联 cascade、句读 parser、寻址 locator、执契 tally、秤星 gauge、判据扫 critsweep、稽 watchcheck、回锚 attnanchor、书单召回 wikirecall 等，总编排见 `sih-tools/COURSE-v2.md`，各工具版本面自带不入 DEC-022 定约辖域。

mcpline 服务器
: `sih-tools/mcpline/` alpha 相只读面 MCP 服务器：五读数工具 chain_query 与 chain_verify 与 critsweep 与 heartbeat 与 locks_read，stdio 传输，入口 `python -m mcpline`，零写入红线承 SPEC-023。

SPEC-023 契约
: `sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md` 随发布对外，五工具名称与入参与出参与错误语义与冷 agent 验收程序在档为 MCP 面对表正典。

视图仓指针
: 视觉身份探索仓 `sih-visual/` 以指针随发布申明：属视觉身份治理探索面，不受 sih-engine 治理约束，不入版本定约辖域。

## 执行步骤 {#steps}

步骤序固定，每步退出码零方进下一步；第七至第九步归主窗终验后执行：

1. 版本位对表：`sih-engine/Cargo.toml` version 位改至 `0.9.0`，与 DEC-022 版本载体条款对表，提交入主线
2. 构建验证：`cd sih-engine && cargo build` 退出码零，五件套与视图件二进制在 `target/debug/` 在位
3. 测试验证：`cd sih-engine && cargo test` 退出码零
4. 工具线冒烟：`cd sih-tools/lease && uv run --project . lease reconcile --repo sih-tools --root <工作区根>` 与 engine 仓同形各一跑，unrouted 与 cert_missing 零新增
5. 链验证：`sih-engine/target/debug/scribe verify --trail <当日链>` status valid
6. MCP 面冒烟：stdio 面 tools/list 与五工具逐工具探针各一笔，形照 `sih-engine/sih/event/plan/mcpline-alpha-settlement-v1.md` 第三节读数，退出码零
7. 打 tag：`git -C sih-engine tag v0.9.0 <提交号>`，归主窗终验后执行
8. 推送：主线与 tag 推送远程，归主窗终验后执行
9. 发布宣告：结算件落 `sih-engine/sih/event/plan/` 并链上认证，归主窗终验后执行

## 已知边界 {#boundaries}

beta 写面未开
: MCP 写操作候会话与租约映射安全模型设计批过闸，未过不开工，承 SPEC-023 红线；0.9.0 对外面只读，写承诺零。

GOV-002 v3 候裁
: 主线锁判据 v2 系现行正典，v3 换版候人节点裁决，0.9.0 不预载 v3 语义。

出参超集申明
: chain_query 出参信封字段系契约核心三字段外补充，承 mcpline alpha 相结算件第四节，随发布如实申明。

帮助面漂移
: 冷 agent 起跑 CLI 帮助面三旗标解析器拒识漂移在档候工具线处置，承 mcpcold 结算件，随发布如实申明。

f2 引用抽取存量红
: mem_recall_f_suite f2_refs_machine_verifiable 自 2026-09-04 即红且 basefix-solo 结果档存量红申报在档，行漂移根因即 pk050sw 结果档收口回填致行移，非本发布内容回归；根因为检索器引用抽取把红申报行内引用形字串录为活引用，修复归 recallfix-solo 后继批；主窗裁定按测试味首发布定位带此已知边界放行，库测 191 绿与金向量重冻复证随发布在案。

## 回滚法 {#rollback}

tag 腿
: `git -C sih-engine tag -d v0.9.0` 删本地 tag；已推远端则 `git push origin :refs/tags/v0.9.0` 删远端 tag。

版本位腿
: Cargo version 位以 revert 或还原提交回退，禁止改写既有提交历史，承 GOV-004 治理域历史不可改。

MCP 注册腿
: mcpline 注册回退照 `sih-engine/sih/event/plan/mcpcold-solo-materials/registration-and-rollback.md` 在档法执行。

零数据迁移申明
: 0.9.0 零数据迁移面，回滚即版本指针回退，链与台账零动。
