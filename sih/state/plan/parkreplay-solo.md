# parkreplay-solo：书简泊跨链重放修复批（mathpipe-a1 前置小批）

> task-packages 治理任务
> 承接：用户 2026-09-03 批准即 pk-041 记账位方案 b 的前置件，mathpipe-a1-solo 件 0 依赖声明
> 队形：单线形 solo——确定性代码与主线亲写零子代理
> 日期：2026-09-03

## 一、问题陈述 {#problem}

scribe park 配对门即重入拒与无主出拒的重放面是 --trail 单链文件。泊件生命周期普遍跨天，全史四十余笔出泊均落记录当日链文件而泊入在先日链，单链重放使跨天出泊机械不可达即无主出拒误拒合法出泊。pk-041 出泊记账即用户 2026-09-03 裁定由 mathpipe-a1 件 0 承接，被此缺口直接阻塞。同日链 pk-043 撞号另暴露号源唯一性无门，属另一缺口不在本批。

## 二、关键设计 {#design}

配对门重放面扩为链目录全量：park 子命令以 --trail 文件的同目录 ndjson 按名序全量重放判定在泊态。追加面仍是 --trail 单文件即当日链自身链序，verify 单链逐文件语义不变。其余子命令调用面与退出码三值逐字节不动。不新增停泊拒类。

## 三、工作清单 {#work}

- [ ] 实装链目录全量重放面于 event_stream 即新函数 load_parking_scope，park 子命令接线
- [ ] 单测三件即跨文件 enter 至 exit 达通、跨文件重入拒、追加面单链不串链
- [ ] SPEC-006 park 入口节修订一即重放面语义
- [ ] SPEC-006 修订一走化格核阅检词三步
- [ ] 认证上链双仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1 跨天路径红绿** | 工程 | 测试链目录即先日链 enter 加当日链 exit，修复前同参形停泊拒 OrphanExit 退出码一，修复后 appended 退出码零，两态读数入档 |
| **F-2 同日行为不回退** | 工程 | 既有同日 enter 至 exit 与重入拒行为单测全绿，cargo test 全绿 |
| **F-3 语义窄改** | 工程 | verify 与 query 与 append 与 intent 与 record 五子命令输出与退出码对表零变化，追加面仍单链 |
| **F-4 写入仅限 allow** | 治理 | 本批写入仅请求写入节所列 |

## 五、必读文件 {#read}

- 必读 1：sih-engine/doc/spec/SPEC-006-scribe-mergeback-gap.md 即 park 入口现行文
- 必读 2：sih-tools/BATCH-FACE.md 即调用面与坑位
- 必读 3：sih-philosophy/emanation/proodos/07-on-assay.md 与 06-on-canon.md 与 08-on-settle.md 引文原文只读切片

## 六、约束 {#constraints}

1. 上链遇锁即等待不绕行
2. 词债不过夜
3. 号源唯一门不入本批即结果档候选申报
4. 泊件名册与 09-02 链未提交行零触碰，pk-041 与 pk-043 记账归 mathpipe-a1 批不属本批

## 七、请求写入 {#requested-writes}

- sih-engine/src/event_stream/park.rs
- sih-engine/src/bin/scribe.rs
- sih-engine/doc/spec/SPEC-006-scribe-mergeback-gap.md
- sih-engine/sih/state/plan/parkreplay-solo.md
- sih-engine/sih/event/plan/parkreplay-solo-results.md
- sih-engine/sih/event/plan/parkreplay-solo-materials/
- sih-engine/sih/event/trail/2026-09-03.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/
- sih-tools/nomenclator/packs/core/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过
- [ ] cargo test 全绿
- [ ] SPEC-006 修订一三步过
- [ ] 认证入链，双仓段结算收约，对表读数在档
