# lease-lockcore-green-solo 结果档

> 承接：任务包 sih/state/plan/lease-lockcore-green-solo.md 与用户 2026-09-13 令「做租约 tdd 开工」续程；SPEC-024 v1.1 腿一；lease-lockcore-solo 批红态三件转绿。
> 日期：2026-09-13　会话：060e904854b1e219　范式：单线 solo 委外零子代理

## 执行实录 {#execution}

| 阶段 | 命令 | 退出码 | 证据 |
|---|---|---|---|
| 实装 | src/bin/lease.rs 锁核 fixture 对等域五子命令 | 0 | cargo build 二进制在位 |
| T2 转绿 | cargo test t2_golden | 0 | 四测全绿即金向量归一逐字节四Receipt比对 |
| T3 转绿 | cargo test t3_exit_codes | 0 | status 0 加未知子命令 2 |
| T4 转绿 | cargo test t4_five_verifications | 0 | 二会话撞锁后至者 locked_elsewhere 拒 |
| 全量回归 | cargo test | 229 绿 2 红 | 两红为主树先存红见偏差三 |
| 意图笔 | scribe intent | 0 | 31254764f |

## 交付摘要 {#deliverable}

引擎锁核 fixture 对等实装约五百行：open（包解析加请求写入段解析加 stem 闸 pack-absent 形加正身与意图哈希加 gauge 探针加 git worktree 立约加会话行追加）、lock（活跃验加撞锁 locked_elsewhere 加同会话重入 duplicate 形加锁行追加）、unlock（持锁验加放锁行）、close（链闸 workspace_chainless 形加声明面分类加 SDDG skip 形加工地拆除加拆本吊销行加回执落盘）、status。金向量 inputs 两件入档即正身与意图原件字节固定。

## F 锚定对表 {#f-table}

| 锚定 | 判据 | 结果 |
|---|---|---|
| F-1 T2 金向量 | 归一后逐字节四Receipt比对全绿 | 过 |
| F-2 T3 退出码 | 0 与 1 与 2 三值对表 | 过 |
| F-3 T4 撞锁 | locked_elsewhere 理由码字面 | 过 |

## 偏差申报 {#declarations}

- 偏差一：实装覆盖域为 fixture 对等域即五子命令，生产级面（真实台账自举与链闸全族与 SDDG 全判据与 hooks）归腿一后继批，承 SPEC-024 腿切分。
- 偏差二：tool.version 承契约面字面 1.46.0 而二进制版本 0.1.0，承 SPEC-015 工件工具名先例即工件工具名版本字段是契约面标识非二进制身份。
- 偏差三：全量回归两红即 scrutinator cli_positional_form_matches_golden 与 golden_des001_gov002 经主树对照验实为先存红非本批引入，处置归核阅金向量卫生候批。
- 偏差四：gauge 探针 all-present 形与 chained workspace 门族未实现即 fixture 域不触发路径，触发即退出码二显式拒不静默，覆盖域收窄承 SPEC-024 腿切分条款。
- 零偏差显式申报位：除上列四条外本批零其他偏差，本节词形承载承 DEC-024 SDDG-3 纪律与 SPEC-024 腿切分。

## 后续待令 {#next}

- 腿二收约执法面（commitcore 与 sddgate 与 guardcore 对表）与腿三附件面候令；SPEC-024 同参形 session_id 归一项修订随腿二批承载。

## 结算节 {#settle}

- wip 提交 facee9d，认证报告经 scribe append 后锁面内落盘，先例同形。
