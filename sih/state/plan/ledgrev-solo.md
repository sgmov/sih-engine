# ledgrev-solo：覆盖账本修订小批（mathpipe 审阅承接件）

> task-packages 治理任务
> 承接：主会话 2026-09-03 审阅报告三处伪读数与一处文档缺口，用户令委外执行
> 队形：单线形 solo——确定性脚本与主线亲写零子代理
> 日期：2026-09-03
> 温故检索：materials/recall-ledgrev.json 即 conclusion 与 experience 双档零命中如实记

## 一、问题陈述 {#problem}

mathpipe-a1-solo 覆盖账本经主会话审阅坐实三处伪读数与一处文档缺口。其一已实例化语义过宽即引擎六件同构 ID 清单来自规格引用与金向量夹具非数学载体，SPEC 前缀双义即 calculus 子仓 SPEC 条目与引擎规格档 SPEC 档撞形。其二判定性 39 混入疑似资源性约十件即并发上限与令牌上限与重试上限族。其三可指认未实例化为零是名匹配伪读数即程序档对挂表的概念级指认未被机械核验。其四两结果档与程序档与 summary 命题层段落缺失即机械三层检查四件全漏层。

## 二、关键设计 {#design}

1. 三态语义修订：概念 ID 白名单即数学仓五子仓 INDEX 磁盘实存 ID 全集；引用面分级标注三注即源码推导、金向量消费、规格引用；已实例化收紧为源码推导面引用白名单 ID；SPEC 双义消歧即引擎 src 引用位默认引擎侧除非白名单命中且引用上下文明示数学条目。修订规则全脚本可复算。
2. 判定性复核：资源性规则精化即并发上限与令牌上限与重试上限族名归资源性初判；每一归类变动逐件申报附依据，零静默改判。
3. 对挂表核验：程序档批四对挂即 lease 至 ORD-020 与 cascade 至 ORD-016 与 tally 至闭包良基与 selector 至谓词划分与 scribe 至 ORD-019 与 identity 至 ALG-002 加 PROB-013，逐对核概念 ID 实存于 INDEX 且条目磁盘实存，可指认未实例化按对挂核验重估。
4. 命题层补段：四件各补一节含立题立场一句与 PRO-07 应用映射即 A-A3.1 与 A-A4.1 与 A-A4.2 逐条与治理贡献自述，补段后过机械三层检查 all_covered。

产出 ledger-rev1.json 与 summary-rev1.md 与 env-params-rev1.json 落 sih-math/docs/mathpipe-coverage-2026-09-03/ 同目录，原三件不动保复算。

## 三、工作清单 {#work}

- [ ] 修订脚本即白名单构建与引用面分级与 SPEC 消歧与资源性规则精化
- [ ] 对挂表核验脚本与核验表
- [ ] rev1 三件落档，双跑逐字节一致
- [ ] 归类变动申报表逐件
- [ ] 四件命题层补段并过机械三层检查
- [ ] 补段件走化格核阅检词三步
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 修订可复算** | 工程 | rev1 规则全脚本化，双跑 cmp 逐字节一致 |
| **F-2 归类零静默** | 治理 | 判定性与资源性与三态每一变动在申报表逐件附依据 |
| **F-3 对挂锚点实存** | 工程 | 每对概念 ID 实存于 INDEX 且条目磁盘在，零命中如实记 |
| **F-4 补段过检** | 工程 | 四件补段后 check_three_proposition_audit all_covered true |
| **F-5 源码零改动** | 治理 | 三仓源码 diff 为零，写入仅请求写入节 |

## 五、必读文件 {#read}

- 必读 1：sih-math/docs/mathpipe-coverage-2026-09-03/ 三件与 ledger.json 全读
- 必读 2：sih-engine/sih/event/plan/mathpipe-a1-solo-results.md 与 parkreplay-solo-results.md 与 mathpipe-full-program-v1.md
- 必读 3：sih-math/llm-friendly-build/mapping.md 即白名单口径参照
- 必读 4：sih-tools/BATCH-FACE.md 即调用面与坑位

## 六、约束 {#constraints}

1. 原账本三件只读不动即复算基线保持
2. 待确认清单仍只呈报不代裁，新载体不立
3. 上链遇锁即等待不绕行，词债不过夜
4. findings 亲读禁管道掩退出码

## 七、请求写入 {#requested-writes}

- sih-math/docs/mathpipe-coverage-2026-09-03/
- sih-engine/sih/event/plan/mathpipe-a1-solo-results.md 与 parkreplay-solo-results.md 与 mathpipe-full-program-v1.md（各补一节）
- sih-engine/sih/state/plan/ledgrev-solo.md
- sih-engine/sih/event/plan/ledgrev-solo-results.md
- sih-engine/sih/event/plan/ledgrev-solo-materials/
- sih-engine/sih/event/trail/<开工实日>.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] rev1 三件在档且原三件字节不动
- [ ] 归类变动申报表与对挂核验表在结果档
- [ ] 认证入链，双仓段结算收约，对表读数在档
