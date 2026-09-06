# redkeep-solo 结果档

> 批机械链全序实录，单线形 solo 委外执行（DEC-018 选形制，与 toolincub-solo 同一委外令串行，本批在后），2026-09-06。任务包 sih-engine/sih/state/plan/redkeep-solo.md 唯一规格源，机械链 sih-tools/BATCH-FACE.md 含勘误节。会话 20de9a003a963cd1（lease 1.30.0）。

## 一、批机械链各步退出码

| 步 | 命令要点 | 退出码 | 读数 |
|---|---|---|---|
| 三问双门 | scrutinator ask3 包＋ask3repeater | 0/0 | findings 0；status ok 三锚（道四间隙 L17、道三意图必复 L15、鉴客观 L53 程序切片） |
| 叩问 | elicit check＋digest | 1/0 | 7 轻信号全 unregistered，digest passed 全覆盖 7 |
| 正身 | identity verify | 0 | anomalies 空 |
| watch 对表 | watchcheck check | 1 | 无主清单 1 件呈报（billwire 实测残件，与批一同件，零代行零触碰） |
| 泊界心跳 | selector route 两线 | 0/0 | 零告警 |
| 租约开工 | lease open | 0 | 双仓工地；八真锁全 exit 0（另一路径形误敲被拒 exit 1 零效果，如实记档） |
| 书简意图 | scribe intent | 0 | 事件 85e8dc11 |
| 基线复现 | 工地 cargo test 全量 | 101 | lib 168 passed 加 2 failed 加 6 ignored；余红 2 件全在任务包 §2.1 清单内，零清单外新红（fail-fast 截停后续套件，见 §三） |
| 逐红复现留痕 | --lib 逐红 --nocapture | 101/101 | 两红断言全文入 materials |
| 最小修复 | 金向量重生成 | 0 | 唯一改动件 src/scrutinator/fixtures/golden/des-001-gov002.json，零 Rust 代码改动 |
| 修后读数 | cargo test --lib | 0 | **170 passed 加 0 failed 加 6 ignored**（165 原绿加 2 余红转绿加 3 自愈，6 ignored 不变） |
| 管线 | 化格→核阅→检词 | 0/2/0 | 金向量已合规范形（与在役绿金向量同形 would-change 0）；核阅域外 exit-2 如实记档；检词 json 域外零检 |
| 温故 recall | retriever recall | 0 | 80 命中入 materials |

## 二、逐红根因账（清单五红）

开工复现红名单对表任务包 §2.1：余红 2 件在清单内，3 件已自愈，零清单外新红（mem 套件事见 §三，属批前既有潜伏红非本批引入）。

| 红名 | 开工态 | 根因归类 | 处置 |
|---|---|---|---|
| cli_positional_and_flag_forms_byte_identical | 已自愈（绿） | 环境依赖类：kernelmerge 基线时点二进制或文档态与今异，两形比对不依赖金向量字节，主树重编后即绿 | 如实记档零改动 |
| exit_compliant_zero | 已自愈（绿） | 同上，仅断退出码零，当前实跑 exit 0 | 如实记档零改动 |
| multipack_attribution_pack_names | 已自愈（绿） | 同上，断 packs 结构不逐字节比对 | 如实记档零改动 |
| golden_des001_gov002 | 红 | 测试过时类：金向量冻结于 GOV-002 v2.4 修订之前，报告内嵌目标内容哈希过期（冻结件 27131b1a 对现行 567afa99），其余字节全同、findings 零、包版本同 0.3.0 | 按任务包 §八预设通道重生成金向量，新旧件逐字节 diff 入 materials，断言零改 |
| cli_positional_form_matches_golden | 红 | 同根因：同一金向量件逐字节比对 | 同上，随件转绿 |

修复实质：GOV-002 经 leaseupclose-solo 批合法修订（v2.4 四处补落），des-001 判定其零违规 exit 0 判定不变，唯内容哈希随文更新。测试断言、判定语义、行为面零触碰；实现零缺陷；修复面仅金向量数据件一行哈希。

