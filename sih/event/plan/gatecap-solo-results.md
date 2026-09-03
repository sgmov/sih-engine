# gatecap-solo 结果档

> 批：gatecap-solo（mathpipe-full 程序收尾批即闸门自维持件，判定性常数裸奔违规类 + tally 推导档机械复核）
> 会话：a1089d85f6aefe6c（前会话 fd6b6ebf07dd1d3a 因任务包请求写入段格式与 lease 包解析器错位收空重开，见误差申报一）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理，单飞零并发
> 承接：mathpipe-full-program-v1.md 收尾批节即闸门自维持——不变量从此由闸门维持不靠惯例；批四起六件收官后本批为程序封顶件
> 意图哈希：a186117459a449495c40d64f29d14d1bd0774f59f5011a55f38ca5691b498c54（ask3 记录内容哈希，双门 status ok 三锚，意图事件 56c7a80b）

## 一、完成度表

| 交付件 | 判据 | 结果 | 证据 |
|---|---|---|---|
| des-001 新违规类 | C 系码位续 + 纯数据入包 | 成 | C007 与 C008 入 rules.toml 包尾，零改判定引擎；包版本 0.1.0 升 0.2.0（承契约「规则语义增删改归规则包版本管理」） |
| fixtures 净脏双目标 | 入 corpus | 成 | corpus/carrier/constant-carried.md（净）与 constant-naked.md（脏，故意裸奔样文） |
| tally R8 推导档复核 | 先红后绿 | 成 | 红态 3 failed 2 passed，绿态 25 测全绿；rules_version des-011-r2 门控 |
| 存量对表 | 全量读数在档 | 成 | 135 面（engine doc 83 + corpus 50 + formatter fixtures 2）全量扫，既有文档零命中，三中全落脏夹具 |
| CONTRACT 修订与 USAGE 同步 | 两 CONTRACT 修订 | 成 | scrutinator 修订四加载体引用节、tally 修订八加载体引用节其三；USAGE 面 CLI 零变化即调用形不变以 CONTRACT 修订承载 |
| 金向量 | 净脏双场景双跑 | 成 | 三场景双跑逐字节一致，冻结于 materials/gatecap-solo-golden-vector.json |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| **F-1 新规拦裸奔** | 脏目标命中码位且净目标零发现，双跑一致 | 过 | 净夹具双实现 exit 0 零 findings；脏夹具双实现 exit 1 命中 C007 行 7、C007 行 9、C008 行 11；双跑逐字节一致加跨实现 findings 与 summary 全等 |
| **F-2 tally 复核** | 推导档缺指或所指不存在即挂起，先红后绿 | 过 | r2 材料缺指与悬指两测挂起、在场通过与 r1 回退同判与 r2 仍走 R1-R7 共五测；红态 3 败后绿态 25 测全绿；活体探针挂起双跑逐字节一致 |
| **F-3 存量如实** | 域内全量读数与命中清单在档 | 过 | materials/gatecap-solo-legacy-sweep.json：135 面全量扫既有文档 C007/C008 零命中，三中全落新脏夹具（设计内），零申报项如实记 |
| **F-4 兼容** | 核阅与 tally 既有测试全绿 | 过 | scrutinator 工具侧 32 测（29 既有加 3 新增）、引擎 cargo 156 测（金向量逐字节断言经重冻后全绿）、tally 25 测（20 既有加 5 新增） |

## 三、活体探针读数（主会验收位）

新规则两态自证，探针证据八件全落 materials/probe-*.json：

- **洁净语料**（constant-carried.md，双指针合规形）：引擎新二进制 exit 0 零 findings、工具侧 exit 0 零 findings，各双跑逐字节一致，跨实现 findings 与 summary 全等。
- **脏语料**（constant-naked.md，故意裸奔样文）：引擎新二进制 exit 1 命中 C007×2 加 C008×1（行 7、9、11）、工具侧 exit 1 同点位，各双跑逐字节一致，跨实现全等。
- **tally R8 活体探针**：r2 材料缺 derivation_path，处置挂起、R8 定位行在案、exit 0（挂起为映射语义非核对失败，承 R5 漂移挂起先例），双跑逐字节一致。

