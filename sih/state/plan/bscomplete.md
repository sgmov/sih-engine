# bscomplete：开域补全链 Rust 承载

> 令源：用户 2026-09-17 令「107出泊修复」（兼裁 launchd 常驻面不算产品形态系开发调试用）
> 范式：solo 单线，agent 亲写零子代理
> 前置：adopt-guide 批已收约（pk-107 在泊即本件）；补全链语义正典即围堰 bootstrap.py run_chain complete 分支（只读对表，围堰零改动）
> stem 认领：bscomplete，甲表三件即 zh 开域补全链承载、code 无承、派生 bscomplete:new

## 一、问题陈述 {#problem}

- pk-107：sihmcp bootstrap --complete 在 Rust 载体未实现，报错教学语指向已冻结的 Python 载体，半态与已开域的补全（镜像缺行、面卡过期、客户端注册补跑）无路可走。
- 语义正典已在围堰（DES-015 修订五）：已开域加 --complete 即补全链——段一余检照跑、签发位复用 active 行、开域位改验域形即读域声明加域链最新一日 scribe verify 零域状态写、镜像核对位即中央 active 行与域镜像末行恒等对表缺行补写恒等行、面卡重写、客户端注册位照跑；未开域给旗标即无害走正常全链；域链缺席即工具异常如实拒。

## 二、关键设计 {#design}

- bootstrap_domain 增 complete 参：opened 加 not complete 照旧拒（幂等守卫语义不变），opened 加 complete 走补全链三分支；三新私有函数即 load_declaration 加 verify_domain_chain 加 reconcile_mirror_row，逐行对表围堰（bscomplete 批只读对表围堰源码零改动）。
- 出参形对表围堰：补全链出参不含 files 与 open_pen_hash，含 mirror_action 即 in_place 或 appended。
- 指南两处口径同步：兜底矩阵半态行与手动备选节，--complete 由候批改在役，域链缺席如实拒候人节点。
- 附带登记：用户裁定 launchd 常驻 8765 面不算产品部署形态，系本机开发调试便利，文档不提；走查批发现的六 bin 在途件经 git log -S 验证不存在于任何提交任何分支（回滚即丢），处置候收编批另令。

## 三、工作清单 {#work}

- [ ] bc-01：bootstrap.rs 补全链三分支实装加 run_cli 旗标贯通
- [ ] bc-02：测试四件先红后绿（健康域 in_place、缺镜像 appended、域链缺席如实拒、未开域旗标无害走全链）
- [ ] bc-03：指南两处口径同步，化格核阅检词三步全零
- [ ] bc-04：pk-107 出泊笔加意图认证笔，settle 加 close 加归并加推双远端

## 四、验收 {#acceptance}

- 四测试全绿；补全链在健康域零域状态写（domain.json 逐字节不变）；缺镜像行补写恒等；无链域链缺席退出码二如实拒；指南零句与实态相悖；三步管线全零；pk-107 出泊；双远端快进。
