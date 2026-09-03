# claimgate-solo 结果档

> 批：claimgate-solo（任务包领取登记闸：claims 账本 + lease claim/unclaim + ttl 过期自动标注 + open 闸一联动警示）
> 会话：d836b884425cd1ae
> 日期：2026-09-04（实日，链文件 2026-09-04.ndjson）
> 队形：单线形 solo，零子代理，单飞零并发
> 承接：用户 2026-09-03 原话照录「我在想是否任务包被领取也加个锁？」；主会判词即闸一到三仍为硬执法、领取登记为声明位非执法位、领取到立约之间碰撞真空由领取登记补可见性与提前避撞
> 意图哈希：642b1f557eef180993b2b9615015c813681582cd0e15274294793b40281882c3（意图事件，ask3 记录内容哈希 c30744a3，双门第一门 exit 0 零违规、repeater status ok 三锚）

## 一、完成度表

| 交付件 | 判据 | 结果 | 证据 |
|---|---|---|---|
| claims 账本 | sih-tools/lease/ledger/claims.ndjson append-only 五字段 | 成 | 行载 package 与 claimant 与 claimed_at 与 ttl_minutes 与 status 五字段，追加即唯一写点零改写，真账一笔在案（probe-1-claim-granted.json） |
| claim 子命令 | 同包未过期再领拒 PackageAlreadyClaimed 载领取人与到期时刻 exit 1 详情非空 | 成 | 红绿测试与真账活针双证（probe-1-claim-rejected.json） |
| unclaim 子命令 | 在领释放落 released 行，no_active_claim 与 not_claimant 两拦 | 成 | probe-3b 两件加测试族 |
| ttl 过期自动标注 | 判定时读时派生标 expired 僵尸领取不靠人清 | 成 | 真实时钟探针（睡 62 秒 claimed 变 expired）加 --at 覆写金向量双跑 |
| status claims 视图 | 在领与过期标注可见，缺旗标零键零噪声 | 成 | probe-3a 四字段读数加缺省 status 无 claims 键断言 |
| open 闸一联动警示 | 同包他人未过期领取载警示行不拒，零在领零噪声 | 成 | probe-4-open-warning.json 活针加 probe-4-control-zero-noise-open.json 对照 |
| CONTRACT 修订与 USAGE 同步 | 修订二十七升 1.16.0 三源对齐 | 成 | CONTRACT 调用面十三子命令加机器形态领取账本节加修订记录；pyproject 与 __init__ 同步 1.16.0 |
| 金向量 | 双领拒与过期放行两场景双跑逐字节一致 | 成 | 五案 verdict IDENTICAL（claimgate-solo-golden-vector.json） |
| 词债 | 叩问两词登记 established | 成 | 领取登记与领取账本两词条入工地 core 包（129 加 2 等于 131） |

## 二、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| **F-1 双领拒** | 同包未过期再领即拒载详情 | 过 | 真账活针 exit 1 载领取人 sess-zcode-260904-claimgate 与到期时刻详情非空；金向量 s1 双跑 exit 1 逐字节一致；测试双断言 |
| **F-2 ttl 过期** | 过期后可再领且旧实效状态标 expired | 过 | 真实时钟探针 62 秒 claimed 变 expired；--at 覆写金向量 s2 过期界后再领放行双跑一致；账本零改写历史笔原文保留断言 |
| **F-3 status 视图** | 在领与过期标注可见 | 过 | probe-3a 四字段齐（claimant 与 claimed_at 与 ttl_minutes 与 status）加 summary 三态计数；释放回转 released 在案 |
| **F-4 兼容** | lease 既有测试全绿 | 过 | 86 测全绿（76 既有加 10 新），既有 schema 与退出码语义零改，status 缺旗标零 claims 键，open 零在领零警示键 |

## 三、活体探针读数（主会验收位，证据留 materials/）

1. **同包二次 claim 必拒**：真账登记领取成（exit 0），同包同领取人再领即拒 exit 1 reason PackageAlreadyClaimed，detail 载 package 与 claimant 与 expires_at 三键非空；金向量双跑读数留证 probe-1 两件。
2. **ttl 过期路径**：真实时钟形即 ttl 1 分钟真领取后在领读数 claimed，睡 62 秒后读数 expired（读时派生零写动作）；--at 覆写形即金向量 s2 过期界前拒过期界后放行双跑逐字节一致，两形齐备。
3. **status 视图四字段与释放回转**：claimant 与 claimed_at 与 ttl_minutes 与 status 四字段在案（probe-3a），unclaim 后 status 回转 released（probe-3b 两件）；本批真账领取于收约后释放回转见收约附表。
4. **open 警示行**：探针领取人 probe-worker 在领时对既有包 ordwire-lease-solo 真实 open，输出载 claims_warning 警示行载领取人与领取时刻与到期时刻且 open 本身不拒（探针会话 2ad4395d8f06335d 随即收约 revoked 清场验零残留）；对照形即本批真实 open 时 claims 账本零在领，输出零 claims_warning 键零噪声（probe-4-control-zero-noise-open.json）。

