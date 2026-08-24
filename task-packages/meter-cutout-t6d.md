# meter 甲路线实施：外挂包裹式工具调用计数器

> T6D-XX task-packages 治理任务
> 承接：sih-tools/proposition/DES/m-meter-variant/disposition.md 裁示 + 待办第一项
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— 本任务包无偏离，标准实施类
> 日期：2026-08-25

## 一、问题陈述 {#problem}

- 裁定已立：甲路线经 facet 测量七发六合一违与确定性核对成立，三件核验收带出即只读契约、零依赖独立、漏计交叉核对
- 待办第一项：meter 计数器变体甲即 meter run 包住工具调用计数不动工具本体，另 DES-013 度量节补机械源现状与三落点
- 边界：meter 计确定性外壳的工具调用，非 LLM 调用，LLM 调用计数待引擎侧；meter 为工作名，正式名待立名程序

## 二、关键设计 {#design}

### 2.1 meter 形态

三子命令。run 即包裹执行子命令、记录即时间戳命令工具归因退出码耗时、退出码透传子命令；count 即计数汇总可按工具按日过滤；crosscheck 即漏计交叉核对。计数载体为 counts 目录按日 ndjson，每行一记录，人类可读不依赖专有工具。

### 2.2 漏计交叉核对语义

书简 trail 的认证事件与意图事件蕴含工具调用即认证事件蕴含书简自身调用加报告路径可归因的工具、意图事件蕴含书简加核阅。事件时间戳晚于 meter 首记录即入检域，早于即计为 meter 落地前事件不旗。逐蕴含工具在 meter counts 找事件日或前一日匹配记录，无匹配即漏计旗，退出码一，报告逐旗点名事件与工具。

### 2.3 接线

五工具调用壳加计量路由行即约定层；meter 调用壳新建落投影位；AGENTS.md 文件索引加 meter 行、技能入口计数随实态校准。

## 三、工作清单 {#work}

### Cluster 1：双子代理并行

- [ ] X1：DES-013 修订二草案落文件即度量节补机械源现状与三落点加修订记录行
- [ ] X2：五工具壳加计量路由行、meter 壳新建、AGENTS.md 两行校准

### Cluster 2：主线亲写最重件

- [ ] meter 工具六件即 pyproject、src 即 __init__ 与 cli、tests、CONTRACT、README、CALL-LOG

### Cluster 3：主线串行验证

- [ ] meter 测试全绿与三件核验收逐项
- [ ] X1 产出过文档链即化格核阅检词书简四件加 doclint
- [ ] X2 产出行级核验加投影实体核验
- [ ] 结果文件落盘加双仓 commit

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1 只读契约** | 跨族治理 | 批后五工具仓无变更即 scribe、scrutinator、formatter、nomenclator、selector 五目录 git diff 为零 |
| **F-2 零依赖独立** | 代码修复 | meter 的 pyproject 零第三方依赖，src 零兄弟工具导入，独立 uv 环境测试全绿 |
| **F-3 漏计交叉核对** | 元层工具 | 夹具即蕴含工具缺 meter 匹配记录时 crosscheck 退出码一且报告点名事件与工具；全匹配夹具退出码零零旗；早于首记录的事件计为落地前不旗 |
| **F-4 DES-013 文档链** | 数据治理 | 修订二过 doclint 退出码零，化格核阅检词书简四件全绿，检词零新增违例 |
| **F-5 三问上链** | 数据治理 | intent_refined 事件在链即事件 6a798ebe 已毕，链验通过 |

## 五、必读文件 {#read}

- 裁示：sih-tools/proposition/DES/m-meter-variant/disposition.md
- 决策：sih-engine/doc/decision/006-ask3-formalization.md 即 LLM 边界与调用计量节
- 设计：sih-engine/doc/design/DES-013-ask3-component-design.md 即度量节
- 载体：sih-engine/skills/sihankor-intent-refine/SKILL.md 第 90 行计量现状
- 形制参照：sih-tools/selector/CONTRACT.md 与 sih-tools/selector/pyproject.toml

## 六、约束 {#constraints}

1. **零 LLM 治理写入**：计数、验货、上链全部确定性程序执行
2. **判据机械化**：F-1 至 F-5 全部可机械复核，不含感觉判据
3. **不扩大 scope**：不动五工具本体与契约，不实现 LLM 调用计数，不立 meter 正式名
4. **F 锚定状态入结果文件**：逐条过败如实登记

## 七、验收标准 {#acceptance}

本任务包验收 = 5 项：

- [ ] F-1 至 F-5 全过
- [ ] meter 真实包裹跑一次工具即计数文件落盘非空
- [ ] DES-013 修订二在链
- [ ] 双仓 commit 即 sih-tools 与 sih-engine
- [ ] 结果文件 meter-cutout-t6d-results.md 落盘

## 八、风险点 {#risks}

- crosscheck 归因即报告路径扫工具名，非六工具名事件仅蕴含书简自身不误伤
- 跨日窗口即事件日前一日也认，报告产物跨日时不误旗
- meter 落地同日的未包裹调用会被如实检为漏计，这是机制非缺陷

## 九、范式偏离声明 {#deviation}

无偏离。双子代理 X1 与 X2 后台并行，主线亲写工具本体即最重件，主线串行验收，结果文件收口。

## 十、关联文件 {#related}

- 任务包源：sih-tools/proposition/topics/2026-08-25-meter-variant.md
- 意图记录：/tmp/meter-batch/record.json 即 intent_refined 事件 6a798ebe
- 工具：sih-tools/meter/
- 跨仓引用：sih-engine/doc/design/DES-013-ask3-component-design.md
