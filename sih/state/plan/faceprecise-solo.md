# faceprecise-solo：锁面超宽阈值调优与未用罚口径差值实证

> 令源：pk-074 出泊裁定（pkexits3 批 2026-09-17）即阈值调优与 used_paths 口径差值实证并批
> 范式：solo 主会治理加子代理数据分析（账单重放实证，主会结算）
> stem 认领：faceprecise-solo，甲表三件即 zh 锁面阈值与罚径实证、code 无承、派生 faceprecise:new

## 一、问题陈述 {#problem}

超宽阈值 20 起步宁宽候数据攒量后裁调即 lockface-bills 已 1699 笔；未用罚口径以 unused_paths 对 allow 面近似比对即 used=allow 近似，精确化候差值实证。

## 二、关键设计 {#design}

阈值实证按哨兵真实语义即 lock_face 配对塌缩逐行重放 locks 册得 401 会话峰值持锁分布，p95=22 上取整 25 穿越常规带落断口，命中 26 会话降 10 即降 61.5% 重尾全保留。罚径实证全量重算 31 笔罚单 124 条路径，以会话窗 mtime 加目录扫描取证得误罚 65 条即 52.4%，精确口径建议即 used_paths 改取实际改动文件集首选举收约 worktree diff。常量改值位引擎 src/bin/watchcheck.rs 41 行即 20 改 25，围堰位随冻结不改动。

## 三、工作清单 {#work}

- [ ] fp-01：数据分析两件落批材料即分析报告与机读结论
- [ ] fp-02：引擎常量 20 改 25 加全量回归
- [ ] fp-03：精确化口径建议落档候后继实装批

## 四、验收 {#acceptance}

watchcheck 套件与全量 cargo test 零失败；实证数据表在档；命中预期 26 降 10。
