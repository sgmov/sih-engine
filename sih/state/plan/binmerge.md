# binmerge：六 bin 腿件收编

> 令源：用户 2026-09-17 令「补收编。另外查一下为啥会漏」（漏因查证随批档，见材料 leak-forensics.md）
> 范式：solo 单线，agent 亲写零子代理
> 前置：bscomplete 批已收约；六 bin 与三 results 档为 dwfrun（2026-09-15/16 动态工作流）最终版工作，git log -S 验证不存在于任何提交任何分支
> stem 认领：binmerge，甲表三件即 zh 六bin收编、code 无承、派生 binmerge:new

## 一、问题陈述 {#problem}

- 主树九件在途改（formatter 加 locator 加 nomenclator 加 parser 加 selector 加 tally 六 bin 约 678 行，domaware 加 mcpdual 加 wengumcp 三 results 档批尾回填）无主悬置一点五日，每次收约撞孤儿闸 bypass 留痕。
- 漏因查证（三层）：直接因即 mergeall-closeout 任务包把六 bin 误分类为「并发批 M 件一律零触碰留原批」，而账册 09-15/16 仅有 mergeall-closeout（allow 全仓）与 adoptface（allow 不含六 bin），批 M 不存在；结构因即结算纪律允许排除性分类但无登记面（不留泊位不留待办不留账行），排除即成无主孤儿；放大器即 bypass 事由首写「并发批在途件」后被各批照抄，无人翻账册验真。

## 二、关键设计 {#design}

- 收编面九件全量复制入 worktree，worktree 内全量测试即验收（对照基线即无六件仓的全量已知红即 gap_parser 两件），零新回归即过。
- 过程缺口入泊 pk-108：排除性分类无登记面，候闸面改造裁定即 bypass-orphan 事由须附处置指针（泊位或待办）加孤儿件带龄升级呈报。
- sih-tools 侧四件（AI-MANUAL 改动、tokens 登记册、m-retireform 三件）不在本批：sih-tools 系冻结只读独立仓，其治理另裁；tokens 登记册为活性账本逐笔增长属设计内。

## 三、工作清单 {#work}

- [ ] bm-01：lease open 加九件锁加 worktree 复制
- [ ] bm-02：worktree 全量测试零新回归（gap_parser 两件系既有红如实申报）
- [ ] bm-03：pk-108 入泊笔加意图认证笔
- [ ] bm-04：settle 加归并加 close 加推双远端，主树九件转净

## 四、验收 {#acceptance}

- 全量测试失败集与基线全等即仅 gap_parser 两件；归并后主树 git status 九件转 clean；pk-108 在泊；双远端快进。
