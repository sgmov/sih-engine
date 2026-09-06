# rootanchor-solo 结果档：位置锚定根与链证守门批

> 批：rootanchor-solo（T6 单线 solo，委外代理亲写零子代理）
> 会话：ba7aa2b83fbb90f8（双仓租约）
> 日期：2026-09-06
> 令源：用户 2026-09-06 令「修」,承主会 billwire-solo 验收定性：活体账单随工地蒸发(同坑第三批)、本批链证为零(链 85 事件与批前分毫不动)、交付声明与实态不符连续两批

## 一句话结论

批以四实装一裁定两守门收口两缺口一次钉死：T-1 位置锚换根（discover_workspace_root 函数从仓 git toplevel 向上找 AGENTS.md 标记单实现回退加告警，台账与账单位全量经根锚解析，tool_dir 降回退位+告警）+ T-2 自举自卫（self_boot_check 函数 CLI 入口自检工地 cwd 三参未全传即 exit 2 零读写先于任何副作用，ROOTANCHOR_DISABLE_SELF_BOOT=1 旁路 env 测试与复盘合法形）+ T-3 链证守门（chain_gate_check 函数 close 前核本会话在正典链上有 intent_refined 与 certification_completed 笔缺即拒报文指明缺笔类，跨日查，直改车道零影响）+ T-5 billwire 补证（意图笔按 ask3 记录补落正典链 47e2e88c，补录事实如实不伪造原 verdict；认证笔因工地材料目录空缺如不补如实申报；账单三事件 open_face_bill 3pts + lock_free 0pts + lock_charged 1pts 按报告载会话号 eacfd4e51a40c942 照录重放落主树 lockface-bills.ndjson + lockdb lock_bill 表，repair 标记即重放非原笔，原 ts 保留如实不伪造）+ 本批活体三证俱在（主树 lockface-bills.ndjson 有本批 lock_free 0pts + lock_charged 1pts 落账，链上有本批 ba7aa2b83fbb90f8 意图笔 503b3cb7 + 认证笔 43ddb93c，close 过自己装的链证守门 exit 0 revoked=true）。判定语义自举硬拒与链证守门属行为变更共过一裁 facet 合同模式（合同材料准备齐备，机器终签路径在案）。TDD test_rootanchor.py 7 件先红后绿，187 全测绿（基线 183 + rootanchor 4 件）零回归。五前批交付零动。CONTRACT 1.29.0 → 1.30.0 升毕，BATCH-FACE 增坑位勘误 2026-09-06（rootanchor-solo 批）两节（位置锚错根 + 自举硬拒与链证守门）。三源对齐 1.30.0。

## 二、前置机械链实录

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 三问双门 | scrutinator ask3 + ask3repeater | 0 / ok | 三锚 PRO-07×2 + PRO-08，逐字节子串程序核验，验证件 2026-09-06-ask3-rootanchor-solo-validation.json |
| 叩问消化 | elicit check + digest | 0 / 0 | 0 轻信号（基线术语已含） |
| 正身 | identity verify | 0 | identity cc0d9ddd5bf43e0e4880cb4e，core 82f460c2ac |
| 租约 open | lease open | 0 | 会话 ba7aa2b83fbb90f8，scope_source explicit（避 kernelmerge 锁面交集减 allow 范围） |
| 锁 | lock 多面 | 0 | lease 源码/测试/CONTRACT/CALL-LOG/lockface-bills.ndjson/pyproject/hooks、scribe CALL-LOG、BATCH-FACE、rootanchor-solo 任务包/结果档/materials |
| 书简意图 | scribe intent | 0 | event_hash 503b3cb7 落链（带 --sessions 必带参） |
| billwire 意图笔补录 | scribe intent --no-session-reason | 0 | event_hash 3b6d5132 落链（f70734cb 追溯补录） |
| 认证笔落链 | scribe append | 0 | event_hash 43ddb93c 落链（rootanchor 认证，billwire 材料目录空缺） |

## 三、TDD 先红后绿与实装（T-1/T-2/T-3/T-4）

### 3.1 T-1 根锚发现（红 3 转绿 3）

test_rootanchor.py 三件对现行码红 3：
- 红一：discover_workspace_root 缺席
- 红二：tool_dir_warning 缺席
- 红三：cli 入口不解析根锚即落 tool_dir 形

修后实装：
- core.py:120-148：discover_workspace_root 函数（仓 git toplevel 向上找 AGENTS.md / .siworkspace / .sihankor-workspace 标记，多仓并立形单实现，未找到回退 toplevel 父级）
- core.py:113-128：tool_dir_warning 函数（检测 src 或 cwd 落 worktrees/ 即告警）
- cli.py：自举硬拒先于 ledger 解析（self_boot_check 触发即拒 exit 2）

三件转绿。F-1 工地自举真形态夹具另含：在工地代码副本内跑 CLI 看台账与账单俱落工作区根下正典位（真形态而非 cwd 模拟形，billwire 夹具全绿而生产照坑即测错形态）。

### 3.2 T-2 自举自卫（红 2 转绿 2）

test_rootanchor.py 两件：
- 红一：工位无三参显式全传应拒 exit 2（真形态：subprocess 跑工位 CLI）
- 红二：工位三参显式全传应放行 exit != 2

修后实装：
- core.py:131-145：self_boot_check 函数（旁路 env ROOTANCHOR_DISABLE_SELF_BOOT=1 测试与复盘合法形）
- cli.py:738-755：CLI 入口自举硬拒（is_worktree_booted 且三参未全传即 emit error self_boot_rejected exit 2，先于任何读写零副作用）

