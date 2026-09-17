# idenlane-solo：链上正身绑定与直改车道

> 令源：用户 2026-09-05 裁定（结算批结果档第六、七节追记原文在档）「链上应该有正身记录，作为agent置信度组件的数据」「人类和agent（LLM）在司衡内作为生产方，是同等待遇，人类的权责（裁决权）更高而已」
> 范式：T6 单线 solo，委外代理亲写零子代理
> 设计约束全文：sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 第六、七节

## 一、问题陈述 {#problem}

- **问题 1**：链行不带身份。实测全链 1744 事件信封零 session 字段、今日链行零 identity 痕迹；闸三写时验会话、验完即弃，操作来源可追溯（工程基线第四条）断在最后一环。
- **问题 2**：正身体系只覆盖 agent 席位（框架:模型:版本三段式），人类节点无正身件，生产方平权缺一半。
- **问题 3**：无租约的轻量写入无正门——要么开全套租约，要么走 bypass 被记成绕行；直改通道（含 human seat）缺合法笔形与守卫放行逻辑。

## 二、关键设计 {#design}

### 2.1 信封绑定（scribe 引擎件）

- scribe 每笔写入把 session_id 与 identity_hash 落进事件信封；identity_hash 从 --sessions 会话台账按 session_id 查得，零新增必填参数。
- 旧事件行零改动，verify 重算兼容（新增字段只出现在新行）。

### 2.2 直改笔形（scribe 新形态）

- 无租约轻量链笔：agent 笔强制挂正身报告（--identity-report），human 笔挂 human seat 正身件；笔体载改动文件清单与事由。
- 笔形与租约笔在信封层同构（同带身份字段），仅会话字段为空或直改标记。

### 2.3 human seat 正身形态（identity 工具件）

- human seat 身份件形与三段式席位的关系、字段面、验真方式由批内设计，呈得一裁。

### 2.4 守卫直改车道（lease 钩子）

- commit-msg 守卫增放行逻辑：携带直改链笔引用（笔事件哈希前八位）的主分支提交放行，其余照旧拒。
- lease CONTRACT 修订与 BATCH-FACE 勘误随批。

## 三、工作清单 {#work}

- [ ] iden-01：scribe 信封绑定实装与测试
- [ ] iden-02：直改笔形实装与守卫放行逻辑
- [ ] iden-03：human seat 正身形态设计与实装
- [ ] iden-04：契约同步（lease CONTRACT、BATCH-FACE）与判定语义得一裁
- [ ] iden-05：活体验证——本批直改面自证（直改笔+守卫放行实录）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 信封绑定 | 新链行带 session_id 与 identity_hash，旧行零改动，scribe verify valid |
| **F-2** | 直改笔形 | agent 笔无正身即拒、有正身即落；human 笔同构；逐笔 grep 验证在档 |
| **F-3** | 守卫车道 | 带链笔引用直改提交过闸，无引用裸提交仍拒；先红后绿夹具在档 |
| **F-4** | human seat | human 正身件可验、可与 agent 席位同面记账 |
| **F-5** | 判定语义 | 什么算合法链笔与合法提交的变更过得一裁，near_threshold 呈用户 |
| **F-6** | 回归 | 既有测试全绿零回归，双跑逐字节一致 |

## 五、必读文件 {#read}

- 设计约束：sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 第六、七节
- 机械链：sih-tools/BATCH-FACE.md（含 2026-09-04 闸三必带参与重编勘误）
- 掩败教训：sih-engine/sih/state/parking/materials/pk-057.json 附记
- 正身现状：sih-tools/identity/（0.4.0，attest 形）
- 守卫现状：sih-tools/lease/hooks/ 与 guardcore.py

## 六、约束 {#constraints}

1. 判定语义任何变更（合法链笔形、合法提交形、human seat 验真）过得一裁
2. scribe 写入裸调逐笔 grep 验证，禁止 meter 包裹掩败（pk-057 附记纪律）
3. TDD 先红后绿；新增判定性常数零裸奔；旧二进制重编后验收
4. 零数据迁移：旧链行不动，信封绑定只向前生效

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] CONTRACT 修订与 CALL-LOG 双笔随批
- [ ] 得一裁材料在 facet/contracts/，裁决结论入结果档

## 八、风险点 {#risks}

- 守卫放行逻辑放宽可能被滥用：链笔引用校验须核事件真实在链（对当日 trail grep），不核引用即形同虚设
- 与 watchcheck-solo 批同触 lease CONTRACT：本批先行，彼批后动

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- sih-engine/sih/event/plan/idenlane-solo-results.md（随批产出）
- sih-engine/src/event_stream/（scribe 源位）
- sih-tools/lease/hooks/、sih-tools/identity/

## 十一、请求写入 {#requested-writes}

- sih-engine/src/event_stream/ 与 sih-engine/target/debug 重编
- sih-tools/lease/hooks/、sih-tools/lease/CONTRACT.md、sih-tools/lease/src/lease/guardcore.py
- sih-tools/identity/、sih-tools/BATCH-FACE.md
- sih-engine/sih/event/plan/idenlane-solo-results.md 与 materials/
