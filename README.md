# 司衡引擎 sih-engine

司衡引擎是治理 LLM 参与开发的确定性执行层。LLM 只生成符号材料，不拥有写入权；治理操作全部由确定性程序执行；人只把注意力投向异常信号。本仓是引擎本体，哲学权威源在 sih-philosophy 仓，数学形式化桥梁在 sih-math 仓。

## 为什么可以信它

- 写操作全部经租约与文件锁，无锁写入会被机械闸拦下
- 每个治理动作落哈希链即 sih/event/trail 每日一文件，一条命令可整链复算
- 判断不认人不认 LLM 只认材料：裁决类产出须经得一测量闸门与执契终签才生效
- 一切断言携带链锚，红证文化即对己不利的读数逐件留档不清洗

## 五条工程基线，大白话版

1. 确定性程序是治理的唯一执行者，LLM 产出只是待校验材料
2. 人类注意力是稀缺资源，系统按此假设设计
3. 人只在视图告警时介入，不看原始日志
4. 所有写入可追溯、可机械校验、历史不可篡改
5. 治理延伸是减少 LLM 参与而非增加

## 五分钟跑起来

```bash
cd sih-engine && cargo build

# 验链：整条治理链是否完整未篡改
target/debug/scribe verify --trail sih/event/trail/$(date +%F).ndjson

# 判据扫：五条主线判据现态
python3 ../sih-tools/critsweep/sweep.py --at $(date +%F) --root ..

# 会话与锁：当前在飞面
cd ../sih-tools/lease && uv run --project . lease status
```

## 两个入口

- 只想用：读 `doc/guide/user-guide-v1.md` 即使用者入门，三十分钟含一个真实批的完整走读
- 想参与引擎开发：读 `doc/guide/contributor-guide-v1.md` 即贡献者入门，含五仓地图、批机械链与提交变更的完整路径

## 仓内一图

| 位 | 职能 |
|---|---|
| src/ | 引擎组件六席即三问、参验、书简、判定器、视图、温故 |
| sih/event/trail/ | 治理哈希链，每日一文件，只经书简写位 |
| sih/state/plan/ 与 sih/event/plan/ | 任务包与结果档 |
| sih/state/parking/ | 泊界材料，未决事项的有界停靠地 |
| doc/ | 治理文档九类，doc/guide/ 是入门面 |
| tools 侧 | sih-tools 仓承载孵化工具，成熟后融回引擎 |
