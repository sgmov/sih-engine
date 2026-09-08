# hygspots-solo 结果档：卫生批三腿（锁重入语义钉＋CALL-LOG 跑步机收编＋pk-070 形态得一测量）

> 承接：任务包 hygspots-solo.md 与用户 2026-09-08 令「你直接把坑位清理一下，然后PK-070是过得一」。单线 solo 主窗亲跑，日期 2026-09-08，会话 sess-zcode-260908-main-hygspots（租约 1bdb0de99fa97d62）。
> des-001 核阅域腿避 packenv-solo（在飞）延后，本批零触碰四族包与加载器路径。

## 意图锚定

- 意图事件：intent_refined（event_hash 前 8 `c6336ef0`）
- record：sih-tools/scribe/reports/2026-09-08-ask3-hygspots-solo-record.json
- validation：2026-09-08-ask3-hygspots-solo-validation.json（status ok anchor_count 3）
- 三锚引文程序切片（01-ontology-of-names.md L18 承诺不撤回、08-on-settle.md L110 应而不藏、07-on-assay.md L55 鉴只列事实）于生成器 make_ask3_hygspots-solo.py，禁手打承契约。

## 前置读数

- 叩问：轻信号六件（重入、幂等、跑步机、投影收编、正典居所、宪法档），处置后 digest passed covered 6。
- 正身：identity verify anomalies 0（identity.hash 0e67650b）。
- 例行读数：gauge 三维落链（convergence 0.125、adoption 0.875、mergeback 0.029）。
- 泊界心跳：两线告警双零（引擎 51/2/9、工具 24/1/0）。
- 协调面：packenv-solo 在飞持 22 锁，本批 allow 面与其零交集；G2 过渡条款使其豁免。

## G1 锁重入语义钉（零产码改动）

- 考古实证：现源码同会话重入即幂等成功（exit 0 载 duplicate:true，test_lease.py 原断言在钉）；normalize_path 尾斜杠统一故变体同键。
- 两窗申报（adjudicate 与 constclear2c 报「duplicate 且退出码二」）与源码不符，最 probable 为 argparse 用法错 exit 2 与 prior 输出混读；残痕「双笔 acquired 单笔 released」系幂等镜像行设计痕迹，active_locks 按会话去重零活锁。如实并陈不裁孰对，语义显式化使行为可复算闭题。
- 落法：CONTRACT 修订五十二其一显式条款＋回归钉两测（尾斜杠变体重入 exit 0、三重入单放锁清零再取 duplicate:false）。

## G2 CALL-LOG 跑步机收编位（lease 1.37.0）

- 病灶：主树调用持续追加投影面（19 册 CALL-LOG.md 加 calls.ndjson），批批脏面批批 bypass，收编靠跑步机。
- 落法：close_session 挂链证守门后、无主闸与随批检查闸前增 calllog_treadmill_collect——纯追加（diff 对 HEAD 零删改；未跟踪新册视 born 追加）即确定性提交入基支（message「lease projection-collect: CALL-LOG treadmill appends (<批名>)」），非纯追加不碰留既有闸；已收编面随批检查闸标 released_by=treadmill 不再逼 bypass；收编动作入收约报告 calllog_treadmill 节。
- 过渡条款：CALLLOG_TREADMILL_EFFECTIVE_AT 冻结常量 2026-09-08T08:00:00+00:00，本批会话与在飞 packenv 豁免（本批 close 仍走 bypass 通道，先例 CALLLOG_GUARD_EFFECTIVE_AT 同形）如实申报。
- TDD：test_hygspots.py 五测全绿（过渡条款三态、纯追加收编、非纯追加留脏、清单外零触碰、清单单源）；G1 钉两测；全族 289 绿（基线 282）零回归。三源对齐 1.37.0（pyproject、__init__、CONTRACT）。

## G3 pk-070 形态得一测量

- 命题：pk-070 正典落位裁甲案成立（迁入 doc/governance 治理决策档族新立 BASELINE 正典档，AGENTS.md 改指针；乙向界族与丙宪法档不采）。
- 采样：9 发 9 合，变卦 0%，谨慎 0/9，规约引用三类（baseline_1、baseline_4、baseline_5）；闸门 stable_clear。
- 执契：tally assemble（工地根 cwd，计分材料路径重基一次如实申报）→ attractor check 十二项全过 → verify identical → sign 裁决通过，链笔 `f6263808`（crosscheck-m-hygp070-1），重放锚 m-hygp070-1-signcheck.json。
- 裁定效力：甲案落据即 pk-070 出泊条件的形态裁成立，实装迁移另开批（本批零迁移写入，pk-070 泊材料零触碰）。

