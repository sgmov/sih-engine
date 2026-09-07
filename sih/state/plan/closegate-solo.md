# closegate-solo：主树改件双闸硬化（收约位拦截闸 + pre-commit 文件面执法位）

> 令源：用户 2026-09-07 令「开」；判定语义双裁在链即 m-closegate-1 九发 stable_clear 机器终签 6a7a7237（收约位拦截闸）与 m-precommit-1 九发 stable_clear 机器终签 b558220e（pre-commit 执法位）；pk-076 出泊随批承载（入泊事件 32ec20a2），pk-074 子项三随批三件出泊
> 范式：T6 单线 solo，委外代理亲写零子代理
> 前置：与 projfix-solo 串行即彼批先行，本批候 CONTRACT 修订位；在飞批撞锁即 wait-turn

## 一、问题陈述 {#problem}

- **问题 1**：主树 tracked 改件无常驻拦截位（pk-076）——watchcheck 只呈报且挂点单点、批域闸只对声明面负责、sweep 五类普查不含 tracked 改件；calllog-solo 跨根写抹 137 行历史零拦截为活体实证。
- **问题 2**：轻车道白名单与共享追加面俱为声明位零执法（T-9 既缓执法位，pk-074 子项三）——多代理直改已实质发生，触发条件达成。

## 二、关键设计 {#design}

### 2.1 收约位拦截闸（m-closegate-1 已裁语义）

- 闸位：close_session 闸序内，chain_gate_check（core.py 1422 邻域）之后、差集闸之前挂无主闸（批内定形挂序并给理由）。
- 谓词：复用 watchcheck 无主清单判定即主树 tracked 脏文件集 −（租约锁面 ∪ 直改链笔声明面 ∪ 共享追加面豁免面）。调用形批内定（子进程调 watchcheck CLI 或抽公共谓词库），给理由；谓词单源不双写。
- 行为：无主清单非零即拦收约退出码一 fail-visible，报告无主文件清单（路径 + mtime）；例外经 bypass 登记通道放行留痕（--bypass-orphan <事由> 落 bypass 行后放行，批内定形对齐已裁语义）。
- 不代裁：回滚与通道处置仍归人节点二值（watchcheck F-6 同族）。

### 2.2 pre-commit 文件面执法位（m-precommit-1 已裁语义）

- 钩位：hooks/pre-commit 由环境预检薄壳扩为执法壳（fail-closed 保持即破即拒）。
- 判定：staged 文件集逐件，命中活跃租约锁面 ∪ SCOPE_SHARED_SURFACE（27 路）∪ DIRECT_LANE_FILE_WHITELIST（30 条目）即放行；未命中即拒退出码非零并指引三通道（开租约、挂直改链笔、bypass 登记 --no-verify）。
- 锁面读取：经 lease 库读活跃锁（hooks 禁依赖 venv 即薄壳 sys.path 直入 src 先例），批内定形。
- merge 提交不触发 pre-commit 即归并路径不受此闸；钩缺席环境执法为零由 watchcheck 例行读数呈报不静默（如实申报条随 CONTRACT）。
- 自举闭环：本批自身提交须先持锁否则被自己的闸拒——批内即活体验收。

### 2.3 出泊承载

- pk-076 出泊：出泊材料 pk-076-exit.json 落本批 materials，出泊事件随批补笔回填链（先例同形）。
- pk-074 子项三出泊：执法位交付即触发条件闭，出泊材料并批承载（子项一、二仍留泊）。

## 三、工作清单 {#work}

