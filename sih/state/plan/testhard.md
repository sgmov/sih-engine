# testhard：测试加固三件

> 令源：用户 2026-09-18 令「那你调2个子代理，把2波都做了」第一波承载
> 病灶出处：f7 系 defectwave 波后全量回归实录（活工作区竞态单跑绿）；假红族系 defectwave 子代理环境处置留痕申报；park 重放面系 pendline-results.md 偏差申报七
> stem 认领：testhard，甲表三件即 zh 测试加固三件、code 无承、派生 testhard:new

## 问题陈述 {#problem}

三件测试与重放面缺陷：一即 f7 同参双跑对真 trail 断字节一致属活工作区竞态（并发写链窗口内必假红）；二即治理二进制寻址走 CARGO_MANIFEST_DIR/target 形在 worktree 加共享 CARGO_TARGET_DIR 形下必假红（scrutinator 21 件加 event_stream 数件，基点同红主树绿）；三即 scribe park 泊界重放面解析 trail 同目录全部 ndjson 非链文件会被误吃。

## 关键设计 {#design}

f7 改冻结输入双跑（trail 快照进临时目录，两跑指快照断字节一致，真 recall 路径零变）；假红族统一切 cargo 标准机制 CARGO_BIN_EXE_binname 编译期注入寻址（二进制缺席降级用例改显式坏路径注入）；park 重放收窄为规范链文件即 --trail 件与同目录日期形件，splinter 处理白名单保留。

## 工作清单 {#work}

- [ ] th-01：f7 冻结输入双跑
- [ ] th-02：假红族 CARGO_BIN_EXE 化双形验收
- [ ] th-03：park 重放面收窄红转绿
- [ ] th-04：settle 加 close 加结果档

## 验收 {#acceptance}

三件各先红后绿红证在案；主树与工地双形验收皆绿；全量回归零新红；splinter 去重行为回归钉在案；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/tests/mem_recall_f_suite.rs
- sih-engine/src/scrutinator/（测试寻址点）
- sih-engine/src/event_stream/（测试寻址点与泊界重放收窄）
- sih-engine/tests/（新测试文件）
