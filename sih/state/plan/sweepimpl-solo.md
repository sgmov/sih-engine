# sweepimpl-solo：租约扫残留子命令实装

> 治理任务包（代码实装类，单线 solo，DEC-018 选形制，委外执行代理亲写零子代理）
> 承接：用户 2026-09-07 令「残留的内容如果不需要人节点裁决直接就清了，需要裁的话过得一」+ 同日令「直接拉子代理做掉」；设计承同日主会话定案（五类普查、三态输出、--fix 走既有通道、既有语义零触碰）
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 多窗口多代理并发是常态工作流，协调态残留（死会话占册、僵尸锁、停滞检验文件、散位收据、无主分支工地）持续产生；今日人工清两批（四幻影会话经 ledger-repair 复正、两散位收据归家）证明判定全机械而触发靠人在场，违工程基线二三五
- 缺一个确定性程序：普查即出三态，自清件径走既有通道清除并留痕，候裁件升视图；LLM 零参与判定

## 二、关键设计 {#design}

### 2.1 子命令形态

`lease sweep [--fix] [--json] [--root ROOT]`，新模块 `src/lease/sweepcore.py`，cli.py 接线一节。退出码：0=净（零自清件零候裁件，净外信息不占位），1=有残留（--fix 后仍候裁件在列亦 1），2=工具异常。既有子命令与退出码语义零触碰。

### 2.2 五类残留与判定判据（全机械）

| 类 | 判据 | 自清件处置（--fix） | 候裁情形 |
|---|---|---|---|
| 幻影活跃会话 | sessions 视图判活跃而撤销证据在案（原 revoked 行、锁全 released、worktree 与分支零存、结果档已归档，四证取二） | ledger-repair 补录修正性 revoked 终笔（reason 载判据与令源） | 证据不足其二 |
| 僵尸锁 | held 锁属主会话已被上条判死 | 转列 takeover 处置清单并调用 | 属主会话活 |
| 停滞检验文件 | pid 探针四态（core.py 既有）：pid_dead 与 legacy_stale 即自清 | takeover 通道清除 | pid_active 与 legacy_active 即净 |
| 散位收据 | 包旁 `*.lease-check.json` 带 closed_at 即散位存量 | 未跟踪件径迁 receipts 正典位后删；git 跟踪件只列迁家指令不提交 | 无 closed_at（活窗口凭据） |
| 无主工地 | `msh/*` 分支与 worktrees 目录项对活跃会话名册零属主且对应结果档已归档 | 只列清单不删（git 写不属本工具） | 有未归并提交 |

### 2.3 边界

- sweep 本体不做 git commit、不走 bypass：跟踪件残留列指令转批处置；--fix 只动未跟踪件与既有通道（ledger-repair、takeover）
- --fix 每动作打印一行清单（类、对象、判据、通道）；--json 出机读全量
- 挂点接线（会话开始四件套、视图告警）另批，本批只出工具位；AGENTS.md 与引擎治理面零触碰

## 三、工作清单 {#work}

### Cluster 1：施工（worktree 内）

- [ ] sweepcore.py：五类判定与三态输出，判据复用 lockcore/sessionview/pid 探针既有函数不另写
- [ ] cli.py 接线 sweep 子命令；CONTRACT.md 修订一节（版本号顺延，附五类判据表与退出码）
- [ ] tests/test_sweep.py：夹具注入四类残件（幻影会话行、假散位收据、假孤儿分支、停滞检验文件）各归其态；--fix 清讫断言；活会话零触碰断言

### Cluster 2：管线与链

- [ ] 化格→核阅（des-001 域外如实记）→检词（新词 sweep 三态处置：未登记则批内登记或懒波申报如实记）
- [ ] 认证入链、双仓 settle、放锁收约、reconcile 与 verify
- [ ] 共享面撞锁（scribe/CALL-LOG.md 等被活批 confrevise 持有）走 wait-turn 有限候叫，候而不扰

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 夹具归态 | 数据治理 | 注入四类残件夹具，sweep 普查各归其态，三态输出与 §2.2 表逐行一致 |
| **F-2** fix 有界 | 治理 | --fix 清讫全部自清件、逐动作清单在档；对活批 confrevise 会话与其锁面零触碰；git 跟踪件只列指令零提交 |
| **F-3** 既有零漂移 | 跨族治理 | 既有测试套件全绿如基线；夹具仓 status 与 check 在 sweep 前后双跑 cmp 逐字节一致，退出码语义零变化 |
| **F-4** 基线绿 | 治理 | 对当前真实态 sweep：自清件零（今日两批人工清理后的实态），净外信息仅列未跟踪收据等已知项不误报 |
| **F-5** 管线绿 | 治理 | 化格核阅检词三步读数在档；认证、settle、close、reconcile、verify 全链 exit 0 |

## 五、必读文件 {#read}

- 命令面权威：`sih-tools/BATCH-FACE.md`（十四步逐命令与坑位注记）
- 域主契约：`sih-tools/lease/CONTRACT.md` 与 `src/lease/cli.py`（takeover/heartbeat/ledger-repair 既有通道）、`src/lease/core.py`（active_sessions 事件序视图、pid 四态探针）
- 先例：`sih-tools/lease/CONTRACT.md` 收据归家注记（declguard 规则）；本批温故检索两份零命中输出随材料入档（`/tmp/lesweep-recall-topic.json`、`/tmp/lesweep-recall-topic2.json`）

## 六、约束 {#constraints}

1. 零 LLM 调用堆叠替代确定性验证；判定全部机械
2. 零触碰：AGENTS.md、引擎治理文档、confrevise 活批锁面（basemgrimpl/acceptor/basemgr/facet/incubation/proposition DES 路径）、在泊材料
3. 主树零直写，施工经工地 settle 通道；守卫在位禁 plain git commit
4. 共享面撞锁走 wait-turn 有限候叫，候而不扰不代清
5. 结果档起草前按事件与时间轴跑温故检索取切面（SPEC-007 消费侧三）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话（本批自己的）
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 sweepimpl-solo-results.md 落 event/plan 含完成度表、F 表、队形验证一行

## 八、风险点 {#risks}

- cli.py 体大（900+ 行），接线须贴既有子命令形不另起炉灶
- wait-turn 死等风险：候叫带超时上限，超限如实记档收工上报
- 幻影会话判据取"四证取二"防误伤重开批次：判据实现须逐证可输出

## 十、关联文件 {#related}

- 令源：用户 2026-09-07 两令（见头部承接）
- 设计定案：主会话同日扫残设计段（五类三态 --fix 既有通道）
- 委外提示词：`sih-engine/sih/state/plan/sweepimpl-solo-prompt.md`

## 十一、请求写入 {#requested-writes}

- `sih-tools/lease/src/lease/sweepcore.py` 与 `cli.py`
- `sih-tools/lease/tests/`（test_sweep.py 与夹具）
- `sih-tools/lease/CONTRACT.md` 与 `CALL-LOG.md`
- `sih-engine/sih/state/plan/sweepimpl-solo.md` 与 `sweepimpl-solo-prompt.md`（批输入件经工地落位）
- `sih-engine/sih/event/plan/sweepimpl-solo-results.md` 与 `sweepimpl-solo-materials/`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`（append）
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`（append）、`sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 sweepimpl-solo 工地
