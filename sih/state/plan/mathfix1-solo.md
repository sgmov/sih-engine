# mathfix1-solo：数学仓拓扑序论卷修复批（修复计划批一）

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图（本轮 round 5），用户同日继续令开批一
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

摸底批体检证实拓扑五条与序论四条共 34 处数学与锚点硬伤：TOP-003 与 ORD-003 定理误述（dcpo 最大不动点不存在、完全格单调迭代须超限）、虚构连接四处、年代错配五处、无意义句三处、无效证明两处、九条引用不存在的 PRO-12 与 PRO-13。

## 二、关键设计 {#design}

改口三去向按字义相容裁决：PRO-02 道一（发散自然收敛必为，原文 80 行）承接全部收敛类断言，定位为收敛实现条件；PRO-07 与 PRO-08 真引用保留；PRO-13 链外补充与 PRO-12 范式无真命题可挂即删句。定理重写按正确三层陈述：完全格加单调（Tarski 两点都有）、dcpo 加 Scott 连续加底元（Kleene 仅最小不动点）、一般单调须超限迭代（Cousot-Cousot）。虚构调用位改候选设计标注。九条改后跑锚点核验脚本：所引命题号须在册、引语逐字、节名实存。块引用状态行转正文句消 F002。

## 三、工作清单 {#work}

- [ ] 九条逐条修订（数学纠错加锚点改口加状态行）
- [ ] 锚点核验脚本全绿
- [ ] 核阅复跑无新增违规类且 C006 计数不增
- [ ] 化格检词复跑零新增
- [ ] fmtfix 丢失认证事件再认证搭载
- [ ] 认证上链三仓收约对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 假锚清零 | 治理 | 九条 PRO-12 与 PRO-13 全仓零残留，新引命题号核验脚本全绿 |
| **F-2** 数学硬伤清零 | 工程 | 摸底 34 处 a 类与 c 类发现逐处有修复，反例不再适用 |
| **F-3** 无新增伤 | 工程治理 | 核阅九条违规类计数 C006 不增且无新类，检词无新增，diff 无第三类变更 |

## 五、必读文件 {#read}

- 必读 1：sih-math/docs/survey-mathprobe-2026-08-30.md 第三节体检清单
- 必读 2：sih-philosophy/emanation/proodos/02-on-first-tao.md 即道一原文

## 六、约束 {#constraints}

1. 锚点仅挂 PRO-02、PRO-07、PRO-08 或工程基线，不造新号
2. C006 不顺手修，计数不增即达标
3. 不动 PROB、ALG、calculus
4. 上链遇锁即等待不绕行
5. 词债不过夜

## 七、请求写入 {#requested-writes}

- sih-math/topology/entries/
- sih-math/order/entries/
- sih-engine/sih/state/plan/mathfix1-solo.md
- sih-engine/sih/state/plan/mathfix1-solo-results.md
- sih-engine/sih/event/plan/mathfix1-solo-results.md
- sih-engine/sih/event/plan/mathfix1-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/proposition/topics/2026-08-30-mfix1.md
- sih-tools/proposition/DES/m-mfix1/
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链含 fmtfix 再认证，三仓段结算收约，对表读数在档
