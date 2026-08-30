---
entry: INT-013-trigonometric-substitution.md
agent: worker-MiniServer-46858-215454
anchors:
  - pro: PRO-04 道三代码自晦意图必复
    source: sih-philosophy/emanation/proodos/04-on-third-tao.md:17
    quote: "代码不会自行揭示意图"
  - pro: PRO-06 法
    source: sih-philosophy/emanation/proodos/06-on-canon.md:83
    quote: "法五：顺势。治理力度按场景适配，不一刀切。"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：三角换元利用勾股恒等式 sin 平方加 cos 平方 等于 1 消除平方根，欧拉与伯努利家族在求解积分时大量使用，三角换元也是椭圆积分和特殊函数理论的起点
- 哲学命题：PRO-04 道三代码自晦意图必复，锚句「代码不会自行揭示意图」含平方根的符号层 sqrt 形式是自晦的，意图层是勾股恒等式约束下的三角函数表达，三角换元是从自晦的平方根形式恢复为意图层三角结构的必复路径，sin 平方加 cos 平方 等于 1 是恢复路径的本体论约束
- 哲学命题：PRO-06 法，锚句「法五：顺势。治理力度按场景适配，不一刀切。」三种基本替换 x 等于 a sin t 配 sqrt 括号 a 平方减 x 平方、x 等于 a tan t 配 sqrt 括号 a 平方加 x 平方、x 等于 a sec t 配 sqrt 括号 x 平方减 a 平方 是按场景适配的顺势，三角换元不强迫使用单一替换而是依据平方根内部符号适配选择
- 形式化：设符号层 S 等于 sqrt 括号 a 平方 减 x 平方，意图层 I 等于 a cos t 其中 x 等于 a sin t。PRO-04 对应 S 在勾股恒等式下等价于 I 即意图必复路径，PRO-06 顺势对应三种替换是按 S 内部加减符号分类适配。判定准则为平方根内部符号即 a 平方 减 x 平方、a 平方 加 x 平方、x 平方 减 a 平方，对应工程判据为根号内表达式的形式，机械可验。
