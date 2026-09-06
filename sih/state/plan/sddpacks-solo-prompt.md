# sddpacks-solo 委外提示词

> 用途：由用户转发给委外执行代理。任务包在 sih-engine/sih/state/plan/sddpacks-solo.md，本提示词是执行入口。

---

你是司衡工作区的委外执行代理，单线 solo 队形、零子代理。工作区根：/Users/moc/workspaces/SiHankor。

## 开工三读

1. 读 AGENTS.md（工作区根）——治理宪法与工程基线
2. 读任务包 sih-engine/sih/state/plan/sddpacks-solo.md——本批全部判据与红线
3. 读 sih-tools/BATCH-FACE.md——批机械链逐命令 verbatim 与坑位勘误

## 任务一句话

把 SDD 五件格式包（变更提案、规格差分、场景清单、技术方案、任务清单）按孵化登记件的包 schema 契约成文落 sih-tools/incubation/packs/sdd-v1/（manifest 加五包加自反狗粮样例五件加自携校验脚本），一命题 m-sddpacks-pack-1 经 facet 与得一 stable_clear 终签落链。纯数据零代码；场景语法即 OpenSpec 骨架三刀形（R-／S- 编号、THEN 判据绑定、独立成件）。上游定案在工作区根 regula-design-draft-2026-09-06.md §二§三。

## 批机械链全序（照 BATCH-FACE 逐命令跑，不跳步）

三问双门 → 叩问 → 正身 → 租约开工（--package sddpacks-solo，双仓工地 engine 与 tools，--allow 按任务包请求写入节）→ 书简意图 → 工地施工（批输入件拷入工地、manifest 与五包、样例五件、validate.py 跑绿）→ 化格核阅检词 → facet 三步 → 得一三步（check／verify／sign）→ 认证 → 双仓 settle → 放锁收约 → 书单对表 → 对账对表 → 结果档与完工报告。

## 四条命门（违反任何一条即批失败）

1. **纯数据零代码且避开租约**：零引擎与工具源码写入，零租约源码写入（租约修复中）；validate.py 是批内校验脚本落批目录不落工具位。
2. **判定面全机械**：包判定面条款逐条可机械验、逐条带三态失败定位；语义层检查显式申报范围外，禁伪装机械。与登记件契约不合即停批上报，不私改契约。
3. **退出码直读**：一切工具调用退出码直接读，禁管道掩码（genpark-solo 勘误在案）。
4. **零越权**：零 sih-math 写入；在泊件与在盘遗留（sih-math 未跟踪件、billwire 残件、engine 未跟踪结果档族、rootanchor 陈旧工地）零触碰不代清；人节点零代行；守卫在位禁 plain git commit；中文零新造正式词。

## 工程纪律

- 包形按孵化登记件契约；契约未定形处用 JSON 过 json-canonical-v1 并申报理由
- 样例五件以本批自身为变更对象（自反狗粮），引用闭包要真闭：样例内引用的 R-／S- 编号须在样例场景清单内可达
- validate.py 用 Python 标准库零依赖，双跑逐字节一致；检查器实装后退役由真机接管，声明入 manifest
- 主树零直写（开工后第一动作把任务包与提示词从主树拷入工地，此后零主树写）
- 收约后：链 verify、reconcile 双仓（退出码直读）、泊界心跳、结果档 sddpacks-solo-results.md 落 event/plan、CALL-LOG 落笔、向用户完工报告（含每步退出码、双仓提交号、终签哈希、越线与误差申报）

## 停批条件

得一裁不收敛；包与契约的缝无法以申报处置；需新造中文词；与在途租约修复批撞锁排队超时或链真分叉。停批即如实呈报现状，不硬闯不绕行。
