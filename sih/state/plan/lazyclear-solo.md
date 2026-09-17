# lazyclear-solo 批任务包：基线管懒波债清偿——升格立名

> 令源：用户 2026-09-09 裁二原话裁定「先清债后复链」（specfix-solo 停批候裁三选项之二，停批报告在 sih-engine/sih/event/plan/specfix-solo-materials/stop-report-2026-09-09.md）
> 形：solo 微批独立立约独立收约，与在飞写批共用今日链走租约排队

## 一、使命

将检词登记册中「基线管」懒波条目升格为正式登记，使 SPEC-021 既有行检词过闸，specfix-solo 得以复链。目标终态：nomenclator check SPEC-021 rc=0，且该词在全库核查零违规。

## 二、考古先行（第一步必做）

1. 读 sih-tools/nomenclator/packs/core/lazy.json「基线管」条目全文与 terms.json 与 dead.json 中 basemgr 与基线相关条目，弄清该实体（sih-tools/basemgr）名下已有哪些登记与死名
2. 读 sih-tools/basemgr/CONTRACT.md 的自名面与 toolincub-solo 批材料（sih-engine/sih/event/plan/toolincub-solo-*），确认工作名来历
3. 读 nomenclator 自己的 CONTRACT.md，确认懒波升格的机械通道（状态迁移形、信封与清单版本义务、测试族义务）

## 三、升格判则（照此执行，不越）

- 判则一：若 terms.json 无该实体已登记正式名（无冲突），即把「基线管」按登记册自身契约升格 proper，理由载三事——名随实达（basemgr 工具已实装）、正典在用（SPEC-021 在用词先于登记且持续）、人节点裁二明授升格立名；lazy 条目按契约消化（消化形照 CONTRACT，不留双态）
- 判则二：若 terms.json 已有该实体另一正式名（有冲突），即停批出报告候裁，附名实档案与建议，禁硬改任何一方
- 判则三：dead.json 若有该实体死名，血统记入升格条目来历，不复活死名
- 本批零触碰：SPEC-021 与 SPEC-022（specfix 冻结域）、basemgr 代码与契约档（除非登记契约强制要求随名改注，且改注属加注不改义）

## 四、红线与验收

- nomenclator packs 版本或信封若 bump：其自身测试族与冻结向量全跑（recclsf 先例：版本嵌入报告，bump 必重跑全族）
- 验收：check SPEC-021 rc=0；check SPEC-022 rc=0 复证；抽测 SPEC-023 与 sih-engine/README.md rc=0 无回归；认证上链、双仓 settle、reconcile 双零
- 与在飞 mcpsec、toolhyg、release09、parksi 四批共用今日链：锁面含 trail，冲突走 lease wait-turn，禁绕行禁 preempt
- 每条命令立即取退出码，失败即停，禁管道掩码；scribe 二进制一律主树 target/debug；commit 须指向登记 worktree

## 五、写入面（allow 清单）

- sih-tools/nomenclator/packs/core/**（lazy.json、terms.json、信封与清单按契约所需）
- sih-tools/nomenclator/tests/**（若契约要求随版补测）
- sih-engine/sih/state/plan/lazyclear-solo.md 与 lazyclear-solo-prompt.md
- sih-engine/sih/event/plan/lazyclear-solo-results.md 与 lazyclear-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-lazyclear-1/

## 六、完工回报形

批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、升格前后两态对照（条目 JSON 前后）、check SPEC-021 与 SPEC-022 与两件抽测的 rc 读数、判则命中哪条。
