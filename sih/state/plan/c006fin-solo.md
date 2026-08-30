# c006fin-solo：C006 终卷手改批

> task-packages 治理任务
> 承接：用户 2026-08-30「开」令 + c006settle-solo 结算态读数 154 残
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

C006 结算态残量 154 即 8 真 SKIP 加 146 待改。146 处分布于 13 文件 126 行，全部为中文内容描述性括号，无可半角化逃逸。本批按逐行人工定夺的改写稿清偿 146 处，8 真 SKIP 留置待规则语义另裁。

## 二、关键设计 {#design}

段一改写稿冻结：126 行逐行通读，每行按语境定改法，映射冻结于材料 c006fin-fixmap.tsv，改法三类即逗号整合、即称改写、语序重排，零语言转换零半角逃逸。

段二副本落笔：编辑位在租约副本，确定性脚本按映射逐行替换，任一 (file,line,old) 失配即整体中止。

段三机械验收：核阅 C006 由 154 降至恰 8，降数恰等于替换数 146；S005 与 M008 与 C002 计数逐一不变即 113 与 51 与 2。

段四三闸与上链：化格 check、核阅、检词报告随材料，认证经 meter 包裹上链。

段五结算收口：链尾复制在末笔追加后，双仓 commit 指副本，close 归并拆本，双仓 reconcile。

## 三、工作清单 {#work}

- [ ] 改写映射 146 对冻结入材料
- [ ] lease open 双仓 + 锁
- [ ] 副本内落笔 13 文件 126 行
- [ ] 核阅 C006 恰降 146 至 8
- [ ] 其余三类计数不变
- [ ] 化格 check + 检词
- [ ] scribe append 认证
- [ ] 结果档
- [ ] lease commit + close
- [ ] 双仓 reconcile + 验链

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** C006 恰降 | 工程 | 核阅 C006 154 → 8，降数 = 替换数 = 146 |
| **F-2** 真零语义漂移 | 治理 | diff 逐行与冻结映射一致，无映射外改动，无语言转换无半角逃逸 |
| **F-3** 其余类不变 | 治理 | S005=113、M008=51、C002=2 逐一不变 |
| **F-4** 8 真 SKIP 不动 | 治理 | DIFF-030 与 INT-016 与 INT-021 与 LIM-003 两处与 MUL-001 与 MUL-003 与 MUL-008 零改动 |
| **F-5** 化格检词 | 治理 | 化格 0 改，检词域外如实记 |
| **F-6** lease 通 | 治理 | open 加 lock 加 commit 指副本加 close 全过 |
| **F-7** 链 valid | 治理 | verify 退出码零，链尾快照在末笔追加后 |

## 五、必读文件 {#read}

- 必读 1：c006settle-solo-results.md 即结算态读数与教训
- 必读 2：c006-jc-solo-results.md 即真即称类二选一口径
- 必读 3：sih-tools/lease/CONTRACT.md 即提交四验

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 请求写入行必须净路径
4. 8 真 SKIP 零触碰
5. 上链遇他会话锁即等待不绕行
6. 链快照复制必须发生在末笔追加之后

## 七、请求写入 {#requested-writes}

- sih-math/algebra/entries/
- sih-math/order/entries/
- sih-math/probability/entries/
- sih-math/topology/entries/
- sih-engine/sih/state/plan/c006fin-solo.md
- sih-engine/sih/event/plan/c006fin-solo-results.md
- sih-engine/sih/event/plan/c006fin-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-7 全过
- [ ] 链 valid
- [ ] lease close
- [ ] 双仓 reconcile 无新疤

## 九、风险点 {#risks}

- 改写映射誊写失配，预案即脚本失配即中止零部分落笔
- 他会话 charterfix 批持链锁，预案即等待窗口备料不绕行
- 标题两处改动牵 M008 前缀判定，预案即前缀 ID 保留验收时复核

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30「开」令
- 链件：sih-engine/sih/event/trail/2026-08-30.ndjson
- 关联：c006-jc、c006-sb、c006-sb2、c006-sb3、c006settle 五批

## 十一、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十二、后续 {#next}

- 8 真 SKIP 处置待规则语义批另裁
- 温故 retriever 空转转工具侧待查
