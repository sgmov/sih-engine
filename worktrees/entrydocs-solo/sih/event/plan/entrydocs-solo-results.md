# entrydocs-solo 结果档：文档面补齐批

> 承接：任务包 sih-engine/sih/state/plan/entrydocs-solo.md 与用户 2026-09-08 令「你来补文档面，务必降低学习曲线。或者是做两版，一版是面对用户的，一版是面对愿意参与引擎社区共同开发的高级用户」。
> 队形单线 solo 主窗 inline 亲做，日期 2026-09-08，会话 sess-zcode-260908-main-entrydocs（租约 aa117f9f847c7134）。

## 交付 {#deliverables}

三件文档与一件补目：

- sih-engine/README.md 即五分钟入口，是什么、三句信任声明、五条基线大白话、五分钟跑起来三命令、双版入口指针、仓内一图
- doc/guide/user-guide-v1.md 即使用者入门，承诺面三条、九词大白话表、真实批走读即魔法数字清账六步、人节点四类动作、验算三命令行内形、问答六问、延伸地图
- doc/guide/contributor-guide-v1.md 即贡献者入门，五仓地图、组件六席与工具一览、开发纪律四条、机械链十三节拍、提交变更五步、泊界纪律、治理文档族谱、新人首周路径
- doc/CASCADE.json 补 guide 双件目即七十三边

## 意图与前置 {#preface}

- 意图笔 66b40464 即 intent_refined，ask3 记录与验证件双门绿即 scrutinator ask3 包 exit 0 加 ask3repeater status ok anchor_count 3
- 叩问 digest passed covered 6，六词描述性使用不立名不登记
- 正身 anomalies 0 status attest，identity_hash 2892ee66
- 租约九面锁全过，写入面与 allow 面一致

## 管线读数 {#pipeline}

- 化格：三件过 packs/general-v1 全 exit 0
- 核阅：两 guide 过 des-001 exit 0；README 根路径与本结果档 event/plan 路径俱域外 exit 2 如实记档不属违规即 des-001 域只盖 doc
- 检词：三件过 packs/core 全 exit 0 零违例
- 修红实录：user-guide 首跑 S002 两笔即围栏内注释行被判一级标题，改行内命令形后再跑 F000 三笔即业务文档禁围栏代码块，改行内反引号形后归零；contributor-guide 首跑 C002 一笔即 U+2192 箭头超字符集，改全序十三节拍顿号列举形后归零。红证在批材料。
- checkcite 守卫：本批引用零推导档 ID，条件不触发如实记档

## 认证与结算 {#settle}

- 内容哈希清单件绑定 README 与双 guide 与本结果档与 CASCADE.json 五路径 sha256，认证事件号随收约补笔回填
- settle 提交号与 verify 与 reconcile 读数随收约补笔回填

## 大白话节 {#plain}

给外人看的门牌和两份导览写好了：门牌五分钟说清这是什么、凭什么信、怎么跑起来；使用者导览三十分钟，先用九个词把黑话清空，再跟着一个真实发生的清账故事走一遍，最后教三条谁都能随时自己跑的验算命令；贡献者导览给想一起造的人，从五仓地图到提交一个变更的完整五步。所有命令和流程都是本日亲手跑过的实态，没有一句虚构能力；引擎根下那份门牌不在文档规则管辖域内，这个事实照实记了账。

## 越线与误差申报 {#violations}

- 任务包与提示词件即 entrydocs-solo.md 落 sih/state/plan/ 未入本批 allow 面，留主树候后继收编批提交，close 无主位如实申报
- scribe intent 首跑缺 --sessions 参 exit 1 零留痕，补参即过
- 其余误差零申报

## F 表 {#ftable}

| F | 判据 | 实态 |
|---|---|---|
| F-1 三件齐 | 产出 | README 加双 guide 加 CASCADE 补目在档 |
| F-2 管线绿 | 治理 | 化格核阅检词序固定全跑，两 guide exit 0，README 域外记档 |
| F-3 链痕全 | 治理 | 意图笔与认证笔与 settle 在链，读数随补笔 |
| F-4 写入仅 allow | 治理 | 九锁面与 allow 一致，任务包留主树申报在案 |
| F-5 收约全绿 | 治理 | 随收约补笔回填 |
