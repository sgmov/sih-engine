# recclsf-solo 任务包：reconcile 分类器认补录形与销账形批

> 令源：用户 2026-09-08 令「走乙，已有先例」即 certarch-solo 呈报的乙案——修订 reconcile 分类器认补录形，先例即分类器已认 bypass 台账形同物种。
> 队形：单线 solo，主窗后台拉起子代理执行，主窗独立复算。批日 2026-09-08 起，会话号 sess-zcode-<实日>-main-recclsf。开工前置即 sweepjson-solo 已收约即 lease 域锁已清，撞锁即排队。

## 范围 {#scope}

0. 跑步机路径形修复：hygspots G2 生产缺陷即 calllog_treadmill_collect 的 faces 根相对模式对 git status 仓相对路径永不匹配致收编恒空（sweepjson-solo 批 2026-09-08 两度 close 实测呈报候人裁，主窗折入本批即缺陷修复非裁决）。修法即路径形态归一匹配，先红后绿加两形回归钉即根相对与仓相对俱命中，收编动作实态复测在档。
1. 分类器修订：lease reconcile 的 cert_missing 判定增两形认账——补录形即当日链存在补录认证笔且其报告 content_hashes 绑定该提交 sha 或原始 cert 哈希即归类 cert_backfilled；销账形即 bypass 台账存在绑定该提交 sha 的销账登记即归类 cert_writtenoff。两形俱转新类计数显式呈报，零隐藏零降格为 pass；不认无绑定与绑定不符件。
2. 判据读数：修订后三仓 reconcile 即 sih-tools 5、sih-engine 3、sih-math 2 的存量应全数转出即 cert_missing 归零、cert_backfilled 5、cert_writtenoff 2；不齐即如实申报差件不硬凑。
3. 先红后绿：修订前红证在档即现分类器判五件为 missing；修订后绿证即七件转新类；回归测试两形各覆盖加绑定不符反例一即不认；金向量随冻。
4. CONTRACT 修订条目：升版入档，语义条款即两形认账条件与先例承引即 bypass 形同物种。
5. 收口读数：三仓 reconcile 前后对照表、链 verify、unrouted 零。

## 产出 {#outputs}

任务包本件与提示词件（主窗预落）、分类器修订与测试与金向量、CONTRACT 修订、红绿证、三仓前后对账表、ask3 记录与验证件、正身件、租约与锁实录、意图链笔、认证清单、结果档含大白话节、投影件 recclsf-ledger.json 认证上链、双仓 settle 提交号、链 verify 与 reconcile 读数、越线与误差申报。

## 边界与红线 {#redlines}

- 认账不降格第一红线即两形转新类显式计数，不得并入 cert accounted 类一混了之，不得静默清零。
- 绑定校验严格即 sha 逐字节比对，绑定不符即仍 missing 加告警行。
- 零触碰 sweep 判定逻辑与 sweepjson 产出面；历史链文件零改写；主树零直写（两件预落除外）；链文件只经引擎 scribe；禁管道掩退出码；误差红证如实记档。
