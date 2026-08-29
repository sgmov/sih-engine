# elicitimpl-solo：叩问组件实装批

> task-packages 治理任务
> 承接：sess-zcode-260830-elicitimpl 三问意图即 2026-08-30 链事件 2bba8215、用户同日令即实装批开工与租约模式 solo 双仓一批到底
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

SPEC-012 契约已冻即 check 与 digest 两操作与三扳机两半机械与两条嫁妆与退出码三值，代码缺位。实装按 DEC-013 外切环落 sih-tools 围堰，Python 纯标准库零第三方依赖，落差项即目录布局与宿主命令名与金向量在本批钉死不另开落差批。

## 二、关键设计 {#design}

家位 sih-tools/elicit 即 pyproject 与 src/elicit 与 tests 与 vectors 与 CONTRACT 与 CALL-LOG。宿主命令名 elicit 原形借词豁免。check 操作：--words 加 --input 喂未登记词查 terms.json 登记面、--topics 对 --recall-face 查零行、--conflict-words 对召回面与 --history-face 出矛盾与裁定冲突候选，信号件 ndjson 五字段按 signal_type 加 subject 字典序。digest 操作：契约逐信号查叩问处置标记即消解或升级，缺一拦。suspend 子命令落挂起记录，组件无代答路径。金向量一组输入输出对冻结双跑比对。落差钉死三项即宿主名 elicit、家位目录如上、金向量一件起步。

## 三、工作清单 {#work}

- [ ] tests 先红即 F-1 至 F-8 测试件
- [ ] src 实现转绿即 cli 与信号与闸与挂起
- [ ] CONTRACT 与 CALL-LOG 与 vectors 落位
- [ ] 任务包与结果档落位与管线在域件如实跑
- [ ] 认证走守卫旗标即出参全显与追加前查链
- [ ] 双仓段结算收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 未登记词 | 工程治理 | 登记面查无词机械检出、有词零信号、双跑逐字节一致 |
| **F-2** 召回零命中 | 工程治理 | 归档面主题零行检出、候选面照出 |
| **F-3** 消化闸 | 工程治理 | 缺处置契约拦退出码一、全处置过闸零 |
| **F-4** 量级相称 | 工程治理 | 信号 weight 字段轻与重在件 |
| **F-5** 无应答挂起 | 工程治理 | suspend 落挂起记录、组件无代答路径 |
| **F-6** 确定性 | 工程治理 | 金向量双跑逐字节一致 |
| **F-7** 退化不崩 | 工程治理 | 登记面或召回面缺席退出码二报件名 |
| **F-8** 外切形制 | 工程治理 | 家位 sih-tools/elicit、CONTRACT 在位、零第三方依赖 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-012-elicit-contract.md 即冻结接口
- 必读 2：sih-tools/locator 即金向量与家位体例先例

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链前必须等绿
4. 零第三方依赖
5. 出参全显与追加前查链

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-8 全过
- [ ] 结果档落 sih/event/plan/elicitimpl-solo-results.md，认证入链，双仓段结算收约

## 八、风险点 {#risks}

中文词边界即未登记词检测以 --words 显式喂词不做分词、分词归后续批。矛盾候选半机械即工具出面判在 agent、候选过宽致噪即 weight 与 conflict-words 显式收窄。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 开工令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 2bba8215
- 关联：SPEC-012 冻结接口、DEC-013 外切环、locator 金向量体例

## 十一、请求写入 {#requested-writes}

- sih-tools/elicit/
- sih-engine/sih/state/plan/elicitimpl-solo.md
- sih-engine/sih/event/plan/elicitimpl-solo-results.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
