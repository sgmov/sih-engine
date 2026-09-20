# schema-robust-serial 任务包

- 批名：schema-robust-serial
- 形：串形（单子代理施工，主线验收，闸门后推 PR）
- 分支：community/fail-loud（栈于 e3d0166 即 PR #52，施工基线 HEAD 6a3cc6c）
- 仓：/Users/moc/workspaces/SiHankor/MiniMax-Code-Plugins
- 插件：plugins/hetaoBackend/mcode-dynamic-workflows
- 日期：2026-09-20

## 背景

用户反馈（2026-09-20 截图）：工作流 synth 成功后 render 节点被判失败，render_error 为最终输出非纯 JSON（worker 在 JSON 后附加说明文字），run 终态 completed_with_gaps，而副作用产物（docx）已写好、render_result 为 null。插件侧同类脆性实锤于 src/structured-output.mjs：提取只有整段 JSON.parse 与首尾锚定围栏（`^```...```$`）两路，JSON 前后带任何散文即 invalid_json 判死。

既有拒绝面（占位符、散文提取、歧义块、JSON 修复）是反猜造哲学的刻意设计且被 checks/structured-output.check.mjs 钉死，本批不得推翻其精神，只翻转两个被现场证据证伪的形态。

引擎不向 worker 注入 JSON 输出指令（schema 只做事后校验，engine.mjs:302），输出纪律归脚本作者 prompt——故插件层的正确修法是无歧义提取，不是替模型修数据、不是自动重试。

## 不变量

1. 无歧义单候选才提取：散文中恰一完整围栏块（任意位置，围栏内容嵌围栏仍拒），或恰一顶层平衡 JSON 候选且闭括号后首个非空白字符非 JSON 结构符（`,` `]` `}` `:` 皆视为截断征兆拒绝）；多候选、相邻结构符、占位符、需修复文本照旧拒绝，错误码（OUTPUT_SCHEMA_INVALID）、reason（invalid_json／schema_mismatch）、suggestion 语义不变。
2. schema 仍是唯一数据闸：提取结果必须过 validate 才接受；不放松校验、不 coercion、不合成字段；文件头 "Compatibility is limited to transport formatting. Never synthesize missing data." 哲学原样保留。候选扫描须字符串感知（跳过 JSON 字符串字面量内部的花括号与转义）。
3. format 元数据透出提取路径：新增 'json_fence_embedded'（散文中恰一围栏）与 'json_embedded'（散文中恰一裸候选）；既有 native／json／json_fence 不变；step.rawOutput 继续保留原文。outputFormat 为只写元数据，无消费面穷举（已核）。
4. 既有拒绝面除两类翻转（单候选散文包裹、散文中恰一围栏）外逐条保持：双围栏、尾逗号（json+','）、未引号键、占位符文本、双编码字符串仍拒；检查件第二测的拒绝清单相应拆分，翻转两形移入接受面并注明事故出处（2026-09-20 用户反馈）。
5. 零新依赖；structuredOutput(raw,validate,stepId) 签名不变（engine.mjs:302 调用面零改动）；dist 同步提交（npm run build 后 git diff --exit-code -- dist 须空）。

## F-锚（验收线）

- G1 红转绿：事故形态（JSON+尾随散文 with schema）从 OUTPUT_SCHEMA_INVALID 翻为成功，output 与原对象 deep-equal，format='json_embedded'；前导散文形 format 亦然。
- G2 拒绝面守恒：双围栏、尾逗号、未引号键、占位符、双编码、无 JSON 纯文本逐一仍拒且错误码正确。
- G3 无回归：npm test 全绿（既有 123 含 fail-loud 5 用例 + 本批新增）、npm run test:package、npm run build、git diff --exit-code -- dist 空、git status 净。
- G4 检查件扩展：checks/structured-output.check.mjs 覆盖新接受面（前导散文、尾随散文、前后散文、散文中恰一围栏）与新拒绝面（双裸候选、闭括号后相邻逗号、围栏内嵌围栏）。
- G5 施工纪律：真实退出码验证（裸命令 + `echo exit=$?`，禁管道吃码）；commit 正文引事故出处与两形态翻转理由；不 push。

## 环境事实

- npm test = `node --test checks/*.check.mjs`；test:package = test/package.test.mjs。
- ajv 已在依赖内（8.20.0），检查件自建 Ajv 实例的既有风格沿用。
- 基线验证记录（主线 2026-09-20 亲跑）：npm test 123/123 exit=0；test:package exit=0；build exit=0；dist 重建幂等；树净。

## 检索申报

retriever recall（topic 结构化输出/schema 校验/提取，2026-09-20）3 命中均为旁系（MCP elicistration 文档、legacy 仓 DEC-026、legacy 仓 MCP API 合同），无直接治理命题，零约束加载。存档 /tmp/recall-schema.log。

## 偏离裁决位

主线。子代理遇不变量冲突即停手申报，不得自裁。
