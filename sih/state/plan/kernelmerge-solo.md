# kernelmerge-solo：谓词求值内核统一批（引擎侧）

> 治理任务包（实装类，单线形 solo，DEC-018）
> 承接：用户 2026-09-06 委外令；机器裁决 m-halfmerge-1 stable_clear 终签 2a7d0e0e（crosscheck_completed 在链）；pk-060 出泊裁决指向即本开工令加该终签
> 日期：2026-09-06

## 一、问题陈述 {#problem}

- 核阅谓词引擎（src/scrutinator/rule.rs）与得一路由谓词机（src/attractor/route.rs）两套空腹谓词机求值内核并存，属登记在案的无主重复（全治理态展开审计）
- pk-060（谓词求值内核统一与否）在泊，裁决已齐（用户开工令＋m-halfmerge-1 终签），待出泊与实施
- 文规格式检查器（后续外切批）需要唯一求值内核可骑，不生第四套

## 二、关键设计 {#design}

### 2.1 半归并实施形

引擎 lib 内立共享求值模块（如 src/predkernel/），核阅与得一路由两消费方改调该模块；命令面、子命令、旗标、退出码语义零变化；谓词方言各自保留（文档谓词与路由谓词域不混）。

### 2.2 零漂移验收命门

统一前后行为逐字节一致是本批成立的唯一硬判据：两机既有金向量全量重放（核阅 golden 12 件、route golden 8 场景）＋同参双跑 cmp 逐字节 IDENTICAL＋退出码对齐。任何漂移即本批失败，不迁就。

### 2.3 先红后绿

新增共享模块的单元测试先红后绿留痕于结果档；既有测试零回归。

### 2.4 pk-060 出泊

pk-060-exit.json 落工地（正形参照在册 exit 件），scribe park exit 入链，exit 载裁决指向即用户 2026-09-06 委外令＋m-halfmerge-1 终签 2a7d0e0e。出泊写入仅在验收全过后执行。

## 三、工作清单 {#work}

### Cluster 1：主线写

- [ ] 共享求值模块实装与两消费方切换（工地内）
- [ ] 新模块单元测试先红后绿
- [ ] 金向量全量重放与双跑对表实录落批材料

### Cluster 2：主线串行验证

- [ ] cargo test 全绿零回归；cargo build 零新增警告红线守住
- [ ] 同参双跑：统一前后两机输出 cmp 逐字节 IDENTICAL 与退出码一致
- [ ] pk-060 出泊写入（仅验收全过后）
- [ ] 管线、认证、双仓 settle、放锁收约、reconcile 与链 verify（退出码直读）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 金向量零漂移 | 工程 | 核阅 golden 12 件与 route golden 8 场景全量重放逐字节一致 |
| **F-2** 契约零漂移 | 工程 | 同参双跑即统一前后两机 CLI 输出 cmp IDENTICAL 且退出码一致，命令面零变化 |
| **F-3** pk-060 出泊 | 数据治理 | 出泊事件在链且 exit 件载裁决指向；验收不过即不出泊 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |

## 五、必读文件 {#read}

- 裁决源：`sih-tools/proposition/DES/m-halfmerge-1/`（命题、计分材料、终签报告）
- 命令面：`sih-tools/BATCH-FACE.md`（全链序与坑位勘误）
- 先例：`sih-engine/sih/event/plan/leaseup-solo-results.md`（引擎行为批先红后绿与双跑实录形）

## 六、约束 {#constraints}

1. 接口契约零变化：两机命令面与退出码语义冻结，任何变化即本批失败
2. 谓词域不混：统一的是求值内核不是谓词域，文档谓词与路由谓词各自保留
3. 零 LLM 参与新增、零网络、零第三方依赖新增
4. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
5. 守卫在位禁 plain git commit，close 通道提交
6. 在盘遗留无主件不豁免不代清，本批不新增无主写

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] 双仓 settle 提交号在档，收约后零活跃锁零活跃会话
- [ ] 链 verify valid，reconcile 对照批前零新增（退出码直读）
- [ ] 结果档 kernelmerge-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 两机谓词语义存在真实差异即无法零漂移统一——如实呈报停批，不硬凑（半归并判据即不并谓词域，若发现语义重叠处须以包装适配不以内核吞并）
- 引擎件合并后主树重编再作任何调用（BATCH-FACE 勘误：旧二进制按旧包作答）
- close 归并遇主树未跟踪任务包按备份让位归并对表法

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`（park 与 append 与 intent 写位）、cargo test 即测试位
- 跨仓引用：`sih-engine/sih/state/parking/materials/pk-060.json`

## 十一、请求写入 {#requested-writes}

- `sih-engine/src/`（共享模块与两消费方）
- `sih-engine/sih/state/plan/kernelmerge-solo.md`
- `sih-engine/sih/state/parking/materials/`
- `sih-engine/sih/event/trail/<实日>.ndjson`
- `sih-engine/sih/event/plan/kernelmerge-solo-results.md`
- `sih-engine/sih/event/plan/kernelmerge-solo-materials/`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- `sih-tools/scribe/CALL-LOG.md`
- worktrees 双仓 kernelmerge-solo 工地
