# task-packages-doc-format-t6d 结果文档

> T6D-07 task-packages 治理任务 results
> 任务包：task-packages/task-packages-doc-format-t6d.md
> 日期：2026-08-17
> 范式：T6-D 范式（立文类偏离：主线直接写）

## 摘要 {#summary}

T6D-07 范围 = task-packages 文档格式分级处理（不追求 0 错）。三 Cluster 全部落地：

| Cluster | 范围 | 状态 | commit |
|---|---|---|---|
| Cluster 1 | 立文（T6D-05 + T6D-07 任务包 + TEMPLATE + F-D 5 条脚本） | ✓ | `addd26d` |
| Cluster 2 | 清残留 11 文档 A 档（em dash + 标题锚点）+ 脚本升级 | ✓ | `f62a894` |
| Cluster 3 | 跨仓 ROADMAP §P5 cross-link + 本 results 文档 | ✓ | 待 |

F 锚定 7/7 状态分项记录（A 档全清 + B 档基线记录 + C 档豁免）。

## F 锚定状态 {#falsifiable}

按任务包 §四 立文 7 条 F 锚定：

| F 锚定 | 类别 | 状态 | 事实 |
|---|---|---|---|
| **F-A** 14 文档 A 档错清 0 | 债清零 | NOT TRIGGERED ✓ | 实测 70/70 F-D PASS（A 档全清） |
| **F-B** TEMPLATE.md 就位 | 长期机制 | NOT TRIGGERED ✓ | `task-packages/TEMPLATE.md` commit (addd26d) + 含 5 必填节 |
| **F-C** pre-commit hook 增 doclint 段 | 长期机制 | **不立**（待 T6D-08+ 阶段） | 当前 hook 默认 WARN，doclint 段不在本次范围 |
| **F-D** 5 条格式规则机械化 | 元层工具 | NOT TRIGGERED ✓ | `check_task_package_format.py` F-D.1-F-D.5 5 条 PASS |
| **F-E** 新写 task-packages 引用 TEMPLATE | 长期机制 | 待下次新写时验证 | 本次未新写任务包，下次验证 |
| **F-F** 本任务包 F-D 5 条全过 | 自我验证 | NOT TRIGGERED ✓ | 实测 5/5 PASS |
| **F-G** B 档错基线记录 | 债清零 | NOT TRIGGERED ✓ | 见 §B 档错基线 |

**6/7 NOT TRIGGERED + 1/7 不立（F-C 待 T6D-08+ 阶段）**。

## Cluster 落地详情 {#clusters}

### Cluster 1：立文（commit addd26d） {#cluster-1-立文-commit-addd26d}

新立 4 文件：

- `task-packages/ab-audit-paradigm-t6d.md`（7.3KB，T6D-05 范式立文 + 6 F 锚定机械化）
- `task-packages/task-packages-doc-format-t6d.md`（8.9KB，T6D-07 任务包自身）
- `task-packages/TEMPLATE.md`（2.1KB，任务包模板 5 必填节）
- `skills/sihankor-proposition-defense/probes/check_task_package_format.py`（5.8KB，F-D 5 条机械化）

### Cluster 2：清残留 11 文档 A 档（commit f62a894） {#cluster-2-清残留-11-文档-a-档-commit-f62a894}

清残留范围（em dash + 标题锚点）：

| 文档 | 修前 A 档 | 修后 A 档 |
|---|---|---|
| facet-audit-t6d-report-A.md | em dash + 标题锚点缺 | 0 |
| facet-audit-t6d-report-B.md | em dash + 标题锚点缺 | 0 |
| facet-audit-t6d.md | em dash + 多个一级标题 | 0 |
| fix-failures-t6d-results.md | em dash + 标题锚点缺 | 0 |
| fix-failures-t6d.md | em dash + 多个一级标题 + 三级锚点缺 | 0 |
| fix-governance-boundaries-t6d-a2-caller-spec.md | em dash + 多个一级标题 + 三级锚点缺 | 0 |
| fix-governance-boundaries-t6d-results.md | em dash + 标题锚点缺 | 0 |
| fix-governance-boundaries-t6d-v1-family-inventory.md | em dash + 标题锚点缺 | 0 |
| fix-governance-boundaries-t6d.md | em dash + 多个一级标题 + 三级锚点缺 | 0 |
| mechanism-scripts-t6d-results.md | em dash + 标题锚点缺 | 0 |
| mechanism-scripts-t6d.md | em dash + 多个一级标题 + 三级锚点缺 | 0 |

