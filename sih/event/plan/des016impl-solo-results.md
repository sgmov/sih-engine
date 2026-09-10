# des016impl-solo 批结果档：DES-016 意图形分离实装

> 批名：des016impl-solo（单线形 solo）
> 会话：a53cab126561a675（sess-zcode-260910-main-des016impl，意图笔 418f4ba3）
> 令源：用户 2026-09-10 令「开工」承 DES-016 设计档终裁；设计正典 sih-engine/doc/design/DES-016-mcp-pure-tool-intent-form-v1.md

## 一、判词总览

五件俱落地俱测试：lease 意图闸结构校验中性化（plain 与 ask3 双形同一必填集，摘除内嵌 scrutinator ask3 包与 ask3repeater 双验）、CONTRACT 修订五十七与 1.41.0 三源对齐、scribe intent plain 通道（--validation 对 plain 形豁免，拒因面收窄）、mcpline plain 模板与描述与 0.6.0 三源对齐、DES-014 修订二。三套测试族全绿零回归：lease 318、引擎 lib 189（含新增 plain 与 ask3 分派单测）、mcpline 100。plain 通道 E2E 三态实测在档。

## 二、测试读数（只列事实）

- lease 族：318 passed（含新增 test_des016 五件即 plain 过闸与 ask3 同通道与缺字段教学拒与 invalid JSON 拒与非对象拒；两处旧语义测试即引文不字节同拒定改结构缺陷拒定，系 DES-016 设计内行为变更的适配上绿；金向量冻结件版本位随 1.41.0 三源对齐刷新）
- 引擎 lib：189 passed 零失败（intent_status_tests 五件含新增 plain_form_without_validation_passes 与 ask3_form_without_validation_refused；首跑 21 失败系工地 target 缺 scrutinator 二进制即纯环境件，build --bins 后归零，与本批改动无涉如实记）
- mcpline 族：100 passed 零失败（init 落地断言三处适配 plain 模板件；四处日期钉死读数位改动态实日即存量测试缺陷顺手治好如实申报；临时放锁两笔跑套件后复锁承 mcpinit 先例）
- plain 通道 E2E：plain 记录零 --validation 上链成功且 intent_form plain 与 anchor_count 0；ask3 记录零 --validation 即拒 RecordMissingField validation；scratch 链 verify valid

## 三、F 锚定验收表

| F | 判据 | 实态 |
|---|---|---|
| F1 | plain 形过闸与缺字段教学拒与 invalid JSON 拒 | 过（test_des016 五件） |
| F2 | scribe plain 上链成功与 ask3 缺验证件拒 | 过（E2E 三态） |
| F3 | mcpline 模板断言与 validation 可选与两仓全绿 | 过 |
| F4 | CONTRACT 修订五十七与 DES-014 修订二与三源对齐与管线 | 过（DES-014 化格零改核阅零 findings 检词零违例） |
| F5 | 主树重编接线验收 | 收约后回填 |
| F6 | settle 归并 close，reconcile 双零，链 verify valid | 收约时回填 |

## 四、队形验证

单线 solo 实跑零子代理，形名相符零偏离。
