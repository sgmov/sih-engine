#!/usr/bin/env python3
"""parksi-solo ask3 记录生成器：三锚引文程序切片逐字节子串，禁手打。"""
import json
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
OUT = ROOT / "sih-tools/scribe/reports/2026-09-09-ask3-parksi-solo-record.json"


def slice_line(rel: str, lineno: int) -> str:
    lines = (ROOT / rel).read_text(encoding="utf-8").splitlines()
    return lines[lineno - 1]


q1 = slice_line("sih-philosophy/emanation/proodos/08-on-settle.md", 13)
q2 = slice_line("sih-philosophy/emanation/proodos/08-on-settle.md", 110)
q3 = slice_line("sih-philosophy/emanation/proodos/07-on-assay.md", 55)

record = {
    "session_id": "sess-zcode-260909-parksi",
    "raw_input": "用户 2026-09-09 原话「司衡引擎的mcp做好了再上，到时候是那边的agent接入，有问题我会回司衡窗口报，你现在不要管SiInfer」即 SiInfer 彩排泊置裁定：pk-080 进泊件照任务包第二节定稿入泊引擎线泊界、PARKING-v1.md 只增册行、scribe park 停泊笔落链、走完批机械链收约；solo 微批独立立约独立收约，与在飞 mcpsec、specfix、toolhyg、release09 四批共用今日链走租约排队；纪律要点：SiInfer 工作区零访问零读取零写入，只进泊不预设出泊，在泊件零触碰，册行只增不改他行，主树零直写（任务包与提示词件两件预落除外随批提交），链文件只经引擎 scribe 写位，禁管道掩退出码，在盘遗留无主件不豁免不代清",
    "round": 1,
    "intent_contract": {
        "goal": "parksi-solo 泊置批：新立 sih-engine/sih/state/parking/materials/pk-080.json 进泊件逐字承载任务包第二节定稿即 entry_id pk-080、title SiInfer 彩排泊置——MCP 线毕候令，接入形为彼侧 agent、context 令源照录用户原话加前情三行加红线、gate.trigger 即 mcpline 线批三 β 实装收约结算件落 sih-engine/sih/event/plan/ 加 gate.declared_at 2026-09-09、exit_condition 人节点裁彩排开工或裁搁置闭项出泊且 gate 触发亦不出泊、related pk-077、ttl_days 90、parking.entered_at 2026-09-09；PARKING-v1.md 在泊名录节只增册行一行行形照地面现有行实况且册行载入泊事件哈希；scribe park 停泊笔落链取 parking_entered 事件哈希；化格核阅检词管线与内容哈希清单认证；机械链全序 BATCH-FACE verbatim 每条立即取退出码失败即停；写入仅 allow",
        "exclusions": [
            "SiInfer 工作区 /Users/moc/workspaces/SiInfer 零访问零读取零写入：泊置是司衡侧裁定档，踏勘已止（红线，任务包第三节）",
            "出泊唯人节点：本批只进泊不预设彩排开工裁决，gate 触发亦不出泊（红线，任务包第二节）",
            "在泊件全量零触碰即 pk-077 只读关联（红线）",
            "PARKING-v1.md 只增册行不改他行（红线，任务包第三节）",
            "主树零直写即待提交件经工地 settle 通道（任务包与提示词件两件预落除外随批提交），链文件只经引擎 scribe 写位（红线，工地铁律）",
            "守卫在位严禁 plain git commit 直提，收约补笔走 bypass 登记（红线）",
            "与在飞四批共用今日链冲突走 lease wait-turn 禁绕行禁 preempt（红线）",
            "在盘遗留无主件不豁免不代清（红线）",
        ],
        "output_format": "ask3 记录与验证件、正身件、租约开收约与锁实录、当日链 intent 与一笔 parking_entered 与认证事件号、pk-080 册行与进泊件路径、化格核阅检词管线读数、内容哈希清单、认证清单、越线与误差申报、收口读数、parksi-solo-results.md 结果档、settle 提交号、链 verify 读数、reconcile 读数、完工报告含链笔哈希与双仓 commit 与 scribe verify 全文与 reconcile 增量",
        "injected_constraints": [
            "工程基线第一条确定性程序是治理唯一执行者：停泊事件经引擎 scribe 写位落链",
            "工程基线第二第三条人类注意力只投向异常信号：进泊即记账机器与 agent 皆可做，出泊唯人节点",
            "PRO-07 鉴只列事实：泊材料 context 零修饰不列建议",
            "PRO-08 应而不藏：含对己不利读数逐件登记",
            "泊界四字组停有痕：每笔进泊都是链上事件，投影与链不符以链为准",
        ],
    },
    "domain_contract": {
        "target_domain": {
            "scope": "sih-engine/sih/state/parking/materials/pk-080.json（新）与 sih-engine/doc/governance/PARKING-v1.md（只增册行）与 sih-engine/sih/state/plan/parksi-solo.md 与 parksi-solo-prompt.md 与 sih-engine/sih/event/plan/parksi-solo-results.md 与 parksi-solo-materials/ 与 sih-engine/sih/event/trail/2026-09-09.ndjson（经引擎 scribe）与 sih-tools/scribe/reports/ 与 sih-tools/identity/reports/ 与 sih-tools/proposition/DES/m-parksi-1/ 与 worktrees 双仓 parksi-solo 工地",
            "max_depth": 1,
        },
        "support_domain": {
            "scope": "sih-engine/sih/state/parking/materials/pk-077.json 与 pk-078.json 与 pk-079.json（键形与 gate 正形对形只读）与 sih-engine/sih/event/plan/mcppark-solo-results.md 与 s2park-solo-results.md（进泊批先例只读）与 sih-engine/doc/governance/PARKING-v1.md（行形地面实况只读）与 sih-philosophy/emanation/proodos/07-on-assay.md 与 08-on-settle.md 引文原文只读切片",
            "max_depth": 1,
        },
    },
    "anchors": [
        {
            "anchor_seq": 1,
            "text": "SiInfer 彩排是应变之未形的微兆：MCP 线批三 β 实装在飞即趋势已显现，接入形未裁即方案在酝酿，彩排数据成熟度低即用户明令彼侧 agent 接入前司衡侧零动作，先入泊界停靠候令不强行开工",
            "inquiry_stage": "first",
            "domain_tag": "target",
            "depth": 0,
            "rationale": "应几的姿态是预判介入而非即答：泊界停靠使彩排裁定有登记位，开工裁决候 MCP 线毕后人节点",
            "confidence": 0.95,
            "philosophy_ref": {
                "source": "sih-philosophy/emanation/proodos/08-on-settle.md",
                "quote": q1,
            },
            "evidence": "sih-philosophy/emanation/proodos/08-on-settle.md:13",
        },
        {
            "anchor_seq": 2,
            "text": "彩排裁定与踏勘前情如实登记：令源原话与前情三行与红线与触发器与出泊条件逐项入泊材料，不藏在会话讨论里蒸发，泊界是未决事项唯一合法停靠地",
            "inquiry_stage": "first",
            "domain_tag": "target",
            "depth": 0,
            "rationale": "藏缺即藏失效：裁定若无登记位即随会话蒸发，治理诚信要求其入档",
            "confidence": 0.95,
            "philosophy_ref": {
                "source": "sih-philosophy/emanation/proodos/08-on-settle.md",
                "quote": q2,
            },
            "evidence": "sih-philosophy/emanation/proodos/08-on-settle.md:110",
        },
        {
            "anchor_seq": 3,
            "text": "鉴只列事实：泊材料 context 只列令源与前情与红线事实，不预设彩排该不该开工的建议，出泊唯人节点，建议归法与人的范畴",
            "inquiry_stage": "first",
            "domain_tag": "target",
            "depth": 0,
            "rationale": "泊材料的效度在不越权：进泊是记账，材料零修饰",
            "confidence": 0.95,
            "philosophy_ref": {
                "source": "sih-philosophy/emanation/proodos/07-on-assay.md",
                "quote": q3,
            },
            "evidence": "sih-philosophy/emanation/proodos/07-on-assay.md:55",
        },
    ],
    "calls_in": 0,
    "calls_out": 0,
    "elicitation": {
        "signals_file": "sih-tools/scribe/reports/2026-09-09-parksi-solo-elicit-signals.ndjson",
        "signals": 6,
        "disposition": [
            "叩问处置[候令]：普通词面描述性使用即候人节点裁令的状态指称，本批不立名不登记，升术语与否归后续命名裁决",
            "叩问处置[彩排]：普通词面描述性使用即 SiInfer 侧接入前演练动作的指称，本批不立名不登记，升术语与否归后续命名裁决",
            "叩问处置[彼侧]：普通词面方位描述词即 SiInfer 工作区一侧的指代，本批不立名不登记，升术语与否归后续命名裁决",
            "叩问处置[接入形]：普通词面描述性使用即彼侧 agent 接入形态的指称，本批不立名不登记，升术语与否归后续命名裁决",
            "叩问处置[泊置]：普通词面描述性使用即入泊裁定动作的指称即泊界记账语义，本批不立名不登记，升术语与否归后续命名裁决",
            "叩问处置[踏勘]：普通词面描述性回指即此前只读侦察会话的指称，本批不立名不登记，升术语与否归后续命名裁决",
        ],
    },
}

OUT.write_text(json.dumps(record, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(f"written {OUT}")
# 逐字节子串自证
src1 = (ROOT / "sih-philosophy/emanation/proodos/08-on-settle.md").read_text(encoding="utf-8")
src3 = (ROOT / "sih-philosophy/emanation/proodos/07-on-assay.md").read_text(encoding="utf-8")
assert q1 in src1 and q2 in src1 and q3 in src3, "quote not byte substring"
print("byte-substring assertions ok")
