# domface-solo 任务包：域面闸组首批（pk-096 域锚修复加 pk-098 其二 archive 枚举教学）

> 令源：用户 2026-09-11 「开始修复」。前置泊件 pk-096 与 pk-098 其二；pk-098 其一生命周期设计变更不在本批候裁另议。
> 队形：单线 solo，零子代理。会话：sess-zcode-260911-domface。域：双仓。

## 交付面

1. `sih-tools/mcpline/src/mcpline/server.py` _nomenclator_check 域锚修复（pk-096）：相对 target 存 layout 即锚 layout.root，layout None 才回退 resolve_root()；params 教学文同步改「所绑域根相对或绝对形」。
2. `sih-tools/mcpline/tests/` 域锚测试：layout 域根相对 target 解析到所绑域根（以缺席拒报文路径断言，不触 CLI）。
3. `sih-engine/src/retriever/mod.rs` RecallError::Blocked 载荷附枚举全表（pk-098 其二）：「档名非枚举值 X（合法值：fact、conclusion、experience、parked、intent）」。
4. 引擎 retriever 测试连带核对（如既有断言涉该串则随改）。
5. 批结果档 `sih-engine/sih/event/plan/domface-solo/domface-solo-results.md`。

## 明确不做

pk-098 其一锁生命周期设计变更（候裁）；pk-094 其四其五与 pk-095（候域形感知批）；司梦域孤儿锁清偿（不在中央账本，其域人节点 CLI 位）。

## F 锚定

| F | 判据 | 验法 |
|---|---|---|
| F-1 | layout 存在时相对 target 锚所绑域根 | pytest |
| F-2 | layout None 回退中央根（stdio 零变） | pytest |
| F-3 | params 教学文与实锚一致 | 对表 |
| F-4 | archive 报错附五档全表 | cargo test |
| F-5 | 双仓 settle 链笔齐全 | 链 verify |
