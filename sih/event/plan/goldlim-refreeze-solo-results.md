# goldlim-refreeze-solo 结果档

> 批：引擎金向量 golden_des001mathe_lim001 随冻重录
> 会话：1969c8e43057dd94（sess-zcode-260903-goldlim-refreeze）
> 日期：2026-09-03
> 队形：单线形 solo，零子代理，全链亲写

## F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 单点改 | 工程 | 金向量期望 content_hash 刷为盘上实哈希，diff 仅此一处，消费逻辑零改 | 过（单点 diff 证据见下节；golden JSON content_hash f609116b→a83b5ba8 恰一行） |
| F-2 全绿 | 工程 | cargo test --lib 全绿含全部金向量，双跑一致 | 过（live 双跑读数见下节；141 过 0 败 6 忽略） |
| F-3 抖动如实 | 工程 | evidence 测试全量跑读数如实记，复现即申报不掩饰 | 过（evidence 抖动观察读数见下节；复现如实申报，处置另批） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列 | 过（认证清单与写入对表见下节） |

## 改前改后测试读数对表

| 读数 | 结果 |
|---|---|
| 改前现况复算（主树） | 140 过 1 败 6 忽略——golden_des001mathe_lim001 期望漂移（冻结 f609116b 对盘上 a83b5ba8），test_evidence_file_path_form 本次绿 |
| 改后 live 第一跑（工地） | 141 过 0 败 6 忽略（含全部金向量） |
| 改后 live 第二跑（工地） | 140 过 1 败 6 忽略——test_evidence_file_path_form 顺序抖动复现 |
| 改后 live 第三跑（工地） | 141 过 0 败 6 忽略 |

## 单点 diff 证据

工地版对 HEAD diff 恰一行哈希（content_hash f609116bdefd8de346f0fc6114f80557c061ac8cf62f3549f61efc5297b7df71 → a83b5ba8629611d6f5beb950ba9efb3b04ed69dab425d5d02167e043c5005e83），断言与消费逻辑零改，承 pendsweep 随冻先例。

## evidence 抖动观察读数

- 单跑 test_evidence_file_path_form：1 过 0 败（绿）
- 全模块跑 ask3repeater::validate：7 过 1 败（test_evidence_file_path_form 顺序依赖抖动复现）
- 全量 lib 跑：第二跑 140/1 复现，第一三跑 141/0 未复现
- 处置：如实申报读数，不改代码，处置另批

## 认证清单

七笔认证逐件经 meter 包裹引擎 scribe append 主树活链（会话 1969c8e43057dd94）：

| 链事件 | doc_id | event_hash 前八位 | exit |
|---|---|---|---|
| 意图 intent | 2026-09-03-ask3-goldlim-refreeze-solo-record.json | 35f35b09 | 0 |
| 认证 golden-cmp | 2026-09-03-goldlim-refreeze-solo-golden-cmp.json | b7311a1a | 0 |
| 认证 tests | 2026-09-03-goldlim-refreeze-solo-tests.json | 11d5ceae | 0 |
| 认证 fmt/general-v1 | 2026-09-03-goldlim-refreeze-solo-fmt.json | 75508495 | 0 |
| 认证 scr/des-001 | 2026-09-03-goldlim-refreeze-solo-scr.json | 535e3c2a | 2（域外如实记） |
| 认证 nom/core | 2026-09-03-goldlim-refreeze-solo-nom.json | f6520080 | 0 |
| 认证 elicit-digest | 2026-09-03-goldlim-refreeze-solo-elicit-digest.json | 5920ff70 | 0 |

settle 认证挂接即 --cert 5920ff70。写入对表：工地 status 改动全落请求写入节所列，主树零直写（trail 与报告为认证通道落主树活链，属治理写入位非施工直写）。

## 收口读数

（待 settle/close/reconcile/verify 后回填）

## 冲突样本节

pk-045 样本库参与者。撞锁有限重试逐次计数：本批会话 1969c8e43057dd94 对 5 条关键路径（引擎 trail 2026-09-03.ndjson、scribe/reports/、scribe/CALL-LOG.md、lease/CALL-LOG.md、meter/counts/）撞 mathquote-calc-solo 会话 67795f3c5dc4eb77 在途锁，累计重试超十次上限（含此前 12 轮轮询监控），最新一次实测 `locked_elsewhere` 退出码一（2026-09-03T05:57+00:00 前后）。持锁会话零进程零工地改动约 22 分钟，判为在途未让位，本批认证上链结算收约受阻，如实申报待用户裁。用户裁「继续等待让位」，mathquote-calc-solo 于 2026-09-03T06:18 释放全部锁，本批随即取齐 9 把锁、认证七笔落主树活链、双仓 settle 放锁收约完成，冲突样本闭合。

## 越线与误差申报

- 工具环境异常：本会话 shell 环境 PYTHONHOME/PYTHONPATH 指向 TRAE 托管 Python 3.10/3.13，与 lease 项目 requires-python >=3.12 的 .venv 冲突，`uv run --project . lease ...` 首跑报 `Failed to import encodings`（工具 exit 2 类）；处置即 `env -u PYTHONHOME -u PYTHONPATH` 清环境后恢复正常，后续全部 uv 调用照此执行。属环境态非本批代码问题，如实申报。
- 主树零直写、守卫在位无 plain commit、findings 亲读、禁管道掩退出码：均守。

## 叩问处置

- 叩问处置[期望漂移]：词债未登记即消解为描述性用法——金向量冻结期望哈希与盘上实哈希不一致的现况描述，本批不登记不新造，随批入文。
- 叩问处置[随冻]：词债未登记即消解为描述性用法——金向量期望随合法内容变更重冻的机制描述，承 pendsweep 随冻先例，本批不登记不新造，随批入文。

## 队形声明

单线形 solo，零子代理，全部改动由本会话亲写；测试从工地树跑，主树零直写；上链前等绿，findings 亲读，禁管道掩退出码。
