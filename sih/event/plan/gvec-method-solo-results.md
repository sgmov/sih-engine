# gvec-method-solo 结果档：金向量方法论立项批

> 承接：任务包 gvec-method-solo.md 与执行指令 dispatch.md。批两件：件一全量金向量用法审计（纯机械），件二金向量方法论命题先裁后行（得一裁，三态分流）。
> 政策行承用户 2026-09-03「等我裁的东西首先过得一，得一有问题的异常才让我看」：件一纯机械闭环；件二 boundary 即进泊界不签，均不扰人节点，得一异常零上报。
> 队形单线形 solo，日期 2026-09-03，会话 sess-zcode-260903-gvecmeth（session_id 8e4e8065d863b287）。

## 意图锚定

- 意图事件：intent_refined `4419744b6f0b4a39b39dbaf394b36358`
- record_path：sih-tools/scribe/reports/2026-09-03-ask3-gvec-method-solo-record.json
- record_hash：`502c2f7828bbdd6aa5c28c636f356d7a792a9eec87c2adbec26f06bbae71f064`
- validation_report_hash：`eab796ac4221c02ed0e6c79a9b5bb3516b9669c56788ef03b02e59f886bdb31f`
- 三锚引文程序切片（07-on-assay 纯粹映照、06-on-canon 由松到紧、08-on-settle 应而不藏）于 ask3 记录，禁手打承契约

## 件一枚举计数与甲乙分类读数

枚举（六族）：
- F1 engine scrutinator golden tests：14 断言 / 12 冻结件
- F2 engine attractor golden：9 test / 12 scenario
- F3 engine route golden：10 test / 8 scenario
- F4 engine event_stream vectors：5 向量 / 5 class
- F5 tools vectors 目录：locator/elicit/parser 三工具五目录
- F6 results F 表：29 件 / 114 处金向量断言行

分类：
- 甲类（变更检测，声明范围内健全）：A1-A6 六族
- 乙类（隐含正确性主张＝越权，漏洞清单）：V1-V5 五件

乙类漏洞清单：
- V1 scrutinator 退出码语义标签断言（src/scrutinator/tests.rs:157、:107-113）
- V2 exit_compliant_zero 显式正确性声明（src/scrutinator/tests.rs:218、:221）
- V3 attractor route 退出码语义对表（tests/attractor_route.rs:114、:159、:175）
- V4 结果档 F 表以金向量零漂移为合格判据（结果档 29 件/114 处）
- V5 锚集形式化缺口（ORD-007 工程注意事项 1/4，锚集先行登记与局部通过不推全局）

材料：materials/enum-registry.json（grep 复算）+ materials/classification-findings.md（逐断言附原文行）。

## 件二裁决结论与三态实态

- gid：gvec-method-guard-1
- seat：ZCode:GLM-5.3-Flash（identity_hash 58e22070 复用 predsplit 标定，可用体温 0.0）
- emit-contract：contract.json（contract_sha256 `414ad91f...`，topic_sha256 `72f4dc33...`）
- 九发独立重采 responses.jsonl（responses_sha256 `cca4a988...`）
- score 闸：consensus_all（九发全 comply）through，依据族三类分散

逐发分布：
- r1-r9 decision 全 comply（合合合合合合合合合，变卦 0%）
- basis 分布：baseline_1×1（r6）、baseline_4×6（r1/r2/r4/r5/r7/r9）、baseline_5×2（r3/r8）
- 规约引用 3 类：baseline_1、baseline_4、baseline_5

闸裁 verdict：**boundary（打回重作）**——依据族三值分散 basis_consensus 挂，判据 v3。
三态实态：**进泊界 pk-046**，未签不执行 SPEC-017。

