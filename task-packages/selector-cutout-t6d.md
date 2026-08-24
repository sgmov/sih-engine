# selector-cutout：路择第五跑即记录级谓词路由器外切落地

> task-packages 治理任务
> 承接：PRO-007 已签项路择／Selector 词条、COURSE-v1 第 2 段记录级先行、孵化环第五跑
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— 本任务包无偏离，标准 T6-D
> 日期：2026-08-22

## 一、问题陈述 {#problem}

- 路择／Selector 名已立即 2026-08-19 与 08-20 用户签署，工程零落地，为现行最大名实错位。
- 工具线方向页第 2 段已定路线即记录级配置族先行、轮级族随后，本包只做记录级。
- 回归标本：过程件 2026-08-18 记 11 件，现存 8 件即漂移如实记录，基线以现存 8 件为准。

## 二、关键设计 {#design}

### 2.1 词条承接

词条原文：受托持人批的谓词包，对到达材料逐件机械判定，择定主线、停放、丢弃三路之一。空腹零 LLM，只读不写，不裁决，判在别处。有余告警与饥饿告警为词条职能。英文路径词 mainline、siding、scrap_track。

### 2.2 材料记录（本规范唯一被路由对象）

材料为一个 JSON 文件，字段四项：

```json
{"id": "m-001", "path": "task-packages/xxx.md", "anchors": ["doc/**"], "requested_writes": ["sih-tools/selector/**"], "state": "process"}
```

id 与 path 与 state 为字符串，anchors 与 requested_writes 为字符串数组。

### 2.3 谓词包（TOML，核阅规则包同构）

包目录两文件。manifest.toml：name、version、domain 含 include 与 exclude。routes.toml：

```toml
[[predicates]]
id = "R001"
kind = "schema_required"
fields = ["id", "path", "state"]
route_on_fail = "scrap_track"

[[predicates]]
id = "R002"
kind = "state_annotation"
allowed = ["process", "draft", "signed", "archived"]
route_on_fail = "siding"

[[predicates]]
id = "R003"
kind = "anchor_whitelist"
whitelist = ["doc/**", "skills/**", "task-packages/**"]
route_on_fail = "siding"

[[predicates]]
id = "R004"
kind = "write_boundary"
allowed_roots = ["sih-tools/**", "sih-engine/src/**"]
route_on_fail = "scrap_track"

[defaults]
pass_route = "mainline"

[alarms]
siding_surplus_threshold = 5
mainline_starvation_threshold = 1
```

谓词语义：schema_required 查字段在且类型正；state_annotation 查 state 在 allowed 枚举内；anchor_whitelist 查 anchors 全部匹配 whitelist 某一 glob 即 fnmatch；write_boundary 查 requested_writes 全部匹配 allowed_roots 某一 glob。

### 2.4 判定算法（顺序钉死）

谓词按包内声明顺序逐条评估，首败定路即首个失败谓词的 route_on_fail 决定该材料去向，全过走 defaults.pass_route。逐材料输出全部谓词的过败记录。

### 2.5 告警（批级）

siding 件数大于等于 siding_surplus_threshold 即有余告警；mainline 件数小于等于 mainline_starvation_threshold 即饥饿告警。告警入报告并置退出码一。

### 2.6 CLI 与输出（面钉死）

唯一子命令：`selector route --pack <目录> <材料.json ...>`。stdout 输出 JSON：

```json
{"header": {"tool": {"name": "selector", "version": "0.1.0"}, "pack": {"name": "...", "version": "..."}, "domain": {...}},
 "routed": [{"id": "...", "path": "...", "route": "mainline|siding|scrap_track", "failed_predicate": "R003|null", "checks": [{"id": "R001", "pass": true}]}],
 "summary": {"total": 8, "mainline": 3, "siding": 4, "scrap_track": 1, "alarms": [{"kind": "siding_surplus", "count": 4, "threshold": 5}]},
 "status": "routed"}
```

退出码：0 即路由毕且无告警；1 即路由毕但有告警；2 即包非法或材料文件缺失或 JSON 坏。只读：本工具无任何写路径，源码零写调用。

