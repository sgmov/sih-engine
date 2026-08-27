# locator-t6d：寻址工具立项批即立名登记与接口契约先行

> T6D-XX task-packages 治理任务
> 承接：sess-zcode-260827-locator 三问意图即 2026-08-27 trail 意图事件、2026-08-27 立名会用户裁定寻址、DEC-007 基础设施五索引槽位
> 范式：T6-D 范式（双子代理并行 + 主线串行验证）—— **本任务包偏离：立文类，契约先行单线立文不派双子代理，主线串行执行**
> 日期：2026-08-27

## 一、问题陈述 {#problem}

- 治理 agent 定位材料靠裸 grep 与 find 即注意力浪费在机械查找上，且无稳定标识寻址，双向绑定与孤儿校验无承载。
- 基础设施五「索引」槽位自 DEC-007 立位以来零落地，工具名未立。
- 本批是立项批即名已立契约先行，实现代码归后续批，本批不产 src。

## 二、关键设计 {#design}

### 2.1 立名登记

中文名寻址英文正名 locator，2026-08-27 立名会用户裁定，弃索引即反候选死因为泛名词噪声重，弃图谱即反候选死因为外部生态强碰撞。槽位名索引照旧不动，槽位与工具两实体两名，承 lease 对 worktree 加 2locks、书简对持久化的先例。词条入检词 terms.json 即 established，连带改写面即 AGENTS 文件索引加一行。

### 2.2 契约先行

接口契约落 sih-tools/locator/CONTRACT.md，五节即调用形态、机器形态、语义边界、验收判据、融回门。机器形态承本会话四轮裁定：多载体解析矩阵、stable id 承 DES-009 派生规则、ndjson 条目流为真契约、存储 trait 六操作五槽位 v1 仅文件后端、域声明包式、退出码三值、零 LLM、双跑逐字节一致。

### 2.3 边界条款即两张期票

精确跨文件解析与语义向量召回写入契约语义边界节为期票非现货，各带进场条件与人节点裁定，防日后冒充完备。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 检词登记寻址词条入 terms.json
- [ ] 契约落 sih-tools/locator/CONTRACT.md
- [ ] 任务包与结果档落位

### Cluster 2：主线串行验证

- [ ] 检词核查零违例
- [ ] 管线三件零发现
- [ ] 双仓免参对表退出码零

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 词条登记 | 数据治理 | 检词册含寻址 established 条目且 check 对契约与包档零违例 |
| **F-2** 契约落盘 | 元层工具 | CONTRACT.md 五节齐备且化格核阅零发现 |
| **F-3** 立项不越界 | 跨族治理 | sih-tools/locator 下无实现代码即仅契约与登记，双仓对表退出码零 |

## 五、必读文件 {#read}

- 必读 1：DEC-007 子决策二即基础设施五索引槽位定义
- 必读 2：sih-tools/cascade/CONTRACT.md 即契约文体先例
- 必读 3：DES-009 即 stable id 派生规则

## 六、约束 {#constraints}

1. **零 LLM 调用**于工具执行层即本批纯立文与登记
2. **机械判据**即三 F 锚定全过方结算
3. **不扩大 scope** 即不动 DEC-007 与 GOV-003 不动 pk-017 不预写后端与槽位实现
4. **期票显式**即两张期票入契约边界条款不省略

## 七、验收标准 {#acceptance}

本任务包验收 = 3 项：

- [ ] F-1 至 F-3 全过
- [ ] 结果档落 sih/event/plan/locator-t6d-results.md，管线三件认证入链，双仓段结算收约
- [ ] AGENTS 文件索引登记寻址工具行

## 八、风险点 {#risks}

- 契约接口六操作若在实现批暴露不足，走契约修订不走接口静默膨胀。
- 词条登记若触发死词或懒波误报，改写词条不改裁定。

## 九、范式偏离声明 {#deviation}

本任务包**偏离 T6-D 范式**的「双子代理实施」流程：

- 理由：立文类是单线确定性立文
- 偏离：双子代理改为主线单线
- 保留：T6-D 命名约定 + F 锚定 + 跨仓同步 + 管线认证

## 十、关联文件 {#related}

- 任务包源：DEC-007 与 2026-08-27 立名会即本会话
- 工具：`sih-tools/locator/`
- 跨仓引用：`sih-tools/nomenclator/packs/core/terms.json`

## 十一、请求写入 {#requested-writes}

- `sih-tools/locator/`
- `sih-tools/nomenclator/`
- `sih-tools/scribe/`
- `sih-tools/lease/`
- `sih-engine/sih/state/plan/locator-t6d.md`
- `sih-engine/sih/event/plan/locator-t6d-results.md`
