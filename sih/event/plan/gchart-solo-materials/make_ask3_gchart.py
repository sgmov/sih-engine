#!/usr/bin/env python3
"""gchart-solo ask3 记录生成器：三锚哲学引文程序切片，禁手打。

引文取哲学原文整行逐字节切片并断言子串在位；evidence 行号由定位动态算出。
raw_input 逐字节读入本批 dispatch.md。产出落 scribe/reports ask3 记录。
"""
import json
from pathlib import Path

ROOT = Path("/Users/moc/workspaces/SiHankor")
OUT = ROOT / "sih-tools/scribe/reports/2026-09-04-ask3-gchart-solo-record.json"


def slice_line(rel: str, key: str) -> tuple[str, str]:
    """按 key 定位行，取该行整行逐字节切片，返回 (引文, evidence)。"""
    p = ROOT / rel
    lines = p.read_text(encoding="utf-8").splitlines()
    for i, line in enumerate(lines, 1):
        if key in line:
            quote = line
            assert quote in p.read_text(encoding="utf-8"), "切片非逐字节子串"
            return quote, f"{rel}:{i}"
    raise SystemExit(f"锚定位失败: {rel} 无 {key!r}")


q1, e1 = slice_line(
    "sih-philosophy/emanation/proodos/07-on-assay.md",
    "司衡的鉴要求：检验时只反映事实",
)
q2, e2 = slice_line(
    "sih-philosophy/emanation/proodos/08-on-settle.md",
    "应变相的具体实施（监测阈值、信号选择）是设计推论",
)
q3, e3 = slice_line(
    "sih-philosophy/emanation/proodos/08-on-settle.md",
    "应的反馈需要载体，载体是留痕",
)

raw_input = (ROOT / "sih-engine/sih/event/plan/gchart-solo-materials/dispatch.md").read_text(
    encoding="utf-8"
)

signals_file = "sih-tools/scribe/reports/2026-09-04-gchart-solo-elicit-signals.ndjson"


def disp(subject: str, disposal: str) -> dict:
    return {"subject": subject, "weight": "轻", "disposal": disposal}


