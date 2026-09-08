# idenlane-envelope-solo:信封绑定收尾与直改笔形

> 令源:用户 2026-09-05 裁定(leaseoptsettle-solo-results.md 第六节、第七节) + 用户 2026-09-05 16:42 拆分裁定(idenlane-solo-results.md) + 用户 2026-09-05 17:53 勘误指令(主树已合 19 文件起步不重做)
> 范式:T6 单线 solo,委外代理亲写零子代理
> 拆分定位:idenlane-solo 拆三批之批 A,承载 F-1 信封绑定补全 + F-2 直改笔形新子命令 + F-5 判定语义一裁(合法链笔形)
> 设计约束:hash 兼容性选 1(顶层字段条件入哈希),hash.rs `if let Some(...)` 模式 None 跳过 Some 入字典序,旧行 verify 兼容,新行篡改断链
> 现状:主树 d7f6256 已合 19 文件实装,本批起步不重做 13 文件,只补 TDD 篡改测试 + 直改子命令 + 判定一裁 + 管线认证收约

## 一、问题陈述 {#problem}

- 链上每行事件缺生产者身份:闸三写时验会话、验完即弃,工程基线第四条的"操作来源可追溯"在最后一环断
- 直改笔形无正门:要么全套租约,要么 bypass 绕行,无租约轻量链笔形缺失
- 链笔形(合法链笔形)的判定语义无规约
- 主树 d7f6256 已合 event.rs + hash.rs + append.rs + 5 构造器 None 占位 + scribe.rs resolve_envelope/bind_envelope + 9 配套文件,但 TDD 篡改测试缺失(用户附加验收条)
- 直改子命令未实装(主树仅 resolve_envelope/bind_envelope 函数,无 `scribe direct` CLI)
- 判定语义一裁未跑

## 二、关键设计 {#design}

### 2.1 信封绑定(主树已实装,本批补 TDD 与认证)

- **现状(主树 d7f6256)**:event.rs Event 与 EventInput 各加 Option<String> 字段 session_id 与 identity_hash;hash.rs 用 `if let Some(...)` 模式 None 跳过 Some 入字典序;append.rs 透传两个字段;scribe.rs 新增 resolve_envelope 按会话台账查 identity_hash + bind_envelope 把 session_id 与 identity_hash 落进 EventInput 信封;certify/intent/park/reading/crosscheck/verify 测试与构造器位补 None 占位
- **本批补**:TDD 篡改测试入 tdd_tests 模组——改 envelope.session_id 须致 verify 断链 + 改 envelope.identity_hash 须致 verify 断链(用户附加验收条)
- **本批补**:管线认证收约(cargo test --lib 绿 + scribe append 走链 + chain pen + 链 verify + 双仓 settle + close + reconcile)

### 2.2 直改笔形(本批实装)

- `scribe direct` 新子命令,无租约轻量链笔:
  - 必填:`--record <轻量链笔 JSON>` `--trail <链文件>`
  - 可选:`--session <会话号>` `--sessions <会话台账路径>` `--identity-report <正身报告 JSON>` `--human-seat <人席位验真 JSON>` `--allow-worktree-trail <1|true>` `--no-session-reason <事由>`
  - 笔体载改动文件清单与事由
  - 笔形与租约笔在信封层同构
- 直改链笔 JSON 形:
  ```json
  {
    "pen_form": "direct",
    "pen_kind": "agent" | "human",
    "actor_id": "<生产者标识>",
    "files": ["路径1", "路径2"],
    "subject": "事由",
    "timestamp": "ISO8601"
  }
  ```
- agent 笔形:identity-report 必填(identity.verify 报告)
- human 笔形:human-seat 必填(批 B 实装)

### 2.3 判定语义(合法链笔形,本批跑一裁)

- 合法链笔形二值:lease_pen(租约笔) / direct_pen(直改笔)
- 直改笔子分类:agent_direct_pen / human_direct_pen
- 判定位:`event_type = direct_edit_completed`,details.pen_form 字段二值
- 判定行为变更过得一裁(facet 合同模式, near_threshold 呈用户确认)

## 三、工作清单 {#work}

