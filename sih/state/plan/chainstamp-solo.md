# chainstamp-solo:链铸时戳与回执确认批

> 令源:用户 2026-09-07 令「同意」,承 2026-09-06 提问「租约在上链的时候总会因为其他并行agent的上链疑惑，如何解决，这个等于白吃了一轮token，而且花时间」与主会三解法设计（时戳自铸加写即回执加confirm 廉价确认）
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:引擎批（scribe 写路径）,治并行上链摩擦的机械解,每消一次重试即省一轮 token（基线五经济兑现）
> 前置:append.rs 已有 flock 排他（并发分叉本防住,本批零动该机制）,锁面在途即排队

## 一、问题陈述 {#problem}

- **时戳调用方自带与全局链序冲突**:事件排序时戳来自调用方（记录件 mtime 与笔内时间字段与 --at）,并行批各带各的表——超前时戳笔（先例批 C 06:12 未来笔）使后来者全被 TimestampNotMonotonic 拒收,agent 只能睡等真实时钟或刷时戳重试,每重试一轮即一轮 token 白烧（主会同日三历:park 拒收睡等 170 秒两次与笔时戳刷新两次）
- **写后靠 grep 找笔**:并行写间链上不断长出他人事件,agent grep 数目对不上即起疑,起疑即全文 verify,再疑再烧;读数面不唯一（rootanchor 虚惊即工地二进制假 broken）
- **确认无廉价通道**:全文 verify 是唯一法定读数,代价与疑惑度同涨

## 二、关键设计 {#design}

### 2.1 链铸时戳（写位单点）

- 落笔临界区内（既有 flock 段内）自铸排序时戳:final_ts = max(调用方 hint, 链尾 ts + 1ms),单调性由构造保证,TimestampNotMonotonic 排序拒收类从根消失
- 调用方时戳降级为元数据:park 与 direct 记录件自带时戳若与铸值不同即入 details.declared_ts 留档（新行新增 details 键,旧行零动,哈希公式零动即 timestamp 字段仍入哈希只是值改铸）
- --at 语义改铸提示:测试确定性靠 hint,凡 hint 低于链尾自动抬升,测试夹具据此适配如实记

### 2.2 confirm 子命令

- `scribe confirm --trail <链文件> --hash <前八位及以上>` 与 `--event-id <id>` 两形:单笔查询,回话一行即 found 加 index 加 event_type 加 hash;退出码 0 在档 / 1 未在档 / 2 用法错
- 唯一读数入口纪律:疑惑先 confirm,全文 verify 只归批收约与审计,一律主树二进制

### 2.3 写即回执纪律

- 落笔返回的 event_hash 即法定凭据:真退出码零（回执在手）即写入成立,grep 降级为审计手段不再逐笔必做
- USAGE 与 BATCH-FACE 勘误节承载纪律文:真退出码加回执即成立、confirm 查疑、verify 归收约审计、禁工地二进制读数

### 2.4 判定语义

- 时戳字段语义从调用方声明改为链铸即链面数据语义变更,过得一裁（facet 合同模式）;confirm 与回执纪律为纯增零裁

## 三、工作清单 {#work}

- [ ] T-1 铸时戳实装（append 临界区内单点,五写位俱过此点即 intent 与 certify 与 park 与 direct 与 record 系）
- [ ] T-2 details.declared_ts 元数据降级（park 与 direct 记录时戳留档）
- [ ] T-3 confirm 子命令（两形查询与三态退出码）
- [ ] T-4 TDD 先红后绿:并行写夹具（未来时戳 hint 与正常 hint 双写者俱成,链 valid,序即到达加单调）、旧行零动夹具（旧文件字节不变与旧形哈希复算一致）、confirm 三态夹具、单调拒收不可达夹具
- [ ] T-5 判定语义一裁（facet 合同模式）
- [ ] T-6 引擎全测零回归与主树重编验收与 USAGE 与 BATCH-FACE 勘误

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 铸时戳 | 未来时戳 hint 写者先落,后来者零拒收零等待即写即成,链 verify valid,时戳序严格单调 |
| **F-2** | 旧行零动 | 既有链文件字节零改;旧形事件 compute_event_hash 复算一致;哈希公式零改 |
| **F-3** | confirm | hash 前缀形与 event-id 形俱中,index 正确,未在档出 1,用法错出 2 |
| **F-4** | 一裁 | 时戳语义变更 stable_clear 过执契终签在链,near_threshold 呈用户 |
| **F-5** | 零回归 | 引擎 cargo test 全绿（既存加新增）;lease 195 侧零改动;活体验收即本批链笔时戳俱铸值且 confirm 查中 |

## 五、必读文件 {#read}

- 写位现状:src/event_stream/append.rs（flock 临界区即铸点）与 event.rs 与 park.rs 与 direct 位与 bin/scribe.rs
- 先例痛点:主会 2026-09-06 三历（泊拒收睡等与笔刷时戳）与 rootanchor 虚惊勘误（其结果档 §十一）
- 机械链:sih-tools/BATCH-FACE.md（含全部勘误节与 verify 主树二进制纪律）

## 六、约束 {#constraints}

1. 一裁红线:near_threshold 呈用户转主会不自行终签
2. 旧链行零改写零迁移;哈希公式零动;flock 机制零动只在其内加铸
3. lease 侧零改动零版本升;工具域零触碰
4. 引擎件合并后主树重编再验收;scribe 裸调真退出码;verify 主树二进制
5. 在途批（leftover 与 declguard 与 redkeep 与 toolincub 等）避让排队,链为共享追加面并行合法即并行不惑为本批题义
6. 测试夹具时序适配如实记不硬凑;des-001 域外 exit-2 如实记

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle + close + reconcile + verify(主树二进制)
- [ ] 结果档 sih-engine/sih/event/plan/chainstamp-solo-results.md 与 materials/
- [ ] USAGE 勘误与 BATCH-FACE 勘误与 CALL-LOG 双笔随批

## 八、风险点 {#risks}

- 既存测试依赖显式时戳序:hint 抬升语义下按新序适配,适配面如实记
- 铸点若漏写位（record 系与 crosscheck 系构造器）:以 grep 全构造位清单核对,漏位即红证
- 并行活体验收窗口:本批自身链笔与他批并行落链,铸值序即为生产读数

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游提问:用户 2026-09-06 并行上链疑惑与 token 浪费
- 同族:rootanchor 虚惊勘误（读数面唯一纪律）

## 十一、请求写入 {#requested-writes}

- sih-engine/src/event_stream/（append.rs 与 event.rs 与构造器位若及）与 src/bin/scribe.rs（confirm 子命令）
- sih-engine/sih/event/plan/chainstamp-solo-results.md 与 materials/
- sih-engine USAGE 与 sih-tools/BATCH-FACE.md（勘误节）与 sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/ 当日链
