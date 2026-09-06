#!/usr/bin/env python3
"""confmath-solo 推导档构建器：公理组从链上材料与在役档程序切片逐字节内嵌，禁手打。

切片源三件：
- pk-073 泊材料 context 字段（六要件 A1-A6 与候选方向 B3）
- GOV-002 v2.3 节行（B1 记账后算分、B2 首免）
- lease CONTRACT 修订四十一/四十二行（账单四事件与 3/0/1 实跑、零结算零评分与 grandfather）
"""
import json
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
OUT = ROOT / "worktrees/sih-math/confmath-solo/docs/confmodel-derivation-2026-09-07.md"

# ---------- 程序切片 ----------

def slice_between(text: str, start: str, end: str, occurrence: int = 1) -> str:
    """取 text 中 start 起至 end 止的逐字节子串（含 start，含到 end 前的 end 标记可选）。"""
    i = -1
    for _ in range(occurrence):
        i = text.index(start, i + 1)
    j = text.index(end, i + len(start))
    return text[i:j + len(end)]


pk73 = json.loads((ROOT / "sih-engine/sih/state/parking/materials/pk-073.json").read_text(encoding="utf-8"))
ctx = pk73["context"]

AXIOM_A = {}
for tag, cn, endmark in [
    ("A1", "一即", "支付 fail-closed"),
    ("A2", "二即", "数值走常数定义化通道"),
    ("A3", "三即", "防高产能莽人刷分通道"),
    ("A4", "四即", "余款充公铸给主动上链者"),
    ("A5", "五即", "grandfather"),
    ("A6", "六即", "本轮为提醒不强制与越界归属"),
]:
    AXIOM_A[tag] = slice_between(ctx, cn, endmark)

B3 = slice_between(ctx, "模型候选方向照录", "涨停跌停概念。")

gov2_line = (ROOT / "sih-engine/doc/governance/GOV-002-mainline-lock-v1.md").read_text(encoding="utf-8").splitlines()
v23 = next(l for l in gov2_line if "v2.3，2026-09-06" in l and "租约修复升级线" in l)
B1 = slice_between(v23, "锁面定价先记账后算分", "阶段一零置信度依赖")
B2 = v23[v23.index("开工锁少与首次临时加锁免费"):].split("、")[0]

contract = (ROOT / "sih-tools/lease/CONTRACT.md").read_text(encoding="utf-8").splitlines()
rev41 = next(l for l in contract if "修订四十一" in l)
rev42 = next(l for l in contract if "修订四十二" in l)
BILLSPEC = slice_between(rev41, "四类事件 bill_session_start", "UNUSED_LOCK_MULTIPLIER 倍）")
ZEROSTAGE = slice_between(rev41, "零余额零结算零评分", "grandfather 激活前行为零追溯")
LIVEPTS = slice_between(rev42, "（open_face_bill 3 pts", "lock_charged 1 pt）")

for name, s in [("A1", AXIOM_A["A1"]), ("A6", AXIOM_A["A6"]), ("B1", B1), ("LIVEPTS", LIVEPTS)]:
    assert s and len(s) > 10, f"slice {name} too short"

# ---------- 推导档主体（formalization by confmath-solo）----------

