# pk050sw-solo：检索缺省位切换批（确定性七项核对全绿后落切）

- 承接：用户 2026-09-04 预签设计「A/B 门过即切」+ 得一裁定材料 m-pk050-switch-1（boundary → 修订一通道分解 → 确定性七项核对全绿：预登记 A/B pass、semantic.py 仅 stdlib 零 LLM、零新增依赖、双跑逐字节一致、载体 PROB-017/ALG-011 终签在链、判变 pk-050 登记在链、词面回退位设计在位）。
- 日期：以开工实日为准（trail 用当日链，会话号 sess-zcode-<实日>-pk050sw）｜ pk-045 参与者
- 队形：单线形 solo，零子代理；开工前置硬条件：lease status 双零净态（与 packhyg-solo 同列接棒队列，按主会排艇序执行）。

## 批件

- 件一 缺省位切换：recall 调用位缺省路径切至确定性向量语义层（PROB-017/ALG-011 载体），词面路径降为显式回退旗标位、行为零改；测试先红后绿。
- 件二 金向量随冻：新缺省行为金向量重冻（携带重放寻径约定 V6 教训），旧词面行为金向量保留为回退位冻结件。
- 件三 七项确定性核对落据：核对清单材料（七项逐项证据）走 tally assemble→check→verify→sign 四步终签落链（承 newcarr 与 pk037impl 终签先例）。
- 件四 pk-050 出泊记账：scribe park exit（disposition promoted，ruling 载用户预签设计与得一裁定材料指引：boundary 通道分解、确定性七项全绿）+ 名册行更新 + 泊材料状态更新。

## 机械链

ask3 → 双门 → 叩问 → 正身 → lease open --package pk050sw-solo → 锁（施工面 exclusive；共享追加面 append 短持）→ intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 → settle 前拷工地 → settle --cert → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH；禁管道掩退出码；close 外提交 --no-verify + bypass。

## 请求写入（锁路径全集）

- sih-tools/wikirecall/（recall 调用位缺省旗标、tests、fixtures、goldens）
- sih-engine/sih/state/parking/materials/pk-050.json 与名册行（doc/governance/PARKING-v1.md）
- sih-engine/sih/event/plan/pk050sw-solo-materials/ 与 pk050sw-solo-results.md
- 当日链 trail（经引擎 scribe）
- 任务包与 dispatch 两件随批入版控

## 禁区

词面回退位行为零改；语义层模块 semantic.py 判定逻辑零改（只切调用位缺省）；主树零直写。

## 裁决点预案

切换实现若发现语义层调用位耦合缺陷，停批如实申报（泊界登记），禁为切而改判定逻辑——那是另一个批。
