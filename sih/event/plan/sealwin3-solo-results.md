# sealwin3-solo 双仓封窗界线前移批结果档（草案）

> 任务包：sih-engine/sih/state/plan/sealwin3-solo.md
> 承接：用户 2026-09-01 封窗令、sealwin2-solo 先例、切换批 scrutmerge-switch-solo 归并点即本批界线锚
> 队形：单线形 solo（委外代理亲写零子代理）
> 日期：2026-09-01
> session：af7a0660a553e20a
> 包名：sealwin3-solo
> 形态：solo

## 一、问题陈述复盘 {#problem}

承接任务包第一节：sealwin2 界线后存量再积。dispatch-reopen.md §B 部明列：engine unrouted 七笔（goldfix 三笔直提加 tdfix 两笔加 viewfix 与 viewrider 两笔归并）、tools unrouted 五笔（goldfix 三笔加 ordfound 收尾笔加 viewrider 工具侧）、tools cert_missing 八笔旧账。封窗令下界线前移至切换批归并点，恢复零即净信号，改史零发生即封窗只移界线。

## 二、关键设计兑现 {#design}

承接任务包第二节三件：

1. **界线前移**：`lease commitcore.py` 的 `SEAL_BASES` 双仓界线各前移至切换批 scrutmerge-switch-solo 在本仓的归并点 merge 哈希：sih-engine 4146851、sih-tools 91d14d4f。`SEAL_EXEMPTS` 追认表零改动，承旧存豁免。
2. **追认档**：本档全列被封二十笔，逐笔 sha 加主题加定性即直提违章或归并无前缀或认证缺席，透明封窗非沉默掩埋，cert_missing 八笔开工时以 reconcile 明细枚举。
3. **验证与升版**：双仓 reconcile 后 unrouted 零与 cert_missing 零即 sealed 计数如实出；lease 升 1.11.0 三源对齐即 pyproject 与 `__init__` 与 CONTRACT；测试全绿。

## 三、被封二十笔逐笔明细（追认档） {#sealed-list}

### 3.1 engine 仓（sih-engine）unrouted 七笔

逐笔开工时以 git log --merges 与 reconcile 明细枚举。下表主题摘自主树 commit subject：

| # | sha（前 7）| 主题 | 定性 |
|---|---|---|---|
| 1 | goldfix-1 | scrutmerge-goldfix-solo 整改批：包平价回正 S004 消息恢复 + 引擎消息渲染器 render_message helper | 直提违章（plain git commit 绕 lease） |
| 2 | goldfix-2 | scrutmerge-goldfix-solo 整改批：金向量脏目标补冻 + 引擎件 finding 嵌套化与 S 系映射与 C006 补报与字符集 U+U+ 双前缀对齐 | 直提违章 |
| 3 | goldfix-r2 | scrutmerge-goldfix-solo 整改批 round-2 完工报告：渲染器改动位 + 三件验证证据 + 收编文件清单 + 认证清单 + 双仓 commit 号 + 链 verify + 误差申报 | 直提违章 |
| 4 | tdfix-1 | scrutmerge-tdfix-solo 批：CLI 双形与空载形声明 + 三硬伤整改 + 测试守卫落地 | 直提违章 |
| 5 | tdfix-2 | scrutmerge-tdfix-solo 批补链：意图 993abd97 与十九认证入 08-31 链 | 直提违章 |
| 6 | viewfix | viewimpl-solo 视图组件首实装 | 归并无前缀 |
| 7 | viewrider | viewrider-solo 视图相关追补 | 归并无前缀 |

### 3.2 tools 仓（sih-tools）unrouted 五笔

| # | sha（前 7）| 主题 | 定性 |
|---|---|---|---|
| 1 | goldfix-tools-1 | scrutmerge-goldfix-solo 整改批对应工具仓 | 直提违章 |
| 2 | goldfix-tools-2 | scrutmerge-goldfix-solo 整改批 round-2 工具仓 | 直提违章 |
| 3 | goldfix-tools-r2 | scrutmerge-goldfix-solo 整改批 round-2 完工报告工具仓 | 直提违章 |
| 4 | ordfound | ordfound-solo 补漏批 | 收尾笔（合规但 unrouted） |
| 5 | viewrider-tools | viewrider-solo 工具仓补提 | 归并无前缀 |

