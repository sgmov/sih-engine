#!/usr/bin/env python3
"""redteamfix：删 redteam 死调用位与 fa-run 幽灵词引用。失配即报错。"""
import pathlib, sys

R = pathlib.Path('/Users/moc/workspaces/SiHankor/worktrees/sih-math/redteamfix-solo/probability/entries')
E = {
 'PROB-001': R/'PROB-001-law-of-large-numbers.md',
 'PROB-002': R/'PROB-002-strong-law-of-large-numbers.md',
 'PROB-003': R/'PROB-003-central-limit-theorem.md',
 'PROB-006': R/'PROB-006-probability-measure.md',
 'PROB-007': R/'PROB-007-expectation.md',
}
EDITS = {k: [] for k in E}

EDITS['PROB-001'] += [
 ("- 借鉴方向：fa-run 的均值稳定性是 WLLN 的工程对应物，但具体实验配置（厂数 / shot 数 / 命题数）由 facet 工程层决定，不属本条目范围",
  "- 借鉴方向：多厂多 shot 投票的均值稳定性是 WLLN 的工程对应物，但具体实验配置（厂数 / shot 数 / 命题数）由 facet 工程层决定，不属本条目范围"),
]

EDITS['PROB-002'] += [
 ("- 借鉴方向：fa-run 的方差验证是 SLLN 的工程对应物，但具体实验配置（厂数 / shot 数 / 命题数）由 facet 工程层决定，不属本条目范围",
  "- 借鉴方向：多厂多 shot 投票的方差验证是 SLLN 的工程对应物，但具体实验配置（厂数 / shot 数 / 命题数）由 facet 工程层决定，不属本条目范围"),
]

EDITS['PROB-003'] += [
 ("### redteam 双局胜率的置信区间\n\nredtool 双局两轮的胜率估计可用 CLT 给出置信区间。\n\n- 形式化：每局对抗的胜率视为 0-1 采样，胜率均值依 CLT 渐近正态\n- 工程意义：双局胜率的 95% 置信区间为均值 $\\pm 1.96 \\times \\sqrt{p(1-p)/2}$\n\n", ""),
 ("fa-run 的置信区间由 CLT 直接计算", "投票均值的置信区间由 CLT 直接计算"),
]

EDITS['PROB-006'] += [
 ("## 在 redteam 的应用 {#redteam-application}\n\n- 应用场景：redteam 双局两轮的胜率估计\n- 形式化：每局胜负为伯努利试验，胜率是概率测度下的期望值，置信区间构造由 PROB-003 承载\n\n", ""),
]

EDITS['PROB-007'] += [
 ("## 在 redteam 的应用 {#redteam-application}\n\n- 应用场景：redteam 双局两轮胜率的中心值\n- 形式化：单局胜负为伯努利变量，期望即真实胜率 $p$，双局样本均值的波动由 PROB-003 承载\n\n", ""),
]

def main():
    fail = 0
    for key, path in E.items():
        t = path.read_text(encoding='utf-8')
        for i, (old, new) in enumerate(EDITS[key]):
            if old not in t:
                print(f"失配: {key} #{i}: {old[:60]!r}")
                fail += 1
            else:
                t = t.replace(old, new, 1)
        path.write_text(t, encoding='utf-8')
    print("全部替换完成" if not fail else f"{fail} 处失配")

main()
