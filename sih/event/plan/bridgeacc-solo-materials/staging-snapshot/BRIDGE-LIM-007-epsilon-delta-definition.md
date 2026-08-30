---
entry: LIM-007-epsilon-delta-definition.md
agent: worker-MiniServer-45170-215316
anchors:
  - pro: PRO-07 鉴
    source: sih-philosophy/emanation/proodos/07-on-assay.md:90
    quote: "鉴的局限：鉴只能发现\"实际是什么\"，不能直接告诉\"应该是什么\"——后者是法与应的工作"
  - pro: PRO-05 道四规约与实现必有间隙
    source: sih-philosophy/emanation/proodos/05-on-fourth-tao.md:89
    quote: "道四命题的数学骨架是双重有损链"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---
## 哲学桥接 {#philosophy-bridge}

- 借鉴源：epsilon-delta 定义的思想史源流是柯西 19 世纪初给出极限的 epsilon-delta 定义雏形，魏尔斯特拉斯严格化，把无穷小从本体论实体降级为极限的语言表达解决微积分的严格性危机。核心特征是 epsilon 和 delta 的量词顺序即 epsilon 先任意给定 delta 随后存在，这个顺序保证逼近是可控的不是偶然的。序列极限与函数极限的 epsilon-delta 形式化统一，极限定义的是静态的逼近行为不描述动态的变化过程本身。
- 哲学命题：PRO-07 鉴，锚句「鉴的局限：鉴只能发现"实际是什么"，不能直接告诉"应该是什么"」；PRO-05 道四规约与实现必有间隙，锚句「规约是意图的有形化，实现是规约的有形化。两次有形化都有自晦，两次自晦累加，就是规约与实现之间的间隙。间隙不可消除，只能识别、记录、治理。」。
- 形式化：epsilon-delta 定义是 PRO-07 鉴的数学化最小实例。epsilon 是任意给定的偏差阈值，函数 f 的极限是 L 即在 delta 邻域内 f(x) 实际值与 L 的距离小于 epsilon，= PRO-07 L90「鉴只能发现"实际是什么"」的具体形态：epsilon-delta 不告诉 f 应该等于什么，只反映 f(x) 在 x 趋近 a 时的实际值与某 L 的距离是否小于给定 epsilon 即「发现实际是什么」。定义形式不预判 L 应该是什么，epsilon 给定后 delta 必须存在即「不能直接告诉 delta 应该是什么」是「法与应的工作」即由规约层给出。L 的存在性是「鉴」的判定结果而非「法」的先验指定。另一锚 PRO-05 刻画「无穷小从本体论实体降级为语言表达」的间隙治理。19 世纪前的微积分把无穷小当作本体论实体即「非零小于任何正数」= PRO-05 L89 双重有损链的第一次编码即无穷小被命名为本体论实体；19 世纪后 epsilon-delta 把无穷小降级为极限语言即「降级」是双重有损链的第二次编码的修订，= 间隙被识别即无穷小不是实体而是语言构造；间隙被记录即 epsilon-delta 是「识别记录缩小间隙」的程序；间隙不可消除即 L 是逼近目标不是真实无穷小量，epsilon-delta 不能「消除」无穷小的本体论争议只能把争议「降级」为语言问题。工程映射，司衡治理系统对「治理动作的可信度」的判定即 epsilon-delta 的工程版：epsilon 对应「治理动作可被允许的偏差上限」即偏差阈值；delta 对应「治理动作必须被覆盖的最小范围」即审计范围；|治理动作 - 标准| 小于 epsilon 对应「治理动作实际值与标准的偏差反映」。鉴的工程版本「只能发现实际是什么不能直接告诉应该是什么」与 epsilon-delta 同源，治理审计只报告实际值与标准的偏差是否在 epsilon 内不预判应该采取什么动作即偏差处理是「法与应」的工作与 PRO-07 L90 同构。