泊界登记：
- entry_id pk-046：金向量生命周期与覆盖方法论另拟
- 停泊事件：parking_entered `5f975d24-03a5-4d31-876a-d09a46f4b714`（event_hash `400e2cee...`）
- PARKING-v1.md 名册：pk-046 经书简 park 事件入链（权威在链），工具线 PARKING-v1.md 名册落笔不在本批 allow 域（F-4 限请求写入节所列），留待泊界复检批收编
- exit_condition：用户裁载体另立与条款组重订归用户裁，ttl 14 天

说明：件二裁量类边界分流为正常机械三态结果，非得一异常，不扰人节点。

## 认证清单

> 备注：书简 append 仅认 JSON 报告件（ReportNotJson 实测），md 交付件非书简认证对象；本批 plan/materials 恰在 des-001 治理域外（des-001 只盖 sih-engine/doc，域外 scrutinator 报 exit-2 如实记档不属违规），formatter 零改动过；链外命题区文件不属本批 allow 提交域。

| 件 | 类型 | 状态 |
|---|---|---|
| materials/classification-findings.md | 件一审计档 | formatter 零改动过 + 随批入 main 提交；scrutinator 域外 exit-2 如实记档（md，非书简 JSON 认证对象） |
| materials/enum-registry.json | 件一注册表 | 书简认证 `37206c13-0d0d`（event_hash `2931e3ea...`） |
| proposition/DES/…/contract.json | 件二合同 | 书简认证 `bcf937f9-0ef8`（event_hash `b9bd9c3c...`），文件本体在链外命题区待泊界批收编 |
| proposition/DES/…/responses.jsonl | 件二九发 | 链外命题区 jsonl，非书简 JSON 认证对象形态 |
| proposition/DES/…/contract-score-material.json | 件二计分 | 书简认证 `47990d5a-19ba`（event_hash `209b6f37...`），文件本体链外待收编 |
| facet/…/gvec-method-guard-1/topic.md | 件二命题 | 并入 tools main（md，非书简认证对象）|
| parking/materials/pk-046.json | 泊界记录 | parking_entered 入链 `5f975d24-03a5`（event_hash `400e2cee...`） |
| PARKING-v1.md | 泊界名册 | 不在本批 allow 域（F-4 限请求写入节），留待泊界复检批收编 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 枚举零漏 | 工程 | 金向量位点全量清单 grep 复算对表 fixtures/测试计数 | 通过（六族计数在档，enum-registry.json grep 复算） |
| F-2 分类可复核 | 工程 | 甲乙分类逐断言附原文行，乙类清单零含糊 | 通过（classification-findings.md 逐行附证据行） |
| F-3 命题合规裁 | 治理 | 全流程引擎件出裁至 check/sign/泊，对己不利双声明入档 | 通过（boundary→pk-046，前门全流程走完，topic 双声明在场） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列 | 通过（写入仅 allow 路径，见内文） |

## 冲突样本节

- 本批与 facepark-solo 平行共享写面（trail、scribe/reports、CALL-LOG、meter/counts）撞锁，有限重试承 pk-045 先例逐次计数换得放锁，未越红线。
- 件二 facet 测量腿依赖 pyyaml/openai 缺席，本地 venv 补装（pip3 install pyyaml openai）后 measure.py 恢复可用，属环境态处置非仓内变更。

## 越线与误差申报

- 件二前门得分闸 boundary：依据族三值分散（baseline_1×1/4×6/5×2）basis_consensus 挂，如实走泊支不签，非越线。
- 无越线项。误差零申报。

## 结算读数

- 双仓 settle（merge 形副本归并）：
  - engine main `94e4d30`：收编 materials/classification-findings.md、enum-registry.json、结果档与 trail 链（冲突贴补取分支权威版）
  - tools integral-stage-build `22c7aff8`：收编 facet gvec-method-guard-1/topic.md、facet/CALL-LOG.md、meter/counts/2026-09-03.ndjson
- 链 verify：settle 前 150 事件 valid（first_hash `94f1dd00...` last_hash `209b6f37...`）；settle 后收约补对表见完工报告。
- reconcile 双仓对表读数：收约后补入完工报告。
