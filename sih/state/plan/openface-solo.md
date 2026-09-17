# openface-solo：lease open 的 repos 声明面推导

> 令源：pk-104 出泊裁定（pkexits3 批 2026-09-17）承用户令「裁，多子代理并行」
> 范式：solo 主会治理加子代理施工（工地 w/openface 产出，主会结算）
> 前置：pathfix 批已收约；allow 面已先落声明面单一源即 scope_source package 在役
> stem 认领：openface-solo，甲表三件即 zh open仓面声明推导、code 无承、派生 openface:new 加 solo:new

## 一、问题陈述 {#problem}

lease open 的 repos 仍纯旗标缺省只挂 sih-engine，与 allow 面已单一源化不对称，声明与授权在仓面两源。

## 二、关键设计 {#design}

repos 推导自任务包声明面仓前缀即 sih-engine 与 sih-tools 双已知仓前缀解析，复用 closegate repo_entries 前缀解析形；推导面空时回落缺省 sih-engine 保金向量兼容；repo 旗标降为显式扩写位即并集去重；域外件不推导仓。

## 三、工作清单 {#work}

- [ ] of-01：derive_repo_names 加 compose_repo_names 两纯函数加 cmd_open 组装位改写
- [ ] of-02：tests/openface.rs 四件即双仓推导、旗标并集、重复去重、回落兼容
- [ ] of-03：全量回归含金向量零破

## 四、验收 {#acceptance}

新套件四件绿即红转绿证据在批材料；cargo test --bin lease 与 lease_mergeback 全族零回归；t2 金向量逐字节比对不破。
