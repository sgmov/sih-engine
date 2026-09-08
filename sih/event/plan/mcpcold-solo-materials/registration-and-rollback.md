# sih server 注册形与回滚法（mcpcold-solo 批材料）

## 注册形

- 目标文件：工作区 ZCode 配置 `.zcode/config.json`（工作区根下，不属任何 git 仓）
- 变更性质：只增不改——`mcp.servers` 下净增 `sih` 一键，退役 `sihankor` 条目与 `hooks` 节原样保留（before/after 双影像程序比对实证：sihankor identical True、hooks identical True、added {'sih'}、removed set()）
- 前影像：`zcode-config-before-image.json`（sha256 `15c2f4253df6ebd392c09149437ee3149a2915d4d68fd1387d9ddf11ad2db603`）
- 后影像：`zcode-config-after-image.json`（sha256 `6f2b7e5a98d979b4c404216f753f4b45bdf6a1859862dbaf73515ea3ce62d9ce`）

## 增补条目全文

```json
"sih": {
  "type": "stdio",
  "command": "/opt/homebrew/bin/uv",
  "args": ["run", "--project", "/Users/moc/workspaces/SiHankor/sih-tools/mcpline", "python", "-m", "mcpline"],
  "env": { "SIH_ROOT": "/Users/moc/workspaces/SiHankor" },
  "enabled": true,
  "timeoutMs": 60000
}
```

字段依据：ZCode MCP 配置 schema stdio 形必填 `command` 可选 `args`/`cwd`/`env`/`enabled`/`timeoutMs`，未知键整条丢弃故零冗余键；`command` 取绝对路径避 PATH 依赖；`SIH_ROOT` 显式传工作区根（服务器根解析的环境变量优先位）；`timeoutMs` 60000 宽于缺省 30000 以容 uv 首启。

## 生效证据

- 装载：冷 agent 进程（HOME 暂存重定）启动日志 `mcpServerCount: 2`（sihankor disabled + sih enabled 俱在册）
- 调用：冷 agent 经 `mcp__sih__chain_query` 与 `mcp__sih__chain_verify` 完成被治理交互（cold-transcript-extract.json）；批方暖冒烟 `mcp__sih__heartbeat` 同面可调（warm-smoke-response.json）
- 会话内热载：本批执行会话自身不热载（进程启动时读配置），冷 agent 系新起进程故配置即时生效

## 回滚法

删除 `.zcode/config.json` 中 `mcp.servers.sih` 一键（或以 `zcode-config-before-image.json` 整文件还原），保留其余各键原样；回滚后本批注册即解除，服务器文件与链面证据不受影响。回滚属配置单键删除，零依赖零副作用。
