# recallfix-solo 批任务包：检索器引用抽取存量红清偿（f2）

> 令源：用户 0.9.0 发行裁定的带债放行配套（清单已知边界第五项载明修复归本批）；红源 basefix-solo 结果档存量红申报在档
> 形：solo 批独立立约独立收约，引擎仓代码批

## 一、使命

清偿 mem_recall_f_suite f2_refs_machine_verifiable 存量红：检索器引用抽取把红申报行内引用形字串（basefix-solo-results.md 第 63 行申报文本内 pk050sw-solo-results.md@3-9）录为活引用，致断言对已行漂移文件误校。终态 cargo test 全目标零败。

## 二、考古与判则

1. 考古先行：复现 f2 红与抽出该行的召回行全貌（reference 与 excerpt 实值），定位抽取位（retriever 或 locator_bridge 的 md 引用扫描），确认摘录取自申报行自身词面
2. 判则：修复形须是机械抽取规则（如红申报标记行内引用不录活引用、或引用行白名单语义），禁弱化或跳过 f2 断言本身，禁改 basefix 与 pk050sw 两结果档（链认证历史）
3. 规则落点与词形由考古证据定，申报于结果档；抽取行为变化牵动的索引与金向量随批重冻（diff 超界即停批候裁）

## 三、红线

1. 禁改 basefix-solo-results.md 与 pk050sw-solo-results.md
2. 禁弱化 mem_recall_f_suite 任何断言；测试只能因抽取规则语义收紧而自然转绿
3. 引擎码改动走完整管线与全测试族（cargo test 全目标含 99 秒慢套件）；金向量漂移只许哈希位与抽取输出位 diff
4. 与主窗发布执行并行：锁面含 trail 与检索器路径，冲突走 lease wait-turn 禁绕行禁 preempt；0.9.0 tag 已打不阻你，归并照常

## 四、写入面（allow 清单）

- sih-engine/src/retriever/** 与 sih-engine/src/locator*/**（考古指向哪改哪，超界停批申报）
- sih-engine/tests/mem_recall_f_suite.rs 仅限随抽取规则语义的读数面调整，断言强度只增不减
- sih-engine/sih/state/plan/recallfix-solo.md 与 recallfix-solo-prompt.md
- sih-engine/sih/event/plan/recallfix-solo-results.md 与 recallfix-solo-materials/
- sih-engine/sih/event/trail/2026-09-09.ndjson
- sih-tools/scribe/reports/、sih-tools/identity/reports/、sih-tools/proposition/DES/m-recallfix-1/

## 五、验收与完工回报形

- cargo test 全目标零败；三腿管线绿（引擎件域内）；认证上链、双仓 settle、reconcile 双零
- 完工回报：批名、座位号、链笔哈希、双仓 commit 哈希、scribe verify 全文、reconcile 增量、根因判词与规则落点、f2 前后对照、金向量重冻 diff 概要
