# mcpinit-solo 批结果档：域自举实装——mcpline init 开域命令与 lease 面表补正

> 批名：mcpinit-solo（solo 批独立立约独立收约，与主窗并行共用 2026-09-09 当日链）
> 会话号：0e9f2983b18c01f3（lease 1.39.0 签发，identity 报告 2026-09-09-mcpinit-solo-identity.json，anomalies 空）
> 令源：用户 2026-09-09 裁定「SiInfer 报 token 域根指向空域（InferServer 仓零 sih 树），我们应该要做好新项目接入的初始化工作，比如开sih文件夹，落资产包和模板」；形承 DES-015 项目域识别形节「域自举程序形」段（唯一设计正典）；任务包 sih-engine/sih/state/plan/mcpinit-solo.md
> 批性质：T6 实装批——sih-tools/mcpline 新模块加 sih-tools/lease 面表补正加 sih-engine 设计档修订三，mcpline 工具域外于 sih-engine 文档规范（mcpserv-solo 先例），本档归 sih/event/plan 域外 des-001 核阅域（exit-2 如实记入档）

## 一、实装六件终值

1. **mcpline init 域自举命令**：`mcpline/src/mcpline/init.py`（新模块，python -m mcpline.init 形，自 sih-tools/mcpline 目录跑）——前置检查六项序固定任一即拒退出码一出教学 JSON（域根存在且为目录、.git 在位、中央登记册有 active 行且 domain_root resolve 加 normpath 归一同路取其 token_id 与 scope、域根非中央根第一域形守卫、sih/domain.json 缺席幂等守卫、sih/ 非空半态拒），落地五步序固定（建目录四件 sih/event/trail 加 sih/ledger 加 sih/state/plan 加 sih/state/parking/materials、写 sih/domain.json 域声明卡七字段 domain_id 与 domain_root 与 token_scope 与 layout_form 固定 canonical 与 opened_at 本地 ISO 秒精度 与 opened_by 缺省 main-window-init 与 mcpline_version 读包版本单点位、落模板两件即任务包模板字节复制中央 lease 模板单一源零分叉加停泊材料骨架模块内置对表 pk-070-exit.json 字段形、写 sih/README.md 域自述即域名域根树图四行用法三条正典指针三条、开域首笔 report 域外 tempfile 暂存用完删经中央 scribe 主树二进制 append 加 --no-session-reason 域自举无租约会话主会处置位落 <域根>/sih/event/trail/<date>.ndjson），开域毕即验域 scribe verify 退出码零且 valid 为开域完成判据，开域笔 event_hash 取 append stdout JSON 哈希字段不可解析即读链末行零猜，出参 stdout 严格 JSON 单对象，退出码零成功一前置拒或验红（验红链留笔不删）二工具自身异常；模块 docstring 申报 init 面是操作位与 writeface 零直写盘红线分面；多 active 牌同域根拒教学即零猜位。版本 0.4.0 升 0.5.0，__init__.py __version__ 原 0.3.0 陈旧位归一 0.5.0 与 pyproject 单点位同步。
2. **lease 任务包搜索面补正**：`lease/src/lease/core.py` TASK_PACKAGE_DIRS 追加新城正典面 sih/state/plan 第四面，注释载明两形论证（第一域历史三面加新城正典一面）与第一域零效应论证（司衡根下 sih/state/plan 缺席零新命中零歧义，测试可证）；解析语义零变更（union 搜、唯一命中、歧义列全拒、零命中列全四面）；CONTRACT 修订五十六入档；版本 1.39.0 升 1.40.0 三源对齐。
3. **测试八件加两件**：`mcpline/tests/test_init_domain.py` 八件（T-1 全流程含落地五步在位与 report 字段对表与幂等再跑教学、T-2 登记册缺席零落盘、T-3 末行 stopped 拒、T-4 域根等于中央根拒、T-5 半态拒、T-6 非 git 仓拒、T-7 尾斜杠归一命中、T-8 验红链留笔）全走 fixture 域根隔离加真 scribe 二进制；`lease/tests/test_mcpinit_facet.py` 两件加两件（正典面裸 stem 命中、第一域形零新命中加零命中列全四面、两面同 stem 歧义拒列全命中、面表两形冻结形）；test_openhyg.py 面表冻结断言随扩面适配四元组；sweepjson 金向量重冻 1.40.0 形（唯一差异即版本位，五类残留判定零改）。
4. **DES-015 修订三**：`sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md` 修订记录节追加 2026-09-09 修订三，令源用户原话入档，六点即实装形 mcpline init 命令与首标识牌登记面即中央登记册（程序形第三步措辞对表为「该域首标识牌行在中央登记册」）与域映射登记面即标识牌行 domain_root 字段自身零额外登记步与任务包搜索面经 lease CONTRACT 修订五十六补正与开域命令面是操作位归主窗执行与生产首域开域归主窗不随批；管线三步俱绿（见第四节）。
5. **结果档与认证**：本档加 materials 四件（测试双日志、tdd-red-green.log、pipeline-report.json、readings-2026-09-09.json）按 BATCH-FACE 形；认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid（哈希见第六节回填与完工回报）。
6. **版本与登记**：mcpline 0.5.0 与 lease 1.40.0 双升（一、二件内已载）；检词新词四件（开域、开域首笔、域声明卡、资产包）懒波登记走收约后直改车道（allow 清单缺口申报见第三节第 6 条），登记清单见完工回报；资产回锚登记面不动（本批无资产盘点动作）。

