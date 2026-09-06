# confledger-solo 结果档：置信度台账实装批（阶段二第一步）

> 承接：任务包 confledger-solo.md 与用户 2026-09-07 令「下一步工作出提示词和任务包」即阶段二开工令。单件：confledger 工具（工作名，meter 先例不立名）按推导档 v3 实装，T₀ 启用笔上链，真账本激活数据面开跑；休眠门全关，抢占业务环零实装。
> 队形单线形 solo（委外执行），日期 2026-09-07，会话 sess-zcode-260907-confledger（session_id 4f85a0ac5a1c3b2c，双仓租约 sih-tools＋sih-engine）。前轮排队候叫约十八分钟无窗口，本轮窗口开再启，零绕行。

## 意图锚定

- 意图事件：intent_refined（event_hash `a8851a11...`）
- record：sih-tools/scribe/reports/2026-09-07-ask3-confledger-solo-record.json（三锚引文程序切片：01-ontology-of-names.md L18 承载不撤回、07-on-assay.md L55 鉴只列事实、08-on-settle.md L110 应而不藏）
- validation：sih-tools/scribe/reports/2026-09-07-ask3-confledger-solo-validation.json（status ok，anchor_count 3）

## 核心交付：confledger v0（备忘录式）

- 工具位：sih-tools/confledger（围堰孵化，判据四条，工作名不立名）；八子命令 open／activate／mint／pay／balance／verify／replay／bills；stdlib 零第三方依赖。
- 台账本体（F-1）：单一账本 ndjson 追加式，事件行 v3 L(t) 五字段加 identity_hash 双键位（账户键 core_hash、流水笔携 identity_hash）；写点 flock 串行加单次 os.write 原子整行（pk057fix 先例）；ref 挂链上事件哈希。
- 铸币触发器（F-2）：mint_clean_close 与 mint_active_pen 各 +1（v3 已定值），ref 须为链上事件哈希且经 --trail 机械见证；mint_repair_foreign 槽位态休眠拒（D_acc 待实证）；mint_self_repair 拒（A2 自产自修不铸币）；字面 grep 零 LLM 零网络零主观打分。
- 支付算子（D3/D10）：fail-closed（不足即拒零部分支付零写入）、即付不退（无退款算子）、人席位拒（A_mech 域）、配对守恒（pay_out／pay_in 同 ref 双行）；抢占业务环零实装仅留算子接口位。
- 休眠门控（F-5）：结算、罚金、D6 分段带与递延、I3 全零触发；常数面 D_ACC／U_PLUS／U_MINUS／W／N_MIN／M_REPAIR 全 None 槽位态；账单面只读摘要（bills 子命令）零改租约退出码。
- 常数面（F-4）：constants.py 仅 M_CLEAN=1 与 M_ACTIVE=1（v3 已定值）加六槽位 None；cli/ledger 无两位数以上字面常量（grep 取证）。

## TDD 与验收（F-3）

- 先红后绿：13 测试先红（RED exit 2，红证 tests/red/tdd-red.log sha256 91c2ca66...）后绿（13 passed）；绿程中两笔迭代如实申报（balance 单账户契约形修正、夹具纪元早于真实 UTC 现在的 grandfather 假排除修正）。
- acceptor 判定包（tests/acceptance-pack.json，词汇表内三查零私扩）：C1 red_then_green pass（红证哈希对表）、C2 double_run pass（replay 同参双跑逐字节一致）、C3 baseline_freeze pass（replay-demo 冻结件对表零漂移）；acceptor exit 0 findings 空。
- 三不变量测试在档：I1 下限零（透支拒恒非负）、I5 fail-closed（拒后零写入）、I7 grandfather（T₀ 前零贡献零追溯）。

## T₀ 启用笔与数据面开跑

- 启用笔：direct_edit_completed event_hash `63d4f48664f2...`（2026-09-06T23:12:00.034390Z UTC），agent 笔携正身件，无会话外申报零——本批会话内在册。
- 真账本：sih-tools/confledger/ledger/confledger.ndjson 首事件 activation（t0_ref 即启用笔 event_hash，t0_ts 即笔时间戳——T₀ 由链上事件定义非人选数）；第二事件 account_open 开本批 agent 席位账户（core_hash 82f460c2...）；verify valid 零 findings。
- 账单只读摘要（标定数据面自此攒量）：lockface-bills 现存 349 笔 875 点（lock_charged 291／lock_free 28／open_face_bill 30／unused_lock_penalty 16），阶段一零追溯（A5）不入台账余额。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 台账形态承规格 | 数据治理 | 事件五字段与追加式与 ref 挂链逐项对 v3 L(t)，双键位测试在档 | 通过（ledger.py 事件形与 flock 写点；双键位测试 test_real_identity_report_core_hash 与流水 identity_hash 字段） |
| F-2 铸币纯机械 | 治理 | 三类判别与 v3 一致，自产自修不铸币有测试，grep 零 LLM 零网络零主观打分 | 通过（触发判别三测试；字面 grep 零命中取证在材料） |
| F-3 不变量红绿 | 跨族治理 | 三不变量先红后绿证在档 | 通过（红证 91c2ca66＋绿 13 passed＋acceptor C1） |
| F-4 常数零新 | 治理 | 消费面仅 v3 已定值，槽位态零新数值 | 通过（constants.py 逐值测试＋grep 取证） |
| F-5 休眠不越权 | 治理 | 休眠门全关零触发；抢占零实装（支付算子除外）；零改租约退出码 | 通过（D6/罚金/I3 零代码路径；bills 只读；租约仓零触碰） |

## 管线与对表读数

- 化格核阅检词：本批 md 产出件（结果档）随管线实录补记；源码件不走文规管线（工具代码，TDD 加判定包承载验收）。
- 书单对表：拼接扫描形扫结果档（批引用产出），读数随实录补记。
- 认证清单：ask3 记录与验证件与正身件三笔（读数随认证实录补记）。

## 越线与误差申报

- 绿程两笔迭代：balance 单账户契约形首版与测试取用形不一致（KeyError 即红腿），统一为扁平形；夹具 activation 纪元（09-07T00:00Z）晚于真实 UTC 现在（09-06T23:xxZ）致真账事件被 grandfather 假排除，纪元改 2026-01-01——两笔均测试先红改后绿，零静默。
- pyproject 首版缺 project.name（uv 解析拒 exit 2）：补 name 即过，acceptor C1/C3 首跑该错如实记档。
- mint 的 identity_hash 字段在机械触发形置 64 零串（铸币是机器触发非身份行为，操作者身份在 ref 指向的链事件内）——口径如实申报候后继批对表。
- 其余零越线零申报。

## 结算读数

- 待补（收约后经补笔通道回填）。