## 四、存量对表读数

新引擎二进制（内嵌 C007/C008）对 des-001 域全量 135 面扫描：全规则 findings 共 461 条全部为既有规则在违规样本夹具与 goldfix 教学夹具上的既有命中；C007/C008 命中 3 条全落本批新脏夹具 constant-naked.md（设计内命中），**既有文档零命中，存量申报清单为空**，读数档 materials/gatecap-solo-legacy-sweep.json。

## 五、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 56c7a80b | 意图笔（scribe intent，meter 包裹） | ask3 记录 a1861174 |
| b7db8c23 | 管线报告（2026-09-04-gatecap-solo-pipeline.json） | 报告件内容哈希 |
| d60b0264 | 推导档报告（2026-09-04-gatecap-solo-derivation.json） | 报告件内容哈希 |
| 78864507 | 金向量报告（2026-09-04-gatecap-solo-golden.json） | 报告件内容哈希 |
| d1d1e40c | 变更件报告（2026-09-04-gatecap-solo-changed-files.json） | 报告件内容哈希 |

认证一律先落主树活链，四笔全 meter 包裹、闸三 --sessions 带、追加态 append 短持锁即取即放。

## 六、冲突样本节（pk-045 样本库）

本批单飞，开工前 lease status 验零活跃会话零锁，十一把施工面 exclusive 长持锁一射取获重试计数 0，共享追加面（trail 与 meter/counts 与 reports）append 短持即取即得，**零撞锁如实记零**；无他批在途，零插队零让位。

## 七、越线与误差申报

1. **任务包请求写入段与 lease 包解析器格式错位**：任务包 §五请求写入为 ｜ 连排段，lease parse_requested_writes 仅解析 `- ` 列表项致取零，scope_source 落 explicit，首把锁 scope_violation 被拒（exit 一，闸在役实录非绕行）。处置按 scriwire2「收空租约补 allow 重开」先例：前会话 fd6b6ebf07dd1d3a 零锁零提交拆除让位（close revoked true 拆三工地），重开会话 a1089d85f6aefe6c 十七路 allow 显式传全。意图笔落前会话名下（同 ask3 记录，闸二拒重追加故不重挂），台账 issued 事件两笔可溯。
2. **引擎金向量八件重冻**：包版本 0.1.0 升 0.2.0 使内嵌版本串入金向量，重冻以 findings 逐条零漂移为判据（八件 findings 集重冻前后全等，仅版本串变），重冻后 cargo 156 测全绿；重冻由本会话亲跑新二进制机械重生成非手改。
3. **检词存量违例一笔**：scrutinator CONTRACT.md 行 81 dead_ban（死档名 Scrutiny，退役登记节历史引文），经主树 HEAD 同件对表为存量违例非本批引入，按「存量命中只申报不越权处置」红线保留原文，处置归人节点（泊界或勘误另批）。
4. **引擎工地 locus 测试五败定性**：工地内 cargo test 首跑 ga1 至 ga4 与 t6 五测败，定性为工地 target 缺 scribe 二进制（测试拉起 manifest/target/debug/scribe），全 bin 重建后 156 测全绿，非本批改动引入。
5. **零停批事件**：全程无不可解释的门与闸拒绝，无工具 exit 2 异常（formatter 对七件全零改，scrutinator 对五件域外 exit 2 如实记非异常，scope_violation 与闸系拒绝均按预期在役）。
6. **政策行在役**：机械链全绿即自行收口推进，结果档不设「等你令」节。

## 八、验收

- [x] F-1 至 F-4 全过
- [x] 存量命中清单在档（零命中如实记）
- [x] 单飞零并发如实记（冲突样本节零撞锁）
- [x] 认证入链，双仓（三仓）结算收约对表（见收约附表）
- [x] 任务包与 dispatch 两件随批在版控（主会 d034799 批前收编在案）

## 九、队形验证

单线形 solo 零子代理，全链由会话 a1089d85f6aefe6c 亲写，零 Agent/Task 派生。

## 十、收约附表（close 后回填）