## 二、验收测试读数（工地全套件，日志在 materials）

1. **mcpline 套**：100 passed in 80.20s，退出码零（既有 92 加新增 8 零回归；test_init_domain 八件全绿，开域全流程走真 scribe 二进制：目录四件文件五件在位、domain.json 七字段形、任务包模板与中央模板字节相等、停泊模板字段集在档、README 含标识牌、链恰一笔 verify valid、出参严格 JSON 且 open_pen_hash 六十四位 hex 且与链笔 event_hash 对表、report 八字段含 domain_declaration_sha256 与 templates 列表对表在档、同域再跑退出码一教学；T-2 至 T-8 五拒位俱实录，T-7 尾斜杠归一命中，T-8 验红链留一笔不删）。
2. **lease 套**：313 passed in 58.63s，退出码零（既有 309 加新增 4 零回归；test_mcpinit_facet 四测全绿即 lease 面表两形测试实录：正典面裸 stem 唯一命中、第一域形零新命中且零命中报文列全四面、两面同 stem 歧义拒列全命中、面表两形冻结形四元组）。
3. **锁面交互申报**：mcpline 既有 test_write_gates 族 14 测在批会话持锁形下首跑红（fixture 树无 AGENTS.md 标记，lease CLI 缺省回主树锁台账，fixture 请求面与批会话锁面冲突，冲突载荷 holder 即本批会话号对表在档）；临时放锁两笔后 100 绿，收工前复锁，非本批改动回归（tdd-red-green.log 第三节在档）。

## 三、边界与误差如实申报（应而不藏，零隐藏）

1. **lease 面表扩面即批级差异**：任务包红线三例外位即 lease 面表一行加注释、CONTRACT 修订五十六、版本 1.40.0、sweepjson 金向量重冻、test_openhyg 冻结断言适配——俱走 CONTRACT 修订渠道，修订行与本档俱如实申报（mcpwrite 批「既有工具代码零改动」是彼批红线，本批 lease 改动依任务包红线三申报）。三源对齐拆分申报：任务包 allow 清单含 lease/pyproject.toml 未含 lease/src/lease/__init__.py（版本单点位），settle 闸 staged_out_of_scope 原位拒（实跑在档），__init__.py 1.40.0 位归收约后直改车道承载（--no-verify 加 lease bypass 登记留痕），三源对齐经 settle 提交加直改提交两笔闭合，归并后主树三源一致。
2. **sweepjson 金向量重冻**：金向量内嵌 tool.version 位，版本升 1.40.0 即红（diffs 仅 .tool.version 一件对表在档），按 sweepjson 批「金向量随冻」既有程序重冻，唯一差异版本位，判定语义零改；重冻产物在 sih-tools 工地 lease/tests/frozen/sweepjson/golden-report.json。
3. **核阅首跑红证**：修订三首稿全角括号论证形 12 处 C006 违例（退出码一），按文档既有半角括号形重写后复跑绿（退出码零，findings 空）；红证未清洗，处置即格式归位零语义改动，本条申报。
4. **包版本单点位陈旧位归一**：mcpline __init__.py __version__ 原 0.3.0 与 pyproject 0.4.0 失同步（mcpbeta 批后未归一），本批按任务包「单点位与 pyproject 同步」立 0.5.0 双源一致。
5. **多 active 牌同域根零猜位**：任务包前置检查三件未列多 active 牌同域根形，实现取拒并教学（列候选 token_id，止一后重跑），零猜不硬选；该位无测试用例（T-1 至 T-8 覆盖形之外），申报候后继批补测或裁定。
6. **检词新词登记走收约后直改车道**：任务包 allow 清单未含 sih-tools/nomenclator/packs/core/（lazy.json 写入位），件六「检词新词按裁量登记」判定在包、写入面缺口——按 BATCH-FACE watch 处置协议直改车道承载（写时声明加 scribe direct 直改链笔加 git commit --no-verify 后 lease bypass 登记留痕），收约后执行，本条申报；登记件四件即开域、开域首笔、域声明卡、资产包（懒波形，source 指本批任务包件一与 DES-015 修订三与令源原话）。
7. **管道掩码自违如实清单**：其一核阅首跑误用 des-015 包名（应为 des-001，BATCH-FACE 坑位形）重跑即过，非掩码属用错包名申报；其二一次 formatter 调用以管道接 python 解析摘要掩退出码，复跑无管道取真退出码零归位；其三测试套件多跑以 | tail 取摘要行未独立取退出码，收工前全部以无管道重定向文件形复跑取真退出码零（mcpline 与 lease 两套）归位；其四 ask3 双门与 lease 全链俱逐命令即取退出码无掩码。
8. **开域首笔无会话位**：init 开域首笔走 --no-session-reason 主会处置位（无租约会话），闸三会话在册验显式放行形，与 lease 会话写路径分面；信封 session_id 位缺席即开域笔归因域自举位，教学与判词在 init docstring 申报。

