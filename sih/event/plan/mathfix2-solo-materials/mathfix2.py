#!/usr/bin/env python3
"""mathfix2 批二修订：概率三条逐处手术 + INDEX/mapping 同步。失配即报错。"""
import pathlib, sys

R = pathlib.Path('/Users/moc/workspaces/SiHankor/worktrees/sih-math/mathfix2-solo')
E = {
 'PROB-001': R/'probability/entries/PROB-001-law-of-large-numbers.md',
 'PROB-002': R/'probability/entries/PROB-002-strong-law-of-large-numbers.md',
 'PROB-003': R/'probability/entries/PROB-003-central-limit-theorem.md',
 'INDEX': R/'probability/INDEX.md',
 'MAPPING': R/'llm-friendly-build/mapping.md',
}
EDITS = {k: [] for k in E}

EDITS['PROB-001'] += [
 ("弱大数定律的多个变体（Bernoulli 大数定律、Khintchine 大数定律）在更弱的条件下成立，但本质机制相同：方差随样本数 n 的增大而缩小到 0。",
  "弱大数定律的多个变体在更弱的条件下成立：Bernoulli 情形是特例；Khintchine 定理只要求独立同分布与期望有限，允许方差无穷。故方差的收缩不是一般机制，Chebyshev 路线只是方差有限情形的证明捷径。"),
 ("- 哲学命题：PRO-07 鉴（多主体协作打破自证循环），PRO-04 同根多源（多源投票的一致性）",
  "- 哲学命题：PRO-07 鉴（多主体协作打破自证循环）"),
 ("- 形式化：PRO-07 鉴层要求多主体协作，每个主体（鉴）独立给出判据。WLLN 形式化鉴层投票的一致性：只要各鉴独立同分布，判据的算术平均依概率收敛到真值，多鉴协作的共识在样本足够大时几乎必然出现。PRO-04 同根多源要求多源指向同一治理意图，WLLN 保证多源在足够多时达成一致。",
  "- 形式化：PRO-07 鉴层要求多主体协作，每个主体独立给出判据。WLLN 给出该协作的统计刻画：只要各判据独立同分布且期望有限，判据算术平均依概率收敛到真值，即样本足够大时偏离超过任一固定阈值的概率可任意小。注意依概率不等于几乎必然，单条样本路径的收敛保证由 PROB-002 承载。"),
 ("- 1900 年 Borel 在连续情况下证明",
  "- 1909 年 Borel 对 Bernoulli 情形给出几乎必然收敛的强形式"),
]

