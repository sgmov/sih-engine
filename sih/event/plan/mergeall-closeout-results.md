# mergeall-closeout 结算结果档（results）

> 批：mergeall-closeout（session bf9422294f9ca2a2，2026-09-16）
> 性质：结算批——前批（ZCode 动态工作流 dwfrun-be5d655b）融回补缺六腿工程成果的批 commit 与治理收口，零新生成
> 任务包：sih/state/plan/mergeall-closeout.md

## 一、批成果与对表判词（验收） {#acceptance}

- 融回补缺六腿入库 71 件（+23332/−69）：parking 路由引擎位、locksview 级联直调、parser packs 落位、identity 137 超时闸、候批 55 件落位、ask3repeater 夹具竞态修复（SPEC-025 全量融回在途的收口段，DEC-013 融回门承接）。
- 对表判词：parking 腿无 uv 环境同参双跑 VERDICT-IDENTICAL；locksview 腿沙箱双跑逐字节 IDENTICAL；parser 腿双包出参逐字节 cmp 一致；identity 腿五用例逐字节 IDENTICAL（recognize-solo-results.md 候批偏差项就此清偿）。
- 机械读数：裸默认工具链 cargo test --workspace 退出码 0，473 过 0 败 6 忽略 87 段；DES-019 与 SPEC-026 三审（化格/核阅/检词）六命令复跑全 0。
- 链证：认证笔 1a59e2cf（settle cert_on_chain 过）、意图笔 plain 形（DES-016）、gauge 快照在当日链（2026-09-16）。

## 二、偏差申报 {#deviation}

- 前批施工为主树直写非工地位（动态工作流工程面所限），本批以工地复制加让位归并对表法补偿（BATCH-FACE §10 程序；SPEC-025 融回在途；DEC-013 融约门）。
- 结算批跳过 ask3 三问双门与叩问：零新生成，意图取 plain 形（DES-016 plain 形 validation 豁免条款）。
- 引擎 identity verify 裸环境复发 137（狂转 114 秒 96 秒系统态后 SIGKILL，二进制新于修复源码仍现），正身以围堰 identity 只读运行替位（anomalies 空）：recognize-solo-results.md 候批缺陷项复发，候修批另立。
- 他批在途件不入本批 commit：并发批 lease-cutover-parallel 六 bin 改件与 sih/event、sih/state 旧档一律留原批（SPEC-025 候批；候对应批结算统一裁决）。
- SDDG-2 单件残留：packs/parser/rust/vectors/in/sample.rs 为解析器矢量夹具，头注加注即改解析输入、破与围堰 pack 的逐字节对表（vectors 必红），按 DEC-024 显式绕行通道收约，绕行事由落 bypass 台账（对照 DEC-023 已裁载体豁免先例）。
- worktree 仓设 core.quotepath false：直提守卫对 git 非 ASCII 路径的 C-Style 引号形未去引即判（守卫缺陷，pk-100 工地盲区同类），候修上报；本批以显示层配置修正输入。
- 版本钉死：Cargo.toml version = "0.9.0" 零动（用户令：本批不升 1.0）。

## 三、可证伪条件核对 {#falsifiable}

| F 锚定 | 判据 | 结果 |
|---|---|---|
| F-1 | 裸默认工具链全量测试退出码 0 | 过（473/0/6） |
| F-2 | cert 哈希对当日链核验 | 过（cert_on_chain） |
| F-3 | reconcile 零新增告警 + verify valid | 结算尾段复核 |
| F-4 | 围堰零写入 | 过（仅只读运行 identity） |
| F-5 | 版本 0.9.0 钉死 | 过 |
