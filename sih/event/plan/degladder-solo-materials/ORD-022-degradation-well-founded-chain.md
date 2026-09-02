# ORD-022 降级良基链与降级矩阵

状态：草稿，degladder-solo 波产出，资产回锚载体条目。

## 定义 {#definition}

降级良基链 (Well-founded Degradation Chain) 与降级矩阵 (Degradation Matrix) 给容错降级一个序结构承载：服务级集 $L$ 上取全序 $\prec$，降级算子 $\delta: L \to L$ 将当前级映到严格更低级，恢复不自动发生，须经独立重校验谓词 $\rho$ 通过后显式回升。降级矩阵是逐失败场景到降级算子的全函数映射，见公式 $\ref{eq:ord022-matrix}$。

<a id="eq:ord022-matrix"></a>
$$M: F \to (L \to L), \quad M(f)(\ell) \prec \ell \quad \text{for all } f \in F,\ \ell \ne \ell_{\min}$$

定理一，降级终止。$(L,\prec)$ 良基时，任何降级序列 $\ell \succ \delta(\ell) \succ \delta^2(\ell) \succ \cdots$ 有限步终止。

证明。无限严格下降链与良基性矛盾，承 ORD-016 的良基定义；有限服务级集自动良基，无限集取良基排序即可。

定理二，不震荡。回升必经重校验谓词 $\rho$ 时，降升无限交替被排除；无 $\rho$ 的自动回升破坏严格下降语义，属实现违例。

证明。降升交替中的每个降步都是 $\delta$ 的一次施加，交替无限即无限严格下降链，与定理一矛盾；无 $\rho$ 回升后再次降级构成同链，故排除震荡的唯一充分条件是 $\rho$ 门控回升。

定理三，矩阵完备性。$M$ 为全函数时任意失败场景都有预设降级路径，无场景落入未定义行为；$M$ 定义域对失败场景集的覆盖审计即完备性检查。

证明。全函数按定义对 $F$ 中每个元素有值，覆盖审计即检验 $M$ 的定义域等于当前失败场景清单，是确定性比价。

## 公理条件 {#axioms}

- 公理一，服务级全序：$(L,\prec)$ 全序，结构承 ORD-001 与 ORD-020
- 公理二，降级严格下降：$\delta(\ell) \prec \ell$，同级保持与升级都不是降级
- 公理三，恢复独立重校验：$\rho$ 以新证据为条件，与降级算子分离
- 公理四，风险下限：最低服务级 $\ell_{\min}$ 不可再降，配置不得低于

## 哲学桥接 {#philosophy-bridge}

- 承接命题：法三知止与元层治强，准入依据为 anchorsort 分拣面裁决组与得一裁 m-degladder 终签
- 哲学命题：原文「> 法三：知止。知道不做什么，比知道做什么更难。」，见 06-on-canon 正本 L65，降级即知道停在哪一级的序形态，最低服务级是知止的量化
- 形式化：原文「- 治强：禁止绕过鉴直接 Ratify，治理主体不得为自己谋求有利变更」，见 09-on-arche 正本 L322，恢复绕过重校验即为自己谋求回升，治强的硬约束形态即公理三与公理四
- 本条只给降级的序结构形态，不立治理结论

## 应用 {#application}

- 引擎降级策略与多档降配：外仓清单 AiCoder 与 SpeCoder 的降级矩阵与双 Profile 裁剪是本条的工程实例，出处见 extinv 清单件
- 租约让步位：Wait-Die 让步即服务级临时下降，重试须重新评估即 $\rho$ 门控
- 验证管道依赖失效回退：ast-grep 与库与版本控制工具的分级回退是 $M$ 的实例
- 边界：降级触发时点的检测归 PROB-013，本条只管触发后的序行为

## 与其他概念的关系 {#relations}

- ORD-016 良基关系与倒推终止：定理一的终止性来源
- ORD-020 全序资源分配与死锁自由：服务级全序与让步策略的序基础
- ORD-021 依赖 DAG 与拓扑序：降级路径与恢复路径的依赖面
- PROB-013 平稳性与变点检测：降级触发时点的检测面

## 历史脉络 {#history}

- 容错计算的优雅降级 (Graceful Degradation) 传统，Avizienis 等给出可靠性的分类正本
- 2003 年 Nygard 的断路器模式将降级与恢复门控确立为工程标准形态

## 工程注意事项 {#engineering-notes}

1. 恢复必须重校验，静默自动回升即震荡违例
2. 降级矩阵的完备性随失败场景新增而重审计，新增场景未入矩阵即未定义行为
3. 最低服务级是风险下限，配置低于它属违例，检查归确定性谓词
4. 降级触发的误报率与延迟归 PROB-013 的检测面，本条不重复

## 参考文献 {#references}

- Avizienis, A., Laprie, J.-C., Randell, B. & Landwehr, C. (2004). Basic Concepts and Taxonomy of Dependable and Secure Computing. IEEE Transactions on Dependable and Secure Computing, 1(1), 11-33
- Nygard, M. (2003). Release It! Pragmatic Bookshelf, ch. 5
