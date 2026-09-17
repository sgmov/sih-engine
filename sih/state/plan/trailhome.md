# trailhome 批任务包：链册双居所——locks 缺省链册单居所漏拦修复

## 一、使命 {#mission}

承 gatefix-parallel 并联批簇K（正典：sih-engine/sih/state/plan/gatefix-parallel.md），兑现 pk-092 出泊条件「用户令立批走引擎域 locks 双居所并集件」。批名 trailhome 查册 unknown，--new-stem 甲表认领；概念锚申报：zh 链册双居所，派生 trail 即链既立语素加 home 即居所，无既裁 code 形即显式申报无承。

一件：

- 件一 sih-tools/locks/src/locks/cli.py 27 至 30 行 _default_trails 单居所（只枚举 sih-tools/scribe/trail 老家）改双居所并集，与 sih-tools/lease/src/lease/cli.py 214 至 219 行先例对齐即引擎新家 sih-engine/sih/event/trail 加工具老家两处并集；病灶即迁链后新家全部链证不入乐观锁基线，check_baseline 对 unknown_target fail-open，脏上游现势实际漏拦

F 锚定：F1 fixture 新家 trail 含哈希时脏上游被拦且先红；F2 老家 trail 路径零回归；F3 locks 全套绿。

## 请求写入 {#requested-writes}

- sih-tools/locks/
- sih-tools/locks/CALL-LOG.md
- sih-engine/sih/state/plan/trailhome.md
- sih-engine/sih/event/plan/trailhome-results.md
- sih-engine/sih/event/plan/trailhome-materials/

## 约束 {#constraints}

- 工地 lease worktree，主树零直写；TDD 先红后绿；locks 全套绿
- 禁区：不碰 lease 与 mcpline 与 attnanchor 与 parser 文件（lease/cli.py 只读对表先例）；不 push；bypass 仅既有通道 reason 必填；工具 exit 2 停手如实报
- locks 版本位随批升
