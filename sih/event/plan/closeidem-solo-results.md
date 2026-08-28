# closeidem-solo：收约删支幂等修批结果档

> 承接：closeidem-solo 任务包、意图 0a59e6f2
> 队形：单线形 solo 即主线亲写零子代理
> 日期：2026-08-28

## 一、总判 {#verdict}

F-1 至 F-3 全过。收约删支步幂等化即删前 rev-parse 查 ref 在席、缺席记 already_gone 照常收敛不判败，重复删支不幂等缺陷修除。结果字段拆 worktree_result 与 branch_result 两列如实分报。失败收约落 close_failed 台账行随败随记。版本三源对齐 1.7.1。三十五测全绿。

## 二、F 判据 {#f}

- F-1 删支幂等：新测 test_close_branch_already_gone_converges 即残局重演一遍收敛退出码零，重演形即重开拆本删支后单跑 close 收敛记 missing 与 already_gone；在席删成记 deleted 即同测首段与 merge_blocked 自愈测增断言双证；result 原值域不动向后兼容；全套三十五测绿即原三十四加新测一
- F-2 失败落台账：close_failed 事件随败随记含会话号与败因明细与 removed 部分结果，merge_blocked 自愈测增台账断言；双惰性核验即 active_sessions 只认 issued 与 revoked、_known_sessions 只认 issued、status 只计 revoked 三消费方逐一在案，全套状态与对表测试绿即实证
- F-3 字段拆列：worktree_result 与 branch_result 两列分列可辨，拆本缺席与删支结果不再混记一值；契约收约节更新即删支幂等与拆列与 close_failed 三件入文；版本三源对齐 1.7.1 落位即契约与 pyproject 与 __init__ 同值

## 三、发现与披露 {#findings}

一版本三张皮即主线契约已升 1.7.0 承撤gate 批修订十三而 pyproject 滞 1.6.0 承封窗批升版漏与 __init__ 滞 1.2.0 更早，台账行版本字段一直记 1.2.0 即以 __init__ 为实，本批三源对齐 1.7.1 台账行版本随实。

二 active_sessions 按会话号弹 revoked 不察时序即同号重开会被旧 revoked 行误抹，生产以真钟时戳入号不撞即未构成活缺陷，测试夹具固定时戳撞出即重开换时戳绕行并在测注记，登记缺陷候选不修待议。

三置信度联动调查即用户同日问：lease 与 identity 源码与契约 trust 与置信度与信任分零命中即无联动，租约现联动的是正身报告双腿即 open 必附与 lock 时验读、哈希入台账非原值，旧 MCP 信任分承功能承载映射明文未承接，若要联动属新能力另立项待令。

四夹具物理一课即分支被副本检出时 branch -D 拒删，残局重演须先拆本再删支与真实半途失败残留形态一致即测试如实。

## 四、结算 {#settle}

- 引擎段：任务包与结果档一段结算
- 工具段：core 与测试与契约与版本三件一段结算
- 认证：两笔即包检词报告与结果检词报告落链，双仓段结算取结果证
- 对表：双仓对表随收约落即预期 unrouted 零与 cert_missing 各二既认账副件链噪音
- 后续待令：租约 SDD 融回批余四证据即 commit 须指围堰与迁链路标机械位与围堰相对根测试与免参不含副件链；本缺陷修后自该批证据单除名
