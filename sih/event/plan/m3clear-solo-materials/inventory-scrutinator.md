# 盘点档：scrutinator（核阅）

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表，sih-tools 侧）；线索：疑已随后续批有载体（gatecap ORD-008）

## 机制职责一句话

空腹谓词引擎：规则包（纯数据 toml）驱动六类 text 谓词（line_rule/line_flag/nav_format/list_format/table 流计等）逐行求值出 findings，包数据与引擎分离即包可携判定语义（scrutinator/src/scrutinator/engine.py 评引擎核）。

## 线索核验（源码引用位实查）

**已坐实有载体（包级）。** gatecap-solo 批（2026-09-03/04）为 des-001 规则包增 C007/C008 两违规类并接 ORD-008 载体：

| 引用位 | 位置 | 承接语义 |
|---|---|---|
| 包规则载体声明 | sih-engine/src/scrutinator/packs/des-001/rules.toml:80 | 「载体：ORD-008 引用图可达与孤悬判定（mapping.md:184「引用是否可达无孤悬」）」 |
| 契约载体节 | sih-tools/scrutinator/CONTRACT.md:61 与 :65 | C007 判定性常数裸奔即孤悬节点行级显形，C008 推导档缺指针即出边缺失，包版本 0.1.0 升 0.2.0，零改判定引擎 |

界定：载体在**包级**（规则数据承 ORD-008 判定语义），引擎本体（六类 text 谓词求值核）仍无在册条目对照——本档如实分层申报。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| C007/C008 规则判据 | sih-engine/src/scrutinator/packs/des-001/rules.toml:80 载体声明行 | 孤悬判定与出边判据 | 判定性（已承 ORD-008，gatecap 实例化） |
| 表格流计上限 | scrutinator/src/scrutinator/engine.py:87 `if table_run <= 3` | 表格违规只记首三行 | 资源性（findings 降噪形：不改退出码通拒，只改 findings 产量） |
| 谓词种类枚举 | scrutinator/src/scrutinator/engine.py:75-100（nav_format/line_flag/list_format 等 kind 分派） | 六类 text 谓词求值语义 | 判定性（引擎本征面，暂无在册条目对照） |
| 退出码枚举 | 0=零违规 1=有违规 2=工具异常/域外 | 三态通拒 | 判定性 |

## 既有归属判

判定性为主，一处资源性。违规判定与退出码直接拦提交；表格流计上限三行是 findings 降噪常数（rev1 原判待确认，本批改判资源性走环境参数登记面）；谓词种类枚举属判定性但条目对照缺失如实申报。

## 载体候选分析

- 包级已实例化：des-001 包 C007/C008 判定语义由 gatecap 批接 ORD-008（mapping.md:184，已建实存），包 0.2.0 在役，金向量八件零漂移在案（gatecap 验收）。
- 引擎本征六谓词族（正则匹配求值）暂无在册条目，零命中如实记；本批单一命题立包级判定语义，引擎本征面如实备注不另立命题（一命题一批次原则，引擎面候选输入随处置清单备注留载体管线）。
- 表格流计上限 3 资源性行走环境参数登记面增量件。
- sih-tools 侧源码概念引用位实测：sih-tools/scrutinator/src 零命中（载体在引擎侧包数据，本批 grep 实查 2026-09-04）。

## 证据行

- rev1：summary-rev1.md:10/11 两处 scrutinator（sih-tools 无可指认 + sih-engine 可指认未实例化，账本时点读数，被 gatecap 批推进）；ledger.json pending_confirmation 一条（engine.py:87 表格流计 3，原判待确认，本批改判资源性）。
- 契约：sih-tools/scrutinator/CONTRACT.md:61/65 载体节 2026-09-04 修订四在案。
