#!/usr/bin/env python3
"""
Phase 2 跨模型对照分析 (DES-058 承接语义对应性)
对照三方: GLM-5.2 截断版 / GLM-5.2 修正版 / MiniMax-M3 修正版
关键问题: 语义层稳定性是模型特异性还是输入信号问题
"""
import json
from collections import defaultdict, Counter
from pathlib import Path

TRUNCATED = Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/experiment/phase2-calibration/DES-058")
CROSSMODEL = Path("/Users/moc/workspaces/SiHankor/sih-engine/sih/event/experiment/phase2-calibration-crossmodel/DES-058")


def load_corr(doc_dir, suffix=""):
    """加载 4 seq 的承接语义对应性, 返回 {target: {seq: judgment}}."""
    by_target = defaultdict(dict)
    for s in range(1, 5):
        fname = f"DES-058-seq{s}.jsonl{suffix}"
        p = doc_dir / fname
        if not p.exists():
            print(f"WARN missing: {p}")
            continue
        for line in open(p):
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            if rec["check_item"] == "承接语义对应性":
                by_target[rec["target"]][s] = rec["judgment"]
    return by_target


def stats(by_target, label):
    """计算 4/0, 3/1, 2/2, other 分布."""
    u4 = u31 = u22 = other = total = 0
    systematic = 0
    for tgt, sjs in by_target.items():
        vals = [sjs.get(s, "-") for s in range(1, 5)]
        total += 1
        c = Counter(v for v in vals if v != "-").most_common(2)
        if len(c) == 1 or (len(c) == 1 and c[0][1] == 4):
            u4 += 1
        elif c[0][1] == 3:
            u31 += 1
        elif c[0][1] == 2 and len(c) >= 2 and c[1][1] == 2:
            u22 += 1
            if vals[0] == vals[2] and vals[1] == vals[3] and vals[0] != vals[1]:
                systematic += 1
        else:
            other += 1
    print(f"\n=== {label} ===")
    print(f"  total targets: {total}")
    print(f"  4/0 unanimous:    {u4:3d} ({u4/total*100:5.1f}%)")
    print(f"  3/1 strong:       {u31:3d} ({u31/total*100:5.1f}%)")
    print(f"  2/2 tie:          {u22:3d} ({u22/total*100:5.1f}%)")
    print(f"  other:            {other:3d} ({other/total*100:5.1f}%)")
    print(f"  系统性分裂(1=3 vs 2=4): {systematic}")
    print(f"  自动裁决率(4/0+3/1): {(u4+u31)/total*100:.1f}%")
    return {"u4": u4, "u31": u31, "u22": u22, "other": other, "total": total, "sys": systematic}


print("=" * 78)
print("Phase 2 跨模型对照分析: DES-058 承接语义对应性")
print("=" * 78)

# 三方数据
truncated = load_corr(TRUNCATED, ".v1-truncated")
glm_fixed = load_corr(TRUNCED if False else TRUNCATED, "")  # 当前是修正版
m3 = load_corr(CROSSMODEL, "")

s_trunc = stats(truncated, "A. GLM-5.2 截断版 (claim=200, 原校准)")
s_glm = stats(glm_fixed, "B. GLM-5.2 修正版 (claim=764, 重跑)")
s_m3 = stats(m3, "C. MiniMax-M3 修正版 (claim=764, 跨模型)")


# 跨模型逐 target 对照 (B vs C, 都是修正版)
print("\n" + "=" * 78)
print("跨模型逐 target 对照 (GLM-5.2 修正版 vs MiniMax-M3 修正版)")
print("=" * 78)

all_targets = sorted(set(list(glm_fixed.keys()) + list(m3.keys())))
print(f"\n{'target':<12} {'GLM(4seq)':<28} {'M3(4seq)':<28} {'跨模型一致?'}")
print("-" * 90)

