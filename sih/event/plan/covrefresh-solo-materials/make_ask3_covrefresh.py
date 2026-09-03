# ask3 记录生成器：covrefresh-solo 批（2026-09-04）
# 三锚引文程序切片：从 sih-philosophy 原文按行号切片逐字节子串，禁手打。
# 用法：python3 make_ask3_covrefresh.py <SIHANKOR_ROOT>
import json
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
DATE = "2026-09-04"
BATCH = "covrefresh-solo"
SESSION = "sess-zcode-260904-covrefresh"


def slice_line(rel_path, line_no):
    """按行号程序切片，返回该行去掉换行符的逐字节原文。"""
    text = (ROOT / rel_path).read_text(encoding="utf-8")
    lines = text.splitlines()
    line = lines[line_no - 1]
    assert line in text, f"切片非原文子串: {rel_path}:{line_no}"
    return line


ANCHOR_DEFS = [
    ("sih-philosophy/emanation/proodos/07-on-assay.md", 61),
    ("sih-philosophy/emanation/proodos/06-on-canon.md", 185),
    ("sih-philosophy/emanation/proodos/08-on-settle.md", 108),
]

anchors_meta = [
    {
        "anchor_seq": 1,
        "text": "账本刷新是映照不是投射：rev2 三态读数由确定性脚本对源码实跑扫描承载，接线批反映逐件可 grep 复现，白名单 167 概念 ID 口径不变，不投射结论不虚构载体，伪读数与漂移一律以实跑为准",
        "inquiry_stage": "first",
        "domain_tag": "target",
        "depth": 0,
        "rationale": "覆盖账本的效度在映照性：三态读数跟可复算脚本走不跟人工记忆走，源码引用位与磁盘 entry 双实存核验零虚构",
        "confidence": 0.95,
    },
    {
        "anchor_seq": 2,
        "text": "读数收紧方向单一：已实例化门槛从源码引用收紧为源码引用载体且推导档在案双门，账面漂移回填走收紧不放松，六件接线批与 gatecap 与 facet A2 与 gauge ga-2 逐件反映零静默，可指认未实例化与无可指认载体只呈报不代裁",
        "inquiry_stage": "first",
        "domain_tag": "target",
        "depth": 0,
        "rationale": "治理力度可由松到紧不可由紧到松：rev1 收紧伪读数，rev2 承其口径把新落地载体按双门收紧入账，方向单一",
        "confidence": 0.95,
    },
    {
        "anchor_seq": 3,
        "text": "应而不藏两态留证：rev2_script.py 同参双跑逐字节一致与归类变动逐件申报零静默与零命中如实记与新判已实例化双实存核验表全数上链，过与不过两态都如实申报构成应鉴循环，rev1 三件原账本只读不动零改写",
        "inquiry_stage": "first",
        "domain_tag": "target",
        "depth": 0,
        "rationale": "账面漂移如实申报的价值在应而不藏：刷新前读数与刷新后读数并列可回溯，rev1 原件零触碰",
        "confidence": 0.95,
    },
]

anchors = []
for anchor, (rel_path, line_no) in zip(anchors_meta, ANCHOR_DEFS):
    quote = slice_line(rel_path, line_no)
    anchor["philosophy_ref"] = {"source": rel_path, "quote": quote}
    anchor["evidence"] = f"{rel_path}:{line_no}"
    anchors.append(anchor)

