# 泊界首建与路择时间维度谓词扩展：结果文件

> 承任务包 parking-selector-time-t6d.md，2026-08-25 收口

## F 锚定对表 {#f-table}

| F 锚定 | 结果 | 证据 |
|---|---|---|
| F-1 停泊写入位 | 过 | scribe 三十四测全绿含 park 八测，重入拒与无主出拒各拒且零留痕，写入前校验四项不松承 trail 模块既有测试，旧测零改 |
| F-2 时间谓词族 | 过 | selector 一百三十七测全绿含 parking 十三测，到期日判败、缺键缺参照时间判败、超期逐件告警、报告头条件键、旧包旧输出逐字节不变全部有测 |
| F-3 双跑一致 | 过 | 进程内与子进程双跑 selector route 输出逐字节一致两测，scribe 同记录同 --at 同 --event-id 双跑事件哈希一致一测 |
| F-4 首批住户入泊 | 过 | 五件 parking_entered 在链即 41ec4a91 加 537f06da 加 788917b4 加 c15f4c7b 加 56b014d9，verify 报 valid，details 逐件含 exit_condition 与 ttl_days |
| F-5 基线实跑 | 过 | selector/baseline/2026-08-25-parking-baseline-report.json 即五件全主线零告警退出码零，两 CALL-LOG 行入档 |
| F-6 文档链 | 过 | 任务包与结果文件经化格与检词零违例，核阅 des-001 域外退出码二如实记非违规即与数学审计批同形态，书简认证在链，round 7 事件在链即 fbd25519 |

## 交付清单 {#deliverables}

- PARKING-v1.md 首建即四字组落位与在泊名禄五项
- scribe 第五子命令 park 即契约修订四，新增八测
- selector 时间维度谓词两件与 parking 谓词包即契约修订三升 0.3.0，新增十三测
- COURSE-v1 v1.3 即泊界节改写，AGENTS 三处接线
- parking/records 五件入泊记录与 parking/materials 五件材料
- 六工具回归全绿合计二百四十二测即 formatter 十加 scrutinator 二十九加 nomenclator 二十三加 scribe 三十四加 selector 一百三十七加 meter 九

## 偏离与如实登记 {#deviations}

1. selector 旧测一处扩充即 test_readonly.py 只读白名单登记 datetime 标准库条目，时间谓词需日期运算，datetime 属标准库不违反只读与空腹纪律，其余旧测零改。
2. trail 内时间戳为 UTC，治理实日为本地日，材料 entered_at 取治理实日 2026-08-25，与链事件经本地时区换算核对一致，此换算约定登记在案供视图族远端实装时机械化。
3. 全仓根目录起跑 pytest 会收集到并行 agent 在建的 latex-helper 测试并报错，属收域污染非本批缺陷，逐仓目录内起跑六工具全绿如实登记。
4. 任务包 F-6 判据随批修订一处即 doclint 退出码零改述为化格检词零违例加核阅域外如实记，修订事由与形态登记于任务包范式偏离声明节。

## 结论 {#conclusion}

F-1 至 F-6 全过，验收四项达成。泊界从立法毕载体零转为在役即账本在链、门在检索、警在谓词、出在唯人节点。
