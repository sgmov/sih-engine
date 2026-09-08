# declguard-solo:声明守卫批（声明对提交差集闸与 lease-check 件归属）

> 令源:用户 2026-09-06 转 watchcheck 代理审计报告工具双缺口,主会修正设计两处（声明面取请求写入节非 allow 全集;豁免走显式认领通道非硬拒）后立项
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:与 leftover-solo（遗留归位批）姊妹批,建议 leftover 收约后本批接续
> 前置:195 基线,lease 1.30.0,closefix 新归并机械与链证守门在役

## 一、问题陈述 {#problem}

- **缺口一 租约自产无主件**:lease 收约把检验凭据转写为包旁 lease-check.json,该族全工作区无归属无清理——引擎仓零件被 settle 过,批批产出批批成无主文件（审计全查在案,全族清单由 leftover-solo 移交）
- **缺口二 close 缺声明对提交差集检查**:现作用域检查只拦写了 allow 面外的,不查声明了却没提交的——PROB-018/019 条目文件（声明路径只提交了 INDEX）与 kernelmerge 结果档（请求写入节列了没走工地）两类漏笔本来可被机械差集拦住;工程基线一精神即叮嘱不构成约束程序才是,代理纪律失守靠闸堵死自动消失

## 二、关键设计 {#design}

### 2.1 缺口二修法:声明对提交差集闸（主会修正版）

- **声明面取请求写入节非 allow 全集**:allow 是许可面非承诺面,全查即海量误报;open_preflight 已解析 parse_requested_writes,差集即 requested_writes 对分支提交树逐路径求「声明未提交」清单
- **处置三态**:差集空即放行;差集非空即拒收约,报文逐件列路径;豁免通道二即其一条件形豁免（包文本行含「若」字样即冻结启发自动豁免,豁免项入报告零静默）,其二显式认领通道 close --ack-uncommitted 逐路径带事由放行,事由记入 revoked 行 detail 留档零粉饰
- 启发式与通道形为冻结常数,判定语义变更过得一裁

### 2.2 缺口一修法:收据归家

- 收据写位迁移:包旁分散位改 lease/ledger/receipts/ 单一目录（receipts/<stem>.json）,该目录入 SCOPE_SHARED_SURFACE 共享面白名单即各批 settle 自然携带收编
- 存量散件（leftover-solo 清单移交）一次性迁入 receipts/ 随本批 settle 收编
- 备选形乙即收据转 runtime-only 加 gitignore 落档不采,理由即收据含 closed_at 与 close_session 与 identity_core 审计值宜进版控

### 2.3 协同与边界

- leftover-solo 先行收约后本批接续（lease 单写面串行）
- 差集闸只挂 close 位,open 与 lock 位零动;直改车道零影响;共享追加面（台账账单）不属 requested_writes 默认面零误伤

## 三、工作清单 {#work}

- [ ] T-1 差集闸实装（requested_writes 对分支树 diff,三态处置,--ack-uncommitted 通道）
- [ ] T-2 收据归家（写位迁移加白名单登记加存量迁移）
- [ ] T-3 TDD 先红后绿（差集非空拒/条件豁免/显式认领/收据新位/存量迁移五族）
- [ ] T-4 判定语义一裁（facet 合同模式）
- [ ] T-5 全测试族零回归与双仓 settle 加 close(过差集闸)加 reconcile 加 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 差集闸 | 声明未提交即拒（PROB-018 形复现夹具红转绿）;条件形自动豁免入报告;--ack-uncommitted 事由入 revoked 行 |
| **F-2** | 收据归家 | 新收据落 receipts/ 目录;白名单登记后批 settle 自然携带;存量散件全量迁入零遗留 |
| **F-3** | 一裁 | 判定语义变更 facet 合同模式 stable_clear 过执契,near_threshold 呈用户 |
| **F-4** | 零回归 | 195 基线全绿加新增;closefix 与 rootanchor 与 ledgerhyg 交付零动;台账行格式零变更 |
| **F-5** | 活体验收 | 本批自身 close 过差集闸（请求写入节全提交或显式认领）,收据落新位 |

## 五、必读文件 {#read}

- 审计报告双缺口段（用户转）
- 现行解析:core.py parse_requested_writes 与 open_preflight 与 close 路径
- 白名单:SCOPE_SHARED_SURFACE（修订三十四）
- 先例:PROB-018/019 漏笔（leftover-solo 归位记录）与 kernelmerge 形
- 机械链:sih-tools/BATCH-FACE.md

## 六、约束 {#constraints}

1. 一裁红线:near_threshold 呈用户转主会不自行终签
2. 声明面严格取请求写入节,allow 全集不入门（误报面零容忍）
3. 台账行格式零变更;ledgerwrite 唯一写点零动;账单台账与链证守门零动
4. 启发式冻结:「若」字样条件形为冻结启发零裸奔,入 CONTRACT
5. 与 leftover-solo 串行;在途批避让排队
6. scribe 裸调逐笔 grep;退出码真值核;verify 主树二进制;任务包绝对路径;des-001 域外如实记

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle + close(过差集闸) + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/declguard-solo-results.md 与 materials/
- [ ] CONTRACT 修订与 CALL-LOG 双笔随批

## 八、风险点 {#risks}

- 请求写入节文本形驳杂（自由文本路径列举）:解析承 parse_requested_writes 既有形,解析不了即呈报告不猜
- 差集闸与既有 closeguard 让位判定交互:差集检查挂收约末位,前序闸零动,顺序在 CONTRACT 注记
- 存量收据迁移与在途批收据并发:迁移窗锁面评估,复用 ledger_lock 形

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 姊妹批:leftover-solo（存量清点移交）
- 上游审计:watchcheck 代理 2026-09-06

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/(core.py、cli.py)与 CONTRACT.md 与 CALL-LOG.md
- sih-tools/lease/tests/(五族夹具)
- sih-tools/lease/ledger/receipts/(新目录随批建,存量迁入)
- sih-engine/sih/event/plan/declguard-solo-results.md 与 materials/ 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/ 当日链
