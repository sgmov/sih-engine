# baseinject-solo：标定上下文注入切换与温度探针拔闸批

> 治理任务包（工程类，单线形 solo，DEC-018；**委外执行**：单代理持本任务包与提示词连续执行全链）
> 承接：用户 2026-09-07 令「开工先出提示词和任务包，我候其他任务结束委外运行」与同日两笔裁定即独立性来源（角度异质＋确定性透镜＋逃逸驱动增长）与温度退役；规格权威即 gateswitch-solo 段三呈裁件 sih-engine/sih/event/plan/gateswitch-solo-materials/pk054-ab-gate-design.md（A/B 门四判据三条件）；pk-044 出泊随本批
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 温度探针标定路径（模式一 agent 侧 temp_probe export-pack、模式二 facet 后台席走 MINIMAX 键）仍以新鲜 LLM 采样维持席位体温，方向上违反工程基线第五条治理延伸是减少 LLM 参与
- A/B 门设计已呈裁在档，其硬前置即 pk-044 独立性来源裁定已由用户 2026-09-07 落定，本批即该设计的 T6 实装承接批
- tally assemble 的基线输入现取 probes/calibration/ledger.jsonl 尾行（openhyg 勘误在案），切换后须由注入路径同形承接，断粮即停批
- pk-044 在泊待出泊，出泊条件两前置即数学管线收尾闸门在役（docmath 收尾合并闸门在位且必跑已核）与独立性来源经裁定（用户 2026-09-07）均已满足

## 二、关键设计 {#design}

### 2.1 B 路注入面（承设计档 §一）

正身报告（identity_hash 与 core_hash 与 summary）＋标定账本在档读数（ledger.jsonl 只读消费，16 行历史零改写）＋pack_version 以注入形进入测量材料上下文；V3-T 体温数值来源改为账本在档最近可用读数即零新鲜采样；漂移判读改为账本内历史配对即同席位同 pack_version 逐行对表；注入构建产 seat-baseline 同形件供 tally assemble --baseline 消费。

### 2.2 A/B 同场对跑（承设计档 §二）

冻结在档语料（现行可复算 904 件）双路对跑：A 路以现行冻结体温表驱动 assess_maturation_v3，B 路以账本在档读数注入（禁新鲜采样）驱动同函数；逐件比对 verdict_v3 与 near_flags 与 basis 指标；对跑读数自身同参双跑逐字节一致。

### 2.3 拔闸动作（承设计档 §二切换条件三）

四判据全过且对跑双跑一致即切换：temp_probe 两模式退役归档；facet/.env 的 MINIMAX 键从 facet 面移除（键不随件走，不迁移到新路径）；标定账本转只读。模式一 agent 侧标定在切换前仍为 R5 执契核对在役依据，退役后由账本在档读数承接。

### 2.4 注入物边界（承设计档 §四）

注入物限于正身件与账本读数与 pack 版本三样，禁把散文塞进采样上下文即违反唯一桥梁裁定（工程生成层合法语义消费面是数学仓映射表与条目）。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] B 路注入实装：注入构建器（读正身件＋账本＋pack 版本产 seat-baseline 同形件）、V3-T 数值来源改道、漂移历史配对
- [ ] A/B 对跑 904 件与四判据读数落批材料
- [ ] 拔闸：temp_probe 两模式退役归档、facet/.env MINIMAX 键移除、账本转只读
- [ ] pk-044 出泊材料与 scribe park exit（唯在 F-1 至 F-5 全过后，裁决文照录两笔裁定）

### Cluster 2：主线串行验证

- [ ] TDD 先红后绿（注入路径新用例先证红再证绿，留痕结果档）
- [ ] 既有测试零回归；金向量按族重放全绿
- [ ] 管线三步、双仓 settle、放锁收约、书单对表、对账对表

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 零判变 | 跨族治理 | 双路 verdict 一致率 100% 即 904/904 逐件三态全同；低于 100% 即不切换，差异件逐件列差异表停批呈裁 |
| **F-2** 数值逐位一致 | 数据治理 | 注入体温读数与基线 flip 率对冻结表数值逐席位逐位相等 |
| **F-3** 漂移告警零 | 数据治理 | drift_alarm 空两路皆然 |
| **F-4** 契约纪律不变 | 治理 | 注入路径测量材料仍带 identity_hash 与合同哈希绑定，pk-035 语义不松动 |
| **F-5** 拔闸后链路自证 | 工程完备 | 切换后跑一次全链活体（出题半＋计分半＋assemble），基线来自注入路径无断粮 |
| **F-6** 写入仅 allow 且出泊不越权 | 治理 | 写入仅请求写入节所列；pk-044 exit 唯在 F-1 至 F-5 全过后执行，只载事实与裁决指向 |

## 五、必读文件 {#read}

- 设计权威：`sih-engine/sih/event/plan/gateswitch-solo-materials/pk054-ab-gate-design.md`
- 纪要与证据：`sih-engine/sih/event/plan/gateswitch-solo-results.md`（段三 A/B 呈裁纪要与三泊件证据包）
- 基线提取调用形：`sih-tools/BATCH-FACE.md` 坑位勘误 2026-09-06（openhyg-solo 批增）facet 合同模式基线提取节
- 在泊件与程序档：`sih-engine/sih/state/parking/materials/pk-044.json` 与 `sih-engine/sih/event/plan/measure-poly-rev1-progdoc.md` §7
- 命令面：`sih-tools/BATCH-FACE.md` 全序与勘误

## 六、约束 {#constraints}

1. 零新鲜采样：B 路全程禁 LLM 调用（注入构建与对跑皆确定性）；账本历史零改写只读消费
2. 命令面冻结：子命令、旗标、退出码语义零变化；温度数值管道保留（数值来源改道非删除）；singleseat 单席位路径与采样合同 temperature 0.0 指令行零触碰（后者是零随机性纪律载体，去留另裁）
3. .env 仅动 MINIMAX 一键，其余零触碰；键移除即从 facet 面删除，不迁移不备份入库
4. 主树零直写即待提交件经工地 settle 通道；守卫在位禁 plain git commit；在盘遗留无主件不豁免不代清
5. 撞锁即排队候叫不绕行（起草时点多会话在飞占共享面，用户令即候其他任务结束后运行）
6. 任务包与提示词两起草件随本批 allow 收编入 settle

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 零新增
- [ ] pk-044 出泊事件在链（parking_exited，disposition promoted，裁决文两笔照录）
- [ ] 结果档 baseinject-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 账本读数落后冻结表版本即 B 路翻转 verdict——F-1 即此风险的拦截闸，停批呈差异表不硬凑（设计档 §四既定）
- 拔闸后 tally assemble 基线断粮——F-5 活体自证拦截
- MINIMAX 键移除遇 .env 格式或权限异常——如实呈报不硬闯
- 904 件语料若因后续批演进增减——以对跑当日可复算全集为准，件数如实记档，判据仍为 100% 一致

## 九、请求写入 {#requested-writes}

- `sih-engine/sih/state/plan/baseinject-solo.md`（本件）与 `sih-engine/sih/state/plan/baseinject-solo-prompt.md`
- `sih-tools/facet/`（注入实装、探针退役、.env 键移除）
- `sih-tools/facet/probes/calibration/`（转只读，零内容写入）
- `sih-engine/sih/state/parking/materials/pk-044-exit.json`
- 当日链 trail（经引擎 scribe，文件名按实际运行日期）
- `sih-engine/sih/event/plan/baseinject-solo-results.md` 与 `-materials/`
- `sih-tools/scribe/reports/`、`sih-tools/identity/reports/`、`sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 baseinject-solo 工地
