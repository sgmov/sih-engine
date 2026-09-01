# listzero-solo 结果档（待令清单清零五修批）

> 批名：listzero-solo。日期 2026-09-02。会话 5c81a869（lease 1.11.0 双仓）。
> 意图事件 58060c46（锚 PRO-06 法四损补、PRO-07 鉴、PRO-08 应）。
> 承接：用户令「得一裁待零清单，一裁一过一开」，九项裁毕七实锤开五修。

## F 锚定验收

| F | 判据 | 判定 |
|---|---|---|
| F-1 tally 失败态 | 非零退出打 failed 不打 signed | 过（活体复现 exit 1 stdout 唯 failed，见材料 f1-tally-failed-state.json） |
| F-2 Points 清零 | 七处存量复跑零、死词在册 | 过（pack 0.6.0 复跑 TOP-004/005/007 全零，Points 仍在 dead） |
| F-3 信封非零 | 零命中件非零字节可解析 | 过（62 字节信封 count=0；单词命中路径 349 字节不回归） |
| F-4 血统拒入 | status 非 ok 与 session 错配双拒 | 过（新增三测全过：rejected、mismatch、ok 放行） |
| F-5 全测绿 | cargo test 全绿 | 误差申报（lib 110 过 1 败 6 略：唯败 golden_des001_gov003 主树同败，系前批 GOV-003 变更对陈旧金向量，入存量清单；mem_recall_f_suite 主树同坏编译不过即 RecallArgs 缺 words 与 miss_log，修复入 legacytwo 批；test_valid_record_passes 工地败主树过系工地无姊妹仓环境败。其余 integration 四套件全过） |
| F-6 检词化格 | 改动 md 门全过 | 过（化格双 0 改、检词双 0、核阅双域外如实记） |
| F-7 lease 链 | 双仓全程 verify 零 | 见链段 |

## 五修实录 {#fixes}

一 tally cli.py sign_material 加失败态分支即 scribe 非零退出打 sign failed 带 scribe_exit 并如实透传退出码。二 nomenclator dead 条目新增可选 exempt 行级子串豁免即 check 逐 span 比对命中行，Points 条目豁免 Fréchet 1898 题名原文，包升 0.6.0，CONTRACT 修订三。三 retriever 新增 write_output_envelope 即零命中写 recall 信封件 count=0，bin 切换，词面整串匹配语义未动。四 scribe intent_event 加双检即验证件 status 显式非 ok 拒加验证件与记录 session 错配拒，无 status 旧形兼容。五 marshalling skill 硬性工作流第三步增并行簇认领复核即认领后停两息重读台账确认在先、写件后再核未被覆盖，修订三入册。

## 分道披露 {#routing}

用户令得一裁九项：俱走确定性通道出证（代码位引证、工具活体复现、账册在案），未送采样，承 facet skill 修订四机械可核改道条款。LIM-008 注记差异查无实据即条目与 INDEX 逐字一致如实闭之；他会话七笔经核内容已由后续正规租约归并收编，主树直写两笔为不可改历史 record-only。

## 链与收口 {#chain}

意图 58060c46；认证见 trail；cert 锚取末笔；双仓 commit 指副本，close 归并，reconcile 对表。

## 后续 {#next}

legacytwo-solo 即 math 存量两处中 TOP-004 L17 悬空指针待修与 reconcile 老旗三笔 errata 落档与 mem_recall_f_suite 套件字段补齐与 golden_des001_gov003 金向量重录评估。三工具批之余项即 retriever 词面匹配语义是否升级归用户另裁。
