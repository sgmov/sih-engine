# P-3 HTTP 面冒烟记录（sihmcp-solo 段3/段4）

- 临时端口 127.0.0.1:18765 起 `SIH_TRANSPORT=http SIH_HTTP_BIND=127.0.0.1:18765 target/debug/sihmcp`。
- 台面三页 curl 200：`/tokens`（令牌管理台）、`/tokens/open`（开域表单）、`/manual`（AI 说明书）。
- MCP Python 客户端（mcp.client.streamable_http）过 `/mcp` 握手：serverInfo sihmcp 0.9.0；
  tools/list 19 具（external 分级形，与 stdio 面同表）；locks_read 见真锁 12；
  chain_verify(2026-09-11) valid 109 笔。
- Bearer 闸：写面十具逐调用新读登记册（甲簇 write_gate_verdict 九测：缺头 401、
  不在册 401、停行 401、readonly 403、custom 403、domain_write 放行）。
- 记录时点：2026-09-11，sihmcp-solo 会话 f85c53836d0eb3a5。
