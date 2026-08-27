# mem-impl-t6d 结果档：项目记忆组件实装批

> 状态：完成。SDD 先行即 SPEC-008 落差规格，TDD 先红后绿即 F-1 至 F-10 全过，引擎 src 落 memory 模块与 memgate 宿主命令，双仓段结算收约。

## 证据链 {#evidence}

- 三问意图：sess-zcode-260827-memimpl 即 2026-08-27 链事件 26f6f00d，双闸零发现即围堰 ask3 包 findings 空与引擎 ask3gate status ok
- 租约会话：首会话 42cb1db12f118b02 因相对根缺陷净拆即零提交零锁拆本吊销，重立会话 6fe80ab6f22947e2 绝对根即双仓副本与 11 路径锁
- SDD 先行：SPEC-008 于测试落盘前在档即 F-9 时间序成立，化格零改
- 红态留痕：库内 9 败 + 集成 9 败全为 todo 桩宏即失败测试先行，输出存 /tmp/memimpl-red-full.txt（8 件 memory 桩败 + t2 旧家路径耦合 1 件即既有缺陷候选非本批产物）
- 绿态留痕：库内 50 过 + 集成 9 过零编译零警告，t2 以 skip 承载即副本布局不可达旧家路径，新家四文件验链在收约段 scribegate verify 承载
- 窄域包实测：194 文件 12519 条目零解析错秒级
- CLI 烟测：memgate recall 于真实工作区跑通即主题轴出五档切面、拦面退出码一、停泊事件按 entry_id 命中、--out 落件

## 判据核对 {#criteria}

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 | 五档覆盖 | 过 | 真实材料组合三轴调用五档各至少一行 |
| F-2 | 出处机械回验 | 过 | md 行区间重开比对、json 标识重建直取、事件哈希在链三载体全覆盖 |
| F-3 | 主题轴跨档 | 过 | pk-024 跨结论、悬置、意图三档 |
| F-4 | 事件轴一致 | 过 | intent_refined 与 certification_completed 两路返回集与 query 位直查相等 |
| F-5 | 时间轴边界 | 过 | 真实链单日过滤全落当日，合成事件证日界首末纳秒含即含 |
| F-6 | 只报不判 | 过 | 键集恰七字段且值域小写枚举，无 score 无 suggestion 无 ranking |
| F-7 | 确定性 | 过 | 同参双跑 ndjson 逐字节相等 |
| F-8 | 退化不崩 | 过 | 缺 locator 与缺 trail 各报缺席件名退出码二，载体哨兵原样 |
| F-9 | SDD 先行 | 过 | SPEC-008 落档时间先于测试落盘 |
| F-10 | 红转绿留痕 | 过 | 红态与绿态输出均在本档证据链登记 |
| F-10b | 包档一致 | 过 | 窄域包 194 文件路径全可归档恰一档 |

## 交付清单 {#deliverables}

- sih-engine/src/memory 五文件即 mod 与 archives 与 axes 与 locator_bridge 与 facet，库面无 CLI 耦合
- sih-engine/src/bin/memgate.rs 即宿主命令 memgate 承 DEC-014 委任立名，子命令 recall
- sih-engine/tests/mem_recall_f_suite.rs 即九件集成测
- sih-engine/doc/spec/SPEC-008-project-memory-implementation-gap.md 即落差规格
- sih-tools/locator/packs/memory/pack.json 即五档窄域包数据件

## 偏离与修正记录 {#deviations}

- 偏离一：桩期 FacetRow 派生缺 Serialize 两处编译错即非合格红态，补派生后重跑红，红态以此为据。
- 修正一：集成测根定位初版 join 上溯少一层即落仓根，红态抓出后改 derive_root 库内上溯，测试与库共用单一定位逻辑。
- 修正二：let 链为 2024 版特性与本仓 edition 2021 不兼容，实现期改嵌套判断。
- 修正三：archive 与 axis 序列化初版出变体名即大写，CLI 烟测抓出契约值域应为小写，修 serde rename 并加固 F-6 值域断言即测试缺口同步补。
- 偏离二：t2 生产复验在副本布局不可达即旧家路径耦合既有缺陷，绿门以 skip 承载不静默，等价验链在收约段执行。
- 偏离三：ask3 validate 共享临时件并发竞态偶发一次失败，隔离三连过，缺陷候选登记不在本批修。
- 偏离四：open 传相对根使会话档副本路径记相对串，commit 别名解析与 close 拆本判定两处不可达，首会话 42cb1db12f118b02 零提交净拆，绝对根重立 6fe80ab6f22947e2，工件经暂存目录全量搬移无损。
- 修正四：清残目录时误删工地根下绑定侧档即 worktrees/.bindings 会话件，从身份件 observed 三态原样重建，unlock 五验全过，自毁自记。
- 偏离五：close 删支先于拆本失败复发三次即既有缺陷候选再添复发实证，副本清障还原净检出后手工拆本，第四次自愈吊销。

## 发现登记 {#findings}

- 缺陷候选：locator core 包全仓构建段错误退出码 139，复现即 root 指工作区根或 sih-engine 均崩、doc 烟测不复发，疑代码载体或大体量文件触发，定位修复归另批。
- 缺陷候选：tdd_tests t2 仍指旧家 sih-tools/scribe/trail 即修订十一迁家后未随迁，主检出布局可达副本不可达。
- 缺陷候选：ask3 validate 测试共享临时件截断写与并发读竞态。
- 缺陷候选：lease 相对根三处显形即 commit 别名解析先 resolve 后比对相对串恒不等、close 以相对串 exists() 判工地恒假跳过拆本、绑定侧档居所随相对根漂移，根治即 open 相对根拒收或入账即绝对化，归租约融回批清付。
- 待人复核：json 载体 ref 补钉即路径@条目稳定标识，是否构成 SPEC-007 破冻待人节点裁，判破冻即走修订记录。
- 性能观察：F-7 双跑 82 秒即 uv 并发争锁叠加每调用即时建索引，缓存复用属后续优化承 SPEC-008 取舍登记。
- 主本运行件：窄域包与三问件按先主本后镜像副本落位即运行期测试解析根用主本，收约归并后两位一致。

## 遗留与后续 {#follow-ups}

- 消费侧三处接线批另开待令即三问前向约束检索加载与批次起草先例检索与段结算结论档回取。
- 正式立名与词条登记待人节点即项目记忆为工作名、memgate 为宿主工作名。
- pk-024 即第六席与退出标准关系仍停泊待人裁。
