# rant-doc：门面吐槽文（README 补充）

> 令源：用户 2026-09-17 令「结合 ai-ex 的各种沉淀、痛点拆解，重新写一份类似用户对话的文档，作为 Readme 的补充。行文风格需要轻快，像是开发日常吐槽，需要参考小红书那种爆款文案的写法」
> 范式：solo 单线，agent 亲写零子代理
> 前置：readme-plain 批已收约（README 痛点框架在役，本文承接同一痛点面）；素材源 ai-ex 沉淀库（不入 git 的经验档，引用其案例事实不搬其文本）
> stem 认领：rant-doc，甲表三件即 zh 吐槽体门面文、code 无承、派生 rant:new 加 doc:new

## 一、问题陈述 {#problem}

- README 是正经门面，痛点讲得克制；社区传播需要一份轻快的对话体补充读物，把 ai-ex 里带日期带原话的真实翻车案例讲成人话。
- 风险二：小红书风（emoji、短句、钩子）与治理域文档规范冲突——对策：落位仓根（des-001 域外），不入 doc/ 目录，不进 T6 管线。
- 风险二：案例失真——对策：六案例逐一锚 ai-ex 原档（AI-MISALIGNMENT-PATTERNS、INTENT-DRIFT-INVENTORY、MINIMAL-FIX-BIAS、SELF-DERIVED-INSTEAD-OF-USING、HAS-REGISTRY-BUT-NOT-READ、SIX-ABANDONED-REPOS-LESSONS），只改述不虚构。

## 二、关键设计 {#design}

- 形态：帖子体（开帖+编号翻车+大结局+置顶+评论区引导+话题标签），文件名 RANTS.md 落仓根。
- 内容：六个真实案例（意图错位、假扫描声明、最小改动偏差、看见没读到、子代理旧地图、存在不等于可用）+ 六废仓大结局；每案例后接「司衡的解法」短评，解法口径与 README 组件表逐条一致，不发明机制。
- 置顶含丑话（手续成本、适用域指回 README），诚实口径与门面一致。
- README「文档与贡献」节补一行链接。

## 三、工作清单 {#work}

- [ ] rd2-01：RANTS.md 落 worktree 仓根
- [ ] rd2-02：README.md 补链接一行
- [ ] rd2-03：用户过目后 settle 与收约与推送

## 四、验收 {#acceptance}

- 六案例与 ai-ex 原档事实一致（日期、原话、数字不虚构）；司衡解法与 README 组件表零矛盾；风格达小红书爆款形（钩子、短句、emoji、互动引导）；用户过目后收约。
