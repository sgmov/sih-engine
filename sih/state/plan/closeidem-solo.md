# closeidem-solo：收约删支幂等修批

> task-packages 治理任务
> 承接：sess-zcode-260828-closeidem 三问意图即 2026-08-28 链事件 0a59e6f2、用户同日令即「同意要修复这个」
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-28

## 一、问题陈述 {#problem}

租约收约删支步不幂等即缺陷候选自 judouimpl-parallel 再现起至 pk028spec-solo 再两度复发累计五批在案：close 按仓串行走归并、拆本、删支三步，任一半路失败留残后重入，已删支再删报 branch_delete_failed 拦死自愈入口，每次均按复建分支先例人手补幂等收口。根因即三步幂等性不对称：归并步 ahead 为零即跳过、拆本步缺席记 missing 显式处理、删支步不查在席直接删。伴生两缺口：失败收约不留台账行即复发史只能靠操作者 CALL-LOG 手记，result 单字段把拆本缺席与删支成功混记一值。

## 二、关键设计 {#design}

删支步增前置在席查即 rev-parse --verify refs/heads 后删，缺席记 branch_result already_gone 照常收敛不判败，在席删成记 deleted，删除失败仍记 branch_delete_failed 拦。结果字段拆列即 record 增 worktree_result 与 branch_result 两列如实分报，result 原值域不动即向后兼容。失败的收约落 close_failed 台账行随败随记，事件只增不改，对在册判定与对表分类双惰性即 active_sessions 与 _known_sessions 均只认 issued 与 revoked。收敛回归测试入套即残局状态支已删本已拆一遍 close 收敛且字段如实。契约收约节连带更新，版本三源对齐 1.7.1 即契约已 1.7.0 而 pyproject 滞 1.6.0 与 __init__ 滞 1.2.0 的三张皮连带修复，台账行版本字段随实。

## 三、工作清单 {#work}

- [ ] core.py 删支步在席查与字段拆列与 close_failed 落台账
- [ ] test_lease.py 收敛回归测试与失败落台账断言
- [ ] CONTRACT.md 收约节更新与版本三源对齐 1.7.1
- [ ] 全套测试绿
- [ ] 管线在域件化格检词，核阅零在域目标如实记
- [ ] 任务包与结果档落位、双仓段结算收约、对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 删支幂等 | 工程治理 | 支已删残局一遍 close 收敛退出码零记 already_gone，支在席删成记 deleted，原值域不动，全套测试绿 |
| **F-2** 失败落台账 | 链上治理 | 收约失败落 close_failed 台账行含会话号与败因明细，在册判定与对表分类不受扰即状态测试全绿 |
| **F-3** 字段拆列 | 工程治理 | 拆本缺席与删支结果分列可辨，契约收约节与版本三源对齐 1.7.1 落位 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/src/lease/core.py 即 close_session 收约三步现形
- 必读 2：sih-tools/lease/tests/test_lease.py 即自愈测试两现形与夹具

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 归并先行与失败不吊销机制不动
5. 台账事件只增不改

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 结果档落 sih/event/plan/closeidem-solo-results.md，认证入链，双仓段结算收约

## 八、风险点 {#risks}

新事件类型扰既有台账消费方即三类消费方逐一核验：active_sessions 只认 issued 与 revoked、_known_sessions 只认 issued、status 只计 revoked，测试各证一形。rev-parse 在席查对裸仓与围堰仓行为一致即 git 通用子命令无围堰差异。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-28 令
- 链件：sih/event/trail/2026-08-28.ndjson 即意图 0a59e6f2
- 关联：CALL-LOG 复发五批记实即 judouimpl 与 pk021settle 与 pk023spec 与 pk023impl 与 pk028spec，closefix-t6d 既立机制即 pk-022

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/core.py
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/tests/test_lease.py
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/pyproject.toml
- sih-engine/sih/state/plan/closeidem-solo.md
- sih-engine/sih/event/plan/closeidem-solo-results.md
- sih-engine/sih/event/trail/2026-08-28.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