## 三、工作清单 {#work}

### Cluster 1（双子代理并行）

- [ ] X1 实现件：sih-tools/selector 即 pyproject、src/selector 五模块（__init__ 含版本、pack、predicates、route、cli）、单元测试，严格按本包 2.2 至 2.6 规范
- [ ] X2 数据件：sih-tools/selector/packs/core 两 TOML 即 2.3 样例原样、tests/fixtures/materials 覆盖 F1 至 F5 全部正反例、baseline/process-files 八件过程件描述记录、README、CALL-LOG 表头

### Cluster 2（主线串行验证）

- [ ] 工作区 pyproject members 增 selector
- [ ] 集成测试驱动 X2 数据过 X1 代码
- [ ] F1 至 F8 逐条实跑验证
- [ ] 契约 CONTRACT.md 主线亲写
- [ ] 兄弟回归与全测
- [ ] 契约走化格核阅检词书简四步链认证
- [ ] COURSE-v1 第 2 段实现状态、AGENTS 文件索引
- [ ] 结果文件与 commit

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 三路 | 数据治理 | 三份材料各走 mainline、siding、scrap_track 一路，工具判定与预期逐件一致 |
| F2 谓词四件 | 元层工具 | schema_required、state_annotation、anchor_whitelist、write_boundary 各有正反例，反例走各自 route_on_fail |
| F3 首败定路 | 元层工具 | 材料连败两谓词且 route_on_fail 不同，路由取包序在前者 |
| F4 有余告警 | 数据治理 | siding 件数达阈值出告警退出码一，低于不出 |
| F5 饥饿告警 | 数据治理 | mainline 件数不高于阈值出告警，高于不出 |
| F6 空腹 | 元层工具 | 空谓词包全走 pass_route 即 mainline，零告警退出码零 |
| F7 只读确定 | 元层工具 | 同输入双跑输出逐字节一致，源码扫描零写调用 |
| F8 基线八件 | 数据治理 | 八件过程件描述记录全路由，报告落 baseline，CALL-LOG 登记 |

## 五、必读文件 {#read}

- sih-engine/doc/proposal/PRO-007-naming-clearance-draft.md 已签项路择条目
- sih-tools/COURSE-v1.md 第 2 段
- sih-tools/nomenclator/CONTRACT.md 姊妹契约范式参照
- sih-tools/scrutinator/packs/des-001/manifest.toml 包形态参照

## 六、约束 {#constraints}

1. 零 LLM：判定全程纯机械，谓词求值无任何模型调用。
2. 只读不写：无写路径，无状态残留，无环境读取除命令行参数。
3. 范围钉死：只做记录级四谓词族与批级两告警，轮级族即锚定密度、常设约束核对、断言附清单、冲突告警一概不做，留待后续。
4. 不裁决：路由结果与告警是机械事实，材料去向的最终处置判在别处。
5. 依赖钉死：仅标准库，tomllib 承 TOML 解析，fnmatch 承 glob。

## 七、验收标准 {#acceptance}

本任务包验收等于：

- [ ] F1 至 F8 逐条过并在结果文件列表
- [ ] selector 全测绿且兄弟回归全绿即核阅六化格十书简二十二检词二十三
- [ ] CONTRACT.md 过四步链上认证
- [ ] 双仓 commit

## 八、风险点 {#risks}

- 过程件计数漂移即 11 至 8：以现存八件为准，漂移记录在案。
- 子代理并行实现与数据格式错位：本包 2.2 至 2.6 规范钉死格式与接口面，主线集成测试兜底。

## 九、范式偏离声明 {#deviation}

无偏离。本任务包为标准 T6-D：X1 与 X2 双子代理并行，主线亲写契约与集成验收。

## 十、关联文件 {#related}

- 任务包源：PRO-007 路择条目、COURSE-v1 第 2 段
- 姊妹参照：sih-tools/nomenclator、sih-tools/scrutinator
- 谱系：ai-ex/T6-PARADIGMS.md、task-packages/README.md、skills/sihankor-t6d/SKILL.md