## 三、mem_recall_f_suite 潜伏红披露（清单外、批前既有、不在本批处置面）

修后 lib 全绿使 cargo test 越过 fail-fast 首次跑入 tests/mem_recall_f_suite.rs 集成套件，暴露 f2_refs_machine_verifiable 红（主树同跑另见 f10_pack_archive_consistency 红）。判定为批前既有潜伏红三证：其一，主树（零本批改动态）同跑复现 f2 红加 f10 红，与本批唯一改动件金向量 json 逻辑无关；其二，kernelmerge 基线证据 cargo-baseline.txt 只含 lib 摘要行，fail-fast 在 lib 五红处截停，该套件从未入基线读数；其三，f2 红因是 pk050sw-solo-results.md 于 2026-09-04 收口回填两笔（c6bf228、08ca83b）编辑在前、召回结论档索引摘录未随重建，属检索档案维护漂移。处置：不越权扩张修复面，如实披露候另批（建议批名 memrecallkeep 类），两套件复现输出已入批材料。修后全量读数因此呈 lib 域 170 加 0 加 6 与全仓 suite 域 1 既有红两层，如实分列。

## 四、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| Cluster 1 前置读数 | 完成 | §一前五行 |
| Cluster 2 施工 | 完成 | §二逐红账＋§一修后读数 |
| Cluster 3 管线与结算 | 完成 | 本档＋双仓 settle＋收约对账 |

## 五、F 验证表

| F 锚定 | 判据 | 结果 |
|---|---|---|
| F-1 五红清零 | cargo test 0 failed，五红名逐一转绿在档 | 过（基线域）：lib 170 加 0 加 6，五红名全绿；全仓 suite 域另有批前既有潜伏红一件如实分列于 §三 |
| F-2 零回归 | 原 165 passed 全仍绿，6 ignored 不变，零清单外新红 | 过：165 全绿，6 ignored 不变，本批零引入新红（§三红为批前既有，主树复现在案） |
| F-3 先红留痕 | 修复前五红复现输出入批材料，红名单对表一致 | 过：两余红 repro log 加三自愈读数加 kernelmerge 清单对表在档 |
| F-4 判据零私改 | 测试断言零删除零弱化 | 过：零测试代码改动，断言原样，金向量数据件按预设通道重生成并逐字节对表 |
| F-5 写入仅 allow 且不越权 | 写入仅请求写入节所列，串行序在先 | 过：唯一内容改动件 src/scrutinator/fixtures/golden/des-001-gov002.json 在 §11 allow 面；toolincub-solo 收约后本批才开工 |

## 六、批材料清单（落 sih/event/plan/redkeep-solo-materials/）

red-repro-golden_des001_gov002.log、red-repro-cli_positional_form_matches_golden.log（先红留痕）、golden-des001-gov002.old.json 与 golden-des001-gov002.diff.txt（重生成逐字节对表）、post-fix-lib-run.log 与 post-fix-full-run.log（修后读数含 mem 套件实录）、memrecall-main-tree-pre-existing.log（主树既有性复现）、recall-topic-2026-09-06.json（温故底稿）。

## 七、误差与越线申报

- 任务包 §2.2 读数目标「0 failed、passed 不小于 170」按基线域（lib）达成 170 加 0 加 6；全仓 cargo test 因 §三批前既有潜伏红（f2）不能全 suite 零 failed，不属本批引入，不硬闯不越权修，候另批处置。此为本批最大误差申报项。
- cargo test 在批一收约前的主树侦察跑（三次）写主树 target 构建缓存：与 kernelmerge 基线采集同形，target 为未跟踪构建缓存非版控内容，如实申报。
- lease lock 一路径形误敲（sih/engine）被拒 exit 1，零效果，如实记档。
- watch 无主清单与批一同件（billwire 实测残件），呈人节点二值裁决，本批零代行。

## 八、收口读数

链 verify 与 reconcile 双仓读数见完工报告（收约后实跑）；本档 settle 于认证之后提交。
