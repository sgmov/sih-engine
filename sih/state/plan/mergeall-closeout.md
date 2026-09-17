# mergeall-closeout：融回补缺六腿成果的批结算收口

> 令源：用户 2026-09-16 「1、2开工」（批 commit 与治理收口两项一并开工）
> 范式：结算批——零新生成，前批（ZCode 动态工作流 dwfrun-be5d655b，2026-09-15/16）工程成果的收口归账

## 一、问题陈述 {#problem}

- 前批以工程面完成融回补缺六腿（parking 路由引擎位、locksview 级联直调、parser packs 落位、identity 137 超时闸、融回候批件暂存 55、ask3repeater 夹具竞态）与 SDD（DES-019）/TDD（SPEC-026）成文三审，但未走租约生命周期、未上链认证、未批 commit——成果在工作树里而治理账上无痕。
- 裸默认工具链复核已齐：cargo test --workspace 退出码 0（473 过 0 败 6 忽略，87 段）；Xcode 许可墙经用户接受许可解除（强制重编+真链接验证过）。
- 已知复发缺陷随批登记：引擎 identity verify 裸环境狂转 114 秒（96 秒系统态）后 SIGKILL 成 137，二进制（03:29）新于修复源码（02:15）仍复发；本批正身以围堰 identity 只读运行替位（anomalies 空）。

## 二、关键设计 {#design}

- 2.1 git 对账分类：本批六腿件与两文档入库；并发批 M 件（formatter/locator/nomenclator/parser/selector/tally 六 bin）、sih/event 与 sih/state 旧档、sih-tools 台账面一律零触碰留原批。
- 2.2 结算链序（BATCH-FACE 正典，引擎位执行）：认证（scribe append 绑批报告）→ lease commit settle 形（session+cert 挂接行过直提守卫）→ 放锁收约（close 过 SDDG 两道门 DEC-024）→ reconcile 与链 verify 对账。
- 2.3 认证先行于 commit：cert 哈希前八位进 commit 报文挂接行，机械对当日链核验（fail-closed）。

## 三、工作清单 {#work}

- [x] c-01：回锚 + 判据扫 + git 对账（191 项入口态在档）
- [x] c-02：正身报告（围堰 identity 只读替位，anomalies 空）
- [x] c-03：任务包与意图记录落批材料目录
- [ ] c-04：lease open 立约拿会话号
- [ ] c-05：本批件 git add 编暂存集并核对
- [ ] c-06：scribe append 认证笔落 2026-09-16 当日链
- [ ] c-07：lease commit settle 落笔
- [ ] c-08：lease close 过 SDDG 两道门
- [ ] c-09：reconcile 与 scribe verify 对账
- [ ] c-10：完工报告 + 回锚五行回显

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 测试基线 | 结算后裸默认工具链全量测试退出码 0（对照读数 473/0/6） |
| **F-2** | 认证在链 | 直改链笔/settle cert 哈希对当日 trail grep 命中（守卫 fail-closed 通过） |
| **F-3** | 对账干净 | reconcile unrouted 与 cert_missing 较批前零新增；scribe verify status valid |
| **F-4** | 围堰零写 | sih-tools 全目录零写入（identity 只读运行除外） |
| **F-5** | 版本钉死 | Cargo.toml version = "0.9.0" 结算前后一致 |