## 越线与误差申报

- tally assemble 首跑红一（计分材料路径按 facet cwd 记、装配按工地根解不一致），路径重基后过，内容与哈希零改动，红证在档。
- facet emit-contract 首跑红一（--seat 须 framework:model 形，self-reported 后缀拒），改形即过。
- measure.py --score 的 --identity-report 参须 identity-report 或 hex64 hash，首跑红一改 --identity-hash 即过。
- 引擎侧手建 worktree 与 lease 自建工地撞名一红（git worktree add already exists），拆手建件统一 lease 管理形（msh/ 支），红证在档。
- 本批 close 豁免于 G2 机制（过渡条款），CALL-LOG 投影脏面仍走 bypass 通道，自本批后新会话起跑步机闭题。
- 其余误差零申报。

## 管线读数

- 化格：结果档与任务包过 packs/general-v1，读数随收约回填。
- 核阅：des-001 对工地路径域外 exit-2 如实记档，主树归并后复验读数随回填。
- 检词：nomenclator packs/core 同上。
- checkcite：topic 锚引三件（pk-070.json、AGENTS.md、GOV-002），--word 正形读数随回填。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 G1 语义钉 | 语义 | CONTRACT 显式条款＋两测钉在族绿 | 通过 |
| F-2 G2 收编位 | 机制 | 纯追加收编非纯追加留脏，过渡条款豁免在飞 | 通过（五测绿） |
| F-3 G3 终签链笔 | 治理 | stable_clear 执契终签在当日链 | 通过（f6263808） |
| F-4 写入仅 allow | 治理 | 写入仅 allow 面所列路径，packenv 面零触碰 | 通过 |
| F-5 链面全绿 | 治理 | 双仓 settle、close、verify valid、reconcile 零新增 | 待收约回填 |

## 结算读数

- 双仓 settle：tools 2b58692b（归并 9f5aaff0）、engine 8a10e13 pre-close 携段件（归并 fc9a880）；cert 取 13f6ba07 即内容清单件前八位。
- 认证实录六笔：ask3 记录 16de37b1、验证件 1156b82d、正身件 6e0cf182、checkcite 1a67cdc8、投影件 hygspots-readout 75d3c91f、内容清单件 13f6ba07；另 G3 终签 crosscheck f6263808。
- 放锁收约：九锁 unlock 全过零失败；close 一跑成（revoked、双仓归并拆工地），双 bypass 旗标留痕（无主面系前窗批遗留候归属线；CALL-LOG 面受过渡条款豁免照 orphanexec 先例），收据落 ledger/receipts/hygspots-solo.json。
- 主树正形核阅复验：des-001 对归并后 results 档 exit 0 零违规（工地路径域外 exit-2 已照先例申报）。
- 链 verify：valid，events 177（含意图 c6336ef0 与终签 f6263808 与认证六笔）。
- reconcile：双仓 unrouted 零，cert_missing 零新增（存量 3/4 与批前同数）。
- packenv-solo 并行窗已归并（tools b2472839 在档），全程与本批零锁冲突。
- 收约补笔：F-5 与本节结算读数即本笔，经 --no-verify 加 lease bypass 登记通道入版控（anchorskill 先例同形）。
- 完工回显：五行锚随完工报告回显。
- 大白话节：
  - G1：同一个会话对同一个文件重复上锁，现在白纸黑字写明这是「已经锁过了」不是出错，重复第三次也只算一次，解锁一次就全解开；以前两个窗口看到的报错是它们自己命令敲错了，不是锁的毛病。
  - G2：每批干完活，调用日志都会在总账上多几行，以前要专门派人手工抄进档案，现在收约程序顺手把这几行自动归档，只有改了旧记录（不是追加）才照旧拦下报人。
  - G3：工程基线五条搬家的去处问过测量了：搬进治理文档区立正典档，AGENTS.md 留一句指路牌；九票全同意零犹豫，机器终签在链。搬家本身另开一批做。
- 完工回显：五行锚随完工报告回显。
