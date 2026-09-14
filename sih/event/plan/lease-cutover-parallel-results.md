# lease-cutover-parallel 批结果档

## 结算要点

- 批名 lease-cutover-parallel（并联形；stem 甲表认领 cutover:new，概念锚 zh 收尾切换，code 形显式申报无承，终裁登记候人节点）
- 会话 cbebbeddc0766c7d 双仓，六锁，意图笔在链；正典 SPEC-025 收尾
- 令源用户 2026-09-14 goal 收尾令；主线插件桥实装加切换簇并行

## 插件桥实装（A4 转正）

- StdioPluginBridge：manifest.command 起插件子进程，行分隔 JSON-RPC 2.0 stdio 协议（请求 call 加 params，响应 result 加 error 映射 Execution）；会话表 Arc 共享 Clone 形；spawn 加 send_request 加 recv 哨加 shutdown 四面全实装。
- PluginToolProvider：manifest.tools 逐工具包装 ToolProvider 注册 registry，tools/list 显形可调用；description 承 PluginToolEntry description 缺省教学形。
- shutdown 健壮形：stdin EOF 候 3 秒超时 kill 兜底（外部插件可能不优雅退）。
- 实装期抓修三笔如实登记：sh 内建 printf 管道全缓冲不冲刷（fixture 改外部 printf）、测试断言 ToolEntry 对象形、shutdown wait 死等兜底补齐。
- 测试：tests/mergeall_final_bridge.rs 五测全绿（静态 manifest 装载加全链往返显形调用加未启动 NotFound 加 recv 哨加 shutdown 后不在场）。

## 切换面（主线执行，切换簇代理死寂无产出如实申报）

- 切换前 spawn 位 39（AGENTS.md 加 15 个 skill 壳），已切引擎位 36，裁留围堰位 2（facet/measure 用量冻结数据区工具与 latex-helper compute 子命令 A6 面），AGENTS.md cd sih-tools 残留 0。
- 实跑验证：引擎 gauge record rc0 三维落链；首战批同参双跑对表在档。
- 围堰退役宣告落 AGENTS.md 工具层静态审计节头：sih-tools 生产工具转冻结只读兼容态，物理退役候终裁。
- AGENTS.md 投影化收口：工具层节加会话例行节加 MCP 节调用位引擎化；MCP 节 sih server 入口引擎位核验坐实。
- 清单档 cutover-invocations.md 随批入 materials。

## 验收对表

- 全族回归：lease 11 与接线四测与 registry 两测与腿一至腿六抽验七套件全绿；桥五测绿。
- 落差申报：协议为自定行分隔 JSON-RPC 形非 MCP 全协议（插件接入最小面，MCP 全协议桥候后）；tool 重名跨插件拒注册承 registry 同名拒。

## 偏差

- 切换簇偏差：切换簇子代理死寂无产出（无输出文件），切换面由主线亲做，队形退化如实申报，处置先例承 SPEC-024 并联批跨簇互挡申报形。
- 桥协议偏差：协议为自定行分隔 JSON-RPC 最小面非 MCP 全协议，SPEC-025 插件节定形，全协议桥候后。
- 测试环境偏差：假插件 sh 内建 printf 管道全缓冲卡 read_line，fixture 改外部 printf 加 shutdown 3 秒 kill 兜底，实装期修正三笔如实登记，裁决先例承 SPEC-025 A9 零网络红线。

## 明确不做与后件

- 围堰源码零改动（退役为宣告态）；物理退役与真实插件生态样例仓候终裁。
