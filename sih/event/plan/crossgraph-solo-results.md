# crossgraph-solo：跨方核毕专属事件与旧投影清残批结果档

> 承任务包 sih/state/plan/crossgraph-solo.md 即 2026-08-30 链事件 0e88aeeb
> 队形：单线形 solo
> 日期：2026-08-30

## 概览 {#overview}

- 跨方核毕专属写入路径落地：引擎 crosscheck.rs 守卫与构建、书简 crosscheck 子命令、执契 sign 换专属落链即 1.3.0::[deliver](#deliver)
- graph 旧投影删除留痕：0.1.0 旧位清残、正式位 doc/CASCADE.json 承 0.4.0 不动::[cleanup](#cleanup)
- 并行批撞锁处置在案：AGENTS tally 行延期候补::[defer](#defer)

## 一、交付实录 {#deliver}

引擎照读数先例新增 crosscheck.rs：守卫冻执契核对报告十二字段形态，另从所指材料拈三即 topic_sha256 与 dc_fingerprint 与 n_shots 承 DES-011 载荷六扩展字段，事件载荷合计十五字段；材料经 --material 显式传参不猜路径即初稿相对解析在真件上碰壁后改显式；event_class 按处置机械分即裁决通过取仅记录余取可消费；doc_id 即 crosscheck-gid。书简 crosscheck 子命令带锁位守卫，真件狗粮即 m-namefit 签报经临时链落 crosscheck-m-namefit 即 15 字段与 record_only 与链 valid。执契 sign 子进程从 append 换 crosscheck，报告十二字段冻结面零动即 watch 既有重放零破，测试先红即 called[0] 断言红于 append 后转绿。引擎 70 测全绿含新增六、执契 13 测全绿。

## 二、清残实录 {#cleanup}

sih/state/graph/CASCADE.json 即 0.1.0 旧位投影删除留痕，删除即归位不改名。正式投影 sih-engine/doc/CASCADE.json 承章程与本日 tokencap 批 0.4.0 重建，双位名实分裂自此归一。

## 三、延期与边界 {#defer}

AGENTS tally 行的期票句改写因并行会话 rulesem-solo 持 AGENTS.md 锁延期，候补即对方收约后即时补笔，词债在册不过夜。旧档 m-namefit 为认证位承载即历史在链不改，本批后其重放经 watch 如实浮边界变更归人。执契报告的 R1 版本署名 VERSION 冻结 1.0.0 为规则血统位不动，包版本另升 1.3.0。

## 四、验收判定 {#acceptance}

- F-1 守卫件：过。十五字段冻结面、材料缺席拒、同输入同判、六形态负例逐字段定位
- F-2 专属落链：过。真件狗粮链 valid、event_class 机械分、doc_id 唯一形
- F-3 执契切换：过。sign 调 crosscheck 带 --material、拒签路径零动、13 测全绿
- F-4 清残：过。旧投影删除、链上留痕、正式位不动
- F-5 收口：过。契约修订二、认证入链、双仓结算收约、unrouted 零、AGENTS 行延期如实记

## 五、关联 {#related}

- 材料：scribe/reports 2026-08-30-ask3-crossgraph 与 elicit 信号件与管线报告
- 关联：tally CONTRACT 期票句、DES-011、读数先例批、tokencap 批、并行 rulesem-solo 会话
