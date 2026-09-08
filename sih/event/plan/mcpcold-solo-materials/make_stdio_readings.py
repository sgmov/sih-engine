#!/usr/bin/env python3
"""mcpcold-solo 批五工具读数实录生成器：经 MCP stdio 面逐工具调用并导出读数。

承 make_smoke_excerpts.py 先例形（mcpserv-solo）。零写入：只读调用，出参落本材料目录。
错误探针两笔：chain_query 与 chain_verify 各喂一非法 date，取错误载荷四字段实证。
"""
import json
import os
from pathlib import Path

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

ROOT = Path("/Users/moc/workspaces/SiHankor")
MCPLINE = ROOT / "sih-tools" / "mcpline"
OUT = Path(__file__).resolve().parent / "stdio-five-tool-readings.json"

env = {"SIH_ROOT": str(ROOT), "PATH": os.environ.get("PATH", "/usr/bin:/bin")}
params = StdioServerParameters(
    command="/opt/homebrew/bin/uv",
    args=["run", "--project", str(MCPLINE), "python", "-m", "mcpline"],
    env=env,
)


async def main() -> None:
    import asyncio

    async with stdio_client(params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            listing = await session.list_tools()
            tools = [
                {
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.inputSchema,
                }
                for t in listing.tools
            ]
            readings = {"tool_count": len(tools), "tools": tools, "calls": {}}

            async def call(name: str, args: dict) -> dict:
                res = await session.call_tool(name, args)
                texts = [b.text for b in res.content if hasattr(b, "text")]
                payload = None
                for t in texts:
                    try:
                        payload = json.loads(t)
                        break
                    except json.JSONDecodeError:
                        continue
                return {
                    "is_error": bool(res.isError),
                    "payload": payload,
                    "raw_text_head": (texts[0][:400] if texts and payload is None else None),
                }

            probe = {"chain_query": {"date": "2026-9-9"}, "chain_verify": {"date": "not-a-date"}}
            plan = {
                "chain_query": {"date": "2026-09-09"},
                "chain_verify": {"date": "2026-09-09"},
                "critsweep": {"date": "2026-09-09"},
                "heartbeat": {},
                "locks_read": {},
            }
            for name, args in plan.items():
                readings["calls"][name] = await call(name, args)
            readings["error_probes"] = {
                name: await call(name, args) for name, args in probe.items()
            }

            OUT.write_text(json.dumps(readings, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
            print(f"written {OUT}")
            print("tool_count:", len(tools))
            for name in plan:
                p = readings["calls"][name].get("payload")
                if isinstance(p, dict):
                    head = {k: p[k] for k in list(p)[:4]}
                    print(name, "->", json.dumps(head, ensure_ascii=False)[:220])
                else:
                    print(name, "->", str(p)[:120])
            for name, r in readings["error_probes"].items():
                pl = r.get("payload") or {}
                print("probe", name, "fields:", sorted(pl.keys()))


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())
