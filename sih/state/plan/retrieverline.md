# retrieverline：retriever 档案面扩容与引擎温故语义通道接线立项

> 令源：2026-09-17 pkexits3 批裁定即用户令「裁，多子代理并行」，pk-039 与 pk-056 出泊 promoted 本包承载（立项开工令形承 pk-045 先例）
> 范式：双件并批立项包（pk-056 入泊候选第三形态即与 pk-039 档案面扩容并批承载），出泊事件在链即 pk-039 与 pk-056 两笔
> stem 认领：retrieverline，甲表三件即 zh 检索线引擎化收口立项、code 无承、派生 retrieverline:new

## 一、问题陈述 {#problem}

pk-039 面：retriever 档案面扩容到资产层即 SETSP 名录与旧仓 doc 索引入检索索引（2026-09-02 入泊，出泊条件即用户裁扩容立项与否，2026-09-17 裁立项本包承接）。pk-056 面：引擎温故即 sih-engine/src/retriever 现有词面通道经寻址只读桥即 locator_bridge 无语义通道，确定性统计向量语义召回落在工具壳 wikirecall（pk-037 用户裁路线即确定性统计向量不做向量库，pk037impl 实装 A/B 门三判据全过，pk-050 promoted 后缺省即语义 K=3 附 --word 回退），检索族底座即句读与寻址已交付缺引擎位接线，即检索族引擎化收口未发生。双件同域俱是检索面：索引面与通道面分批则互为等待并批一次收口。

## 二、关键设计 {#design}

- 档案面扩容：SETSP 名录与旧仓 sihankor/doc 目录（decision 与 design 与 draft 与 governance 与 knowledge 与 proposal 与 research 与 spec 各面）索引入 retriever 检索索引即 archives 面扩容；检索可见面非引用权威面，入索引供召回不改变引用纪律（沉默参考与旧仓只作盘点源不作引用源承工程基线与资产回锚登记先例），索引件标注来源域零伪装正典。
- 语义通道接线：引擎温故增语义通道，算法对齐 wikirecall 语义召回承 pk-037 既裁即确定性统计向量不做向量库，缺省语义 K=3，词面通道降显式回退旗标位承 pk-050 切换形。
- 行为对表：引擎语义通道与工具壳 wikirecall 同参双跑判词一致承 mcpdual-parallel 先例形，对表材料随批入档。
- 消费面零破坏：MCP retriever_recall 面承接 CLI 形不变（--root 显式传，cwd 不承根），四轴语义零变更。

## 三、工作清单 {#work}

- [ ] rl-01：资产面盘点即 SETSP 名录与旧仓 sihankor/doc 目录逐件清点建册
- [ ] rl-02：档案面扩容即盘点面入 retriever 检索索引（archives 面），来源域标注随件
- [ ] rl-03：引擎温故语义通道实装对齐 wikirecall 语义算法（确定性统计向量，缺省 K=3，词面留显式回退位）
- [ ] rl-04：同参双跑对表即引擎件与工具壳判词一致加测试族
- [ ] rl-05：settle 加 close 加结果档

## 四、验收 {#acceptance}

SETSP 名录与旧仓 doc 索引可经 retriever recall 四轴检索命中且来源域标注在；引擎语义通道与 wikirecall 同参双跑判词一致在档；词面回退位在役；MCP retriever_recall 行为零破坏；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/src/retriever/——语义通道接线与 archives 面扩容
- sih-engine/sih/state/plan/retrieverline.md——本任务包
