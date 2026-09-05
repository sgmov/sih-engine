# viewline 线任务包 v1：视图组件线（协调视图与实时告警）

> 线：viewline（视图线，主线 v2.2 退出标准第四条承载，用户 2026-09-05 令源直入）
> 令源：「视图要进入向界」点名 pk-047 + pk-059；承「人类视图统一由视图组件承载」「jsonl和数据库双写」裁定
> 形：单线 solo 逐批，每批独立立约独立收约

## 一、问题陈述 {#problem}

- 人类视图现由 agent 人话汇报过渡承载，视图组件（viewer 六席位，src/view 在位）本体未实装（泊界向界语「远端组件零实装」）。
- watch 对表（watchcheck-solo 批在飞）产出告警语义后，缺人类消费面。
- 置信度组件（用户裁定指向）的聚合视图缺承载位。

## 二、批次拆解 {#batches}

**批一 viewcontract（契约起草批）**
- viewer 契约：数据源=链投影库（jsonl 正本 + 数据库双写投影，重建逐字节可复现）；告警语义=watchcheck 对表输出（无主修改清单、豁免面、退出码）；展示面=泊位状态、链簿事件流、置信度聚合视图（依赖 idenlane 信封绑定在役）；参考面=pk-047 OpenTelemetry GenAI 语义约定采撷。
- 产出：契约文档（doc/spec/）过得一裁（展示语义与数据边界判定），批二按契约实装。

**批二 viewerimpl（实装批）**
- viewer 按契约实装，接投影库与告警语义；人类视图自 viewer 在役日起由组件承载，agent 人话汇报降为补充。
- 退出标准第四条达成即主线 v2 结算解锁件之一。

## 三、跨批红线 {#redlines}

- 数据源纪律：viewer 只读投影库与投影面，正本链零直读直写
- 判定语义（展示边界、告警分级）过得一裁
- 契约先行：批二不得先于批一契约落档开工

## 四、验收（线级） {#acceptance}

- viewer 在役：人类不开数据库、不翻原始台账即可知链上事件、在泊状态、告警异常
- 告警链路：watchcheck 告警 → viewer 呈现 → 人节点二值裁决 → 回滚或通道，全程留痕
- 置信度聚合视图有位（数据源依赖 idenlane 信封绑定）

## 五、必读文件 {#read}

- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md（v2.2 第四条）
- sih-engine/sih/event/plan/leaseoptsettle-solo-results.md 第六、七、八节（双写与视图裁定）
- sih-tools/lease/ledger/checks/ 与 watchcheck 产出（告警语义源）
- sih-engine/sih/state/parking/materials/pk-047-exit.json（参考面材料指针）

## 六、请求写入（批一） {#requested-writes}

- sih-engine/doc/spec/（viewer 契约文档）
- sih-tools/proposition/DES/（契约判定测量材料）
- 结果档随批
