# scrutmerge-goldfix-solo 整改批 round-2 完工报告

> 委外代理亲写零子代理
> 承接：dispatch-round2（主会 2026-09-01 复核裁定打回两件）
> 队形：单线形 solo
> 收口态：完成

## 整改路径

1. **包平价回正**：`src/scrutinator/packs/des-001/rules.toml:29` 与 `src/scrutinator/packs/des-001-mathe/rules.toml:45` S004 消息由 `{#锚点}` 恢复 `{{#锚点}}` 双花括号原样
2. **引擎渲染器改造**：`src/scrutinator/rule.rs:22-44` 新增 `render_message(template, subs)` helper，对齐 Python `str.format` 转义语义（`{{` → 字面 `{`、`}}` → 字面 `}`，后接 `{name}` 占位符替换）；4 处替换位由 `rule.message.replace(...)` 改 helper（`rule.rs:291` 字符集 / `410` 锚点缺失 / `425` 首个 H2 名 / `443` 跳级）
3. **三件验证齐过**：cargo test 107 passed 零漂移、SPEC-013/DEC-020/GOV-002 双跑 cmp IDENTICAL、rules.toml 双仓零差
4. **版控收编**：engine 仓 8f85112 commit 含 8 件（rule.rs + 2 rules.toml + 链 59 事件 + 结果档追加 + 3 件材料），tools 仓 5bfb95b6 commit 含 10 件 untracked（5 件 dispatch 明示 + 5 件二轮新增），0add10f3 commit 含 5 件 CALL-LOG/meter 补笔

## 渲染器改动位

`src/scrutinator/rule.rs:22-44` 新增 helper：

```rust
/// 消息模板渲染：对齐 Python `str.format` 转义语义。
///
/// 处理序：
/// 1. `{{` → 字面 `{`、`}}` → 字面 `}`（承 str.format 双花括号转义）
/// 2. `{name}` → 替换为对应值
fn render_message(template: &str, subs: &[(&str, &str)]) -> String {
    const OPEN: &str = "\u{0001}";
    const CLOSE: &str = "\u{0002}";
    let mut out = template.replace("{{", OPEN).replace("}}", CLOSE);
    for (name, value) in subs {
        out = out.replace(&format!("{{{name}}}"), value);
    }
    out.replace(OPEN, "{").replace(CLOSE, "}")
}
```

4 处替换位调用形：
- `rule.rs:291`（CharsetForbid/CharsetAllow 字符集消息）：`render_message(&rule.message, &[("char", &format!("U+{:04X}", cp))])`
- `rule.rs:410`（HeaderStructure.anchor_required 锚点缺失）：`render_message(&rule.message, &[("level", &lv_str)])`
- `rule.rs:425`（HeaderStructure.first_h2_name 首 H2 名）：`render_message(&rule.message, &[("title", &bare)])`
- `rule.rs:443-447`（HeaderStructure.no_level_skip 跳级）：`render_message(&rule.message, &[("prev", &p_str), ("level", &lv_str)])`

## 三件验证证据

### 1. cargo test --lib

```
test result: ok. 107 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out
```

含金向量 12 件（净目标 6 + 脏目标 6）逐字节断言全过 + 14 件既有测试全过 + 1 件 CLI 双形一致全过 + 4 件 scrutinator 模块测试 + 60 件其他模块测试。

注：单次跑出现 ask3repeater::test_valid_record_passes flake，stashed 后跑 1 passed，重跑全 107 全过，确认为 pre-existing 偶发 flake，与本批改动无关。

### 2. 双跑 cmp 三目标

| 目标 | engine exit | tools exit | cmp exit | engine 字节 | tools 字节 |
|---|---|---|---|---|---|
| SPEC-013-scrutiny-mergeback-gap.md | 0 | 0 | 0 | 871 | 871 |
| 020-deyi-component-naming.md | 1 | 1 | 0 | 3073 | 3073 |
| GOV-002-mainline-lock-v1.md | 0 | 0 | 0 | 869 | 869 |

全 cmp_exit=0 = IDENTICAL，engine 与 tools 退出码 0/1/0 完全一致。

### 3. rules.toml 双仓 diff

| 包 | engine 路径 | tools 路径 | diff exit |
|---|---|---|---|
| des-001 | `sih-engine/src/scrutinator/packs/des-001/rules.toml` | `sih-tools/scrutinator/packs/des-001/rules.toml` | 0 |
| des-001-mathe | `sih-engine/src/scrutinator/packs/des-001-mathe/rules.toml` | `sih-tools/scrutinator/packs/des-001-mathe/rules.toml` | 0 |

全 diff_exit=0 = 双仓逐字节一致。

## 收编文件清单

### engine 仓（commit 8f85112，8 件）

- `src/scrutinator/rule.rs`（+36 / -7）：render_message helper + 4 替换位改 helper
- `src/scrutinator/packs/des-001/rules.toml`（1 行）：S004 消息恢复 `{{#锚点}}`
- `src/scrutinator/packs/des-001-mathe/rules.toml`（1 行）：S004 消息恢复 `{{#锚点}}`
- `sih/event/plan/scrutmerge-goldfix-solo-results.md`（+47 / -11）：追加二轮记录
- `sih/event/plan/scrutmerge-goldfix-solo-materials/dispatch-round2.md`（新文件）：本轮整改指令
- `sih/event/plan/scrutmerge-goldfix-solo-materials/round2-cmp.json`（新文件）：三件验证证据件
- `sih/event/plan/scrutmerge-goldfix-solo-materials/round2-checksheet.json`（新文件）：检核清单
- `sih/event/trail/2026-09-01.ndjson`（+13 / -0）：本批追加 1 intent + 3 认证 4 笔

