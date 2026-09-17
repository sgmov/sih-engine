# mcpboot-solo 批任务包：MCP 引入初始化固化——mcpline bootstrap／域自举全链命令与台面开域视图

> 令源：用户 2026-09-10 令「现在司梦引入mcp的初始化还是手工在做，需要固化成司衡引擎的一个固定工具。」；同日三裁定：另立新模块、客户端注册旗标选入、开域进视图一并做进本批；同日立名裁决：bootstrap／域自举（双名并行，典承 DES-015 域自举程序形判词，检词正登随批）
> 形：solo 批独立立约独立收约
> version: v1

## 一、使命

把新城引入 MCP 的手工残差固化为固定工具：既有 mcpline init 开域单步（DES-015 修订三正典形）零改动核心，新模块 mcpline.bootstrap 在其外层编排全链五段——前置检查、标识牌签发位、开域位、令牌双落位（镜像行）、客户端注册旗标选入——一令到底；并把开域进管理台视图（DES-015 修订三显式候裁项「开域进视图位候后继裁定」，本批承用户裁定落位），台面是薄壳调同一模块函数，单一 canonical 路径零二次实现。

## 二、关键设计

**件一 bootstrap 全链命令（sih-tools/mcpline/src/mcpline/bootstrap.py，python -m mcpline.bootstrap 形）**

