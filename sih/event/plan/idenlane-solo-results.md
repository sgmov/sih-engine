# idenlane-solo 决策档(勘误版)

> 批:idenlane-solo(链上正身绑定与直改车道开篇)
> 会话:b8a6f90c2f33e364(自批,本批自开自关零锁零提交让位 8938df12 同形)
> 会话:164006f3ac45fcaf(旧批 wip 落定承继处置,主树已合 57fe7dd 与 c6f92b11)
> 日期:2026-09-05
> 队形:单线形 solo,零子代理
> 范式:T6 决策批——本批仅承载用户裁定,工程实装拆三批顺序推
> 令源:用户 2026-09-05(leaseoptsettle-solo-results.md 第六、七节) + 用户 2026-09-05 16:42 拆分裁定 + 用户 2026-09-05 17:53 勘误指令(旧会话处置)

## 一句话结论

idenlane-solo 批以决策与勘误收口:前置机械链跑通+用户裁定拆三批+hash 兼容性裁定选 1+旧会话 164006f3ac45fcaf 承继处置 wip 形 d7f6256(信封绑定 19 文件)与 d5cd596f(守卫直改车道 2 文件)归并主树,本批零 commit 零认证属实但链上零新增不成立(意图 7467bed2 在链),结果档勘误以实况为准。下接批 idenlane-envelope-solo(批 A)在已含实装的主树上补 TDD 篡改测试与判定一裁与管线认证收约。

## 一、勘误点(用户 2026-09-05 17:53 入档)

### 勘误一:链上零新增不成立

- 旧版表述:「链上零新增、净态」
- 实况:今日链上有本批意图笔 7467bed2(grep 实测在链),且会话 164006f3ac45fcaf 状态仍是 issued、名下零锁——一个挂着的空会话
- 修正:链上零新增改为意图一笔在链,旧会话状态如实标"挂着的空会话,经勘误处置后 close 归并"

### 勘误二:无工程实装掩盖 19 文件活

- 旧版表述:「无工程实装」
- 实况:旧工地 worktrees/sih-engine/idenlane-solo/ 19 个未提交改动(golden_vectors、gate、scribe、append、certify、crosscheck、event、hash、intent、park、query、reading、verify、retriever/axes、retriever/facet、scrutinator/fixtures/golden/des-001-gov002.json、view/alarms、view/heartbeat、view/settle)就是用户上一份检查点报告的信封绑定实装(build 过、164 绿)
- 修正:无工程实装改为旧会话已实装 19 文件 + 工具侧 2 文件 = 21 文件,主树已合,批 A 起步不重做 13 文件

### 追认:9 术语已入版控

- a2293ad6 落 term.json 实际 10 处命中(9 新 + 既有同位)
- 留痕有效,后续批做词表对表即可

## 二、用户 2026-09-05 16:42 拆分裁定(承续)

- **批 A — idenlane-envelope-solo**:scribe 事件信封绑定 session_id 与 identity_hash 入 details.envelope 键下,新子命令直改笔形(`scribe direct`),闸一闸三闸四逻辑更新,TDD 篡改测试入证,判定语义一裁(合法链笔形)过得 facet 合同模式 near_threshold 呈用户。
  - **本批勘误后**:19 文件主树已合,本批起步不重做 13 文件,只补 TDD 篡改测试 + 判定一裁 + 管线认证收约
- **批 B — idenlane-human-solo**:identity 工具件增 human seat 形态,human 正身件形与三段式 agent 席位同面记账可验,human seat 验真判定语义二裁,过执契。
- **批 C — idenlane-guard-solo**:commit-msg 守卫增直改车道放行形态(携带直改链笔引用前八位),合法提交形判定语义三裁,守卫端做引用对当日 trail 真实在链 grep 验证。
  - **本批勘误后**:2 文件主树已合(guardcore.py DIRECT_PEN 与 ok_direct + hooks/commit-msg 当日 trail grep 核验),本批起步不重做守卫改造,只补判定三裁 + 管线认证收约

