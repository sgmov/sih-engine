# gateshape 结算结果档（判据形借入批）

> 批身份：单线形 solo，委外代理亲写零子代理。会话 1bcd413b6d914b4f；意图笔 7debeae8（2026-09-11 trail）；新词认领在案：gateshape，概念锚「闸形」，code 形显式申报无承，派生 gateshape:new——术语注册候立名程序人节点终裁，本档用语以「判据形」与既有词承载
> 承接：用户 2026-09-11 令「你能立批进行改造吗？」；上游即同日三测试包拆解清单（case-01、case-26、jsonl-db）与「同一个病」定位
> 数学根据：PROB-007 期望、PROB-010 假设检验、PROB 信念更新、PROB-012 相关、PROB-014 期望信息增益、PROB-008 信息内容与可证伪性；零命中申报：数据处理不等式、博弈占优（未入文承重）

## 一、F 表验收 {#ftable}

### F1 critsweep 阈值封印——对表验收（改判：refseal-solo 批已实装）

- 开约后摸底发现判据面封印已由 refseal-solo 批落进 sih-tools 基础分支：`enforce_threshold_seal`（sweep.py:372）、`--threshold-reason`（:410）、`register_threshold_override` 登记 ledger/referee.ndjson（:386、:432）。本包拟 threshold_source 字段改判不做——对方字段形 threshold_registered 机制等价且更强（事由强制加登记加降级可见），零重复实装
- 活体验收双路径：无事由 `--threshold 7` 即拒（教学 JSON，reason_code threshold_reason_missing，gate 裁判面封印 m-referee-seal-r2）；有事由同参即过，threshold_registered true，ledger 末行 {at, reason: "gateshape F1 对表验收登记", threshold: 7, tool} 在案
- V1 判：过（以 refseal 实装形验收）

### F2 tally R9 证据登记闸——本批新增

- 红先：tests/test_r9_evidence.py 六测先行，4 红 2 绿（红即缺证拦截三类与成形通过，绿即缺席跳过重放类）；实装后全绿
- 实装：src/tally/cli.py R8 块后增 R9 块——材料自带 evidence 数组即逐项机械对表，required 项 status 非 provided 即 failed 落 R9（where 载逐项 id 与实际 status），处置走优先级映射即材料退回；evidence 缺席即跳过，r1 与 r2 旧材料重放逐字节同判；synthetic 在非 required 项合法（诚实标注的合成证据不算缺证），required 项即拦；malformed 项即拦
- 绿证：tally 全套 31 passed（旧 25 全绿即重放兼容实锤，新 6 即 R9 行为面）
- 判据形出处：case-26 缺证闸门形布尔化（缺证不得标 ready）；数学根据 PROB-008
- 边界如实：evidence 数组本身系材料自报，完整性锚材料整体哈希绑定链（R2 复算），逐项独立锚定候后继

### F3 facet 复核输入独立性条款——本批新增

- CONTRACT.md 修订记录增 2026-09-11 gateshape 笔：复核与终签位输入锚原始材料（采样合同、响应原文、当日基线），上一环节结论件不得回流为下一环节判定依据——tally 终签按基线核对不按本工具判定行，计分材料装配锚响应哈希不锚模型自述；成文钉规，调用面与契约件零改动
- contract_mode.py 模块 docstring 随附一行同源指针
- V3 判：过（条款在场，行为零改动故无测试面）

### F4 裁判面查己清单——本批新增

- 落 sih/event/plan/gateshape-solo-materials/referee-audit.md：十件判据面逐件三件套对表
- 判档合计：sealed 六件（critsweep 阈值、lease bypass、tally 常数族、mcpline 与 identity 结构输入、R9 本批新增）、unsealed 两件活体标本（gauge --union-threshold 承重件数覆写零登记、ROOTANCHOR_DISABLE_SELF_BOOT 环境阀静默过闸）、疑似两件（reconcile --limit 覆盖收缩、pack 任意外包零警示）
- V4 判：过（全子工具扫、活体标本在档）
- 后继批候令（可合一批 refseal2）：gauge 阈值照 refseal 形封印、ROOTANCHOR env 告警与登记、reconcile coverage 申报字段、pack provenance 警示行、R9 evidence 逐项哈希锚定

## 二、验收总表 {#verify}

| 包内可证伪条件 | 结果 |
|---|---|
| V1 critsweep 阈值封印 | 过（refseal 实装形，双路径活体验收） |
| V2 tally 缺证闸拦截与重放兼容 | 过（红先绿后，31 passed） |
| V3 facet 条款在场且检词零违例 | 过（管线见下） |
| V4 查己清单全覆盖含活体标本 | 过 |
| V5 链 verify valid、reconcile 双零、码面全经锁面 | 结算段执行（见链证据） |

## 三、链证据 {#chain}

- 意图笔：7debeae8（event_id 6a9c0ad2，2026-09-11，--no-session-reason 批首放行）
- 开约：会话 1bcd413b6d914b4f，stem 查册闸 new_coinage_acknowledged（gateshape 认领三件在回执）
- 结算笔：见本档修订记录追加（cert 认证笔哈希结算时补录本节）

## 四、未决 {#open}

- 术语「闸形」候立名程序人节点终裁（甲表三件已认领，收敛后 nomenclator register）
- refseal2 合批候令（F4 后继封印五件）
- DES-011 零改动：R9 条件触发零版本升，规则版 des-011-r1 与 des-011-r2 材料行为不变，故无契约源修订需求