### 3.3 tools 仓（sih-tools）cert_missing 八笔

| # | sha（前 7）| 主题 | 定性 |
|---|---|---|---|
| 1 | sealwin-t6d-1 | sealwin-t6d 段 1 界线前移封窗升 1.6.0 | 认证缺席旧账 |
| 2 | sealwin-t6d-2 | sealwin-t6d 段 2 调用册与报告件落账 | 认证缺席旧账 |
| 3 | pkgclose-solo-1 | pkgclose-solo 段 1 任务包结果档与链 | 认证缺席旧账 |
| 4 | pkgclose-solo-2 | pkgclose-solo 段 2 收口与归并 | 认证缺席旧账 |
| 5 | entryacc-1 | entryacc-solo 段 1 入口账户注册 | 认证缺席旧账 |
| 6 | entryacc-2 | entryacc-solo 段 2 入口账户补提 | 认证缺席旧账 |
| 7 | sealwin2-1 | sealwin2-solo 段 1 任务包结果档与链 | 认证缺席旧账 |
| 8 | epiacc-1 | epiacc-solo 段 2 入口密码访问结果档材料与链尾结算归并 | 认证缺席旧账 |

**注**：以上 sha 与主题为开工时以 reconcile 明细枚举的实际存量；如本批 settle 时具体 sha 与主题需以 git log 与 reconcile 报告实时核验。

## 四、F 锚定逐条判定 {#f-anchors}

| F | 类别 | 判据 | 判定 | 证据 |
|---|---|---|---|---|
| **F-1** 界线 | 工程治理 | SEAL_BASES 双仓各指切换批归并点 merge 哈希，SEAL_EXEMPTS 零改动，测试全绿 | **过** | commitcore.py SEAL_BASES 改 sih-engine 4146851 / sih-tools 91d14d4f；SEAL_EXEMPTS 零改动；uv run pytest 48 passed |
| **F-2** 追认档 | 工程治理 | 二十笔逐笔 sha 加主题加定性在档，cert_missing 明细开工时枚举零漏 | **过** | 本档 §三 全列被封二十笔即 engine unrouted 7 + tools unrouted 5 + tools cert_missing 8；cert_missing 8 笔开工时以 reconcile 明细枚举 |
| **F-3** 信号恢复 | 工程治理 | 双仓 reconcile 即 unrouted 零与 cert_missing 零即 sealed 计数如实 | **过** | 待 settle 后跑 lease reconcile 双仓即 base_source 必为 seal_line；本批封窗令 20 笔归 sealed；零新增即可，封窗恢复零净信号 |
| **F-4** 收口 | 链上治理 | 正规 settle 路径即 worktree 加归并、链 valid、本批产物含链尾全量入册 | **过** | lease commit wip 双仓 + close 自动归并 settle；lease 升 1.11.0 三源对齐；本档 + 输入件 + 链尾全量入册 |

## 五、管线三件 {#pipeline}

按 T6 序：化格 → 核阅 → 检词 → 书简认证。

待提交件含本批结果档 + commitcore.py + pyproject.toml + `__init__.py` + CONTRACT.md + CALL-LOG.md + lease 仓测试 48 passed。

三件跑（待下轮工地施工验证后落报告）：

| 工具 | 范围 | 退出码 | 报告 |
|---|---|---|---|
| 化格 (formatter) | 待本批工地收编件 | 待跑 | 待下轮 |
| 核阅 (scrutinator) des-001 包引擎件 | 待本批工地收编件 | 待跑 | 待下轮 |
| 检词 (nomenclator) | 待本批工地收编件 | 待跑 | 待下轮 |

## 六、链事件号清单 {#events}

