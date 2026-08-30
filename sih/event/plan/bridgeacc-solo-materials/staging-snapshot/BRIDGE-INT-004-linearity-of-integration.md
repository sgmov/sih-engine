---
entry: INT-004-linearity-of-integration.md
agent: worker-MiniServer-42881-215042
anchors:
  - pro: PRO-06 法
    source: sih-philosophy/emanation/proodos/06-on-canon.md:15
    quote: "五法不是独立的方法论，是四道在治理动作上的具体展开"
  - pro: PRO-08 应
    source: sih-philosophy/emanation/proodos/08-on-settle.md:108
    quote: "司衡之应：应而不藏，应辨当下，应几未来。"
selfcheck:
  pro_ids_verified: 2
  quotes_verified: 2
---

## 哲学桥接 {#philosophy-bridge}

- 借鉴源：积分线性性是黎曼积分作为极限定义的直接推论，19 世纪黎曼、勒贝格的工作确立线性性是所有积分定义的共同代数特征，信号处理中线性时不变系统即 LTI 的核心数学基础是积分的线性性
- 哲学命题：PRO-06 法，锚句「五法不是独立的方法论，是四道在治理动作上的具体展开」积分线性性对应治理动作的可分解性，五法损补、顺势、顺因、有度、知止各自作用于治理对象，整体效果等于各法效果的线性叠加，与五法作为道之展开而非独立方法论同源
- 哲学命题：PRO-08 应，锚句「司衡之应：应而不藏，应辨当下，应几未来。」积分线性性对应应对扰动的可分解性，扰动之和的应对等于各扰动应对之和，承接应辨当下与应几未来两层，应对函数的线性假设是 ATT 入口控制、多轮审阅、convergence 收敛率统计的隐含前提
- 形式化：设治理方法向量 m 等于 m1 加 m2 加 m3，对象状态变换 T 是 T1 叠加 T2 叠加 T3。线性性要求 T 复合 m 等于 T1 复合 m1 加上 T2 复合 m2 加上 T3 复合 m3，与积分算子在函数空间上为线性泛函同构。判定准则为 T 是否满足柯西方程 T 加法齐次与 T 数乘齐次，对应工程判据为治理动作的效果是否可叠加，机械可验。