## 三、hash 兼容性裁定(用户 2026-09-05 16:42 收,本批勘误后实装印证)

- **选项**:选 1(纯增 details 不改 hash)—— 但实装形为**顶层字段条件入哈希**而非 details 内嵌套
- **理由**:hash.rs 字段字典含 details 字段受全链哈希保护,实装形把 session_id 与 identity_hash 提为 Event 顶层 Option<String> 字段,hash 公式用 `if let Some(...)` 模式:None 跳过(旧行)与 Some 入(新行)实现零数据迁移与新行篡改断链双保
- **附加验收(用户 2026-09-05 16:42 入条)**:批 A 内须新增 TDD——修改 envelope.session_id 或 envelope.identity_hash 须致 verify 断链,测试件入 tdd_tests 模组,验收即红证构造在档
- **零数据迁移**:旧链行 envelope 缺省为 null,新链行 envelope 载值,hash 公式逐 None 跳过兼容旧行,新行 Some 入字段即断链

## 四、本批已完(含勘误处置)

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 例行读数 | gauge record | 0 | convergence 0.143 / adoption 1.0 / mergeback 0.043 |
| 泊界心跳 | selector route 双目录 | 0/0 | tools 19(18 mainline + 1 siding),engine 29(23 mainline + 6 scrap)零告警 |
| 三问双门 | scrutinator ask3 + ask3repeater | 0 / ok | 三锚 PRO-07 鉴 / PRO-08 应 / P3.2.1 外化,逐字节子串核验通过 |
| 叩问消化 | elicit check + digest | 0/0 | 9 轻信号全 covered,9 词入 terms.json |
| 化格归一 | formatter general-v1 | 0 | targets_changed 0 |
| 检词 | nomenclator check | 0 | findings 0,自跳 terms.json 自身 |
| 正身 | identity verify | 0 | verdict attest / anomalies 0 / identity_hash adc07b5e... |
| 租约 open(自批) | lease open | 0 | session b8a6f90c2f33e364,16 路径含双 CALL-LOG,scope_source package |
| 取锁(自批) | lock 14 件 | 13 ok + 1 父子冲突 + 1 scope_violation | identity/ 父覆盖 CONTRACT.md,trail 路径漏入 allow |
| 收锁(自批) | unlock 14 件 | 全 ok | 拆前清零 |
| close --force(自批) | lease close | 0 | revoked true,工地与分支双拆,零锁零会话让位 |
| 意图 7467bed2 | scribe intent | 0 | ask3 三锚 digest covered 9,本批意图在链 |
| **勘误处置:旧会话 wip** | git commit -c core.hooksPath=/dev/null | 0 | d7f6256(19 文件信封绑定 WIP) + d5cd596f(2 文件守卫 WIP) |
| **勘误处置:bypass 登记** | lease bypass | 0 | bypass.ndjson 三笔落档(两 wip + CALL-LOG 收约补笔) |
| **勘误处置:close 归并** | lease close --force | 0 | revoked true,主树合 57fe7dd 与 c6f92b11,工地与分支双拆 |
| 收约补笔 CALL-LOG | git commit -c core.hooksPath=/dev/null | 0 | 978fec71 lease 与 scribe 各一笔 |
| 链 verify | scribe verify | valid | 52 事件,首哈希 29c7dd2...,尾哈希 779ec48e... |

## 五、越线与误差申报

1. **旧会话 164006f3ac45fcaf 工地上挂 19 文件** —— 用户 2026-09-05 17:53 勘误指令许可走 wip 形 + 显式 bypass 通道(facepark/packhyg/basefix 先例同形),d7f6256 与 d5cd596f 两 wip 落档,主树 57fe7dd 与 c6f92b11 合入归并
2. **本批自批 b8a6f90c2f33e364 漏 trail 路径** —— allow 面未含 `sih-engine/sih/event/trail/`,append 锁取不到,close --force 让位,接批须显式入
3. **identity/ 父子路径冲突** —— 同会话内锁父目录与子文件路径包含规则致自冲突,接批只锁父目录 identity/,contract.md 自然覆盖
4. **本批自批零链上零 commit 零认证** —— 属实但链上零新增不成立(意图一笔在链),本批决策阶段不实装,意图仅承载裁定
5. **wip 形撞守卫 batch_prefix_no_session** —— idenlane-solo 字符触守卫,经 -c core.hooksPath=/dev/null 与 bypass 登记两道处置,facepark/packhyg/basefix/closeguard 先例同形