- 调用形（自 sih-tools/mcpline 目录）：`uv run python -m mcpline.bootstrap <域根> --by <事由> [--token-id <词形>] [--scope domain_write] [--date <YYYY-MM-DD>] [--client-config <路径>]`
- 序固定五段：
  1. 前置检查：复用 init.precheck 域根面检查（存在、目录、.git、非中央根、幂等、半态）加登记册读数，零重实现
  2. 签发位：本域根无 active 行或末行 stopped → 以 --token-id（词形循 TOKEN_ID_RE）加 --scope（缺省 domain_write）构造 issue_row 经 tokens_mod.append_row 落中央登记册（issued_by 承 --by）；已有 active 行 → 复用不重签（出参 issued.done 假，注 reused）；须签发而缺 --token-id → 拒教学
  3. 开域位：调 init 既有 precheck 加 open_domain（落地五步加开域首笔经中央 scribe 加 verify valid 收口）
  4. 镜像位：验域 valid 后，中央 active 行逐字段恒等镜像写 <域根>/sih/ledger/tokens.ndjson（append_row 既有函数）——补全 DES-015 登记册形判词「落位两形」的正典实装缺口
  5. 客户端注册位：--client-config 给了才写（缺省零触碰 .zcode）；合并写：读既有 JSON（缺席即空对象），仅设 mcp.servers.sih = {type:http, url:http://127.0.0.1:8765/mcp, headers:{Authorization:Bearer <token_id>}, enabled:true, timeoutMs:60000}，其余键原样保留，indent=2 尾换行；幂等可重跑
- 出参：stdout 严格 JSON 单对象 {ok, domain_id, domain_root, opened_at, issued:{done,token_id,scope,note}, files, open_pen_hash, chain_verify, mirror_row_path, client_config:{written,path}, next}；拒即教学 JSON
- 退出码循 init 既有约定：零成功；一前置拒或验红；二工具自身异常
- 分面申报：bootstrap 是操作位编排层，落盘面即签发行（经 tokens_mod 唯一 sanctioned 写点位）加镜像行加 tempfile report 加可选客户端配置件；writeface 零直写红线不变
- 语汇：标识牌（防呆锚点非安全凭据，DEC-010 已拒凭证语义）

**件二 台面开域视图（web.py）**

- 两步确认形循签发先例：/tokens 台面加开域表单（域根、标识、档位、事由）→ POST /tokens/open 确认面 → POST /tokens/confirm-open 执行
- 执行经 bootstrap 模块同一函数（台面零二次实现）；台面路径不含客户端注册（台面不知客户端路径，CLI 旗标专属）
- 无管理钥回环零防、签发撤销三动作零变更、assert_http_face fail-closed 不动

**件三 检词正登**

- bootstrap／域自举 established 登记（源 2026-09-10 立名会，典承 DES-015 域自举程序形）；撞懒波区既有档显式升格（upgrade 真值）；登记输入件与登记后包形入批材料

**件四 DES-015 修订四**

- 修订记录节追加，内容五点：全链固化形（bootstrap 命令五段序）；镜像写入位定形（登记册落位两形实装补全，逐字段恒等）；台面开域视图位（两步确认形，候裁项落位）；客户端注册旗标选入位（合并写形与固定 payload）；管理台签发撤销三动作零变更与语汇判词（标识牌非凭据）
- 令源用户原话入修订行；走化格核阅检词管线

**件五 测试**

- B-1 全链 fresh：空册加 --token-id → 中央行落加开域加真 scribe 链恰一笔 verify valid 加镜像行在位加出参严格 JSON
- B-2 已有 active 行 → 不重签（issued.done 假）加已开域幂等拒回归
- B-3 末行 stopped → 轮换签发新行后开域
- B-4 须签发缺 --token-id → 拒，零写入
- B-5 --client-config：新建、既有文件其他 server 条目保留、重跑幂等
- B-6 镜像行与中央末行六字段恒等对表
- B-7 验红链留笔（循 T-8 形）
- 台面：开域两步确认路由测试（确认前零写入、确认后全链落）
- 零写守卫：test_zero_write 白名单扩项（bootstrap.py 加 web.py 开域动作写点）后全套绿

**件六 版本与结果档**

- mcpline 0.5.0 升 0.6.0（pyproject 加 __init__ 双点同步）
- 结果档 sih-engine/sih/event/plan/mcpboot-solo-results.md 加 materials/；认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid

## 三、工作清单

- [ ] T-1 bootstrap 模块五段链实装
- [ ] T-2 web.py 开域两步确认两路由
- [ ] T-3 测试 B-1..B-7 加台面测试加零写守卫白名单
- [ ] T-4 检词正登 bootstrap／域自举
- [ ] T-5 DES-015 修订四加管线三步
- [ ] T-6 README 用法节加版本 0.6.0
- [ ] T-7 结果档加认证上链加双仓 settle 加收约对账

## 四、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F-1 | 功能 | B-1 全链 fresh 退出码零，链 verify valid，镜像行六字段恒等 |
| F-2 | 防呆 | B-2/B-3/B-4 三态出参如实（reused、轮换、拒教学），B-4 零写入 |
| F-3 | 合并写 | B-5 三例（新建、保留、幂等）客户端配置字节形在档 |
| F-4 | 红证 | B-7 验红链留笔不删 |
| F-5 | 台面 | 两步确认：确认前零写入断言、确认后全链落断言 |
| F-6 | 回归 | mcpline 全套 pytest 绿（零写守卫白名单扩项后）；台面既有四动作零回归 |
| F-7 | 文档 | DES-015 化格核阅检词三步退出码零；检词正登件在档 |

## 五、必读文件

- sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md（唯一设计正典）
- sih-tools/mcpline/src/mcpline/init.py 与 tokens.py 与 web.py 与 writeface/domains.py
- sih-tools/mcpline/tests/fixture_root.py 与 conftest.py 与 test_init_domain.py（夹具先例）
- sih-tools/nomenclator/CONTRACT.md（登记前校验四查）
- sih-tools/BATCH-FACE.md（批机械链）

## 六、约束（红线）

1. DES-015 唯一正典，缺口停批候裁禁自由发挥
2. init.py 核心零改动（仅 docstring 指针微调申报）；writeface 与 httpface 与 tokens 与 runtime 与 domains 模块零改动；bootstrap 是新模块
3. writeface 零直写盘红线不变；tokens 直写只经 tokens_mod 既有函数
4. 禁触：sih-visual/**、SPEC-023、中央登记册 tokens.ndjson 主树实测数据（测试用 fixture 册）、InferServer 与司梦工作区
5. scribe 与 identity 与 lease 与 gauge 零改动；引擎 Rust 零改动零重编
6. 每条命令立即取退出码，失败即停整链，禁管道掩码
7. scribe 二进制一律主树 sih-engine/target/debug/scribe
8. 主树零直写：一切待提交件先入工地（worktrees 双仓 mcpboot-solo）
9. 先红留痕：首跑红证入批材料禁清洗重跑

## 七、验收标准

- 测试全绿：mcpline 既有加新增（真 scribe 二进制，fixture 根隔离）
- DES-015 管线三步俱绿；检词正登件在档
- 认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid
- 台面既有动作零回归（test_token_console 与 test_web_smoke 绿）

## 八、风险点

- 并行窗申报：des016impl 批（另一窗）ask3 已备，其范围含 mcpline init 模板面与版本升降（0.5.0 升 0.6.0）；本批按锁台账判定开工（锁零即开工，packhyg 纪律），版本位先落者得 0.6.0，后开批重读现状顺延；init.py 冲突面仅 docstring 一行，让位归并可承载
- 台面执行开域动作耗时（真 scribe 落链）——同步处理，测试内超时上限放宽
- 客户端配置合并写对畸形 JSON 输入：解析失败即拒教学零写入（报文在档）

## 九、范式偏离声明

计划期定三路由（GET 表单加两 POST），实装循签发/撤销既有两路由形（表单嵌台面页加 action 加 confirm），少一 GET 路由，形变更贴既有家族模式，随批如实申报。

## 十、关联文件

- pk-084（DES-016 设计批，已出泊）：本批零触碰意图模板面，其实现批落 TEMPLATE-意图-plain.json 后 bootstrap 零改动兼容
- mcpinit-solo 结果档与 CALL-LOG（init 先例）
- 司梦开域观测报告锚点（双落位实物形与 .zcode 实物形之证源）

## 十一、请求写入（逐路径分行）

sih-tools/mcpline/src/mcpline/bootstrap.py
sih-tools/mcpline/src/mcpline/web.py
sih-tools/mcpline/src/mcpline/init.py
sih-tools/mcpline/src/mcpline/__init__.py
sih-tools/mcpline/pyproject.toml
sih-tools/mcpline/README.md
sih-tools/mcpline/tests/test_bootstrap_domain.py
sih-tools/mcpline/tests/test_token_console.py
sih-tools/mcpline/tests/test_zero_write.py
sih-tools/mcpline/tests/fixture_root.py
sih-tools/nomenclator/packs/core/terms.json
sih-tools/nomenclator/packs/core/lazy.json
sih-tools/nomenclator/registers/
（目录级理由：检词登记输入件出生地，件名 2026-09-10-bootstrap.json 随批新出生）
sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md
sih-engine/sih/state/plan/mcpboot-solo.md
sih-engine/sih/event/plan/mcpboot-solo-results.md
sih-engine/sih/event/plan/mcpboot-solo-materials/
（目录级理由：批材料出生地，先红留痕与登记件与管线报告入档）
sih-engine/sih/event/trail/2026-09-10.ndjson
sih-tools/scribe/reports/
（目录级理由：ask3 记录与双门验证件与叩问信号与 digest 与认证报告出生地）
sih-tools/identity/reports/
（目录级理由：正身件出生地）
