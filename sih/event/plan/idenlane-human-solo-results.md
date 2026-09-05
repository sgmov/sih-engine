# idenlane-human-solo 收口档

> 批:idenlane-human-solo(human seat 正身形态 + 判定语义二裁)
> 会话:be316fe3596bf16e
> 日期:2026-09-05
> 队形:单线形 solo,委外代理亲写零子代理
> 范式:T6 单线 solo
> 令源:用户 2026-09-05 拆分裁定(idenlane-solo-results.md 第二节批 B 项) + 任务包 sih-engine/sih/state/plan/idenlane-human-solo.md

## 一句话结论

idenlane-human-solo 批 B 收口:human seat 验真五闸面 TDD 先红后绿实装(19 新测与 49 旧测合计 68 绿零回归),identity verify 增 --human-seat 旗标升 0.5.0,判定语义二裁 facet 合同模式九发 stable_clear 过执契(attractor check 12 项 pass,verdict=pass,direction=comply,终签 crosscheck dd210f69 在链),红线六件逐字节零动,CONTRACT 修订九随批,双仓 settle 收约,链 verify valid。

## 一、工作清单(全数达成)

- [x] T-1 红证:缺 seat 字段即拒 — test_human_seat.py::test_t1_missing_seat_rejected PASS
- [x] T-2 红证:seat 非 "human" 即拒 — test_human_seat.py::test_t2_wrong_seat_value_rejected PASS
- [x] T-3 红证:缺 name 即拒 — test_human_seat.py::test_t3_missing_name_rejected PASS
- [x] T-4 红证:缺 attest_at 即拒 — test_human_seat.py::test_t4_missing_attest_at_rejected PASS
- [x] T-5 绿:合法人类席位验真即过 — test_human_seat.py::test_t5_valid_seat_attests PASS
- [x] T-6 identity 工具件增 `--human-seat <文件>` 旗标 — cli.py 实装,帮助词在场测试 PASS,退出码三值语义不变
- [x] T-7 判定语义二裁(facet 合同模式) — 九发 stable_clear 过执契,见第三节

## 二、human seat 件形与五闸(实装形态)

件形(独立于 v3 身份串):

```json
{
  "seat": "human",
  "name": "<人类席位名称>",
  "attest_at": "ISO8601",
  "attest_subject": "事由(可选)",
  "guardian": "<裁决人标识或同会主裁>(可选)",
  "fingerprint": "可选,键盘布局 keyboard_layout / boottime / 链路代号 link_code 三形任一"
}
```

五闸判定语义(verify_human_seat,core.py 尾节):

- 闸一:验真件须为合法 JSON 对象,非对象即拒
- 闸二:seat 字段必为 "human",缺席或他值即拒
- 闸三:name 与 attest_at 必填,缺席或空串即拒
- 闸四:guardian 在场即人节点已签字(签字位只记状态不拒)
- 闸五:fingerprint 三形任一在场即指纹登记(登记位只记状态不拒)

前闸拒后闸记:闸一至闸三任一不过 verdict=reject 退出码一;合法件 verdict=attest。判定常数命名登记零裸奔:HUMAN_SEAT / HUMAN_SEAT_REQUIRED / HUMAN_SEAT_OPTIONAL / FINGERPRINT_FORMS。报告增 human_seat 顶层块与 summary.human_seat_verdict 镜像,quiet 挂旗标时增 human_seat_verdict 键,无旗标形逐字节不变(回归测试在册)。验真件非法 JSON 与文件不可读属工具异常退出码二。

红线六件逐字节零动:v3 身份串、盐哈希、new_salt、COMPONENT_ORDER、identity_hash 字段、core_hash 字段。金标 core_hash 与 legacy 40 报告复算回归测试在册全绿。

## 三、判定语义二裁(T-7,F-5b)

