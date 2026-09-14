# lease-cutover-parallel 任务包（并联形）

## 形声明与令源

- 队形：并联形（parallel），主线亲写插件桥实装（引擎 Rust 面收尾），切换簇子代理并行执行生产调用面切换与验证。
- 令源：用户 2026-09-14 goal 收尾令「进程外插件桥实装与切换批」；正典 SPEC-025。

## 目标

1. **插件桥实装转正**（主线）：ExternalPluginBridge 四面（spawn 加 send_request 加 recv 加 shutdown）实装——外部插件进程按 manifest 起停，行分隔 JSON-RPC 2.0 stdio 协议收发，PluginProvider 包装外部工具注册进 registry（tools/list 显形可调用），A4 缺席申报转正。测试用 sh fixture 假插件全链验证。
2. **切换面**（切换簇）：生产调用面切引擎 bin 位——skill 壳与 AGENTS.md 各节（attnanchor 加 critsweep 加 gauge 加 formatter 加 nomenclator 等全部 sih-tools spawn 位），逐一替换逐一实跑验证退出码；围堰退役宣告（sih-tools 生产工具转冻结只读兼容态，物理退役归终裁）。
3. **AGENTS.md 全面投影化收口**：工具层节加会话例行节加文件索引节的调用位引擎化收口，级联工作区文件随批申报。

## 验收判据

- F1 桥四面实装：manifest 假插件全链（注册加 tools/list 显形加 call 往返加 shutdown）测试绿。
- F2 切换面：每调用位替换后实跑退出码验证在档；围堰退役宣告落 AGENTS.md。
- F3 全族回归不破；registry 测试族绿。
- F4 收约归并重建后 sihmcp 实测（tools/list 指纹对表）。

## 明确不做

- 围堰源码零改动（退役为宣告态，物理删除候终裁）。
- 插件生态件（真实外部插件样例仓）候后。

## 改动文件清单

sih-engine/src/tools_registry.rs（桥实装）、tests/mergeall_final_bridge.rs（新）、本包、结果档与 materials、当日 trail；工作区级联（AGENTS.md 与 .agents/skills/ 各壳，批外文件申报制）。
