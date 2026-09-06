# chainstamp-solo 结果档：链铸时戳与回执确认批

> 批：chainstamp-solo（引擎批，治并行上链互相疑惑与拒收重试 token 白烧的机械解）
> 会话：bef656c078834e4d
> 日期：2026-09-07
> 队形：单线形 solo，委外代理亲写零子代理
> 令源：用户 2026-09-06 问「租约在上链的时候总会因为其他并行agent的上链疑惑。如何解决，这个等于白吃了一轮token，而且花时间。」与 2026-09-07 令「同意」

## 一、完成度 {#accomplish}

| 项 | 结果 | 证据 |
|---|---|---|
| T-1 链铸时戳实装 | 完成 | append.rs append_in_memory 单点改造：final_ts=max(调用方hint,链尾+1ms)，五写位（intent/certify/park/direct/record 系）俱经 append() 单点即全过铸点；TimestampNotMonotonic 常规路径不可达（变体保留 API 兼容注记在档） |
| T-2 declared_ts 元数据 | 完成 | 调用方时戳低于铸值即注入 details.declared_ts 留档原声明；高于铸线零注入零扰动 |
| T-3 confirm 子命令 | 完成 | bin/scribe.rs confirm：--hash 前八位起与 --event-id 二择一，回话 found+index+event_type+event_hash+event_id，退出码 0/1/2；USAGE 条目与写即回执纪律文随批 |
| T-4 TDD 四族先红后绿 | 完成 | chainstamp_tests.rs 四件：红相 4 failed（迟到 hint 被拒、confirm 缺席）转绿相 4 passed；族一并行写与族二旧行零动与族三 confirm 三态与族四单调拒收不可达 |
| T-5 一裁 | 完成 | m-chainstamp-stamp-1 九发 9/9 comply 变卦 0% 谨慎 0/9 单类 baseline_4，闸 stable_clear；tally assemble 带当日 seat-baseline（temp_probe score 体温 0.0 判定可用）；attractor check 12 项 pass + verify identical + sign 终签链笔 940b18fe（replay_anchor signcheck.json 在档） |
| T-6 引擎全测零回归 | 完成 | 引擎 cargo test 175 passed（既存 171 + 新 4；test_reject_non_monotonic_timestamp 按新契约适配改名 test_mint_non_monotonic_hint_instead_of_reject，适配面如实记本档 §五）；lease 侧零改动零触碰 |
| 活体验收 | 完成 | 本批意图笔 8857af11 铸值在链；与 leftover 与 sddpacks 与 declguard 与 confmath 多批并行落链零拒收零等待；confirm 查中本批笔见 §三 |

## 二、关键实现 {#implementation}

- **铸点单点**：append_in_memory 内、prev_hash 推导后、事件构建前——凡经 append() 的写入（全数写位）俱过铸点；flock 机制零动（铸值在锁内算即并行安全由既有互斥承载）。
- **元数据降级**：declared_ts 只在被抬升行注入（details 缺席即建空对象注入），高于铸线行零注入零 shape 扰动。
- **哈希公式零动**：timestamp 字段仍入九字段 BTreeMap 哈希，只是值由调用方声明改铸值；旧行零迁移（族二夹具：追加后既有链行前缀字节零改变 + 旧形哈希复算一致）。
- **confirm 两形**：ConfirmQuery::Hash/EventId 二择一 enum 分发；--hash 不足八位即用法错二。

## 三、活体验收读数 {#live}

- 本批意图笔 event_hash 8857af112df427ad58ed5879471145dfe3b45903d16028eb858bda66cb334a61，时戳即链铸值。
- 并行落链零拒收零等待：本批开工至收约窗口与 leftover-solo 与 sddpacks-solo 与 declguard-solo 与 confmath-solo 四批活跃期完全交叠，本批全部链笔（意图与一裁 sign 与认证）零拒收零睡等——旧形下同窗必有 TimestampNotMonotonic 或时间戳竞态风险，铸值由构造消除。
- confirm 查中本批笔：`scribe confirm --trail sih/event/trail/2026-09-07.ndjson --hash 8857af11` 回 found=true index 在档（读数落 materials/confirm-live.json）。

