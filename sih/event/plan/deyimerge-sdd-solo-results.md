# deyimerge-sdd-solo 结果档（得一融回规格批）

> 批名：deyimerge-sdd-solo。日期 2026-09-02。会话 8938df12（lease 1.12.0 双仓）。
> 意图事件 09d30179（锚三门前会话已绿续用：ask3 记录 98a91eb9 双门过与正身 b75de4d2 零异常与叩问 digest passed covered 6）。
> 承接：用户 2026-09-02 得一裁融回令即开门，融回三步曲第一步只落 SPEC-014 零实现；重派续跑即前会话 23b178d1 撞锁停批后陈旧会话拆除重开。

## F 锚定验收

| F | 判据 | 判定 |
|---|---|---|
| F-1 规格 | SPEC-014 七节全、两工具契约逐条对表、家位 attractor、双模条款在场 | 过（家位与模块形、接口契约对表、双模并存条款、腿切分清单、验收判据、金向量条款、回迁债、测试计划全载；crosscheck 十五字段负载逐字段列即报告十二加拈三；采样腿 emit-contract 与 answer 与 score 与机械腿六子命令逐条零漂移；家位 src/attractor/ 加 src/bin/attractor.rs 承 DEC-020） |
| F-2 教训承接 | 金向量脏目标条款与同参形条款显式在场 | 过（SPEC-014 金向量脏目标与同参形条款节承 SPEC-013 修订四原文引用即「金向量六件全零发现净目标对结构与语义天然免疫等价性从未被字节级钉住」，净一加脏三以上与双侧绝对路径同参两条款在场，A1 与 T1 挂钩） |
| F-3 管线 | 化格核阅检词域内零违规、认证入链 | 过（SPEC-014 化格 exit 0 零改；核阅工地路径 exit 2 域外如实记，同哈希内容 fixtures 域内重跑 exit 0 findings 0；检词 core 0.6.0 零违例；管线报告逐件认证入链） |
| F-4 收口 | 双仓 settle 归并、reconcile 双零、链 valid、包档随批提交 | 见链段 |

## 规格实录 {#spec}

SPEC-014 得一融回落差规格落 doc/spec，零实现即引擎与两工具源码零碰。腿切分清单二十三行即融回十一加留堰十二：融回面 contract_mode 加 facet_stats 三子模块与门面加 compiler 加 validators 加 anchors 加 model_utils 加 paradigm_loader 加 tally cli 与测试面；留堰面 llm_client 加 engine.py 加 runner 加 env_loader 加 req 加 config 加 thinking_resolver 加 concurrency 加 persistence 加 audit 加三腿 CLI 与 probes 加 timestamp。留堰核心理据即 engine.py import openai 实证 LLM 调用腿本体，机械腿融回闭包零依赖 llm_client 与 req 与 env_loader 即依赖面天然可切。跨腿契约即计分材料字段经 assemble 至 tally-check-input 逐字段兼容，引擎 signcheck 经既有 guard_crosscheck 零改通过。

## tally 五修对表差异申报 {#tally-delta}

tally 承 listzero-solo 五修即签署印守卫落 cli.py 与 CALL-LOG 而 CONTRACT.md 修订记录未载，本规格对表以修后现文为准即 sign 三态 refused 与 failed 与 signed，failed 形即 scribe 非零退出打 sign failed 带 scribe_exit 不打 signed 退出码如实透传；SPEC-014 接口对表 sign 节与 A2 退出码判据按修后形态落档，CONTRACT 现行文与源码现文的承载差异如实申报。

## 误差申报 {#deviations}

1. 前会话陈旧会话 23b178d1 (issued 零锁零提交) 经 close 拆除即空副本删支重开新会话 8938df12，无损。
2. 任务包检词自愿核检出 2 findings 即死词两处 (粤义项一词与 SPEC-013 文件名核阅英文对子串)，包在检词 core 域 include 之外，照 scrutmerge 系先例随批原样提交不改，报告 nom-pkg.json 在材料。
3. 主树链尾前批在途未提交十事件即 legacytwo 系 certification 与 intent_refined 尾随本批随 settle 提交承载，guardhook 先例同形。
4. tools 侧 CALL-LOG 四件主树在途 legacytwo 行回灌工地随批提交即归并并集无损，guardhook 回灌先例同形。

## 链与收口 {#chain}

意图 09d30179；认证见 trail；cert 锚取末笔；双仓 commit 指副本，close 归并，reconcile 对表。

## 后续 {#next}

融回三步曲第二步即 attractor TDD 批按 SPEC-014 腿切分清单与 T1 至 T6 先红后绿，金向量冻结含脏目标；第三步切换批双模并存以生产 trail 全量复验双跑一致为判据，GOV-002 判据一判定器席落 src 即门开全量。
