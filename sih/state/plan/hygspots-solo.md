# hygspots-solo 任务包：卫生批三腿（锁重入语义钉＋CALL-LOG 跑步机收编＋pk-070 形态得一测量）

> 令源：用户 2026-09-08 令「你直接把坑位清理一下，然后PK-070是过得一」。单线 solo 主窗亲跑，会话号 sess-zcode-260908-main-hygspots。des-001 核阅域腿避 packenv-solo（在飞）延后，本批零触碰四族包与加载器路径。

## 腿一 G1 锁重入语义钉（语义显式化，零产码改动）{#g1}

考古实证在案：现源码同会话重入即幂等成功（exit 0 载 duplicate:true，test_lease.py:278 已钉）；adjudicate 与 constclear2c 两窗报「duplicate 且退出码二」与源码不符，最 probable 为 argparse 用法错 exit 2 与 prior stdout 混读，残痕「双笔 acquired 单笔 released」系幂等重入设计痕迹（active_locks 按会话去重，零活锁）。落法：CONTRACT 修订四十四显式条款（重入即幂等成功，重复非失败，exit 2 仅工具异常与用法错）＋回归测试补钉（尾斜杠变体重入 exit 0、三重入单放锁清零、放锁后再取 duplicate:false）。产码零改动。

## 腿二 G2 CALL-LOG 跑步机收编（lease close 投影收编位）{#g2}

病灶：调用留痕投影面（19 册 CALL-LOG.md＋calls.ndjson，直改车道合法文书类）由主树调用持续追加，批批脏面批批 bypass，收编靠跑步机。落法：lease close 增确定性投影收编位——无主闸前扫描两仓脏投影面，diff 对 HEAD 纯追加（零删改）即 git add 加确定性 message 提交入主树（追加行即真实调用记录，收编零语义损失），非纯追加（改写删除）仍拦走既有闸与 bypass 通道；过渡条款 CALLLOG_TREADMILL_EFFECTIVE_AT 冻结常量，会话 issued_at 严格晚于才受本位管辖，本批与在飞 packenv 豁免（本批 close 仍走 bypass 通道，先例 CALLLOG_GUARD_EFFECTIVE_AT 同形）。收编动作入 close 报告 calllog_treadmill 块载面清单与提交号。TDD 先红后绿。

## 腿三 G3 pk-070 形态得一测量 {#g3}

pk-070（工程基线五条正典居所迁离 AGENTS.md，用户记一笔待清令在泊）出泊条件即用户裁正典落位形态；用户 2026-09-08 令该裁走得一。命题（材料同权送测）：「工程基线五条与禁止条款正典迁入 sih-engine/doc/governance/ 治理决策档族即新立工程基线正典档 BASELINE-v1.md，迁移走 T6 管线与认证，AGENTS.md 相应节改指针；向界族与另立宪法档两形不采。」锚：pk-070 泊材料 context 原文、AGENTS.md 工程基线节现状、GOV-002 冻结清单。stable_clear 即终签落据，实装另开迁移批（本批零迁移写入）；刀锋即转人诊断零硬凑。

## 产出 {#outputs}

任务包本件、结果档 sih-engine/sih/event/plan/hygspots-solo-results.md、CONTRACT 修订四十四、测试件（G1 钉＋G2 红绿）、DES 单元格与 facet 材料与终签链笔、ask3 记录与验证件、正身件、租约与锁实录、意图链笔、认证清单（md 走内容哈希清单件先例）、大白话节、机器可读投影 hygspots-readout.json 认证上链、越线与误差申报、双仓 settle 提交号、链 verify 与 reconcile 读数。

## 边界与红线 {#redlines}

- 主树零直写（任务包本件主窗预落除外）；链文件只经引擎 scribe 写位；禁管道掩退出码；误差红证如实记档。
- packenv 面零触碰（四族包与加载器与 scruttinator 域）；在泊件材料零触碰（pk-070 只测量不写材料）；G3 命题与 G1 考古是材料非预设结论。
- 视图板块与审计追溯：大白话节＋投影件＋零裸断言。
