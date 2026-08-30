# bridgeacc-solo：桥接入闸批

> task-packages 治理任务
> 承接：用户 2026-08-30 两波多代理桥接完成令 + verify.py 四项全绿 113 件
> 队形：单线形 solo——主线亲写零子代理（子代理产出已在收件台，本批为验收落笔）
> 日期：2026-08-30

## 一、问题陈述 {#problem}

收件台 113 件桥接草稿经固化验钞机四项全绿即锚点 225 条逐字验过、正文零全角括号、条目实存且无桥接节、节头格式正确。本批将草稿正文机械插入各条目「关系」节与「工程映射」节之间，经三闸与认证上链，结算归并正身。

## 二、关键设计 {#design}

段一前置已验：113 件条目全部带 `## (工程|应用)映射 {#engineering}` 插入锚，插入位统一在锚行前。

段二副本落笔：确定性脚本逐件插入，任一草稿 front matter 解析失败或锚行缺席即整体中止，插入后逐件计数 113 全变更。

段三读数不回归：核阅全量 128 件 C006 恰为 8、S005 113、M008 51、C002 2，与 c006fin 收口态逐一相同。

段四材料归档：收件台全量即 113 草稿与 CLAIMS 与 REPAIRS 与 WAVES 与 verify.py 复制入批材料，编排过程可复现。

段五收口：三认证经 meter 上链、链尾快照在末笔后、双仓 commit 指副本、close 归并、双仓对表。

## 三、工作清单 {#work}

- [ ] 插入脚本冻结与执行 113 件
- [ ] 核阅全量读数不回归
- [ ] 化格 check + 检词
- [ ] 材料归档
- [ ] 三认证上链
- [ ] 结果档
- [ ] lease commit + close
- [ ] 双仓 reconcile + 验链

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 恰 113 件变更 | 工程 | 插入后 git diff --name-only 恰 113 件 calculus 条目 |
| **F-2** 读数不回归 | 工程 | C006=8、S005=113、M008=51、C002=2 逐一相同 |
| **F-3** 插入位统一 | 治理 | 每件插入段紧邻 {#engineering} 锚行前，锚点在正文逐字在册 |
| **F-4** 化格检词 | 治理 | 化格 0 改，检词域外如实记 |
| **F-5** lease 通 | 治理 | open 加 lock 加 commit 指副本加 close 全过 |
| **F-6** 链 valid | 治理 | verify 退出码零，快照在末笔后 |

## 五、必读文件 {#read}

- 必读 1：agent-drafts/bridge/WAVES.md 即五波编排实录与竞态记录
- 必读 2：agent-drafts/bridge/verify.py 即验收判据本体
- 必读 3：c006fin-solo-results.md 即读数基线

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 上链遇他会话锁即等待
4. 链快照复制在末笔追加后
5. 认领账本与波次记录只归档不改写

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-engine/sih/state/plan/bridgeacc-solo.md
- sih-engine/sih/event/plan/bridgeacc-solo-results.md
- sih-engine/sih/event/plan/bridgeacc-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 链 valid
- [ ] lease close
- [ ] 双仓 reconcile 无新疤

## 九、风险点 {#risks}

- 草稿 YAML 解析个别失败即中止重验
- 他会话并发撞链窗口，预案即等待

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30 完成令
- 链件：sih-engine/sih/event/trail/2026-08-30.ndjson
- 关联：c006fin-solo 读数基线、marshalling 并联形首个租约下活体

## 十一、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十二、后续 {#next}

- 编组并联形首个租约下活体实证入 ai-ex 谱系（待令）
- 认领协议竞态覆盖缺陷候选转工具侧（待令）
