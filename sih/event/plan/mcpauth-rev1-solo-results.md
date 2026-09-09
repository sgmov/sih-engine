# mcpauth-rev1-solo 批结果档

## 概览 {#overview}

- 本批承用户 2026-09-09 简化裁定「我觉得没有那么复杂，我们的机制是为了防止善意的误操作，锁工作区，而不是做安全防护」与补预留节令，DES-015 修订二防呆形落档 sih-engine/doc/design/DES-015-mcp-http-multitenant-auth-v1.md，修订七点逐点有判词与依据，修订前现役版存批材料 des015-prev-foolproof-base.md 作对照
- 得一测量 m-mcpauth-2 九发九合变卦零谨慎零，规约引用 baseline_1 与 baseline_4 与 baseline_5 三类，闸门裁决清晰稳定 stable_clear，执契 attractor check 裁决通过 comply verify identical sign 终签上链 cf826465 经 confirm 单笔确认 index 152；旧测 m-mcpauth-1 存对照零触碰
- 会话 ef077fb894280586 八路径锁全取零冲突，意图笔 e6178c91，管线三步绿，checkcite pass 一次过，双仓 settle，收约读数见收约节
- 零实装代码红线达成：管理台只作设计定形，sih-tools/mcpline 与任何既有工具代码零触碰

## 修订七点判词摘要 {#verdicts}

- 点一令牌降为项目标识牌：明文短标识即可读名字牌，台账即明文登记册六字段 token_id 与 domain_root 与 scope 与 status 与 issued_at 与 issued_by，砍哈希落册与明文一次性交付与轮换双活窗口三件，停行即追加行末行为准
- 点二签发台进视图：mcpline.web 令牌管理台三动作签发列表撤销，无管理钥，签发撤销俱带一步确认防手滑，签发即台账追加行，管理台与面板静态页同栈
- 点三鉴权改识别语义：连接头带项目标识即可，Bearer 头形留守判词即 HTTP 惯例连接头承载位不进 URL 落日志与 SDK 原生支持与单端口形相容，错误直给教学语直说哪一因与修复动作，砍 401 五因不区分与反枚举形
- 点四威胁模型缩为防呆清单：四条即防错域与防误写与防并发踩踏与防不可追溯，每条指认误操作形态与既有防位；对抗面整章移除，概览一句带过本机制防善意误操作不做安全防护
- 点五留守五件不动：域绑定、租约五验与锁队列、链上归因信封记项目标识非哈希、撤销即停行、127.0.0.1 缺省形态表述非安全边界
- 点六修订记录载裁定原文与日期：修订二节全文照录裁定原话与六点与补令
- 点七多人多agent预留节：四缝只声明缝不预装机制，即凭证槽位不变律（身份解析单点未来名字牌升级签名牌只换牌不拆墙）与归因槽位（信封主体预留人级粒度扩展台账加行即扩不改历史）与传输缝（127.0.0.1 为部署缝非永久约束多人版绑址放开加 TLS 反代加通道认证）与签发台即人节点入口（三动作即未来 RBAC 动作集归因闭环到人），节末总纲四缝留齐加人即换牌加行

## 测量与执契读数 {#measurement}

| 项 | 读数 |
|---|---|
| gid 与席位 | m-mcpauth-2，ZCode:GLM-5.3:self-reported，九发；谱系即按用户简化裁定自 m-mcpauth-1 命题改写第 1 次，方向对己不利向即自曝安全面收缩并明示不防对抗面 |
| 采样 | 9 发 9 合，变卦 0%，谨慎 0/9，规约引用 baseline_1 与 baseline_4 与 baseline_5 三类 |
| 闸门 | 清晰稳定 stable_clear（判据 v3） |
| 标定 | temp_probe agent 模式 retired 件 /tmp 副本重跑 score 挂本批正身件，体温 0.0，violate 5/5 判违，knife bdy 1.0，判定可用，漂移无 |
| R5 配对 | 基线 identity bc94e78edb2757cd 与计分材料一致（哈希优先配对，本批当日正身） |
| 执契 | assemble 后 attractor check 裁决通过 direction comply，verify identical，sign crosscheck-m-mcpauth-2 落链 cf826465 index 152，confirm 在档 |
| 材料 | 合同与响应与基线与计分材料落 facet/contracts/m-mcpauth-2/ 工地位并以快照形复制入 proposition/DES/m-mcpauth-2/contract-snapshot/，topic 与飞轮 trail 与计分材料与执契材料与 check 报告与 signcheck 落 proposition/DES/m-mcpauth-2/ |

