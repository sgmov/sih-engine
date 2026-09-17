# domaware-solo 批结果档：M2 域感知四件

> 会话：d4da9313f4a8906f（重开形；首会话 20fc87add089a4d7 因 open 漏传 --allow 拆户让位，拆户认证 9f1a9ec0 在档）
> 令源：用户 2026-09-10 令「同意M2实装」；工作台 P0/P0.5 同令划出司梦域；M5 候裁不在批
> 形：solo 单线；工地施工主树零直写

## 一、四件落地实态

件一 critsweep 布局感知：detect_layout 静态判别（first_domain 双仓标记或 canonical sih/ledger 面）加 domain_faces 三面路径表加 central_code_root 两根分离（码根中央数据根本域），出参增 root_form 字段，canonical 形泊界单线 domain 走中央 selector 路由。版本 1.0.0 升 1.1.0。

件二 lease rootanchor 域感知：is_canonical_domain 与 detect_domain_context 两函数加 discover_workspace_root 域界即停加 CLI 台账缺省域解析（canonical 域落 <域>/sih/ledger，中央形 tool_dir 现形零变）。城内裸调跨域污染（实证 db073c5c16e82a5b）根治。版本 1.42.0 升 1.43.0。

件三 收约回执 gates_skipped：collect_gates_skipped 纯归集（无主 bypass 与 CALL-LOG bypass 与差集认领三事实），入 close 回执与 revoked 链行 detail 双面，零新旁路。

件四 gauge 域件与跨域卫：--domain-root 一旗展开（数据面本域 sih/ 形、码面中央码根、显式参恒优先）加 _guard_same_domain 跨域卫 fail-closed exit 2（read 与 record 与 gqueue 三面）。版本 0.8.0 升 0.9.0。

## 二、测试读数（工地）

- critsweep：23 passed（test_domaware 5 新 + 既有 18 零回归）
- lease：test_domaware 10 passed；closefix 与 closegate 与 stemgate 邻域 41 passed
- gauge：48 passed 3 skipped（test_domaware 6 新 + 既有零回归，gqueue 金向量形复原）
- nomenclator：30 passed（登记后词表 canonical 形，formatter 零改证实）

先红留痕：tdd-red-critsweep.log（5 红 root_form 缺）与 tdd-red-lease.log（ImportError 三函数未实装）俱在 materials。

## 三、温故检索（批第 0 步）

六轴 recall 全 exit 0（域感知与根锚与新城与判据扫与秤星与无主闸），材料 recall-*.json 六件在 materials。命中锚：mcpauth-solo-results.md@12「每项目一域，链与锁册与台账与泊界住各项目自己的 sih 目录」即本批令源正典；多实例与坐标系类零直接命中如实记。

## 四、开闸失误申报（先红留痕）

首会话 20fc87add089a4d7 open 漏传 --allow 致会话 allow 空取锁全数 scope_violation；任务包请求写入节裸路径行不属 parse_requested_writes 读取形（上批 mcpnomgate 同形但以显式 --allow 开立，本批首开误承包派生预期）。处置：补意图与拆户认证两笔上链后 close 拆户，重开带二十条显式 allow。意图复用经 --allow-reintent 1 主会处置（重意图闸既裁通道）。

## 五、文档与登记

- lease CONTRACT 修订五十九、critsweep CONTRACT 修订三（管线读数见认证报告）
- 新词登记：域感知／DomainAwareness／domaware established 入 core 包（manifest 0.11.0 升 0.12.0），register 双词 query 双 clean
- gates_skipped 工程字段名不登记（叩问处置在 ask3 件）

## 六、结算

管线与测试认证 ad8e91b8；双仓 settle seq1 tools 65f8875b 与 engine f6f3845，seq2 engine e408e90（terms 备份件）；收约 revoked 双仓归并；reconcile 双零（tools unrouted 0 cert_missing 0，engine 同）；链 verify valid。收约自身由收约前主树码执行故回执 gates_skipped 缺席属自举限界（mcpnomgate 同款），链面证明承本批 test_close_receipt_carries_gates_skipped 与后继批收约实跑。

主树真跑：critsweep 23 passed 实扫 root_form=first_domain 零降级 v1.1.0；gauge 48 passed 3 skipped；nomenclator 30 passed；lease 正典 uv 形 339 passed 1 failed——失败件 test_path_inclusion_conflict 即环境红：他会话 28d1c5c35a945234（mcpmanual-solo 批）现持 sih-engine/doc 锁面，无 --locks 形预检读中央册真撞（前置提交 65f8875b~1 隔离复跑 passed 在案，env-red-path-inclusion.log 在 materials）；非回归，他会话收约后自愈。sweepjson 金向量刷新（tool/version 1.42.0 升 1.43.0 单字段正当漂移在档）。

收约后补笔两笔：terms.json 并集回补并行窗四词（直改笔 d9ea85d1，写时序先于他会话 16:21:22Z 取锁，超集验真 237 词包载入零损）与本结算节回填（直改笔第二笔）。

## 七、M5 与工作台边界（用户令在档）

M5（温故 retriever 投影 MCP 面加新城域五档索引）候裁未实装未过得一：按 DEC-021 分派表工程实装非裁决类，立项时循 mcpline 线先例走批立与测量裁决；若实装中发现数学核须先走数学载体管线。工作台 P0/P0.5 批属司梦域不归本工作区处置。
