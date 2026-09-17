# scrutpath-solo 任务包：des-001 工地路径域外误判修复（核阅器路径基修正）

> 令源：用户 2026-09-08 令「拉子代理全开」承坑位清理尾腿；病灶经三窗独立实证（acceptclose、constclear2c、adjudicate2 结果档同款申报）：des-001 规则包按调用 cwd 相对 glob 判定域，工地路径形（worktrees/<仓>/<批>/...）一律域外 exit 2，致 doc 类批核阅正形只能等归并后主树路径，批内无法即时核阅。会话号 sess-zcode-260908-forkC-scrutpath。

## 范围 {#scope}

1. 病灶定位：读 sih-engine/src/（scrutinator 二进制源码位）与 des-001 规则包数据件，机械定位域判定 glob 的路径基（cwd 相对或根相对），diff 复现三窗申报形。
2. 修法实施：域判定路径基改工作区根锚定（root 显式参或 anchor.py 同款上溯判根先例），worktree 路径归一化后与根相对 glob 匹配——工地内的 sih-engine/doc/... 即域内。修法最小化，域清单语义零变（des-001 只盖 sih-engine/doc 不扩面）。
3. TDD：先红（工地路径域外红证复现）后绿（工地路径域内判定）；主树路径既有判定零回归；退出码三值语义零变。
4. 引擎件重建：cargo build 落 target/debug/scrutinator，重建后对 des-001 金向量与既有测试族零回归读数在档。
5. 连带申报：若修复使 在飞批惯例（域外 exit-2 如实记档）失去必要，BATCH-FACE 勘误候选一条记档不实施（勘误走后续批）。

## 产出 {#outputs}

任务包本件、结果档 sih-engine/sih/event/plan/scrutpath-solo-results.md、diff 红绿证、金向量与测试读数、ask3 记录与验证件、正身件、租约与锁实录、意图链笔、认证清单、大白话节、机器可读投影 scrutpath-readout.json 认证上链、越线与误差申报、engine settle 提交号、链 verify 与 reconcile 读数。

## 红线 {#redlines}

- 域清单零扩面：des-001 域仍只盖 sih-engine/doc，本批修的是路径基不是域边界（扩面走另批）。
- 并行窗面零触碰：entrydocs（doc/guide 与 README）、baselineexit（doc/governance 与 AGENTS.md 与 pk-070 泊材料）、confreopen（confledger 与置信度面与 proposition 置信度格）。cargo build 与 baselineexit 的管线核阅存在二进制共用：重建前先确认无核阅进程在跑（pgrep 一查即证，读数入档）。
- 主树零直写（本任务包主窗预落除外），链文件只经引擎 scribe 写位，禁管道掩退出码，误差红证如实记档。
- 坑位必防：正身件在场直至 unlock 毕；close 无主闸与 CALL-LOG 闸非本批活面双旗 bypass 留痕不代收；des-001 对工地路径在修复前的域外红证如实记档不清洗。
