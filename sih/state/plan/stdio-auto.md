# stdio-auto：stdio 自动签发与开域

> 令源：用户 2026-09-17 裁定原话「stdio应该是自动签发令牌，令牌最重要的目的是选定治理空间」（承 pk-105 加 pk-106 裁定问的答）
> 范式：solo 单线，agent 亲写零子代理
> 前置：readme-adopt 批已收约（走查证据与两泊件在链）；SPEC-023 承 MCP 载体契约
> stem 认领：stdio-auto，甲表三件即 zh stdio 自动签发开域、code 无承、派生 stdio:new 加 auto:new

## 一、问题陈述 {#problem}

- 走查实证（readme-adopt 批）：新域接入唯一活路是管理台 /tokens/open 两步确认，令牌本义是域绑定锚点（防呆非安全凭据）却强迫浏览器仪式，与缺省 stdio 轻接入矛盾（DEC-026）。
- pk-105：bootstrap 开域链不写 .git/info/exclude，sih 树污染用户 git；exclude 写入只在 sih init 窄口（双向不可达）。
- 根解析陷阱：runtime.resolve_root 以 SIH_ROOT 为中央工作区根，而 stdio 客户端以 SIH_ROOT 指域根，同进程直调 bootstrap_domain 会把中央登记册落进项目内（指南第六问警告的同族病）。

## 二、关键设计 {#design}

- 用户裁定承载：stdio 连接遇未开域的 SIH_ROOT（.git 在位且 sih/domain.json 缺席）即自动走开域全链，令牌自动签发（token_id 缺省派生），stderr 透明告知，不静默。
- 子进程隔离：同进程直调会踩根解析陷阱，故 stdio 位以 current_exe 子进程跑 `sihmcp bootstrap <域根> --by stdio-auto`，env 卸 SIH_ROOT、cwd 取 code_root()（SIH_MCPLINE_CODE_ROOT 或标记上溯），中央登记册解析自然正确；失败如实报 stderr 后以未开域形态继续（读面教学照常）。
- pk-105 收口：bootstrap.rs open_domain 落成后幂等写 exclude（sih/ 一行，建 info 目录，已有即跳过），出参载 flywheel_git_exclude 判词；与 sih.rs 窄口同语义零二账。三个入口（stdio 自动、CLI bootstrap、控制台 open）由此全覆盖。
- 不动面：签发闸牌先于域语义零改动（HTTP 面仍然正典）；sih init 窄口本批零触碰，其被本能力取代的退役裁定候另批（pk-106 半收口）。

## 三、工作清单 {#work}

- [ ] sa-01：bootstrap.rs open_domain 补 exclude 幂等写入与出参判词
- [ ] sa-02：sihmcp.rs stdio 位自动开域子进程（决策谓词可测）
- [ ] sa-03：测试三族先红后绿（exclude 幂等单元、CLI bootstrap 集成含 exclude、stdio 自动开域端到端夹具）
- [ ] sa-04：README 接入节改自动开域口径，检词指纹自检
- [ ] sa-05：pk-105 出泊笔、意图认证笔、settle 收约、推双远端

## 四、验收 {#acceptance}

- 隔离夹具（临时工作区标记 + 临时域根）内：stdio 首连自动开域成功，domain.json 与 exclude 行与中央登记册 active 行俱在且登记册落工作区不落项目；已开域域根零重开（幂等）；非 git 域根零自动开；cargo test 全绿；README 口径与实态一致。
