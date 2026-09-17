# orphanexec-solo：三裁定值处置执行批（P1 入账、P2 形态转换、P3 检查位实装）

> 治理任务包（实装类，串行 serial，DEC-018）
> 承接：m-orphanrule-1 三件裁定值已落据成规则（机器终签 d099dd46 stable_clear 在链），本批为执行面；用户 2026-09-07 令「开，你拉起子代理自己跑」即串行子代理执行、主线起草验收（orphanrule-solo 同形）
> 形声明：串行 serial——主会起草与验收，串行子代理持全量上下文连续执行链；落包前温故检索件落 orphanexec-solo-materials/recall-precedents.json
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- P1/P2 处置件悬置即 confpreempt-solo 收约被无主闸堵（两件在无主清单），规则已落据而执行面缺位
- P3 检查位已立则未实装，CALL-LOG 追加不入版控的行为缺口仍开
- 两仓残面（本批系任务包件、台账累积、链尾追加）部分即新规则定义的处置对象

## 二、关键设计 {#design}

### 2.1 Cluster A 微处置（补笔 bypass 通道逐笔登记，callloghyg 归账通道先例）

- A1（P1 执行）：attnanchor/anchor.py mode change（100644→100755）按裁定入账，--no-verify 提交加 lease bypass 登记事由载 m-orphanrule-1 P1
- A2（P2 执行）：calllog/calls.ndjson 现脏态整体入账（B 案形态转换：725 起草快照后的累积为多批权威腿正常追加，提交事由载归属说明），自此 B 案随批节奏为常设纪律
- A3（本批系谱系入库）：orphanrule-solo 任务包与提示词与命题生成器脚本与命题正文（若主树仍未跟踪）随批入库
- 残面悬置不动：chaingreen 与 doorprep 结果档 M 态归属未清悬置；confpreempt 任务包属在飞批零触碰；19 件 watchcheck 遗留候裁不动

### 2.2 Cluster B 检查位实装（P3 执行，TDD）

- 位点：lease closeguard 检查族（src/lease/ 下 close 路径守卫），新增 CALL-LOG 随批检查：close 时点扫描仓内 CALL-LOG.md 脏面（已修改或未跟踪），脏即拦并列文件清单；放行条件即该批 settle 提交含全部 CALL-LOG 脏面或显式 bypass 登记事由
- 语义申报（承裁定④基线兼容）：既有检查与退出码语义零改动，新增拦截条件即扩展；CONTRACT 升位随批；新增拦截的报文三要素（文件清单加缺件事由加处置指引）
- TDD：新检查先红后绿留痕；全测试族零回归（lease 269 基线）；对表腿 v1 形即脏面存在性检查（追加行逐行与权威腿对表留后继加强位，本批申报不越裁定文本）
- 双跑：检查判定同参双跑逐字节一致

### 2.3 解堵验证

- watchcheck 复跑：两件出清无主清单读数在档
- confpreempt 无主闸预检：残面清零读数在档（其 close 归其自身会话，本批零触碰）

## 三、工作清单 {#work}

- [ ] Cluster A 三笔 bypass 入账逐笔登记
- [ ] Cluster B closeguard 检查位 TDD 实装加 CONTRACT 升位
- [ ] 解堵验证两读数
- [ ] 管线三步、书单对表、泊界心跳、例行读数（若当日未落）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** P1 入账 | 数据治理 | anchor.py mode change 入版控，bypass 台账登记载裁定号 |
| **F-2** P2 形态转换 | 数据治理 | calls.ndjson 入版控零内容改动（提交即纯追加态快照），watchcheck 复跑两件出清 |
| **F-3** P3 检查位红绿 | 跨族治理 | 新检查先红后绿证在档，全测试族零回归，同参双跑一致，既有退出码语义零改动 |
| **F-4** 越界面 | 治理 | confpreempt 会话与工地零触碰；悬置面零代清；bypass 逐笔登记 |
| **F-5** 解堵读数 | 治理 | 无主清单残面清零读数在档（confpreempt 解堵条件成立读数） |

## 五、必读文件 {#read}

- 裁定源：`sih-engine/sih/event/plan/orphanrule-solo-results.md`（三裁定值全文）
- 实装位点：`sih-tools/lease/src/lease/`（closeguard 检查族：cli.py 与 core.py 与 guardcore.py）
- 通道先例：`sih-engine/sih/event/plan/callloghyg-solo-results.md`（归账 bypass 通道）
- 命令面：`sih-tools/BATCH-FACE.md`（含 2026-09-07 新四条）

## 六、约束 {#constraints}

1. 裁定即边界：只执行三裁定值覆盖面，P3 对表腿不越裁定文本自扩
2. confpreempt 会话、工地、任务包零触碰；其 close 归其自身
3. bypass 逐笔登记；主树零直写除登记通道；守卫禁 plain commit
4. 悬置面零代清；在泊件零触碰；遗留无主件不豁免
5. 全链照 BATCH-FACE，退出码直读，撞锁排队候叫

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] settle 提交号在档，收约后零活跃锁
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 orphanexec-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 检查位与在飞批惯例冲突（confpreempt 自身 CALL-LOG 未入版控即被新检查拦）→ 新检查生效面自本批后批起算，本批申报过渡条款
- anchor.py 与 calls.ndjson 同笔提交还是分笔 → 分笔逐裁定号登记，追溯清晰

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（写位）、`sih-tools/lease`（bypass 与 closeguard）
- 受益：confpreempt-solo 解堵；后继批 CALL-LOG 纪律机械化

## 十一、请求写入 {#requested-writes}

- `sih-tools/attnanchor/anchor.py` 与 `sih-tools/calllog/calls.ndjson`（裁定执行面）
- `sih-tools/lease/`（检查位实装与 CONTRACT）
- `sih-engine/sih/state/plan/orphanexec-solo.md`
- `sih-engine/sih/state/plan/orphanrule-solo.md` 与 `orphanrule-solo-prompt.md`（A3 入库）
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-engine/sih/event/plan/orphanexec-solo-results.md` 与 `orphanexec-solo-materials/`
- `sih-tools/scribe/reports/` 与 `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 相关仓 orphanexec-solo 工地
