# mcpinit-solo 批任务包：域自举实装——mcpline init 开域命令与 lease 面表补正

> 令源：用户 2026-09-09 裁定「SiInfer 报 token 域根指向空域（InferServer 仓零 sih 树），我们应该要做好新项目接入的初始化工作，比如开 sih 文件夹，落资产包和模板」；形承 DES-015 项目域识别形节「域自举程序形」段（新城立域四步已定形，自举的实装细节归实装批，本批即该实装批）
> 形：solo 批独立立约独立收约，与主窗并行共用今日链走租约排队
> version: v1

## 一、使命

按 DES-015「域自举程序形」实装 `mcpline init` 域自举命令，终态：新项目接入治理域 = 管理台签发标识牌（已实装）加一条 `mcpline init <域根>` 命令即开域落地（正典 sih 树加域声明卡加资产包与模板落位加经中央 scribe 落开域首笔加开域毕即验域），读面即接即用，开域幂等可防呆；并补正 lease 任务包搜索面缺新城正典面一事（DES-015 域目录布局规约「sih/state/ 住 plan 任务包区」的实施缺口）。

生产开域（/Users/moc/workspaces/SiInfer/InferServer 实际执行 init）归主窗批毕后执行，本批零触碰 SiInfer 工作区。

## 二、实装六件

**件一 mcpline init 域自举命令（sih-tools/mcpline/src/mcpline/init.py，python -m mcpline.init 形）**

- 调用形（自 sih-tools/mcpline 目录）：`uv run python -m mcpline.init <域根> [--by <事由>] [--date <YYYY-MM-DD>]`
- 前置检查六项（任一即拒，退出码一，stdout 出教学 JSON，报错即教学，零静默）：
  1. 域根存在且为目录（相对形先 resolve 再判）
  2. 域根下 .git 在位（域根须为 git 仓——lease git 机制要求，防呆位即 SiInfer 彩排发现三「lease 需 git 仓」的防位）
  3. 中央登记册（经 tokens_mod.load_last_rows 与 default_layout(resolve_root()).tokens_path()，即第一域位 sih-tools/mcpline/ledger/tokens.ndjson 的域布局投影）有 active 行且该行 domain_root 与域根 resolve 后同路（normpath 归一比对）；取该行 token_id 为域声明 domain_id、scope 为 token_scope；无行或末行 stopped 即拒并教学（先经管理台 /tokens 签发）
  4. 域根 resolve 后不等于中央根 resolve 后（第一域映射形守卫——第一域是历史映射形不开域）
  5. 域根下 sih/domain.json 缺席（幂等守卫——已开域即拒并教学，无 reinit 旗标，重开域候后继裁定）
  6. 半态拒：sih/ 目录存在且非空但 domain.json 缺席（半态位人节点处置，拒并教学）
- 落地五步（序固定，开域首笔前全部文件面落地）：
  1. 建目录四件：sih/event/trail 加 sih/ledger 加 sih/state/plan 加 sih/state/parking/materials
  2. 写 sih/domain.json 域声明卡（单 JSON 对象，字段固定七件）：domain_id（标识牌 token_id）、domain_root（resolve 后绝对形）、token_scope（登记册行 scope）、layout_form（固定字符串 canonical）、opened_at（本地 ISO 秒精度）、opened_by（--by 参，缺省 main-window-init）、mcpline_version（读包版本单点位）
  3. 落模板两件：sih/state/plan/TEMPLATE-任务包.md 即中央 lease 模板字节复制（源 code_root()/sih-tools/lease/TASK-PACKAGE-TEMPLATE.md，单一源零分叉，源缺席即退出码二）；sih/state/parking/TEMPLATE-停泊材料.json 即模块内置停泊材料骨架（字段形对表 sih-engine/sih/state/parking/materials/pk-070-exit.json：action 与 entry_id 与 context 与 ruling 与 state 与 parking 子对象即 entered_at 加 ttl_days，值全为占位文本）
  4. 写 sih/README.md 域自述（生成文本）：域名即标识牌、域根、树图四行（event/trail 逐日链、ledger 台账册族、state/plan 任务包区与模板、state/parking/materials 泊界材料）、用法三条（MCP 连接头携 Authorization: Bearer <标识牌> 读本域；新批任务包写 sih/state/plan 照模板形；泊界材料写 sih/state/parking/materials）、正典指针三条（DES-015 与 DES-014 与 SPEC-023 的工作区根相对路径）
  5. 开域首笔：report JSON 写 tempfile（域外暂存，用完删除），字段：batch 固定 domain-open、domain_id、domain_root、at（实日或 --date）、what（域开域初始化：正典 sih 树落地与资产包与模板落位）、opened_by、domain_declaration_sha256（domain.json 字节 sha256）、templates（两模板域根相对路径列表）；经中央 scribe 主树二进制落笔，argv 形：append --report <暂存> --exit-code 0 --trail <域根>/sih/event/trail/<date>.ndjson --no-session-reason 域自举无租约会话主会处置位；date 缺省实日
