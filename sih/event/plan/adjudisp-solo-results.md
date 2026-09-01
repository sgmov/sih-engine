# adjudisp-solo 结果档：裁决分派令立名与得一狗粮回填批

> 队形：单线形 solo（委外代理亲写零子代理）
> 日期：2026-09-02。会话 f808eb692bb60134（lease 1.11.0 双仓 engine+tools）
> 意图事件哈希 32750df8（当日链第 19 笔），ask3repeater status ok 三锚，elicit digest passed covered=4，正身零异常 hash c80d562d

## 一、F 验证表

| F | 判据 | 判定 | 证据 |
|---|---|---|---|
| F-1 分派令 | DEC-021 落档、三态分派表全、管线 des-001 零违规、检词登记零拒、投影核验一致 | 过 | doc/decision/021-adjudication-dispatch.md 三态表过得一四目加人节点四目加代裁两目加灰区从严全载；化格 exit 0 零改；核阅工地路径 exit 2 域外如实记，同哈希内容 fixtures 域内重跑 exit 0 findings 0；检词 check exit 0；register 落册 terms 87 至 88 条零拒，manifest 0.4.0 升 0.5.0；naming 与 facet-measure 两 skill 零引用即一致且主树投影 diff 一致 |
| F-2 狗粮测量 | facet 契约模式 9 发实跑、tally R1-R7 核对、gate_verdict 出 stable_clear 或挂起归人、全程无钓样本 | 过 | 合同 adisp-guard-1 契约哈希 82266a20 绑定响应哈希 bf4d55a6，9 发实写作答 9 合零边界零空转，闸裁决 stable_clear 待人签核；tally check 全过 12 项零失败（R1 至 R7 加 R5 当日基线可用同哈希配对），处置裁决通过方向 comply；终签材料与得分明细落 facet_task_packages/adisp-guard-1/ 十一件 |
| F-3 链上落据 | crosscheck_completed 事件入当日链、disposition 与 gid 载狗粮命题、非自证即非 mbgate 系列、基线先行有对照材料 | 过 | 事件哈希 2ada7331 载十五字段 gid adisp-guard-1 处置裁决通过方向 comply n_shots 9；非 mbgate 前缀即链上首笔非自证机器终签；基线材料件 sealwin2 与 sealwin3 封窗先例加 goldfix 六笔直提违章加 reconcile 双零读数加 crosscheck 自证基线四节全载 |
| F-4 收口 | 双仓 settle、链 valid、reconcile 双零保持、本批产物全量入版控、inputlog 逐字 | 过 | 双仓 lease commit settle 归并、当日链 verify valid、reconcile 双仓 unrouted 0 cert_missing 0、inputlog 补录 seq 2 至 5 四笔逐字、identity 件不入版控 |

## 二、第一步立名实录

查档：检词 query 裁决分派、AdjudicationRouting、adjudisp 三串全零命中（state unknown），DEC 清点 020 为最新。推导：正推从自证循环与损补两哲学锚出三态，反推从链上裁决类型盘点态位，覆盖面穷举表目未尽入灰区从严。落档节构：概览、命名集、推导、三态分派表（过得一、人节点、代裁、灰区四节）、适用界、与硬件钥匙模型的关系、备选方案（全过得一与维持现状两拒）、席位与连带。哲学锚引文从 07-on-assay 自证循环节与 06-on-canon 损补条程序切片逐字节子串载于 ask3 记录，DEC 正文不承载原文承 000 格式。检词登记：zh 裁决分派、en AdjudicationRouting、code adjudisp、state established、source 本批、signed 2026-09-02，manifest 升 0.5.0。

## 三、第二步狗粮实录

命题：lease 应增 pre-commit 守卫拦无模板 plain git commit，--no-verify 显式留痕放行。基线先行：goldfix 六笔（engine 23df965、8f85112、9732b22 加 tools 0238bc6b、5bfb95b6、0add10f3）直提违章在追认档，sealwin2 界线 22550a6 与 sealwin3 界线 4146851 与 91d14d4f 两度封窗先例，开工 reconcile 双仓 unrouted 0 cert_missing 0，链上 crosscheck 仅 m-mbgate-scrut（a0bbcb40）与 m-mbgate-deyi（bb30582e）两笔自证。测量：facet emit-contract 出题 9 发，席位 ZCode:GLM-5.3-Flash:self-reported 逐发实写作答，score 九发九合零边界闸裁决 stable_clear。执契：R5 当日席位基线经 temp_probe 标定 20 发判可用体温 0.0 准确性过且与材料同哈希 c80d562d 配对，tally check 12 项全过处置裁决通过，tally sign 经引擎 scribe crosscheck 落 crosscheck_completed 事件。最终开权归用户：守卫是否实装由用户裁定，本测量只出裁决材料。

## 四、越线与误差申报

1. facet score 的飞轮 trail 按工具契约落 proposition/DES/adisp-guard-1/，该路径在包请求写入节之外且 lease lock 扩围被拒（scope_violation），已将 flywheel-trail.jsonl 逐字节迁入 facet_task_packages/adisp-guard-1/（迁移前后 sha256 同为 7daa1985，proposition/ 目录回净零差异），score 材料内 trail_path 等三处路径引用同步改为域内实位，合同与响应哈希复算不变。
2. R5 当日基线经 temp_probe score 生成时账本 append 落在 facet/probes/calibration/ledger.jsonl（域外已跟踪件），为守 scope 纪律已 git checkout 回退该笔账本行，基线 json 本体另存域内材料件，申报此账本行未随批入版控。
3. 核阅 des-001 对工地路径固有域外：DEC-021 工地路径核阅 exit 2 域外如实记，另以同内容经 scrutinator/fixtures/corpus 域内路径重跑得零违规实证（内容哈希 b79f3702 与工地件一致），两报告俱在档俱认证。
4. tally sign 首跑因核对报告 material 字段为绝对路径被跨方核毕守卫拒（scribe exit 非零），改从 facet 工地基目录以相对材料引用重跑过，首跑 signcheck 被重跑覆写，拒签守卫行为如实记。
5. 分派令适用界从本批起算既往不回溯：链上既有裁决零回溯重裁，本批狗粮为分派令表目首例适用（命题类裁决过得一）。

## 五、收口

链：intent 32750df8 加认证若干加 crosscheck 2ada7331 入当日链。双仓 settle 与归并与 reconcile 对表见链事件与 git 记录。调用册六件各补一行随批提交。identity 件不入版控。存量 untracked 零收编：batch-materials、c006-sb3、viewimpl 系、08-30 inputlog、task-packages、sealwin3-solo-materials、leasepatch 与 proposition-defense 两个 M。
