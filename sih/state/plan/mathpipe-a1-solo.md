# mathpipe-a1-solo：数学管线全量载体覆盖审计批（mathpipe-full 程序批一）

> task-packages 治理任务
> 承接：用户 2026-09-03 全量裁定（pk-041 出泊）与 sih-engine/sih/event/plan/mathpipe-full-program-v1.md（M-1 至 M-4 四不变量）
> 队形：单线形 solo——确定性脚本与主线亲写零子代理
> 日期：2026-09-03

## 一、问题陈述 {#problem}

数学管线哲学到数学侧已密（161 载体全经终签），数学到工程消费侧实证为疏：sih-tools 全源码仅 gauge 一工具引用载体，facet 阈值 0.34 源码自注"工程经验值，未经假设检验"，gauge 契约挂 PROB-003 与 PROB-005 两张期票。程序批一产出全量覆盖账本，为批序定稿、M-3 清账与收尾批闸门提供唯一地图。

## 二、关键设计 {#design}

纯脚本四步，零行为变更、零 LLM、零网络、时间显式给参（承 gauge 契约纪律与路线档"零模型进消费链"）：

1. 机制枚举：口径即 sih-tools 编组（COURSE-v2）加引擎 bin 清单，逐工具出机制行（工具名、命令面、判定面）。
2. 常数枚举：扫源码中命名常数定义（THRESHOLD/MAX/BUDGET/CAP/LIMIT/TTL/SKEW/TIMEOUT 族）与比较位数字面量，逐条出常数行（文件、行号、名、值、上下文摘录）。
3. 分类：单点扰动判据——改该常数会改变治理判定（放行/拦截/配对/裁决/退出码）即判定性，否则资源性。语法启发式（比较位 vs 超时/上限参数位）做主分类；落入边界的件进**待确认清单**，账本内标"待裁定"态，不静默归类。提取与主分类全脚本生成零人工填项，待确认清单是唯一个人接口，其确认结果属后续批不属本批。
4. 载体匹配：机制行与判定性常数行对 sih-math/llm-friendly-build/mapping.md 与五子仓 INDEX 匹配，出三态 {已实例化、可指认未实例化、无可指认载体}；可指认件的载体条目路径须磁盘实存（承 mathfix4 四查"工程调用位路径磁盘实存"），零命中显式申报不虚构。

产出三件：覆盖账本 ledger.json（机制行加常数行加待确认清单）、环境参数登记面初版 env-params.json（资源性参数）、汇总 summary.md（人读，走化格核阅检词三步）。账本落 sih-math/docs/mathpipe-coverage-<开工实日>/，脚本与工件落本批 materials 目录，全部脚本可复算。

## 三、工作清单 {#work}

- [x] 件 0 记账：pk-041 出泊记账执行。记录件落 materials（pk-041-exit.json），裁决文即用户 2026-09-03 原话加 mathpipe-full-program-v1.md 承载。记账位经用户 2026-09-03 批准走方案 b 即开工实日链，前置依赖 parkreplay-solo 批交付 scribe park 多链重放
- [x] 件 0b 撞号改正：pk-043 温度直觉件重编 pk-044。出泊事件 disposition discarded 载用户 2026-09-03 批准文，重入泊 pk-044 内容原样，补 pk-044.json 材料，名册投影同步即 pk-043 行改 pk-044 加 intanchor 意图对齐件入历史住户
- [x] 件 0c 名册归位：pk-042 错线行从主线名册移除，工具线 sih-tools/PARKING-v1.md 补 pk-042 行
- [x] 枚举加分类加匹配脚本四件套
- [x] 覆盖账本三件落 sih-math/docs/mathpipe-coverage-<开工实日>/
- [x] 待确认清单逐件呈报（只列不裁）
- [x] summary.md 化格核阅检词三步
- [x] 认证上链双仓收约对表，链快照在末笔后

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 枚举零漏 | 工程 | 账本机制行数与 COURSE-v2 编组加引擎 bin 清单对表零差；常数行逐条可 grep 复算 |
| **F-2** 账本可复算 | 工程治理 | 脚本全量重跑与账本逐字节一致；提取与主分类零人工填项；待确认清单件全为待裁定态 |
| **F-3** 匹配可追溯 | 工程治理 | 每条"可指认载体"的 mapping.md 或 INDEX 锚点磁盘实存；零命中如实记零命中，账本无虚构锚点 |
| **F-4** 零源码改动 | 治理 | sih-tools 与 sih-engine 与 sih-math 条目源码 diff 为零，本批写入仅限请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/event/plan/mathpipe-full-program-v1.md 即程序文档与 M 不变量
- 必读 2：sih-math/docs/governance-math-route-2026-09-02.md 即路线档三档
- 必读 3：sih-math/llm-friendly-build/mapping.md 即匹配面（全读是消费面验收线）
- 必读 4：sih-engine/sih/state/plan/mathfix4-solo.md 即机械四查纯脚本先例

## 六、约束 {#constraints}

1. 脚本层零 LLM 零网络，时间显式给参
2. 数学仓、工具线、引擎线源码只读，写入仅限请求写入节
3. 上链遇锁即等待不绕行；链快照复制在最后一笔追加之后
4. 待确认清单只呈报不代裁；新载体不立（M-3 清账属后续批）
5. 词债不过夜

## 七、请求写入 {#requested-writes}

- sih-math/docs/mathpipe-coverage-2026-09-03/
- sih-engine/sih/state/parking/materials/pk-044.json
- sih-engine/sih/state/plan/mathpipe-a1-solo.md
- sih-engine/sih/state/plan/mathpipe-a1-solo-results.md
- sih-engine/sih/event/plan/mathpipe-a1-solo-results.md
- sih-engine/sih/event/plan/mathpipe-a1-solo-materials/
- sih-engine/sih/event/trail/2026-09-03.ndjson（件 0 pk-041 出泊与件 0b pk-043 撞号改正记账）
- sih-tools/PARKING-v1.md（件 0c pk-042 归位投影同步）
- sih-engine/doc/governance/PARKING-v1.md（名册投影同步）
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/tally/CALL-LOG.md
- sih-tools/meter/counts/

## 八、验收标准 {#acceptance}

- [x] F-1 至 F-4 全过
- [x] pk-041 出泊、pk-043 撞号改正、pk-042 归位三账在链在册，主线在泊六项转四项即 pk-016 与 pk-037 与 pk-039 与 pk-044，intanchor 意图对齐件入历史住户，工具线名册补 pk-042 行
- [x] 待确认清单逐件在结果档呈报
- [x] 认证入链，双仓段结算收约，对表读数在档