record = {
    "session_id": SESSION,
    "raw_input": (
        "用户 2026-09-04 委外执行 covrefresh-solo 批（数学管线覆盖账本刷新 rev2，纯脚本机械批，"
        "零行为变更零裁决点）：rev1 账本（ledgrev-solo 产，读数 1 已实例化 / 12 可指认未实例化 / "
        "10 无可指认载体）之后载体接线批已落地六件即 tally（ORD-006/011）与 selector（ALG-002 第二消费位）"
        "与 lease（ORD-020）与 scribe（ORD-019）与 identity（ALG-002/PROB-013）与 cascade（ORD-016），"
        "另有 gatecap 与 scrutinator 闸（C007/C008，载体 ORD-008）与 facet 数学基础件"
        "（facet_stats_inf 检验函数族 A2，载体 PROB-010）与 gauge ga-2（PROB-003/005 期票清偿），"
        "账面已整体漂移须 rev2；rev2_script.py 承 rev1_script.py 口径白名单 167 概念 ID 不变，"
        "同参双跑逐字节一致；三态重判已实例化=源码引用载体且推导档在案，与源码扫描对表零漏项，"
        "六件接线批与 gatecap 与 facet A2 与 gauge ga-2 逐件反映；新判已实例化逐件 SIH 引用位与磁盘 "
        "entry 双实存核验；命名常数与比较位字面量两表按 rev1 口径重扫归类变动逐件申报零静默；"
        "rev1 三件原账本只读不动，rev2 件全部 -rev2 后缀新落；"
        "政策行（用户 2026-09-04 裁定）：继续推进数学仓的完善，遇到裁决点过得一裁，退人节点——"
        "本批零裁决点，机械链全绿即自行收口推进；并行批 facetmath-solo（sih-tools/facet 施工面）与 "
        "m3clear-solo（泊界与命题区面）同日在跑，共享追加面一律 append 短持即取即放，"
        "exclusive 撞锁有限重试上限十次逐次计数"
    ),
    "round": 1,
    "intent_contract": {
        "goal": (
            "covrefresh-solo 数学管线覆盖账本刷新 rev2 批：rev2_script.py 承 rev1 口径出件，"
            "白名单 167 概念 ID 不变，双跑逐字节一致；逐机制三态重判（已实例化=源码引用载体且推导档在案；"
            "可指认未实例化；无可指认载体）与源码扫描对表零漏项，六件接线批与 gatecap 与 facet A2 与 "
            "gauge ga-2 逐件反映；新判已实例化每一件 SIH 引用位与磁盘 entry 双实存核验承对挂核验表形；"
            "命名常数与比较位字面量两表按 rev1 口径重扫归类变动逐件申报零静默；产出仅落 "
            "sih-math/docs/mathpipe-coverage-2026-09-03/（rev2 后缀三件加脚本）与本批结果档与材料，"
            "rev1 三件原账本只读不动"
        ),
        "exclusions": [
            "零行为变更是底线即纯账面刷新零改任何工具与引擎源码零改判定逻辑（红线，任务包约束）",
            "rev1 三件原账本（ledger.json 与 summary.md 与 env-params.json）只读不动（红线，rev1 先例）",
            "主树零直写一切待提交件先入工地从工地施工，共享追加面 append 短持即取即放（红线，工地铁律）",
            "守卫在位严禁 plain git commit 直提，close 通道以外提交须 --no-verify 加 lease bypass 登记（红线）",
            "禁 LLM 直改链文件，链写入只经引擎 scribe 写位，闸三 --session 加 --sessions 必带（红线）",
            "不碰 facet/selector 源码（facetmath 批面）与命题区（m3clear 批面），禁管道掩退出码（红线）",
        ],
        "output_format": (
            "ask3 记录与验证件、叩问信号与 digest、正身件、租约开收约与锁实录、当日链 intent 与认证事件、"
            "rev2_script.py 双跑逐字节一致读数、载体三态 rev1 与 rev2 对表、新判已实例化双实存核验表、"
            "归类变动逐件申报表、三步管线读数、认证清单、冲突样本节、math 与 engine 两仓 settle 提交号、"
            "链 verify 前后对表、reconcile 读数、F 表、结果档与完工报告"
        ),
        "injected_constraints": [
            "工程基线第一条确定性程序是治理操作的唯一执行者：三态读数与双实存核验由 rev2_script.py 机械承载",
            "工程基线第二第三条人类注意力只投向异常信号：纯机械链全绿即自行收口推进，结果档不设等你令节（用户裁定 2026-09-04 政策行照录）",
            "唯一桥梁裁定 2026-09-02：工程产出以数学仓映射表与条目为合法消费面，新判已实例化逐件条目磁盘实存核验",
            "PRO-07 鉴映照：rev2 账本映照源码接线实况，判定交确定性脚本双跑核验零投射",
            "PRO-08 应而不藏：账面漂移与归类变动与零命中如实申报，rev1 原件只读零改写",
            "A-A4.2 裁决权归确定性引擎：三态判与归类改判由脚本按规则机械执行，裁决权不下放人工主观判断",
        ],
    },
    "domain_contract": {
        "target_domain": {
            "scope": (
                "sih-math/docs/mathpipe-coverage-2026-09-03/（rev2 四件：ledger-rev2.json 与 summary-rev2.md "
                "与 env-params-rev2.json 与 rev2_script.py）与 sih-engine/sih/event/plan/covrefresh-solo-materials/ "
                "与 sih-engine/sih/event/plan/covrefresh-solo-results.md 与 sih-engine/sih/state/plan/covrefresh-solo.md "
                "与 sih-engine/sih/event/trail/2026-09-04.ndjson 与 sih-tools/scribe/reports/ "
                "与 sih-tools/scribe/CALL-LOG.md 与 sih-tools/lease/CALL-LOG.md 与 sih-tools/meter/counts/ "
                "与 sih-tools/nomenclator/packs/core/ 与 sih-tools/identity/reports/ "
                "与 worktrees/sih-tools/covrefresh-solo 与 worktrees/sih-engine/covrefresh-solo 与 worktrees/sih-math/covrefresh-solo"
            ),
            "max_depth": 1,
        },
        "support_domain": {
            "scope": (
                "sih-math/docs/mathpipe-coverage-2026-09-03/ 原账本三件与 rev1 三件（只读）与 "
                "sih-engine/sih/event/plan/ledgrev-solo-materials/rev1_script.py（口径基）与 "
                "sih-engine/sih/event/plan/ledgrev-solo-results.md 先例 与 sih-math/docs/ 八件推导档（只读核验）"
                "与 sih-math 五子仓 INDEX.md 与 entries/（只读核验）与 sih-tools/BATCH-FACE.md 调用面含 2026-09-04 坑位勘误节 "
                "与 sih-philosophy/emanation/proodos/07-on-assay.md 与 06-on-canon.md 与 08-on-settle.md 引文原文只读切片"
            ),
            "max_depth": 1,
        },
    },
    "anchors": anchors,
    "calls_in": 0,
    "calls_out": 0,
    "elicitation": {
        "signals_file": f"sih-tools/scribe/reports/{DATE}-{BATCH}-elicit-signals.ndjson",
        "signals": 3,
        "disposition": [
            {
                "subject": "载体三态",
                "weight": "轻",
                "disposal": (
                    "叩问处置[载体三态]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，"
                    "定义承覆盖账本三态语义即已实例化=源码引用载体且推导档在案、可指认未实例化=有指认面"
                    "无源码实例化、无可指认载体=三面零命中，ledgrev-solo 立三态本批承接口径刷新"
                ),
            },
            {
                "subject": "期票",
                "weight": "轻",
                "disposal": (
                    "叩问处置[期票]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，"
                    "定义承载体先登记后清偿的期许票面即 gauge ga-2 承 PROB-003/005 两张期票经 "
                    "mathpipe-a3 推导档清偿在案"
                ),
            },
            {
                "subject": "账面漂移",
                "weight": "轻",
                "disposal": (
                    "叩问处置[账面漂移]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，"
                    "定义承账本读数与源码实况的结构性偏离即接线批落地后账本未刷新，本批刷新对象即此"
                ),
            },
        ],
    },
}

out = ROOT / f"sih-tools/scribe/reports/{DATE}-ask3-{BATCH}-record.json"
out.write_text(json.dumps(record, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(f"written: {out}")
for a in anchors:
    print(f"anchor {a['anchor_seq']}: {a['evidence']} quote={a['philosophy_ref']['quote'][:40]}...")