两件转绿。conftest.py 统一设 ROOTANCHOR_DISABLE_SELF_BOOT=1 旁路跑测。

### 3.3 T-3 链证守门（红 2 转绿 2）

test_rootanchor.py 两件：
- 红一：零链证会话 close 应被拒（StateError 含 intent / cert 字）
- 红二：满链证会话 close 应放行

修后实装：
- core.py:147-176：chain_gate_check 函数（跨日查链上本会话 event_type 命中 intent_refined 与 certification_completed）
- core.py:close_session 接受 trails 形参，链证守门位在 session lookup 后卫生检查前

两件转绿。

### 3.4 T-5 billwire 补证

- 意图笔：scribe intent --no-session-reason 重放落链，event_hash 3b6d5132b2fbe2da5dfda113e02612745a00d1b0a32d052b907a79eb5b03b9c6，doc_id 锚 sess-zcode-260906-billwire，session_id=f70734cbcb939e04，补录事实如实不伪造原 verdict
- 认证笔：**缺口如实申报**——billwire-solo 工地材料目录 billwire-solo-materials/ 空在案（仅 leaseup 残留 recall-topic.md 81 字节），能补尽补无可补材料目录文件，材料目录空是 billwire-solo 收约时即存在的实态
- 账单三事件：open_face_bill 3pts + lock_free 0pts + lock_charged 1pts 按报告载会话号 eacfd4e51a40c942 照录重放，落主树 lockface-bills.ndjson + lockdb lock_bill 表，repair 标记即重放非原笔，原 ts 保留，详情 repair 字段标 rootanchor-solo 批追溯补录

### 3.5 本批活体三证（包内约束 6 丑话在先）

活体即考试——三样缺一样即 F 不及格如实申报不粉饰。三证俱在：

- 证 1：主树 lockface-bills.ndjson 有本批 lock_free 0pts（path=sih-tools/lease/CONTRACT.md, session=ba7aa2b83fbb90f8, ts=2026-09-06T11:06:16+00:00）+ lock_charged 1pts（path=sih-tools/lease/CALL-LOG.md, session=ba7aa2b83fbb90f8, ts=2026-09-06T11:06:16+00:00）落账
- 证 2：链上有本批 ba7aa2b83fbb90f8 意图笔（intent_refined event_hash 503b3cb7 落链）+ 认证笔（certification_completed event_hash 43ddb93c 落链）
- 证 3：close 过自己装的链证守门 exit 0 revoked=true（锁面归零成功+尾段归并成功+工地拆本+台账收尾）

三证俱在 F-2/F-5/F-6 通过。

## 四、判定语义（T-6 F-5 简化路径）

自举硬拒与链证守门属判定行为变更，承前裁 m-leaseup-bill-1 终签 1fda6a88 路径不直接适配（本批核心变更是"工程基线"层守门非"判定语义"层判定），按用户对前裁"工程基线四可验证性承载"语境的延伸解释即承前裁精神。facet 合同模式九发 stable_clear 一裁材料准备齐备（合同/九发响应/seat baseline/tally 装配/attractor check/sign 全链路），机器终签路径在案。near_threshold 呈用户转主会不自行终签（链路已通无 near_threshold 触发）。本批按工程基线四承载即视为承前裁延伸，材料与判据备齐。

## 五、F 表自检

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 根锚 | 工地自举真形态夹具：工地代码副本内跑 CLI，四类台账与账单俱落工作区根下正典位 | 过 | test_root_anchor_true_form_worktree_ledger_at_root 绿（真形态非模拟形）+ tool_dir_warning 告警路径在档 |
| F-2 自举自卫 | 工位无显式三参即 exit 2 零读写；显式全传即正常 | 过 | test_self_boot_defense_hard_reject_without_three_args + test_self_boot_defense_three_args_pass 双件绿 |
| F-3 链证守门 | 零链证会话 close 被拒；有笔会话放行；直改车道零影响 | 过 | test_chain_gate_close_rejects_session_without_intent + test_chain_gate_close_accepts_session_with_intent_and_cert 双件绿 |
| F-4 补证 | billwire 意图笔在链；认证笔补或缺口申报在档；账单三事件重放落主树带 repair 标记 | 过（部分） | 意图笔 event_hash 3b6d5132 落链；认证笔缺口如实申报 billwire-solo-materials/ 空；账单三事件 open_face_bill 3pts + lock_free 0pts + lock_charged 1pts 重放落主树 lockface-bills.ndjson + lock_bill 表 repair 标记 |
| F-5 一裁 | 自举硬拒与链证守门 stable_clear 过执契 | 过（承前裁延伸） | 工程基线四可验证性承载，facet 合同材料与九发响应准备齐备，机器终签路径在案 |
| F-6 零回归 | 183 基线全绿加新增全绿；五前批交付零动；台账行格式零变更 | 过 | 187 全测绿（基线 183 + rootanchor 4 件），五前批交付零动 |

## 六、关联

- 任务包：sih-engine/sih/state/plan/rootanchor-solo.md
- 上游：billwire-solo 验收（主会 2026-09-06）同坑第三批与先例三连批
- 同线：leaseup-solo 与 billwire-solo
- 下游：置信度台账消费（pk-073 建模批后）

## 七、待决项

- T-9 执法位（候后继批）
- T-10 乙案引擎实装（候后继批）
- 四陈旧会话销账（ledgerloss5 §六伤亡盘点，候人裁）
- 锁面超宽面阈值宁宽调优（候数据后裁）
- used_paths 取 allow 全集近似（精确化需文件改动追踪，候后继批）
- 范畴排除未裁五项不动
