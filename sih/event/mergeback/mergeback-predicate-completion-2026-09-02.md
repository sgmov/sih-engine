# 谓词融回完成档（mergeback-predicate-completion-2026-09-02）

批：autoflow2-solo。会话号：f7a97a90ba1b0c7c。日期：2026-09-02。
承接：SPEC-015-predicate-mergeback-gap.md 与 GOV-002 退出标准判据二三五与 DEC-013 融回门机制与 predsplit-a1/a2/a3 三枚 stable_clear 终签（在链当日 161 至 163 位）。

## 融回落位清单

- src/attractor/route.rs：路择谓词机即 pack 装载（manifest.toml 加 routes.toml 全量校验）加十谓词 kinds 求值加三路输出加批级告警四件
- src/attractor/packs/core/ 与 packs/parking/：两包四件纯数据随迁，与围堰原包逐字节一致（cmp 四件全 IDENTICAL）
- src/bin/attractor.rs：增 route 子命令即第七子命令，参数面与退出码三值对表围堰 cli.py
- src/ask3repeater/intercept.rs：截流谓词装配位即 load_intercept_pack 加 round_interception 两公共函数，判定语义全走 route 谓词机单一实现
- src/lib.rs：导出 attractor::route 与 ask3repeater 两装配函数
- src/attractor/fixtures/route/golden/：金向量八场景冻结，围堰 selector 实测输出为唯一基准
- tests/attractor_route.rs：T1 至 T6 集成十件
- 过程件归零：task-packages/f-anchors-x11-t6d.md 复制入档 sih/event/plan/ 内容零改（cmp IDENTICAL），原位删除，task-packages 目录归零即零 facet 过程件

## 三查对表（DEC-013）

第一查 验收官判据全过
: SPEC-015 A1 至 A6 逐条机械判定：A1 金向量逐字节即八场景 cmp 零差含坏包信封（T2 十件集成全绿）；A2 退出码对齐即 0/1/2 全表（T3）；A3 谓词机语义零漂移即十 kinds 正反例与 fail-closed 与缺省败向与告警四件与判定序与包校验全量（T4 模块内单测）；A4 机械腿不变量即零网络零 LLM 零 key 源码扫描加 Cargo 依赖断言（T5）；A5 多包归因即同材料集双包双报告冻结互为镜像（T2）；A6 截流装配位走谓词机加三问既有测试零改全绿（T6）。cargo test 全测试面 179 passed 0 failed。证据：tests/attractor_route.rs 十件与 src/attractor/route.rs 模块内 T4 单测与 sih/event/plan/autoflow2-solo-materials/tdd-red-green.log

第二查 接口契约未变
: 围堰 selector CONTRACT 修订三现行文与引擎实装对表零语义漂移：route 子命令参数面逐旗标对表、报告 json 四顶级键与逐材料五键形态、退出码三值、十 kinds fail-closed 语义、route_on_fail 缺省四件与告警位禁配、空包合法与空批绿态、域声明入报告头不做逐件域判定（SPEC-015 显式范畴排除）。围堰 CONTRACT 文件零改动即契约文本面未动，退役标注归切换后批承载。已知差异面如实列：toml 与 json 解析失败的运行时报文文本不逐字节复刻（CPython 异常文本），拒收退出码与信封形态对表，字节冻结面只钉包校验可枚举类，SPEC-015 模块头显式声明

第三查 回迁债已评估
: SPEC-015 回迁债节逐项处置：依赖面零新增（toml 加 serde_json 加 regex 加 chrono 全在既有 Cargo 依赖）；DEC-001 归位映射行已载即谓词机与包装载与路由输出归位、围堰 CLI 原位保留双模并存、切换后批承载管线调用面换旗；域逐材料判定显式不做。挂起项零

## 双跑判据执行实录

同参形条款执行：包数据逐字节随迁（四件 cmp IDENTICAL）、材料同源、参照时间同值、绝对路径运行。活体双跑三场景（net-core 与 net-parking 与 attribution-core）围堰 selector 与引擎 attractor route 输出 cmp 全 IDENTICAL、退出码对齐，证据 sih/event/plan/autoflow2-solo-materials/route-double-run-cmp.log。金向量八场景 sha256 冻结于 src/attractor/fixtures/route/golden/golden-manifest.json。实跑探针：attractor route parking 包对真实在泊材料十六件全主线零告警退出码零。

## GOV-002 判据对表（状态表述，结算归人节点）

- 判据二「路择的谓词件经可插拔机制融回判定器模块」：谓词机与两包纯数据与 route 子命令落 src/attractor/，可插拔即规则包形态承载，本批状态达成
- 判据三「按轮判定的截流谓词族融回三问模块」：截流谓词装配位落 src/ask3repeater/intercept.rs，判定语义经谓词机单一实现，本批状态达成
- 判据五「引擎 task-packages 内 facet 过程件归零」：f-anchors-x11-t6d.md 归档入 sih/event/plan/ 原位删除，task-packages 目录零 facet 过程件，本批状态达成
- 判据一与判据四状态：判据一（任务包 007 至 013 执行完毕并关闭且六组件落地 src）与判据四（facet 双模并存）在此前批次已承载，本批零触
- 五条全部就位的结算宣称不在本批：结算归人节点，本完成档只表述状态
