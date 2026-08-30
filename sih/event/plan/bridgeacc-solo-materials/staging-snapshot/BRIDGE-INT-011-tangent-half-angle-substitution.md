---
entry: INT-011-tangent-half-angle-substitution.md
agent: worker-MiniServer-46272-215416
anchors:
  - pro: PRO-06 法
    source: sih-philosophy/emanation/proodos/06-on-canon.md:83
    quote: "法五：顺势。治理力度按场景适配，不一刀切。"
  - pro: PRO-04 道三代码自晦意图必复
    source: sih-philosophy/emanation/proodos/04-on-third-tao.md:92
    quote: "有形系统的固有属性是「可读性有限」"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：19 世纪魏尔斯特拉斯在三角函数积分系统化中发现 t 等于 tan 括号 x 除以 2 替换可把三角函数有理式转化为普通有理函数，几何直观是单位圆到实数线的立体投影
- 哲学命题：PRO-06 法，锚句「法五：顺势。治理力度按场景适配，不一刀切。」正切半角换元是顺势在积分层的具体形态，对周期信号场景的三角函数有理式治理力度按场景适配，不强用 INT-013 三角换元一刀切，t 等于 tan 括号 x 除以 2 是顺势选取的变量替换
- 哲学命题：PRO-04 道三代码自晦意图必复，锚句「有形系统的固有属性是「可读性有限」」三角函数符号层 sin x 与 cos x 自晦，意图层是有理函数积分 ∫ R 撇 t dt，正切半角换元是不经形式化分解的意图必复路径，从自晦的三角符号直接恢复到代数意图
- 形式化：设符号层 S 等于 R 括号 sin x 逗号 cos x，意图层 I 等于 ∫ R 撇 t dt。PRO-06 顺势对应 t 等于 tan 括号 x 除以 2 是按场景适配的变量选择，PRO-04 对应 dx 等于 2 除以 1 加 t 平方 dt 的链式结构是符号层到意图层的反链式必复路径。判定准则为 R 是否为 sin x 与 cos x 的有理式，对应工程判据为周期信号是否可化为三角函数有理组合，机械可验。