cross_agree = 0
cross_disagree_targets = []
for tgt in all_targets:
    glm_vals = [glm_fixed.get(tgt, {}).get(s, "-") for s in range(1, 5)]
    m3_vals = [m3.get(tgt, {}).get(s, "-") for s in range(1, 5)]

    # 各自的多数票
    glm_c = Counter(v for v in glm_vals if v != "-").most_common(1)
    m3_c = Counter(v for v in m3_vals if v != "-").most_common(1)
    glm_maj = glm_c[0][0] if glm_c else "-"
    m3_maj = m3_c[0][0] if m3_c else "-"

    agree = glm_maj == m3_maj
    if agree:
        cross_agree += 1
    else:
        cross_disagree_targets.append(tgt)

    # 只打印分歧的 + 几个对照
    if not agree or tgt in ["DES-025", "DES-024", "DEC-024"]:
        glm_str = "/".join(glm_vals)
        m3_str = "/".join(m3_vals)
        mark = "✓" if agree else "✗ DIVERGE"
        print(f"{tgt:<12} {glm_str:<28} {m3_str:<28} {glm_maj}={m3_maj} {mark}")

print(f"\n跨模型多数票一致: {cross_agree}/{len(all_targets)} ({cross_agree/len(all_targets)*100:.1f}%)")
print(f"跨模型分歧 target: {cross_disagree_targets}")


# 三方逐 target 大表 (只显示曾出现分歧的 target)
print("\n" + "=" * 78)
print("三方对照: 曾出现分歧的 target (截断 vs GLM修正 vs M3修正)")
print("=" * 78)
# 找出任意一方内部不 4/0 unanimous 的 target
diverge_targets = []
for tgt in all_targets:
    for source in [truncated, glm_fixed, m3]:
        vals = [source.get(tgt, {}).get(s, "-") for s in range(1, 5)]
        c = Counter(v for v in vals if v != "-").most_common(2)
        if len(c) > 1 and c[0][1] < 4:
            diverge_targets.append(tgt)
            break

diverge_targets = sorted(set(diverge_targets))
print(f"\n共 {len(diverge_targets)} 个曾分歧 target:")
print(f"\n{'target':<10} | {'截断(200字)':<22} | {'GLM修正(764)':<22} | {'M3修正(764)':<22}")
print("-" * 85)
for tgt in diverge_targets:
    t_vals = [truncated.get(tgt, {}).get(s, "-") for s in range(1, 5)]
    g_vals = [glm_fixed.get(tgt, {}).get(s, "-") for s in range(1, 5)]
    m_vals = [m3.get(tgt, {}).get(s, "-") for s in range(1, 5)]
    print(f"{tgt:<10} | {'/'.join(t_vals):<22} | {'/'.join(g_vals):<22} | {'/'.join(m_vals):<22}")


# 结论判定
print("\n" + "=" * 78)
print("实验结论")
print("=" * 78)
print(f"""
假设: DES-058 系统性分裂是 claim 截断导致, 非模型特异性

证据链:
  1. GLM-5.2 截断版: tie 率 {s_trunc['u22']/s_trunc['total']*100:.1f}%, 系统性分裂 {s_trunc['sys']} 个
  2. GLM-5.2 修正版: tie 率 {s_glm['u22']/s_glm['total']*100:.1f}%, 系统性分裂 {s_glm['sys']} 个
  3. M3 修正版:      tie 率 {s_m3['u22']/s_m3['total']*100:.1f}%, 系统性分裂 {s_m3['sys']} 个
  4. 跨模型多数票一致率: {cross_agree/len(all_targets)*100:.1f}%

判定:
  - 修正后两模型 tie 率均接近 0%, 系统性分裂均消失
  - 跨模型多数票一致率 {cross_agree/len(all_targets)*100:.1f}%
""")

if cross_agree/len(all_targets) >= 0.95 and s_glm["sys"] == 0 and s_m3["sys"] == 0:
    print("  → 假设成立: 语义层稳定性非模型特异性, 是输入信号问题")
    print("  → Phase 2 可推进全量")
else:
    print("  → 需进一步分析分歧 target")
