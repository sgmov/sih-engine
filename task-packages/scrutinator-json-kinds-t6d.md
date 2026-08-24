# scrutinator-json-kinds：核阅引擎增收 JSON 记录规则类

> task-packages 治理任务
> 承接：SPEC-005 确定性外壳节、/tmp/ask3-shell-test 实测结论即规则包单用五中一
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— 本任务包无偏离，标准 T6-D
> 日期：2026-08-22

## 一、问题陈述 {#problem}

- 实测证明核阅现有规则类全部行文字向，JSON 记录的四类核心校验不可表达：必填字段在场、嵌套数组逐项 schema、数值区间、跨字段算术。
- 三问确定性外壳的校验腿须落在核阅上即路径甲：引擎扩词汇，SPEC-005 落为规则包，知识仍全在包内。

## 二、关键设计（钉到字段级） {#design}

### 2.1 材料轴分支

manifest.toml 增可选键 material，取值 text 或 json，缺省 text。text 即现行行为逐字节不变。json 即目标先按 JSON 解析，解析失败产单条合成发现 rule_id 为 material_json_invalid、location 为 {"line": 1}，与规则无关不可配置。

### 2.2 新规则类五件

一 json_parse：无参数。材料非合法 JSON 即一发现。包可显式声明以获得可读消息，不声明时解析失败仍由材料轴合成发现兜底。

二 json_field：参数 path 即点分路径、type 即 str/int/float/number/bool/object/array、可选 enum 即字符串数组。逐段解析对象键，缺段或缺键或类型不符即一发现，location 为 {"path": 路径}。int 要 JSON 整数排除布尔，number 收整数或浮点，float 收浮点或整数值浮点写法，str 排除数字与布尔。enum 仅 str 时生效，值不在枚举即发现。

三 json_array_schema：参数 path、min_items、fields。fields 为字段名到类型的映射，类型值可递归即 {"type": "object", "fields": {...}}。数组每元素必须是对象且逐字段过类型，元素违规每元素一发现，location 为 {"path": "路径[下标].字段"}。数组缺场或非数组或长度小于 min_items 各一发现。

四 json_number_range：参数 path、min、max 闭区间。字段非 number 或越界即一发现。

五 json_field_compare：参数 left 即点分路径、op 即小于等于、小于、大于等于、大于、等于五者、right 即点分路径或数字字面量。两侧解析为数，按 op 比较，不成立即一发现，任一侧缺失或非数即一发现。

### 2.3 ask3 规则包

packs/ask3/ 即 manifest 含 material 等于 json 加 rules 按 SPEC-005：记录八字段必填带类型、意图契约子字段齐备、认知域两深度整型、锚点数组 min_items 一且九字段含 philosophy_ref 递归 source 加 quote 两字符串、inquiry_stage 枚举 first second third、domain_tag 枚举 target support、confidence 数值区间 0.0 至 1.0、支撑深度小于等于目标深度、calls_out 大于等于 calls_in、round 大于等于 1。子字段以 SPEC-005 原文为准，X2 自行核对落包。

### 2.4 语料

fixtures 造合法与带病两份记录。带病至少五处覆盖：锚点缺 philosophy_ref、confidence 越界、支撑深度超目标、calls_out 小于 calls_in、必填字段缺失。expectations 映射每处发现到规则 id。

## 三、工作清单 {#work}

### Cluster 1（双子代理并行）

- [ ] X1 实现件：manifest 材料轴解析、JSON 求值分支、五规则类、单元测试
- [ ] X2 数据件：packs/ask3 两 TOML、fixtures 语料与 expectations、README 补一节

### Cluster 2（主线串行验证）

- [ ] CONTRACT 修订二主线亲写
- [ ] F 锚定集成测试主线亲写
- [ ] 实测复跑即 /tmp 带病记录经 ask3 包五处全捕
- [ ] 兄弟回归与全测
- [ ] ask3 包跑语料报告上链、契约过检词
- [ ] 结果文件与双仓 commit

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F1 五处全捕 | 数据治理 | 带病记录经 ask3 包发现数大于等于五且覆盖四新类各至少一 |
| F2 合法零误报 | 数据治理 | 合法记录零发现退出码 0 |
| F3 旧件零伤 | 跨族治理 | 既有六测零改全绿，des-001 包对现行语料输出与扩展前逐字节一致 |
| F4 空腹不破 | 元层工具 | 引擎源码扫描零三问知识即无 ask3 与 session_id 与 anchors 字面量，知识只在包 |
| F5 契约在册 | 数据治理 | CONTRACT 修订二含材料轴与五类词表 |
| F6 上链 | 跨族治理 | ask3 包语料报告经书简认证入链链校验全绿 |
| F7 回归 | 跨族治理 | 五工具全测全绿即核阅化格书简检词路择 |
| F8 过检词 | 元层工具 | 修订后 CONTRACT 与 README 零违例 |

## 五、必读文件 {#read}

- sih-engine/doc/spec/SPEC-005-ask3-interface.md 即 ask3 包 schema 之源
- sih-tools/scrutinator/CONTRACT.md 与 src/scrutinator/engine.py 与 packs.py 即扩展对象
- sih-tools/scrutinator/packs/des-001/ 即包形态参照
- /tmp/ask3-shell-test/ 即实测结论现场

## 六、约束 {#constraints}

1. 向后兼容最高优先级：text 材料路径逐字节不变，既有测试零改。
2. 引擎零领域知识：新类是通用 JSON 校验动作，不得内嵌三问或 SPEC-005 任何具体字段名。
3. 仅标准库，json 模块入白名单。
4. X1 只碰 src 与 tests 新文件，X2 只碰 packs/ask3 与 fixtures 与 README，互不越界，CONTRACT 归主线。
5. 发现的 location 对 JSON 类用 {"path": ...}，text 类维持 {"line": ...} 不变。

## 七、验收标准 {#acceptance}

- [ ] F1 至 F8 逐条过并列表于结果文件
- [ ] 全测与兄弟回归绿
- [ ] 链认证与检词零违例
- [ ] 双仓 commit

## 八、风险点 {#risks}

- 类型判定边角即布尔是不是整数、浮点写法：2.2 已钉死，X1 照办，主线集成测试兜底。
- 双子代理对递归 fields 结构各自理解：2.2 三钉死递归形态，X2 落包前与 X1 实现冒烟对表由主线集成承载。

## 九、范式偏离声明 {#deviation}

无偏离。标准 T6-D：X1 与 X2 并行，主线亲写契约修订与集成验收。

## 十、关联文件 {#related}

- 任务包源：SPEC-005、/tmp 实测、核阅 CONTRACT
- 谱系：task-packages/README.md、skills/sihankor-t6d/SKILL.md
