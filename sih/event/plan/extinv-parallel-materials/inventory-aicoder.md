# AiCoder（Spec-Flow）工程机制清单

盘点批：extinv-parallel，日期：2026-09-02，盘点方式：子代理只读通读。

| # | 机制名 | 出处 | 类型标签 | 一句话内核 | 重复度初判 |
|---|--------|------|----------|------------|------------|
| 1 | 审查修复闭环 | agents/pm_agent.py:308 | 编排先例 | 审查修复定轮上限，超限出人工介入报告并停机 | 同构：治理状态机（有限重试转人工升级分支） |
| 2 | 三级约束体系 | spec-flow-docs/architecture/spec-flow-04-rules.md:5 | 范式实践 | L1 铁律 L2 角色 L3 片段逐级收窄，上级覆盖下级 | 同构：F/G/J 规则引擎（静态 markdown 版） |
| 3 | 规则编译注入 | spec-flow-docs/architecture/spec-flow-04-rules.md:453 | 降格先行件 | 调度前展开引用合并规则，经临时文件传子代理 | 同构：约束注入（预先展开，无动态生效校验） |
| 4 | 人工卡点 | spec-flow-docs/architecture/spec-flow-05-governance.md:31 | 范式实践 | stdout 出 needs_approval 即挂起，超时视为拒绝 | 新机制（人工闸门节点，可补司衡缺项） |
| 5 | 产物防覆盖 | rules/constraints/artifact-persistence.md:14 | 范式实践 | 产物头 human_modified 标记人工改动，为真则不覆盖 | 新机制（人机产物污染区分） |
| 6 | 知识缺口标记 | spec-flow-docs/architecture/spec-flow-07-missing-knowledge.md:75 | 降格先行件 | 本地查不到不猜：建缺口标记续跑 | 同构：追问引擎（降格为标记缺口不追问人） |
| 7 | 降级矩阵 | spec-flow-docs/architecture/spec-flow-05-governance.md:166 | 降格先行件 | 逐失败场景预设策略、是否阻塞与恢复方式 | 同构：治理状态机（异常转移表文档化雏形） |
| 8 | 断点续跑 | spec-flow-docs/architecture/spec-flow-05-governance.md:135 | 编排先例 | 按阶段恢复，先查产物是否被人工改 | 同构：治理状态机（检查点恢复路径） |
| 9 | 知识权威梯 | spec-flow-docs/architecture/spec-flow-07-missing-knowledge.md:35 | 范式实践 | 唯一可信源，五级知识查找冲突取高 | 新机制（知识来源优先级） |
| 10 | 断网铁律 | rules/constraints/no-network.md:3 | 范式实践 | 禁一切联网，知识只取本地件与 MCP 与库 | 新机制（离线知识封闭性原则） |
| 11 | 产物溯源留存 | mcp_server/git_artifact_server.py:43 | 范式实践 | 产物分型落盘，头记溯源元数据 | 同构：审计留痕（产物面留痕，与日志面互补） |
| 12 | 审计步日志 | mcp_server/pipeline_server.py:28 | 降格先行件 | 记 agent 与 action 与 result，双写日志与库 | 重复：审计留痕（字段更简，无因果链） |
| 13 | 阶段串行调度 | rules/pm-agent.md:16 | 编排先例 | 按配置拉起，同步串行四阶段 | 同构：编排调度与 DAG 队形（线性退化形） |
| 14 | 退出码协议 | agents/pm_agent.py:234 | 范式实践 | 0 过 1 可重试 2 不可修，调度按码分叉 | 同构：治理状态机（错误分类驱动转移） |
| 15 | 诊疗分离 | rules/reviewer-agent.md:11 | 范式实践 | 审查者只诊断不治疗，绝不直接改代码 | 新机制（角色职责隔离） |
| 16 | 安全模式扫描 | mcp_server/git_artifact_server.py:213 | 降格先行件 | 九条正则扫高危，按严重度排序出报告 | 同构：验证管道（规则匹配级） |
| 17 | 迁移审查闸门 | mcp_server/database_server.py:80 | 降格先行件 | 静态审高危词，执行强制人工确认不落地 | 同构：验证管道（高危前置审查加卡点复合） |
| 18 | 多档降配 | .spec-project.json:27 | 范式实践 | 双 Profile 裁剪 Server 与 Agent | 新机制（资源档位降级） |