EDITS['PROB-002'] += [
 ("几乎必然收敛是依概率收敛的加强。依概率收敛允许偏离事件以零概率出现但事件本身可能发生；几乎必然收敛要求偏离事件在概率空间上完全不可能出现。",
  "几乎必然收敛是依概率收敛的加强：几乎必然是说偏离事件全体的概率为零，概率为零的事件仍可能发生只是可忽略；依概率收敛只要求每个固定误差的偏离概率随样本数趋于零，不保证单条样本路径最终稳定。"),
 ("- 概率空间：$(\\Omega, \\mathcal{F}, \\mathbb{P})$ 是完备概率空间",
  "- 概率空间：$(\\Omega, \\mathcal{F}, \\mathbb{P})$ 为一般概率空间即可，完备性非必需，完备化只影响零集子集的可测性而不影响几乎处处陈述"),
 ("Kolmogorov 零一律（Kolmogorov's 0-1 Law）在 SLLN 证明中起核心作用：尾事件 $\\sigma$-代数上的事件概率为 0 或 1，而 $\\lim S_n$ 的存在性是尾事件，从而其概率为 0 或 1。",
  "Kolmogorov 零一律在部分 SLLN 证明中起辅助作用：$\\lim S_n$ 的存在性是尾事件，其概率只能为 0 或 1，从而证明只需排除概率为 0 的情形。零一律本身不给出收敛，收敛的成立仍依赖独立性与矩条件。"),
 ("- 哲学命题：PRO-07 鉴（多主体协作打破自证循环），PRO-04 同根多源（多源投票的一致性）",
  "- 哲学命题：PRO-07 鉴（多主体协作打破自证循环）"),
 ("- 形式化：PRO-07 鉴层要求多主体独立给出判据。SLLN 形式化鉴层投票的强一致性：只要各鉴独立同分布且期望存在，判据均值的极限以概率 1 等于真值（见公式 $\\ref{eq:prob002-slln}$），多鉴协作的共识在样本路径上是必然事件，不是高概率事件。PRO-04 同根多源要求多源指向同一治理意图，SLLN 将多源一致性从概率保证升级为几乎必然保证",
  "- 形式化：PRO-07 鉴层要求多主体独立给出判据。SLLN 给出该协作的强统计刻画：判据独立同分布且期望存在时，判据均值的极限以概率 1 等于真值（见公式 $\\ref{eq:prob002-slln}$）。以概率 1 意为止可忽略例外而非必然，个别样本路径仍可能偏离，只是这类路径的概率为零"),
 ("Etemadi 证明的关键观察是只需 $\\mathbb{E}[|X_i|] < \\infty$ 而不需 $X_i$ 同分布（独立即可）。证明分三步。",
  "Etemadi 证明的关键观察是相互独立可减弱为两两独立，同分布与期望有限仍保留：两两独立同分布且 $\\mathbb{E}[|X_i|] < \\infty$ 即得 SLLN。证明分三步。"),
 ("3. 截断到原始的回归：证 $\\mathbb{E}[\\sum |X_i| \\mathbf{1}_{|X_i| > i}] < \\infty$ 蕴含截断不影响极限",
  "3. 截断到原始的回归：由 $\\mathbb{E}[|X_1|] < \\infty$ 推出 $\\sum_{i} \\mathbb{P}(|X_i| > i) < \\infty$，经 Borel-Cantelli 得大截断项只发生有限次，截断不影响极限"),
 ("Etemadi 证明的优势是不依赖 $X_i$ 同分布假设，仅需独立与期望存在，使 SLLN 的应用范围更广。",
  "Etemadi 证明的优势是把相互独立减弱为两两独立，同分布与期望有限假设保留，使 SLLN 在弱依赖场景仍适用。"),
 ("- TOP-001 Banach 不动点：SLLN 的证明可借助 Banach 压缩映射构造辅助序列（不同入口同结论，见公式 $\\ref{eq:top001-contractive}$）\n", ""),
 ("3. 概率空间是否完备：未完备化的概率空间上\"几乎处处\"无意义",
  "3. 概率测度是否配置：未配置概率测度的输出空间上 SLLN 无定义，完备化非必需"),
 ("- 1981 年 Nasrollah Etemadi 给出不依赖同分布假设的简化证明，被现代概率论教材广泛采用",
  "- 1981 年 Nasrollah Etemadi 给出把相互独立减弱为两两独立的简化证明，被现代概率论教材广泛采用"),
]

