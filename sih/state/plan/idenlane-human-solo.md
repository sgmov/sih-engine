# idenlane-human-solo:human seat 正身形态

> 令源:用户 2026-09-05 裁定(leaseoptsettle-solo-results.md 第七节) + 用户 2026-09-05 16:42 拆分裁定(idenlane-solo-results.md)
> 范式:T6 单线 solo,委外代理亲写零子代理
> 拆分定位:idenlane-solo 拆三批之批 B,承载 F-4 human seat + F-5 判定语义二裁(human seat 验真)
> 范式:批 A 落定后开,human seat 形态与三段式 agent 席位同面记账

## 一、问题陈述

- 司衡内人类节点无正身件,生产方平权裁定(leaseoptsettle-solo-results.md 第七节)只实现一半
- 置信度组件无 human 端数据
- 直改笔形 human_direct_pen 缺验真面

## 二、关键设计

### 2.1 human seat 形态

- 新增 human seat 验真件形(独立于 identity v3 串):
  ```json
  {
    "seat": "human",
    "name": "<人类席位名称>",
    "attest_at": "ISO8601",
    "attest_subject": "事由",
    "guardian": "<裁决人标识或同会主裁>",
    "fingerprint": "可选,任一人类可识别指纹(键盘布局签名 + boottime + 链路代号)"
  }
  ```
- 字段面三件:seat/name/attest_at 为必填,attest_subject/guardian/fingerprint 为可选
- 与三段式 agent 席位(框架:模型:版本)同面记账:在所有信封 envelope 内 `human_seat` 字段落位
- 不破坏 v3 身份串:human 节点不走 verify 流程(承留痕不签),human seat 自带时间戳与事由

### 2.2 验真规则

- 闸一面:human seat 验真件 JSON 须为合法对象
- 闸二面:seat 字段必须为 "human"
- 闸三面:name 与 attest_at 必填
- 闸四面:guardian 字段在场时即人节点已签字(主会协调同款)
- 闸五面:fingerprint 字段可选,三件中任一在场即人脸键盘布局等指纹登记

## 三、工作清单

- [ ] T-1 human seat 验真面红证:缺 seat 字段即拒
- [ ] T-2 human seat 验真面红证:seat 非 "human" 即拒
- [ ] T-3 human seat 验真面红证:缺 name 即拒
- [ ] T-4 human seat 验真面红证:缺 attest_at 即拒
- [ ] T-5 human seat 绿:合法人类席位验真即过
- [ ] T-6 identity 工具件增 `--human-seat <文件>` 旗标
- [ ] T-7 判定语义二裁(facet 合同模式)

## 四、可证伪条件

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-4** | human seat | human 正身件可验可与 agent 席位同面记账 |
| **F-5b** | 判定语义二裁 | human seat 验真规则,facet 合同模式九发 stable_clear 过执契 |

## 五、必读文件

- 设计约束:sih-engine/sih/event/plan/idenlane-solo-results.md
- 批 A 任务包:sih-engine/sih/state/plan/idenlane-envelope-solo.md
- 现有 identity 源位:sih-tools/identity/src/identity/(core.py, cli.py)
- 现有 identity 契约:sih-tools/identity/CONTRACT.md

## 六、约束

1. 判定语义二裁过得一裁(facet 合同模式, near_threshold 呈用户)
2. v3 身份串与盐哈希与 new_salt 零动
3. identity_hash 与 core_hash 字段不动
4. TDD 先红后绿
5. 新增判定常数零裸奔

## 七、验收标准

- [ ] F-4 / F-5b 全过
- [ ] identity verify 既有测试全绿零回归
- [ ] 双仓 settle + close + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/idenlane-human-solo-results.md
- [ ] CONTRACT 修订与 CALL-LOG 双笔随批

## 八、风险点

- human seat 与 v3 身份串在 envelope 字段同存时,字段命名冲突——`envelope.session_id`(agent)/`envelope.identity_hash`(agent 全席位)/`envelope.human_seat`(human 节点)三分
- human seat 无 anchor 验证(无 v3 串),可作伪风险由 guardian 字段对冲(主会签字)

## 九、范式偏离声明

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步;本批为 idenlane-solo 拆三批之批 B。

## 十、关联文件

- 上游决策:sih-engine/sih/event/plan/idenlane-solo-results.md
- 上游任务包:sih-engine/sih/state/plan/idenlane-solo.md
- 上游批 A:idenlane-envelope-solo
- 下游批 C:idenlane-guard-solo

## 十一、请求写入

- sih-tools/identity/src/identity/(core.py, cli.py) —— 加 human seat 验真面
- sih-tools/identity/CONTRACT.md —— 修订九
- sih-tools/identity/tests/ —— 新增 human seat 验真测试
- sih-engine/sih/event/plan/idenlane-human-solo-results.md 与 materials/
- sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/2026-09-05.ndjson
