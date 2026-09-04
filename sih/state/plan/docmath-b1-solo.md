# docmath-b1-solo：文档数学化程序批一规则分拣账本批

> task-packages 治理任务，承 docmath 程序（sih-engine/sih/event/plan/docmath-full-program-v1.md 与任务包 docmath-task-package-2026-09-04.md）
> 队形：单线形 solo，确定性脚本与主线亲写零子代理
> 日期：2026-09-04

## 概览 {#overview}

- 批性质：纯脚本零行为变更，三族规则包全部规则机械枚举与逐件三态分拣，产出分拣账本
- 三族规则包：核阅 des-001（rules.toml 十五条）、化格 general-v1（operations.toml L001）、检词 core（术语三态登记面四件加 manifest 域声明面）
- 三态分拣：数学可锚判定性（候选锚位）、纯约定（环境参数登记面）、待确认（如实申报不硬凑），缺省从严判判定性
- 验收线：零人工填项、脚本可复算、复算逐字节一致、账本与规则包对表零漏项

## 一、问题陈述 {#problem}

文档规则当前以惯例运行，无归因账。批一立分拣账本回答规模问题：三族规则包里多少规则是数学可锚判定性、多少是纯约定、多少待确认，判定性规则的候选锚位是什么。账本是批三逐条挂锚与批二载体入仓的输入，在账本出来前不做规模估计。

## 二、关键设计 {#design}

分拣判据以声明形机械准则承载，全部由脚本从规则包文件机械导出，零人工逐件填项：

- 判定性面映射承 docmath 程序三层模型：结构谓词族（header_structure 结构查项与 nav_format）候选锚 ALG-012 形式文法与解析确定性，锚点必填查项加 ORD-008 引用可达；重写面（化格 line_ops）候选锚 ORD-023 抽象重写系统与确定性范式；术语面（检词四登记面）候选锚 ALG-002 等价关系与商集隔离加 ORD-024 区间序与串跨度匹配；判定性常数裸奔闸三类（C007 与 C008 与 C009）以包内注记已携载体与推导档指针为机械事实判已挂锚，锚 ORD-008
- 纯约定面：判定性扰动缺席的观感风格规则（标点、字符集、节序名、围栏语言、排版元素禁用）入环境参数登记面诚实标注为约定，不做伪装数学
- 待确认面：如实申报不硬凑，缺省从严判判定性（2026-09-04 人节点裁定）；检词 candidates 面零判定位消费面（check 与 elicit 均不消费）如实记
- 包装载器不消费的惰性残留件逐笔盘点申报，不藏漏项

## 三、工作清单 {#work}

- [ ] 分拣脚本与三件产出（ledger.json 与 env-params.json 与 summary.md）落 sih-math/docs/docmath-coverage-2026-09-04/
- [ ] 双跑复算逐字节一致入批材料
- [ ] 叩问三词随批经检词 register 通道登记消解，登记产物化格归一
- [ ] 化格核阅检词三步管线与 checkcite 书单对表
- [ ] 认证上链三仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 账本成立 | 工程治理 | 三族全部规则逐件三态入账，零人工填项，双跑复算逐字节一致 |
| F-2 对表零漏项 | 治理 | 账本枚举计数与规则包解析计数逐族相等，惰性残留件逐笔在案 |
| F-3 登记随批 | 治理 | 叩问三信号全数消解，词表登记四查过，登记产物化格归一绿 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/sih/event/plan/docmath-task-package-2026-09-04.md 即任务包
- 必读 2：sih-engine/sih/event/plan/docmath-full-program-v1.md 即程序立项文档
- 必读 3：sih-math/docs/mathpipe-coverage-2026-09-03/ledger.json 即账本模板先例

## 六、约束 {#constraints}

1. 全部产出脚本可复算，复算不符即失败
2. 三工具 CLI 与退出码语义零改，des-001 判定位零触碰，批一规则包只读
3. 数学仓 mapping 与 entries 与 INDEX 零触碰，哲学仓只读不代立命题
4. sih-math probability/ 在途残留件零触碰，与 mathpipe 与 measure-poly 不抢道
5. 上链遇锁即等待不绕行；主树零直写，待提交件经工地 settle 通道
6. 词债不过夜

## 七、请求写入 {#requested-writes}

- sih-math/docs/docmath-coverage-2026-09-04/
- sih-engine/sih/state/plan/docmath-b1-solo.md
- sih-engine/sih/event/plan/docmath-b1-solo-results.md
- sih-engine/sih/event/plan/docmath-b1-solo-materials/
- sih-engine/sih/event/trail/2026-09-04.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/terms.json（register 通道三词）
- sih-tools/formatter/CALL-LOG.md、sih-tools/scrutinator/CALL-LOG.md、sih-tools/nomenclator/CALL-LOG.md、sih-tools/scribe/CALL-LOG.md、sih-tools/lease/CALL-LOG.md

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，三仓段结算收约，对表读数在档
