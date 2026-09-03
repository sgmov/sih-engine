# 盘点档：latex-helper

- 批：m3clear-solo ｜ 件序 F-1 ｜ 日期：2026-09-04 ｜ rev1 状态：无可指认载体（summary-rev1.md 载体三态表）

## 机制职责一句话

LaTeX 数学计算与档面辅助引擎：compute 面封装 SymPy 十二类数学运算（积分/级数/极限/求导/解方程/化简/因式/展开/泰勒/行列式/逆/特征值），另有 validate 与 suggest 与 autofix 与 knowledge 与 block-create 五辅助面（latex-helper/src/latex_helper/compute.py:1-15 模块docstring）。

## 判定性常数与判据位枚举

| 判据位 | 位置 | 语义 | 归属判 |
|---|---|---|---|
| 退出码枚举 | latex-helper/src/latex_helper/cli.py:33-35 `EXIT_OK = 0 / EXIT_VIOLATION = 1 / EXIT_ERROR = 2` | 合规/违规/异常三态 | 判定性 |
| 运算种类枚举 | latex-helper/src/latex_helper/compute.py:4-14（integral/sum/limit/derivative/solve/simplify/factor/expand/series/matrix_det/matrix_inv/matrix_eig） | 十二类运算判定语义 | 判定性 |
| LaTeX 解析通道 | latex-helper/src/latex_helper/compute.py:26-29（pylatexenc 或 sympy.parse_latex） | 输入解析通道选择 | 待确认（通道实现属库行为） |

## 既有归属判

判定性为主。rev1 账本原判三常数待确认；本批改判：退出码枚举判违规位属判定性（证据 cli.py:33-35），运算种类枚举属判定性（compute 面输出直接进数学档面），解析通道选型属待确认（外部库行为不在源码判定面）。rev1 账本三件只读，改判在本档申报不回写账本。

## 载体候选分析

- compute 面十二类运算的判定语义与数学仓 calculus/algebra 已建条目族直接同构：极限 ↔ LIM-007 epsilon-delta 定义（mapping.md:68）、累加/积分 ↔ INT-007 微积分基本定理（mapping.md:102）、级数 ↔ SER-001 无穷级数（mapping.md:69）、矩阵特征值 ↔ ALG-001 矩阵与特征值（mapping.md:179）。
- 实例化候选形态即 compute 面源码判定位注释锚点接四条目族（selwire 注释锚点先例），零行为变更。
- 源码概念引用位实测：sih-tools/latex-helper/src 零命中（本批 grep 实查 2026-09-04）。

## 证据行

- rev1：summary-rev1.md:11 无可指认载体十件含 latex-helper；ledger.json pending_confirmation 三条（cli.py:33/34/35 退出码常数，原判待确认）。
- 消费面：sih-math/llm-friendly-build/mapping.md 全读（232 行），四条目状态已建。