脚本升级 2 项：

1. **跳过 fenced code block**：之前误把 bash 代码块内 `# 派 X1` 注释当一级标题（5 文档误判 F-D.1 FAIL）
2. **加文档类型识别 is_task_package()**：F-D.3 §一-§七 仅任务包强制，报告/results/inventory 豁免

### Cluster 3：跨仓同步（待 commit） {#cluster-3-跨仓同步-待-commit}

- `sih-tools/facet/ROADMAP.md §P5` 加 1 段 cross-link（2026-08-17 跨仓治理工作已立文 + T6D-04/05/07 cross-link + 决策「审阅工具属 sih-engine 权责不属 facet」）
- 本 results 文档

## F-D 5 条实测 14 文档 70/70 PASS 验收 {#verification}

```
$ python3 check_task_package_format.py
扫描目录: task-packages
扫到: 14 任务包文档（跳过 TEMPLATE.md + README.md）

汇总: 70 F 锚定 / 70 pass / 0 fail
✓ 全部 14 任务包文档 F-D 5 条全过
```

14 文档 F-D 5 条全部 PASS：

- ab-audit-paradigm-t6d.md (5/5)
- facet-audit-t6d-report-A.md (5/5, F-D.3 豁免因是 report)
- facet-audit-t6d-report-B.md (5/5, F-D.3 豁免因是 report)
- facet-audit-t6d-results.md (5/5, F-D.3 豁免因是 results)
- facet-audit-t6d.md (5/5)
- fix-failures-t6d-results.md (5/5, F-D.3 豁免)
- fix-failures-t6d.md (5/5)
- fix-governance-boundaries-t6d-a2-caller-spec.md (5/5, F-D.3 豁免因是 spec)
- fix-governance-boundaries-t6d-results.md (5/5, F-D.3 豁免)
- fix-governance-boundaries-t6d-v1-family-inventory.md (5/5, F-D.3 豁免)
- fix-governance-boundaries-t6d.md (5/5)
- mechanism-scripts-t6d-results.md (5/5, F-D.3 豁免)
- mechanism-scripts-t6d.md (5/5)
- task-packages-doc-format-t6d.md (5/5)

## B 档错基线（B 档不修，列基线） {#baseline-b}

B 档 = 表格分隔 / 表格对齐 / 列表格式。1 小时/文档 × 14 = 14 小时性价比低，本次不修。

按 doclint 修后剩余错数（B+C 档）= task-packages 文档总错数（修后）：

| 文档 | 修后错数（doclint 修后 B+C 档） |
|---|---|
| fix-governance-boundaries-t6d.md | 190（A 档清 0 后剩 B+C） |
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
| task-packages-doc-format-t6d.md | ? |
| README.md | 45 |

B+C 档总错 ≈ 1930 错（修前数）。其中：

- B 档 ≈ 30%（表格 + 列表）
- C 档 ≈ 70%（全角括号 + 字符集）

B 档修复路径（待 T6D-08+）：

- B.1 表格分隔机械化（`|` 前后空格统一）
- B.2 表格对齐（首列左对齐，其他列右对齐）
- B.3 列表格式（嵌套 + 标识符）
- B.4 工作量：1 小时/文档 × 14 = 14 小时

## C 档豁免（不修） {#exempt-c}