- 命题:gid m-human-seat-1,human seat 验真五闸判定语义命题:人类席位正身件形与五闸验真的相容性(ng medium,n 9)
- 出题半:contract.json emit 九发,零 LLM 零网络
- 采样:ZCode:GLM-5.3-Flash:self-reported 九发原文回填,decision 全 comply,变卦 0%,谨慎信号 0/9,boundary_rate 0.0,基线引用单一 baseline_1
- 判据 v3 闸:stable_clear,四子判据全过(decision_stable / boundary_low / basis_consensus / foregrounding_stable),dc_fingerprint 2b100f3030f3f263
- 当日席位基线:temp_probe_agent 四命题二十发,verdict 可用,身份哈希与本批正身件配对(0e182797f0a19204...),core_hash 金标 82f460c2ac2741fc...
- 执契:attractor check R1-R7 十二项全过 verdict=pass direction=comply;attractor sign 机器终签落据 m-human-seat-1-signcheck.json,crosscheck_completed 终签笔 dd210f69 在链(doc_id crosscheck-m-human-seat-1);attractor verify 重放 identical
- near_threshold/boundary:不触发,无呈用户事项

## 四、F 表

| F 锚定 | 类别 | 判据 | 结论 | 证据 |
|---|---|---|---|---|
| **F-4** | human seat | human 正身件可验可与 agent 席位同面记账 | 过 | 五闸 TDD 19 测绿;--human-seat 旗标实装;件形与三段式席位同面记账由 envelope 三分命名承载(五节风险对表);scribe direct human 笔接口位(主树 d7f6256 已留)消费同形验真件 |
| **F-5b** | 判定语义二裁 | human seat 验真规则,facet 合同模式九发 stable_clear 过执契 | 过 | 第三节全证:stable_clear + check 12 项 pass + sign dd210f69 在链 + verify identical |

## 五、envelope 三分命名风险对表(任务包第八节)

human seat 与 v3 身份串在 envelope 字段同存的命名冲突,以三分域互斥化解:

- `envelope.session_id` — agent 会话(租约会话号入链)
- `envelope.identity_hash` — agent 全席位(正身报告 identity.hash 入链)
- `envelope.human_seat` — human 节点(本批命名登记位)

本批写入面(任务包第十一节)不含 sih-engine 源码,`envelope.human_seat` 字段落 Event 结构属 scribe 侧工程件改造,留后续批按管线承载;本批落位的是命名登记、human seat 验真件形与 --human-seat 旗标(消费位接口契约:scribe direct human 笔已按 seat=="human" 与 name 与 attest_at 三闸先验,与本批五闸面前三闸同义,后两闸面 guardian 与 fingerprint 为五闸超集兼容)。字段冲突排除:三分域名互斥,agent 会话号与正身哈希与 human 节点三值域无交,设计不冲突。human seat 无 anchor 验证(无 v3 串)的作伪风险由 guardian 字段对冲(在场即人节点签字)。

## 六、测试与回归

- TDD 序:先红(19 新测 ImportError 即实现缺位)后绿(实现后全绿)
- 全族:68 passed(49 既有零回归 + 19 新增),`uv run python -m pytest tests/ -q`
- 新增组:test_human_seat.py 十九测,红证四件 + 绿证一件 + 闸一补充非对象拒 + 闸四闸五状态面四件 + 空串必填拒 + CLI 接线四件(绿/reject/非法 JSON/缺席文件) + 帮助词在场 + 常数登记 + 无旗标零块回归
- 版本:0.4.0 → 0.5.0(__init__.py 与 pyproject.toml)

## 七、管线与链

- **例行读数**:gauge record 三维落链 convergence 0.714286 / adoption 0.833333 / mergeback 0.043478
- **泊界心跳**:selector route 双目录退出码 0/0 零告警(tools 19 件,engine 35 件)
- **三问双门**:ask3 三锚(PRO-07 鉴 / PRO-08 应 / P3.1 注意力稀缺)逐字节子串核验,scrutinator ask3 findings 0,ask3repeater ok
- **叩问消化**:elicit check 四轻信号(五闸/验真件/人类节点/指纹登记),digest passed covered 4,处置入 ask3 记录批内自然语用
- **正身**:identity verify verdict attest / anomalies 0 / identity_hash 0e182797f0a19204...
- **租约**:open be316fe3596bf16e,allow 九路径含 trail 目录(承 idenlane-solo 漏 trail 教训),scope_source package
- **锁**:九把(identity 父目录承 idenlane-solo 父子冲突教训、双 CALL-LOG、results、materials、scribe/reports、meter/counts、facet 合同目录、trail 文件 append 形)
- **书简意图**:5cc6a71c 在链,envelope.session_id=be316fe3596bf16e 与 identity_hash=0e182797... 双字段在档
- **管线三步**:化格(CONTRACT 0 / topic 0 / tally-material 1 已归一)→ 核阅(des-001 域外 exit-2 两笔如实记:CONTRACT.md 与 topic.md 均不在 sih-engine/doc 域)→ 检词(两目标 findings 0)
- **二裁链笔**:crosscheck dd210f69 终签在链(7f698533 为化格位次序事故作废前签,见第八节申报一)
- **双仓 settle**:见第九节
- **链 verify**:valid,见第九节

