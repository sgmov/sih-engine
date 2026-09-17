# cascadeclose-solo：CASCADE.json 投影 close 前重建

> 令源：pk-055 出泊裁定（pkexits3 批 2026-09-17）形态即 close 前重建挂收约流程承 pk-052 择案先例
> 范式：solo 主会治理加子代理施工（工地 w/cascadeclose 产出，主会结算）
> stem 认领：cascadeclose-solo，甲表三件即 zh 收约前投影重建、code 无承、派生 cascadeclose:new

## 一、问题陈述 {#problem}

级联边册投影 doc/CASCADE.json 认证后不随动，mtime 停在 2026-09-08，消费位 locksview 缺省读它作乐观锁基线，滞后即基线失真。

## 二、关键设计 {#design}

建册核心自 bin/cascade.rs 提取库件 src/cascade_registry.rs 行为零改动（t4 金向量逐字节佐证），close 闸序末位预归并接线即全准入闸后重建防 SDDG 差分扰动，重建自工地语料建册而 header.root 锚主树位防伪 diff，diff 落工地自成 pre-close 笔承归并路径，失败 fail-visible 拒收约，新城域形与无基建与无载体三形跳过注记不炸。

## 三、工作清单 {#work}

- [ ] cc-01：库件提取加 build_registry_recorded 加闸序接线
- [ ] cc-02：tests 六面即 stale 重建逐字节一致与 in_sync 与三跳过形与毒件拒收
- [ ] cc-03：投影刷新落工地随归并清偿 09-08 滞后

## 四、验收 {#acceptance}

新套件六件绿；t4 cascade 金向量三件零回归；t2 close 回执金向量逐字节不破即跳过形不增键；投影 edges 73 至 99 零孤儿。