- [ ] T-1 无主闸谓词接入与挂序定形
- [ ] T-2 拦收约与 bypass-orphan 例外通道
- [ ] T-3 pre-commit 执法壳（staged 面判定 + 三通道指引 + fail-closed）
- [ ] T-4 误伤夹具族：共享追加面改件过、锁面内改件过、直改链笔声明面过、白名外面拒
- [ ] T-5 自举活体：本批提交全程被闸对表（拒一次后持锁放行或 bypass 留痕，如实记档）
- [ ] T-6 pk-076 与 pk-074 子项三出泊材料与链事件
- [ ] T-7 CONTRACT 修订（双闸语义、执法位升位、钩缺席如实申报条）与 watchcheck 联动读数呈报位

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 闸拦截 | 主树放一个无主 tracked 改件，close 必拦退出码一且清单呈报 |
| **F-2** | 例外通道 | bypass-orphan 放行留 bypass 行可对表 |
| **F-3** | 零误伤 | 共享追加面/锁面/直改链笔/白名四族夹具零拦 |
| **F-4** | 提交执法 | staged 白名外面即拒并指引三通道；merge 路径不受闸 |
| **F-5** | 自举活体 | 本批自身提交被闸实录在档（拒与放行两态） |
| **F-6** | 双跑一致 | 闸判定与执法判定同参双跑逐字节一致 |
| **F-7** | 零 LLM | 双闸全程零模型调用零网络 |
| **F-8** | 语义承证 | 按 m-closegate-1 与 m-precommit-1 落地零偏离，遇边界停批呈报 |
| **F-9** | 出泊 | pk-076 与 pk-074 子项三出泊材料与链事件在档 |

## 五、必读文件 {#read}

- 双裁语义：sih-tools/proposition/DES/m-closegate-1/ 与 m-precommit-1/（topic.md 与计分材料）
- 无主谓词母本：sih-engine/sih/state/plan/watchcheck-solo.md 2.1 节
- 闸序现行：sih-tools/lease/src/lease/core.py close_session 1422 邻域
- 钩位现行：sih-tools/lease/hooks/（commit-msg 与 pre-commit 薄壳）与 guardcore.py
- 泊材料：sih-engine/sih/state/parking/materials/pk-076.json 与 pk-074.json

## 六、约束 {#constraints}

1. TDD 先红后绿；既有全族测试零回归；谓词单源不双写
2. 判定语义零新裁：双裁落地遇边界停批呈报
3. 闸与执法全 fail-closed/fail-visible，零静默放行零静默降级
4. 主树验收硬性项：F-1 与 F-4 须主树真跑
5. 收约机械链全序照 BATCH-FACE；拆工地前先 cd 出去

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-9 全过
- [ ] CONTRACT 修订与双泊位出泊随批
- [ ] 结果档 sih-engine/sih/event/plan/closegate-solo-results.md（T6 管线）与 materials/
- [ ] 双仓 settle + close + reconcile + verify 全绿

## 八、风险点 {#risks}

- 谓词调用形（子进程 vs 库引用）影响闸耗时与部署面——批内定形给理由
- pre-commit 执法对存量提交流（结果档直提等）的误伤面——四族夹具宁全勿漏，误伤如实申报
- 与 projfix-solo 的 CONTRACT 修订串行位——后开批候叫

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理；保留 F 锚定、得一裁红线、双仓同步；判定语义已前置双裁非本批内裁。

## 十、关联文件 {#related}

- 命题与裁决：DES/m-closegate-1/ 与 m-precommit-1/；材料 facet/contracts/zc-glm53-20260907-closegate/ 与 -precommit/
- 泊位投影：sih-engine/doc/governance/PARKING-v1.md（pk-076 行、pk-074 行随批更新）
- 上游批：projfix-solo（CONTRACT 串行位）

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/（core.py 闸序 + cli.py + watchcheck 接入）
- sih-tools/lease/hooks/（pre-commit 执法壳）与 sih-tools/lease/src/lease/guardcore.py（执法判定函数）
- sih-tools/lease/CONTRACT.md 与 sih-tools/lease/tests/
- sih-engine/doc/governance/PARKING-v1.md（双泊位行更新）
- sih-engine/sih/state/parking/materials/pk-076-exit.json 与 pk-074-exit-iii.json
- sih-engine/sih/event/plan/closegate-solo-results.md 与 materials/
- sih-engine/sih/event/trail/（收约与出泊链笔）
