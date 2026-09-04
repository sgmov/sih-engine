# docmath-b3-solo：文档数学化程序批三规则包逐条挂锚批

> task-packages 治理任务，承 docmath 程序（docmath-full-program-v1.md 与 docmath-task-package-2026-09-04.md）
> 队形：单线形 solo，确定性脚本与主线亲写零子代理
> 日期：2026-09-04

## 概览 {#overview}

- 批性质：挂锚批，纯注释与纯数据入包，规则谓词与退出码语义零变化，只加锚
- 挂锚面：核阅 des-001 rules.toml 十五条（双实现逐字节同源双改）、化格 general-v1 operations.toml L001、检词 core 登记面（pack 目录新增 anchors.json 纯数据锚位件，包装载器零消费）
- 判定性四候选（S002 与 S004 与 S006 与 N002）挂批二推导档与载体引用；纯约定八件标注环境参数登记面归依；C009 推导档指针补实指
- 验收：裸奔数为零机械复算（anchorcheck 脚本）、引擎重编零判变（golden findings 逐条等值）、三仓收约

## 一、问题陈述 {#problem}

批一分拣账本产出判定性四百三十八件与纯约定八件与待确认二十一件；批二两载体入仓。批三把判定性规则的载体引用落到规则包本体：des-001 四候选结构谓词挂 ALG-012 与 ORD-008 加批二推导档，L001 挂 ORD-023 加批二硬测试，检词登记面挂 ALG-002 与 ORD-024 加锚位件，纯约定八件标注环境参数归依不伪装数学。

## 二、关键设计 {#design}

- 挂锚形承 gatecap 先例：规则前注释块携载体 ID 与推导档实指路径；C007 与 C008 既有注记零触碰，C009 补推导档实指（gatecap 推导档与 SPEC-019）
- 双实现同步：sih-engine 与 sih-tools 两份 des-001 rules.toml 逐字节同源双改，引擎包 include_str! 内嵌故重编主树二进制
- 零判变验证：重编后引擎 golden fixtures findings 逐条等值（des-001-gov002 等九件）加 cargo test 全绿
- 检词面锚位件 anchors.json 纯数据新增（name 与 version 与 domain 之外键，装载器零消费），登记面锚 ALG-002 加 ORD-024 指批二推导档与 carrwire 推导档
- anchorcheck 脚本复算裸奔数：读三族规则包挂锚面与批一账本对表，判定性规则无锚即红，双跑逐字节一致

## 三、工作清单 {#work}

- [ ] 双实现 rules.toml 十五条注记挂锚与 L001 注记挂锚与 anchors.json 新增
- [ ] 引擎 cargo build 重编与 golden findings 零判变对表与 cargo test
- [ ] anchorcheck 脚本双跑逐字节一致裸奔数为零
- [ ] 叩问信号随批 register 通道登记消解
- [ ] 三步管线与 checkcite 与认证上链三仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 挂锚齐 | 治理 | anchorcheck 复算裸奔数为零，双跑逐字节一致 |
| F-2 零判变 | 工程 | 重编后 golden fixtures findings 逐条等值，退出码语义零变化，cargo test 全绿 |
| F-3 登记随批 | 治理 | 叩问信号全数消解，词表登记双面一致 |

## 五、必读文件 {#read}

- 必读 1：sih-math/docs/docmath-coverage-2026-09-04/ledger.json（批一分拣账本）
- 必读 2：sih-math/docs/docmath-carriers-derivation-2026-09-04.md（批二推导档）
- 必读 3：sih-engine/src/scrutinator/packs/des-001/rules.toml（C007 注记挂锚先例）

## 六、约束 {#constraints}

1. 规则谓词与参数与退出码语义零变化，只加注释与纯数据
2. 数学仓 mapping 与 entries 与 INDEX 零触碰；三工具 CLI 零改
3. 主树零直写（引擎包重编除外，重编属构建非源写）；词债不过夜
4. 上链遇锁即等待不绕行，链文件只经引擎 scribe 写位

## 七、请求写入 {#requested-writes}

- sih-engine/src/scrutinator/packs/des-001/rules.toml 与 sih-tools/scrutinator/packs/des-001/rules.toml
- sih-tools/formatter/packs/general-v1/operations.toml
- sih-tools/nomenclator/packs/core/anchors.json
- sih-math/docs/docmath-coverage-2026-09-04/（anchorcheck 脚本与读数）
- sih-engine/sih/state/plan/docmath-b3-solo.md 与 sih-engine/sih/event/plan/docmath-b3-solo-results.md 与批材料目录
- sih-tools/nomenclator/packs/core/terms.json（register 通道）与五件 CALL-LOG 与 sih-tools/scribe/reports/ 与 sih-tools/identity/reports/ 与 sih-tools/meter/counts/ 与 sih-engine/sih/event/trail/2026-09-04.ndjson

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，三仓段结算收约，对表读数在档