EDITS['PROB-003'] += [
 ("- 概率空间：$(\\Omega, \\mathcal{F}, \\mathbb{P})$ 完备",
  "- 概率空间：$(\\Omega, \\mathcal{F}, \\mathbb{P})$ 为一般概率空间即可，完备性非必需"),
 ("$$\\varphi_{S_n^*}(t) = \\left[\\varphi_{X_0}\\left(\\frac{t}{\\sigma \\sqrt{n}}\\right) \\cdot e^{-i t \\mu / (\\sigma \\sqrt{n})}\\right]^n \\label{eq:prob003-characteristic-fn}$$",
  "$$\\varphi_{S_n^*}(t) = \\left[\\varphi_{X_0}\\left(\\frac{t}{\\sigma \\sqrt{n}}\\right)\\right]^n \\label{eq:prob003-characteristic-fn}$$"),
 ("其中 $\\varphi_{X_0}$ 是 $X_i - \\mu$ 的特征函数。利用 Taylor 展开，$\\varphi_{X_0}(s) = 1 - s^2 \\sigma^2 / 2 + o(s^2)$。代入得",
  "其中 $\\varphi_{X_0}$ 是中心化变量 $X_i - \\mu$ 的特征函数，右端是 $n$ 个独立同分布中心化项特征函数的乘方。利用 Taylor 展开，$\\varphi_{X_0}(s) = 1 - s^2 \\sigma^2 / 2 + o(s^2)$。代入得"),
 ("Lindeberg 1922 的证明处理独立不同分布情形。核心是证明 Lindeberg 条件蕴含 Lyapunov 条件（见公式 $\\ref{eq:prob003-lyapunov}$）或直接通过截断+特征函数论证。",
  "Lindeberg 1922 的证明处理独立不同分布情形。两条件的关系是 Lyapunov 蕴含 Lindeberg，即 Lyapunov 更强：取 $\\delta > 0$，凡 $|X_i - \\mu_i| > \\varepsilon s_n$ 时 $|X_i - \\mu_i|^{2+\\delta} \\geq \\varepsilon^{\\delta} s_n^{\\delta} |X_i - \\mu_i|^2$，逐项放缩即得。Lindeberg 条件本身经截断加特征函数论证直接验证。"),
 ("- 哲学命题：PRO-04 同根多源（多 voter 投票平均的正态分布收敛），PRO-07 鉴（多主体协作的统计稳定性）",
  "- 哲学命题：PRO-07 鉴（多主体协作的统计稳定性）"),
 ("- 形式化：PRO-04 同根多源要求多源指向同一治理意图。CLT 形式化多源一致的统计结构：各 voter 独立同分布时，voter 均值的标准化偏差收敛到正态分布（见公式 $\\ref{eq:prob003-clt-limit}$），多源共识的偏差分布有明确的数学刻画，使多源投票的偏离行为可预测、可量化、可设置信区间。PRO-07 鉴层要求多主体协作打破自证循环，CLT 为多主体投票的稳定性提供中心极限定理：足够多独立主体投票时，投票均值的统计涨落服从正态分布，鉴层共识有可计算的置信度",
  "- 形式化：PRO-07 鉴层要求多主体协作打破自证循环。CLT 为该协作提供偏差的统计刻画：各主体判据独立同分布时，判据均值的标准化偏差收敛到正态分布（见公式 $\\ref{eq:prob003-clt-limit}$），多主体投票的偏离行为可预测、可量化、可设置信区间，鉴层共识有可计算的置信度"),
 ("- 应用场景：sih-engine 决策的信念更新",
  "- 应用场景：决策信念更新（候选设计，尚无组件实装）"),
 ("- 1930 年 Kolmogorov 在 SLLN 工作期间发展特征函数方法",
  "- 1901 年 Lyapunov 首创特征函数方法证明 CLT，此后成为标准工具"),
 ("4. 概率空间是否完备：未配概率测度的输出空间 CLT 不适用",
  "4. 概率测度是否配置：未配置概率测度的输出空间上 CLT 无定义"),
]

EDITS['INDEX'] += [
 ("| PROB-006 | 概率测度 | 待建 |", "| PROB-006 | 概率测度 | 已建 |"),
 ("| PROB-007 | 期望 | 待建 |", "| PROB-007 | 期望 | 已建 |"),
 ("2026-08-25 概率论子仓随数学仓升格为四子仓架构时新设。承接哲学命题中概率推理工具的形式化需求，初始登记 7 个核心概念。",
  "2026-08-25 概率论子仓随数学仓升格为四子仓架构时新设。承接哲学命题中概率推理工具的形式化需求，初始登记 7 个核心概念。\n\n2026-08-30 PROB-006 与 PROB-007 由 mathfix2 批建条，已建 5 条。"),
]

EDITS['MAPPING'] += [
 ("| 概率测度的定义 | PROB-006 | 概率测度 | 待建 | PROB |", "| 概率测度的定义 | PROB-006 | 概率测度 | 已建 | PROB |"),
 ("| 期望的构造 | PROB-007 | 期望 | 待建 | PROB |", "| 期望的构造 | PROB-007 | 期望 | 已建 | PROB |"),
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