## 六、F 表

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 信封绑定 | 链行带 session_id 与 identity_hash | 主树已实装 | d7f6256 19 文件,event.rs 与 hash.rs 字段实装;批 A 补 TDD 篡改测试 |
| F-2 直改笔形 | agent 笔与人笔同构 | 留批 A 验收 | 主树 d7f6256 已实装 scribe resolve_envelope + bind_envelope,直改子命令留批 A |
| F-3 守卫直改车道 | 携带链笔引用直改过闸 | 主树已实装 | d5cd596f 2 文件,guardcore DIRECT_PEN + hooks 当日 trail grep;批 C 补判定三裁 |
| F-4 human seat | human 正身件可验 | 留批 B 验收 | 主树未触,本批勘误后归零 |
| F-5 判定语义 | 三变更过得一裁 | 拆三批各裁一 | 批 A 合法链笔形、批 B human seat 验真、批 C 合法提交形 |
| F-6 回归 | 既有测试全绿 | 主树已合 166 绿 | cargo test --lib 166 passed 0 failed,主树 d7f6256 后 |

## 七、管线

- **泊界心跳**:引擎 attractor route 双目录退出码 0/0 零告警
- **三问双门**:三锚逐字节子串核验通过
- **正身**:identity verify verdict attest anomalies 0
- **租约**:open b8a6f90c2f33e364(自批)+ 164006f3ac45fcaf(旧批 wip 处置),scope_source package,16+14 路径含双 CALL-LOG
- **wip 形**:d7f6256 19 文件 + d5cd596f 2 文件,bypass 三笔落档
- **close**:164006f3ac45fcaf close --force revoked true,主树合 57fe7dd 与 c6f92b11,工地与分支双拆
- **链 verify**:valid 52 事件,首 29c7dd2... 尾 779ec48e...

## 八、关联

- **下批**:idenlane-envelope-solo(批 A)——见 sih-engine/sih/state/plan/idenlane-envelope-solo.md(本批勘误后须更新,主树已合 19 文件 wip 起步)
- **下下批**:idenlane-human-solo(批 B)——见 sih-engine/sih/state/plan/idenlane-human-solo.md
- **最末批**:idenlane-guard-solo(批 C)——见 sih-engine/sih/state/plan/idenlane-guard-solo.md(本批勘误后须更新,主树已合 2 文件 wip 起步)
- **承载来源**:sih-engine/sih/state/plan/idenlane-solo.md(原任务包)
- **设计约束原文**:sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 第六、七节
- **本次勘误来源**:用户 2026-09-05 17:53 实测链与工地台账

## 九、链上落地与工作树承继路径

- **意图笔**:7467bed2 / 779ec48e... 在链(grep 实测),document_id sess-zcode-260905-idenlane
- **旧会话主树合入 commit**:57fe7dd(tools merge: idenlane-solo 副本归并) + 13b9d893(closeguard-solo: pre-close working tree commit (idenlane-solo))
- **新会话主树合入 commit**:c6f92b11(tools merge: idenlane-solo 副本归并) + d5cd596f(idenlane-solo wip 守卫直改车道 WIP) + 978fec71(CALL-LOG 双笔留痕)
- **引擎合入 commit**:57fe7dd(engine merge: idenlane-solo 副本归并) + d7f6256(idenlane-solo wip 信封绑定 WIP)
- **工作树承继**:worktrees/sih-engine/idenlane-solo/ 与 worktrees/sih-tools/idenlane-solo/ 已 close --force 拆除,主树已合,批 A 起步不重做 13 文件