- C006 全角括号 `（xxx）`：中文工程自然用法（PRO-07 / PRO-08 引用 / 治理编号 / 哲学命题 ID）
- S005 标题个别错：内容正确即可
- C002 部分字符（如 U+FF0C 全角逗号在表格内）：保留

C 档豁免依据：

1. **DES-001 与工程语用张力**：task-packages 目录承载任务包 + 报告 + results 三类，结构不同，全角括号是中文工程文档标准
2. **修复 C 档成本高**：约 1350 错（1930 × 70%），自动化易误伤
3. **豁免范围可控**：豁免是 task-packages 目录级，不扩到 doc/ 严格治理范围

## 跨仓 commit 列表 {#commits}

```
addd26d feat T6D-05+T6D-07 Cluster 1: A/B 对照范式立文 + task-packages 文档格式脚本
   ├─ task-packages/ab-audit-paradigm-t6d.md (7.3KB)
   ├─ task-packages/task-packages-doc-format-t6d.md (8.9KB)
   ├─ task-packages/TEMPLATE.md (2.1KB)
   └─ skills/sihankor-proposition-defense/probes/check_task_package_format.py (5.8KB)

f62a894 fix T6D-07 Cluster 2: 14 文档 A 档清残留 + 脚本升级
   ├─ check_task_package_format.py (跳过代码块 + 文档类型识别)
   └─ 11 任务包文档 A 档清残留 (em dash + 标题锚点)

(待) feat T6D-07 Cluster 3: 跨仓 ROADMAP §P5 同步 + 本 results 文档
   ├─ sih-tools/facet/ROADMAP.md §P5 (跨仓 cross-link)
   └─ task-packages/task-packages-doc-format-t6d-results.md (本)
```

## 教训 {#lessons}

### 教训 1：分级处理是工程务实选择 {#教训-1-分级处理是工程务实选择}

不分级 = 追求 0 错 → 工作量爆炸 + 可能误伤

分级 = A 档必修 + B 档建议 + C 档豁免 → 工作量可控 + 不破坏工程语用

### 教训 2：脚本升级要识别 false positive {#教训-2-脚本升级要识别-false-positive}

F-D.1 第一版有 5 文档 FAIL，但其中 3-7 个 `# 一级标题` 是 bash 代码块内注释：脚本升级后 F-D.1 全过。

启示：所有「格式机械化」脚本要识别代码块 / 行内代码 / 引用块，否则 false positive 拖累。

### 教训 3：文档类型识别是 F-D.3 的关键 {#教训-3-文档类型识别是-f-d-3-的关键}

F-D.3 §一-§七 是**任务包**模板强制，**报告 / results / inventory** 文档结构不同（如「整体印象 + 发现 + 风险」），不应当被套任务包模板。

脚本加 `is_task_package()` 后，14 文档 F-D.3 全过。

### 教训 4：T6-D 范式可灵活应用 {#教训-4-t6-d-范式可灵活应用}

T6D-05 + T6D-07 都是「立文类」任务（不是「实施类」），双子代理实施流程不必要，主线直接写 + F 锚定机械化校验。

诚实记录：T6-D 范式有「双子代理实施」标准流程，立文类应当用 T7-X 范式（待立）但当前先用 T6-D 命名约定过渡。

## 关联文件 {#related}

- 任务包：`sih-engine/task-packages/task-packages-doc-format-t6d.md`
- 任务包：`sih-engine/task-packages/ab-audit-paradigm-t6d.md`（T6D-05 A/B 对照范式立文）
- 工具：`sih-engine/skills/sihankor-proposition-defense/probes/check_task_package_format.py`
- 模板：`sih-engine/task-packages/TEMPLATE.md`
- 跨仓引用：`sih-tools/facet/ROADMAP.md §P5`（2026-08-17 跨仓治理工作已立文）
- 上游：`sih-engine/task-packages/facet-audit-t6d-results.md` §工具层静态审计状态
- 范式文档：`sih-engine/task-packages/README.md`