### tools 仓（commit 5bfb95b6，10 件）

**dispatch 明示 5 件**：
- `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-record.json`（首轮 ask3 record 补登）
- `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-validation.json`（首轮 ask3 validation 补登）
- `scribe/reports/2026-09-01-scrutmerge-goldfix-solo-elicit-signals.ndjson`
- `scribe/reports/2026-09-01-ask3-cmdface-r2-record.json`（已 claimed 未入仓补救）
- `scribe/reports/2026-09-01-ask3-cmdface-r2-validation.json`（已 claimed 未入仓补救）

**二轮新增 5 件**：
- `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-r2-record.json`（二轮 ask3 record）
- `scribe/reports/2026-09-01-ask3-scrutmerge-goldfix-solo-r2-validation.json`（二轮 ask3 validation，ask3 验证零发现）
- `scribe/reports/2026-09-01-r2-goldfix-formatter-results.json`（二轮化格，0 changes）
- `scribe/reports/2026-09-01-r2-goldfix-nomenclator-results.json`（二轮检词，0 findings）
- `scribe/reports/2026-09-01-r2-goldfix-scrutinator-results.json`（二轮核阅，域外 exit-2 如实记）

### tools 仓（commit 0add10f3，5 件）

- `scribe/CALL-LOG.md`（+1 行）：round-2 留痕
- `nomenclator/CALL-LOG.md`（+1 行）：round-2 检词留痕
- `scrutinator/CALL-LOG.md`（+1 行）：round-2 核阅留痕
- `formatter/CALL-LOG.md`（+1 行）：round-2 化格留痕
- `meter/counts/2026-09-01.ndjson`（+1 行）：round-2 包独立计数行

## 认证清单

| 事件哈希 | 类型 | 工具 | 退出码 | 报告 |
|---|---|---|---|---|
| 8331865503c303bfd19af3c495850384bbb097d4314fc1dc9f3de59ffd62a333 | intent_refined | scribe | 0 | ask3-scrutmerge-goldfix-solo-r2-record.json + r2-validation.json |
| d7c6fea69c3a70f5a854c18a2d7a80a32e6e9fc2a6651dc6938bf9d39ac72187 | certification_completed | scribe | 0 | r2-goldfix-formatter-results.json |
| 5ad47aac4067d447a43c9dbb6e1e8b7ea4fe42e66ff21b1336e8a51770e52d44 | certification_completed | scribe | 0 | r2-goldfix-nomenclator-results.json |
| d9dfefcbfa636fff36264466291011a26a72f4d517fad831ee83dc84a8281bcf | certification_completed | scribe | 2 | r2-goldfix-scrutinator-results.json（域外 exit-2 如实记承 pkgclose/viewreg-solo 先例） |

## 双仓 commit 号

- **engine 仓**：`8f85112e586486d2e8d714ba100c088b8e9d8bd0`（main 分支）
- **tools 仓**：`0add10f3`（CALL-LOG/meter 补笔）/ `5bfb95b6`（10 件 untracked 入仓）（integral-stage-build 分支）

## 链 verify

```
{
  "status": "valid",
  "events": 59,
  "first_hash": "c2ddb2c2b049e785c4f8f8e1e0dade17bbcef51d4dcd7bb1e9f71c3f49acaf19",
  "last_hash": "d9dfefcbfa636fff36264466291011a26a72f4d517fad831ee83dc84a8281bcf"
}
```

55 → 59 事件（追加 1 intent + 3 认证），valid。

## 误差申报

第一轮 tools 仓 commit 0238bc6b 消息「6 件 switch-stop 证据保全 + ask3 record/validation + elicit/identity + 6 笔 goldfix 管线认证 + 4 工具 CALL-LOG 补笔 + meter counts」声称含 ask3 record/validation 实际未含——查 git show 0238bc6b 实际入库列表不含 `2026-09-01-ask3-scrutmerge-goldfix-solo-{record,validation}.json` 两件。**本轮补救**：tools 仓 5bfb95b6 commit 显式列入两件，含 commit 消息中明示「已 claimed 未入仓补救」。

## 越线与误差

- 工具件 scrutinator 的 src/tests/packs 零改动（红线守住）
- 引擎改动限 src/scrutinator/ 域（红线守住）
- 金向量冻结后零漂移（cargo test 107 passed 即证，工具件零改）
- U+U+2026 小瑕照抄不修（红线守住）
- 上链前等绿、findings 亲读、禁管道掩退出码（红线守住）
- 改引擎源码后 settle 前 cargo build + 实跑三目标双跑 + cargo test（红线守住）
- 撞锁即报（lease status 零锁 2 active session 与本批无冲突，本批未撞）

## 后续动作

- 切换批二次执刀：核阅腿换引擎件 + AGENTS.md 字节级 diff 备查仅限三行核阅位 + 工具件退役标注转兼容只读 + 双仓 settle
- 工具件 BATCH-FACE 新坑位两条（引擎源码批 settle 前必 cargo build 实跑、U+U+2026 小瑕照抄不修）落入 BATCH-FACE.md
- DEC-013 修订：承本批与切换批成果完成融回门三步曲整体记录
- render_message helper 注释明示对齐工具件 cli.py L101/L125（双跑锚定，切换时如工具件路径变更须同步审）
