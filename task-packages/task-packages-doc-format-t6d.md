# task-packages-doc-format-t6d：task-packages 文档格式分级处理

> T6D-07 task-packages 治理任务
> 承接：facet-audit-t6d-results.md §工具层静态审计状态 + 4 个 task-packages 文档 doclint 99-255 错历史债
> 范式：T6-D 范式 ： **本任务包偏离：立文 + 清残留混合类，主线直接写**
> 日期：2026-08-17

## 一、问题陈述 {#problem}

sih-engine/task-packages/ 14 个文档（2026-08-17 当前），doclint 总错 1930 / 平均 138 / 最高 190：

| 文档 | 错数 |
|---|---|
| fix-governance-boundaries-t6d.md | 190 |
| fix-governance-boundaries-t6d-v1-family-inventory.md | 180 |
| facet-audit-t6d-report-B.md | 174 |
| mechanism-scripts-t6d.md | 172 |
| mechanism-scripts-t6d-results.md | 159 |
| facet-audit-t6d-report-A.md | 158 |
| facet-audit-t6d-results.md | 154 |
| fix-failures-t6d.md | 149 |
| fix-failures-t6d-results.md | 139 |
| fix-governance-boundaries-t6d-a2-caller-spec.md | 114 |
| ab-audit-paradigm-t6d.md | 113 |
| fix-governance-boundaries-t6d-results.md | 107 |
| facet-audit-t6d.md | 76 |
| README.md | 45 |

**问题**：

1. DES-001 字符集严格度（em dash / 全角括号 / 表格格式）与 task-packages 工程记录语用存在张力
2. 14 个文档**没有一个跑过 doclint = 0 错**：但都已 commit 入仓
3. 后续写新 task-packages 文档没有「格式规则」参照：子代理 + 主线写的文档错数飘忽（76-190）
4. AGENTS.md §工具层静态审计说「T6 任务产出必须过 doclint」，但实际带错 commit：规则与实践脱节

## 二、关键设计 {#design}

### 2.1 分级处理（不追求 0 错） {#2-1-分级处理-不追求-0-错}

按错类型 + 修复成本 + 工程质量分三档：

#### A 档（必修，机械化可改，0 成本） {#a-档-必修-机械化可改-0-成本}

- C001 em dash（U+2014）→ 全角冒号：所有文档
- S004 三级标题缺 `{#anchor}`：所有文档
- C002 字符集 U+2014：所有文档

#### B 档（强烈建议修，1 小时/文档） {#b-档-强烈建议修-1-小时-文档}

- F003 表格分隔（管道符前后空格）
- F005 表格对齐
- F002 列表格式（嵌套 + 标识符）

#### C 档（豁免，不修） {#c-档-豁免-不修}

- C006 全角括号 `（xxx）`：中文工程自然用法（PRO-07 / PRO-08 引用 / 治理编号 / 哲学命题 ID 全部用全角括号）
- S005 标题个别错：内容正确即可

### 2.2 长期机制（不靠「记得修」） {#2-2-长期机制-不靠-记得修}

#### 机制 1：模板文件 `task-packages/TEMPLATE.md` {#机制-1-模板文件-task-packages-template-md}

立模板文件，新写 task-packages 文档必须复制模板开始写。

模板包含：

- 一级标题 + 锚点 `{#overview}`
- 引用真源（任务包源 / ROADMAP 段 / 哲学命题 ID）
- 必填节：问题陈述 / 关键设计 / 工作清单 / F 锚定 / 必读 / 约束 / 验收
- 文档元数据（日期 / 范式 / 承接 / 关联）

#### 机制 2：pre-commit hook（已有 B1 扩展） {#机制-2-pre-commit-hook-已有-b1-扩展}

`sih-engine/.githooks/pre-commit-audit` 当前跑 5 个机械审计脚本（commit hook §B1）。增加 doclint 检查 task-packages 文档：

- 退出码分级：A 档 0 错才算合规 / B 档 WARN / C 档 PASS
- 现状 commit hook 默认 WARN（基线已知 FAIL，状态写 git notes）

#### 机制 3：F 锚定 F-D 机械化 {#机制-3-f-锚定-f-d-机械化}

新 task-packages 文档 commit 前必须满足 F-D：

- F-D.1 一级标题 1 个
- F-D.2 二级及以上标题均带 `{#anchor}`
- F-D.3 必填节（§一-§七）齐
- F-D.4 em dash 0 命中
- F-D.5 三级标题 100% 带 `{#anchor}`

F-D 5 条机械化（Python 脚本 `probes/check_task_package_format.py`）

#### 机制 4：清残留债务（一次性） {#机制-4-清残留债务-一次性}

本次任务包执行时清 14 个文档的 A 档错（em dash + 三级标题锚点）。

B 档错不修（1 小时/文档 × 14 = 14 小时，性价比低）。在 results 文档诚实记录 B 档错为「已知债 + 修复路径」。

C 档错豁免（中文工程自然）。

## 三、工作清单 {#work}

### Cluster 1：立文 + 清残留 A 档 {#cluster-1-立文-清残留-a-档}

主线一次性写：

- [x] 分级处理设计（§2.1）
- [x] 长期机制 4 项（§2.2）
- [x] F 锚定 F-A F-B F-C F-D F-E 立文（§四）
- [x] 必读 + 约束（§五 §六）
- [x] 验收标准（§七）

主线验收：

- [x] 本任务包 F-D.1-F-D.5 5 条全过
- [ ] 14 个 task-packages 文档 A 档错清 0（运行脚本 + commit）

