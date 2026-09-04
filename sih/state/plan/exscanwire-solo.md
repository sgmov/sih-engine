# exscanwire-solo：例扫挂点执行与 pk-052 出泊批

- 承接：m-exscanhook-2 九发 stable_clear 已执契机器终签落据（重放锚 sih-tools/proposition/DES/m-exscanhook-2/m-exscanhook-2-signcheck.json，链 sign 笔在 2026-09-04 当日链）；用户 2026-09-04 令「得一裁一过一执行一」。挂点位置为机制形之确定性后承：调用形登记落既有例行读数程序位，close 关键路径零新增闸。
- 日期：2026-09-04｜会话：sess-zcode-2026-09-04-exscanwire｜队形：单线形 solo，零子代理；与 newcarr2-solo 并联（施工面不相交，共享追加面 append 短持，pk-045 冲突模式成员）
- 温故检索：materials/recall-exscanwire.json 如实记

## 批件

- 件一 pk-052 出泊：出泊材料 pk-052-exit.json（disposition 记 A 案经 m-exscanhook-2 终签执行），出泊事件经引擎 scribe 停泊子命令上链（调用形先查 scribe --help 与 pk-050-exit 先例 b68b178a 同形），PARKING-v1.md 名册行状态改出泊附事件哈希。
- 件二 泊界投影照链补齐：sih-engine/sih/state/parking/materials/pk-052.json 自 sih/event/plan/mathscan-solo-materials/pk-052.json 照链补齐（mathscan 结果档申报 11.7 交后批照链补齐先例），内容以链上 park 事件与原泊材料为准零改写。
- 件三 调用形登记：sih-tools/BATCH-FACE.md 增「例扫日扫调用形（m-exscanhook-2 终签执行）」节——例行读数 gauge record 落链后同跑 rev3 双跑与 checkmath 四表核对，命令 verbatim（cd sih-math/docs/mathpipe-coverage-2026-09-04 后 python3 rev3_script.py 两跑 cmp 加 python3 checkmath.py），读数只看红灰，红即停批上报，零红灰如实转述；AGENTS.md 零触碰（例行读数条目入宪法另走治理，本批只登记调用面）。
- 件四 首跑实录：按件三调用形实跑一次，双跑 cmp 与 checkmath 读数落 materials/first-run-2026-09-04/（文件自 sih-math/docs/mathpipe-coverage-2026-09-04/ 只读复制或读数转写），sih-math 全仓零写入。

## F 清单

- F-1 出泊事件在链 verify valid，名册行与 materials 副本与链三面一致
- F-2 调用形登记在 BATCH-FACE 且命令经首跑实测可复现
- F-3 首跑读数在档（双跑 IDENTICAL 或漂移如实记，checkmath 红灰如实记）
- F-4 写入仅 allow：sih-engine 的 state/parking/materials/ 与 doc/governance/PARKING-v1.md 与 event/plan/exscanwire-solo-materials/ 与 event/plan/exscanwire-solo-results.md 与 state/plan/exscanwire-solo.md 与 event/trail/，sih-tools 的 BATCH-FACE.md 与 scribe/reports/ 与 identity/reports/ 与 meter/counts/；工程源码与 sih-math 零触碰

## 管线与机械链

ask3 → 双门 → 叩问 → 正身 → lease open --package exscanwire-solo → 锁（engine 面 exclusive 按路径；tools BATCH-FACE exclusive；共享追加面 append 短持）→ intent 上链（闸三 --sessions 必带）→ 主树活链认证先落 → 工地施工 → 化格→核阅→检词 → settle 前拷工地 → 双仓 settle --cert → close → reconcile → 当日链 verify。uv 调用前缀 env -u PYTHONHOME -u PYTHONPATH；禁管道掩退出码；close 外提交 --no-verify 加 lease bypass 登记；任务包与 dispatch 随批入版控。

## 禁区

工程仓源码零触碰；sih-math 零写入；AGENTS.md 零触碰；主树零直写；pk-052 泊材料零改写（出泊另立 exit 件）。
