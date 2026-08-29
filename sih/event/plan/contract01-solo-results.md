# contract01-solo 结果档

- 批次：contract01-solo，单线形即主线亲写零子代理
- 会话：sess-zcode-260829-contract01，意图 0f36d7df969a9bfb，租约会话 0dd1d5cfffaeada5
- 日期：2026-08-29

## 一、完成度表

| 工作项 | 状态 |
|---|---|
| 规格 CONTRACT-MODE-SPEC.md v1 | 已落 facet/docs，两半切分与两工件格式与三腿接线与范畴排除与 F 判据 |
| 共享件 contract_mode.py | 已落 facet，出题计分验证纯函数，零 LLM 零网络零 key |
| measure.py 两段式 | emit-contract 与 score 入口，一次性直跑模式原样保留 |
| singleseat.py 观察腿合同模式 | emit-contract 与 score-contract 两子命令，与直跑同一 derive 解析路径，子进程复用 k2t_driver |
| dose_driver.py 调整腿两段式 | emit-contract 与 score 子命令，响应计分与直跑判定流同构，dose_analyze 照常消费 |
| 测试 | test_contract_mode.py 九测全绿零网络，全套 466 过 6 挂（挂者皆 ng_assembler 工作区路径既登记缺陷，主仓同测全过，与本批无关） |
| skill 修订五 | facet-measure SKILL.md 合同模式节加修订记录，state 权威源与 .agents 实体投影双写 |
| 任务包与结果档 | 已落 sih 两 plan 位 |
| 管线三件与三证 | 化格与核阅与检词跑毕如实记，三证入 08-29 工作区链 |
| 双仓段结算收约 | 已执行见调用册 |

## 二、F 验证表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| F-1 | 规格定稿 | 过 | facet/docs/CONTRACT-MODE-SPEC.md §0–§6 全载 |
| F-2 | 观察腿合同化 | 过 | measure 两段冒烟通；test_runner_path_byte_identity_mocked 过真 runner 路径 mock 对表 system 与 user 逐字节一致；singleseat 合同模式与直跑同一 derive_probes 加 seat_prop_ids 加 k2t 子进程路径 |
| F-3 | 调整腿合同化 | 过 | dose_driver emit-contract 与 score 幂等续跑，append_contract_runs 同构 flywheel_run 记录 |
| F-4 | 测试零网络全绿 | 过 | 9 passed；含响应严格对表五违例、原文解析四变体、哈希绑定、幂等追加、yaml clip 同源 |
| F-5 | skill 修订五 | 过 | 合同模式节与修订记录入 SKILL.md，state 与 .agents 投影 diff 一致 |
| F-6 | 治理收口 | 过 | 管线三件、三证入链、双仓段结算收约、对表与心跳绿态 |

## 三、越线与误差申报

1. yaml 块标量 clip 缺口：合同出题初版用裸 NG 文本，与直跑 runner 所见形差一尾换行，测试对表捕获后以 scheme_clipped_ng 补齐同形并加 yaml 往返对表测，本批内闭环。
2. 观察腿 emit 对未满额既有格沿用 setup_one_prop 清空重建语义，与直跑 run-probe 同一幂等防御，未新增加固；满额格照旧跳过保护。
3. recall #1 复合主题词零命中、原子词「接口」三条相邻先例（旧审计接口、斜杠协议接口、维护任务接口），零直接先例如实记；recall #3 命中今日链事实切面，输出件随批入档。
4. 单测套 6 挂为 ng_assembler 读 worktrees/sih-tools/AGENTS.md 缺席，属 dose01 批既登记围堰缺陷（缺省相对根读不到工作区根），主仓同测 9 过，非本批引入。

## 四、残留申报（不属本批）

- ng_assembler 相对根缺陷的根修（参数化或路径解析改仓群根），候补工具批。
- k2t_driver 模块级 parse_args 使进程级旗标不可二次覆写，子进程复用是现行唯一安全形，既有登记改进候补。
- .agents 投影不受版控（仓库群根不建仓裁定），物理双写为现行惯例。

## 五、队形声明

单线形即主线亲写零子代理，治理批默认形，选形即声明，本批零子代理实跑与声明一致。