## 八、越线与误差申报

1. **化格位次序事故致二签**:tally-material.json 在首次 attractor sign 之后跑化格被归一改写(exit 1),verify 重放出 divergent(整包作废)。处置:以化格规范形为准在位重跑 check(pass 12 项)与 sign,新终签 dd210f69,重放 identical;作废前签 7f698533 仍在链不可篡改,本笔即其作废声明。教训:化格必须先于核签(笔在核前对材料件同样成立)。
2. **首次采样 basis 不共识重采**:首轮回填九发基线引用混用三基线(baseline_1/4/5),判据 v3 闸 basis_consensus 挂出 boundary「打回重作」。处置:工地内飞轮(未认证工件)清零重样,九发以单一最强依据 baseline_1 诚实重填重测,得 stable_clear。重样发生在任何认证之前,作答内容未因判定结果反向挑选(九发全部 comply 未变,仅依据域归一)。
3. **撞锁候位实录**:开工时并行批 docmath-namefit-solo(ab0aa6b23047a03d,15:13:52 issued)持支撑面四路(identity/reports 与 scribe/reports 与 meter/counts append 及 trail 目录),六路径取锁 locked_elsewhere。按 packhyg 条款实查锁台账与时间戳判定其在途,不绕行;对方 15:17:33 全量放锁 15:17:43 收约让位后,本会话 15:15:56 已持四锁 + 补锁五把完成取锁,零绕行零 takeover。
4. **watchcheck-solo 预检判空**:编排提示并行 watchcheck-solo 批在 lease 域施工,预检实测锁台账、会话台账、双仓工地、trail 四面均无其痕迹,判定未开工或已收约,未候位直接开工,属实申报。

## 九、双仓收口

| 仓 | settle 段 1 | 归并 |
|---|---|---|
| sih-tools | 6f97044e | msh/idenlane-human-solo → main(close 归并) |
| sih-engine | (段 2 补记) | msh/idenlane-human-solo → main(close 归并) |

- 工具侧随批件:identity(src/identity core.py 与 cli.py 与 __init__.py、pyproject.toml、tests/test_human_seat.py、CONTRACT.md 修订九)、facet/contracts/idenlane-human-260905/m-human-seat-1/(topic.md、contract.json、responses.jsonl、flywheel-trail.jsonl、cal-pack.json、cal-responses.jsonl、seat-baseline.json、tally-material.json、tally-check-report.json、contract-score-material.json、m-human-seat-1-signcheck.json)、CALL-LOG 双笔
- 引擎侧随批件:sih/event/plan/idenlane-human-solo-results.md(本档)
- close 后链 verify 与 reconcile 读数:(收口后补记)

## 十、批 C 起步状态(idenlane-guard-solo 令)

- 主树在役面:信封绑定(session_id + identity_hash 条件入哈希,TDD 篡改测试在册)+ scribe direct(agent 笔 --identity-report 强制,human 笔 --human-seat 三闸先验 + --allow-human-stub 测试形)+ 守卫直改车道(d5cd596f,guardcore DIRECT_PEN + hooks commit-msg 当日 trail grep)+ human seat 验真件形与五闸面(本批,identity 0.5.0)
- 批 C 余量:判定语义三裁(合法提交形,facet 合同模式)+ lease tests 全测绿读数 + 管线认证收约
- 本批收约读数即批 C 起步:双仓 main 头、链 verify valid、零锁零会话(本批 close 后)

## 十一、关联

- **上批**:idenlane-envelope-solo 批 A(sih-engine/sih/event/plan/idenlane-envelope-solo-results.md)
- **决策档**:sih-engine/sih/event/plan/idenlane-solo-results.md(勘误版)
- **任务包**:sih-engine/sih/state/plan/idenlane-human-solo.md
- **下批**:idenlane-guard-solo 批 C(sih-engine/sih/state/plan/idenlane-guard-solo.md)
- **测量工件**:sih-tools/facet/contracts/idenlane-human-260905/m-human-seat-1/(close 归并后主树路径)
