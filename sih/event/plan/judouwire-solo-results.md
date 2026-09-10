# judouwire-solo 批结果档

> 承接：judouwire-solo 任务包、意图 5b1f6966、session 6067a4e51c42ca8f
> 令源：用户 2026-09-11 令「修复」承 tree-sitter 重接债链上判因
> 队形：solo 单线，主线亲写零子代理
> 日期：2026-09-11

## 一、总判 {#verdict}

五件俱落地俱测试：locator code 载体切句读完成即 tree-sitter 与 tree-sitter-rust 两依赖撤除换 parser path source 接线；崩溃语料四件端到端真重接即旧载体段崩溃语料今以 73 条零 parse_errors 全链过；金向量再冻结即旧 expected 入材料留痕新 expected 31 条冻结双跑一致；零依赖判据复用消费仓即 uv tree 零分布加源树零引用双机械证明；温故 recall 两轴活体回归退出码零。DEC-016 三占位销账与 DEC-013 占位承载条款两修订过管线三步。主树复跑 locator 68 测全绿。

## 二、先红留痕 {#red}

- 活体复现一：判因阶段主线进程对 sih-engine/src 五件真语料跑旧 parse_code 即 SIGSEGV(139) 整进程炸红
- 活体复现二：全 58 件 rust 真语料逐文件隔离探测，崩 21 件即 36%（清单与原始输出入 ab-old-treesitter-crash-probe.json）
- 活体复现三：本批新测 test_carrier_judou_wire.py 对旧引擎跑即 pytest 进程 139（red-treesitter-segv.txt 在材料）

## 三、绿态与 F 判据 {#green}

- F-1 真重接：崩溃语料四件 parse_code 零崩各得条目（append 14 与 mod 24 与 query 16 与 validate 19），build 管线级 4 文件 73 条零 parse_errors（test_carrier_judou_wire 8 测全绿）
- F-2 切流成立：locator 源树 tree_sitter 零命中（测试断言）加 uv tree 零分布（zero-dep-uvtree.txt）
- F-3 确定性：vectors 冻结后双跑逐字节一致，10/10 向量过
- F-4 金向量纪律：旧 expected 留痕 vectors-expected-pre-judouwire.ndjson，新 expected 31 条冻结（原 33 条，差 2 条即 const/static 初始化器形缺口，见残余节）
- F-5 温故轴回归：recall --word 句读与 --topic 温故 两轴退出码零命中在档
- F-6 病灶销账：DEC-016 v1.3 三占位下落逐个在档，DEC-013 v1.6 占位承载条款在档，两件管线三步零违例（016 首过检词红一笔即「落账」惰性词，改「在案」复过，红改绿在案）
- F-7 只读面：parser 仓 git status parser 面空即零改动，句读 rust 包与引擎 Rust 组件内容零触碰

主树复跑于收约归并后执行（F-1 主树真跑）：locator 68 测全绿。

## 四、漂移实测与残余 {#drift}

双活 37 件对表（ab-drift-report.json）：旧 311 条对新 301 条，逐字节全等 294，行位漂 6，text 漂 1，缺失 10，新独有 7。根因两句：

1. impl 名语义差：句读取 ty_bounds for ty 全形（如 "T for Foo"），旧绑定单取 type 位（"Foo"）——实现批已登记待深阶，本批随切流兑现为现形并钉入测试
2. rust 包 v1 初始化器形缺口：const/static 带 EQ 初始化器树级失败（expr_chunk 组合缺口），条目缺席——金向量 33 改 31 即此两形；归 pk027expr 后继修文法，本批零文法改动，缺口钉入 test_initializer_const_static_gap_registered 防静默变化

stable_id 判词：294 全等条目标识零漂；漂移条目标识随 text/name 如实换新；现役生产索引（memory 包）走 markdown 与 json 载体零涉 code 载体，漂移面收敛于 locator 自身金向量与语料。

## 五、改道与协调 {#coord}

- 无改道。在飞两批协调：wengumcp-parallel（M5 温故）面与本批零相交，本批 recall 回归跑主树现形；mcpmanual-solo 于本批中途收约让出锁面，packs/core 新词登记（judouwire 与切流与载体）候批面如实申报
- zsh 坑位一笔：zsh 不做未引用变量词分裂，批量取锁脚本 rc=2 全数伪造，改内联逐发后八锁全落——工具无恙，调用姿势坑

## 六、段结算 {#settle}

双仓段结算即 tools 副本载 locator 切流全量与新金向量与新测试与 CALL-LOG 追加行，engine 副本载任务包与结果档与材料五件与 DEC-016/DEC-013 两修订。链上意图 5b1f6966，管线认证见链。收约后回锚重跑完工报告回显五行。
