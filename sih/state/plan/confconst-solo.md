# confconst-solo：置信度常数出生通道修订批

> 治理任务包（立文类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 裁「我不拍板数字，所有常数都要有数学模型支撑」——即 confmodel-derivation-2026-09-07.md 常数候选表"呈裁"形态废止，数值出生纪律修订令
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 现推导档常数表九槽位呈裁态，数值最终来源是人手挑选，与裁定冲突：人类裁规则不裁数值
- 支撑面须升级：依据行（每值附模型位）只是必要条件，出生通道才是充分条件——每个数值必须从模型内通道出生，人零挑数

## 二、关键设计 {#design}

### 2.1 三通道分类（数值出生的唯一合法面）

- **归一值**（闭位）：单位与纪元约定，模型内定义即成立。q／LOCK_BILL_UNIT／UNUSED_LOCK_MULTIPLIER 契约冻结常量照录；m_clean:=1 即记账单位锚（选单位不是经验断言）；T₀ 即台账纪元定义
- **推导值**（闭位或带重推导条款）：公理加定理给出。m_active 上界自 A2 小额定性加归一导出；N_min 下界自防刷分不等式加窗口可铸上界导出，U⁺ 标定前用可证保守界并在档挂"U⁺ 标定后机械重推导"条款
- **标定值**（开位带程序与触发）：模型冻结估计程序，数值由确定性程序对数据机械产出并上链。U／U⁻／U⁺、窗宽 w、D_acc、m_repair 分档表——程序同参双跑逐字节一致、显式给参禁读钟禁随机，触发条件绑账单数据攒量与 pk-074 子项一/二

### 2.2 修订面

confmodel-derivation-2026-09-07.md 同文件升版（版本史追加制），常数表重构为三通道形；甲案 c:=b 维持并从"模型裁定"升格为归一通道条目。一裁重过 gid m-confconst-1，单锚 baseline_4 可验证性（同先例形）。

### 2.3 人节点收窄后的职责面

人裁规则不裁数值：裁模型版本与升版、裁标定程序（程序本身是规则）、裁机制启停时刻；数值产出零人手。

## 三、工作清单 {#work}

- [ ] 推导档升版：常数表重构三通道，逐槽位标注通道与出生依据，呈裁态清零
- [ ] 推导值附证明行与复算式；标定值附程序文本、双跑读数与触发条件
- [ ] 一裁测量与执契终签落链
- [ ] 管线三步、书单对表、泊界心跳

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 通道完备 | 数据治理 | 常数表全槽位落三通道，呈裁残留零 |
| **F-2** 推导可复算 | 治理 | 推导值逐值附证明行，复算式重跑一致 |
| **F-3** 标定可双跑 | 治理 | 标定程序同参双跑逐字节一致，显式给参零随机零读钟，触发条件显式在档 |
| **F-4** 一裁过 | 跨族治理 | m-confconst-1 stable_clear 加机器终签在链 |
| **F-5** 零改动零越界 | 治理 | A 族 B 族公理零改动，零代码零台账实装，机制启停不在本批 |

## 五、必读文件 {#read}

- 修订对象：`sih-math/docs/confmodel-derivation-2026-09-07.md`（升版底本）
- 裁定令源：本任务包头部承接节
- 先例：`sih-engine/sih/event/plan/confmath-solo-results.md`（建模批结论与槽位现状）

## 六、约束 {#constraints}

1. 零代码改动，零 A/B 公理改动，零在役判据触碰
2. 数值零人挑：本批内任何数值落档必附通道出生依据，批档自身不产生新呈裁数值
3. 标定程序写入推导档正文体即规则面，其执行与数值产出归后继批
4. 数学仓写入面即 docs 推导档升版；mapping.md 与条目改动走数学仓自身程序
5. 主树零直写经工地 settle 通道，链文件只经引擎 scribe 写位
6. 在泊件零触碰不并批；在盘遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增，checkcite pass
- [ ] 结果档 confconst-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 保守界推导不闭合（窗口可铸上界不可证）→ 显式申报缺口与所需公理补面，呈用户裁公理增补，不硬凑
- 标定程序依赖的数据面字段未定 → 槽位留接口申报，不虚构数据形

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（写位）、`sih-engine/target/debug/attractor`（一裁与终签）
- 接口泊件：pk-074 子项一/二（标定触发与数据面）

## 十一、请求写入 {#requested-writes}

- `sih-math/docs/confmodel-derivation-2026-09-07.md`（升版）
- `sih-engine/sih/state/plan/confconst-solo.md`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/confconst-solo-results.md` 与 `confconst-solo-materials/`
- `sih-tools/proposition/DES/m-confconst-1/`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 confconst-solo 工地
