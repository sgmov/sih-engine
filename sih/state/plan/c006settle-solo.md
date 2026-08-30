# c006settle-solo：C006 四批结算补办与勘误上链批

> task-packages 治理任务
> 承接：用户 2026-08-30「同意批准修复清单」令 + 本会话盘面核验结论
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

c006-jc 与 c006-sb 与 c006-sb2 与 c006-sb3 四批的内容改动真实落在 sih-math 主检出工作树即 113 文件 1236 行，但结算从未发生：两仓零提交、四批内容认证事件零笔在链、jc 批自记机械撤回、sb 批 F-6 自记链未跑、sb3 结果档「1344 改 8 即 99.4% 归零」被机械核阅读数 154 证伪。本批补办结算与勘误，不动内容、不重写历史。

## 二、关键设计 {#design}

段一范围冻结：sih-math 取 `git diff --name-only` 的 113 件；sih-engine 取四条 trail 日链文件、c006 四任务包、jc 与 sb 与 sb2 三结果档加两材料目录、本批新件。主检出他人线改动即 proposition-defense SKILL、inputlog、task-packages/f-anchors-x11-t6d.md 一律零扫入。

段二回围堰移植：sih-math 以 diff 应用入工作树副本；sih-engine 逐件复制入副本；移植后逐件 SHA-256 与主检出比对一致方动主检出副本。

段三三闸：化格 check 模式、核阅 des-001-mathe、检词 core 各跑一轮，读数如实记录即 C006 154 加 S005 113 加 M008 51 加 C002 2 为已提交规则下的结算态读数，报告 JSON 随材料入档含内容哈希。

段四链补真：本批结果档承载四批实况勘误与归零声明修正，scribe append 认证事件附结果档内容哈希；四批历史结果档原文零改动。

段五链文件最后收口：trail 四件最后复制入副本并复验，主检出副本即 checkout 收回，归并回补；复制后若他线再追加即重复制重比对，循环抓静默窗口，抓不到则如实申报缓收。

段六收口：lease commit --stage settle --cert 指副本，lease close 归并拆本，双仓 reconcile。

## 三、工作清单 {#work}

- [ ] 范围清单落 materials/c006settle-scope.tsv
- [ ] sih-math 113 件移植入副本 + 哈希比对
- [ ] sih-engine 材料移植入副本 + 哈希比对
- [ ] 化格 check + 核阅 + 检词 三闸报告
- [ ] c006settle-solo-results.md 勘误档
- [ ] scribe append 认证事件
- [ ] trail 四件末位复制 + 比对 + 主检出收回
- [ ] lease commit + close
- [ ] 双仓 reconcile
- [ ] scribe verify 链 valid

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** sih-math 结算 | 工程 | 113 件经副本提交归并，主检出对应文件洁净，移植前后逐件哈希一致 |
| **F-2** 材料结算 | 工程 | 四任务包与三结果档与两材料目录入库，他人线改动零扫入 |
| **F-3** 链补真 | 治理 | 勘误事件在链即四批实况加 154 修正，历史结果档原文未改 |
| **F-4** 三闸如实 | 治理 | 化格 check 过、核阅与检词报告随材料含引擎版本与包版本与内容哈希，读数不粉饰 |
| **F-5** 链 valid | 治理 | 末笔追加后快照，verify 退出码零 |
| **F-6** lease 通 | 治理 | open 加 lock 加 commit 指副本加 close 全过 |
| **F-7** reconcile | 治理 | 双仓对表无新疤，既有疤如实申报 |

## 五、必读文件 {#read}

- 必读 1：c006-sb3-solo-results.md 即数字校准段自认 17 残与漏算教训
- 必读 2：c006-jc-solo-results.md 即真即称类二选一撤回口径
- 必读 3：sih-tools/lease/CONTRACT.md 即 1.8.2 提交四验

## 六、约束 {#constraints}

1. 零 LLM 调用于工具执行层
2. 词债不过夜
3. 历史不可改：勘误只追加不重写
4. 他人线改动零扫入
5. 链快照复制必须发生在末笔追加之后
6. 主检出直提拒止：commit 只指副本

## 七、请求写入 {#requested-writes}

- sih-math/calculus/llm-friendly-build/entries/
- sih-engine/sih/state/plan/c006-jc-solo.md
- sih-engine/sih/state/plan/c006-sb-solo.md
- sih-engine/sih/state/plan/c006-sb2-solo.md
- sih-engine/sih/state/plan/c006-sb3-solo.md
- sih-engine/sih/state/plan/c006settle-solo.md
- sih-engine/sih/event/plan/c006-jc-solo-results.md
- sih-engine/sih/event/plan/c006-sb-solo-results.md
- sih-engine/sih/event/plan/c006-sb-solo-materials/
- sih-engine/sih/event/plan/c006-sb2-solo-results.md
- sih-engine/sih/event/plan/c006-sb2-solo-materials/
- sih-engine/sih/event/plan/c006settle-solo-results.md
- sih-engine/sih/event/plan/c006settle-solo-materials/
- sih-engine/sih/event/trail/2026-08-25.ndjson
- sih-engine/sih/event/trail/2026-08-26.ndjson
- sih-engine/sih/event/trail/2026-08-28.ndjson
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

- 他线并发追加 trail 致移植窗口抓不静默，预案即如实申报缓收链文件
- 主检出收回副本误伤未提交他人改动，预案即逐件清单操作零通配
- 核阅读数 154 含 8 真 SKIP 结构性残余，本批只记录不追改

## 十、关联文件 {#related}

- 任务包源：用户 2026-08-30「同意批准修复清单」令
- 链件：sih-engine/sih/event/trail/2026-08-30.ndjson
- 关联：c006-jc、c006-sb、c006-sb2、c006-sb3 四批与 pk031gap 提交拒直提新规

## 十一、队形声明 {#formation}

单线形即主线亲写零子代理。

## 十二、后续 {#next}

- 余量 154 即 8 真 SKIP 加 146 待改手改分卷（待令）
- sih-tools 144 件未提交面属他线，不入本批
