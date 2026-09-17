# wengumcp-parallel 批结果档：M5 修复——温故投影 MCP 只读面与新城域自身档案索引

> 会话：cbdff9259c1aa6b0（首试即开，他会话 28d1c5c3 恰收约让位）；意图 e4bbe649
> 令源：用户 2026-09-11 令「同意。开始M5的修复，你拉子代理」
> 形：**并联 parallel 首个租约下活体**——并行簇 A 子代理领 mcpline 投影，主线簇 B 亲写温故 Rust 本体，主线串行验收

## 一、两件落地实态

件一温故投影 MCP 只读面：retriever_recall 工具 stdio 与 HTTP 两面（四轴编排，恒传 --root，NDJSON 与退出码三值透传，out 与 miss-log 零投影，register 式零写盘），面计数 stdio 19 升 20、HTTP 17 升 18 即 alpha 相八读数；SPEC-023 修订二、矩阵只读行、README、AGENTS.md MCP 节计数同步；mcpline 0.9.0。

件二温故新城域索引：retriever --root 显式数据根旗标（bin 层严判两形根拒面 exit 2）加 canonical 布局判别（layout_form 与 derive_root 同构判据）加分形链目录与分形判档（城档映射：任务包归意图档、结果档归结论档、泊界材料归悬置档、事件档属现表共用、经验档城零命中如实）加 locator 码根祖先回溯加内嵌 canonical 记忆包（临时文件承载零城侧配置零工作区落盘）；first_domain 形逐字节零变；SPEC-007 v1.3。

## 二、簇交付与读数

- 簇 A（子代理，93 分钟 93 工具调用）：A1..A7 俱交付，test_wengu_tools 10 测新件，面计数三处断言绿；先红留痕 tdd-red-clusterA.log（ImportError 首跑）；15 红俱环境干扰有 HEAD 基线对照证 clusterA-env-control.log（在飞租约锁撞写门测试与例行读数日态），放锁窗复跑验。
- 簇 B（主线）：Rust 四文件加测试套件 retriever_canonical_suite 4 测（T1 布局判别加 T2 城档索引三档加 T3 城链事件轴加 T4 降级承袭），既有 mem_recall_f_suite 9 测零回归；先红留痕 tdd-clusterB-run1.log（夹具事件缺必填八字段）与 run2（F-8 降级命名语义被布局严判破坏，改宽判回落承袭）；bin 实弹三形烟测全绿。

## 三、主线串行验收（F 锚定）

- F-1 根：T1 绿（canonical 判别加 derive 自子目录加中央 first_domain 零回归）加 bin 拒面 exit 2 实弹
- F-2 索引：T2 绿（word 轴三档命中）加 T3 绿（事件轴事实档）
- F-3 投影：簇 A stub 透传三值与 argv 断言绿
- F-4 面计数：stdio 20 与 HTTP 18 断言绿；AGENTS.md 与 SPEC-023 修订二在档
- F-5 e2e：MCP core（簇 A 码）对新二进制（簇 B 码）打 canonical 夹具，3 行 conclusion 加 intent 加 parked 命中，e2e-mcp-core.json 在 materials
- F-6 回归：工地内两 Rust 套件绿；主树复跑批尾回填
- F-7 文档：两 SPEC 修订走管线三步；新词两笔 query established

## 四、管线与谱系申报

- SPEC-007 v1.3 管线三步零违例；SPEC-023 修订二首跑核阅 1 红十项（两枚希腊 alpha 字符超字符集闸加八处全角括号越权内容，含修订一存量两处），逐项精修后三步全绿——alpha 字符转写纪律对工具侧文件不适用但对 des-001 域文档在役，子代理产出经主线验收补正即并联形收敛位实证
- 新词登记两笔：温故投影 wengumcp 与温故检索具 retriever_recall 俱 established，manifest 0.12.0 升 0.13.0，化格零改证实规范形
- 并联形谱系注记终结：本批即并联形租约下首个活体，簇间以文件面零交集对冲（A 簇 mcpline 与 SPEC-023，B 簇 retriever 四文件与 SPEC-007），收敛验收归主线，违约零笔

## 五、结算

管线与簇验收认证 b522a9ad；双仓 settle seq1 tools 62522df5 与 engine bc583c0，seq2 engine d14db68（terms 备份件）；收约 revoked 双仓归并；reconcile 双零（双仓 unrouted 0 cert_missing 0）；当日链 verify valid；terms.json 主树回补直改笔 f3ef256d（并行窗四词超集验真 239 词包载入零损）。

主树真跑：cargo build 重编后新二进制烟测绿（域感知词即 canonical 检索命中 domaware-solo 结果档）；retriever_canonical_suite 4 passed 与 mem_recall_f_suite 9 passed 主树双绿；mcpline 全测 112 passed 加 15 failed——十五红俱环境干扰非回归：十四件 write_gate 撞他会话 mcpmanual-solo 再开后现持 sih-engine/doc/design 锁面、一件 heartbeat 属例行读数日态，与簇 A 的 HEAD 基线对照证同形（clusterA-env-control.log），README 既载绿读数以放锁窗复跑为准；本批自产 test_wengu_tools 十测与全部面计数投影断言主树绿。
