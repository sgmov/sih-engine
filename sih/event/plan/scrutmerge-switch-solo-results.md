# scrutmerge-switch-solo 切换批停批档

> 委外代理亲写零子代理
> 承接：scrutmerge-switch-solo 任务包、用户 2026-09-01 批准通行令、DEC-013 三步曲第三步执刀
> 收口态：主会 2026-09-01 裁定停批转 scrutmerge-goldfix-solo 整改批
> 日期：2026-09-01

## 概览 {#overview}

切换批双跑判据揭出引擎件与工具件三层差异，三层差异按主会裁定即切换挂起转 goldfix 整改批。三层差异为 GOV-002 路径回显调用形伪差异（属双参形差异非等价性差异）、finding 结构差异（引擎平铺对工具嵌套）、规则语义差异（SPEC-013 引擎误报五处与 DEC-020 引擎漏报六处与字符集消息格式 U+U+ 双前缀差）。根因为金向量六件全零发现净目标对结构与语义天然免疫等价性从未被字节级钉住。规约洞在 SPEC-013 § 验收判据 A1 未要求含发现目标。本档如实记录停批事实与三层差异并由 goldfix 整改批承续。

## 三层差异 {#three-diff}

### 第一层：GOV-002 路径回显调用形伪差异

- 引擎件：CLI 传入相对路径 `doc/governance/GOV-002-mainline-lock-v1.md` 时，targets 与 content_hashes 与 finding.path 字段均回显相对路径
- 工具件：CLI 传入相对路径时，targets 与 content_hashes 与 finding.path 字段均回显绝对路径（`str(pathlib.Path(target))` 走 resolve）
- 同参形：CLI 两侧均传入绝对路径 `/Users/moc/workspaces/SiHankor/sih-engine/doc/governance/GOV-002-mainline-lock-v1.md` 时，输出逐字节一致
- 性质：调用形差异非等价性差异，SPEC-013 修订四已明记双跑同参形条款

### 第二层：finding 结构差异

- 工具件 finding 形：嵌套形 `{pack, rule_id, path, location: {line}|{path}, message}`
- 引擎件 finding 形：平铺形 `{rule_id, line, message, pack}`
- 根因：金向量六件全零 findings，净目标对结构差异天然免疫
- 修复位：goldfix 整改批 § 主体步骤 2 引擎 finding 序列化改嵌套形

### 第三层：规则语义差异

- S 系误报：SPEC-013 引擎件报五处 S002 标题层级跳级，工具件 0
- C006 漏报：DEC-020 引擎件 4 处，工具件 6 处
- 字符集消息：引擎件 `字符 U+{char} 超出...` 工具件 `字符 U+U+{char} 超出...` 双前缀
- 修复位：goldfix 整改批 § 主体步骤 3 S 系映射修、C006 手写补报、字符集消息双前缀照抄

## 根因 {#root}

金向量六件全零发现，净目标对 finding 结构与规则语义天然免疫，等价性从未被字节级钉住。规约洞在 SPEC-013 § 验收判据 A1 仅要求同包同目标逐字节一致，未要求金向量必须含发现目标。

## 收口 {#close}

- 切换批 lease session `c951fcf6360be7f4` 已 revoked（worktree 删除、branch 删除）
- 切换批占位 allow 30 项已释放
- 双跑证据件六件（switch-stop-engine-SPEC-013 / tools-SPEC-013 / engine-DEC-020 / tools-DEC-020 / engine-GOV-002 / tools-GOV-002）已复制入 `sih-tools/scribe/reports/2026-09-01-switch-stop-*` 保全
- 主会 2026-09-01 裁定：本档落地为停批凭证，转 `scrutmerge-goldfix-solo` 整改批承续 DEC-013 三步曲第三步执刀

## 后续动作 {#next}

goldfix 整改批承接本档：脏目标金向量补冻、引擎件结构与语义对齐、SPEC-013 修订四加金向量须含脏目标条款与双跑同参形条款、T6 管线核阅腿**仍以工具件为校验位**（切换未成引擎件仍任校验位），工具件退役标注留待切换批二次执刀。