HEADER = f"""# confmodel 置信度数学模型推导档：置信度经济六要件公理化与常数候选表

> 批：confmath-solo（pk-073 建模批，2026-09-07，单线 solo，零 sih-engine 写入零台账实装）
> 令源：用户 2026-09-07 令「置信度数学模型先行，你出任务提示词和任务包」，即 pk-073 出泊条件之立项裁落位；主会据以出任务包与委外提示词，本批委外执行。
> 上游：pk-073 泊材料（停泊事件 e785481b，六要件三轮对话用户裁定照录）；GOV-002 v2.3 既裁面；lease CONTRACT 修订四十一／四十二账单实装面。
> 批性质：零代码、零台账实装、零数值终裁——本批只给置信度经济立数学模型与常数候选表，一切数值呈用户裁；阶段二件（台账与抢占实装）只在文档面出现。

## 1. 公理化口径 {{#caliber}}

- 公理组 A 即 pk-073 照录六要件，程序切片逐字节内嵌（生成器 build_confmodel.py 随批材料，切片自链上泊材料 JSON），禁手打转述。
- 公理组 B 即在役既裁补面，切片自 GOV-002 v2.3 节与 lease CONTRACT 修订四十一／四十二行，同样程序切片。
- 处置形承 constmodel-derivation-2026-09-05.md 四形判据：承接（数学仓已有条目覆盖语义给引用锚）、现推（从公理自行推导）、定义化（约定条文如实声明禁伪装推导）、呈裁（候选值列依据行呈人节点）。本批新事物是算子与不变量形式化，不是值级推导——一切数值走常数候选表呈裁。
- 载体纪律：先全读 sih-math/llm-friendly-build/mapping.md 再定位概念 ID；引用只落书单及图闭包；数学仓无对应的新概念显式申报提案态。

## 2. 公理组 A：六要件照录 {{#axioms}}

> 以下六条为 pk-073 链上泊材料 context 字段程序切片，逐字节照录零改写。

"""

AXIOMS_SECTION = "\n\n".join(
    f"**{tag}（照录）**：{text}" for tag, text in AXIOM_A.items()
)

AXIOM_B_SECTION = f"""

## 3. 公理组 B：既裁补面照录 {{#axioms-b}}

> 以下四条为在役档程序切片（GOV-002 v2.3 节行与 lease CONTRACT 修订四十一／四十二行），逐字节照录零改写。

**B1（照录）**：{B1}

**B2（照录）**：{B2}

**B4（账单事件与冻结常量照录）**：{BILLSPEC}

**B5（阶段一处置照录）**：{ZEROSTAGE}

**B6（实跑点值照录）**：{LIVEPTS}

**B3（候选方向照录）**：{B3}

"""

