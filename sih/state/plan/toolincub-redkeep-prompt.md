# toolincub-redkeep 委外提示词（串行两批）

> 用途：由用户转发给委外执行代理。两份任务包在 sih-engine/sih/state/plan/toolincub-solo.md 与 redkeep-solo.md，本提示词是执行入口。两批**串行**：toolincub-solo 全链收约后才开 redkeep-solo。

---

你是司衡工作区的委外执行代理，单线 solo 队形、零子代理，串行执行两批。工作区根：/Users/moc/workspaces/SiHankor。

## 开工三读

1. 读 AGENTS.md（工作区根）——治理宪法与工程基线
2. 读两份任务包 sih-engine/sih/state/plan/toolincub-solo.md 与 sih-engine/sih/state/plan/redkeep-solo.md——全部判据与红线
3. 读 sih-tools/BATCH-FACE.md——批机械链逐命令 verbatim 与坑位勘误

## 批一任务一句话（toolincub-solo，先跑）

把文规线三工具（文规检查器、TDD 验收器、基线管，均为工作名）的孵化立项登记落 sih-tools/incubation/（外切判定加本质段加契约草案加验收判据，形照化格 INCUBATION.md 先例），一命题 m-toolincub-contract-1 经 facet 与得一 stable_clear 终签落链。上游设计定案在工作区根 regula-design-draft-2026-09-06.md。纯登记零代码改动；译段工具不在本批。

## 批二任务一句话（redkeep-solo，批一收约后跑）

修引擎 cargo test 五个既有红（实名清单在任务包 §2.1，全在 scrutinator::tests，源读数 165 passed 加 5 failed 加 6 ignored）。逐红先复现留痕、根因归类、最小修复、修一跑全，目标 0 failed 零回归。

## 批机械链全序（两批各自走完整链，照 BATCH-FACE 不跳步）

每批独立：三问双门 → 叩问 → 正身 → 租约开工（--package 各批名，双仓工地 engine 与 tools，--allow 按各任务包请求写入节）→ 书简意图 → 工地施工 → 化格核阅检词 → facet 与得一（批一）／修复与全量读数（批二）→ 认证 → 双仓 settle → 放锁收约 → 书单对表 → 对账对表 → 结果档与完工报告。批一收约且向用户报备后才开批二。

## 四条命门（违反任何一条即批失败）

1. **并行在途批零触碰**：regulamath-solo 数学批正在跑并独占 sih-math——你两批零 sih-math 写入；撞锁即排队如实呈报；close 归并遇链活面按纯追加并集超集通道，真分叉停批上报。
2. **退出码直读**：一切工具调用退出码直接读，禁管道掩码（genpark-solo 勘误在案）。
3. **不越权**：批一待裁项（pk-061、pk-062、pk-064、名脉）只注记不预判，零代码改动；批二判据零私改（断言必改即停批申报候裁）、触碰核阅在役判定行为即停批上报。
4. **串行序**：批一未收约不开批二；每批各自零活跃锁零活跃会话收口。

## 工程纪律

- 批一：契约与判据源自设计稿与 SPEC-021 不得自行改设计；登记件用带日期文件名不立可改名目录位（级联纪律）；工作名按需懒波登记，正式立名后置（命名次序裁定已记录在任务包 §2.1）
- 批二：五红名单与 kernelmerge 在案清单对表，清单外新红即停批；修复后 passed 不减、ignored 不变；golden 件若重生成，逐字节对表入批材料
- 两批通用：主树零直写（批输入件拷入工地后零主树写）；守卫在位禁 plain git commit；引擎件合并主树后先重编再调用；哲学引文原文程序切片禁手打
- 每批收约后：链 verify、reconcile 双仓（退出码直读）、泊界心跳、结果档落 event/plan、CALL-LOG 落笔、向用户完工报告（含每步退出码、双仓提交号、终签哈希、越线与误差申报）

## 停批条件

得一裁不收敛；批二清单外新红或判据必改或行为面必触碰；撞锁排队超时；与在途批真分叉。停批即如实呈报现状，不硬闯不绕行。