- 前置分道：事实面（修订二在盘七点齐、任务包七点在档、旧测对照材料在档）归确定性核对通道不入采样，采样只裁防呆定位与基线一四五与零 LLM 裁决位条款的规约相容性，不裁防呆与安全防护孰优
- 旧测对照：m-mcpauth-1 九发 stable_clear 终签 a4f11822 index 146 在链存对照，其测对象为修订一多域安全模型稿，本测对象为修订二防呆形稿，两测材料零互写零触碰

## 管线三步与书单对表读数 {#pipeline}

| 闸 | 首跑 | 修复 | 终态 |
|---|---|---|---|
| 化格 formatter general-v1 | exit 0 零改动 | C006 材料修复后复跑 exit 0 | 绿 |
| 核阅 scrutinator des-001 | exit 1 红证 11 笔全 C006 全角括号中文内容非法（红证留批材料 des015-rev-scrutinator-firstrun-red.json） | 全角括号成对转半角材料修复判定语义零改动 | exit 0 零违规绿 |
| 检词 nomenclator core | exit 0 零违例 | 修复后复跑 exit 0 | 绿 |
| checkcite | exit 0 verdict pass 一次过 | 无 | 绿 |

- 材料修复申报：核阅首跑红证俱材料形缺陷即全角括号字符类，按 facepatch F-1 材料修复即修复重核通道推进，判定语义零改动不停批；先红留痕纪律红证全量归档批材料

## 误差与红证申报 {#deviations}

- 标定基线重跑申报：m-mcpauth-solo 批标定基线 identity 2e50dded 系前一会话哈希，本批正身 bc94e78e 新会话新哈希，R5 哈希优先配对须当日当席，故 /tmp 副本对同一标定响应集重跑 score 入账本尾行出新基线，响应集系同席同温同命题集产出零重采，二十发作答零改写
- contract-snapshot 快照申报：facet/contracts/m-mcpauth-2/ 五件不在任务包第五节 allow 面，承前批先例预防性以快照形复制入 allow 覆盖的 proposition/DES/m-mcpauth-2/contract-snapshot/，facet/contracts 原位随工地拆除散失如实申报
- 其余误差零申报：ask3 双门首跑绿、叩问十词与十信号一致、租约八锁一次全取零 wait-turn 实发、管线认证未复跑、sign 一次过未复踩闸三坑位、取码全程重定向形零管道掩码

## 链笔与收约 {#settle}

- 意图笔 intent_refined e6178c91d3d2ea46（session ef077fb894280586）；终签笔 crosscheck_completed cf8264657c2cc529 index 152 经 confirm 确认；认证 append 四笔即管线报告 db106788 与计分材料 765f673b 与 check 报告 f1f494e0 与 signcheck 62a96aec
- 链 verify：status valid 157 笔（首哈希 25989caf 末哈希 62a96aec，即本批 signcheck 认证笔）
- reconcile 双仓：unrouted 0 与 cert_missing 0 即验收双零达成，reconcile 退出码 1 由跨批存量 alarm 族承载（tools unbypassed 125 加 session_orphan 19、engine unbypassed 90 加 session_orphan 24，与 mcpauth-solo 批读数同值俱先于本批在册候裁），本批增量零 bypass 零 unrouted 零 cert_missing
- 双仓 settle：提交号随段 2 补笔回填本节
- 收约读数与回锚：close 收据落 lease/ledger/receipts/mcpauth-rev1-solo.json，收约后读数入完工回报转述；批后回锚重跑五行锚入完工回报
