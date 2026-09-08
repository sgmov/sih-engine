#!/usr/bin/env python3
"""mcpserv-solo 批材料生成器：smoke 出参摘录与 TDD 红绿时间线转录。

smoke 摘录经真实 stdio 客户端逐工具调用取实参（完工回报摘录源）。
"""
import asyncio
import datetime
import json
import os
from pathlib import Path

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

MCPLINE_DIR = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-tools/mcpserv-solo/mcpline")
MAT = Path("/Users/moc/workspaces/SiHankor/worktrees/sih-engine/mcpserv-solo/sih/event/plan/mcpserv-solo-materials")

TOOLS = [
    ("chain_query", {"date": datetime.date.today().isoformat()}),
    ("chain_verify", {"date": datetime.date.today().isoformat()}),
    ("critsweep", {"date": datetime.date.today().isoformat()}),
    ("heartbeat", {}),
    ("locks_read", {}),
]


async def main():
    env = {k: v for k, v in os.environ.items() if k not in ("PYTHONHOME", "PYTHONPATH", "VIRTUAL_ENV")}
    env["SIH_ROOT"] = "/Users/moc/workspaces/SiHankor"
    params = StdioServerParameters(
        command="uv",
        args=["run", "--project", str(MCPLINE_DIR), "python", "-m", "mcpline"],
        env=env,
    )
    excerpts = {}
    async with stdio_client(params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            listing = await session.list_tools()
            excerpts["_tools_listed"] = sorted(t.name for t in listing.tools)
            for name, args in TOOLS:
                res = await session.call_tool(name, args)
                text = "".join(c.text for c in res.content if hasattr(c, "text"))
                obj = json.loads(text)
                if name == "critsweep":
                    slim = {k: obj[k] for k in ("at", "degraded") if k in obj}
                    slim["criteria_ids"] = [c["id"] for c in obj["criteria"]]
                    slim["parking"] = obj.get("parking")
                    slim["inflight"] = obj.get("inflight")
                    obj = slim
                elif name == "chain_query":
                    obj["events"] = obj["events"][:2] + ["…截断，全量见链"]
                elif name == "locks_read":
                    obj["held_locks"] = obj["held_locks"][:3] + ["…截断"]
                excerpts[name] = obj
    out = MAT / "smoke-excerpts.json"
    out.write_text(json.dumps(excerpts, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"written {out}")
    print("tools:", excerpts["_tools_listed"])


if __name__ == "__main__":
    asyncio.run(main())