record = {
    "session_id": "sess-zcode-260904-gchart",
    "raw_input": raw_input,
    "round": 1,
    "intent_contract": {
        "goal": "gchart-solo 秤星升控制图批：gauge 新增只读 gchart 子命令，读路径从链上同维同主体 reading_recorded 序列出 SPC 控制图读数——基线期均值与样本标准差定控制限（承 PROB-010 水平先验 k 显式给参双侧误报率随行声明）、越限告警谓词载越限侧与限值与标准化偏离与统计依据行不裸报布尔（承 PROB-010 公理二拒绝语义）、CUSUM 变点统计量越阈报信号位不报结论（承 PROB-013 定理一与工程注意事项三微兆信号是视图告警处置归人节点）；基线窗口长必填显式给参承 PROB-013 公理一基线先验与公理三窗口对价；推导档落 sih-math/docs/gchart-derivation-2026-09-04.md，载体引用 cli.py 注记锚点与数学仓 mapping.md 行 200 与 203 实取；record 写路径与读数事件 schema 零改动，既有测试零回归，新读路径测试先红后绿，金向量夹具读数序列双跑逐字节一致加越限与未越限两态用例",
        "exclusions": [
            "record 写路径与读数事件 schema 零改动是红线（ga-2 在役面，任务包禁区）",
            "不碰 sih-tools/wikirecall 与 sih-tools/elicit 源码（并行批 recallloop-solo 与 elicitwire-solo 施工面，红线）；elicit check 只读调用属机械链义务不属施工",
            "主树零直写，一切待提交件先入工地从工地施工，共享追加面 append 短持即取即放（红线，工地铁律）",
            "守卫在位严禁 plain git commit 直提，close 通道以外提交须 --no-verify 加 lease bypass 登记（红线）",
            "禁 LLM 直改链文件，链写入只经引擎 scribe 写位，闸三 --session 加 --sessions 必带（红线）",
            "两载体已在仓已裁定，本批零新裁决点；若实跑发现载体语义不覆盖即 M-3 模型域缺口，停批走清账路径泊界登记，禁硬挂禁自造载体（用户 2026-09-04 政策行）",
            "禁管道掩退出码，管线 findings 必须亲读（红线）",
        ],
        "output_format": "ask3 记录与验证件、叩问信号与 digest、正身件、租约开收约与锁实录与领取登记、当日链 intent 与认证事件、gauge gchart 读路径实现与 CONTRACT 修订与测试先红后绿实录、金向量双跑逐字节一致与越限未越限两态用例读数、推导档、管线三步读数、认证清单、冲突样本节、三仓 settle 提交号、链 verify 对表、reconcile 读数、F 表、结果档与完工报告",
        "injected_constraints": [
            "工程基线第一条确定性程序是治理操作的唯一执行者：控制限与越限与变点判定全由 gauge 确定性算半承载，同参双跑逐字节一致，零 LLM 参与",
            "工程基线第三条人类注意力只投向异常信号：越限与变点信号即视图告警，人只看告警视图不看原始日志，处置归人节点不自动触发治理动作（PROB-013 工程注意事项三同款）",
            "工程基线第五条治理延伸是减少 LLM 参与：gchart 是纯统计读路径，无任何生成式环节",
            "PRO-07 鉴映照：gchart 只映照链上读数序列的越限事实不投射建议，输出零建议零排序零自动处置",
            "PRO-08 应而不藏：告警输出载越限侧与限值与统计依据行不裸报布尔，管线与双跑与零命中如实上链",
            "唯一桥梁裁定 2026-09-02：载体语义消费面为数学仓映射表与条目即 mapping.md 行 200 PROB-010 与行 203 PROB-013 与两 entry 原文，引用落在书单图闭包内",
        ],
    },
    "domain_contract": {
        "target_domain": {
            "scope": "sih-tools/gauge/（src/gauge/cli.py 与 tests/test_gauge.py 或新增 tests/test_gchart.py 与 tests/fixtures/gchart/ 与 CONTRACT.md 与 pyproject 不动面核验）与 sih-math/docs/gchart-derivation-2026-09-04.md 与 sih-tools/nomenclator/packs/core/terms.json（本批六词登记）与 sih-engine/sih/event/plan/gchart-solo-materials/ 与 sih-engine/sih/event/plan/gchart-solo-results.md 与 sih-engine/sih/state/plan/gchart-solo.md 与 sih-engine/sih/event/trail/2026-09-04.ndjson 与 sih-tools/scribe/reports/ 与 sih-tools/scribe/CALL-LOG.md 与 sih-tools/lease/CALL-LOG.md 与 sih-tools/meter/counts/ 与 sih-tools/identity/reports/ 与 worktrees/sih-tools/gchart-solo 与 worktrees/sih-engine/gchart-solo 与 worktrees/sih-math/gchart-solo",
            "max_depth": 1,
        },
        "support_domain": {
            "scope": "sih-math/probability/entries/PROB-010-hypothesis-testing-and-significance.md 与 PROB-013-stationarity-and-change-point.md（只读载体条目）与 sih-math/llm-friendly-build/mapping.md 行 200 与 203（只读）与 sih-tools/gauge/CONTRACT.md 既有契约与 tests/test_gauge.py 既有测试（只读回归面）与 sih-philosophy/emanation/proodos/07-on-assay.md 与 08-on-settle.md 引文原文只读切片与 sih-tools/BATCH-FACE.md 调用面含 2026-09-04 坑位勘误节",
            "max_depth": 1,
        },
    },
    "anchors": [
        {
            "anchor_seq": 1,
            "text": "控制图读路径是鉴不是法：gchart 从链上读数序列映照越限事实，控制限从基线期数据算出，输出零建议零排序零自动处置，判断留给视图另一端的人节点",
            "inquiry_stage": "first",
            "domain_tag": "target",
            "depth": 0,
            "rationale": "只报不判是 gauge 契约既有边界，gchart 读路径承之：映照越限而不投射处置，读数值跟确定性算半走不跟生成式走",
            "confidence": 0.95,
            "philosophy_ref": {"source": "sih-philosophy/emanation/proodos/07-on-assay.md", "quote": q1},
            "evidence": e1,
        },
        {
            "anchor_seq": 2,
            "text": "控制限与越限告警是应几的设计推论：监测阈值与信号选择显式给参（k 与 b 与 h 与基线窗长全部声明），误报率由阈值显式承担，参数变更须重标定，微兆信号是视图告警处置归人节点",
            "inquiry_stage": "first",
            "domain_tag": "target",
            "depth": 0,
            "rationale": "PROB-013 定理一 CUSUM 与公理三窗口对价、PROB-010 公理一水平先验共同要求阈值在察看数据前固定且显式声明，gchart 参数面照此落地",
            "confidence": 0.95,
            "philosophy_ref": {"source": "sih-philosophy/emanation/proodos/08-on-settle.md", "quote": q2},
            "evidence": e2,
        },
        {
            "anchor_seq": 3,
            "text": "告警与判定的反馈以留痕为载体：越限输出载越限侧与限值与统计依据行不裸报布尔，管线三步与金向量双跑与零命中如实上链，应鉴循环的构成性条件由当日链承载",
            "inquiry_stage": "first",
            "domain_tag": "target",
            "depth": 0,
            "rationale": "F-3 告警可解释判据直接承应而不藏：告警若只报布尔则循环断链，证据行随行使视图端可回溯可审计",
            "confidence": 0.95,
            "philosophy_ref": {"source": "sih-philosophy/emanation/proodos/08-on-settle.md", "quote": q3},
            "evidence": e3,
        },
    ],
    "calls_in": 0,
    "calls_out": 0,
    "elicitation": {
        "signals_file": signals_file,
        "signals": 6,
        "disposition": [
            disp(
                "控制图",
                "叩问处置[控制图]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，定义承统计过程控制视图即基线期定控制限与监测点越限即告警的时序视图件，gauge gc-1 读路径承载",
            ),
            disp(
                "控制限",
                "叩问处置[控制限]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，定义承基线期均值加减 k 倍基线标准差的判定界线，k 显式给参承 PROB-010 水平先验",
            ),
            disp(
                "越限",
                "叩问处置[越限]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，定义承监测点出控制限的判定事实，越限侧与限值与统计依据随告警输出",
            ),
            disp(
                "基线期",
                "叩问处置[基线期]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，定义承监察前确立的参照窗口，承 PROB-013 公理一基线先验与公理三窗口对价，窗口长显式给参",
            ),
            disp(
                "变点",
                "叩问处置[变点]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，定义承序列分布移位的信号位，CUSUM 统计量越阈 h 报信号位不报结论，承 PROB-013 定理一",
            ),
            disp(
                "告警",
                "叩问处置[告警]：轻信号未登记，本批登记 established 入工地 core 包随批入版控，定义承视图层异常信号输出，处置归人节点不自动触发治理动作，承工程基线第三条与 PROB-013 工程注意事项三",
            ),
        ],
    },
}

OUT.write_text(json.dumps(record, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(f"written: {OUT}")
for q, e in ((q1, e1), (q2, e2), (q3, e3)):
    print(f"anchor ok {e}: {q[:40]}...")
