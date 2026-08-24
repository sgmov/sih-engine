# selector-round-level：路择轮级谓词族四件落地

> task-packages 治理任务
> 承接：COURSE-v1 第 2 段轮级族、selector/CONTRACT.md 修订二、孵化环第五跑续
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— 本任务包无偏离，标准 T6-D
> 日期：2026-08-22

## 一、问题陈述 {#problem}

- 路择记录级族已落地即单件材料判定，轮级族待建即按轮产出判定，方向页原文四件即锚定密度、常设约束核对、断言须附清单、推导结论与既有登记冲突即告警。
- 回归标本两件即「无孤儿路径」与「doclint 分支冲突」，规则须能抓住这两类真出过的事。
- 上轮接口缝教训：双子代理对空数组语义与报错文案各自理解，本包把全部边界钉到字段级。

## 二、关键设计 {#design}

### 2.1 轮记录（在材料五字段上加一键）

```json
{
  "id": "r-001", "path": "task-packages/xxx.md", "anchors": ["task-packages/**"],
  "requested_writes": [], "state": "process",
  "round": {
    "assertions": [
      {"claim": "书简外切裁定在册", "evidence": ["sih-engine/doc/proposal/PRO-007-naming-clearance-draft.md:30"]}
    ],
    "conclusions": [{"kind": "establishes_term", "term": "路择"}],
    "receipts": [{"tool": "nomenclator", "exit_code": 1, "flagged_terms": ["双门槛"]}]
  }
}
```

round 键缺省或非对象时，凡轮级谓词对该材料一律判败即 fail-closed。assertions 与 conclusions 与 receipts 缺省按空数组处理。

### 2.2 四谓词语义与参数

一 anchor_density：params 含 density_threshold 浮点默认 1.0 与 resolve_root 字符串必填。语义：逐断言查 evidence 数组，条目为「路径」或「路径:行」形；条目路径在 resolve_root 下解析为真实存在文件方计一个有效锚。有效锚不少于一条的断言占比小于 density_threshold 即判败。route_on_fail 默认 siding。

二 standing_constraints：params 含 required_receipts 字符串数组。语义：round.receipts 的 tool 字段集合须覆盖 required_receipts 全部，缺一即判败。route_on_fail 默认 siding。

三 assertion_lists：无 params。语义：每条断言必须携带 evidence 数组且非空，缺数组或空数组即判败。route_on_fail 默认 siding。与 anchor_density 分工：本件只查结构在场非空，不解析路径真伪。

四 registry_conflict：无 params。语义：逐结论查，kind 为 establishes_term 且其 term 出现在任一 receipt 的 flagged_terms 内即冲突。冲突不是路由败而是批级告警，告警形态 {"kind": "registry_conflict", "term": "<词>", "conclusion": "<kind>"}，入 summary.alarms，有告警退出码一。无 route_on_fail 配置位。

### 2.3 包与向后兼容

新包 packs/round/ 即 manifest.toml 加 routes.toml，结构同 packs/core，谓词用上述四件可混入记录级四件。packs/core 与既有全部语料一字不动。既有六十五测必须零修改全绿，packs/core 上的既有运行输出逐字节不变。

### 2.4 CLI 与输出

子命令仍唯一即 selector route。routed 每件增 round 字段即布尔标记是否轮记录。summary 增 conflicts 计数可缺省。退出码语义不变即 0 无告警、1 有告警含 registry_conflict、2 错误。

## 三、工作清单 {#work}

### Cluster 1（双子代理并行）

- [ ] X1 实现件：四谓词求值、轮记录装载 fail-closed、registry_conflict 告警入批、单元测试
- [ ] X2 数据件：packs/round 两 TOML、轮语料含两标本复刻、空轮包、第五跑轮基线记录、expectations

### Cluster 2（主线串行验证）

- [ ] 契约修订二升 0.2.0 主线亲写
- [ ] F1 至 F9 逐条实跑即集成测试
- [ ] 旧件零伤验证即六十五测零改全绿加 packs/core 输出逐字节比对
- [ ] 兄弟回归全绿
- [ ] 契约过检词零违例后上链
- [ ] pre-output-self-check skill 增轮级调用指引并同步投影
- [ ] COURSE-v1 第 2 段收尾、AGENTS 路择行补按轮判定
- [ ] 结果文件与双仓 commit

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 孤儿路径标本 | 数据治理 | 断言 evidence 指向不存在文件，anchor_density 判败走 siding；全真路径通过 |
| F2 密度阈值 | 元层工具 | 有效锚占比低于 density_threshold 判败，达标通过 |
| F3 常设约束 | 元层工具 | 缺任一 required_receipts 判败走 siding，齐则过 |
| F4 断言清单 | 元层工具 | 断言缺 evidence 数组或空数组判败，非空过 |
| F5 冲突告警标本 | 数据治理 | establishes_term 结论撞 receipt 的 flagged_terms 出 registry_conflict 告警退出码一，无冲突退出码零 |
| F6 旧件零伤 | 跨族治理 | 既有六十五测零修改全绿，packs/core 语料运行输出与上轮逐字节一致 |
| F7 只读确定 | 元层工具 | 轮语料双跑逐字节一致，源码扫描零写调用 |
| F8 空腹轮包 | 元层工具 | 零谓词轮包下轮记录全走 pass_route 零告警 |
| F9 轮基线 | 数据治理 | 第五跑真实轮记录即断言取自结果文件真锚、回执取自链上真报告，路由入档 CALL-LOG |

## 五、必读文件 {#read}

- sih-tools/selector/CONTRACT.md 即现行契约
- sih-engine/task-packages/selector-cutout-t6d.md 与 results 即上轮范式与接口缝教训
- sih-tools/COURSE-v1.md 第 2 段轮级族定义
- sih-tools/selector/packs/core 即包形态参照

## 六、约束 {#constraints}

1. 零 LLM 零写路径零环境读取，仅标准库六模块沿用。
2. 向后兼容为硬约束即旧测零改旧输出逐字节不变，违者即败。
3. packs/core 与既有语料一字不动。
4. 回执自包含即 flagged_terms 写在轮记录内，selector 不解析外部报告文件不读检词包，零跨包耦合。
5. 空数组语义钉死：manifest 的 exclude 允许空数组，报错文案沿用现行字串不得改动。
6. 范围不越轮级四件，三问正式化与引擎侧不碰。

## 七、验收标准 {#acceptance}

- [ ] F1 至 F9 逐条过并列表于结果文件
- [ ] selector 全测绿含新轮级测试，兄弟回归全绿
- [ ] 契约修订二过检词零违例上链
- [ ] 双仓 commit

## 八、风险点 {#risks}

- 上轮接口缝重演：本包 2.1 至 2.4 钉到字段级，主线集成测试兜底。
- 轮记录字段膨胀诱发语义解读：四谓词只作结构与存在性判定，不解读 claim 文本。

## 九、范式偏离声明 {#deviation}

无偏离。标准 T6-D：X1 与 X2 并行，主线亲写契约修订与集成验收。

## 十、关联文件 {#related}

- 任务包源：COURSE-v1 第 2 段、selector/CONTRACT.md
- 上轮：task-packages/selector-cutout-t6d.md
- 谱系：task-packages/README.md、skills/sihankor-t6d/SKILL.md