- 开域毕即验域：scribe verify --trail <同链文件> 退出码零且判词 valid；开域笔 event_hash 提取（scribe append stdout 可解析 JSON 即取其哈希字段，不可解析即读链文件末行 event_hash，零猜）
- 出参：stdout 严格 JSON 单对象 {ok: true, domain_id, domain_root, opened_at, files: [落地文件域根相对路径列表], open_pen_hash, chain_verify: "valid", next: 教学语即读面经标识牌绑定本域可用}
- 退出码语义：零成功；一前置拒或开域毕验红（教学 JSON 出 stdout，验红时链文件留笔不删如实）；二工具自身异常（scribe 二进制缺席、模板源缺席、登记册行形损坏透传等）
- 模块 docstring 申报：init 面是操作位（主窗或人节点执行），与 writeface 零直写盘红线分面——writeface 依旧只 CLI 透传零文件写盘；init 落盘面即落地五步加 tempfile report 两件，开域首笔经中央 scribe 落（DES-015 域自举程序形第二步「经中央 scribe 落首笔域登记链事件」）
- 版本：pyproject 0.4.0 升 0.5.0，description 加域自举 init 一句；包版本单点位即 mcpline/__init__.py 的 __version__（现缺席则新立，与 pyproject 同步；init 读此点不读 pyproject）
- 实装前对表一件：读在档既有 report 文件与其对应链笔对表 report 字段到链笔 details 的映射形（sih-engine/sih/event/plan/*/materials/ 下 report 件与当日链笔俱在档），零猜字段

**件二 lease 任务包搜索面补正**

- TASK_PACKAGE_DIRS 追加新城正典面 sih/state/plan（sih-tools/lease/src/lease/core.py，第四面）；注释更新：面表两形即第一域历史三面加新城正典一面，补正令源即 DES-015 域目录布局规约「sih/state/ 住 plan 任务包区」实施缺口加本批域自举模板落位面
- 第一域零效应论证入注释：司衡工作区根下 sih/state/plan 缺席故零新命中、零歧义风险（由测试件三 T-lease-2 可证）
- CONTRACT.md 文末追加修订一条：编号取现行末号加一（动笔前读 CONTRACT.md 尾段确认实际末号，现行在档引用见到的是修订四十六，以实读为准）；内容即面表扩面登记加两形布局论证加第一域零效应申报
- 版本：pyproject 1.39.0 升 1.40.0

**件三 测试（mcpline 新 tests/test_init_domain.py 加 lease 面表测试）**

mcpline 侧（fixture 根隔离：SIH_ROOT 指 fixture 数据根、SIH_MCPLINE_CODE_ROOT 指真工作区根，照 tests/fixture_root.py 与 conftest.py 既立模式；fixture 域根先 git init；登记册行经 tokens_mod.append_row 落 fixture 中央位）：

- T-1 全流程：fixture 域根加 active 登记行 → init 退出码零；断言目录四件与文件五件（domain.json 与 README.md 与两模板与链文件）俱在位；domain.json 七字段形；任务包模板与中央 lease 模板字节相等；停泊模板 JSON 可解析且字段集在档；README 含 token_id；链文件恰一笔且 scribe verify 退出码零 valid；init 出参严格 JSON 单对象且 ok 真且 open_pen_hash 为六十四位 hex；同域再跑即退出码一且教学（幂等）
- T-2 登记册缺席（fixture 中央登记册文件不存在）→ 退出码一教学，fixture 根零新文件落盘
- T-3 登记行末行 stopped → 退出码一教学
- T-4 域根等于中央根（fixture 根本身）→ 退出码一（第一域形守卫）
- T-5 半态（fixture 域根 sih/ 下有散文件无 domain.json）→ 退出码一教学
- T-6 非 git 仓（未 init 的 fixture 临时目录）→ 退出码一教学
- T-7 域根尾斜杠或归一形输入 → 与登记行 normpath 同路即命中（实现按 resolve 加 normpath 归一，测试锁定确定性行为）
- T-8 开域毕验红路径（monkeypatch scribe verify 返非零）→ 退出码一教学，链文件留一笔不删

lease 侧：

- T-lease-1 resolve_package 裸 stem 命中新城正典面（fixture 域根下 sih/state/plan/<stem>.md 在位即唯一命中）
- T-lease-2 第一域形零新命中且歧义语义不变（fixture 根下第一域面与正典面同 stem 俱在位即歧义拒列全命中；仅第一域面在位即唯一命中照旧）

- 零回归：mcpline 既有全套与 lease 既有全套俱绿

**件四 DES-015 修订三（sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md 修订记录节追加）**

- 内容六点：mcpinit-solo 批把域自举程序形实装为 mcpline init 域自举命令（正典树落地加域声明卡幂等守卫位加资产包与模板落位加开域首笔经中央 scribe 无会话主会处置位加开域毕即验域以链 verify valid 为开域完成判据）；首标识牌登记面即中央登记册（凭证槽位不变律与解析单点律不变），程序形第三步「管理台签发首标识牌入域册」措辞按实装对表为「该域首标识牌行在中央登记册」；域映射登记面即标识牌行 domain_root 字段自身（域布局按行寻径），零额外登记步；任务包搜索面经 lease CONTRACT 修订补正新城正典面（域布局规约实施缺口填补，第一域零效应申报在档）；开域命令面是操作位归主窗执行，管理台续载标识牌三动作，开域进视图位候后继裁定；生产首域开域（InferServer）归主窗执行不随批
- 令源用户原话入修订行：「我们应该要做好新项目接入的初始化工作，比如开sih文件夹，落资产包和模板」
- 本文走化格核阅检词管线；新造词（开域、开域首笔、域声明卡、资产包等）按检词三态裁量该登即登

**件五 结果档与认证**

- 结果档 sih-engine/sih/event/plan/mcpinit-solo-results.md 与 materials/ 按 BATCH-FACE 形
- 认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid

**件六 版本与登记**

- mcpline 0.5.0、lease 1.40.0 双升（上节各件内已载）
- 检词新词登记按裁量；资产回锚登记面不动（本批无资产盘点动作）

## 三、验收

- 测试全绿：mcpline 既有加新增、lease 既有加新增（全 CLI 真 scribe 二进制，fixture 域根内开域全流程）
- 幂等与六项前置拒俱有实录（教学 JSON 原样入结果档）
- 文档三步：DES-015 化格核阅检词俱绿（退出码零）
- 认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid
- 本批零触碰 /Users/moc/workspaces/SiInfer/**、中央登记册 tokens.ndjson（init 只读册不写册，测试用 fixture 册）

## 四、红线

1. DES-015（含域自举程序形段与域目录布局规约）唯一正典，缺口停批候裁禁自由发挥
2. writeface 与 httpface 与 tokens 与 runtime 与 domains 模块零改动；init 是新模块；包版本单点位 __init__.py 与 pyproject 同步
3. scribe 与 identity 与 gauge 与 critsweep 零改动；例外即 lease 面表一行加 CONTRACT 修订加版本升，走 CONTRACT 修订渠道属批级差异，修订行与结果档俱如实申报（mcpwrite 批「既有工具代码零改动」是彼批红线，本批 lease 改动依此申报）
4. 禁触：sih-visual/**、SPEC-023、.zcode 配置、/Users/moc/workspaces/SiInfer/**、中央登记册 tokens.ndjson
5. 与主窗并行：锁面含 sih-engine/sih/event/trail/2026-09-09.ndjson，冲突走 lease wait-turn 禁绕行禁 preempt
6. 每条命令立即取退出码，失败即停整链，禁管道掩码
7. scribe 二进制一律主树 sih-engine/target/debug/scribe；commit 须指向登记 worktree

## 五、写入面（allow 清单）

- sih-tools/mcpline/src/mcpline/init.py
- sih-tools/mcpline/src/mcpline/__init__.py
- sih-tools/mcpline/pyproject.toml
- sih-tools/mcpline/tests/test_init_domain.py
- sih-tools/lease/src/lease/core.py
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/pyproject.toml
- sih-tools/lease/tests/（面表测试新文件或既立文件追加，按实申报）
- sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md
- sih-engine/sih/state/plan/mcpinit-solo.md 与 mcpinit-solo-prompt.md
- sih-engine/sih/event/plan/mcpinit-solo-results.md 与 mcpinit-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/ 与 sih-tools/identity/reports/

## 六、完工回报形

批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、六件各自终值、测试读数（mcpline 套与 lease 套各总数与绿数）、lease 面表两形测试实录、DES-015 管线三步退出码、检词新词登记清单、红线差异申报（lease 面表改动）、管道掩码自违如实清单（有则列）。
