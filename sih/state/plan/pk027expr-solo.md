# pk027expr-solo：句读 v2 表达式层批

> task-packages 治理任务
> 承接：sess-zcode-260830-pk027expr 三问意图即 2026-08-30 链事件 c399d5f1、用户同日双出泊令即 pk-027 出泊
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

v1 表达式内部为配平节点即深度分级出版的首版形态，级联边抽取与寻址消费方即将需要表达式层。泊档钉范围即运算符优先级分层与调用与字段访问与字面量，顺带词法四件中三件可纯数据承接。

## 二、关键设计 {#design}

词法三件即块注释有界嵌套至三层与原始标识符 r# 前缀与非 ASCII 标识符即 XID 近似字符类。文法件即 expr 全链分层从赋值到一元到后缀到初等即十三层、后缀含调用与字段访问与下标、初等含全字面量族与括号组与路径表达式，expr_chunk 有序选择先结构后配平保崩溃语料恢复语义。三件包 version 升 2。impl 取名语义距经核需令牌捕获属引擎面即明示延期另批。

## 三、工作清单 {#work}

- [ ] tokens 三件词法扩展
- [ ] grammar 表达式十三层
- [ ] 表达式测试新增与 v1 回归与崩溃零崩
- [ ] 金向量重冻与契约修订
- [ ] pk-027 出泊与管线认证与结算收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 表达式分层 | 工程治理 | 优先级树形正确即乘法深于加法、调用与字段后缀成节点、测试绿 |
| **F-2** 零引擎与回归 | 工程治理 | src/parser 零改动 git diff 空、221 件对表逐字节不变、崩溃语料零崩 |
| **F-3** 收口 | 链上治理 | 金向量重冻过、契约修订留痕、pk-027 exit promoted、认证入链 |

## 五、必读文件 {#read}

- 必读 1：packs/rust/grammar.json 即现行文法
- 必读 2：src/parser/peg.py 即组合子语义

## 六、约束 {#constraints}

1. 零引擎改动
2. v1 条目流逐字节不变
3. 双跑逐字节一致
4. 上链前必须等绿

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过

## 八、风险点 {#risks}

expr_chunk 换结构可能破崩溃语料恢复即有序选择保留配平回退、测试先行钉死再动。

## 九、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 双出泊令
- 链件：sih/event/trail/2026-08-30.ndjson 即意图 c399d5f1
- 关联：pk-027 泊档、DEC-016、级联 0.3.0 即 pk-033

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[配平节点]: 消解 即契约既用语直述不作登记
叩问处置[表达式层]: 消解 即 v2 深化层直述工作名不作登记
叩问处置[有界嵌套]: 消解 即正则深度上限直述不作登记

## 十一、请求写入 {#requested-writes}

- sih-tools/parser/packs/rust/
- sih-tools/parser/tests/
- sih-tools/parser/CONTRACT.md
- sih-engine/sih/state/plan/pk027expr-solo.md
- sih-engine/sih/event/plan/pk027expr-solo-results.md
- sih-tools/parking/materials/pk-027-exit.json
- sih-tools/PARKING-v1.md
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scribe/reports/
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/meter/counts/
