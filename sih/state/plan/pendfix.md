# pendfix：pendline dispatch 提示词 basis 枚举声明修复

> 令源：用户 2026-09-18 令「修复」即 pendlive 批首活缺陷披露候令件；同令司梦域裁定即暂时忽略不纳入 sih-engine 治理候令簿除名（记档不入本包域）
> 病灶实证：pendlive 批 dispatch 腿产出合同被 facet 评分器 declared_basis_enum 硬拒即 ValueError shot r1，评分读数与残次合同俱在 pendlive-materials 在档
> stem 认领：pendfix，甲表三件即 zh pendline 提示词枚举修复、code 无承、派生 pendfix:new

## 问题陈述 {#problem}

src/pendline/dispatch.rs 的 v1 固定系统提示词只令输出 JSON 判定四键未声明 basis_regulation 合法枚举，facet 评分器合同模式强制从各发 system_prompt 提取枚举零回落（contract_mode.py declared_basis_enum），pendline 自产合同进不了自家判词上游即 dispatch 与评分两腿断裂。

## 关键设计 {#design}

提示词补输出格式行即字面 basis_regulation 值段 baseline_1 或 baseline_4 或 baseline_5 承 adjudicate2 rubric 同形，四键 schema 全声明；单元测试按评分器同语义正则即提取值段按或分割断言枚举集恰三值且各发一致，先红后绿；活体验收即修复后 dispatch 产出合同经 python declared_basis_enum 解析通过加 fixture 回填评分零拒。

## 工作清单 {#work}

- [x] pf-01：SYSTEM_PROMPT 补四键 schema 与枚举声明
- [x] pf-02：单元测试红转绿（评分器同语义提取断言）
- [x] pf-03：活体回路验收即 dispatch 产出合同过 python 枚举解析与评分
- [ ] pf-04：settle 加 close 加结果档

## 验收 {#acceptance}

单元测试先红后绿在档；活体回路 python declared_basis_enum 解析通过读数在档；既有 pendline 十五测试零回归；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/src/pendline/dispatch.rs