## 四、管线读数

- 化格：CONTRACT.md 与双 CALL-LOG 与 terms.json 四手写件 exit 0 零改；四 py 变更件亦 exit 0 零改（general-v1 实测对 py 零出域外错，读数如实记）。
- 核阅：des-001 包对 CONTRACT.md 与双 CALL-LOG 与 terms.json 与 claimcore.py 与 test_claims.py 六目标实测 exit 0 零发现。
- 检词：core 包对四手写件加 claimcore.py 五目标 exit 0 零违例（两新词 established 在册）。
- 结果档本件管线：化格 exit 0、核阅 des-001 exit 2 域外如实记（event/plan 不在 des-001 域内，不属违规）、检词 exit 0（落盘后复验读数）。
- 词债：领取登记与领取账本两词 established 入工地 core 包随批入版控。

## 五、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 642b1f55 | 意图笔（scribe intent，meter 包裹） | ask3 记录 c30744a3 |
| 7ce7131b | 管线报告（2026-09-04-claimgate-solo-pipeline.json） | 报告件内容哈希 |
| d6216f3b | 金向量报告（2026-09-04-claimgate-solo-golden.json） | 报告件内容哈希 |
| b827fb72 | 探针报告（2026-09-04-claimgate-solo-probes.json） | 报告件内容哈希 |
| f90326e7 | 变更件报告（2026-09-04-claimgate-solo-changed-files.json） | 报告件内容哈希 |

认证一律先落主树活链，全 meter 包裹、闸三 --sessions 带。

## 六、冲突样本节（单飞如实记零）

本批单飞，开工前 lease status 验零活跃会话零在锁；八把施工面 exclusive 长持一射取获重试计数 0；共享追加面（trail 与 scribe/reports 与 meter/counts 与 claims 账本）append 短持即取即放，**零撞锁如实记零**；无他批在途，零插队零让位。

## 七、越线与误差申报

1. **任务包请求写入段与 lease 包解析器格式错位（既有）**：任务包 §五请求写入为 ｜ 连排段，parse_requested_writes 仅解析列表项致取零，scope_source 落 explicit，本批按 scriwire2 与 gatecap 先例显式 --allow 十路传全，零拦截零绕行；任务包文件本批零改动（日期行已由主会批前勘误）。
2. **cert 段 cwd 误置致 scribe append 首轮四笔 exit 2**：cert 循环误在 lease 工地调 meter，`uv run --project .` 解析不到 meter 可执行即 Failed to spawn，四笔零留痕零写入（trail 22 事件未动）；更正 cwd 后 pipeline 笔在无预持锁形落链（7ce7131b），后三笔在 trail append 预持锁形落链；首笔与后三笔锁形差异如实申报，单飞零并发无竞态暴露。
3. **探针误置主树两件即改**：金向量重放脚本与意图事件证据初版误写主树 materials，即时发现即移入引擎工地对应路径，主树 git status 复验仅剩 trail 运行账本在册脏，零直写残留。
4. **expired 读时派生设计裁量**：任务包设计「过期即自动标 expired」落为实效状态三值读时派生（status 与 claim 判定时按参照时刻标注），过期不落笔零改写承账本 append-only 零改写红线，避免读路径写与重复标记；设计裁量如实申报，判据（过期后可再领加僵尸不靠人清）全达成。
5. **真账 runtime 态**：claims.ndjson 随探针生成真账笔（登记一笔在领），settle 前一次性拷入 tools 工地随批入版控，收约后释放回转（released 笔）留主树活账与 locks/sessions 同款运行态。
6. **零停批事件**：全程无不可解释的门与闸拒绝，无工具 exit 2 异常（cert 段首轮 exit 2 为本人 cwd 误置已申报于第 2 条，非工具异常）；政策行在役即机械链全绿自行收口推进，结果档不设「等你令」节。

## 八、验收

- [x] F-1 至 F-4 全过
- [x] 金向量两场景双跑逐字节一致（verdict IDENTICAL 五案）
- [x] 单飞零并发如实记（冲突样本节零撞锁）
- [x] 认证入链，双仓结算收约对表（见收约附表）
- [x] 任务包与 dispatch 两件随批在版控（主会批前收编在案，本批零改动）

## 九、队形验证

单线形 solo 零子代理，全链由会话 d836b884425cd1ae 亲写，零 Agent/Task 派生。

## 十、收约附表（close 后回填）

- 本节由收约后 wip 提交回填：双仓 settle 提交号与归并号、reconcile 读数、链 verify 前后对表、真账释放回转读数。
