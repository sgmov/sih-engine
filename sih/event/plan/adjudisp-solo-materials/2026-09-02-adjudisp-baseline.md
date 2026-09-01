# adjudisp-solo 狗粮测量基线材料

> 批：adjudisp-solo 第二步狗粮测量基线先行，2026-09-02
> 命题：lease 应增 pre-commit 守卫拦无模板 plain git commit，--no-verify 显式留痕放行
> gid：adisp-guard-1
> 本档为既有判定基线导出，供 tally R1-R7 核对与对照，零新判定

## 一、直提病灶基线（goldfix 六笔）

sealwin3-solo 追认档（sih/event/plan/sealwin3-solo-results.md 三节）在案：plain git commit 直提违章六笔，engine 三笔加 tools 三笔，sealwin3 封窗界线前移封入存量，历史零改写。

engine 仓三笔（归并点 4146851 界后封入）：

1. 23df965 scrutmerge-goldfix-solo 整改批 settle：金向量脏目标补冻 6 件
2. 8f85112 scrutmerge-goldfix-solo 整改批 round-2 settle：包平价回正 S004 消息恢复
3. 9732b22 scrutmerge-goldfix-solo 整改批 round-2 完工报告

tools 仓三笔（归并点 91d14d4f 界后封入）：

4. 0238bc6b scrutmerge-goldfix-solo 整改批 settle：6 件 switch-stop 证据保全
5. 5bfb95b6 scrutmerge-goldfix-solo 整改批 round-2 settle：5 件 dispatch 明示 untracked 入仓
6. 0add10f3 scrutmerge-goldfix-solo 整改批 round-2 CALL-LOG 与 meter 补笔

病灶语义：直提绕 lease 使 settle 前后对表失效、归并无前缀、认证缺席，正是无模板 plain commit 无守卫的可复发通道。本批命题即针对此通道设 pre-commit 守卫。

## 二、封窗追认基线（sealwin2 与 sealwin3 先例）

sealwin2-solo（2026-08-31）：引擎界线 d2b4a24 前移至 22550a6，七笔已披露主树直写 unrouted 与十一笔链事故 cert_missing 封入存量，历史零改写，封窗语义承修订十二即对表起算窗前移不是身份追认。sealwin3-solo（2026-09-01）：界线再前移至切换批归并点 engine 4146851 与 tools 91d14d4f，被封二十笔逐笔 sha 加主题加定性全列追认档。

追认类裁决在本批 DEC-021 分派表中归过得一类：封窗令本身留人节点，封窗追认过得一。两先例的追认档即追认类裁决的既有判定材料。

## 三、reconcile 读数基线（本批开工时点实测）

- sih-engine：unrouted 0、cert_missing 0、sealed 0、session_orphan 0、total 10、base_source seal_line 4146851
- sih-tools：unrouted 0、cert_missing 0、sealed 0、session_orphan 0、total 8、base_source seal_line 91d14d4f

双零保持即封窗后净信号基线，本批收口对表以此为对照，零新增为过。

## 四、链上 crosscheck 终签基线（自证循环读数）

链上 crosscheck_completed 终签现仅两笔且均为判定器融回资格自证材料：

1. 2026-08-31 m-mbgate-scrut 裁决通过（事件哈希前八 a0bbcb40）
2. 2026-08-31 m-mbgate-deyi 裁决通过（事件哈希前八 bb30582e）

另有 2026-08-30 一笔无 gid 旧形事件（0e88aeeb），历史在链不改。零非自证机器终签即本批狗粮要回填的结构空位：本批 adisp-guard-1 若出 stable_clear 即链上首笔非自证 stable_clear。

## 五、基线结论

命题所治通道（plain git commit 直提）有六笔实证病灶，追认类先例两单，reconcile 双零基线在案，链上非自证终签空位在案。基线齐，测量开工。