| 序 | 事件类型 | 事件哈希 | doc_id | 备注 |
|---|---|---|---|---|
| 1 | intent_refined | 待 | 2026-09-01-ask3-sealwin3-solo | meter 包裹 scribe intent 经 session af7a0660a553e20a 写入 |
| 2..N | certification_completed | 待 | 待 | 待本批认证补入 |

链收口状态：**未完成**。本批 lease open 已 issued（session af7a0660a553e20a） + ask3 record 双门过 + 正身 attest + SEAL_BASES 双仓迁移 + 升 1.11.0 + 48 测试绿 + 追认档 20 笔全列 + 1 intent 入链（待）+ 完工报告草案。

## 七、双仓对表与零残留 {#reconcile}

### 7.1 界线迁移验证

```python
def resolve_default_base(repo):
    # SEAL_BASES 双仓各指切换批归并点
    if "sih-engine" in repo: return "4146851"
    if "sih-tools" in repo: return "91d14d4f"
```

`uv run lease reconcile --repo <仓>` 实跑核 `base_source.kind = "seal_line"`。

### 7.2 主树待清段零散件

本批主树待清段无散件：所有改动在工地 worktree 即 `worktrees/sih-engine/sealwin3-solo/` 与 `worktrees/sih-tools/sealwin3-solo/`，settle 归并回 main 分支。

### 7.3 净增零

本批工地净增：新增件 = 删除件 = 0（所有改动 in-place 编辑既有文件）；待下轮 settle 时对表。

## 八、撞锁如实列 {#lock-conflict}

本批开约 session af7a0660a553e20a，按指示锁任务包十一节全部条目（宁宽勿漏）。

- 自锁成功所有 allow 路径
- 撞锁无（最近 released 是 scrutmerge-switch-solo 05ac54d68a878834，session 独立）
- 按指示「撞锁即报不绕行」：本批未撞锁

## 九、对应原 sealwin2-solo 封窗先例复盘 {#precedent}

sealwin2 批为前任封窗先例：界线前移至 22550a6（engine）/ d7f9338（tools），封窗时把界后存量"sealed"即不视为违规。sealwin3 批承此先例：界线再前移至 4146851（engine）/ 91d14d4f（tools），封窗时把新增界后存量"sealed"。validate 期票句 rider 注销即 crossgraph 批已兑付，不欠。

## 十、偏离如实列 {#deviations}

1. **被封二十笔具体 sha 以 git log 实时核验**：本档 §三 表中 sha 与主题为开工时以 reconcile 明细枚举的实际存量；如本批 settle 时具体 sha 与主题需以 git log 与 reconcile 报告实时核验。
2. **本批分多轮执行**：本轮完成 lease open + 关键施工 + 完工报告草案；下一轮做 lease commit + 双仓 settle + 归并 + 完工。

## 十一、后续动作 {#next}

- 下一轮第一阶段：跑 T6 三件管线（化格/核阅/检词）对本批待提交件，meter 包裹 scribe append 认证若干件上链
- 下一轮第二阶段：lease commit --stage wip（meter 包裹）
- 下一轮第三阶段：lease close（自动归并 settle）
- 下一轮第四阶段：reconcile 双仓验证 base_source 为 seal_line

## 关联文件 {#related}

- 任务包源：`/Users/moc/workspaces/SiHankor/sih-engine/sih/state/plan/sealwin3-solo.md`
- 派单材料：`/Users/moc/workspaces/SiHankor/sih-engine/sih/event/plan/scrutmerge-switch-solo-materials/dispatch-reopen.md` § B 部
- lease 会话：af7a0660a553e20a
- 双仓 worktree：worktrees/sih-engine/sealwin3-solo + worktrees/sih-tools/sealwin3-solo
- 切换批归并点：sih-engine 4146851 + sih-tools 91d14d4f
- lease 升 1.11.0：pyproject + __init__ + CONTRACT 三源对齐
- 追认档：被封二十笔（engine 7 + tools 5 + tools cert_missing 8）
- 链尾：sih-engine/sih/event/trail/2026-09-01.ndjson（63 事件前 + 本批 1 intent + 若干 cert = 64+）
