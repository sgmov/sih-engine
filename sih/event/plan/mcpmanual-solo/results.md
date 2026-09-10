# mcpmanual-solo 批结果档：MCP 调用 AI 使用说明书

> 令源：用户 2026-09-10 令「司梦彩排阶段，对于司衡mcp的使用没有概念，我们缺失了一份针对mcp调用的ai使用说明书。」
> 形：solo 批；会话 28d1c5c35a945234；intent 笔 81d79fe1（2026-09-11 链）
> 候窗实录：批二 domaware-solo（父会话分叉后在飞）持锁至 00:12，本批候窗轮询 2 次出窗即开，零绕行零预检冲突

## 一、批件清单与读数

| 件 | 实录 |
|---|---|
| AI-MANUAL.md 正典件 | 九节成文：三十秒上手、接入与身份、工具目录（α 七加 β 十逐具表加 stdio 本地两具申报）、标准剧本五则、错误语义（载荷形加退出码三值加十五行理由码表）、命名纪律、域治理面速览、边界如实申报七条、正典指针 |
| /manual 路由 | web.py manual_file() 单点解析加 manual_page 只读 GET text/markdown，缺席 404 教学语；路由表 +1 零写通道 |
| 页脚链接 | _page 页脚加 /manual 链接（管理台与确认页全带） |
| 双实例 instructions | server.py 与 httpface.py 两 FastMCP instructions 增说明书两位指针加体检三具速览加五步写入链速览加被拒教学一句；工具描述冻结文本零改动 |
| 漂移守卫 | tests/test_ai_manual.py 十测：面十七具名逐一对表、反向鬼工具守卫、本地两具与 takeover/bypass 申报对表、理由码九码抽样、双实例 instructions 指针断言、路由 200 与缺席 404、页脚链接、版本双点位同步 |
| 词形登记 | mcpmanual（code）与 AI 使用说明书（zh）一词条 established 入 core 包；register 后双 query 读数 established；化格归一双件 exit 0；nomenclator 包测试 30 passed |
| 版本 | mcpline 0.8.0 升 0.9.0（__init__ 与 pyproject 与 README 同步）；README 增说明书节 |
| DES-015 修订六 | 说明书位四点登记（正典件、/manual 获取位、双实例 instructions、漂移守卫）；走管线三步 |

## 二、测试读数

- 漂移守卫先红：9 failed 1 passed（exit 1，tdd-red.log 在档，红证未清洗）
- 漂移守卫后绿：10 passed（exit 0，tdd-green.log 在档）；中途两红为守卫自证即节名标记缺席与反向守卫误纳理由码，修文与修测各一笔如实申报
- mcpline 全套件工地读数：112 passed 15 failed（suite-worktree.log 在档）；15 红全数归因两处，其一 test_write_gates.py 十四件即 README 记档在案的持锁窗真锁面互撞（本批自身持 trail 与 reports 面锁），放锁窗主树复跑为准；其二 test_heartbeat_happy 一件即跨批回归，见第三节
- nomenclator 包：30 passed；登记后 canonical 形绿

## 三、跨批回归如实申报（非本批病灶，候修复批）

test_heartbeat_happy 主树现红（本批开工前即在红，归因洁净）：domaware-solo 批 gauge read 域卫改线（_domain_marker 与 _guard_same_domain 与 _expand_domain）打红 mcpline heartbeat 单元测，days_since_last_snapshot 为 None。domaware F-5 只复跑 critsweep 与 lease 与 gauge 三族，mcpline 族未在其复跑面。gauge/cli.py 不在本批 allow 面，不越界代修；修复候批（gauge 侧读线或 mcpline 夹具适配二择一），候人节点裁处。

## 四、闸与对表实录

- stem 查册闸：mcpmanual unknown → --new-stem 认领，回执 disposition new_coinage_acknowledged，teaching 新词认领在案（闸既裁通道非偏离）
- 叩问：三信号（说明书、AI 使用说明书、mcpmanual）unregistered 轻，digest passed covered 3，处置即登记在册消解
- 三问双门：scrutinator packs/ask3 exit 0；ask3repeater status ok anchor_count 3（validation 件在档）
- 正身：anomalies 空
- 命名空间保留位：event/plan 面父目录声明按 facefit 冻结设计替换为 mcpmanual-solo/ 子路径保留位，results 与 materials 住保留位内，本档即保留位形首件

## 五、结算（待收约时回填）

- 认证笔：见链 2026-09-11 certification_completed 笔
- 双仓 settle 提交号：收约归并时载入
- reconcile：双仓 unrouted 0 cert_missing 0 目标
- 主树复跑：mcpline 族放锁窗复跑读数以完工报告载出

## 六、并支与改号实录（结算补记）

- 双度并主：tools 8ae08eb2 与 d31a17e3（wengumcp α 八具并面加 sim 词条直改笔 25d1bb42 加 bootstrap M4 面，CALL-LOG 投影腿自动并合）；engine e2d96ce（DES-015 撞号解投：修订五=M4 bootstrap-solo 先落主树，本批说明书位改号六并注明让位缘由，批件引用同步改号）
- 并支携带声明：merge 几何携带 main 已提交态的 treadmill CALL-LOG 与他批件，scope 闸拒于合并几何，bypass 四笔登记在案（tools 8ae08eb2 与 d31a17e3、engine e2d96ce 等三笔加 sim 代提交 25d1bb42）
- 并支后守卫 10 绿；说明书扩 retriever_recall 行随 α 八具实态（wengumcp 并面）
- 检词改号后 6 笔：5 笔修订三/四存量 lazy（继承债已在案）加 1 笔 M4 修订五段开域懒波词（并行批文本，归并带入，非本批著作，如实记档）
- 测试隔离缺陷申报：test_ai_manual 置于套件首时闸测夹具施工面预检渗读真实会话面（open_precheck_conflict 撞在飞批锁面，mcpnomgate 隔离病族新面目）；本批自排序垫尾自保（pytest_collection_modifyitems），根因修复归后继夹具密闭批
- domaware 跨批回归与零写证明两红归因同前节，主树复跑读数以完工报告载出