## 四、管线与链证

- 管线三步（DES-015 修订三，工作区根相对形目标）：化格 formatter --pack packs/general-v1 --write 退出码零（零改）、核阅 scrutinator --pack des-001 退出码零（findings 零，首跑红证见第三节第 3 条）、检词 nomenclator check --pack packs/core 退出码零（findings 零）；读数在 materials/pipeline-report.json。
- 本档与 materials 归 sih/event/plan 域外 des-001 核阅域（exit-2 如实记入档，mcpserv-solo 先例同形）。
- 书单对表（§10.5 必跑步，specfix-solo 先例同形）：checkcite 三跑——第一跑红即唯一引用 ID 系工程规格正典指针（SPEC 族）非 sih-math 数学消费引用，书单空间边界形态（留痕 2026-09-09-mcpinit-solo-checkcite-firstrun-red.json）；第二跑自伤红即申报件自含 ID 字面串再拾取（留痕 2026-09-09-mcpinit-solo-checkcite-run2-selfinflicted-red.json）；第三跑数学引用面申报件零 ID 即 pass（cited 空，missing 空，2026-09-09-mcpinit-solo-checkcite.json）。本批零 sih-math 数学消费引用如实申报。
- 链面：当日 intent 笔 f06e58d0（event_id 8f0ff02a-c6d2-4de0-891b-e882ab416f59），认证笔与 settle 与直改笔哈希见第六节回填与完工回报。
- 双仓 settle：tools 工地（mcpline init.py 与 __init__.py 与 pyproject 与 test_init_domain.py 加 lease core.py 与 CONTRACT.md 与 pyproject 与 __init__.py 与 golden-report.json 与 test_openhyg.py 与 test_mcpinit_facet.py）与 engine 工地（DES-015 修订三加本档加 materials 四件），提交号见第六节回填。

## 五、完工判定

使命达成：新项目接入治理域等于管理台签发标识牌（已实装）加一条 mcpline init <域根> 命令即开域落地——正典 sih 树四目录、域声明卡七字段幂等守卫位、资产包与模板落位（任务包模板字节复制单一源加停泊骨架加域自述）、开域首笔经中央 scribe 无会话主会处置位落域链、开域毕即验域以链 verify valid 为开域完成判据；读面即接即用（Bearer 标识牌绑定本域），开域幂等可防呆（六项前置拒俱教学 JSON）；lease 裸 stem 解析命中新城正典面且第一域零效应测试可证。测试 mcpline 100 绿与 lease 313 绿俱在档，DES-015 修订三管线三步俱绿，红线零违（DES-015 唯一正典缺口停批候裁、writeface 与 httpface 与 tokens 与 runtime 与 domains 零改动、scribe 与 identity 与 gauge 与 critsweep 零改动、SiInfer 与中央登记册与 sih-visual 与 SPEC-023 与 .zcode 与 8765 守护进程零触碰、与主窗并行锁面冲突走放让与复锁零绕行零 preempt、逐命令退出码即取即断、scribe 二进制一律主树）。生产首域开域（InferServer）归主窗批毕后执行，本批零触碰 SiInfer 工作区。

## 六、双仓 settle 提交号（认证后回填）

- tools 段1：569ef093（branch msh/mcpinit-solo，base integral-stage-build@9331420e，settle 凭据 cert cae61412 与 session 0e9f2983b18c01f3 与 note 申报）
- engine 段1：cad8b8f（branch msh/mcpinit-solo，base main@d6204c1，settle 凭据同上）
- engine 段2（本回填提交）：提交号见完工回报与链上 settle 笔
- 认证笔：cae61412008d58822e6b5caf530575df87a8232b5bacbb890e9ab362918ad348（event_id 0da5a01e-88f8-4e0f-8a5c-2827c6b34f97，session 0e9f2983b18c01f3）
- 直改车道笔（lease 版本位与检词登记，收约后 scribe direct 落链）：哈希见完工回报