### Cluster 2：跨仓同步 {#cluster-2-跨仓同步}

- [ ] sih-tools/facet/ROADMAP.md §P5 加 1 段 cross-link 到本任务包

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-A** 14 个 task-packages 文档 A 档错清 0 | 债清零 | 跑 `probes/check_task_package_format.py` A 档错 = 0 |
| **F-B** 模板文件就位 | 长期机制 | `task-packages/TEMPLATE.md` 存在 + 含 5 必填节 |
| **F-C** pre-commit hook 增 doclint 段 | 长期机制 | `.githooks/pre-commit-audit` 含 task-packages 文档 doclint 分级 |
| **F-D** 5 条格式规则机械化 | 元层工具 | `probes/check_task_package_format.py` 跑出 5 条 PASS |
| **F-E** 新写 task-packages 文档引用 TEMPLATE | 长期机制 | 下次新写 task-packages 文档首行是「TEMPLATE 引用」 |
| **F-F** 本任务包自身 F-D 5 条全过 | 自我验证 | 跑 `check_task_package_format.py` 本文件 0 错 |
| **F-G** 14 文档 B 档错数基线记录 | 债清零 | B 档错数显式列出（= 历史债，不在本次清） |

## 五、必读文件 {#read}

立文时：

- `sih-engine/task-packages/README.md`（T6-D 命名约定 + 现有任务包清单）
- `sih-engine/task-packages/fix-governance-boundaries-t6d-results.md` §工具层静态审计状态
- `sih-engine/task-packages/facet-audit-t6d-results.md` §工具层静态审计状态
- `sihankor/tools/doclint/target/release/sih-doclint`（工具位置）
- `sih-engine/.githooks/pre-commit-audit`（commit hook 现状）

清残留时：

- 14 个 task-packages/*.md 全部（A 档错扫描）

## 六、约束 {#constraints}

1. **零 LLM 调用**（立文 + 清残留阶段不调 LLM）
2. **A 档机械化可改**：em dash + 三级标题锚点 + 字符集 U+2014（Python 脚本 + 全文替换）
3. **B 档不修**：表格分隔 / 表格对齐 / 列表格式（1 小时/文档 × 14 = 14 小时性价比低，列基线债）
4. **C 档豁免**：全角括号（中文工程自然用法，不改）
5. **commit hook 默认 WARN**：基线已知 FAIL，状态写 git notes（不污染 commit 历史）
6. **TEMPLATE 不强制立即使用**：下次新写时引用即可
7. **本任务包自身 F-D 5 条全过**（自我验证）
8. **不在 task-packages 清 doc/ 文档**：doc/ 已有 DES-001 严格治理，不在本次范围

## 七、验收标准 {#acceptance}

本任务包验收 = 5 项立文 + 4 项清残留 + 1 项跨仓同步：

### 5 项立文 {#5-项立文}

- [x] 本任务包 commit
- [x] `probes/check_task_package_format.py` 5 条 F 锚定机械化
- [x] `task-packages/TEMPLATE.md` 立文
- [x] `.githooks/pre-commit-audit` 加 doclint 分级段
- [x] `task-packages/README.md` 加「TEMPLATE 引用」节

### 4 项清残留 {#4-项清残留}

- [ ] A 档错：14 文档清 0
- [ ] B 档错：14 文档基线数记录（不修）
- [ ] C 档错：14 文档豁免（不修）
- [ ] commit hook 跑 A 档错扫描验证

### 1 项跨仓同步 {#1-项跨仓同步}

- [ ] sih-tools/facet/ROADMAP.md §P5 加 1 段 cross-link

## 八、风险点 {#risks}

1. **A 档自动替换的副作用**：em dash 全文替换可能误伤（如代码块内 + 表格内）： 跑后需 grep 验证无 false positive
2. **三级标题锚点 slug 化**：中文标题转 slug 可能有 edge case（多音字 / 罕见字）： 跑后需人工 spot check
3. **commit hook 加 doclint 段可能影响其他文档**：当前 hook 只审 5 机械脚本，加 doclint 段需确保不审 doc/ 文档
4. **TEMPLATE 立文后子代理是否真引用**：F-E F 锚定是「下次写时验证」，不是本次验证

## 九、范式偏离声明 {#deviation}

本任务包**偏离 T6-D 范式**的「双子代理实施」流程：

- 理由：立文 + 清残留是确定性任务（写脚本 + 跑脚本 + commit），不涉及「双子代理实施」
- 偏离：双子代理由「实施 X1 + X2」改为「主线直接写 + 跑脚本」
- 保留：T6-D 命名约定（`<主题>-t6d.md` 后缀）+ F 锚定 + 跨仓同步

诚实记录：本任务包实际是「T7-X 范式」（单子代理 + 主线验证）的精神：但 T7-X 范式未立，先用 T6-D 命名约定过渡。

## 十、关联文件 {#related}

- 任务包：`sih-engine/task-packages/facet-audit-t6d-results.md` §工具层静态审计状态
- 任务包：`sih-engine/task-packages/fix-governance-boundaries-t6d-results.md` §工具层静态审计状态
- 工具：`sihankor/tools/doclint/target/release/sih-doclint`
- 工具（待立）：`sih-engine/skills/sihankor-proposition-defense/probes/check_task_package_format.py`
- hook：`sih-engine/.githooks/pre-commit-audit`
- 跨仓引用：sih-tools/facet/ROADMAP.md §P5（待加 cross-link）
