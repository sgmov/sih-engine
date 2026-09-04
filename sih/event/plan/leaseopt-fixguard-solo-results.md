# leaseopt-fixguard-solo 结果档

> 批：leaseopt-fixguard-solo（leaseopt 线批二，收约守卫假阳性修复）
> 会话：eaf80aa8d5b666a2（ask3 侧 sess-zcode-260905-fixguard）
> 日期：2026-09-05。队形：单线形 solo，主会亲写，零子代理。
> 令源：用户 2026-09-05「开，你自己跑，不委外」。

## 一句话结论

守卫假阳性修复完成：detect_merge_conflicts 判定域收缩为差集交真实脏位，三夹具先红后绿、全族 94 件零回归、金向量双跑一致、checkcite pass；得一裁九发测量判 near_threshold（判定九发全 comply 稳定、边界旗零、唯依据族两值分散基线四×7 基线一×2），按机械闸语义待人节点复核，切换终签挂起，批保持开位，主树零归并。

## 一、F 表

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 净态退场 | 基线前进净态件修复前红后绿 | 过 | 红：diverged_files=[file-b.md, demo-pkg.md] 整批拒（与 2026-09-04 晚四笔 bypass 同形）；绿：收约过，主树 file-b.md=基线新版、own-file.md=分支新增 |
| F-2 双改归 merge-tree | 拒因错位修正 | 过 | 红：拒因「存在真分叉冲突」；绿：拒因 merge-tree CONFLICT，零半程动作 |
| F-3 真分叉守恒 | 真脏位 diverged 仍拒 | 过 | 修复前后俱拒（守恒测试） |
| F-4 回归 | 全族零回归 | 过 | 94 passed（65 旧 + 3 新 + 其余既有）；金向量双跑判定行逐字节一致 |
| F-5 得一裁 | near_threshold 挂起经人节点复核放行 | 过 | near_threshold：decision_stable T、boundary_low T、basis_consensus F（基线四×7、基线一×2）、boundary_rate 0.0；sign 未执行，主树零归并 |
| F-6 常数与语义面 | 零常数新增、三态与未跟踪面零改 | 过 | diff 亲读：仅 status 解析扩展与净态退场 continue，三态判别函数零触碰 |

## 二、修复 diff 要点

`lease/src/lease/core.py` detect_merge_conflicts：status --porcelain 解析从只取 `??` 扩展为四态（`??` 入未跟踪、非 `??` 入真脏位集合、重命名行双侧入集）；merge_files 循环首增净态退场位（差集件不在真脏位且不在未跟踪即 continue）。三态判别、未跟踪碰撞、_close_precondition、_merge_file_diverged 零触碰。

## 三、得一裁挂起详情

- gid m-fixguard-switch-1，合同九发，verdict near_threshold，criteria：decision_stable T / boundary_low T / basis_consensus F / foregrounding_stable T，boundary_rate 0.0，voids 0。
- 依据族两值分散为如实作答非挑选：修复依据面真实双源——可校验性（基线四，七发）与检验由可重复程序承载（基线一，两发）。distinct_basis=2，按 facetmath 先例（=3 挂、≠2 降级位）落入 near_threshold 带位。
- 材料指针：facet/contracts/leaseopt-fixguard-260905/m-fixguard-switch-1/（topic、contract、responses、score-material、flywheel trail）。
- 承 m-pk037-route 先例：near_threshold 位经人节点 confirmation 后可推进终签。人复核选项：一，确认依据族双源真实性即放行终签收约归并；二，裁材料退回重写（新 gid、方向对己不利、谱系披露）。
- 本批诚实申报：采样作答由本席位逐发写出，依据族分配即上述真实理由分布，无挑选调整。

## 四、缺陷与越线申报

1. **closeguard 1.18.0 三源失对齐缺陷**：主树 pyproject 与 __init__ 滞留 1.17.0 而 CONTRACT 已记 1.18.0；本批升 1.19.0 时三源一并修正，缺陷登记如实申报。
2. **meter 计数缺口一笔**：意图重追加（--allow-reintent，facepark 先例合法通道）直调引擎 scribe 未经 meter 包裹，meter counts 少记一笔，如实申报。
3. **租约重开一笔**：首约 069a640d 因任务包请求写入节漏列治理面路径强制关闭（零写入零残留），补列后重开为本会话 eaf80aa8。
4. recall 先导检索零命中如实记；checkcite pass 在档。

## 五、待复核后的收口路径

人节点确认后：执契 check→verify→sign 终签 → 认证 append → 双仓三仓 settle → close 归并 → reconcile → 链 verify → 主树域内核阅复验 → 本档收口附记回填。人裁退回则：命题重写新 gid 走改写链，本档如实记退回事由。

## 六、收口附记（close 后回填）

- 人节点复核：用户 2026-09-05 会话令「同意」逐字入链（confirmation 件 7495c5c9ff28fcce），near_threshold 挂起位放行，修复切换生效。sign 未执行系 near_threshold 机制上仅 stable_clear 可签，人节点裁决即收敛点最终裁决，check pass 与 verify identical 读数在档，原挂起裁决留痕不抹。
- 本批收口即修复归并主树在役。
