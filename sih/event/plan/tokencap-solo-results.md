# tokencap-solo：句读令牌捕获清三笔期票批结果档

> 承任务包 sih/state/plan/tokencap-solo.md 即 2026-08-30 链事件 0c5792f6 加修正意图 4c8847fa 与 f5346728 与 34951f64
> 队形：单线形 solo
> 日期：2026-08-30

## 概览 {#overview}

- 三笔期票两讫一修：impl 取名经引擎 text 形与直接子区间合成、use 边经级联自走树即 21 件 rs 真实依赖图在案、运行期边账四计数器入报告视图::[deliver](#deliver)
- 批内发现并修复前批缺陷：pk027expr 上线的 rust 包 lint 真退出码一即文法引用八词法名缺定义，前批验收被管道掩码，本批机械对账修复并加守卫::[defect](#defect)
- 修正环四轮意图在链：范围面发现即收约改包重开、机制两经机械对账修正即 SHR 与 v1 泛型闭合不可两立::[loop](#loop)

## 一、交付实录 {#deliver}

引擎面两处即 name_from 增 text 形与 join_rules 增 direct，langpack 校验同步收形，空腹零语言知识保持。rust 包三件修订三：impl_item 取名改 ty_bounds 加 ty 直接子区间以 for 连接，泛型参规则不进名即 name None 实锤修复，reading.rs 实测即 std::fmt::Display for ReadingError。级联 0.4.0：scan_code 在既有解析程内自走 use_item 取 use_tree 结构，链式取最深基段与组成员各自取段与别名取原名与通配只记不配，符号表取八标识类条目名排除 impl，末段唯一解析建边与文档边同表；真实狗粮即引擎 src 全量 21 件 rs 连出依赖图、bin/scribe.rs 十上游在册、use_ambiguous 四与 use_unmapped 二十三与 use_wildcard 一如实入注记；check 报告内嵌 runtime_ledger 四计数器，零写保持即册与链双跑零动。引擎侧边册投影 sih-engine/doc/CASCADE.json 以 0.4.0 重建即 71 边点，租约缺省路径由虚转实。

## 二、前批缺陷披露 {#defect}

pk027expr 批上线的 rust 包 lint 真退出码一即文法引用 BANGEQ、DAMP、DBANG、DBAR、GE、LE、SHL、SHR 八词法名未定义，v2 表达式层对比较与逻辑与移位运算符由缺名而不可达即树退配平汤。根因两件：其一前批验收命令带管道即退出码被 tail 掩为绿，其二测试面无 rust 包 lint 守卫。本批修复即四笔误换既有名（NE 与 OR2 与 AND2 复用）与 LE 加 GE 加 SHL 补表与 SHR 走文法 GT 对，lint 真零收口，新测试件钉住 lint 绿与包校验负例。归因机械不推人，防复发即裸退出码判定与守卫测试双落。

## 三、修正环实录 {#loop}

四轮意图在链。一轮 0c5792f6 立原始目标；开工首跑发现前批缺陷超范围面即收约会话一改包重开，二轮 4c8847fa 并入修复；三轮 f5346728 机械对账修正机制即八名四笔误四真缺；四轮 34951f64 即 SHR 补表与 v1 泛型闭合不可两立经对表机械证，改文法 GT 对序列。教训两条：验收看裸退出码不看管道回显；范围面在开工首跑即验，零提交时收约代价最小。

## 四、验收判定 {#acceptance}

- F-1 引擎取名面：过。text 形四变体与 direct 直测，markdown 与 json 包零动
- F-2 impl 取名：过。泛型参不进名即 Wrapper<T> for Queue<T> 断言，过滤对表逐字节不变，向量重冻全过
- F-3 use 边：过。九测即唯一解析与链式与组与别名与通配与自引与歧义注记与双跑一致
- F-4 运行期边账：过。四计数器内嵌、复演确定、册与语料双跑零动
- F-5 词法缺口：过。lint 真零、比较与逻辑与移位树成节点、泛型嵌套闭合不变
- F-6 收口：过。契约修订二与五、AGENTS 两行、认证入链、结算收约

## 五、关联 {#related}

- 测试：句读 79 测全绿含新增 15、级联 28 测全绿含新增 10
- 材料：scribe/reports 2026-08-30-ask3-tokencap 四轮与 elicit 信号件与管线报告
- 关联：pk027expr 与 pk033casc 两前批、SPEC-009、两 CONTRACT、graph 旧投影 sih/state/graph/CASCADE.json 仍留 0.1.0 即历史位未清如实记