BODY = """## 4. 记号与载体锚 {#notation}

- 台账 L(t)：追加式事件序列，事件 e = (ts, kind, account, amount, ref)，ref 挂链上事件哈希（流水上链，A1）。追加式与版本化外化语义承 ORD-019（mapping.md:196，版本偏序与外化状态存储——台账形态参照，非本批新推导）。
- 账户域 A：开户函数 open: K → A 由正身核验记录诱导，账户键 k = core_hash（identity 契约修订八：六核心键去包装确定性哈希，跨报告配对键；identity_hash 缺省盐随机逐报告不同，不作账户键）。流水逐笔携 identity_hash 保追溯（租约会话与 facet 材料先例同形）。
- 余额 b: A × Time → ℕ，量纲信用点（pts）。
- 人席位账户集 H ⊆ A，机械强制算子域 A_mech = A \\ H（D11）。

## 5. 定义与算子 {#defs}

**D1（余额与守恒，承 PROB-016）**：b_k(t) = Σ_{e∈L(t), acct(e)=k} amt(e)，amt: E → ℤ（铸币正、支付与罚金负）。总供给 M(t) = Σ_k b_k(t)。每事件守恒式 Σ_k Δb_k(e) = mint(e) − burn(e)；即余额是事件集上带符号计数测度的分账求和，可加性恒等式 I2 可机械回放核对。锚：PROB-016 计数测度与可加性（mapping.md:211）。

**D2（余额下限零）**：b_k(t) ≥ 0（A1 照录“余额下限零”的值域化），值域 ℕ 的偏序结构承 ORD-001 偏序集（mapping.md:165）。

**D3（支付算子，fail-closed 全化）**：pay(k, δ) := 若 [b_k(t) ≥ δ 且 O_k = ∅（D8）且 k ∈ A_mech] 则 b_k ← b_k − δ 并返回 ok；否则拒绝且余额零变动（I5）。guarded subtraction on ℕ——部分算子经守卫全化，fail-closed 即守卫失败落拒不动状态。支付算子对余额序单调：b ≤ b′ ⟹ pay 结果序保持（被拒形持平、成功形同减 δ），单调算子语义承 ORD-004 monotone operator（mapping.md:162）。

**D4（铸币算子，A2 形式化）**：mint(k, c)，触发谓词 T(c) ∈ {clean_close（干净收约），active_pen（主动链笔小额），repair_foreign（修复他因事故）}，纯机械触发零主观打分（A2 照录）；数额 c = f(kind) 为常数表步进函数（§7 候选表），自产自修事件类不触发（A2 照录）。供给面：铸币是唯一正 mint 源。

**D5（罚金算子与防刷分不等式，A3 形式化）**：penalty(k, D)，D 为损害量级测度（事件分级：轻级/事故级，阈值槽位 D_acc）。罚金函数 p(D) 单调非降，且事故级下界公理：p(D) ≥ N_min · m_clean 对一切 D ≥ D_acc。防刷分论证（现推）：设单结算窗单账户最大清洁铸币次数上界 n_max（由窗界 D7 与产能上限诱导），刷分净收益上界 = n_max·m_clean − p(D) ≤ (n_max − N_min)·m_clean；故 N_min > n_max ⟹ 事故刷分通道净负。即 A3 的形式化即参数不等式 N_min > n_max，方向由公理钉死，数值候选呈裁。

**D6（涨停跌停——窗变动界机制，B3 候选方向形式化）**：对结算窗 w 与账户 k，净变动 Δ_b(k,w) = Σ_{e∈w, acct=k} amt(e) 受界：|Δ_b(k,w)| ≤ U（对称带候选形一）或 Δ_b(k,w) ∈ [−U⁻, U⁺]（分段带候选形二），两形二择一呈裁。超界变动递延下一窗（递延队列，先进先出）。机制语义：涨界防单窗暴铸，跌界防单窗清零（可恢复性）。参数槽位 U、U⁻、U⁺ 与递延队列纪律呈裁。锚：有界变差机制在数学仓无对应条目，显式申报提案态（§9）。

**D7（欠账与抢占门槛，A1 推论）**：欠账集 O_k := 账户 k 的未决结算事件集（挂起的 settlement 或分期罚金记录；余额下限零使欠账不体现为负余额，体现为未决义务记录）。抢占门槛公理化：O_k ≠ ∅ ⟹ pay(k, ·) 拒（A1 照录“欠账未清不得抢占”）。

**D8（抢占闭环算子，A4 形式化）**：preempt(k₁, target, δ_p)：
1. 即付：pay(k₁, δ_p) 成功即托管 escrow ← δ_p，**不退性**：escrow 不因后悔或撤回退回（A4 照录“付置信度不退”）；
2. 裁决：终局函数 settle 以得一裁事实与理由为输入（A4 照录），放锁落链（锁态变更走链上事件）；
3. 分配：被抢方修复按实据报销 r ∈ [0, δ_p]（实据面核验），escrow − r 为余款；无责回滚形 r = 0 且锁态回滚，托管全额转余款；
4. 充公：余款入公共池账户 P（sink 账户，非人非 agent），P 按机械触发器（主动链笔事件）铸给在链主动者（A4 照录“余款充公铸给主动上链者”）。
约束回放校验：r ≤ δ_p（I6 附件），escrow 生命周期逐笔可对账。

**D9（置信度语义，二择一显式裁定）**：
- 候选甲（取）：c_k(t) := b_k(t)——置信度与余额同一量的两个名字。依据：A1 照录原文“置信度即信用点即支付货币”，同名直取是公理的原样读法，零新增公理。历史即信誉曲线（A1 照录）即余额轨迹 (t, b_k(t))。
- 候选乙（不取，登记提案态）：c = φ(b) 经单调映射。须新增公理定 φ（单调性、标定、界），本批公理组无此公理，如实登记为无公理支撑提案，不冒充已立。PROB-005 贝叶斯更新（mapping.md:155）登记为乙案未来的语义参照面（若信誉曲线需要加权信念解释时另批），本批不消费。

**D10（人节点账户，A6 形式化）**：人席位 h ∈ H 照常计分（D1 同一求和），但机械强制算子域排除：pay/penalty/preempt 的主体域为 A_mech = A \\ H；对人的唯一作用面是视图提醒（viewer alarms 语义）。负余额倾向与不修正不触发任何机械强制，治理归现实世界（A6 照录“人类职责最高穿透一切机械锁”），模型内即 H 上无强制算子、仅观测与提醒。与结算批第八节人类置信度评分条款两裁各表（A6 照录原文声明不矛盾）。

**D11（阶段一 grandfather，A5 + B5 形式化）**：模型纪元 T₀（候选：置信度台账上线日，呈裁）。求和域限定：b_k(t) = Σ_{e∈L(t), acct(e)=k, ts(e) ≥ T₀} amt(e)。T₀ 前账单事件（含 B6 实跑 3/0/1 pts）零余额贡献（I7）。billwire 实跑点值地位显式裁定：**grandfather**（A5 与 B5 照录直接覆盖），实跑值仅作常数候选表的量级参照（§7 依据行），不作模型常数的推导输入，不静默改数。

## 6. 不变量组（可机械检验，逐条回放判真伪） {#invariants}

| 编号 | 不变量 | 检验形 |
|---|---|---|
| I1 | b_k(t) ≥ 0 ∀k,t | 事件流回放逐步核对 |
| I2 | 守恒式 Σ_k Δb_k(e) = mint(e) − burn(e) 逐事件成立 | 回放核对，M(t) 与分账和零偏差 |
| I3 | 事故级罚金 p(D) ≥ N_min·m_clean | 回放核对（参数化，候选域内判真伪） |
| I4 | |Δ_b(k,w)| ≤ U 或 Δ_b(k,w) ∈ [−U⁻, U⁺] 且递延队列零溢出 | 回放核对（两形随裁） |
| I5 | 被拒支付零余额变动（fail-closed） | 回放核对 |
| I6 | O_k ≠ ∅ 的 pay/preempt 拒绝率 100%；报销 r ≤ δ_p | 回放核对 |
| I7 | T₀ 前事件零余额贡献 | 回放核对 |
| I8 | 人席位 H 上无强制算子触发记录 | 回放核对（A6） |

无脚本形态：每条不变量给定任一事件流 L 与参数候选域即可逐条手验，真伪二元可判——单锚可验证性的承载面。

## 7. 常数候选表（零裸常数，逐值携依据行，一切数值呈用户裁） {#constants}

| 槽位 | 候选值 | 模型位 | 依据行 | 状态 |
|---|---|---|---|---|
| m_clean 清洁收约铸币额 | 1 pt | D4 步进函数值 | 与 B6 实跑 LOCK_BILL_UNIT=1 同量级（量级参照非推导），A2“干净收约铸币”触发面 | 呈裁 |
| m_active 主动链笔小额铸币 | ≤ 1 pt（上界候选） | D4 | A2 照录“小额”定性给出上界方向；具体值无公理定 | 呈裁 |
| m_repair 修复他因事故铸币 | 按损害量级分档表（槽位） | D4 | A2“修复他因事故铸币”，分档界随 D_acc 联动 | 呈裁 |
| N_min 事故罚金下界倍数 | 10（方向论证值） | D5 不等式 N_min > n_max | 现推：刷分净负方向论证；数值候选须满足 n_max < N_min 且实测 n_max 后标定 | 呈裁 |
| D_acc 事故级损害阈值 | 槽位待实证 | D5 事件分级 | pk-074 子项一（超宽阈值调优）同族参数槽位，数据攒量后裁 | 呈裁 |
| U 或 U⁻/U⁺ 窗变动界 | 槽位待实证 | D6 | B3 候选方向（涨停跌停），机制形固定、值域待数据 | 呈裁 |
| 窗宽 w 结算窗 | 槽位待实证 | D6 | 与 U 联动标定 | 呈裁 |
| T₀ 模型纪元 | 候选：置信度台账上线日 | D11 | A5 grandfather 起点；上线日随阶段二批定 | 呈裁 |
| q 首免配额 | 1 | D3 支付面账单分段（B2） | 契约冻结常量 EXPANSION_FREE_QUOTA=1（B4 照录，定义化通道，非推导值） | 既裁照录 |
| LOCK_BILL_UNIT | 1 | 账单计费常数（B4） | 契约冻结常量（定义化通道照录） | 既裁照录 |
| UNUSED_LOCK_MULTIPLIER | 1 | 账单计费常数（B4） | 契约冻结常量（定义化通道照录） | 既裁照录 |

零裸常数核验：上表覆盖本档出现的全部数值（不含编号与日期）；每个数值或槽位都落定义行或论证行；既裁常量标注定义化通道照录，候选值全部呈裁状态。

## 8. 与 pk-074 参数槽位对表 {#pk074}

- pk-074 子项一（超宽阈值调优）：与 D_acc、窗界族同属“数据攒量后用户裁阈值调优批”的参数域，本批只立槽位不立值，两泊件联动不冲突。
- pk-074 子项二（未用罚口径精确化）：UNUSED_LOCK_MULTIPLIER 在本档定义化照录，精确化批若改值走常数表升版，模型面无冲突。

## 9. 新概念提案态显式申报 {#proposals}

以下概念数学仓 mapping.md 无对应条目，本档只作提案态登记，不冒充已立，走数学仓自身演化程序：

1. **窗变动界机制（涨停跌停形式化）**：有界变差带与递延队列的复合结构（D6）。
2. **托管结算语义（escrow）**：即付不退、终局分配、余款充公的三段算子（D8）。
3. **欠账即未决义务集**：余额非负约束下欠账从负余额转义务记录的表示（D7）。

## 10. 停批申报 {#settlement}

- **公理与实跑点值冲突申报**：无不可调和冲突。B6 实跑 3/0/1 pts 属阶段一账单事件，A5/B5 grandfather 照录直接覆盖其地位（零追溯），实跑值降为量级参照，张力消解于 D11 显式裁定。
- **数值零改动申报**：本批零 sih-engine 写入，契约冻结常量三件照录零改动；全部候选值未落任何实装位。
- **语义二择一申报**：D9 甲案（c := b）依 A1 原文裁定为模型立场；乙案登记提案态。若用户裁乙案，本档 §5 D9 升版，常数表增 φ 标定槽位。
- **checkmath**：本批产出只落 sih-math/docs/，mapping 与 entries 与 INDEX 零触碰。

## 11. 参考索引 {#reference}

- 数学仓消费面：sih-math/llm-friendly-build/mapping.md（全读）：PROB-016（mapping.md:211）、ORD-001（mapping.md:165）、ORD-004（mapping.md:162）、ORD-019（mapping.md:196）、PROB-015（mapping.md:205，抢占排队负载面前瞻槽位登记，本批不推导）、PROB-005（mapping.md:155，乙案提案语义参照，本批不消费）
- 条目实文：probability/entries/PROB-016-counting-measure-and-additivity.md、order/entries/ORD-001、ORD-004、ORD-019
- 仓内先例：sih-math/docs/constmodel-derivation-2026-09-05.md（四形判据与定义化通道）、facetmath-derivation-2026-09-04.md（检验判据式）
- 要件源：sih-engine/sih/state/parking/materials/pk-073.json（链上泊材料，程序切片源）
- 既裁面：sih-engine/doc/governance/GOV-002-mainline-lock-v1.md v2.3 节；sih-tools/lease/CONTRACT.md 修订四十一／四十二
- 制度约束：AGENTS.md 数学仓地位与唯一桥梁、既裁约束“引擎数学常数须有数学模型支撑”、工程基线一二四五条
"""

OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(HEADER + AXIOMS_SECTION + AXIOM_B_SECTION + BODY, encoding="utf-8")
print(f"written {OUT}")
print("slices:", {k: len(v) for k, v in AXIOM_A.items()}, "| B1", len(B1), "| B3", len(B3), "| LIVEPTS", len(LIVEPTS))