- [ ] T-1 envelope 兼容:旧链行无 envelope 字段时 verify valid(主树 d7f6256 隐式,本批跑 verify 链 52 事件)
- [ ] T-2 envelope 红证:envelope.session_id 改动即 verify 断链(用户附加验收条,本批新增)
- [ ] T-3 envelope 红证:envelope.identity_hash 改动即 verify 断链(用户附加验收条,本批新增)
- [ ] T-4 envelope 绿:certify 写入时 envelope 字段就位(本批跑管线认证,链 grep 验证)
- [ ] T-5 envelope 绿:intent 写入时 envelope 字段就位(本批跑管线认证,链 grep 验证)
- [ ] T-6 envelope 绿:park 写入时 envelope 字段就位(本批跑管线认证,链 grep 验证)
- [ ] T-7 envelope 绿:record 写入时 envelope 字段就位(本批跑管线认证,链 grep 验证)
- [ ] T-8 envelope 绿:crosscheck 写入时 envelope 字段就位(本批跑管线认证,链 grep 验证)
- [ ] T-9 direct 红证:无 --identity-report 即拒(本批新增)
- [ ] T-10 direct 红证:无 --record 即拒(本批新增)
- [ ] T-11 direct 绿:agent 笔 envelope 字段就位(本批新增)
- [ ] T-12 直改笔形判定语义一裁(facet 合同模式,本批跑)

## 四、可证伪条件(跑前立文) {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 信封绑定 | 新链行带 envelope.session_id 与 envelope.identity_hash,旧链行零改动,scribe verify valid |
| **F-2** | 直改笔形 | agent 笔无正身即拒、有正身即落;逐笔 grep 验证 envelope.session_id 与 envelope.identity_hash 在档 |
| **F-5a** | 判定语义一裁 | 合法链笔形与租约笔形分类清楚,facet 合同模式九发 stable_clear 过执契 |

## 五、必读文件 {#read}

- 设计约束:sih-engine/sih/event/plan/idenlane-solo-results.md 第二、三节
- 机械链:sih-tools/BATCH-FACE.md(2026-09-04 勘误)
- 掩败教训:sih-engine/sih/state/parking/materials/pk-057.json 附记
- 现有 scribe 源位:sih-engine/src/event_stream/(append.rs, intent.rs, certify.rs, park.rs, reading.rs, crosscheck.rs, event.rs, hash.rs)
- 现有 bin:sih-engine/src/bin/scribe.rs

## 六、约束 {#constraints}

1. 判定语义一裁(合法链笔形)过得一裁(facet 合同模式, near_threshold 呈用户)
2. scribe 写入裸调逐笔 grep 验证,禁止 meter 包裹掩败
3. TDD 先红后绿
4. 新增判定常数零裸奔,入 terms.json 入 CONTRACT 入注释
5. 引擎件合并后主树 cargo build 重编再验收
6. 零数据迁移:旧链行零改动,hash 公式零改动
7. 守卫放行逻辑的实装放批 C,本批不触 hooks/

## 七、验收标准 {#acceptance}

- [ ] F-1 / F-2 / F-5a 全过
- [ ] 篡改测试(T-2/T-3)红证构造在档
- [ ] 既存 18 + TDD 新增 N 测全绿零回归
- [ ] 双仓 settle + close + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/idenlane-envelope-solo-results.md
- [ ] CONTRACT 修订与 CALL-LOG 双笔随批

## 八、风险点 {#risks}

- scribe.rs CLI 参数解析手写(HashMap-based),新增 --identity-report 与 --human-seat 不破坏既有旗标
- direct 子命令引入新 event_type direct_edit_completed,既有的 detail 字段面与 verify 路径需新增处理

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步;本批为 idenlane-solo 拆三批之批 A。

## 十、关联文件 {#related}

- 上游决策:sih-engine/sih/event/plan/idenlane-solo-results.md
- 上游任务包:sih-engine/sih/state/plan/idenlane-solo.md
- 下游批 B:idenlane-human-solo
- 下下游批 C:idenlane-guard-solo

## 十一、请求写入 {#requested-writes}

- sih-engine/src/event_stream/(append.rs, intent.rs, certify.rs, park.rs, reading.rs, crosscheck.rs, event.rs, hash.rs) —— event.rs 加 envelope 文档;intent/certify/park/reading/crosscheck 五构造器透传 envelope
- sih-engine/src/bin/scribe.rs —— 新增 direct 子命令与 --session/--sessions/--identity-report/--human-seat 旗标到五既有子命令
- sih-engine/src/event_stream/tdd_tests.rs —— 新增篡改测试 T-2/T-3
- sih-engine/sih/event/plan/idenlane-envelope-solo-results.md 与 materials/
- sih-engine/Cargo.toml —— 若需新 crate 依赖
- sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/2026-09-05.ndjson 与后续日链
