# redtest-t6d：检词计数断言去僵化与路择轮语料锚点迁位

> T6D-XX task-packages 治理任务
> 承接：sess-zcode-260827-redtest 三问意图即 2026-08-27 trail 意图事件、cleanup 与 structmig 两批遗留测试债
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：清残留类，确定性小修不派双子代理，主线单线执行**
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 红点一：检词真实包测试 `tests/test_realpack.py:29` 断言 `len(pack.lazy) == 10`，懒波册是活册，近批三名（名随物走、范式第二形态、测量侧执行入口）升格已立后缩至 7，等值断言对合法演化双向脆化，套件 1 红 22 绿。
- 红点二：路择轮语料七件全带 `task-packages/**` 引用，structmig 批已把任务包迁至 `sih-engine/sih/event/plan/`，其中四件（f1、f2 活锚、f3、spec-conflict）的活引用全死，R102 以 `resolve_root=/Users/moc/workspaces/SiHankor/sih-engine` 拼接验存在性得密度零落侧线，套件 2 红 138 绿。
- 边界：不动检词引擎与 `packs/core` 词册，不动路择谓词实现与 R102 resolve_root 与密度阈值与期望表路由值，孤儿标本语料的死证据保持死。

## 二、关键设计 {#design}

### 2.1 检词断言去僵化

`len(pack.lazy) == 10` 改为下限断言 `len(pack.lazy) >= 5`。理由：懒波册只随登记增、随升格减，双向演化皆合法，下限 5 在现状 7 之下再留两名升格余量，仍保懒波册非空且有规模的检验力。terms 与 dead 与 candidates 的既有下限断言不动。

### 2.2 路择轮语料锚点迁位

活引用迁移规则：`task-packages/selector-cutout-t6d-results.md` 改 `sih/event/plan/selector-cutout-t6d-results.md`，`task-packages/selector-round-level-t6d.md` 改 `sih/event/plan/selector-round-level-t6d.md`，涉及 f1、f2、f3、f4、spec-conflict、no-round-key、spec-orphan-path 七件的 path 与 anchors 字段与活证据字段即 spec-orphan-path 的 path 指向活文件须同迁其孤儿证据保持死。孤儿标本证据 `task-packages/selector-density-missing-evidence.md:1` 与 `task-packages/selector-orphan-path-missing.md:1` 保持指向不存在路径即按设计死。迁移恢复语料原语义，期望表七条路由值全部不动。

### 2.3 轮包域声明的地位

`packs/round/routes.toml` 的 `domain.include = ["task-packages/**", "doc/**"]` 是包内信息性声明，路由不执行域过滤（route.py 无域检查），语料实位早已在域声明外即测试曾经全绿佐证。本批不动包配置，域声明的更新留待轮包下次版本变更一并处理。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 检词 `tests/test_realpack.py` 懒波等值断言改下限
- [ ] 路择 `tests/fixtures/round/` 七件语料活引用迁现位

### Cluster 2：主线串行验证

- [ ] 检词套件全绿
- [ ] 路择套件全绿
- [ ] 双仓免参对表退出码零

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 检词计数去僵化 | 代码修复 | `uv run pytest tests/` 于 nomenclator 退出码零全绿 |
| **F-2** 轮语料锚点迁位 | 数据治理 | `uv run pytest tests/` 于 selector 退出码零全绿，期望表路由值与迁移前一致 |
| **F-3** 修复不引入绕行 | 跨族治理 | lease reconcile 双仓免参均退出码零 |

## 五、必读文件 {#read}

- 必读 1：本任务包与 `sih/event/plan/selector-cutout-t6d-results.md` 即语料锚定目标现位
- 必读 2：`sih-tools/selector/packs/round/routes.toml` 即 R102 resolve_root 与密度阈值
- 必读 3：`sih-tools/nomenclator/packs/core/lazy.json` 即懒波册现状 7 条

## 六、约束 {#constraints}

1. **零 LLM 调用**于工具执行层即测试与语料修改是确定性操作
2. **机械判据**即三 F 锚定全过方结算
3. **不扩大 scope** 即不动词册不动谓词不动包配置不动期望表路由值
4. **F 锚定状态记录**于结果档，认证经 scribegate 入链

## 七、验收标准 {#acceptance}

本任务包验收 = 3 项：

- [ ] 检词与路择两套件全绿即 F-1 F-2
- [ ] 双仓免参对表退出码零即 F-3
- [ ] 结果档落 `sih/event/plan/redtest-t6d-results.md`，管线三件认证入链，双仓段结算收约

## 八、风险点 {#risks}

- 路择语料 path 与 anchors 字段是信息性字段，路由只看 round 数据内 evidence，迁移须三字段同改防语料自相矛盾。
- 孤儿标本若误迁即死证据变活锚，F-2 密度语义破坏，迁移须逐件核对证据清单。

## 九、范式偏离声明 {#deviation}

本任务包**偏离 T6-D 范式**的「双子代理实施」流程：

- 理由：清残留类是确定性小修即一处断言加七件单行语料
- 偏离：双子代理改为主线单线执行
- 保留：T6-D 命名约定 + F 锚定 + 跨仓同步 + 管线认证

## 十、关联文件 {#related}

- 任务包源：`sih/state/plan/cleanup-t6d.md` 与 `sih/state/plan/structmig` 批即两债来源
- 工具：`sih-tools/nomenclator/` 与 `sih-tools/selector/`
- 跨仓引用：`sih/event/plan/selector-cutout-t6d-results.md` 与 `sih/event/plan/selector-round-level-t6d.md`

## 十一、请求写入 {#requested-writes}

- `sih-tools/nomenclator/`
- `sih-tools/selector/`
- `sih-tools/scribe/`
- `sih-tools/lease/`
- `sih-engine/sih/state/plan/redtest-t6d.md`
- `sih-engine/sih/event/plan/redtest-t6d-results.md`