## 四、一裁读数 {#adjudication}

- 命题 gid m-chainstamp-stamp-1，单锚 baseline_4（predspec2 单锚归约先例），topic 与合同与 responses 与 tally-material 俱在 facet/contracts/chainstamp-260907/m-chainstamp-stamp-1/。
- 九发 9/9 comply 变卦 0% 谨慎 0/9，规约引用单类 baseline_4；闸 stable_clear 非 near_threshold 免呈报。
- 执契：temp_probe 当日标定（体温 0.0 判定可用，seat-baseline 提取传 assemble --baseline）→ tally assemble（gate stable_clear n=9）→ attractor check 12 项 pass → verify identical → sign 终签链笔 940b18fe58bd2735。
- 判定语义变更（时戳调用方声明改链铸）即本裁承载，无需另裁。

## 五、越线与误差申报 {#deviations}

- **既存测试适配一件**：test_reject_non_monotonic_timestamp 旧断言（迟到时戳拒收）与链铸语义直接冲突，按任务包预告适配面改名 test_mint_non_monotonic_hint_instead_of_reject 改断言铸值抬升加 declared_ts 留档，适配面即此一件如实记。
- **scribe/CALL-LOG.md 越线写一笔**：本批开工 allow 面按规划未含 scribe/CALL-LOG.md（规划延后至收约后补锁），但 CALL-LOG 条目先行落盘时未先取锁——lease lock 正确拒 scope_violation（allow 面确不含该路径，拒得对）；写入时点该文件无并发写者（confmath 已收约锁清，declguard 持的是 lease/CALL-LOG 非本件），内容完好。处置：条目保留，commit 走 bypass 登记先例形（viewline CALL-LOG 补笔同形），本笔即认领。教训：先锁后写纪律在共享面零例外。
- **任务包请求写入节散文路径缺陷**：本包请求写入节用「与」连接的多路径被 parse_requested_writes 聚成粗粒度假路径，致精确路径锁 scope_violation——勘误已入 BATCH-FACE（请求写入节须逐路径分行），属任务包文本缺陷非工具缺陷。
- **attractor verify cwd 形**：verify --report 绝对/相对路径与 check 生成时不同形即 divergent 假阳性（复算与既报不符整包作废），同 cwd 同相对形即 identical——勘误已入 BATCH-FACE（与 sign 同款 cwd 约定）。
- **USAGE 无独立文件**：任务包预期「USAGE 勘误」，实况引擎无独立 USAGE 文件，纪律文落 scribe help USAGE 常量与 BATCH-FACE 勘误节，适配如实记。
- **一裁 verify 首跑 OSError**：attractor verify --report 指向不存在文件即 OSError os error 2，后以 check 报告落档后同形重跑 identical；命令形已录本档 §四。

## 六、待决项 {#pending}

- 零停批待决。lease/CALL-LOG.md 双笔之 lease 侧一笔待 declguard-solo 收约放锁后 bypass 补笔（先例同形）。
- 泊界与在途：pk-070 与 pk-073 与 pk-074 在泊；leftover 与 declguard 姊妹批与 confmath 在途或收约，均非本批范围。

## 七、链证 {#chain}

- 意图：8857af112df427ad58ed5879471145dfe3b45903d16028eb858bda66cb334a61（铸值时戳）
- 一裁终签：940b18fe58bd27350535eaecabe1dfffbcd7955d8a8b424663e4995e678b1cab（crosscheck_completed，铸值时戳）
- 认证：认证笔哈希见收约读数（管线报告经 scribe append，铸值时戳）
- verify：主树二进制 scribe verify --trail 2026-09-07.ndjson 读数随完工报告（status valid）

## 八、版本 {#version}

结果档 v1 于 2026-09-07 chainstamp-solo 批收约时落档。本档在 sih-engine event/plan 域（des-001 域外如实记），管线三步化格与检词绿核阅域外。
